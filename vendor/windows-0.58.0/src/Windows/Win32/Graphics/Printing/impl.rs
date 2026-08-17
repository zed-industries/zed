pub trait IAsyncGetSendNotificationCookie_Impl: Sized + IPrintAsyncCookie_Impl {
    fn FinishAsyncCallWithData(&self, param0: Option<&IPrintAsyncNotifyDataObject>, param1: super::super::Foundation::BOOL) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IAsyncGetSendNotificationCookie {}
impl IAsyncGetSendNotificationCookie_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IAsyncGetSendNotificationCookie_Vtbl
    where
        Identity: IAsyncGetSendNotificationCookie_Impl,
    {
        unsafe extern "system" fn FinishAsyncCallWithData<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut core::ffi::c_void, param1: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IAsyncGetSendNotificationCookie_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAsyncGetSendNotificationCookie_Impl::FinishAsyncCallWithData(this, windows_core::from_raw_borrowed(&param0), core::mem::transmute_copy(&param1)).into()
        }
        Self { base__: IPrintAsyncCookie_Vtbl::new::<Identity, OFFSET>(), FinishAsyncCallWithData: FinishAsyncCallWithData::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAsyncGetSendNotificationCookie as windows_core::Interface>::IID || iid == &<IPrintAsyncCookie as windows_core::Interface>::IID
    }
}
pub trait IAsyncGetSrvReferralCookie_Impl: Sized {
    fn FinishAsyncCall(&self, param0: windows_core::HRESULT) -> windows_core::Result<()>;
    fn CancelAsyncCall(&self, param0: windows_core::HRESULT) -> windows_core::Result<()>;
    fn FinishAsyncCallWithData(&self, param0: &windows_core::PCWSTR) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IAsyncGetSrvReferralCookie {}
impl IAsyncGetSrvReferralCookie_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IAsyncGetSrvReferralCookie_Vtbl
    where
        Identity: IAsyncGetSrvReferralCookie_Impl,
    {
        unsafe extern "system" fn FinishAsyncCall<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: windows_core::HRESULT) -> windows_core::HRESULT
        where
            Identity: IAsyncGetSrvReferralCookie_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAsyncGetSrvReferralCookie_Impl::FinishAsyncCall(this, core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn CancelAsyncCall<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: windows_core::HRESULT) -> windows_core::HRESULT
        where
            Identity: IAsyncGetSrvReferralCookie_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAsyncGetSrvReferralCookie_Impl::CancelAsyncCall(this, core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn FinishAsyncCallWithData<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IAsyncGetSrvReferralCookie_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAsyncGetSrvReferralCookie_Impl::FinishAsyncCallWithData(this, core::mem::transmute(&param0)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            FinishAsyncCall: FinishAsyncCall::<Identity, OFFSET>,
            CancelAsyncCall: CancelAsyncCall::<Identity, OFFSET>,
            FinishAsyncCallWithData: FinishAsyncCallWithData::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAsyncGetSrvReferralCookie as windows_core::Interface>::IID
    }
}
pub trait IBidiAsyncNotifyChannel_Impl: Sized + IPrintAsyncNotifyChannel_Impl {
    fn CreateNotificationChannel(&self) -> windows_core::Result<()>;
    fn GetPrintName(&self, param0: *const Option<IPrintAsyncNotifyDataObject>) -> windows_core::Result<()>;
    fn GetChannelNotificationType(&self, param0: *const Option<IPrintAsyncNotifyDataObject>) -> windows_core::Result<()>;
    fn AsyncGetNotificationSendResponse(&self, param0: Option<&IPrintAsyncNotifyDataObject>, param1: Option<&IAsyncGetSendNotificationCookie>) -> windows_core::Result<()>;
    fn AsyncCloseChannel(&self, param0: Option<&IPrintAsyncNotifyDataObject>, param1: Option<&IPrintAsyncCookie>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IBidiAsyncNotifyChannel {}
impl IBidiAsyncNotifyChannel_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IBidiAsyncNotifyChannel_Vtbl
    where
        Identity: IBidiAsyncNotifyChannel_Impl,
    {
        unsafe extern "system" fn CreateNotificationChannel<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IBidiAsyncNotifyChannel_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IBidiAsyncNotifyChannel_Impl::CreateNotificationChannel(this).into()
        }
        unsafe extern "system" fn GetPrintName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *const *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IBidiAsyncNotifyChannel_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IBidiAsyncNotifyChannel_Impl::GetPrintName(this, core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn GetChannelNotificationType<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *const *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IBidiAsyncNotifyChannel_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IBidiAsyncNotifyChannel_Impl::GetChannelNotificationType(this, core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn AsyncGetNotificationSendResponse<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut core::ffi::c_void, param1: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IBidiAsyncNotifyChannel_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IBidiAsyncNotifyChannel_Impl::AsyncGetNotificationSendResponse(this, windows_core::from_raw_borrowed(&param0), windows_core::from_raw_borrowed(&param1)).into()
        }
        unsafe extern "system" fn AsyncCloseChannel<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut core::ffi::c_void, param1: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IBidiAsyncNotifyChannel_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IBidiAsyncNotifyChannel_Impl::AsyncCloseChannel(this, windows_core::from_raw_borrowed(&param0), windows_core::from_raw_borrowed(&param1)).into()
        }
        Self {
            base__: IPrintAsyncNotifyChannel_Vtbl::new::<Identity, OFFSET>(),
            CreateNotificationChannel: CreateNotificationChannel::<Identity, OFFSET>,
            GetPrintName: GetPrintName::<Identity, OFFSET>,
            GetChannelNotificationType: GetChannelNotificationType::<Identity, OFFSET>,
            AsyncGetNotificationSendResponse: AsyncGetNotificationSendResponse::<Identity, OFFSET>,
            AsyncCloseChannel: AsyncCloseChannel::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IBidiAsyncNotifyChannel as windows_core::Interface>::IID || iid == &<IPrintAsyncNotifyChannel as windows_core::Interface>::IID
    }
}
pub trait IBidiRequest_Impl: Sized {
    fn SetSchema(&self, pszschema: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn SetInputData(&self, dwtype: u32, pdata: *const u8, usize: u32) -> windows_core::Result<()>;
    fn GetResult(&self) -> windows_core::Result<windows_core::HRESULT>;
    fn GetOutputData(&self, dwindex: u32, ppszschema: *mut windows_core::PWSTR, pdwtype: *mut u32, ppdata: *mut *mut u8, usize: *mut u32) -> windows_core::Result<()>;
    fn GetEnumCount(&self) -> windows_core::Result<u32>;
}
impl windows_core::RuntimeName for IBidiRequest {}
impl IBidiRequest_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IBidiRequest_Vtbl
    where
        Identity: IBidiRequest_Impl,
    {
        unsafe extern "system" fn SetSchema<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszschema: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IBidiRequest_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IBidiRequest_Impl::SetSchema(this, core::mem::transmute(&pszschema)).into()
        }
        unsafe extern "system" fn SetInputData<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwtype: u32, pdata: *const u8, usize: u32) -> windows_core::HRESULT
        where
            Identity: IBidiRequest_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IBidiRequest_Impl::SetInputData(this, core::mem::transmute_copy(&dwtype), core::mem::transmute_copy(&pdata), core::mem::transmute_copy(&usize)).into()
        }
        unsafe extern "system" fn GetResult<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, phr: *mut windows_core::HRESULT) -> windows_core::HRESULT
        where
            Identity: IBidiRequest_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IBidiRequest_Impl::GetResult(this) {
                Ok(ok__) => {
                    phr.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOutputData<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwindex: u32, ppszschema: *mut windows_core::PWSTR, pdwtype: *mut u32, ppdata: *mut *mut u8, usize: *mut u32) -> windows_core::HRESULT
        where
            Identity: IBidiRequest_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IBidiRequest_Impl::GetOutputData(this, core::mem::transmute_copy(&dwindex), core::mem::transmute_copy(&ppszschema), core::mem::transmute_copy(&pdwtype), core::mem::transmute_copy(&ppdata), core::mem::transmute_copy(&usize)).into()
        }
        unsafe extern "system" fn GetEnumCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwtotal: *mut u32) -> windows_core::HRESULT
        where
            Identity: IBidiRequest_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IBidiRequest_Impl::GetEnumCount(this) {
                Ok(ok__) => {
                    pdwtotal.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetSchema: SetSchema::<Identity, OFFSET>,
            SetInputData: SetInputData::<Identity, OFFSET>,
            GetResult: GetResult::<Identity, OFFSET>,
            GetOutputData: GetOutputData::<Identity, OFFSET>,
            GetEnumCount: GetEnumCount::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IBidiRequest as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IBidiRequestContainer_Impl: Sized {
    fn AddRequest(&self, prequest: Option<&IBidiRequest>) -> windows_core::Result<()>;
    fn GetEnumObject(&self) -> windows_core::Result<super::super::System::Com::IEnumUnknown>;
    fn GetRequestCount(&self) -> windows_core::Result<u32>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IBidiRequestContainer {}
#[cfg(feature = "Win32_System_Com")]
impl IBidiRequestContainer_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IBidiRequestContainer_Vtbl
    where
        Identity: IBidiRequestContainer_Impl,
    {
        unsafe extern "system" fn AddRequest<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, prequest: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IBidiRequestContainer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IBidiRequestContainer_Impl::AddRequest(this, windows_core::from_raw_borrowed(&prequest)).into()
        }
        unsafe extern "system" fn GetEnumObject<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IBidiRequestContainer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IBidiRequestContainer_Impl::GetEnumObject(this) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRequestCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pucount: *mut u32) -> windows_core::HRESULT
        where
            Identity: IBidiRequestContainer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IBidiRequestContainer_Impl::GetRequestCount(this) {
                Ok(ok__) => {
                    pucount.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AddRequest: AddRequest::<Identity, OFFSET>,
            GetEnumObject: GetEnumObject::<Identity, OFFSET>,
            GetRequestCount: GetRequestCount::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IBidiRequestContainer as windows_core::Interface>::IID
    }
}
pub trait IBidiSpl_Impl: Sized {
    fn BindDevice(&self, pszdevicename: &windows_core::PCWSTR, dwaccess: u32) -> windows_core::Result<()>;
    fn UnbindDevice(&self) -> windows_core::Result<()>;
    fn SendRecv(&self, pszaction: &windows_core::PCWSTR, prequest: Option<&IBidiRequest>) -> windows_core::Result<()>;
    fn MultiSendRecv(&self, pszaction: &windows_core::PCWSTR, prequestcontainer: Option<&IBidiRequestContainer>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IBidiSpl {}
impl IBidiSpl_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IBidiSpl_Vtbl
    where
        Identity: IBidiSpl_Impl,
    {
        unsafe extern "system" fn BindDevice<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszdevicename: windows_core::PCWSTR, dwaccess: u32) -> windows_core::HRESULT
        where
            Identity: IBidiSpl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IBidiSpl_Impl::BindDevice(this, core::mem::transmute(&pszdevicename), core::mem::transmute_copy(&dwaccess)).into()
        }
        unsafe extern "system" fn UnbindDevice<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IBidiSpl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IBidiSpl_Impl::UnbindDevice(this).into()
        }
        unsafe extern "system" fn SendRecv<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszaction: windows_core::PCWSTR, prequest: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IBidiSpl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IBidiSpl_Impl::SendRecv(this, core::mem::transmute(&pszaction), windows_core::from_raw_borrowed(&prequest)).into()
        }
        unsafe extern "system" fn MultiSendRecv<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszaction: windows_core::PCWSTR, prequestcontainer: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IBidiSpl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IBidiSpl_Impl::MultiSendRecv(this, core::mem::transmute(&pszaction), windows_core::from_raw_borrowed(&prequestcontainer)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            BindDevice: BindDevice::<Identity, OFFSET>,
            UnbindDevice: UnbindDevice::<Identity, OFFSET>,
            SendRecv: SendRecv::<Identity, OFFSET>,
            MultiSendRecv: MultiSendRecv::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IBidiSpl as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IBidiSpl2_Impl: Sized {
    fn BindDevice(&self, pszdevicename: &windows_core::PCWSTR, dwaccess: u32) -> windows_core::Result<()>;
    fn UnbindDevice(&self) -> windows_core::Result<()>;
    fn SendRecvXMLString(&self, bstrrequest: &windows_core::BSTR) -> windows_core::Result<windows_core::BSTR>;
    fn SendRecvXMLStream(&self, psrequest: Option<&super::super::System::Com::IStream>) -> windows_core::Result<super::super::System::Com::IStream>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IBidiSpl2 {}
#[cfg(feature = "Win32_System_Com")]
impl IBidiSpl2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IBidiSpl2_Vtbl
    where
        Identity: IBidiSpl2_Impl,
    {
        unsafe extern "system" fn BindDevice<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszdevicename: windows_core::PCWSTR, dwaccess: u32) -> windows_core::HRESULT
        where
            Identity: IBidiSpl2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IBidiSpl2_Impl::BindDevice(this, core::mem::transmute(&pszdevicename), core::mem::transmute_copy(&dwaccess)).into()
        }
        unsafe extern "system" fn UnbindDevice<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IBidiSpl2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IBidiSpl2_Impl::UnbindDevice(this).into()
        }
        unsafe extern "system" fn SendRecvXMLString<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrrequest: core::mem::MaybeUninit<windows_core::BSTR>, pbstrresponse: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IBidiSpl2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IBidiSpl2_Impl::SendRecvXMLString(this, core::mem::transmute(&bstrrequest)) {
                Ok(ok__) => {
                    pbstrresponse.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SendRecvXMLStream<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, psrequest: *mut core::ffi::c_void, ppsresponse: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IBidiSpl2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IBidiSpl2_Impl::SendRecvXMLStream(this, windows_core::from_raw_borrowed(&psrequest)) {
                Ok(ok__) => {
                    ppsresponse.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            BindDevice: BindDevice::<Identity, OFFSET>,
            UnbindDevice: UnbindDevice::<Identity, OFFSET>,
            SendRecvXMLString: SendRecvXMLString::<Identity, OFFSET>,
            SendRecvXMLStream: SendRecvXMLStream::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IBidiSpl2 as windows_core::Interface>::IID
    }
}
pub trait IFixedDocument_Impl: Sized {
    fn GetUri(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetPrintTicket(&self) -> windows_core::Result<IPartPrintTicket>;
    fn SetPrintTicket(&self, pprintticket: Option<&IPartPrintTicket>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IFixedDocument {}
impl IFixedDocument_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IFixedDocument_Vtbl
    where
        Identity: IFixedDocument_Impl,
    {
        unsafe extern "system" fn GetUri<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, uri: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IFixedDocument_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IFixedDocument_Impl::GetUri(this) {
                Ok(ok__) => {
                    uri.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPrintTicket<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppprintticket: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IFixedDocument_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IFixedDocument_Impl::GetPrintTicket(this) {
                Ok(ok__) => {
                    ppprintticket.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPrintTicket<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pprintticket: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IFixedDocument_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IFixedDocument_Impl::SetPrintTicket(this, windows_core::from_raw_borrowed(&pprintticket)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetUri: GetUri::<Identity, OFFSET>,
            GetPrintTicket: GetPrintTicket::<Identity, OFFSET>,
            SetPrintTicket: SetPrintTicket::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFixedDocument as windows_core::Interface>::IID
    }
}
pub trait IFixedDocumentSequence_Impl: Sized {
    fn GetUri(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetPrintTicket(&self) -> windows_core::Result<IPartPrintTicket>;
    fn SetPrintTicket(&self, pprintticket: Option<&IPartPrintTicket>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IFixedDocumentSequence {}
impl IFixedDocumentSequence_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IFixedDocumentSequence_Vtbl
    where
        Identity: IFixedDocumentSequence_Impl,
    {
        unsafe extern "system" fn GetUri<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, uri: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IFixedDocumentSequence_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IFixedDocumentSequence_Impl::GetUri(this) {
                Ok(ok__) => {
                    uri.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPrintTicket<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppprintticket: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IFixedDocumentSequence_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IFixedDocumentSequence_Impl::GetPrintTicket(this) {
                Ok(ok__) => {
                    ppprintticket.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPrintTicket<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pprintticket: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IFixedDocumentSequence_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IFixedDocumentSequence_Impl::SetPrintTicket(this, windows_core::from_raw_borrowed(&pprintticket)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetUri: GetUri::<Identity, OFFSET>,
            GetPrintTicket: GetPrintTicket::<Identity, OFFSET>,
            SetPrintTicket: SetPrintTicket::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFixedDocumentSequence as windows_core::Interface>::IID
    }
}
pub trait IFixedPage_Impl: Sized + IPartBase_Impl {
    fn GetPrintTicket(&self) -> windows_core::Result<IPartPrintTicket>;
    fn GetPagePart(&self, uri: &windows_core::PCWSTR) -> windows_core::Result<windows_core::IUnknown>;
    fn GetWriteStream(&self) -> windows_core::Result<IPrintWriteStream>;
    fn SetPrintTicket(&self, ppprintticket: Option<&IPartPrintTicket>) -> windows_core::Result<()>;
    fn SetPagePart(&self, punk: Option<&windows_core::IUnknown>) -> windows_core::Result<()>;
    fn DeleteResource(&self, uri: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetXpsPartIterator(&self) -> windows_core::Result<IXpsPartIterator>;
}
impl windows_core::RuntimeName for IFixedPage {}
impl IFixedPage_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IFixedPage_Vtbl
    where
        Identity: IFixedPage_Impl,
    {
        unsafe extern "system" fn GetPrintTicket<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppprintticket: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IFixedPage_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IFixedPage_Impl::GetPrintTicket(this) {
                Ok(ok__) => {
                    ppprintticket.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPagePart<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, uri: windows_core::PCWSTR, ppunk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IFixedPage_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IFixedPage_Impl::GetPagePart(this, core::mem::transmute(&uri)) {
                Ok(ok__) => {
                    ppunk.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetWriteStream<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppwritestream: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IFixedPage_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IFixedPage_Impl::GetWriteStream(this) {
                Ok(ok__) => {
                    ppwritestream.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPrintTicket<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppprintticket: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IFixedPage_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IFixedPage_Impl::SetPrintTicket(this, windows_core::from_raw_borrowed(&ppprintticket)).into()
        }
        unsafe extern "system" fn SetPagePart<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, punk: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IFixedPage_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IFixedPage_Impl::SetPagePart(this, windows_core::from_raw_borrowed(&punk)).into()
        }
        unsafe extern "system" fn DeleteResource<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, uri: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IFixedPage_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IFixedPage_Impl::DeleteResource(this, core::mem::transmute(&uri)).into()
        }
        unsafe extern "system" fn GetXpsPartIterator<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pxpspartit: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IFixedPage_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IFixedPage_Impl::GetXpsPartIterator(this) {
                Ok(ok__) => {
                    pxpspartit.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: IPartBase_Vtbl::new::<Identity, OFFSET>(),
            GetPrintTicket: GetPrintTicket::<Identity, OFFSET>,
            GetPagePart: GetPagePart::<Identity, OFFSET>,
            GetWriteStream: GetWriteStream::<Identity, OFFSET>,
            SetPrintTicket: SetPrintTicket::<Identity, OFFSET>,
            SetPagePart: SetPagePart::<Identity, OFFSET>,
            DeleteResource: DeleteResource::<Identity, OFFSET>,
            GetXpsPartIterator: GetXpsPartIterator::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFixedPage as windows_core::Interface>::IID || iid == &<IPartBase as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Ole")]
pub trait IImgCreateErrorInfo_Impl: Sized + super::super::System::Ole::ICreateErrorInfo_Impl {
    fn AttachToErrorInfo(&self, perrorinfo: *mut ImgErrorInfo) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Ole")]
impl windows_core::RuntimeName for IImgCreateErrorInfo {}
#[cfg(feature = "Win32_System_Ole")]
impl IImgCreateErrorInfo_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IImgCreateErrorInfo_Vtbl
    where
        Identity: IImgCreateErrorInfo_Impl,
    {
        unsafe extern "system" fn AttachToErrorInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, perrorinfo: *mut ImgErrorInfo) -> windows_core::HRESULT
        where
            Identity: IImgCreateErrorInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IImgCreateErrorInfo_Impl::AttachToErrorInfo(this, core::mem::transmute_copy(&perrorinfo)).into()
        }
        Self { base__: super::super::System::Ole::ICreateErrorInfo_Vtbl::new::<Identity, OFFSET>(), AttachToErrorInfo: AttachToErrorInfo::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IImgCreateErrorInfo as windows_core::Interface>::IID || iid == &<super::super::System::Ole::ICreateErrorInfo as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IImgErrorInfo_Impl: Sized + super::super::System::Com::IErrorInfo_Impl {
    fn GetDeveloperDescription(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetUserErrorId(&self) -> windows_core::Result<windows_core::GUID>;
    fn GetUserParameterCount(&self) -> windows_core::Result<u32>;
    fn GetUserParameter(&self, cparam: u32) -> windows_core::Result<windows_core::BSTR>;
    fn GetUserFallback(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetExceptionId(&self) -> windows_core::Result<u32>;
    fn DetachErrorInfo(&self, perrorinfo: *mut ImgErrorInfo) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IImgErrorInfo {}
#[cfg(feature = "Win32_System_Com")]
impl IImgErrorInfo_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IImgErrorInfo_Vtbl
    where
        Identity: IImgErrorInfo_Impl,
    {
        unsafe extern "system" fn GetDeveloperDescription<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrdevdescription: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IImgErrorInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IImgErrorInfo_Impl::GetDeveloperDescription(this) {
                Ok(ok__) => {
                    pbstrdevdescription.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetUserErrorId<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, perrorid: *mut windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IImgErrorInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IImgErrorInfo_Impl::GetUserErrorId(this) {
                Ok(ok__) => {
                    perrorid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetUserParameterCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcuserparams: *mut u32) -> windows_core::HRESULT
        where
            Identity: IImgErrorInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IImgErrorInfo_Impl::GetUserParameterCount(this) {
                Ok(ok__) => {
                    pcuserparams.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetUserParameter<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, cparam: u32, pbstrparam: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IImgErrorInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IImgErrorInfo_Impl::GetUserParameter(this, core::mem::transmute_copy(&cparam)) {
                Ok(ok__) => {
                    pbstrparam.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetUserFallback<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrfallback: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IImgErrorInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IImgErrorInfo_Impl::GetUserFallback(this) {
                Ok(ok__) => {
                    pbstrfallback.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetExceptionId<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pexceptionid: *mut u32) -> windows_core::HRESULT
        where
            Identity: IImgErrorInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IImgErrorInfo_Impl::GetExceptionId(this) {
                Ok(ok__) => {
                    pexceptionid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DetachErrorInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, perrorinfo: *mut ImgErrorInfo) -> windows_core::HRESULT
        where
            Identity: IImgErrorInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IImgErrorInfo_Impl::DetachErrorInfo(this, core::mem::transmute_copy(&perrorinfo)).into()
        }
        Self {
            base__: super::super::System::Com::IErrorInfo_Vtbl::new::<Identity, OFFSET>(),
            GetDeveloperDescription: GetDeveloperDescription::<Identity, OFFSET>,
            GetUserErrorId: GetUserErrorId::<Identity, OFFSET>,
            GetUserParameterCount: GetUserParameterCount::<Identity, OFFSET>,
            GetUserParameter: GetUserParameter::<Identity, OFFSET>,
            GetUserFallback: GetUserFallback::<Identity, OFFSET>,
            GetExceptionId: GetExceptionId::<Identity, OFFSET>,
            DetachErrorInfo: DetachErrorInfo::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IImgErrorInfo as windows_core::Interface>::IID || iid == &<super::super::System::Com::IErrorInfo as windows_core::Interface>::IID
    }
}
pub trait IInterFilterCommunicator_Impl: Sized {
    fn RequestReader(&self, ppireader: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn RequestWriter(&self, ppiwriter: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IInterFilterCommunicator {}
impl IInterFilterCommunicator_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IInterFilterCommunicator_Vtbl
    where
        Identity: IInterFilterCommunicator_Impl,
    {
        unsafe extern "system" fn RequestReader<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppireader: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IInterFilterCommunicator_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInterFilterCommunicator_Impl::RequestReader(this, core::mem::transmute_copy(&ppireader)).into()
        }
        unsafe extern "system" fn RequestWriter<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppiwriter: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IInterFilterCommunicator_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInterFilterCommunicator_Impl::RequestWriter(this, core::mem::transmute_copy(&ppiwriter)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            RequestReader: RequestReader::<Identity, OFFSET>,
            RequestWriter: RequestWriter::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IInterFilterCommunicator as windows_core::Interface>::IID
    }
}
pub trait IPartBase_Impl: Sized {
    fn GetUri(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetStream(&self) -> windows_core::Result<IPrintReadStream>;
    fn GetPartCompression(&self) -> windows_core::Result<EXpsCompressionOptions>;
    fn SetPartCompression(&self, compression: EXpsCompressionOptions) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IPartBase {}
impl IPartBase_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IPartBase_Vtbl
    where
        Identity: IPartBase_Impl,
    {
        unsafe extern "system" fn GetUri<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, uri: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IPartBase_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPartBase_Impl::GetUri(this) {
                Ok(ok__) => {
                    uri.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStream<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppstream: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPartBase_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPartBase_Impl::GetStream(this) {
                Ok(ok__) => {
                    ppstream.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPartCompression<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcompression: *mut EXpsCompressionOptions) -> windows_core::HRESULT
        where
            Identity: IPartBase_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPartBase_Impl::GetPartCompression(this) {
                Ok(ok__) => {
                    pcompression.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPartCompression<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, compression: EXpsCompressionOptions) -> windows_core::HRESULT
        where
            Identity: IPartBase_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPartBase_Impl::SetPartCompression(this, core::mem::transmute_copy(&compression)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetUri: GetUri::<Identity, OFFSET>,
            GetStream: GetStream::<Identity, OFFSET>,
            GetPartCompression: GetPartCompression::<Identity, OFFSET>,
            SetPartCompression: SetPartCompression::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPartBase as windows_core::Interface>::IID
    }
}
pub trait IPartColorProfile_Impl: Sized + IPartBase_Impl {}
impl windows_core::RuntimeName for IPartColorProfile {}
impl IPartColorProfile_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IPartColorProfile_Vtbl
    where
        Identity: IPartColorProfile_Impl,
    {
        Self { base__: IPartBase_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPartColorProfile as windows_core::Interface>::IID || iid == &<IPartBase as windows_core::Interface>::IID
    }
}
pub trait IPartDiscardControl_Impl: Sized {
    fn GetDiscardProperties(&self, urisentinelpage: *mut windows_core::BSTR, uriparttodiscard: *mut windows_core::BSTR) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IPartDiscardControl {}
impl IPartDiscardControl_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IPartDiscardControl_Vtbl
    where
        Identity: IPartDiscardControl_Impl,
    {
        unsafe extern "system" fn GetDiscardProperties<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, urisentinelpage: *mut core::mem::MaybeUninit<windows_core::BSTR>, uriparttodiscard: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IPartDiscardControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPartDiscardControl_Impl::GetDiscardProperties(this, core::mem::transmute_copy(&urisentinelpage), core::mem::transmute_copy(&uriparttodiscard)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetDiscardProperties: GetDiscardProperties::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPartDiscardControl as windows_core::Interface>::IID
    }
}
pub trait IPartFont_Impl: Sized + IPartBase_Impl {
    fn GetFontProperties(&self, pcontenttype: *mut windows_core::BSTR, pfontoptions: *mut EXpsFontOptions) -> windows_core::Result<()>;
    fn SetFontContent(&self, pcontenttype: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn SetFontOptions(&self, options: EXpsFontOptions) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IPartFont {}
impl IPartFont_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IPartFont_Vtbl
    where
        Identity: IPartFont_Impl,
    {
        unsafe extern "system" fn GetFontProperties<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcontenttype: *mut core::mem::MaybeUninit<windows_core::BSTR>, pfontoptions: *mut EXpsFontOptions) -> windows_core::HRESULT
        where
            Identity: IPartFont_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPartFont_Impl::GetFontProperties(this, core::mem::transmute_copy(&pcontenttype), core::mem::transmute_copy(&pfontoptions)).into()
        }
        unsafe extern "system" fn SetFontContent<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcontenttype: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IPartFont_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPartFont_Impl::SetFontContent(this, core::mem::transmute(&pcontenttype)).into()
        }
        unsafe extern "system" fn SetFontOptions<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, options: EXpsFontOptions) -> windows_core::HRESULT
        where
            Identity: IPartFont_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPartFont_Impl::SetFontOptions(this, core::mem::transmute_copy(&options)).into()
        }
        Self {
            base__: IPartBase_Vtbl::new::<Identity, OFFSET>(),
            GetFontProperties: GetFontProperties::<Identity, OFFSET>,
            SetFontContent: SetFontContent::<Identity, OFFSET>,
            SetFontOptions: SetFontOptions::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPartFont as windows_core::Interface>::IID || iid == &<IPartBase as windows_core::Interface>::IID
    }
}
pub trait IPartFont2_Impl: Sized + IPartFont_Impl {
    fn GetFontRestriction(&self) -> windows_core::Result<EXpsFontRestriction>;
}
impl windows_core::RuntimeName for IPartFont2 {}
impl IPartFont2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IPartFont2_Vtbl
    where
        Identity: IPartFont2_Impl,
    {
        unsafe extern "system" fn GetFontRestriction<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, prestriction: *mut EXpsFontRestriction) -> windows_core::HRESULT
        where
            Identity: IPartFont2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPartFont2_Impl::GetFontRestriction(this) {
                Ok(ok__) => {
                    prestriction.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: IPartFont_Vtbl::new::<Identity, OFFSET>(), GetFontRestriction: GetFontRestriction::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPartFont2 as windows_core::Interface>::IID || iid == &<IPartBase as windows_core::Interface>::IID || iid == &<IPartFont as windows_core::Interface>::IID
    }
}
pub trait IPartImage_Impl: Sized + IPartBase_Impl {
    fn GetImageProperties(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetImageContent(&self, pcontenttype: &windows_core::PCWSTR) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IPartImage {}
impl IPartImage_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IPartImage_Vtbl
    where
        Identity: IPartImage_Impl,
    {
        unsafe extern "system" fn GetImageProperties<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcontenttype: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IPartImage_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPartImage_Impl::GetImageProperties(this) {
                Ok(ok__) => {
                    pcontenttype.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetImageContent<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcontenttype: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IPartImage_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPartImage_Impl::SetImageContent(this, core::mem::transmute(&pcontenttype)).into()
        }
        Self {
            base__: IPartBase_Vtbl::new::<Identity, OFFSET>(),
            GetImageProperties: GetImageProperties::<Identity, OFFSET>,
            SetImageContent: SetImageContent::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPartImage as windows_core::Interface>::IID || iid == &<IPartBase as windows_core::Interface>::IID
    }
}
pub trait IPartPrintTicket_Impl: Sized + IPartBase_Impl {}
impl windows_core::RuntimeName for IPartPrintTicket {}
impl IPartPrintTicket_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IPartPrintTicket_Vtbl
    where
        Identity: IPartPrintTicket_Impl,
    {
        Self { base__: IPartBase_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPartPrintTicket as windows_core::Interface>::IID || iid == &<IPartBase as windows_core::Interface>::IID
    }
}
pub trait IPartResourceDictionary_Impl: Sized + IPartBase_Impl {}
impl windows_core::RuntimeName for IPartResourceDictionary {}
impl IPartResourceDictionary_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IPartResourceDictionary_Vtbl
    where
        Identity: IPartResourceDictionary_Impl,
    {
        Self { base__: IPartBase_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPartResourceDictionary as windows_core::Interface>::IID || iid == &<IPartBase as windows_core::Interface>::IID
    }
}
pub trait IPartThumbnail_Impl: Sized + IPartBase_Impl {
    fn GetThumbnailProperties(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetThumbnailContent(&self, pcontenttype: &windows_core::PCWSTR) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IPartThumbnail {}
impl IPartThumbnail_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IPartThumbnail_Vtbl
    where
        Identity: IPartThumbnail_Impl,
    {
        unsafe extern "system" fn GetThumbnailProperties<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcontenttype: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IPartThumbnail_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPartThumbnail_Impl::GetThumbnailProperties(this) {
                Ok(ok__) => {
                    pcontenttype.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetThumbnailContent<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcontenttype: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IPartThumbnail_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPartThumbnail_Impl::SetThumbnailContent(this, core::mem::transmute(&pcontenttype)).into()
        }
        Self {
            base__: IPartBase_Vtbl::new::<Identity, OFFSET>(),
            GetThumbnailProperties: GetThumbnailProperties::<Identity, OFFSET>,
            SetThumbnailContent: SetThumbnailContent::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPartThumbnail as windows_core::Interface>::IID || iid == &<IPartBase as windows_core::Interface>::IID
    }
}
pub trait IPrintAsyncCookie_Impl: Sized {
    fn FinishAsyncCall(&self, param0: windows_core::HRESULT) -> windows_core::Result<()>;
    fn CancelAsyncCall(&self, param0: windows_core::HRESULT) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IPrintAsyncCookie {}
impl IPrintAsyncCookie_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IPrintAsyncCookie_Vtbl
    where
        Identity: IPrintAsyncCookie_Impl,
    {
        unsafe extern "system" fn FinishAsyncCall<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: windows_core::HRESULT) -> windows_core::HRESULT
        where
            Identity: IPrintAsyncCookie_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPrintAsyncCookie_Impl::FinishAsyncCall(this, core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn CancelAsyncCall<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: windows_core::HRESULT) -> windows_core::HRESULT
        where
            Identity: IPrintAsyncCookie_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPrintAsyncCookie_Impl::CancelAsyncCall(this, core::mem::transmute_copy(&param0)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            FinishAsyncCall: FinishAsyncCall::<Identity, OFFSET>,
            CancelAsyncCall: CancelAsyncCall::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPrintAsyncCookie as windows_core::Interface>::IID
    }
}
pub trait IPrintAsyncNewChannelCookie_Impl: Sized + IPrintAsyncCookie_Impl {
    fn FinishAsyncCallWithData(&self, param0: *const Option<IPrintAsyncNotifyChannel>, param1: u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IPrintAsyncNewChannelCookie {}
impl IPrintAsyncNewChannelCookie_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IPrintAsyncNewChannelCookie_Vtbl
    where
        Identity: IPrintAsyncNewChannelCookie_Impl,
    {
        unsafe extern "system" fn FinishAsyncCallWithData<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *const *mut core::ffi::c_void, param1: u32) -> windows_core::HRESULT
        where
            Identity: IPrintAsyncNewChannelCookie_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPrintAsyncNewChannelCookie_Impl::FinishAsyncCallWithData(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1)).into()
        }
        Self { base__: IPrintAsyncCookie_Vtbl::new::<Identity, OFFSET>(), FinishAsyncCallWithData: FinishAsyncCallWithData::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPrintAsyncNewChannelCookie as windows_core::Interface>::IID || iid == &<IPrintAsyncCookie as windows_core::Interface>::IID
    }
}
pub trait IPrintAsyncNotify_Impl: Sized {
    fn CreatePrintAsyncNotifyChannel(&self, param0: u32, param1: *const windows_core::GUID, param2: PrintAsyncNotifyUserFilter, param3: PrintAsyncNotifyConversationStyle, param4: Option<&IPrintAsyncNotifyCallback>) -> windows_core::Result<IPrintAsyncNotifyChannel>;
    fn CreatePrintAsyncNotifyRegistration(&self, param0: *const windows_core::GUID, param1: PrintAsyncNotifyUserFilter, param2: PrintAsyncNotifyConversationStyle, param3: Option<&IPrintAsyncNotifyCallback>) -> windows_core::Result<IPrintAsyncNotifyRegistration>;
}
impl windows_core::RuntimeName for IPrintAsyncNotify {}
impl IPrintAsyncNotify_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IPrintAsyncNotify_Vtbl
    where
        Identity: IPrintAsyncNotify_Impl,
    {
        unsafe extern "system" fn CreatePrintAsyncNotifyChannel<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32, param1: *const windows_core::GUID, param2: PrintAsyncNotifyUserFilter, param3: PrintAsyncNotifyConversationStyle, param4: *mut core::ffi::c_void, param5: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPrintAsyncNotify_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPrintAsyncNotify_Impl::CreatePrintAsyncNotifyChannel(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2), core::mem::transmute_copy(&param3), windows_core::from_raw_borrowed(&param4)) {
                Ok(ok__) => {
                    param5.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreatePrintAsyncNotifyRegistration<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *const windows_core::GUID, param1: PrintAsyncNotifyUserFilter, param2: PrintAsyncNotifyConversationStyle, param3: *mut core::ffi::c_void, param4: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPrintAsyncNotify_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPrintAsyncNotify_Impl::CreatePrintAsyncNotifyRegistration(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2), windows_core::from_raw_borrowed(&param3)) {
                Ok(ok__) => {
                    param4.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreatePrintAsyncNotifyChannel: CreatePrintAsyncNotifyChannel::<Identity, OFFSET>,
            CreatePrintAsyncNotifyRegistration: CreatePrintAsyncNotifyRegistration::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPrintAsyncNotify as windows_core::Interface>::IID
    }
}
pub trait IPrintAsyncNotifyCallback_Impl: Sized {
    fn OnEventNotify(&self, pchannel: Option<&IPrintAsyncNotifyChannel>, pdata: Option<&IPrintAsyncNotifyDataObject>) -> windows_core::Result<()>;
    fn ChannelClosed(&self, pchannel: Option<&IPrintAsyncNotifyChannel>, pdata: Option<&IPrintAsyncNotifyDataObject>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IPrintAsyncNotifyCallback {}
impl IPrintAsyncNotifyCallback_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IPrintAsyncNotifyCallback_Vtbl
    where
        Identity: IPrintAsyncNotifyCallback_Impl,
    {
        unsafe extern "system" fn OnEventNotify<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pchannel: *mut core::ffi::c_void, pdata: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPrintAsyncNotifyCallback_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPrintAsyncNotifyCallback_Impl::OnEventNotify(this, windows_core::from_raw_borrowed(&pchannel), windows_core::from_raw_borrowed(&pdata)).into()
        }
        unsafe extern "system" fn ChannelClosed<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pchannel: *mut core::ffi::c_void, pdata: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPrintAsyncNotifyCallback_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPrintAsyncNotifyCallback_Impl::ChannelClosed(this, windows_core::from_raw_borrowed(&pchannel), windows_core::from_raw_borrowed(&pdata)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnEventNotify: OnEventNotify::<Identity, OFFSET>,
            ChannelClosed: ChannelClosed::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPrintAsyncNotifyCallback as windows_core::Interface>::IID
    }
}
pub trait IPrintAsyncNotifyChannel_Impl: Sized {
    fn SendNotification(&self, pdata: Option<&IPrintAsyncNotifyDataObject>) -> windows_core::Result<()>;
    fn CloseChannel(&self, pdata: Option<&IPrintAsyncNotifyDataObject>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IPrintAsyncNotifyChannel {}
impl IPrintAsyncNotifyChannel_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IPrintAsyncNotifyChannel_Vtbl
    where
        Identity: IPrintAsyncNotifyChannel_Impl,
    {
        unsafe extern "system" fn SendNotification<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdata: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPrintAsyncNotifyChannel_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPrintAsyncNotifyChannel_Impl::SendNotification(this, windows_core::from_raw_borrowed(&pdata)).into()
        }
        unsafe extern "system" fn CloseChannel<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdata: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPrintAsyncNotifyChannel_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPrintAsyncNotifyChannel_Impl::CloseChannel(this, windows_core::from_raw_borrowed(&pdata)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SendNotification: SendNotification::<Identity, OFFSET>,
            CloseChannel: CloseChannel::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPrintAsyncNotifyChannel as windows_core::Interface>::IID
    }
}
pub trait IPrintAsyncNotifyDataObject_Impl: Sized {
    fn AcquireData(&self, ppnotificationdata: *mut *mut u8, psize: *mut u32, ppschema: *mut *mut windows_core::GUID) -> windows_core::Result<()>;
    fn ReleaseData(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IPrintAsyncNotifyDataObject {}
impl IPrintAsyncNotifyDataObject_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IPrintAsyncNotifyDataObject_Vtbl
    where
        Identity: IPrintAsyncNotifyDataObject_Impl,
    {
        unsafe extern "system" fn AcquireData<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppnotificationdata: *mut *mut u8, psize: *mut u32, ppschema: *mut *mut windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IPrintAsyncNotifyDataObject_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPrintAsyncNotifyDataObject_Impl::AcquireData(this, core::mem::transmute_copy(&ppnotificationdata), core::mem::transmute_copy(&psize), core::mem::transmute_copy(&ppschema)).into()
        }
        unsafe extern "system" fn ReleaseData<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPrintAsyncNotifyDataObject_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPrintAsyncNotifyDataObject_Impl::ReleaseData(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AcquireData: AcquireData::<Identity, OFFSET>,
            ReleaseData: ReleaseData::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPrintAsyncNotifyDataObject as windows_core::Interface>::IID
    }
}
pub trait IPrintAsyncNotifyRegistration_Impl: Sized {
    fn RegisterForNotifications(&self) -> windows_core::Result<()>;
    fn UnregisterForNotifications(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IPrintAsyncNotifyRegistration {}
impl IPrintAsyncNotifyRegistration_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IPrintAsyncNotifyRegistration_Vtbl
    where
        Identity: IPrintAsyncNotifyRegistration_Impl,
    {
        unsafe extern "system" fn RegisterForNotifications<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPrintAsyncNotifyRegistration_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPrintAsyncNotifyRegistration_Impl::RegisterForNotifications(this).into()
        }
        unsafe extern "system" fn UnregisterForNotifications<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPrintAsyncNotifyRegistration_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPrintAsyncNotifyRegistration_Impl::UnregisterForNotifications(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            RegisterForNotifications: RegisterForNotifications::<Identity, OFFSET>,
            UnregisterForNotifications: UnregisterForNotifications::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPrintAsyncNotifyRegistration as windows_core::Interface>::IID
    }
}
pub trait IPrintAsyncNotifyServerReferral_Impl: Sized {
    fn GetServerReferral(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn AsyncGetServerReferral(&self, param0: Option<&IAsyncGetSrvReferralCookie>) -> windows_core::Result<()>;
    fn SetServerReferral(&self, prmtserverreferral: &windows_core::PCWSTR) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IPrintAsyncNotifyServerReferral {}
impl IPrintAsyncNotifyServerReferral_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IPrintAsyncNotifyServerReferral_Vtbl
    where
        Identity: IPrintAsyncNotifyServerReferral_Impl,
    {
        unsafe extern "system" fn GetServerReferral<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IPrintAsyncNotifyServerReferral_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPrintAsyncNotifyServerReferral_Impl::GetServerReferral(this) {
                Ok(ok__) => {
                    param0.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AsyncGetServerReferral<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPrintAsyncNotifyServerReferral_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPrintAsyncNotifyServerReferral_Impl::AsyncGetServerReferral(this, windows_core::from_raw_borrowed(&param0)).into()
        }
        unsafe extern "system" fn SetServerReferral<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, prmtserverreferral: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IPrintAsyncNotifyServerReferral_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPrintAsyncNotifyServerReferral_Impl::SetServerReferral(this, core::mem::transmute(&prmtserverreferral)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetServerReferral: GetServerReferral::<Identity, OFFSET>,
            AsyncGetServerReferral: AsyncGetServerReferral::<Identity, OFFSET>,
            SetServerReferral: SetServerReferral::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPrintAsyncNotifyServerReferral as windows_core::Interface>::IID
    }
}
pub trait IPrintBidiAsyncNotifyRegistration_Impl: Sized + IPrintAsyncNotifyRegistration_Impl {
    fn AsyncGetNewChannel(&self, param0: Option<&IPrintAsyncNewChannelCookie>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IPrintBidiAsyncNotifyRegistration {}
impl IPrintBidiAsyncNotifyRegistration_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IPrintBidiAsyncNotifyRegistration_Vtbl
    where
        Identity: IPrintBidiAsyncNotifyRegistration_Impl,
    {
        unsafe extern "system" fn AsyncGetNewChannel<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPrintBidiAsyncNotifyRegistration_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPrintBidiAsyncNotifyRegistration_Impl::AsyncGetNewChannel(this, windows_core::from_raw_borrowed(&param0)).into()
        }
        Self { base__: IPrintAsyncNotifyRegistration_Vtbl::new::<Identity, OFFSET>(), AsyncGetNewChannel: AsyncGetNewChannel::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPrintBidiAsyncNotifyRegistration as windows_core::Interface>::IID || iid == &<IPrintAsyncNotifyRegistration as windows_core::Interface>::IID
    }
}
pub trait IPrintClassObjectFactory_Impl: Sized {
    fn GetPrintClassObject(&self, pszprintername: &windows_core::PCWSTR, riid: *const windows_core::GUID, ppnewobject: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IPrintClassObjectFactory {}
impl IPrintClassObjectFactory_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IPrintClassObjectFactory_Vtbl
    where
        Identity: IPrintClassObjectFactory_Impl,
    {
        unsafe extern "system" fn GetPrintClassObject<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszprintername: windows_core::PCWSTR, riid: *const windows_core::GUID, ppnewobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPrintClassObjectFactory_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPrintClassObjectFactory_Impl::GetPrintClassObject(this, core::mem::transmute(&pszprintername), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppnewobject)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetPrintClassObject: GetPrintClassObject::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPrintClassObjectFactory as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
pub trait IPrintCoreHelper_Impl: Sized {
    fn GetOption(&self, pdevmode: *const super::Gdi::DEVMODEA, cbsize: u32, pszfeaturerequested: &windows_core::PCSTR) -> windows_core::Result<windows_core::PCSTR>;
    fn SetOptions(&self, pdevmode: *mut super::Gdi::DEVMODEA, cbsize: u32, bresolveconflicts: super::super::Foundation::BOOL, pfopairs: *const PRINT_FEATURE_OPTION, cpairs: u32, pcpairswritten: *mut u32, pdwresult: *mut u32) -> windows_core::Result<()>;
    fn EnumConstrainedOptions(&self, pdevmode: *const super::Gdi::DEVMODEA, cbsize: u32, pszfeaturekeyword: &windows_core::PCSTR, pconstrainedoptionlist: *const *const *const windows_core::PCSTR, pdwnumoptions: *mut u32) -> windows_core::Result<()>;
    fn WhyConstrained(&self, pdevmode: *const super::Gdi::DEVMODEA, cbsize: u32, pszfeaturekeyword: &windows_core::PCSTR, pszoptionkeyword: &windows_core::PCSTR, ppfoconstraints: *mut *mut PRINT_FEATURE_OPTION, pdwnumoptions: *mut u32) -> windows_core::Result<()>;
    fn EnumFeatures(&self, pfeaturelist: *mut *mut *mut windows_core::PCSTR, pdwnumfeatures: *mut u32) -> windows_core::Result<()>;
    fn EnumOptions(&self, pszfeaturekeyword: &windows_core::PCSTR, poptionlist: *mut *mut *mut windows_core::PCSTR, pdwnumoptions: *mut u32) -> windows_core::Result<()>;
    fn GetFontSubstitution(&self, psztruetypefontname: &windows_core::PCWSTR, ppszdevfontname: *const windows_core::PCWSTR) -> windows_core::Result<()>;
    fn SetFontSubstitution(&self, psztruetypefontname: &windows_core::PCWSTR, pszdevfontname: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn CreateInstanceOfMSXMLObject(&self, rclsid: *const windows_core::GUID, punkouter: Option<&windows_core::IUnknown>, dwclscontext: u32, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl windows_core::RuntimeName for IPrintCoreHelper {}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl IPrintCoreHelper_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IPrintCoreHelper_Vtbl
    where
        Identity: IPrintCoreHelper_Impl,
    {
        unsafe extern "system" fn GetOption<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdevmode: *const super::Gdi::DEVMODEA, cbsize: u32, pszfeaturerequested: windows_core::PCSTR, ppszoption: *mut windows_core::PCSTR) -> windows_core::HRESULT
        where
            Identity: IPrintCoreHelper_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPrintCoreHelper_Impl::GetOption(this, core::mem::transmute_copy(&pdevmode), core::mem::transmute_copy(&cbsize), core::mem::transmute(&pszfeaturerequested)) {
                Ok(ok__) => {
                    ppszoption.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOptions<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdevmode: *mut super::Gdi::DEVMODEA, cbsize: u32, bresolveconflicts: super::super::Foundation::BOOL, pfopairs: *const PRINT_FEATURE_OPTION, cpairs: u32, pcpairswritten: *mut u32, pdwresult: *mut u32) -> windows_core::HRESULT
        where
            Identity: IPrintCoreHelper_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPrintCoreHelper_Impl::SetOptions(this, core::mem::transmute_copy(&pdevmode), core::mem::transmute_copy(&cbsize), core::mem::transmute_copy(&bresolveconflicts), core::mem::transmute_copy(&pfopairs), core::mem::transmute_copy(&cpairs), core::mem::transmute_copy(&pcpairswritten), core::mem::transmute_copy(&pdwresult)).into()
        }
        unsafe extern "system" fn EnumConstrainedOptions<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdevmode: *const super::Gdi::DEVMODEA, cbsize: u32, pszfeaturekeyword: windows_core::PCSTR, pconstrainedoptionlist: *const *const *const windows_core::PCSTR, pdwnumoptions: *mut u32) -> windows_core::HRESULT
        where
            Identity: IPrintCoreHelper_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPrintCoreHelper_Impl::EnumConstrainedOptions(this, core::mem::transmute_copy(&pdevmode), core::mem::transmute_copy(&cbsize), core::mem::transmute(&pszfeaturekeyword), core::mem::transmute_copy(&pconstrainedoptionlist), core::mem::transmute_copy(&pdwnumoptions)).into()
        }
        unsafe extern "system" fn WhyConstrained<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdevmode: *const super::Gdi::DEVMODEA, cbsize: u32, pszfeaturekeyword: windows_core::PCSTR, pszoptionkeyword: windows_core::PCSTR, ppfoconstraints: *mut *mut PRINT_FEATURE_OPTION, pdwnumoptions: *mut u32) -> windows_core::HRESULT
        where
            Identity: IPrintCoreHelper_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPrintCoreHelper_Impl::WhyConstrained(this, core::mem::transmute_copy(&pdevmode), core::mem::transmute_copy(&cbsize), core::mem::transmute(&pszfeaturekeyword), core::mem::transmute(&pszoptionkeyword), core::mem::transmute_copy(&ppfoconstraints), core::mem::transmute_copy(&pdwnumoptions)).into()
        }
        unsafe extern "system" fn EnumFeatures<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfeaturelist: *mut *mut *mut windows_core::PCSTR, pdwnumfeatures: *mut u32) -> windows_core::HRESULT
        where
            Identity: IPrintCoreHelper_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPrintCoreHelper_Impl::EnumFeatures(this, core::mem::transmute_copy(&pfeaturelist), core::mem::transmute_copy(&pdwnumfeatures)).into()
        }
        unsafe extern "system" fn EnumOptions<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszfeaturekeyword: windows_core::PCSTR, poptionlist: *mut *mut *mut windows_core::PCSTR, pdwnumoptions: *mut u32) -> windows_core::HRESULT
        where
            Identity: IPrintCoreHelper_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPrintCoreHelper_Impl::EnumOptions(this, core::mem::transmute(&pszfeaturekeyword), core::mem::transmute_copy(&poptionlist), core::mem::transmute_copy(&pdwnumoptions)).into()
        }
        unsafe extern "system" fn GetFontSubstitution<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, psztruetypefontname: windows_core::PCWSTR, ppszdevfontname: *const windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IPrintCoreHelper_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPrintCoreHelper_Impl::GetFontSubstitution(this, core::mem::transmute(&psztruetypefontname), core::mem::transmute_copy(&ppszdevfontname)).into()
        }
        unsafe extern "system" fn SetFontSubstitution<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, psztruetypefontname: windows_core::PCWSTR, pszdevfontname: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IPrintCoreHelper_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPrintCoreHelper_Impl::SetFontSubstitution(this, core::mem::transmute(&psztruetypefontname), core::mem::transmute(&pszdevfontname)).into()
        }
        unsafe extern "system" fn CreateInstanceOfMSXMLObject<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, rclsid: *const windows_core::GUID, punkouter: *mut core::ffi::c_void, dwclscontext: u32, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPrintCoreHelper_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPrintCoreHelper_Impl::CreateInstanceOfMSXMLObject(this, core::mem::transmute_copy(&rclsid), windows_core::from_raw_borrowed(&punkouter), core::mem::transmute_copy(&dwclscontext), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppv)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetOption: GetOption::<Identity, OFFSET>,
            SetOptions: SetOptions::<Identity, OFFSET>,
            EnumConstrainedOptions: EnumConstrainedOptions::<Identity, OFFSET>,
            WhyConstrained: WhyConstrained::<Identity, OFFSET>,
            EnumFeatures: EnumFeatures::<Identity, OFFSET>,
            EnumOptions: EnumOptions::<Identity, OFFSET>,
            GetFontSubstitution: GetFontSubstitution::<Identity, OFFSET>,
            SetFontSubstitution: SetFontSubstitution::<Identity, OFFSET>,
            CreateInstanceOfMSXMLObject: CreateInstanceOfMSXMLObject::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPrintCoreHelper as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
pub trait IPrintCoreHelperPS_Impl: Sized + IPrintCoreHelper_Impl {
    fn GetGlobalAttribute(&self, pszattribute: &windows_core::PCSTR, pdwdatatype: *mut u32, ppbdata: *mut *mut u8, pcbsize: *mut u32) -> windows_core::Result<()>;
    fn GetFeatureAttribute(&self, pszfeaturekeyword: &windows_core::PCSTR, pszattribute: &windows_core::PCSTR, pdwdatatype: *mut u32, ppbdata: *mut *mut u8, pcbsize: *mut u32) -> windows_core::Result<()>;
    fn GetOptionAttribute(&self, pszfeaturekeyword: &windows_core::PCSTR, pszoptionkeyword: &windows_core::PCSTR, pszattribute: &windows_core::PCSTR, pdwdatatype: *mut u32, ppbdata: *mut *mut u8, pcbsize: *mut u32) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl windows_core::RuntimeName for IPrintCoreHelperPS {}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl IPrintCoreHelperPS_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IPrintCoreHelperPS_Vtbl
    where
        Identity: IPrintCoreHelperPS_Impl,
    {
        unsafe extern "system" fn GetGlobalAttribute<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszattribute: windows_core::PCSTR, pdwdatatype: *mut u32, ppbdata: *mut *mut u8, pcbsize: *mut u32) -> windows_core::HRESULT
        where
            Identity: IPrintCoreHelperPS_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPrintCoreHelperPS_Impl::GetGlobalAttribute(this, core::mem::transmute(&pszattribute), core::mem::transmute_copy(&pdwdatatype), core::mem::transmute_copy(&ppbdata), core::mem::transmute_copy(&pcbsize)).into()
        }
        unsafe extern "system" fn GetFeatureAttribute<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszfeaturekeyword: windows_core::PCSTR, pszattribute: windows_core::PCSTR, pdwdatatype: *mut u32, ppbdata: *mut *mut u8, pcbsize: *mut u32) -> windows_core::HRESULT
        where
            Identity: IPrintCoreHelperPS_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPrintCoreHelperPS_Impl::GetFeatureAttribute(this, core::mem::transmute(&pszfeaturekeyword), core::mem::transmute(&pszattribute), core::mem::transmute_copy(&pdwdatatype), core::mem::transmute_copy(&ppbdata), core::mem::transmute_copy(&pcbsize)).into()
        }
        unsafe extern "system" fn GetOptionAttribute<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszfeaturekeyword: windows_core::PCSTR, pszoptionkeyword: windows_core::PCSTR, pszattribute: windows_core::PCSTR, pdwdatatype: *mut u32, ppbdata: *mut *mut u8, pcbsize: *mut u32) -> windows_core::HRESULT
        where
            Identity: IPrintCoreHelperPS_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPrintCoreHelperPS_Impl::GetOptionAttribute(this, core::mem::transmute(&pszfeaturekeyword), core::mem::transmute(&pszoptionkeyword), core::mem::transmute(&pszattribute), core::mem::transmute_copy(&pdwdatatype), core::mem::transmute_copy(&ppbdata), core::mem::transmute_copy(&pcbsize)).into()
        }
        Self {
            base__: IPrintCoreHelper_Vtbl::new::<Identity, OFFSET>(),
            GetGlobalAttribute: GetGlobalAttribute::<Identity, OFFSET>,
            GetFeatureAttribute: GetFeatureAttribute::<Identity, OFFSET>,
            GetOptionAttribute: GetOptionAttribute::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPrintCoreHelperPS as windows_core::Interface>::IID || iid == &<IPrintCoreHelper as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
pub trait IPrintCoreHelperUni_Impl: Sized + IPrintCoreHelper_Impl {
    fn CreateGDLSnapshot(&self, pdevmode: *mut super::Gdi::DEVMODEA, cbsize: u32, dwflags: u32, ppsnapshotstream: *mut Option<super::super::System::Com::IStream>) -> windows_core::Result<()>;
    fn CreateDefaultGDLSnapshot(&self, dwflags: u32) -> windows_core::Result<super::super::System::Com::IStream>;
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
impl windows_core::RuntimeName for IPrintCoreHelperUni {}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
impl IPrintCoreHelperUni_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IPrintCoreHelperUni_Vtbl
    where
        Identity: IPrintCoreHelperUni_Impl,
    {
        unsafe extern "system" fn CreateGDLSnapshot<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdevmode: *mut super::Gdi::DEVMODEA, cbsize: u32, dwflags: u32, ppsnapshotstream: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPrintCoreHelperUni_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPrintCoreHelperUni_Impl::CreateGDLSnapshot(this, core::mem::transmute_copy(&pdevmode), core::mem::transmute_copy(&cbsize), core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&ppsnapshotstream)).into()
        }
        unsafe extern "system" fn CreateDefaultGDLSnapshot<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: u32, ppsnapshotstream: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPrintCoreHelperUni_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPrintCoreHelperUni_Impl::CreateDefaultGDLSnapshot(this, core::mem::transmute_copy(&dwflags)) {
                Ok(ok__) => {
                    ppsnapshotstream.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: IPrintCoreHelper_Vtbl::new::<Identity, OFFSET>(),
            CreateGDLSnapshot: CreateGDLSnapshot::<Identity, OFFSET>,
            CreateDefaultGDLSnapshot: CreateDefaultGDLSnapshot::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPrintCoreHelperUni as windows_core::Interface>::IID || iid == &<IPrintCoreHelper as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
pub trait IPrintCoreHelperUni2_Impl: Sized + IPrintCoreHelperUni_Impl {
    fn GetNamedCommand(&self, pdevmode: *const super::Gdi::DEVMODEA, cbsize: u32, pszcommandname: &windows_core::PCWSTR, ppcommandbytes: *mut *mut u8, pcbcommandsize: *mut u32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
impl windows_core::RuntimeName for IPrintCoreHelperUni2 {}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
impl IPrintCoreHelperUni2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IPrintCoreHelperUni2_Vtbl
    where
        Identity: IPrintCoreHelperUni2_Impl,
    {
        unsafe extern "system" fn GetNamedCommand<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdevmode: *const super::Gdi::DEVMODEA, cbsize: u32, pszcommandname: windows_core::PCWSTR, ppcommandbytes: *mut *mut u8, pcbcommandsize: *mut u32) -> windows_core::HRESULT
        where
            Identity: IPrintCoreHelperUni2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPrintCoreHelperUni2_Impl::GetNamedCommand(this, core::mem::transmute_copy(&pdevmode), core::mem::transmute_copy(&cbsize), core::mem::transmute(&pszcommandname), core::mem::transmute_copy(&ppcommandbytes), core::mem::transmute_copy(&pcbcommandsize)).into()
        }
        Self { base__: IPrintCoreHelperUni_Vtbl::new::<Identity, OFFSET>(), GetNamedCommand: GetNamedCommand::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPrintCoreHelperUni2 as windows_core::Interface>::IID || iid == &<IPrintCoreHelper as windows_core::Interface>::IID || iid == &<IPrintCoreHelperUni as windows_core::Interface>::IID
    }
}
pub trait IPrintCoreUI2_Impl: Sized + IPrintOemDriverUI_Impl {
    fn GetOptions(&self, poemuiobj: *const OEMUIOBJ, dwflags: u32, pmszfeaturesrequested: *const i8, cbin: u32, pmszfeatureoptionbuf: windows_core::PSTR, cbsize: u32, pcbneeded: *mut u32) -> windows_core::Result<()>;
    fn SetOptions(&self, poemuiobj: *const OEMUIOBJ, dwflags: u32, pmszfeatureoptionbuf: *const i8, cbin: u32) -> windows_core::Result<u32>;
    fn EnumConstrainedOptions(&self, poemuiobj: *const OEMUIOBJ, dwflags: u32, pszfeaturekeyword: &windows_core::PCSTR, pmszconstrainedoptionlist: windows_core::PSTR, cbsize: u32, pcbneeded: *mut u32) -> windows_core::Result<()>;
    fn WhyConstrained(&self, poemuiobj: *const OEMUIOBJ, dwflags: u32, pszfeaturekeyword: &windows_core::PCSTR, pszoptionkeyword: &windows_core::PCSTR, pmszreasonlist: windows_core::PSTR, cbsize: u32, pcbneeded: *mut u32) -> windows_core::Result<()>;
    fn GetGlobalAttribute(&self, poemuiobj: *const OEMUIOBJ, dwflags: u32, pszattribute: &windows_core::PCSTR, pdwdatatype: *mut u32, pbdata: *mut u8, cbsize: u32, pcbneeded: *mut u32) -> windows_core::Result<()>;
    fn GetFeatureAttribute(&self, poemuiobj: *const OEMUIOBJ, dwflags: u32, pszfeaturekeyword: &windows_core::PCSTR, pszattribute: &windows_core::PCSTR, pdwdatatype: *mut u32, pbdata: *mut u8, cbsize: u32, pcbneeded: *mut u32) -> windows_core::Result<()>;
    fn GetOptionAttribute(&self, poemuiobj: *const OEMUIOBJ, dwflags: u32, pszfeaturekeyword: &windows_core::PCSTR, pszoptionkeyword: &windows_core::PCSTR, pszattribute: &windows_core::PCSTR, pdwdatatype: *mut u32, pbdata: *mut u8, cbsize: u32, pcbneeded: *mut u32) -> windows_core::Result<()>;
    fn EnumFeatures(&self, poemuiobj: *const OEMUIOBJ, dwflags: u32, pmszfeaturelist: windows_core::PSTR, cbsize: u32, pcbneeded: *mut u32) -> windows_core::Result<()>;
    fn EnumOptions(&self, poemuiobj: *const OEMUIOBJ, dwflags: u32, pszfeaturekeyword: &windows_core::PCSTR, pmszoptionlist: windows_core::PSTR, cbsize: u32, pcbneeded: *mut u32) -> windows_core::Result<()>;
    fn QuerySimulationSupport(&self, hprinter: super::super::Foundation::HANDLE, dwlevel: u32, pcaps: *mut u8, cbsize: u32, pcbneeded: *mut u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IPrintCoreUI2 {}
impl IPrintCoreUI2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IPrintCoreUI2_Vtbl
    where
        Identity: IPrintCoreUI2_Impl,
    {
        unsafe extern "system" fn GetOptions<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, poemuiobj: *const OEMUIOBJ, dwflags: u32, pmszfeaturesrequested: *const i8, cbin: u32, pmszfeatureoptionbuf: windows_core::PSTR, cbsize: u32, pcbneeded: *mut u32) -> windows_core::HRESULT
        where
            Identity: IPrintCoreUI2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPrintCoreUI2_Impl::GetOptions(this, core::mem::transmute_copy(&poemuiobj), core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&pmszfeaturesrequested), core::mem::transmute_copy(&cbin), core::mem::transmute_copy(&pmszfeatureoptionbuf), core::mem::transmute_copy(&cbsize), core::mem::transmute_copy(&pcbneeded)).into()
        }
        unsafe extern "system" fn SetOptions<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, poemuiobj: *const OEMUIOBJ, dwflags: u32, pmszfeatureoptionbuf: *const i8, cbin: u32, pdwresult: *mut u32) -> windows_core::HRESULT
        where
            Identity: IPrintCoreUI2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPrintCoreUI2_Impl::SetOptions(this, core::mem::transmute_copy(&poemuiobj), core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&pmszfeatureoptionbuf), core::mem::transmute_copy(&cbin)) {
                Ok(ok__) => {
                    pdwresult.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumConstrainedOptions<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, poemuiobj: *const OEMUIOBJ, dwflags: u32, pszfeaturekeyword: windows_core::PCSTR, pmszconstrainedoptionlist: windows_core::PSTR, cbsize: u32, pcbneeded: *mut u32) -> windows_core::HRESULT
        where
            Identity: IPrintCoreUI2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPrintCoreUI2_Impl::EnumConstrainedOptions(this, core::mem::transmute_copy(&poemuiobj), core::mem::transmute_copy(&dwflags), core::mem::transmute(&pszfeaturekeyword), core::mem::transmute_copy(&pmszconstrainedoptionlist), core::mem::transmute_copy(&cbsize), core::mem::transmute_copy(&pcbneeded)).into()
        }
        unsafe extern "system" fn WhyConstrained<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, poemuiobj: *const OEMUIOBJ, dwflags: u32, pszfeaturekeyword: windows_core::PCSTR, pszoptionkeyword: windows_core::PCSTR, pmszreasonlist: windows_core::PSTR, cbsize: u32, pcbneeded: *mut u32) -> windows_core::HRESULT
        where
            Identity: IPrintCoreUI2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPrintCoreUI2_Impl::WhyConstrained(this, core::mem::transmute_copy(&poemuiobj), core::mem::transmute_copy(&dwflags), core::mem::transmute(&pszfeaturekeyword), core::mem::transmute(&pszoptionkeyword), core::mem::transmute_copy(&pmszreasonlist), core::mem::transmute_copy(&cbsize), core::mem::transmute_copy(&pcbneeded)).into()
        }
        unsafe extern "system" fn GetGlobalAttribute<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, poemuiobj: *const OEMUIOBJ, dwflags: u32, pszattribute: windows_core::PCSTR, pdwdatatype: *mut u32, pbdata: *mut u8, cbsize: u32, pcbneeded: *mut u32) -> windows_core::HRESULT
        where
            Identity: IPrintCoreUI2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPrintCoreUI2_Impl::GetGlobalAttribute(this, core::mem::transmute_copy(&poemuiobj), core::mem::transmute_copy(&dwflags), core::mem::transmute(&pszattribute), core::mem::transmute_copy(&pdwdatatype), core::mem::transmute_copy(&pbdata), core::mem::transmute_copy(&cbsize), core::mem::transmute_copy(&pcbneeded)).into()
        }
        unsafe extern "system" fn GetFeatureAttribute<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, poemuiobj: *const OEMUIOBJ, dwflags: u32, pszfeaturekeyword: windows_core::PCSTR, pszattribute: windows_core::PCSTR, pdwdatatype: *mut u32, pbdata: *mut u8, cbsize: u32, pcbneeded: *mut u32) -> windows_core::HRESULT
        where
            Identity: IPrintCoreUI2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPrintCoreUI2_Impl::GetFeatureAttribute(this, core::mem::transmute_copy(&poemuiobj), core::mem::transmute_copy(&dwflags), core::mem::transmute(&pszfeaturekeyword), core::mem::transmute(&pszattribute), core::mem::transmute_copy(&pdwdatatype), core::mem::transmute_copy(&pbdata), core::mem::transmute_copy(&cbsize), core::mem::transmute_copy(&pcbneeded)).into()
        }
        unsafe extern "system" fn GetOptionAttribute<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, poemuiobj: *const OEMUIOBJ, dwflags: u32, pszfeaturekeyword: windows_core::PCSTR, pszoptionkeyword: windows_core::PCSTR, pszattribute: windows_core::PCSTR, pdwdatatype: *mut u32, pbdata: *mut u8, cbsize: u32, pcbneeded: *mut u32) -> windows_core::HRESULT
        where
            Identity: IPrintCoreUI2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPrintCoreUI2_Impl::GetOptionAttribute(this, core::mem::transmute_copy(&poemuiobj), core::mem::transmute_copy(&dwflags), core::mem::transmute(&pszfeaturekeyword), core::mem::transmute(&pszoptionkeyword), core::mem::transmute(&pszattribute), core::mem::transmute_copy(&pdwdatatype), core::mem::transmute_copy(&pbdata), core::mem::transmute_copy(&cbsize), core::mem::transmute_copy(&pcbneeded)).into()
        }
        unsafe extern "system" fn EnumFeatures<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, poemuiobj: *const OEMUIOBJ, dwflags: u32, pmszfeaturelist: windows_core::PSTR, cbsize: u32, pcbneeded: *mut u32) -> windows_core::HRESULT
        where
            Identity: IPrintCoreUI2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPrintCoreUI2_Impl::EnumFeatures(this, core::mem::transmute_copy(&poemuiobj), core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&pmszfeaturelist), core::mem::transmute_copy(&cbsize), core::mem::transmute_copy(&pcbneeded)).into()
        }
        unsafe extern "system" fn EnumOptions<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, poemuiobj: *const OEMUIOBJ, dwflags: u32, pszfeaturekeyword: windows_core::PCSTR, pmszoptionlist: windows_core::PSTR, cbsize: u32, pcbneeded: *mut u32) -> windows_core::HRESULT
        where
            Identity: IPrintCoreUI2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPrintCoreUI2_Impl::EnumOptions(this, core::mem::transmute_copy(&poemuiobj), core::mem::transmute_copy(&dwflags), core::mem::transmute(&pszfeaturekeyword), core::mem::transmute_copy(&pmszoptionlist), core::mem::transmute_copy(&cbsize), core::mem::transmute_copy(&pcbneeded)).into()
        }
        unsafe extern "system" fn QuerySimulationSupport<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hprinter: super::super::Foundation::HANDLE, dwlevel: u32, pcaps: *mut u8, cbsize: u32, pcbneeded: *mut u32) -> windows_core::HRESULT
        where
            Identity: IPrintCoreUI2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPrintCoreUI2_Impl::QuerySimulationSupport(this, core::mem::transmute_copy(&hprinter), core::mem::transmute_copy(&dwlevel), core::mem::transmute_copy(&pcaps), core::mem::transmute_copy(&cbsize), core::mem::transmute_copy(&pcbneeded)).into()
        }
        Self {
            base__: IPrintOemDriverUI_Vtbl::new::<Identity, OFFSET>(),
            GetOptions: GetOptions::<Identity, OFFSET>,
            SetOptions: SetOptions::<Identity, OFFSET>,
            EnumConstrainedOptions: EnumConstrainedOptions::<Identity, OFFSET>,
            WhyConstrained: WhyConstrained::<Identity, OFFSET>,
            GetGlobalAttribute: GetGlobalAttribute::<Identity, OFFSET>,
            GetFeatureAttribute: GetFeatureAttribute::<Identity, OFFSET>,
            GetOptionAttribute: GetOptionAttribute::<Identity, OFFSET>,
            EnumFeatures: EnumFeatures::<Identity, OFFSET>,
            EnumOptions: EnumOptions::<Identity, OFFSET>,
            QuerySimulationSupport: QuerySimulationSupport::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPrintCoreUI2 as windows_core::Interface>::IID || iid == &<IPrintOemDriverUI as windows_core::Interface>::IID
    }
}
pub trait IPrintJob_Impl: Sized {
    fn Name(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Id(&self) -> windows_core::Result<u32>;
    fn PrintedPages(&self) -> windows_core::Result<u32>;
    fn TotalPages(&self) -> windows_core::Result<u32>;
    fn Status(&self) -> windows_core::Result<PrintJobStatus>;
    fn SubmissionTime(&self) -> windows_core::Result<f64>;
    fn RequestCancel(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IPrintJob {}
impl IPrintJob_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IPrintJob_Vtbl
    where
        Identity: IPrintJob_Impl,
    {
        unsafe extern "system" fn Name<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrname: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IPrintJob_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPrintJob_Impl::Name(this) {
                Ok(ok__) => {
                    pbstrname.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Id<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pulid: *mut u32) -> windows_core::HRESULT
        where
            Identity: IPrintJob_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPrintJob_Impl::Id(this) {
                Ok(ok__) => {
                    pulid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PrintedPages<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pulpages: *mut u32) -> windows_core::HRESULT
        where
            Identity: IPrintJob_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPrintJob_Impl::PrintedPages(this) {
                Ok(ok__) => {
                    pulpages.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TotalPages<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pulpages: *mut u32) -> windows_core::HRESULT
        where
            Identity: IPrintJob_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPrintJob_Impl::TotalPages(this) {
                Ok(ok__) => {
                    pulpages.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Status<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstatus: *mut PrintJobStatus) -> windows_core::HRESULT
        where
            Identity: IPrintJob_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPrintJob_Impl::Status(this) {
                Ok(ok__) => {
                    pstatus.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SubmissionTime<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, psubmissiontime: *mut f64) -> windows_core::HRESULT
        where
            Identity: IPrintJob_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPrintJob_Impl::SubmissionTime(this) {
                Ok(ok__) => {
                    psubmissiontime.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestCancel<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPrintJob_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPrintJob_Impl::RequestCancel(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Name: Name::<Identity, OFFSET>,
            Id: Id::<Identity, OFFSET>,
            PrintedPages: PrintedPages::<Identity, OFFSET>,
            TotalPages: TotalPages::<Identity, OFFSET>,
            Status: Status::<Identity, OFFSET>,
            SubmissionTime: SubmissionTime::<Identity, OFFSET>,
            RequestCancel: RequestCancel::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPrintJob as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IPrintJobCollection_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Count(&self) -> windows_core::Result<u32>;
    fn GetAt(&self, ulindex: u32) -> windows_core::Result<IPrintJob>;
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IPrintJobCollection {}
#[cfg(feature = "Win32_System_Com")]
impl IPrintJobCollection_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IPrintJobCollection_Vtbl
    where
        Identity: IPrintJobCollection_Impl,
    {
        unsafe extern "system" fn Count<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pulcount: *mut u32) -> windows_core::HRESULT
        where
            Identity: IPrintJobCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPrintJobCollection_Impl::Count(this) {
                Ok(ok__) => {
                    pulcount.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAt<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulindex: u32, ppjob: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPrintJobCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPrintJobCollection_Impl::GetAt(this, core::mem::transmute_copy(&ulindex)) {
                Ok(ok__) => {
                    ppjob.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppunk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPrintJobCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPrintJobCollection_Impl::_NewEnum(this) {
                Ok(ok__) => {
                    ppunk.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Count: Count::<Identity, OFFSET>,
            GetAt: GetAt::<Identity, OFFSET>,
            _NewEnum: _NewEnum::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPrintJobCollection as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
pub trait IPrintOemCommon_Impl: Sized {
    fn GetInfo(&self, dwmode: u32, pbuffer: *mut core::ffi::c_void, cbsize: u32, pcbneeded: *mut u32) -> windows_core::Result<()>;
    fn DevMode(&self, dwmode: u32, poemdmparam: *mut OEMDMPARAM) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl windows_core::RuntimeName for IPrintOemCommon {}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl IPrintOemCommon_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IPrintOemCommon_Vtbl
    where
        Identity: IPrintOemCommon_Impl,
    {
        unsafe extern "system" fn GetInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwmode: u32, pbuffer: *mut core::ffi::c_void, cbsize: u32, pcbneeded: *mut u32) -> windows_core::HRESULT
        where
            Identity: IPrintOemCommon_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPrintOemCommon_Impl::GetInfo(this, core::mem::transmute_copy(&dwmode), core::mem::transmute_copy(&pbuffer), core::mem::transmute_copy(&cbsize), core::mem::transmute_copy(&pcbneeded)).into()
        }
        unsafe extern "system" fn DevMode<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwmode: u32, poemdmparam: *mut OEMDMPARAM) -> windows_core::HRESULT
        where
            Identity: IPrintOemCommon_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPrintOemCommon_Impl::DevMode(this, core::mem::transmute_copy(&dwmode), core::mem::transmute_copy(&poemdmparam)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetInfo: GetInfo::<Identity, OFFSET>, DevMode: DevMode::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPrintOemCommon as windows_core::Interface>::IID
    }
}
pub trait IPrintOemDriverUI_Impl: Sized {
    fn DrvGetDriverSetting(&self, pci: *mut core::ffi::c_void, feature: &windows_core::PCSTR, poutput: *mut core::ffi::c_void, cbsize: u32, pcbneeded: *mut u32, pdwoptionsreturned: *mut u32) -> windows_core::Result<()>;
    fn DrvUpgradeRegistrySetting(&self, hprinter: super::super::Foundation::HANDLE, pfeature: &windows_core::PCSTR, poption: &windows_core::PCSTR) -> windows_core::Result<()>;
    fn DrvUpdateUISetting(&self, pci: *mut core::ffi::c_void, poptitem: *mut core::ffi::c_void, dwpreviousselection: u32, dwmode: u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IPrintOemDriverUI {}
impl IPrintOemDriverUI_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IPrintOemDriverUI_Vtbl
    where
        Identity: IPrintOemDriverUI_Impl,
    {
        unsafe extern "system" fn DrvGetDriverSetting<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pci: *mut core::ffi::c_void, feature: windows_core::PCSTR, poutput: *mut core::ffi::c_void, cbsize: u32, pcbneeded: *mut u32, pdwoptionsreturned: *mut u32) -> windows_core::HRESULT
        where
            Identity: IPrintOemDriverUI_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPrintOemDriverUI_Impl::DrvGetDriverSetting(this, core::mem::transmute_copy(&pci), core::mem::transmute(&feature), core::mem::transmute_copy(&poutput), core::mem::transmute_copy(&cbsize), core::mem::transmute_copy(&pcbneeded), core::mem::transmute_copy(&pdwoptionsreturned)).into()
        }
        unsafe extern "system" fn DrvUpgradeRegistrySetting<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hprinter: super::super::Foundation::HANDLE, pfeature: windows_core::PCSTR, poption: windows_core::PCSTR) -> windows_core::HRESULT
        where
            Identity: IPrintOemDriverUI_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPrintOemDriverUI_Impl::DrvUpgradeRegistrySetting(this, core::mem::transmute_copy(&hprinter), core::mem::transmute(&pfeature), core::mem::transmute(&poption)).into()
        }
        unsafe extern "system" fn DrvUpdateUISetting<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pci: *mut core::ffi::c_void, poptitem: *mut core::ffi::c_void, dwpreviousselection: u32, dwmode: u32) -> windows_core::HRESULT
        where
            Identity: IPrintOemDriverUI_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPrintOemDriverUI_Impl::DrvUpdateUISetting(this, core::mem::transmute_copy(&pci), core::mem::transmute_copy(&poptitem), core::mem::transmute_copy(&dwpreviousselection), core::mem::transmute_copy(&dwmode)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            DrvGetDriverSetting: DrvGetDriverSetting::<Identity, OFFSET>,
            DrvUpgradeRegistrySetting: DrvUpgradeRegistrySetting::<Identity, OFFSET>,
            DrvUpdateUISetting: DrvUpdateUISetting::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPrintOemDriverUI as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
pub trait IPrintOemUI_Impl: Sized + IPrintOemCommon_Impl {
    fn PublishDriverInterface(&self, piunknown: Option<&windows_core::IUnknown>) -> windows_core::Result<()>;
    fn CommonUIProp(&self, dwmode: u32, poemcuipparam: *const OEMCUIPPARAM) -> windows_core::Result<()>;
    fn DocumentPropertySheets(&self, ppsuiinfo: *mut PROPSHEETUI_INFO, lparam: super::super::Foundation::LPARAM) -> windows_core::Result<()>;
    fn DevicePropertySheets(&self, ppsuiinfo: *const PROPSHEETUI_INFO, lparam: super::super::Foundation::LPARAM) -> windows_core::Result<()>;
    fn DevQueryPrintEx(&self, poemuiobj: *const OEMUIOBJ, pdqpinfo: *const DEVQUERYPRINT_INFO, ppublicdm: *const super::Gdi::DEVMODEA, poemdm: *const core::ffi::c_void) -> windows_core::Result<()>;
    fn DeviceCapabilitiesA(&self, poemuiobj: *mut OEMUIOBJ, hprinter: super::super::Foundation::HANDLE, pdevicename: &windows_core::PCWSTR, wcapability: u16, poutput: *mut core::ffi::c_void, ppublicdm: *const super::Gdi::DEVMODEA, poemdm: *const core::ffi::c_void, dwold: u32, dwresult: *mut u32) -> windows_core::Result<()>;
    fn UpgradePrinter(&self, dwlevel: u32, pdriverupgradeinfo: *const u8) -> windows_core::Result<()>;
    fn PrinterEvent(&self, pprintername: &windows_core::PCWSTR, idriverevent: i32, dwflags: u32, lparam: super::super::Foundation::LPARAM) -> windows_core::Result<()>;
    fn DriverEvent(&self, dwdriverevent: u32, dwlevel: u32, pdriverinfo: *const u8, lparam: super::super::Foundation::LPARAM) -> windows_core::Result<()>;
    fn QueryColorProfile(&self, hprinter: super::super::Foundation::HANDLE, poemuiobj: *const OEMUIOBJ, ppublicdm: *const super::Gdi::DEVMODEA, poemdm: *const core::ffi::c_void, ulquerymode: u32, pvprofiledata: *mut core::ffi::c_void, pcbprofiledata: *mut u32, pflprofiledata: *mut u32) -> windows_core::Result<()>;
    fn FontInstallerDlgProc(&self, hwnd: super::super::Foundation::HWND, usmsg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> windows_core::Result<()>;
    fn UpdateExternalFonts(&self, hprinter: super::super::Foundation::HANDLE, hheap: super::super::Foundation::HANDLE, pwstrcartridges: &windows_core::PCWSTR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl windows_core::RuntimeName for IPrintOemUI {}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl IPrintOemUI_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IPrintOemUI_Vtbl
    where
        Identity: IPrintOemUI_Impl,
    {
        unsafe extern "system" fn PublishDriverInterface<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, piunknown: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPrintOemUI_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPrintOemUI_Impl::PublishDriverInterface(this, windows_core::from_raw_borrowed(&piunknown)).into()
        }
        unsafe extern "system" fn CommonUIProp<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwmode: u32, poemcuipparam: *const OEMCUIPPARAM) -> windows_core::HRESULT
        where
            Identity: IPrintOemUI_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPrintOemUI_Impl::CommonUIProp(this, core::mem::transmute_copy(&dwmode), core::mem::transmute_copy(&poemcuipparam)).into()
        }
        unsafe extern "system" fn DocumentPropertySheets<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppsuiinfo: *mut PROPSHEETUI_INFO, lparam: super::super::Foundation::LPARAM) -> windows_core::HRESULT
        where
            Identity: IPrintOemUI_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPrintOemUI_Impl::DocumentPropertySheets(this, core::mem::transmute_copy(&ppsuiinfo), core::mem::transmute_copy(&lparam)).into()
        }
        unsafe extern "system" fn DevicePropertySheets<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppsuiinfo: *const PROPSHEETUI_INFO, lparam: super::super::Foundation::LPARAM) -> windows_core::HRESULT
        where
            Identity: IPrintOemUI_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPrintOemUI_Impl::DevicePropertySheets(this, core::mem::transmute_copy(&ppsuiinfo), core::mem::transmute_copy(&lparam)).into()
        }
        unsafe extern "system" fn DevQueryPrintEx<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, poemuiobj: *const OEMUIOBJ, pdqpinfo: *const DEVQUERYPRINT_INFO, ppublicdm: *const super::Gdi::DEVMODEA, poemdm: *const core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPrintOemUI_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPrintOemUI_Impl::DevQueryPrintEx(this, core::mem::transmute_copy(&poemuiobj), core::mem::transmute_copy(&pdqpinfo), core::mem::transmute_copy(&ppublicdm), core::mem::transmute_copy(&poemdm)).into()
        }
        unsafe extern "system" fn DeviceCapabilitiesA<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, poemuiobj: *mut OEMUIOBJ, hprinter: super::super::Foundation::HANDLE, pdevicename: windows_core::PCWSTR, wcapability: u16, poutput: *mut core::ffi::c_void, ppublicdm: *const super::Gdi::DEVMODEA, poemdm: *const core::ffi::c_void, dwold: u32, dwresult: *mut u32) -> windows_core::HRESULT
        where
            Identity: IPrintOemUI_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPrintOemUI_Impl::DeviceCapabilitiesA(this, core::mem::transmute_copy(&poemuiobj), core::mem::transmute_copy(&hprinter), core::mem::transmute(&pdevicename), core::mem::transmute_copy(&wcapability), core::mem::transmute_copy(&poutput), core::mem::transmute_copy(&ppublicdm), core::mem::transmute_copy(&poemdm), core::mem::transmute_copy(&dwold), core::mem::transmute_copy(&dwresult)).into()
        }
        unsafe extern "system" fn UpgradePrinter<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwlevel: u32, pdriverupgradeinfo: *const u8) -> windows_core::HRESULT
        where
            Identity: IPrintOemUI_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPrintOemUI_Impl::UpgradePrinter(this, core::mem::transmute_copy(&dwlevel), core::mem::transmute_copy(&pdriverupgradeinfo)).into()
        }
        unsafe extern "system" fn PrinterEvent<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pprintername: windows_core::PCWSTR, idriverevent: i32, dwflags: u32, lparam: super::super::Foundation::LPARAM) -> windows_core::HRESULT
        where
            Identity: IPrintOemUI_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPrintOemUI_Impl::PrinterEvent(this, core::mem::transmute(&pprintername), core::mem::transmute_copy(&idriverevent), core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&lparam)).into()
        }
        unsafe extern "system" fn DriverEvent<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwdriverevent: u32, dwlevel: u32, pdriverinfo: *const u8, lparam: super::super::Foundation::LPARAM) -> windows_core::HRESULT
        where
            Identity: IPrintOemUI_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPrintOemUI_Impl::DriverEvent(this, core::mem::transmute_copy(&dwdriverevent), core::mem::transmute_copy(&dwlevel), core::mem::transmute_copy(&pdriverinfo), core::mem::transmute_copy(&lparam)).into()
        }
        unsafe extern "system" fn QueryColorProfile<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hprinter: super::super::Foundation::HANDLE, poemuiobj: *const OEMUIOBJ, ppublicdm: *const super::Gdi::DEVMODEA, poemdm: *const core::ffi::c_void, ulquerymode: u32, pvprofiledata: *mut core::ffi::c_void, pcbprofiledata: *mut u32, pflprofiledata: *mut u32) -> windows_core::HRESULT
        where
            Identity: IPrintOemUI_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPrintOemUI_Impl::QueryColorProfile(this, core::mem::transmute_copy(&hprinter), core::mem::transmute_copy(&poemuiobj), core::mem::transmute_copy(&ppublicdm), core::mem::transmute_copy(&poemdm), core::mem::transmute_copy(&ulquerymode), core::mem::transmute_copy(&pvprofiledata), core::mem::transmute_copy(&pcbprofiledata), core::mem::transmute_copy(&pflprofiledata)).into()
        }
        unsafe extern "system" fn FontInstallerDlgProc<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwnd: super::super::Foundation::HWND, usmsg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> windows_core::HRESULT
        where
            Identity: IPrintOemUI_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPrintOemUI_Impl::FontInstallerDlgProc(this, core::mem::transmute_copy(&hwnd), core::mem::transmute_copy(&usmsg), core::mem::transmute_copy(&wparam), core::mem::transmute_copy(&lparam)).into()
        }
        unsafe extern "system" fn UpdateExternalFonts<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hprinter: super::super::Foundation::HANDLE, hheap: super::super::Foundation::HANDLE, pwstrcartridges: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IPrintOemUI_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPrintOemUI_Impl::UpdateExternalFonts(this, core::mem::transmute_copy(&hprinter), core::mem::transmute_copy(&hheap), core::mem::transmute(&pwstrcartridges)).into()
        }
        Self {
            base__: IPrintOemCommon_Vtbl::new::<Identity, OFFSET>(),
            PublishDriverInterface: PublishDriverInterface::<Identity, OFFSET>,
            CommonUIProp: CommonUIProp::<Identity, OFFSET>,
            DocumentPropertySheets: DocumentPropertySheets::<Identity, OFFSET>,
            DevicePropertySheets: DevicePropertySheets::<Identity, OFFSET>,
            DevQueryPrintEx: DevQueryPrintEx::<Identity, OFFSET>,
            DeviceCapabilitiesA: DeviceCapabilitiesA::<Identity, OFFSET>,
            UpgradePrinter: UpgradePrinter::<Identity, OFFSET>,
            PrinterEvent: PrinterEvent::<Identity, OFFSET>,
            DriverEvent: DriverEvent::<Identity, OFFSET>,
            QueryColorProfile: QueryColorProfile::<Identity, OFFSET>,
            FontInstallerDlgProc: FontInstallerDlgProc::<Identity, OFFSET>,
            UpdateExternalFonts: UpdateExternalFonts::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPrintOemUI as windows_core::Interface>::IID || iid == &<IPrintOemCommon as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
pub trait IPrintOemUI2_Impl: Sized + IPrintOemUI_Impl {
    fn QueryJobAttributes(&self, hprinter: super::super::Foundation::HANDLE, pdevmode: *const super::Gdi::DEVMODEA, dwlevel: u32, lpattributeinfo: *const u8) -> windows_core::Result<()>;
    fn HideStandardUI(&self, dwmode: u32) -> windows_core::Result<()>;
    fn DocumentEvent(&self, hprinter: super::super::Foundation::HANDLE, hdc: super::Gdi::HDC, iesc: i32, cbin: u32, pvin: *mut core::ffi::c_void, cbout: u32, pvout: *mut core::ffi::c_void, piresult: *mut i32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl windows_core::RuntimeName for IPrintOemUI2 {}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl IPrintOemUI2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IPrintOemUI2_Vtbl
    where
        Identity: IPrintOemUI2_Impl,
    {
        unsafe extern "system" fn QueryJobAttributes<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hprinter: super::super::Foundation::HANDLE, pdevmode: *const super::Gdi::DEVMODEA, dwlevel: u32, lpattributeinfo: *const u8) -> windows_core::HRESULT
        where
            Identity: IPrintOemUI2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPrintOemUI2_Impl::QueryJobAttributes(this, core::mem::transmute_copy(&hprinter), core::mem::transmute_copy(&pdevmode), core::mem::transmute_copy(&dwlevel), core::mem::transmute_copy(&lpattributeinfo)).into()
        }
        unsafe extern "system" fn HideStandardUI<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwmode: u32) -> windows_core::HRESULT
        where
            Identity: IPrintOemUI2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPrintOemUI2_Impl::HideStandardUI(this, core::mem::transmute_copy(&dwmode)).into()
        }
        unsafe extern "system" fn DocumentEvent<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hprinter: super::super::Foundation::HANDLE, hdc: super::Gdi::HDC, iesc: i32, cbin: u32, pvin: *mut core::ffi::c_void, cbout: u32, pvout: *mut core::ffi::c_void, piresult: *mut i32) -> windows_core::HRESULT
        where
            Identity: IPrintOemUI2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPrintOemUI2_Impl::DocumentEvent(this, core::mem::transmute_copy(&hprinter), core::mem::transmute_copy(&hdc), core::mem::transmute_copy(&iesc), core::mem::transmute_copy(&cbin), core::mem::transmute_copy(&pvin), core::mem::transmute_copy(&cbout), core::mem::transmute_copy(&pvout), core::mem::transmute_copy(&piresult)).into()
        }
        Self {
            base__: IPrintOemUI_Vtbl::new::<Identity, OFFSET>(),
            QueryJobAttributes: QueryJobAttributes::<Identity, OFFSET>,
            HideStandardUI: HideStandardUI::<Identity, OFFSET>,
            DocumentEvent: DocumentEvent::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPrintOemUI2 as windows_core::Interface>::IID || iid == &<IPrintOemCommon as windows_core::Interface>::IID || iid == &<IPrintOemUI as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
pub trait IPrintOemUIMXDC_Impl: Sized {
    fn AdjustImageableArea(&self, hprinter: super::super::Foundation::HANDLE, cbdevmode: u32, pdevmode: *const super::Gdi::DEVMODEA, cboemdm: u32, poemdm: *const core::ffi::c_void, prclimageablearea: *mut super::super::Foundation::RECTL) -> windows_core::Result<()>;
    fn AdjustImageCompression(&self, hprinter: super::super::Foundation::HANDLE, cbdevmode: u32, pdevmode: *const super::Gdi::DEVMODEA, cboemdm: u32, poemdm: *const core::ffi::c_void, pcompressionmode: *mut i32) -> windows_core::Result<()>;
    fn AdjustDPI(&self, hprinter: super::super::Foundation::HANDLE, cbdevmode: u32, pdevmode: *const super::Gdi::DEVMODEA, cboemdm: u32, poemdm: *const core::ffi::c_void, pdpi: *mut i32) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl windows_core::RuntimeName for IPrintOemUIMXDC {}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl IPrintOemUIMXDC_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IPrintOemUIMXDC_Vtbl
    where
        Identity: IPrintOemUIMXDC_Impl,
    {
        unsafe extern "system" fn AdjustImageableArea<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hprinter: super::super::Foundation::HANDLE, cbdevmode: u32, pdevmode: *const super::Gdi::DEVMODEA, cboemdm: u32, poemdm: *const core::ffi::c_void, prclimageablearea: *mut super::super::Foundation::RECTL) -> windows_core::HRESULT
        where
            Identity: IPrintOemUIMXDC_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPrintOemUIMXDC_Impl::AdjustImageableArea(this, core::mem::transmute_copy(&hprinter), core::mem::transmute_copy(&cbdevmode), core::mem::transmute_copy(&pdevmode), core::mem::transmute_copy(&cboemdm), core::mem::transmute_copy(&poemdm), core::mem::transmute_copy(&prclimageablearea)).into()
        }
        unsafe extern "system" fn AdjustImageCompression<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hprinter: super::super::Foundation::HANDLE, cbdevmode: u32, pdevmode: *const super::Gdi::DEVMODEA, cboemdm: u32, poemdm: *const core::ffi::c_void, pcompressionmode: *mut i32) -> windows_core::HRESULT
        where
            Identity: IPrintOemUIMXDC_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPrintOemUIMXDC_Impl::AdjustImageCompression(this, core::mem::transmute_copy(&hprinter), core::mem::transmute_copy(&cbdevmode), core::mem::transmute_copy(&pdevmode), core::mem::transmute_copy(&cboemdm), core::mem::transmute_copy(&poemdm), core::mem::transmute_copy(&pcompressionmode)).into()
        }
        unsafe extern "system" fn AdjustDPI<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hprinter: super::super::Foundation::HANDLE, cbdevmode: u32, pdevmode: *const super::Gdi::DEVMODEA, cboemdm: u32, poemdm: *const core::ffi::c_void, pdpi: *mut i32) -> windows_core::HRESULT
        where
            Identity: IPrintOemUIMXDC_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPrintOemUIMXDC_Impl::AdjustDPI(this, core::mem::transmute_copy(&hprinter), core::mem::transmute_copy(&cbdevmode), core::mem::transmute_copy(&pdevmode), core::mem::transmute_copy(&cboemdm), core::mem::transmute_copy(&poemdm), core::mem::transmute_copy(&pdpi)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AdjustImageableArea: AdjustImageableArea::<Identity, OFFSET>,
            AdjustImageCompression: AdjustImageCompression::<Identity, OFFSET>,
            AdjustDPI: AdjustDPI::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPrintOemUIMXDC as windows_core::Interface>::IID
    }
}
pub trait IPrintPipelineFilter_Impl: Sized {
    fn InitializeFilter(&self, pinegotiation: Option<&IInterFilterCommunicator>, pipropertybag: Option<&IPrintPipelinePropertyBag>, pipipelinecontrol: Option<&IPrintPipelineManagerControl>) -> windows_core::Result<()>;
    fn ShutdownOperation(&self) -> windows_core::Result<()>;
    fn StartOperation(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IPrintPipelineFilter {}
impl IPrintPipelineFilter_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IPrintPipelineFilter_Vtbl
    where
        Identity: IPrintPipelineFilter_Impl,
    {
        unsafe extern "system" fn InitializeFilter<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinegotiation: *mut core::ffi::c_void, pipropertybag: *mut core::ffi::c_void, pipipelinecontrol: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPrintPipelineFilter_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPrintPipelineFilter_Impl::InitializeFilter(this, windows_core::from_raw_borrowed(&pinegotiation), windows_core::from_raw_borrowed(&pipropertybag), windows_core::from_raw_borrowed(&pipipelinecontrol)).into()
        }
        unsafe extern "system" fn ShutdownOperation<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPrintPipelineFilter_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPrintPipelineFilter_Impl::ShutdownOperation(this).into()
        }
        unsafe extern "system" fn StartOperation<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPrintPipelineFilter_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPrintPipelineFilter_Impl::StartOperation(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            InitializeFilter: InitializeFilter::<Identity, OFFSET>,
            ShutdownOperation: ShutdownOperation::<Identity, OFFSET>,
            StartOperation: StartOperation::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPrintPipelineFilter as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IPrintPipelineManagerControl_Impl: Sized {
    fn RequestShutdown(&self, hrreason: windows_core::HRESULT, preason: Option<&IImgErrorInfo>) -> windows_core::Result<()>;
    fn FilterFinished(&self) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IPrintPipelineManagerControl {}
#[cfg(feature = "Win32_System_Com")]
impl IPrintPipelineManagerControl_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IPrintPipelineManagerControl_Vtbl
    where
        Identity: IPrintPipelineManagerControl_Impl,
    {
        unsafe extern "system" fn RequestShutdown<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hrreason: windows_core::HRESULT, preason: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPrintPipelineManagerControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPrintPipelineManagerControl_Impl::RequestShutdown(this, core::mem::transmute_copy(&hrreason), windows_core::from_raw_borrowed(&preason)).into()
        }
        unsafe extern "system" fn FilterFinished<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPrintPipelineManagerControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPrintPipelineManagerControl_Impl::FilterFinished(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            RequestShutdown: RequestShutdown::<Identity, OFFSET>,
            FilterFinished: FilterFinished::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPrintPipelineManagerControl as windows_core::Interface>::IID
    }
}
pub trait IPrintPipelineProgressReport_Impl: Sized {
    fn ReportProgress(&self, update: EXpsJobConsumption) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IPrintPipelineProgressReport {}
impl IPrintPipelineProgressReport_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IPrintPipelineProgressReport_Vtbl
    where
        Identity: IPrintPipelineProgressReport_Impl,
    {
        unsafe extern "system" fn ReportProgress<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, update: EXpsJobConsumption) -> windows_core::HRESULT
        where
            Identity: IPrintPipelineProgressReport_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPrintPipelineProgressReport_Impl::ReportProgress(this, core::mem::transmute_copy(&update)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), ReportProgress: ReportProgress::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPrintPipelineProgressReport as windows_core::Interface>::IID
    }
}
pub trait IPrintPipelinePropertyBag_Impl: Sized {
    fn AddProperty(&self, pszname: &windows_core::PCWSTR, pvar: *const windows_core::VARIANT) -> windows_core::Result<()>;
    fn GetProperty(&self, pszname: &windows_core::PCWSTR) -> windows_core::Result<windows_core::VARIANT>;
    fn DeleteProperty(&self, pszname: &windows_core::PCWSTR) -> super::super::Foundation::BOOL;
}
impl windows_core::RuntimeName for IPrintPipelinePropertyBag {}
impl IPrintPipelinePropertyBag_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IPrintPipelinePropertyBag_Vtbl
    where
        Identity: IPrintPipelinePropertyBag_Impl,
    {
        unsafe extern "system" fn AddProperty<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszname: windows_core::PCWSTR, pvar: *const core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IPrintPipelinePropertyBag_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPrintPipelinePropertyBag_Impl::AddProperty(this, core::mem::transmute(&pszname), core::mem::transmute_copy(&pvar)).into()
        }
        unsafe extern "system" fn GetProperty<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszname: windows_core::PCWSTR, pvar: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IPrintPipelinePropertyBag_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPrintPipelinePropertyBag_Impl::GetProperty(this, core::mem::transmute(&pszname)) {
                Ok(ok__) => {
                    pvar.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteProperty<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszname: windows_core::PCWSTR) -> super::super::Foundation::BOOL
        where
            Identity: IPrintPipelinePropertyBag_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPrintPipelinePropertyBag_Impl::DeleteProperty(this, core::mem::transmute(&pszname))
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AddProperty: AddProperty::<Identity, OFFSET>,
            GetProperty: GetProperty::<Identity, OFFSET>,
            DeleteProperty: DeleteProperty::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPrintPipelinePropertyBag as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi")]
pub trait IPrintPreviewDxgiPackageTarget_Impl: Sized {
    fn SetJobPageCount(&self, counttype: PageCountType, count: u32) -> windows_core::Result<()>;
    fn DrawPage(&self, jobpagenumber: u32, pageimage: Option<&super::Dxgi::IDXGISurface>, dpix: f32, dpiy: f32) -> windows_core::Result<()>;
    fn InvalidatePreview(&self) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Dxgi")]
impl windows_core::RuntimeName for IPrintPreviewDxgiPackageTarget {}
#[cfg(feature = "Win32_Graphics_Dxgi")]
impl IPrintPreviewDxgiPackageTarget_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IPrintPreviewDxgiPackageTarget_Vtbl
    where
        Identity: IPrintPreviewDxgiPackageTarget_Impl,
    {
        unsafe extern "system" fn SetJobPageCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, counttype: PageCountType, count: u32) -> windows_core::HRESULT
        where
            Identity: IPrintPreviewDxgiPackageTarget_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPrintPreviewDxgiPackageTarget_Impl::SetJobPageCount(this, core::mem::transmute_copy(&counttype), core::mem::transmute_copy(&count)).into()
        }
        unsafe extern "system" fn DrawPage<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, jobpagenumber: u32, pageimage: *mut core::ffi::c_void, dpix: f32, dpiy: f32) -> windows_core::HRESULT
        where
            Identity: IPrintPreviewDxgiPackageTarget_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPrintPreviewDxgiPackageTarget_Impl::DrawPage(this, core::mem::transmute_copy(&jobpagenumber), windows_core::from_raw_borrowed(&pageimage), core::mem::transmute_copy(&dpix), core::mem::transmute_copy(&dpiy)).into()
        }
        unsafe extern "system" fn InvalidatePreview<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPrintPreviewDxgiPackageTarget_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPrintPreviewDxgiPackageTarget_Impl::InvalidatePreview(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetJobPageCount: SetJobPageCount::<Identity, OFFSET>,
            DrawPage: DrawPage::<Identity, OFFSET>,
            InvalidatePreview: InvalidatePreview::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPrintPreviewDxgiPackageTarget as windows_core::Interface>::IID
    }
}
pub trait IPrintReadStream_Impl: Sized {
    fn Seek(&self, dlibmove: i64, dworigin: u32, plibnewposition: *mut u64) -> windows_core::Result<()>;
    fn ReadBytes(&self, pvbuffer: *mut core::ffi::c_void, cbrequested: u32, pcbread: *mut u32, pbendoffile: *mut super::super::Foundation::BOOL) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IPrintReadStream {}
impl IPrintReadStream_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IPrintReadStream_Vtbl
    where
        Identity: IPrintReadStream_Impl,
    {
        unsafe extern "system" fn Seek<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dlibmove: i64, dworigin: u32, plibnewposition: *mut u64) -> windows_core::HRESULT
        where
            Identity: IPrintReadStream_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPrintReadStream_Impl::Seek(this, core::mem::transmute_copy(&dlibmove), core::mem::transmute_copy(&dworigin), core::mem::transmute_copy(&plibnewposition)).into()
        }
        unsafe extern "system" fn ReadBytes<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvbuffer: *mut core::ffi::c_void, cbrequested: u32, pcbread: *mut u32, pbendoffile: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IPrintReadStream_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPrintReadStream_Impl::ReadBytes(this, core::mem::transmute_copy(&pvbuffer), core::mem::transmute_copy(&cbrequested), core::mem::transmute_copy(&pcbread), core::mem::transmute_copy(&pbendoffile)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Seek: Seek::<Identity, OFFSET>, ReadBytes: ReadBytes::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPrintReadStream as windows_core::Interface>::IID
    }
}
pub trait IPrintReadStreamFactory_Impl: Sized {
    fn GetStream(&self) -> windows_core::Result<IPrintReadStream>;
}
impl windows_core::RuntimeName for IPrintReadStreamFactory {}
impl IPrintReadStreamFactory_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IPrintReadStreamFactory_Vtbl
    where
        Identity: IPrintReadStreamFactory_Impl,
    {
        unsafe extern "system" fn GetStream<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppstream: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPrintReadStreamFactory_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPrintReadStreamFactory_Impl::GetStream(this) {
                Ok(ok__) => {
                    ppstream.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetStream: GetStream::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPrintReadStreamFactory as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IPrintSchemaAsyncOperation_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Start(&self) -> windows_core::Result<()>;
    fn Cancel(&self) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IPrintSchemaAsyncOperation {}
#[cfg(feature = "Win32_System_Com")]
impl IPrintSchemaAsyncOperation_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IPrintSchemaAsyncOperation_Vtbl
    where
        Identity: IPrintSchemaAsyncOperation_Impl,
    {
        unsafe extern "system" fn Start<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPrintSchemaAsyncOperation_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPrintSchemaAsyncOperation_Impl::Start(this).into()
        }
        unsafe extern "system" fn Cancel<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPrintSchemaAsyncOperation_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPrintSchemaAsyncOperation_Impl::Cancel(this).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Start: Start::<Identity, OFFSET>,
            Cancel: Cancel::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPrintSchemaAsyncOperation as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IPrintSchemaAsyncOperationEvent_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Completed(&self, pticket: Option<&IPrintSchemaTicket>, hroperation: windows_core::HRESULT) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IPrintSchemaAsyncOperationEvent {}
#[cfg(feature = "Win32_System_Com")]
impl IPrintSchemaAsyncOperationEvent_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IPrintSchemaAsyncOperationEvent_Vtbl
    where
        Identity: IPrintSchemaAsyncOperationEvent_Impl,
    {
        unsafe extern "system" fn Completed<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pticket: *mut core::ffi::c_void, hroperation: windows_core::HRESULT) -> windows_core::HRESULT
        where
            Identity: IPrintSchemaAsyncOperationEvent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPrintSchemaAsyncOperationEvent_Impl::Completed(this, windows_core::from_raw_borrowed(&pticket), core::mem::transmute_copy(&hroperation)).into()
        }
        Self { base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(), Completed: Completed::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPrintSchemaAsyncOperationEvent as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IPrintSchemaCapabilities_Impl: Sized + IPrintSchemaElement_Impl {
    fn GetFeatureByKeyName(&self, bstrkeyname: &windows_core::BSTR) -> windows_core::Result<IPrintSchemaFeature>;
    fn GetFeature(&self, bstrname: &windows_core::BSTR, bstrnamespaceuri: &windows_core::BSTR) -> windows_core::Result<IPrintSchemaFeature>;
    fn PageImageableSize(&self) -> windows_core::Result<IPrintSchemaPageImageableSize>;
    fn JobCopiesAllDocumentsMinValue(&self) -> windows_core::Result<u32>;
    fn JobCopiesAllDocumentsMaxValue(&self) -> windows_core::Result<u32>;
    fn GetSelectedOptionInPrintTicket(&self, pfeature: Option<&IPrintSchemaFeature>) -> windows_core::Result<IPrintSchemaOption>;
    fn GetOptions(&self, pfeature: Option<&IPrintSchemaFeature>) -> windows_core::Result<IPrintSchemaOptionCollection>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IPrintSchemaCapabilities {}
#[cfg(feature = "Win32_System_Com")]
impl IPrintSchemaCapabilities_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IPrintSchemaCapabilities_Vtbl
    where
        Identity: IPrintSchemaCapabilities_Impl,
    {
        unsafe extern "system" fn GetFeatureByKeyName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrkeyname: core::mem::MaybeUninit<windows_core::BSTR>, ppfeature: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPrintSchemaCapabilities_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPrintSchemaCapabilities_Impl::GetFeatureByKeyName(this, core::mem::transmute(&bstrkeyname)) {
                Ok(ok__) => {
                    ppfeature.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFeature<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrname: core::mem::MaybeUninit<windows_core::BSTR>, bstrnamespaceuri: core::mem::MaybeUninit<windows_core::BSTR>, ppfeature: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPrintSchemaCapabilities_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPrintSchemaCapabilities_Impl::GetFeature(this, core::mem::transmute(&bstrname), core::mem::transmute(&bstrnamespaceuri)) {
                Ok(ok__) => {
                    ppfeature.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PageImageableSize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pppageimageablesize: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPrintSchemaCapabilities_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPrintSchemaCapabilities_Impl::PageImageableSize(this) {
                Ok(ok__) => {
                    pppageimageablesize.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn JobCopiesAllDocumentsMinValue<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, puljobcopiesalldocumentsminvalue: *mut u32) -> windows_core::HRESULT
        where
            Identity: IPrintSchemaCapabilities_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPrintSchemaCapabilities_Impl::JobCopiesAllDocumentsMinValue(this) {
                Ok(ok__) => {
                    puljobcopiesalldocumentsminvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn JobCopiesAllDocumentsMaxValue<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, puljobcopiesalldocumentsmaxvalue: *mut u32) -> windows_core::HRESULT
        where
            Identity: IPrintSchemaCapabilities_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPrintSchemaCapabilities_Impl::JobCopiesAllDocumentsMaxValue(this) {
                Ok(ok__) => {
                    puljobcopiesalldocumentsmaxvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSelectedOptionInPrintTicket<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfeature: *mut core::ffi::c_void, ppoption: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPrintSchemaCapabilities_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPrintSchemaCapabilities_Impl::GetSelectedOptionInPrintTicket(this, windows_core::from_raw_borrowed(&pfeature)) {
                Ok(ok__) => {
                    ppoption.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOptions<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfeature: *mut core::ffi::c_void, ppoptioncollection: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPrintSchemaCapabilities_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPrintSchemaCapabilities_Impl::GetOptions(this, windows_core::from_raw_borrowed(&pfeature)) {
                Ok(ok__) => {
                    ppoptioncollection.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: IPrintSchemaElement_Vtbl::new::<Identity, OFFSET>(),
            GetFeatureByKeyName: GetFeatureByKeyName::<Identity, OFFSET>,
            GetFeature: GetFeature::<Identity, OFFSET>,
            PageImageableSize: PageImageableSize::<Identity, OFFSET>,
            JobCopiesAllDocumentsMinValue: JobCopiesAllDocumentsMinValue::<Identity, OFFSET>,
            JobCopiesAllDocumentsMaxValue: JobCopiesAllDocumentsMaxValue::<Identity, OFFSET>,
            GetSelectedOptionInPrintTicket: GetSelectedOptionInPrintTicket::<Identity, OFFSET>,
            GetOptions: GetOptions::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPrintSchemaCapabilities as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<IPrintSchemaElement as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IPrintSchemaCapabilities2_Impl: Sized + IPrintSchemaCapabilities_Impl {
    fn GetParameterDefinition(&self, bstrname: &windows_core::BSTR, bstrnamespaceuri: &windows_core::BSTR) -> windows_core::Result<IPrintSchemaParameterDefinition>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IPrintSchemaCapabilities2 {}
#[cfg(feature = "Win32_System_Com")]
impl IPrintSchemaCapabilities2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IPrintSchemaCapabilities2_Vtbl
    where
        Identity: IPrintSchemaCapabilities2_Impl,
    {
        unsafe extern "system" fn GetParameterDefinition<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrname: core::mem::MaybeUninit<windows_core::BSTR>, bstrnamespaceuri: core::mem::MaybeUninit<windows_core::BSTR>, ppparameterdefinition: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPrintSchemaCapabilities2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPrintSchemaCapabilities2_Impl::GetParameterDefinition(this, core::mem::transmute(&bstrname), core::mem::transmute(&bstrnamespaceuri)) {
                Ok(ok__) => {
                    ppparameterdefinition.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: IPrintSchemaCapabilities_Vtbl::new::<Identity, OFFSET>(), GetParameterDefinition: GetParameterDefinition::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPrintSchemaCapabilities2 as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<IPrintSchemaElement as windows_core::Interface>::IID || iid == &<IPrintSchemaCapabilities as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IPrintSchemaDisplayableElement_Impl: Sized + IPrintSchemaElement_Impl {
    fn DisplayName(&self) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IPrintSchemaDisplayableElement {}
#[cfg(feature = "Win32_System_Com")]
impl IPrintSchemaDisplayableElement_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IPrintSchemaDisplayableElement_Vtbl
    where
        Identity: IPrintSchemaDisplayableElement_Impl,
    {
        unsafe extern "system" fn DisplayName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrdisplayname: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IPrintSchemaDisplayableElement_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPrintSchemaDisplayableElement_Impl::DisplayName(this) {
                Ok(ok__) => {
                    pbstrdisplayname.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: IPrintSchemaElement_Vtbl::new::<Identity, OFFSET>(), DisplayName: DisplayName::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPrintSchemaDisplayableElement as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<IPrintSchemaElement as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IPrintSchemaElement_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn XmlNode(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn Name(&self) -> windows_core::Result<windows_core::BSTR>;
    fn NamespaceUri(&self) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IPrintSchemaElement {}
#[cfg(feature = "Win32_System_Com")]
impl IPrintSchemaElement_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IPrintSchemaElement_Vtbl
    where
        Identity: IPrintSchemaElement_Impl,
    {
        unsafe extern "system" fn XmlNode<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppxmlnode: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPrintSchemaElement_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPrintSchemaElement_Impl::XmlNode(this) {
                Ok(ok__) => {
                    ppxmlnode.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Name<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrname: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IPrintSchemaElement_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPrintSchemaElement_Impl::Name(this) {
                Ok(ok__) => {
                    pbstrname.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NamespaceUri<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrnamespaceuri: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IPrintSchemaElement_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPrintSchemaElement_Impl::NamespaceUri(this) {
                Ok(ok__) => {
                    pbstrnamespaceuri.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            XmlNode: XmlNode::<Identity, OFFSET>,
            Name: Name::<Identity, OFFSET>,
            NamespaceUri: NamespaceUri::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPrintSchemaElement as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IPrintSchemaFeature_Impl: Sized + IPrintSchemaDisplayableElement_Impl {
    fn SelectedOption(&self) -> windows_core::Result<IPrintSchemaOption>;
    fn SetSelectedOption(&self, poption: Option<&IPrintSchemaOption>) -> windows_core::Result<()>;
    fn SelectionType(&self) -> windows_core::Result<PrintSchemaSelectionType>;
    fn GetOption(&self, bstrname: &windows_core::BSTR, bstrnamespaceuri: &windows_core::BSTR) -> windows_core::Result<IPrintSchemaOption>;
    fn DisplayUI(&self) -> windows_core::Result<super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IPrintSchemaFeature {}
#[cfg(feature = "Win32_System_Com")]
impl IPrintSchemaFeature_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IPrintSchemaFeature_Vtbl
    where
        Identity: IPrintSchemaFeature_Impl,
    {
        unsafe extern "system" fn SelectedOption<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppoption: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPrintSchemaFeature_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPrintSchemaFeature_Impl::SelectedOption(this) {
                Ok(ok__) => {
                    ppoption.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSelectedOption<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, poption: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPrintSchemaFeature_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPrintSchemaFeature_Impl::SetSelectedOption(this, windows_core::from_raw_borrowed(&poption)).into()
        }
        unsafe extern "system" fn SelectionType<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pselectiontype: *mut PrintSchemaSelectionType) -> windows_core::HRESULT
        where
            Identity: IPrintSchemaFeature_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPrintSchemaFeature_Impl::SelectionType(this) {
                Ok(ok__) => {
                    pselectiontype.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOption<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrname: core::mem::MaybeUninit<windows_core::BSTR>, bstrnamespaceuri: core::mem::MaybeUninit<windows_core::BSTR>, ppoption: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPrintSchemaFeature_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPrintSchemaFeature_Impl::GetOption(this, core::mem::transmute(&bstrname), core::mem::transmute(&bstrnamespaceuri)) {
                Ok(ok__) => {
                    ppoption.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DisplayUI<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbshow: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IPrintSchemaFeature_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPrintSchemaFeature_Impl::DisplayUI(this) {
                Ok(ok__) => {
                    pbshow.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: IPrintSchemaDisplayableElement_Vtbl::new::<Identity, OFFSET>(),
            SelectedOption: SelectedOption::<Identity, OFFSET>,
            SetSelectedOption: SetSelectedOption::<Identity, OFFSET>,
            SelectionType: SelectionType::<Identity, OFFSET>,
            GetOption: GetOption::<Identity, OFFSET>,
            DisplayUI: DisplayUI::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPrintSchemaFeature as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<IPrintSchemaElement as windows_core::Interface>::IID || iid == &<IPrintSchemaDisplayableElement as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IPrintSchemaNUpOption_Impl: Sized + IPrintSchemaOption_Impl {
    fn PagesPerSheet(&self) -> windows_core::Result<u32>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IPrintSchemaNUpOption {}
#[cfg(feature = "Win32_System_Com")]
impl IPrintSchemaNUpOption_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IPrintSchemaNUpOption_Vtbl
    where
        Identity: IPrintSchemaNUpOption_Impl,
    {
        unsafe extern "system" fn PagesPerSheet<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pulpagespersheet: *mut u32) -> windows_core::HRESULT
        where
            Identity: IPrintSchemaNUpOption_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPrintSchemaNUpOption_Impl::PagesPerSheet(this) {
                Ok(ok__) => {
                    pulpagespersheet.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: IPrintSchemaOption_Vtbl::new::<Identity, OFFSET>(), PagesPerSheet: PagesPerSheet::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPrintSchemaNUpOption as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<IPrintSchemaElement as windows_core::Interface>::IID || iid == &<IPrintSchemaDisplayableElement as windows_core::Interface>::IID || iid == &<IPrintSchemaOption as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IPrintSchemaOption_Impl: Sized + IPrintSchemaDisplayableElement_Impl {
    fn Selected(&self) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn Constrained(&self) -> windows_core::Result<PrintSchemaConstrainedSetting>;
    fn GetPropertyValue(&self, bstrname: &windows_core::BSTR, bstrnamespaceuri: &windows_core::BSTR) -> windows_core::Result<windows_core::IUnknown>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IPrintSchemaOption {}
#[cfg(feature = "Win32_System_Com")]
impl IPrintSchemaOption_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IPrintSchemaOption_Vtbl
    where
        Identity: IPrintSchemaOption_Impl,
    {
        unsafe extern "system" fn Selected<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbisselected: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IPrintSchemaOption_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPrintSchemaOption_Impl::Selected(this) {
                Ok(ok__) => {
                    pbisselected.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Constrained<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, psetting: *mut PrintSchemaConstrainedSetting) -> windows_core::HRESULT
        where
            Identity: IPrintSchemaOption_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPrintSchemaOption_Impl::Constrained(this) {
                Ok(ok__) => {
                    psetting.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPropertyValue<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrname: core::mem::MaybeUninit<windows_core::BSTR>, bstrnamespaceuri: core::mem::MaybeUninit<windows_core::BSTR>, ppxmlvaluenode: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPrintSchemaOption_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPrintSchemaOption_Impl::GetPropertyValue(this, core::mem::transmute(&bstrname), core::mem::transmute(&bstrnamespaceuri)) {
                Ok(ok__) => {
                    ppxmlvaluenode.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: IPrintSchemaDisplayableElement_Vtbl::new::<Identity, OFFSET>(),
            Selected: Selected::<Identity, OFFSET>,
            Constrained: Constrained::<Identity, OFFSET>,
            GetPropertyValue: GetPropertyValue::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPrintSchemaOption as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<IPrintSchemaElement as windows_core::Interface>::IID || iid == &<IPrintSchemaDisplayableElement as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IPrintSchemaOptionCollection_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Count(&self) -> windows_core::Result<u32>;
    fn GetAt(&self, ulindex: u32) -> windows_core::Result<IPrintSchemaOption>;
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IPrintSchemaOptionCollection {}
#[cfg(feature = "Win32_System_Com")]
impl IPrintSchemaOptionCollection_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IPrintSchemaOptionCollection_Vtbl
    where
        Identity: IPrintSchemaOptionCollection_Impl,
    {
        unsafe extern "system" fn Count<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pulcount: *mut u32) -> windows_core::HRESULT
        where
            Identity: IPrintSchemaOptionCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPrintSchemaOptionCollection_Impl::Count(this) {
                Ok(ok__) => {
                    pulcount.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAt<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulindex: u32, ppoption: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPrintSchemaOptionCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPrintSchemaOptionCollection_Impl::GetAt(this, core::mem::transmute_copy(&ulindex)) {
                Ok(ok__) => {
                    ppoption.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppunk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPrintSchemaOptionCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPrintSchemaOptionCollection_Impl::_NewEnum(this) {
                Ok(ok__) => {
                    ppunk.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Count: Count::<Identity, OFFSET>,
            GetAt: GetAt::<Identity, OFFSET>,
            _NewEnum: _NewEnum::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPrintSchemaOptionCollection as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IPrintSchemaPageImageableSize_Impl: Sized + IPrintSchemaElement_Impl {
    fn ImageableSizeWidthInMicrons(&self) -> windows_core::Result<u32>;
    fn ImageableSizeHeightInMicrons(&self) -> windows_core::Result<u32>;
    fn OriginWidthInMicrons(&self) -> windows_core::Result<u32>;
    fn OriginHeightInMicrons(&self) -> windows_core::Result<u32>;
    fn ExtentWidthInMicrons(&self) -> windows_core::Result<u32>;
    fn ExtentHeightInMicrons(&self) -> windows_core::Result<u32>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IPrintSchemaPageImageableSize {}
#[cfg(feature = "Win32_System_Com")]
impl IPrintSchemaPageImageableSize_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IPrintSchemaPageImageableSize_Vtbl
    where
        Identity: IPrintSchemaPageImageableSize_Impl,
    {
        unsafe extern "system" fn ImageableSizeWidthInMicrons<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pulimageablesizewidth: *mut u32) -> windows_core::HRESULT
        where
            Identity: IPrintSchemaPageImageableSize_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPrintSchemaPageImageableSize_Impl::ImageableSizeWidthInMicrons(this) {
                Ok(ok__) => {
                    pulimageablesizewidth.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ImageableSizeHeightInMicrons<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pulimageablesizeheight: *mut u32) -> windows_core::HRESULT
        where
            Identity: IPrintSchemaPageImageableSize_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPrintSchemaPageImageableSize_Impl::ImageableSizeHeightInMicrons(this) {
                Ok(ok__) => {
                    pulimageablesizeheight.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OriginWidthInMicrons<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, puloriginwidth: *mut u32) -> windows_core::HRESULT
        where
            Identity: IPrintSchemaPageImageableSize_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPrintSchemaPageImageableSize_Impl::OriginWidthInMicrons(this) {
                Ok(ok__) => {
                    puloriginwidth.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OriginHeightInMicrons<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, puloriginheight: *mut u32) -> windows_core::HRESULT
        where
            Identity: IPrintSchemaPageImageableSize_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPrintSchemaPageImageableSize_Impl::OriginHeightInMicrons(this) {
                Ok(ok__) => {
                    puloriginheight.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExtentWidthInMicrons<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pulextentwidth: *mut u32) -> windows_core::HRESULT
        where
            Identity: IPrintSchemaPageImageableSize_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPrintSchemaPageImageableSize_Impl::ExtentWidthInMicrons(this) {
                Ok(ok__) => {
                    pulextentwidth.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExtentHeightInMicrons<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pulextentheight: *mut u32) -> windows_core::HRESULT
        where
            Identity: IPrintSchemaPageImageableSize_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPrintSchemaPageImageableSize_Impl::ExtentHeightInMicrons(this) {
                Ok(ok__) => {
                    pulextentheight.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: IPrintSchemaElement_Vtbl::new::<Identity, OFFSET>(),
            ImageableSizeWidthInMicrons: ImageableSizeWidthInMicrons::<Identity, OFFSET>,
            ImageableSizeHeightInMicrons: ImageableSizeHeightInMicrons::<Identity, OFFSET>,
            OriginWidthInMicrons: OriginWidthInMicrons::<Identity, OFFSET>,
            OriginHeightInMicrons: OriginHeightInMicrons::<Identity, OFFSET>,
            ExtentWidthInMicrons: ExtentWidthInMicrons::<Identity, OFFSET>,
            ExtentHeightInMicrons: ExtentHeightInMicrons::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPrintSchemaPageImageableSize as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<IPrintSchemaElement as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IPrintSchemaPageMediaSizeOption_Impl: Sized + IPrintSchemaOption_Impl {
    fn WidthInMicrons(&self) -> windows_core::Result<u32>;
    fn HeightInMicrons(&self) -> windows_core::Result<u32>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IPrintSchemaPageMediaSizeOption {}
#[cfg(feature = "Win32_System_Com")]
impl IPrintSchemaPageMediaSizeOption_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IPrintSchemaPageMediaSizeOption_Vtbl
    where
        Identity: IPrintSchemaPageMediaSizeOption_Impl,
    {
        unsafe extern "system" fn WidthInMicrons<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pulwidth: *mut u32) -> windows_core::HRESULT
        where
            Identity: IPrintSchemaPageMediaSizeOption_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPrintSchemaPageMediaSizeOption_Impl::WidthInMicrons(this) {
                Ok(ok__) => {
                    pulwidth.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HeightInMicrons<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pulheight: *mut u32) -> windows_core::HRESULT
        where
            Identity: IPrintSchemaPageMediaSizeOption_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPrintSchemaPageMediaSizeOption_Impl::HeightInMicrons(this) {
                Ok(ok__) => {
                    pulheight.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: IPrintSchemaOption_Vtbl::new::<Identity, OFFSET>(),
            WidthInMicrons: WidthInMicrons::<Identity, OFFSET>,
            HeightInMicrons: HeightInMicrons::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPrintSchemaPageMediaSizeOption as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<IPrintSchemaElement as windows_core::Interface>::IID || iid == &<IPrintSchemaDisplayableElement as windows_core::Interface>::IID || iid == &<IPrintSchemaOption as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IPrintSchemaParameterDefinition_Impl: Sized + IPrintSchemaDisplayableElement_Impl {
    fn UserInputRequired(&self) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn UnitType(&self) -> windows_core::Result<windows_core::BSTR>;
    fn DataType(&self) -> windows_core::Result<PrintSchemaParameterDataType>;
    fn RangeMin(&self) -> windows_core::Result<i32>;
    fn RangeMax(&self) -> windows_core::Result<i32>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IPrintSchemaParameterDefinition {}
#[cfg(feature = "Win32_System_Com")]
impl IPrintSchemaParameterDefinition_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IPrintSchemaParameterDefinition_Vtbl
    where
        Identity: IPrintSchemaParameterDefinition_Impl,
    {
        unsafe extern "system" fn UserInputRequired<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbisrequired: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IPrintSchemaParameterDefinition_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPrintSchemaParameterDefinition_Impl::UserInputRequired(this) {
                Ok(ok__) => {
                    pbisrequired.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnitType<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrunittype: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IPrintSchemaParameterDefinition_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPrintSchemaParameterDefinition_Impl::UnitType(this) {
                Ok(ok__) => {
                    pbstrunittype.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DataType<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdatatype: *mut PrintSchemaParameterDataType) -> windows_core::HRESULT
        where
            Identity: IPrintSchemaParameterDefinition_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPrintSchemaParameterDefinition_Impl::DataType(this) {
                Ok(ok__) => {
                    pdatatype.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RangeMin<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, prangemin: *mut i32) -> windows_core::HRESULT
        where
            Identity: IPrintSchemaParameterDefinition_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPrintSchemaParameterDefinition_Impl::RangeMin(this) {
                Ok(ok__) => {
                    prangemin.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RangeMax<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, prangemax: *mut i32) -> windows_core::HRESULT
        where
            Identity: IPrintSchemaParameterDefinition_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPrintSchemaParameterDefinition_Impl::RangeMax(this) {
                Ok(ok__) => {
                    prangemax.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: IPrintSchemaDisplayableElement_Vtbl::new::<Identity, OFFSET>(),
            UserInputRequired: UserInputRequired::<Identity, OFFSET>,
            UnitType: UnitType::<Identity, OFFSET>,
            DataType: DataType::<Identity, OFFSET>,
            RangeMin: RangeMin::<Identity, OFFSET>,
            RangeMax: RangeMax::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPrintSchemaParameterDefinition as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<IPrintSchemaElement as windows_core::Interface>::IID || iid == &<IPrintSchemaDisplayableElement as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IPrintSchemaParameterInitializer_Impl: Sized + IPrintSchemaElement_Impl {
    fn Value(&self) -> windows_core::Result<windows_core::VARIANT>;
    fn SetValue(&self, pvar: *const windows_core::VARIANT) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IPrintSchemaParameterInitializer {}
#[cfg(feature = "Win32_System_Com")]
impl IPrintSchemaParameterInitializer_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IPrintSchemaParameterInitializer_Vtbl
    where
        Identity: IPrintSchemaParameterInitializer_Impl,
    {
        unsafe extern "system" fn Value<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvar: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IPrintSchemaParameterInitializer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPrintSchemaParameterInitializer_Impl::Value(this) {
                Ok(ok__) => {
                    pvar.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetValue<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvar: *const core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IPrintSchemaParameterInitializer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPrintSchemaParameterInitializer_Impl::SetValue(this, core::mem::transmute_copy(&pvar)).into()
        }
        Self { base__: IPrintSchemaElement_Vtbl::new::<Identity, OFFSET>(), Value: Value::<Identity, OFFSET>, SetValue: SetValue::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPrintSchemaParameterInitializer as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<IPrintSchemaElement as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IPrintSchemaTicket_Impl: Sized + IPrintSchemaElement_Impl {
    fn GetFeatureByKeyName(&self, bstrkeyname: &windows_core::BSTR) -> windows_core::Result<IPrintSchemaFeature>;
    fn GetFeature(&self, bstrname: &windows_core::BSTR, bstrnamespaceuri: &windows_core::BSTR) -> windows_core::Result<IPrintSchemaFeature>;
    fn ValidateAsync(&self) -> windows_core::Result<IPrintSchemaAsyncOperation>;
    fn CommitAsync(&self, pprintticketcommit: Option<&IPrintSchemaTicket>) -> windows_core::Result<IPrintSchemaAsyncOperation>;
    fn NotifyXmlChanged(&self) -> windows_core::Result<()>;
    fn GetCapabilities(&self) -> windows_core::Result<IPrintSchemaCapabilities>;
    fn JobCopiesAllDocuments(&self) -> windows_core::Result<u32>;
    fn SetJobCopiesAllDocuments(&self, uljobcopiesalldocuments: u32) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IPrintSchemaTicket {}
#[cfg(feature = "Win32_System_Com")]
impl IPrintSchemaTicket_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IPrintSchemaTicket_Vtbl
    where
        Identity: IPrintSchemaTicket_Impl,
    {
        unsafe extern "system" fn GetFeatureByKeyName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrkeyname: core::mem::MaybeUninit<windows_core::BSTR>, ppfeature: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPrintSchemaTicket_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPrintSchemaTicket_Impl::GetFeatureByKeyName(this, core::mem::transmute(&bstrkeyname)) {
                Ok(ok__) => {
                    ppfeature.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFeature<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrname: core::mem::MaybeUninit<windows_core::BSTR>, bstrnamespaceuri: core::mem::MaybeUninit<windows_core::BSTR>, ppfeature: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPrintSchemaTicket_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPrintSchemaTicket_Impl::GetFeature(this, core::mem::transmute(&bstrname), core::mem::transmute(&bstrnamespaceuri)) {
                Ok(ok__) => {
                    ppfeature.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ValidateAsync<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppasyncoperation: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPrintSchemaTicket_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPrintSchemaTicket_Impl::ValidateAsync(this) {
                Ok(ok__) => {
                    ppasyncoperation.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CommitAsync<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pprintticketcommit: *mut core::ffi::c_void, ppasyncoperation: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPrintSchemaTicket_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPrintSchemaTicket_Impl::CommitAsync(this, windows_core::from_raw_borrowed(&pprintticketcommit)) {
                Ok(ok__) => {
                    ppasyncoperation.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NotifyXmlChanged<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPrintSchemaTicket_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPrintSchemaTicket_Impl::NotifyXmlChanged(this).into()
        }
        unsafe extern "system" fn GetCapabilities<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcapabilities: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPrintSchemaTicket_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPrintSchemaTicket_Impl::GetCapabilities(this) {
                Ok(ok__) => {
                    ppcapabilities.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn JobCopiesAllDocuments<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, puljobcopiesalldocuments: *mut u32) -> windows_core::HRESULT
        where
            Identity: IPrintSchemaTicket_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPrintSchemaTicket_Impl::JobCopiesAllDocuments(this) {
                Ok(ok__) => {
                    puljobcopiesalldocuments.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetJobCopiesAllDocuments<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, uljobcopiesalldocuments: u32) -> windows_core::HRESULT
        where
            Identity: IPrintSchemaTicket_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPrintSchemaTicket_Impl::SetJobCopiesAllDocuments(this, core::mem::transmute_copy(&uljobcopiesalldocuments)).into()
        }
        Self {
            base__: IPrintSchemaElement_Vtbl::new::<Identity, OFFSET>(),
            GetFeatureByKeyName: GetFeatureByKeyName::<Identity, OFFSET>,
            GetFeature: GetFeature::<Identity, OFFSET>,
            ValidateAsync: ValidateAsync::<Identity, OFFSET>,
            CommitAsync: CommitAsync::<Identity, OFFSET>,
            NotifyXmlChanged: NotifyXmlChanged::<Identity, OFFSET>,
            GetCapabilities: GetCapabilities::<Identity, OFFSET>,
            JobCopiesAllDocuments: JobCopiesAllDocuments::<Identity, OFFSET>,
            SetJobCopiesAllDocuments: SetJobCopiesAllDocuments::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPrintSchemaTicket as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<IPrintSchemaElement as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IPrintSchemaTicket2_Impl: Sized + IPrintSchemaTicket_Impl {
    fn GetParameterInitializer(&self, bstrname: &windows_core::BSTR, bstrnamespaceuri: &windows_core::BSTR) -> windows_core::Result<IPrintSchemaParameterInitializer>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IPrintSchemaTicket2 {}
#[cfg(feature = "Win32_System_Com")]
impl IPrintSchemaTicket2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IPrintSchemaTicket2_Vtbl
    where
        Identity: IPrintSchemaTicket2_Impl,
    {
        unsafe extern "system" fn GetParameterInitializer<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrname: core::mem::MaybeUninit<windows_core::BSTR>, bstrnamespaceuri: core::mem::MaybeUninit<windows_core::BSTR>, ppparameterinitializer: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPrintSchemaTicket2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPrintSchemaTicket2_Impl::GetParameterInitializer(this, core::mem::transmute(&bstrname), core::mem::transmute(&bstrnamespaceuri)) {
                Ok(ok__) => {
                    ppparameterinitializer.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: IPrintSchemaTicket_Vtbl::new::<Identity, OFFSET>(), GetParameterInitializer: GetParameterInitializer::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPrintSchemaTicket2 as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<IPrintSchemaElement as windows_core::Interface>::IID || iid == &<IPrintSchemaTicket as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
pub trait IPrintTicketProvider_Impl: Sized {
    fn GetSupportedVersions(&self, hprinter: super::super::Foundation::HANDLE, ppversions: *mut *mut i32, cversions: *mut i32) -> windows_core::Result<()>;
    fn BindPrinter(&self, hprinter: super::super::Foundation::HANDLE, version: i32, poptions: *mut SHIMOPTS, pdevmodeflags: *mut u32, cnamespaces: *mut i32, ppnamespaces: *mut *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn QueryDeviceNamespace(&self, pdefaultnamespace: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn ConvertPrintTicketToDevMode(&self, pprintticket: Option<&super::super::Data::Xml::MsXml::IXMLDOMDocument2>, cbdevmodein: u32, pdevmodein: *mut super::Gdi::DEVMODEA, pcbdevmodeout: *mut u32, ppdevmodeout: *mut *mut super::Gdi::DEVMODEA) -> windows_core::Result<()>;
    fn ConvertDevModeToPrintTicket(&self, cbdevmode: u32, pdevmode: *mut super::Gdi::DEVMODEA, pprintticket: Option<&super::super::Data::Xml::MsXml::IXMLDOMDocument2>) -> windows_core::Result<()>;
    fn GetPrintCapabilities(&self, pprintticket: Option<&super::super::Data::Xml::MsXml::IXMLDOMDocument2>) -> windows_core::Result<super::super::Data::Xml::MsXml::IXMLDOMDocument2>;
    fn ValidatePrintTicket(&self, pbaseticket: Option<&super::super::Data::Xml::MsXml::IXMLDOMDocument2>) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
impl windows_core::RuntimeName for IPrintTicketProvider {}
#[cfg(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
impl IPrintTicketProvider_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IPrintTicketProvider_Vtbl
    where
        Identity: IPrintTicketProvider_Impl,
    {
        unsafe extern "system" fn GetSupportedVersions<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hprinter: super::super::Foundation::HANDLE, ppversions: *mut *mut i32, cversions: *mut i32) -> windows_core::HRESULT
        where
            Identity: IPrintTicketProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPrintTicketProvider_Impl::GetSupportedVersions(this, core::mem::transmute_copy(&hprinter), core::mem::transmute_copy(&ppversions), core::mem::transmute_copy(&cversions)).into()
        }
        unsafe extern "system" fn BindPrinter<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hprinter: super::super::Foundation::HANDLE, version: i32, poptions: *mut SHIMOPTS, pdevmodeflags: *mut u32, cnamespaces: *mut i32, ppnamespaces: *mut *mut windows_core::BSTR) -> windows_core::HRESULT
        where
            Identity: IPrintTicketProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPrintTicketProvider_Impl::BindPrinter(this, core::mem::transmute_copy(&hprinter), core::mem::transmute_copy(&version), core::mem::transmute_copy(&poptions), core::mem::transmute_copy(&pdevmodeflags), core::mem::transmute_copy(&cnamespaces), core::mem::transmute_copy(&ppnamespaces)).into()
        }
        unsafe extern "system" fn QueryDeviceNamespace<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdefaultnamespace: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IPrintTicketProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPrintTicketProvider_Impl::QueryDeviceNamespace(this, core::mem::transmute_copy(&pdefaultnamespace)).into()
        }
        unsafe extern "system" fn ConvertPrintTicketToDevMode<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pprintticket: *mut core::ffi::c_void, cbdevmodein: u32, pdevmodein: *mut super::Gdi::DEVMODEA, pcbdevmodeout: *mut u32, ppdevmodeout: *mut *mut super::Gdi::DEVMODEA) -> windows_core::HRESULT
        where
            Identity: IPrintTicketProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPrintTicketProvider_Impl::ConvertPrintTicketToDevMode(this, windows_core::from_raw_borrowed(&pprintticket), core::mem::transmute_copy(&cbdevmodein), core::mem::transmute_copy(&pdevmodein), core::mem::transmute_copy(&pcbdevmodeout), core::mem::transmute_copy(&ppdevmodeout)).into()
        }
        unsafe extern "system" fn ConvertDevModeToPrintTicket<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, cbdevmode: u32, pdevmode: *mut super::Gdi::DEVMODEA, pprintticket: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPrintTicketProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPrintTicketProvider_Impl::ConvertDevModeToPrintTicket(this, core::mem::transmute_copy(&cbdevmode), core::mem::transmute_copy(&pdevmode), windows_core::from_raw_borrowed(&pprintticket)).into()
        }
        unsafe extern "system" fn GetPrintCapabilities<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pprintticket: *mut core::ffi::c_void, ppcapabilities: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPrintTicketProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPrintTicketProvider_Impl::GetPrintCapabilities(this, windows_core::from_raw_borrowed(&pprintticket)) {
                Ok(ok__) => {
                    ppcapabilities.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ValidatePrintTicket<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbaseticket: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPrintTicketProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPrintTicketProvider_Impl::ValidatePrintTicket(this, windows_core::from_raw_borrowed(&pbaseticket)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetSupportedVersions: GetSupportedVersions::<Identity, OFFSET>,
            BindPrinter: BindPrinter::<Identity, OFFSET>,
            QueryDeviceNamespace: QueryDeviceNamespace::<Identity, OFFSET>,
            ConvertPrintTicketToDevMode: ConvertPrintTicketToDevMode::<Identity, OFFSET>,
            ConvertDevModeToPrintTicket: ConvertDevModeToPrintTicket::<Identity, OFFSET>,
            GetPrintCapabilities: GetPrintCapabilities::<Identity, OFFSET>,
            ValidatePrintTicket: ValidatePrintTicket::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPrintTicketProvider as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
pub trait IPrintTicketProvider2_Impl: Sized + IPrintTicketProvider_Impl {
    fn GetPrintDeviceCapabilities(&self, pprintticket: Option<&super::super::Data::Xml::MsXml::IXMLDOMDocument2>) -> windows_core::Result<super::super::Data::Xml::MsXml::IXMLDOMDocument2>;
    fn GetPrintDeviceResources(&self, pszlocalename: &windows_core::PCWSTR, pprintticket: Option<&super::super::Data::Xml::MsXml::IXMLDOMDocument2>) -> windows_core::Result<super::super::Data::Xml::MsXml::IXMLDOMDocument2>;
}
#[cfg(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
impl windows_core::RuntimeName for IPrintTicketProvider2 {}
#[cfg(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
impl IPrintTicketProvider2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IPrintTicketProvider2_Vtbl
    where
        Identity: IPrintTicketProvider2_Impl,
    {
        unsafe extern "system" fn GetPrintDeviceCapabilities<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pprintticket: *mut core::ffi::c_void, ppdevicecapabilities: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPrintTicketProvider2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPrintTicketProvider2_Impl::GetPrintDeviceCapabilities(this, windows_core::from_raw_borrowed(&pprintticket)) {
                Ok(ok__) => {
                    ppdevicecapabilities.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPrintDeviceResources<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszlocalename: windows_core::PCWSTR, pprintticket: *mut core::ffi::c_void, ppdeviceresources: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPrintTicketProvider2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPrintTicketProvider2_Impl::GetPrintDeviceResources(this, core::mem::transmute(&pszlocalename), windows_core::from_raw_borrowed(&pprintticket)) {
                Ok(ok__) => {
                    ppdeviceresources.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: IPrintTicketProvider_Vtbl::new::<Identity, OFFSET>(),
            GetPrintDeviceCapabilities: GetPrintDeviceCapabilities::<Identity, OFFSET>,
            GetPrintDeviceResources: GetPrintDeviceResources::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPrintTicketProvider2 as windows_core::Interface>::IID || iid == &<IPrintTicketProvider as windows_core::Interface>::IID
    }
}
pub trait IPrintUnidiAsyncNotifyRegistration_Impl: Sized + IPrintAsyncNotifyRegistration_Impl {
    fn AsyncGetNotification(&self, param0: Option<&IAsyncGetSendNotificationCookie>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IPrintUnidiAsyncNotifyRegistration {}
impl IPrintUnidiAsyncNotifyRegistration_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IPrintUnidiAsyncNotifyRegistration_Vtbl
    where
        Identity: IPrintUnidiAsyncNotifyRegistration_Impl,
    {
        unsafe extern "system" fn AsyncGetNotification<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPrintUnidiAsyncNotifyRegistration_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPrintUnidiAsyncNotifyRegistration_Impl::AsyncGetNotification(this, windows_core::from_raw_borrowed(&param0)).into()
        }
        Self { base__: IPrintAsyncNotifyRegistration_Vtbl::new::<Identity, OFFSET>(), AsyncGetNotification: AsyncGetNotification::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPrintUnidiAsyncNotifyRegistration as windows_core::Interface>::IID || iid == &<IPrintAsyncNotifyRegistration as windows_core::Interface>::IID
    }
}
pub trait IPrintWriteStream_Impl: Sized {
    fn WriteBytes(&self, pvbuffer: *const core::ffi::c_void, cbbuffer: u32) -> windows_core::Result<u32>;
    fn Close(&self);
}
impl windows_core::RuntimeName for IPrintWriteStream {}
impl IPrintWriteStream_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IPrintWriteStream_Vtbl
    where
        Identity: IPrintWriteStream_Impl,
    {
        unsafe extern "system" fn WriteBytes<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvbuffer: *const core::ffi::c_void, cbbuffer: u32, pcbwritten: *mut u32) -> windows_core::HRESULT
        where
            Identity: IPrintWriteStream_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPrintWriteStream_Impl::WriteBytes(this, core::mem::transmute_copy(&pvbuffer), core::mem::transmute_copy(&cbbuffer)) {
                Ok(ok__) => {
                    pcbwritten.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Close<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void)
        where
            Identity: IPrintWriteStream_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPrintWriteStream_Impl::Close(this)
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), WriteBytes: WriteBytes::<Identity, OFFSET>, Close: Close::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPrintWriteStream as windows_core::Interface>::IID
    }
}
pub trait IPrintWriteStreamFlush_Impl: Sized {
    fn FlushData(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IPrintWriteStreamFlush {}
impl IPrintWriteStreamFlush_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IPrintWriteStreamFlush_Vtbl
    where
        Identity: IPrintWriteStreamFlush_Impl,
    {
        unsafe extern "system" fn FlushData<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPrintWriteStreamFlush_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPrintWriteStreamFlush_Impl::FlushData(this).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), FlushData: FlushData::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPrintWriteStreamFlush as windows_core::Interface>::IID
    }
}
pub trait IPrinterBidiSetRequestCallback_Impl: Sized {
    fn Completed(&self, bstrresponse: &windows_core::BSTR, hrstatus: windows_core::HRESULT) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IPrinterBidiSetRequestCallback {}
impl IPrinterBidiSetRequestCallback_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IPrinterBidiSetRequestCallback_Vtbl
    where
        Identity: IPrinterBidiSetRequestCallback_Impl,
    {
        unsafe extern "system" fn Completed<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrresponse: core::mem::MaybeUninit<windows_core::BSTR>, hrstatus: windows_core::HRESULT) -> windows_core::HRESULT
        where
            Identity: IPrinterBidiSetRequestCallback_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPrinterBidiSetRequestCallback_Impl::Completed(this, core::mem::transmute(&bstrresponse), core::mem::transmute_copy(&hrstatus)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Completed: Completed::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPrinterBidiSetRequestCallback as windows_core::Interface>::IID
    }
}
pub trait IPrinterExtensionAsyncOperation_Impl: Sized {
    fn Cancel(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IPrinterExtensionAsyncOperation {}
impl IPrinterExtensionAsyncOperation_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IPrinterExtensionAsyncOperation_Vtbl
    where
        Identity: IPrinterExtensionAsyncOperation_Impl,
    {
        unsafe extern "system" fn Cancel<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPrinterExtensionAsyncOperation_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPrinterExtensionAsyncOperation_Impl::Cancel(this).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Cancel: Cancel::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPrinterExtensionAsyncOperation as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IPrinterExtensionContext_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn PrinterQueue(&self) -> windows_core::Result<IPrinterQueue>;
    fn PrintSchemaTicket(&self) -> windows_core::Result<IPrintSchemaTicket>;
    fn DriverProperties(&self) -> windows_core::Result<IPrinterPropertyBag>;
    fn UserProperties(&self) -> windows_core::Result<IPrinterPropertyBag>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IPrinterExtensionContext {}
#[cfg(feature = "Win32_System_Com")]
impl IPrinterExtensionContext_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IPrinterExtensionContext_Vtbl
    where
        Identity: IPrinterExtensionContext_Impl,
    {
        unsafe extern "system" fn PrinterQueue<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppqueue: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPrinterExtensionContext_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPrinterExtensionContext_Impl::PrinterQueue(this) {
                Ok(ok__) => {
                    ppqueue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PrintSchemaTicket<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppticket: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPrinterExtensionContext_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPrinterExtensionContext_Impl::PrintSchemaTicket(this) {
                Ok(ok__) => {
                    ppticket.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DriverProperties<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pppropertybag: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPrinterExtensionContext_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPrinterExtensionContext_Impl::DriverProperties(this) {
                Ok(ok__) => {
                    pppropertybag.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UserProperties<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pppropertybag: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPrinterExtensionContext_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPrinterExtensionContext_Impl::UserProperties(this) {
                Ok(ok__) => {
                    pppropertybag.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            PrinterQueue: PrinterQueue::<Identity, OFFSET>,
            PrintSchemaTicket: PrintSchemaTicket::<Identity, OFFSET>,
            DriverProperties: DriverProperties::<Identity, OFFSET>,
            UserProperties: UserProperties::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPrinterExtensionContext as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IPrinterExtensionContextCollection_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Count(&self) -> windows_core::Result<u32>;
    fn GetAt(&self, ulindex: u32) -> windows_core::Result<IPrinterExtensionContext>;
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IPrinterExtensionContextCollection {}
#[cfg(feature = "Win32_System_Com")]
impl IPrinterExtensionContextCollection_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IPrinterExtensionContextCollection_Vtbl
    where
        Identity: IPrinterExtensionContextCollection_Impl,
    {
        unsafe extern "system" fn Count<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pulcount: *mut u32) -> windows_core::HRESULT
        where
            Identity: IPrinterExtensionContextCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPrinterExtensionContextCollection_Impl::Count(this) {
                Ok(ok__) => {
                    pulcount.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAt<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulindex: u32, ppcontext: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPrinterExtensionContextCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPrinterExtensionContextCollection_Impl::GetAt(this, core::mem::transmute_copy(&ulindex)) {
                Ok(ok__) => {
                    ppcontext.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppunk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPrinterExtensionContextCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPrinterExtensionContextCollection_Impl::_NewEnum(this) {
                Ok(ok__) => {
                    ppunk.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Count: Count::<Identity, OFFSET>,
            GetAt: GetAt::<Identity, OFFSET>,
            _NewEnum: _NewEnum::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPrinterExtensionContextCollection as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IPrinterExtensionEvent_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn OnDriverEvent(&self, peventargs: Option<&IPrinterExtensionEventArgs>) -> windows_core::Result<()>;
    fn OnPrinterQueuesEnumerated(&self, pcontextcollection: Option<&IPrinterExtensionContextCollection>) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IPrinterExtensionEvent {}
#[cfg(feature = "Win32_System_Com")]
impl IPrinterExtensionEvent_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IPrinterExtensionEvent_Vtbl
    where
        Identity: IPrinterExtensionEvent_Impl,
    {
        unsafe extern "system" fn OnDriverEvent<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, peventargs: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPrinterExtensionEvent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPrinterExtensionEvent_Impl::OnDriverEvent(this, windows_core::from_raw_borrowed(&peventargs)).into()
        }
        unsafe extern "system" fn OnPrinterQueuesEnumerated<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcontextcollection: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPrinterExtensionEvent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPrinterExtensionEvent_Impl::OnPrinterQueuesEnumerated(this, windows_core::from_raw_borrowed(&pcontextcollection)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            OnDriverEvent: OnDriverEvent::<Identity, OFFSET>,
            OnPrinterQueuesEnumerated: OnPrinterQueuesEnumerated::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPrinterExtensionEvent as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IPrinterExtensionEventArgs_Impl: Sized + IPrinterExtensionContext_Impl {
    fn BidiNotification(&self) -> windows_core::Result<windows_core::BSTR>;
    fn ReasonId(&self) -> windows_core::Result<windows_core::GUID>;
    fn Request(&self) -> windows_core::Result<IPrinterExtensionRequest>;
    fn SourceApplication(&self) -> windows_core::Result<windows_core::BSTR>;
    fn DetailedReasonId(&self) -> windows_core::Result<windows_core::GUID>;
    fn WindowModal(&self) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn WindowParent(&self) -> windows_core::Result<super::super::Foundation::HANDLE>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IPrinterExtensionEventArgs {}
#[cfg(feature = "Win32_System_Com")]
impl IPrinterExtensionEventArgs_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IPrinterExtensionEventArgs_Vtbl
    where
        Identity: IPrinterExtensionEventArgs_Impl,
    {
        unsafe extern "system" fn BidiNotification<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrbidinotification: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IPrinterExtensionEventArgs_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPrinterExtensionEventArgs_Impl::BidiNotification(this) {
                Ok(ok__) => {
                    pbstrbidinotification.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReasonId<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, preasonid: *mut windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IPrinterExtensionEventArgs_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPrinterExtensionEventArgs_Impl::ReasonId(this) {
                Ok(ok__) => {
                    preasonid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Request<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pprequest: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPrinterExtensionEventArgs_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPrinterExtensionEventArgs_Impl::Request(this) {
                Ok(ok__) => {
                    pprequest.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SourceApplication<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrapplication: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IPrinterExtensionEventArgs_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPrinterExtensionEventArgs_Impl::SourceApplication(this) {
                Ok(ok__) => {
                    pbstrapplication.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DetailedReasonId<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdetailedreasonid: *mut windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IPrinterExtensionEventArgs_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPrinterExtensionEventArgs_Impl::DetailedReasonId(this) {
                Ok(ok__) => {
                    pdetailedreasonid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WindowModal<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbmodal: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IPrinterExtensionEventArgs_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPrinterExtensionEventArgs_Impl::WindowModal(this) {
                Ok(ok__) => {
                    pbmodal.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WindowParent<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, phwndparent: *mut super::super::Foundation::HANDLE) -> windows_core::HRESULT
        where
            Identity: IPrinterExtensionEventArgs_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPrinterExtensionEventArgs_Impl::WindowParent(this) {
                Ok(ok__) => {
                    phwndparent.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: IPrinterExtensionContext_Vtbl::new::<Identity, OFFSET>(),
            BidiNotification: BidiNotification::<Identity, OFFSET>,
            ReasonId: ReasonId::<Identity, OFFSET>,
            Request: Request::<Identity, OFFSET>,
            SourceApplication: SourceApplication::<Identity, OFFSET>,
            DetailedReasonId: DetailedReasonId::<Identity, OFFSET>,
            WindowModal: WindowModal::<Identity, OFFSET>,
            WindowParent: WindowParent::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPrinterExtensionEventArgs as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<IPrinterExtensionContext as windows_core::Interface>::IID
    }
}
pub trait IPrinterExtensionManager_Impl: Sized {
    fn EnableEvents(&self, printerdriverid: &windows_core::GUID) -> windows_core::Result<()>;
    fn DisableEvents(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IPrinterExtensionManager {}
impl IPrinterExtensionManager_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IPrinterExtensionManager_Vtbl
    where
        Identity: IPrinterExtensionManager_Impl,
    {
        unsafe extern "system" fn EnableEvents<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, printerdriverid: windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IPrinterExtensionManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPrinterExtensionManager_Impl::EnableEvents(this, core::mem::transmute(&printerdriverid)).into()
        }
        unsafe extern "system" fn DisableEvents<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPrinterExtensionManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPrinterExtensionManager_Impl::DisableEvents(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            EnableEvents: EnableEvents::<Identity, OFFSET>,
            DisableEvents: DisableEvents::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPrinterExtensionManager as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IPrinterExtensionRequest_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Cancel(&self, hrstatus: windows_core::HRESULT, bstrlogmessage: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Complete(&self) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IPrinterExtensionRequest {}
#[cfg(feature = "Win32_System_Com")]
impl IPrinterExtensionRequest_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IPrinterExtensionRequest_Vtbl
    where
        Identity: IPrinterExtensionRequest_Impl,
    {
        unsafe extern "system" fn Cancel<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hrstatus: windows_core::HRESULT, bstrlogmessage: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IPrinterExtensionRequest_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPrinterExtensionRequest_Impl::Cancel(this, core::mem::transmute_copy(&hrstatus), core::mem::transmute(&bstrlogmessage)).into()
        }
        unsafe extern "system" fn Complete<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPrinterExtensionRequest_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPrinterExtensionRequest_Impl::Complete(this).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Cancel: Cancel::<Identity, OFFSET>,
            Complete: Complete::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPrinterExtensionRequest as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IPrinterPropertyBag_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn GetBool(&self, bstrname: &windows_core::BSTR) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn SetBool(&self, bstrname: &windows_core::BSTR, bvalue: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn GetInt32(&self, bstrname: &windows_core::BSTR) -> windows_core::Result<i32>;
    fn SetInt32(&self, bstrname: &windows_core::BSTR, nvalue: i32) -> windows_core::Result<()>;
    fn GetString(&self, bstrname: &windows_core::BSTR) -> windows_core::Result<windows_core::BSTR>;
    fn SetString(&self, bstrname: &windows_core::BSTR, bstrvalue: &windows_core::BSTR) -> windows_core::Result<()>;
    fn GetBytes(&self, bstrname: &windows_core::BSTR, pcbvalue: *mut u32, ppvalue: *mut *mut u8) -> windows_core::Result<()>;
    fn SetBytes(&self, bstrname: &windows_core::BSTR, cbvalue: u32, pvalue: *const u8) -> windows_core::Result<()>;
    fn GetReadStream(&self, bstrname: &windows_core::BSTR) -> windows_core::Result<super::super::System::Com::IStream>;
    fn GetWriteStream(&self, bstrname: &windows_core::BSTR) -> windows_core::Result<super::super::System::Com::IStream>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IPrinterPropertyBag {}
#[cfg(feature = "Win32_System_Com")]
impl IPrinterPropertyBag_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IPrinterPropertyBag_Vtbl
    where
        Identity: IPrinterPropertyBag_Impl,
    {
        unsafe extern "system" fn GetBool<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrname: core::mem::MaybeUninit<windows_core::BSTR>, pbvalue: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IPrinterPropertyBag_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPrinterPropertyBag_Impl::GetBool(this, core::mem::transmute(&bstrname)) {
                Ok(ok__) => {
                    pbvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBool<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrname: core::mem::MaybeUninit<windows_core::BSTR>, bvalue: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IPrinterPropertyBag_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPrinterPropertyBag_Impl::SetBool(this, core::mem::transmute(&bstrname), core::mem::transmute_copy(&bvalue)).into()
        }
        unsafe extern "system" fn GetInt32<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrname: core::mem::MaybeUninit<windows_core::BSTR>, pnvalue: *mut i32) -> windows_core::HRESULT
        where
            Identity: IPrinterPropertyBag_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPrinterPropertyBag_Impl::GetInt32(this, core::mem::transmute(&bstrname)) {
                Ok(ok__) => {
                    pnvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInt32<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrname: core::mem::MaybeUninit<windows_core::BSTR>, nvalue: i32) -> windows_core::HRESULT
        where
            Identity: IPrinterPropertyBag_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPrinterPropertyBag_Impl::SetInt32(this, core::mem::transmute(&bstrname), core::mem::transmute_copy(&nvalue)).into()
        }
        unsafe extern "system" fn GetString<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrname: core::mem::MaybeUninit<windows_core::BSTR>, pbstrvalue: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IPrinterPropertyBag_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPrinterPropertyBag_Impl::GetString(this, core::mem::transmute(&bstrname)) {
                Ok(ok__) => {
                    pbstrvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetString<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrname: core::mem::MaybeUninit<windows_core::BSTR>, bstrvalue: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IPrinterPropertyBag_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPrinterPropertyBag_Impl::SetString(this, core::mem::transmute(&bstrname), core::mem::transmute(&bstrvalue)).into()
        }
        unsafe extern "system" fn GetBytes<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrname: core::mem::MaybeUninit<windows_core::BSTR>, pcbvalue: *mut u32, ppvalue: *mut *mut u8) -> windows_core::HRESULT
        where
            Identity: IPrinterPropertyBag_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPrinterPropertyBag_Impl::GetBytes(this, core::mem::transmute(&bstrname), core::mem::transmute_copy(&pcbvalue), core::mem::transmute_copy(&ppvalue)).into()
        }
        unsafe extern "system" fn SetBytes<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrname: core::mem::MaybeUninit<windows_core::BSTR>, cbvalue: u32, pvalue: *const u8) -> windows_core::HRESULT
        where
            Identity: IPrinterPropertyBag_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPrinterPropertyBag_Impl::SetBytes(this, core::mem::transmute(&bstrname), core::mem::transmute_copy(&cbvalue), core::mem::transmute_copy(&pvalue)).into()
        }
        unsafe extern "system" fn GetReadStream<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrname: core::mem::MaybeUninit<windows_core::BSTR>, ppvalue: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPrinterPropertyBag_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPrinterPropertyBag_Impl::GetReadStream(this, core::mem::transmute(&bstrname)) {
                Ok(ok__) => {
                    ppvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetWriteStream<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrname: core::mem::MaybeUninit<windows_core::BSTR>, ppvalue: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPrinterPropertyBag_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPrinterPropertyBag_Impl::GetWriteStream(this, core::mem::transmute(&bstrname)) {
                Ok(ok__) => {
                    ppvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            GetBool: GetBool::<Identity, OFFSET>,
            SetBool: SetBool::<Identity, OFFSET>,
            GetInt32: GetInt32::<Identity, OFFSET>,
            SetInt32: SetInt32::<Identity, OFFSET>,
            GetString: GetString::<Identity, OFFSET>,
            SetString: SetString::<Identity, OFFSET>,
            GetBytes: GetBytes::<Identity, OFFSET>,
            SetBytes: SetBytes::<Identity, OFFSET>,
            GetReadStream: GetReadStream::<Identity, OFFSET>,
            GetWriteStream: GetWriteStream::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPrinterPropertyBag as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IPrinterQueue_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Handle(&self) -> windows_core::Result<super::super::Foundation::HANDLE>;
    fn Name(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SendBidiQuery(&self, bstrbidiquery: &windows_core::BSTR) -> windows_core::Result<()>;
    fn GetProperties(&self) -> windows_core::Result<IPrinterPropertyBag>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IPrinterQueue {}
#[cfg(feature = "Win32_System_Com")]
impl IPrinterQueue_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IPrinterQueue_Vtbl
    where
        Identity: IPrinterQueue_Impl,
    {
        unsafe extern "system" fn Handle<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, phprinter: *mut super::super::Foundation::HANDLE) -> windows_core::HRESULT
        where
            Identity: IPrinterQueue_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPrinterQueue_Impl::Handle(this) {
                Ok(ok__) => {
                    phprinter.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Name<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrname: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IPrinterQueue_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPrinterQueue_Impl::Name(this) {
                Ok(ok__) => {
                    pbstrname.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SendBidiQuery<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrbidiquery: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IPrinterQueue_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPrinterQueue_Impl::SendBidiQuery(this, core::mem::transmute(&bstrbidiquery)).into()
        }
        unsafe extern "system" fn GetProperties<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pppropertybag: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPrinterQueue_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPrinterQueue_Impl::GetProperties(this) {
                Ok(ok__) => {
                    pppropertybag.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Handle: Handle::<Identity, OFFSET>,
            Name: Name::<Identity, OFFSET>,
            SendBidiQuery: SendBidiQuery::<Identity, OFFSET>,
            GetProperties: GetProperties::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPrinterQueue as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IPrinterQueue2_Impl: Sized + IPrinterQueue_Impl {
    fn SendBidiSetRequestAsync(&self, bstrbidirequest: &windows_core::BSTR, pcallback: Option<&IPrinterBidiSetRequestCallback>) -> windows_core::Result<IPrinterExtensionAsyncOperation>;
    fn GetPrinterQueueView(&self, ulviewoffset: u32, ulviewsize: u32) -> windows_core::Result<IPrinterQueueView>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IPrinterQueue2 {}
#[cfg(feature = "Win32_System_Com")]
impl IPrinterQueue2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IPrinterQueue2_Vtbl
    where
        Identity: IPrinterQueue2_Impl,
    {
        unsafe extern "system" fn SendBidiSetRequestAsync<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrbidirequest: core::mem::MaybeUninit<windows_core::BSTR>, pcallback: *mut core::ffi::c_void, ppasyncoperation: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPrinterQueue2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPrinterQueue2_Impl::SendBidiSetRequestAsync(this, core::mem::transmute(&bstrbidirequest), windows_core::from_raw_borrowed(&pcallback)) {
                Ok(ok__) => {
                    ppasyncoperation.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPrinterQueueView<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulviewoffset: u32, ulviewsize: u32, ppjobview: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPrinterQueue2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPrinterQueue2_Impl::GetPrinterQueueView(this, core::mem::transmute_copy(&ulviewoffset), core::mem::transmute_copy(&ulviewsize)) {
                Ok(ok__) => {
                    ppjobview.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: IPrinterQueue_Vtbl::new::<Identity, OFFSET>(),
            SendBidiSetRequestAsync: SendBidiSetRequestAsync::<Identity, OFFSET>,
            GetPrinterQueueView: GetPrinterQueueView::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPrinterQueue2 as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<IPrinterQueue as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IPrinterQueueEvent_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn OnBidiResponseReceived(&self, bstrresponse: &windows_core::BSTR, hrstatus: windows_core::HRESULT) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IPrinterQueueEvent {}
#[cfg(feature = "Win32_System_Com")]
impl IPrinterQueueEvent_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IPrinterQueueEvent_Vtbl
    where
        Identity: IPrinterQueueEvent_Impl,
    {
        unsafe extern "system" fn OnBidiResponseReceived<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrresponse: core::mem::MaybeUninit<windows_core::BSTR>, hrstatus: windows_core::HRESULT) -> windows_core::HRESULT
        where
            Identity: IPrinterQueueEvent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPrinterQueueEvent_Impl::OnBidiResponseReceived(this, core::mem::transmute(&bstrresponse), core::mem::transmute_copy(&hrstatus)).into()
        }
        Self { base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(), OnBidiResponseReceived: OnBidiResponseReceived::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPrinterQueueEvent as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IPrinterQueueView_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn SetViewRange(&self, ulviewoffset: u32, ulviewsize: u32) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IPrinterQueueView {}
#[cfg(feature = "Win32_System_Com")]
impl IPrinterQueueView_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IPrinterQueueView_Vtbl
    where
        Identity: IPrinterQueueView_Impl,
    {
        unsafe extern "system" fn SetViewRange<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulviewoffset: u32, ulviewsize: u32) -> windows_core::HRESULT
        where
            Identity: IPrinterQueueView_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPrinterQueueView_Impl::SetViewRange(this, core::mem::transmute_copy(&ulviewoffset), core::mem::transmute_copy(&ulviewsize)).into()
        }
        Self { base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(), SetViewRange: SetViewRange::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPrinterQueueView as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IPrinterQueueViewEvent_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn OnChanged(&self, pcollection: Option<&IPrintJobCollection>, ulviewoffset: u32, ulviewsize: u32, ulcountjobsinprintqueue: u32) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IPrinterQueueViewEvent {}
#[cfg(feature = "Win32_System_Com")]
impl IPrinterQueueViewEvent_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IPrinterQueueViewEvent_Vtbl
    where
        Identity: IPrinterQueueViewEvent_Impl,
    {
        unsafe extern "system" fn OnChanged<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcollection: *mut core::ffi::c_void, ulviewoffset: u32, ulviewsize: u32, ulcountjobsinprintqueue: u32) -> windows_core::HRESULT
        where
            Identity: IPrinterQueueViewEvent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPrinterQueueViewEvent_Impl::OnChanged(this, windows_core::from_raw_borrowed(&pcollection), core::mem::transmute_copy(&ulviewoffset), core::mem::transmute_copy(&ulviewsize), core::mem::transmute_copy(&ulcountjobsinprintqueue)).into()
        }
        Self { base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(), OnChanged: OnChanged::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPrinterQueueViewEvent as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IPrinterScriptContext_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn DriverProperties(&self) -> windows_core::Result<IPrinterScriptablePropertyBag>;
    fn QueueProperties(&self) -> windows_core::Result<IPrinterScriptablePropertyBag>;
    fn UserProperties(&self) -> windows_core::Result<IPrinterScriptablePropertyBag>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IPrinterScriptContext {}
#[cfg(feature = "Win32_System_Com")]
impl IPrinterScriptContext_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IPrinterScriptContext_Vtbl
    where
        Identity: IPrinterScriptContext_Impl,
    {
        unsafe extern "system" fn DriverProperties<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pppropertybag: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPrinterScriptContext_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPrinterScriptContext_Impl::DriverProperties(this) {
                Ok(ok__) => {
                    pppropertybag.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueueProperties<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pppropertybag: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPrinterScriptContext_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPrinterScriptContext_Impl::QueueProperties(this) {
                Ok(ok__) => {
                    pppropertybag.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UserProperties<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pppropertybag: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPrinterScriptContext_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPrinterScriptContext_Impl::UserProperties(this) {
                Ok(ok__) => {
                    pppropertybag.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            DriverProperties: DriverProperties::<Identity, OFFSET>,
            QueueProperties: QueueProperties::<Identity, OFFSET>,
            UserProperties: UserProperties::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPrinterScriptContext as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IPrinterScriptablePropertyBag_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn GetBool(&self, bstrname: &windows_core::BSTR) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn SetBool(&self, bstrname: &windows_core::BSTR, bvalue: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn GetInt32(&self, bstrname: &windows_core::BSTR) -> windows_core::Result<i32>;
    fn SetInt32(&self, bstrname: &windows_core::BSTR, nvalue: i32) -> windows_core::Result<()>;
    fn GetString(&self, bstrname: &windows_core::BSTR) -> windows_core::Result<windows_core::BSTR>;
    fn SetString(&self, bstrname: &windows_core::BSTR, bstrvalue: &windows_core::BSTR) -> windows_core::Result<()>;
    fn GetBytes(&self, bstrname: &windows_core::BSTR) -> windows_core::Result<super::super::System::Com::IDispatch>;
    fn SetBytes(&self, bstrname: &windows_core::BSTR, parray: Option<&super::super::System::Com::IDispatch>) -> windows_core::Result<()>;
    fn GetReadStream(&self, bstrname: &windows_core::BSTR) -> windows_core::Result<IPrinterScriptableStream>;
    fn GetWriteStream(&self, bstrname: &windows_core::BSTR) -> windows_core::Result<IPrinterScriptableStream>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IPrinterScriptablePropertyBag {}
#[cfg(feature = "Win32_System_Com")]
impl IPrinterScriptablePropertyBag_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IPrinterScriptablePropertyBag_Vtbl
    where
        Identity: IPrinterScriptablePropertyBag_Impl,
    {
        unsafe extern "system" fn GetBool<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrname: core::mem::MaybeUninit<windows_core::BSTR>, pbvalue: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IPrinterScriptablePropertyBag_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPrinterScriptablePropertyBag_Impl::GetBool(this, core::mem::transmute(&bstrname)) {
                Ok(ok__) => {
                    pbvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBool<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrname: core::mem::MaybeUninit<windows_core::BSTR>, bvalue: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IPrinterScriptablePropertyBag_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPrinterScriptablePropertyBag_Impl::SetBool(this, core::mem::transmute(&bstrname), core::mem::transmute_copy(&bvalue)).into()
        }
        unsafe extern "system" fn GetInt32<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrname: core::mem::MaybeUninit<windows_core::BSTR>, pnvalue: *mut i32) -> windows_core::HRESULT
        where
            Identity: IPrinterScriptablePropertyBag_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPrinterScriptablePropertyBag_Impl::GetInt32(this, core::mem::transmute(&bstrname)) {
                Ok(ok__) => {
                    pnvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInt32<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrname: core::mem::MaybeUninit<windows_core::BSTR>, nvalue: i32) -> windows_core::HRESULT
        where
            Identity: IPrinterScriptablePropertyBag_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPrinterScriptablePropertyBag_Impl::SetInt32(this, core::mem::transmute(&bstrname), core::mem::transmute_copy(&nvalue)).into()
        }
        unsafe extern "system" fn GetString<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrname: core::mem::MaybeUninit<windows_core::BSTR>, pbstrvalue: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IPrinterScriptablePropertyBag_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPrinterScriptablePropertyBag_Impl::GetString(this, core::mem::transmute(&bstrname)) {
                Ok(ok__) => {
                    pbstrvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetString<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrname: core::mem::MaybeUninit<windows_core::BSTR>, bstrvalue: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IPrinterScriptablePropertyBag_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPrinterScriptablePropertyBag_Impl::SetString(this, core::mem::transmute(&bstrname), core::mem::transmute(&bstrvalue)).into()
        }
        unsafe extern "system" fn GetBytes<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrname: core::mem::MaybeUninit<windows_core::BSTR>, pparray: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPrinterScriptablePropertyBag_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPrinterScriptablePropertyBag_Impl::GetBytes(this, core::mem::transmute(&bstrname)) {
                Ok(ok__) => {
                    pparray.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBytes<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrname: core::mem::MaybeUninit<windows_core::BSTR>, parray: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPrinterScriptablePropertyBag_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPrinterScriptablePropertyBag_Impl::SetBytes(this, core::mem::transmute(&bstrname), windows_core::from_raw_borrowed(&parray)).into()
        }
        unsafe extern "system" fn GetReadStream<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrname: core::mem::MaybeUninit<windows_core::BSTR>, ppstream: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPrinterScriptablePropertyBag_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPrinterScriptablePropertyBag_Impl::GetReadStream(this, core::mem::transmute(&bstrname)) {
                Ok(ok__) => {
                    ppstream.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetWriteStream<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrname: core::mem::MaybeUninit<windows_core::BSTR>, ppstream: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPrinterScriptablePropertyBag_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPrinterScriptablePropertyBag_Impl::GetWriteStream(this, core::mem::transmute(&bstrname)) {
                Ok(ok__) => {
                    ppstream.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            GetBool: GetBool::<Identity, OFFSET>,
            SetBool: SetBool::<Identity, OFFSET>,
            GetInt32: GetInt32::<Identity, OFFSET>,
            SetInt32: SetInt32::<Identity, OFFSET>,
            GetString: GetString::<Identity, OFFSET>,
            SetString: SetString::<Identity, OFFSET>,
            GetBytes: GetBytes::<Identity, OFFSET>,
            SetBytes: SetBytes::<Identity, OFFSET>,
            GetReadStream: GetReadStream::<Identity, OFFSET>,
            GetWriteStream: GetWriteStream::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPrinterScriptablePropertyBag as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IPrinterScriptablePropertyBag2_Impl: Sized + IPrinterScriptablePropertyBag_Impl {
    fn GetReadStreamAsXML(&self, bstrname: &windows_core::BSTR) -> windows_core::Result<windows_core::IUnknown>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IPrinterScriptablePropertyBag2 {}
#[cfg(feature = "Win32_System_Com")]
impl IPrinterScriptablePropertyBag2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IPrinterScriptablePropertyBag2_Vtbl
    where
        Identity: IPrinterScriptablePropertyBag2_Impl,
    {
        unsafe extern "system" fn GetReadStreamAsXML<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrname: core::mem::MaybeUninit<windows_core::BSTR>, ppxmlnode: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPrinterScriptablePropertyBag2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPrinterScriptablePropertyBag2_Impl::GetReadStreamAsXML(this, core::mem::transmute(&bstrname)) {
                Ok(ok__) => {
                    ppxmlnode.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: IPrinterScriptablePropertyBag_Vtbl::new::<Identity, OFFSET>(), GetReadStreamAsXML: GetReadStreamAsXML::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPrinterScriptablePropertyBag2 as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<IPrinterScriptablePropertyBag as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IPrinterScriptableSequentialStream_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Read(&self, cbread: i32) -> windows_core::Result<super::super::System::Com::IDispatch>;
    fn Write(&self, parray: Option<&super::super::System::Com::IDispatch>) -> windows_core::Result<i32>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IPrinterScriptableSequentialStream {}
#[cfg(feature = "Win32_System_Com")]
impl IPrinterScriptableSequentialStream_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IPrinterScriptableSequentialStream_Vtbl
    where
        Identity: IPrinterScriptableSequentialStream_Impl,
    {
        unsafe extern "system" fn Read<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, cbread: i32, pparray: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPrinterScriptableSequentialStream_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPrinterScriptableSequentialStream_Impl::Read(this, core::mem::transmute_copy(&cbread)) {
                Ok(ok__) => {
                    pparray.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Write<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, parray: *mut core::ffi::c_void, pcbwritten: *mut i32) -> windows_core::HRESULT
        where
            Identity: IPrinterScriptableSequentialStream_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPrinterScriptableSequentialStream_Impl::Write(this, windows_core::from_raw_borrowed(&parray)) {
                Ok(ok__) => {
                    pcbwritten.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(), Read: Read::<Identity, OFFSET>, Write: Write::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPrinterScriptableSequentialStream as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IPrinterScriptableStream_Impl: Sized + IPrinterScriptableSequentialStream_Impl {
    fn Commit(&self) -> windows_core::Result<()>;
    fn Seek(&self, loffset: i32, streamseek: super::super::System::Com::STREAM_SEEK) -> windows_core::Result<i32>;
    fn SetSize(&self, lsize: i32) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IPrinterScriptableStream {}
#[cfg(feature = "Win32_System_Com")]
impl IPrinterScriptableStream_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IPrinterScriptableStream_Vtbl
    where
        Identity: IPrinterScriptableStream_Impl,
    {
        unsafe extern "system" fn Commit<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPrinterScriptableStream_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPrinterScriptableStream_Impl::Commit(this).into()
        }
        unsafe extern "system" fn Seek<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, loffset: i32, streamseek: super::super::System::Com::STREAM_SEEK, plposition: *mut i32) -> windows_core::HRESULT
        where
            Identity: IPrinterScriptableStream_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPrinterScriptableStream_Impl::Seek(this, core::mem::transmute_copy(&loffset), core::mem::transmute_copy(&streamseek)) {
                Ok(ok__) => {
                    plposition.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lsize: i32) -> windows_core::HRESULT
        where
            Identity: IPrinterScriptableStream_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPrinterScriptableStream_Impl::SetSize(this, core::mem::transmute_copy(&lsize)).into()
        }
        Self {
            base__: IPrinterScriptableSequentialStream_Vtbl::new::<Identity, OFFSET>(),
            Commit: Commit::<Identity, OFFSET>,
            Seek: Seek::<Identity, OFFSET>,
            SetSize: SetSize::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPrinterScriptableStream as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<IPrinterScriptableSequentialStream as windows_core::Interface>::IID
    }
}
pub trait IXpsDocument_Impl: Sized {
    fn GetThumbnail(&self) -> windows_core::Result<IPartThumbnail>;
    fn SetThumbnail(&self, pthumbnail: Option<&IPartThumbnail>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IXpsDocument {}
impl IXpsDocument_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IXpsDocument_Vtbl
    where
        Identity: IXpsDocument_Impl,
    {
        unsafe extern "system" fn GetThumbnail<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppthumbnail: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsDocument_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsDocument_Impl::GetThumbnail(this) {
                Ok(ok__) => {
                    ppthumbnail.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetThumbnail<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pthumbnail: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsDocument_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXpsDocument_Impl::SetThumbnail(this, windows_core::from_raw_borrowed(&pthumbnail)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetThumbnail: GetThumbnail::<Identity, OFFSET>,
            SetThumbnail: SetThumbnail::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXpsDocument as windows_core::Interface>::IID
    }
}
pub trait IXpsDocumentConsumer_Impl: Sized {
    fn SendXpsUnknown(&self, punknown: Option<&windows_core::IUnknown>) -> windows_core::Result<()>;
    fn SendXpsDocument(&self, pixpsdocument: Option<&IXpsDocument>) -> windows_core::Result<()>;
    fn SendFixedDocumentSequence(&self, pifixeddocumentsequence: Option<&IFixedDocumentSequence>) -> windows_core::Result<()>;
    fn SendFixedDocument(&self, pifixeddocument: Option<&IFixedDocument>) -> windows_core::Result<()>;
    fn SendFixedPage(&self, pifixedpage: Option<&IFixedPage>) -> windows_core::Result<()>;
    fn CloseSender(&self) -> windows_core::Result<()>;
    fn GetNewEmptyPart(&self, uri: &windows_core::PCWSTR, riid: *const windows_core::GUID, ppnewobject: *mut *mut core::ffi::c_void, ppwritestream: *mut Option<IPrintWriteStream>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IXpsDocumentConsumer {}
impl IXpsDocumentConsumer_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IXpsDocumentConsumer_Vtbl
    where
        Identity: IXpsDocumentConsumer_Impl,
    {
        unsafe extern "system" fn SendXpsUnknown<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, punknown: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsDocumentConsumer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXpsDocumentConsumer_Impl::SendXpsUnknown(this, windows_core::from_raw_borrowed(&punknown)).into()
        }
        unsafe extern "system" fn SendXpsDocument<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pixpsdocument: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsDocumentConsumer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXpsDocumentConsumer_Impl::SendXpsDocument(this, windows_core::from_raw_borrowed(&pixpsdocument)).into()
        }
        unsafe extern "system" fn SendFixedDocumentSequence<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pifixeddocumentsequence: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsDocumentConsumer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXpsDocumentConsumer_Impl::SendFixedDocumentSequence(this, windows_core::from_raw_borrowed(&pifixeddocumentsequence)).into()
        }
        unsafe extern "system" fn SendFixedDocument<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pifixeddocument: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsDocumentConsumer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXpsDocumentConsumer_Impl::SendFixedDocument(this, windows_core::from_raw_borrowed(&pifixeddocument)).into()
        }
        unsafe extern "system" fn SendFixedPage<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pifixedpage: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsDocumentConsumer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXpsDocumentConsumer_Impl::SendFixedPage(this, windows_core::from_raw_borrowed(&pifixedpage)).into()
        }
        unsafe extern "system" fn CloseSender<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsDocumentConsumer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXpsDocumentConsumer_Impl::CloseSender(this).into()
        }
        unsafe extern "system" fn GetNewEmptyPart<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, uri: windows_core::PCWSTR, riid: *const windows_core::GUID, ppnewobject: *mut *mut core::ffi::c_void, ppwritestream: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsDocumentConsumer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXpsDocumentConsumer_Impl::GetNewEmptyPart(this, core::mem::transmute(&uri), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppnewobject), core::mem::transmute_copy(&ppwritestream)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SendXpsUnknown: SendXpsUnknown::<Identity, OFFSET>,
            SendXpsDocument: SendXpsDocument::<Identity, OFFSET>,
            SendFixedDocumentSequence: SendFixedDocumentSequence::<Identity, OFFSET>,
            SendFixedDocument: SendFixedDocument::<Identity, OFFSET>,
            SendFixedPage: SendFixedPage::<Identity, OFFSET>,
            CloseSender: CloseSender::<Identity, OFFSET>,
            GetNewEmptyPart: GetNewEmptyPart::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXpsDocumentConsumer as windows_core::Interface>::IID
    }
}
pub trait IXpsDocumentProvider_Impl: Sized {
    fn GetXpsPart(&self) -> windows_core::Result<windows_core::IUnknown>;
}
impl windows_core::RuntimeName for IXpsDocumentProvider {}
impl IXpsDocumentProvider_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IXpsDocumentProvider_Vtbl
    where
        Identity: IXpsDocumentProvider_Impl,
    {
        unsafe extern "system" fn GetXpsPart<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppixpspart: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsDocumentProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsDocumentProvider_Impl::GetXpsPart(this) {
                Ok(ok__) => {
                    ppixpspart.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetXpsPart: GetXpsPart::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXpsDocumentProvider as windows_core::Interface>::IID
    }
}
pub trait IXpsPartIterator_Impl: Sized {
    fn Reset(&self);
    fn Current(&self, puri: *mut windows_core::BSTR, ppxpspart: *mut Option<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn IsDone(&self) -> super::super::Foundation::BOOL;
    fn Next(&self);
}
impl windows_core::RuntimeName for IXpsPartIterator {}
impl IXpsPartIterator_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IXpsPartIterator_Vtbl
    where
        Identity: IXpsPartIterator_Impl,
    {
        unsafe extern "system" fn Reset<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void)
        where
            Identity: IXpsPartIterator_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXpsPartIterator_Impl::Reset(this)
        }
        unsafe extern "system" fn Current<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, puri: *mut core::mem::MaybeUninit<windows_core::BSTR>, ppxpspart: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsPartIterator_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXpsPartIterator_Impl::Current(this, core::mem::transmute_copy(&puri), core::mem::transmute_copy(&ppxpspart)).into()
        }
        unsafe extern "system" fn IsDone<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> super::super::Foundation::BOOL
        where
            Identity: IXpsPartIterator_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXpsPartIterator_Impl::IsDone(this)
        }
        unsafe extern "system" fn Next<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void)
        where
            Identity: IXpsPartIterator_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXpsPartIterator_Impl::Next(this)
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Reset: Reset::<Identity, OFFSET>,
            Current: Current::<Identity, OFFSET>,
            IsDone: IsDone::<Identity, OFFSET>,
            Next: Next::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXpsPartIterator as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Storage_Xps")]
pub trait IXpsRasterizationFactory_Impl: Sized {
    fn CreateRasterizer(&self, xpspage: Option<&super::super::Storage::Xps::IXpsOMPage>, dpi: f32, nontextrenderingmode: XPSRAS_RENDERING_MODE, textrenderingmode: XPSRAS_RENDERING_MODE) -> windows_core::Result<IXpsRasterizer>;
}
#[cfg(feature = "Win32_Storage_Xps")]
impl windows_core::RuntimeName for IXpsRasterizationFactory {}
#[cfg(feature = "Win32_Storage_Xps")]
impl IXpsRasterizationFactory_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IXpsRasterizationFactory_Vtbl
    where
        Identity: IXpsRasterizationFactory_Impl,
    {
        unsafe extern "system" fn CreateRasterizer<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, xpspage: *mut core::ffi::c_void, dpi: f32, nontextrenderingmode: XPSRAS_RENDERING_MODE, textrenderingmode: XPSRAS_RENDERING_MODE, ppixpsrasterizer: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsRasterizationFactory_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsRasterizationFactory_Impl::CreateRasterizer(this, windows_core::from_raw_borrowed(&xpspage), core::mem::transmute_copy(&dpi), core::mem::transmute_copy(&nontextrenderingmode), core::mem::transmute_copy(&textrenderingmode)) {
                Ok(ok__) => {
                    ppixpsrasterizer.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), CreateRasterizer: CreateRasterizer::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXpsRasterizationFactory as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Storage_Xps")]
pub trait IXpsRasterizationFactory1_Impl: Sized {
    fn CreateRasterizer(&self, xpspage: Option<&super::super::Storage::Xps::IXpsOMPage>, dpi: f32, nontextrenderingmode: XPSRAS_RENDERING_MODE, textrenderingmode: XPSRAS_RENDERING_MODE, pixelformat: XPSRAS_PIXEL_FORMAT) -> windows_core::Result<IXpsRasterizer>;
}
#[cfg(feature = "Win32_Storage_Xps")]
impl windows_core::RuntimeName for IXpsRasterizationFactory1 {}
#[cfg(feature = "Win32_Storage_Xps")]
impl IXpsRasterizationFactory1_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IXpsRasterizationFactory1_Vtbl
    where
        Identity: IXpsRasterizationFactory1_Impl,
    {
        unsafe extern "system" fn CreateRasterizer<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, xpspage: *mut core::ffi::c_void, dpi: f32, nontextrenderingmode: XPSRAS_RENDERING_MODE, textrenderingmode: XPSRAS_RENDERING_MODE, pixelformat: XPSRAS_PIXEL_FORMAT, ppixpsrasterizer: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsRasterizationFactory1_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsRasterizationFactory1_Impl::CreateRasterizer(this, windows_core::from_raw_borrowed(&xpspage), core::mem::transmute_copy(&dpi), core::mem::transmute_copy(&nontextrenderingmode), core::mem::transmute_copy(&textrenderingmode), core::mem::transmute_copy(&pixelformat)) {
                Ok(ok__) => {
                    ppixpsrasterizer.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), CreateRasterizer: CreateRasterizer::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXpsRasterizationFactory1 as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Storage_Xps")]
pub trait IXpsRasterizationFactory2_Impl: Sized {
    fn CreateRasterizer(&self, xpspage: Option<&super::super::Storage::Xps::IXpsOMPage>, dpix: f32, dpiy: f32, nontextrenderingmode: XPSRAS_RENDERING_MODE, textrenderingmode: XPSRAS_RENDERING_MODE, pixelformat: XPSRAS_PIXEL_FORMAT, backgroundcolor: XPSRAS_BACKGROUND_COLOR) -> windows_core::Result<IXpsRasterizer>;
}
#[cfg(feature = "Win32_Storage_Xps")]
impl windows_core::RuntimeName for IXpsRasterizationFactory2 {}
#[cfg(feature = "Win32_Storage_Xps")]
impl IXpsRasterizationFactory2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IXpsRasterizationFactory2_Vtbl
    where
        Identity: IXpsRasterizationFactory2_Impl,
    {
        unsafe extern "system" fn CreateRasterizer<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, xpspage: *mut core::ffi::c_void, dpix: f32, dpiy: f32, nontextrenderingmode: XPSRAS_RENDERING_MODE, textrenderingmode: XPSRAS_RENDERING_MODE, pixelformat: XPSRAS_PIXEL_FORMAT, backgroundcolor: XPSRAS_BACKGROUND_COLOR, ppixpsrasterizer: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsRasterizationFactory2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsRasterizationFactory2_Impl::CreateRasterizer(this, windows_core::from_raw_borrowed(&xpspage), core::mem::transmute_copy(&dpix), core::mem::transmute_copy(&dpiy), core::mem::transmute_copy(&nontextrenderingmode), core::mem::transmute_copy(&textrenderingmode), core::mem::transmute_copy(&pixelformat), core::mem::transmute_copy(&backgroundcolor)) {
                Ok(ok__) => {
                    ppixpsrasterizer.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), CreateRasterizer: CreateRasterizer::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXpsRasterizationFactory2 as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Imaging")]
pub trait IXpsRasterizer_Impl: Sized {
    fn RasterizeRect(&self, x: i32, y: i32, width: i32, height: i32, notificationcallback: Option<&IXpsRasterizerNotificationCallback>) -> windows_core::Result<super::Imaging::IWICBitmap>;
    fn SetMinimalLineWidth(&self, width: i32) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Imaging")]
impl windows_core::RuntimeName for IXpsRasterizer {}
#[cfg(feature = "Win32_Graphics_Imaging")]
impl IXpsRasterizer_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IXpsRasterizer_Vtbl
    where
        Identity: IXpsRasterizer_Impl,
    {
        unsafe extern "system" fn RasterizeRect<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, x: i32, y: i32, width: i32, height: i32, notificationcallback: *mut core::ffi::c_void, bitmap: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsRasterizer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXpsRasterizer_Impl::RasterizeRect(this, core::mem::transmute_copy(&x), core::mem::transmute_copy(&y), core::mem::transmute_copy(&width), core::mem::transmute_copy(&height), windows_core::from_raw_borrowed(&notificationcallback)) {
                Ok(ok__) => {
                    bitmap.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMinimalLineWidth<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, width: i32) -> windows_core::HRESULT
        where
            Identity: IXpsRasterizer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXpsRasterizer_Impl::SetMinimalLineWidth(this, core::mem::transmute_copy(&width)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            RasterizeRect: RasterizeRect::<Identity, OFFSET>,
            SetMinimalLineWidth: SetMinimalLineWidth::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXpsRasterizer as windows_core::Interface>::IID
    }
}
pub trait IXpsRasterizerNotificationCallback_Impl: Sized {
    fn Continue(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IXpsRasterizerNotificationCallback {}
impl IXpsRasterizerNotificationCallback_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IXpsRasterizerNotificationCallback_Vtbl
    where
        Identity: IXpsRasterizerNotificationCallback_Impl,
    {
        unsafe extern "system" fn Continue<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXpsRasterizerNotificationCallback_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXpsRasterizerNotificationCallback_Impl::Continue(this).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Continue: Continue::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXpsRasterizerNotificationCallback as windows_core::Interface>::IID
    }
}
