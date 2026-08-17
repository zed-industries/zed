pub trait INSNetSourceCreator_Impl: Sized {
    fn Initialize(&self) -> windows_core::Result<()>;
    fn CreateNetSource(&self, pszstreamname: &windows_core::PCWSTR, pmonitor: Option<&windows_core::IUnknown>, pdata: *const u8, pusercontext: Option<&windows_core::IUnknown>, pcallback: Option<&windows_core::IUnknown>, qwcontext: u64) -> windows_core::Result<()>;
    fn GetNetSourceProperties(&self, pszstreamname: &windows_core::PCWSTR) -> windows_core::Result<windows_core::IUnknown>;
    fn GetNetSourceSharedNamespace(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn GetNetSourceAdminInterface(&self, pszstreamname: &windows_core::PCWSTR) -> windows_core::Result<windows_core::VARIANT>;
    fn GetNumProtocolsSupported(&self) -> windows_core::Result<u32>;
    fn GetProtocolName(&self, dwprotocolnum: u32, pwszprotocolname: windows_core::PWSTR, pcchprotocolname: *mut u16) -> windows_core::Result<()>;
    fn Shutdown(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for INSNetSourceCreator {}
impl INSNetSourceCreator_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> INSNetSourceCreator_Vtbl
    where
        Identity: INSNetSourceCreator_Impl,
    {
        unsafe extern "system" fn Initialize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: INSNetSourceCreator_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            INSNetSourceCreator_Impl::Initialize(this).into()
        }
        unsafe extern "system" fn CreateNetSource<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszstreamname: windows_core::PCWSTR, pmonitor: *mut core::ffi::c_void, pdata: *const u8, pusercontext: *mut core::ffi::c_void, pcallback: *mut core::ffi::c_void, qwcontext: u64) -> windows_core::HRESULT
        where
            Identity: INSNetSourceCreator_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            INSNetSourceCreator_Impl::CreateNetSource(this, core::mem::transmute(&pszstreamname), windows_core::from_raw_borrowed(&pmonitor), core::mem::transmute_copy(&pdata), windows_core::from_raw_borrowed(&pusercontext), windows_core::from_raw_borrowed(&pcallback), core::mem::transmute_copy(&qwcontext)).into()
        }
        unsafe extern "system" fn GetNetSourceProperties<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszstreamname: windows_core::PCWSTR, pppropertiesnode: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: INSNetSourceCreator_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match INSNetSourceCreator_Impl::GetNetSourceProperties(this, core::mem::transmute(&pszstreamname)) {
                Ok(ok__) => {
                    pppropertiesnode.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNetSourceSharedNamespace<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppsharednamespace: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: INSNetSourceCreator_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match INSNetSourceCreator_Impl::GetNetSourceSharedNamespace(this) {
                Ok(ok__) => {
                    ppsharednamespace.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNetSourceAdminInterface<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszstreamname: windows_core::PCWSTR, pval: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: INSNetSourceCreator_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match INSNetSourceCreator_Impl::GetNetSourceAdminInterface(this, core::mem::transmute(&pszstreamname)) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNumProtocolsSupported<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcprotocols: *mut u32) -> windows_core::HRESULT
        where
            Identity: INSNetSourceCreator_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match INSNetSourceCreator_Impl::GetNumProtocolsSupported(this) {
                Ok(ok__) => {
                    pcprotocols.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProtocolName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwprotocolnum: u32, pwszprotocolname: windows_core::PWSTR, pcchprotocolname: *mut u16) -> windows_core::HRESULT
        where
            Identity: INSNetSourceCreator_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            INSNetSourceCreator_Impl::GetProtocolName(this, core::mem::transmute_copy(&dwprotocolnum), core::mem::transmute_copy(&pwszprotocolname), core::mem::transmute_copy(&pcchprotocolname)).into()
        }
        unsafe extern "system" fn Shutdown<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: INSNetSourceCreator_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            INSNetSourceCreator_Impl::Shutdown(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Initialize: Initialize::<Identity, OFFSET>,
            CreateNetSource: CreateNetSource::<Identity, OFFSET>,
            GetNetSourceProperties: GetNetSourceProperties::<Identity, OFFSET>,
            GetNetSourceSharedNamespace: GetNetSourceSharedNamespace::<Identity, OFFSET>,
            GetNetSourceAdminInterface: GetNetSourceAdminInterface::<Identity, OFFSET>,
            GetNumProtocolsSupported: GetNumProtocolsSupported::<Identity, OFFSET>,
            GetProtocolName: GetProtocolName::<Identity, OFFSET>,
            Shutdown: Shutdown::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<INSNetSourceCreator as windows_core::Interface>::IID
    }
}
pub trait INSSBuffer_Impl: Sized {
    fn GetLength(&self) -> windows_core::Result<u32>;
    fn SetLength(&self, dwlength: u32) -> windows_core::Result<()>;
    fn GetMaxLength(&self) -> windows_core::Result<u32>;
    fn GetBuffer(&self) -> windows_core::Result<*mut u8>;
    fn GetBufferAndLength(&self, ppdwbuffer: *mut *mut u8, pdwlength: *mut u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for INSSBuffer {}
impl INSSBuffer_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> INSSBuffer_Vtbl
    where
        Identity: INSSBuffer_Impl,
    {
        unsafe extern "system" fn GetLength<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwlength: *mut u32) -> windows_core::HRESULT
        where
            Identity: INSSBuffer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match INSSBuffer_Impl::GetLength(this) {
                Ok(ok__) => {
                    pdwlength.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLength<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwlength: u32) -> windows_core::HRESULT
        where
            Identity: INSSBuffer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            INSSBuffer_Impl::SetLength(this, core::mem::transmute_copy(&dwlength)).into()
        }
        unsafe extern "system" fn GetMaxLength<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwlength: *mut u32) -> windows_core::HRESULT
        where
            Identity: INSSBuffer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match INSSBuffer_Impl::GetMaxLength(this) {
                Ok(ok__) => {
                    pdwlength.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBuffer<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppdwbuffer: *mut *mut u8) -> windows_core::HRESULT
        where
            Identity: INSSBuffer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match INSSBuffer_Impl::GetBuffer(this) {
                Ok(ok__) => {
                    ppdwbuffer.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBufferAndLength<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppdwbuffer: *mut *mut u8, pdwlength: *mut u32) -> windows_core::HRESULT
        where
            Identity: INSSBuffer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            INSSBuffer_Impl::GetBufferAndLength(this, core::mem::transmute_copy(&ppdwbuffer), core::mem::transmute_copy(&pdwlength)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetLength: GetLength::<Identity, OFFSET>,
            SetLength: SetLength::<Identity, OFFSET>,
            GetMaxLength: GetMaxLength::<Identity, OFFSET>,
            GetBuffer: GetBuffer::<Identity, OFFSET>,
            GetBufferAndLength: GetBufferAndLength::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<INSSBuffer as windows_core::Interface>::IID
    }
}
pub trait INSSBuffer2_Impl: Sized + INSSBuffer_Impl {
    fn GetSampleProperties(&self, cbproperties: u32) -> windows_core::Result<u8>;
    fn SetSampleProperties(&self, cbproperties: u32, pbproperties: *const u8) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for INSSBuffer2 {}
impl INSSBuffer2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> INSSBuffer2_Vtbl
    where
        Identity: INSSBuffer2_Impl,
    {
        unsafe extern "system" fn GetSampleProperties<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, cbproperties: u32, pbproperties: *mut u8) -> windows_core::HRESULT
        where
            Identity: INSSBuffer2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match INSSBuffer2_Impl::GetSampleProperties(this, core::mem::transmute_copy(&cbproperties)) {
                Ok(ok__) => {
                    pbproperties.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSampleProperties<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, cbproperties: u32, pbproperties: *const u8) -> windows_core::HRESULT
        where
            Identity: INSSBuffer2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            INSSBuffer2_Impl::SetSampleProperties(this, core::mem::transmute_copy(&cbproperties), core::mem::transmute_copy(&pbproperties)).into()
        }
        Self {
            base__: INSSBuffer_Vtbl::new::<Identity, OFFSET>(),
            GetSampleProperties: GetSampleProperties::<Identity, OFFSET>,
            SetSampleProperties: SetSampleProperties::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<INSSBuffer2 as windows_core::Interface>::IID || iid == &<INSSBuffer as windows_core::Interface>::IID
    }
}
pub trait INSSBuffer3_Impl: Sized + INSSBuffer2_Impl {
    fn SetProperty(&self, guidbufferproperty: &windows_core::GUID, pvbufferproperty: *const core::ffi::c_void, dwbufferpropertysize: u32) -> windows_core::Result<()>;
    fn GetProperty(&self, guidbufferproperty: &windows_core::GUID, pvbufferproperty: *mut core::ffi::c_void, pdwbufferpropertysize: *mut u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for INSSBuffer3 {}
impl INSSBuffer3_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> INSSBuffer3_Vtbl
    where
        Identity: INSSBuffer3_Impl,
    {
        unsafe extern "system" fn SetProperty<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, guidbufferproperty: windows_core::GUID, pvbufferproperty: *const core::ffi::c_void, dwbufferpropertysize: u32) -> windows_core::HRESULT
        where
            Identity: INSSBuffer3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            INSSBuffer3_Impl::SetProperty(this, core::mem::transmute(&guidbufferproperty), core::mem::transmute_copy(&pvbufferproperty), core::mem::transmute_copy(&dwbufferpropertysize)).into()
        }
        unsafe extern "system" fn GetProperty<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, guidbufferproperty: windows_core::GUID, pvbufferproperty: *mut core::ffi::c_void, pdwbufferpropertysize: *mut u32) -> windows_core::HRESULT
        where
            Identity: INSSBuffer3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            INSSBuffer3_Impl::GetProperty(this, core::mem::transmute(&guidbufferproperty), core::mem::transmute_copy(&pvbufferproperty), core::mem::transmute_copy(&pdwbufferpropertysize)).into()
        }
        Self { base__: INSSBuffer2_Vtbl::new::<Identity, OFFSET>(), SetProperty: SetProperty::<Identity, OFFSET>, GetProperty: GetProperty::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<INSSBuffer3 as windows_core::Interface>::IID || iid == &<INSSBuffer as windows_core::Interface>::IID || iid == &<INSSBuffer2 as windows_core::Interface>::IID
    }
}
pub trait INSSBuffer4_Impl: Sized + INSSBuffer3_Impl {
    fn GetPropertyCount(&self) -> windows_core::Result<u32>;
    fn GetPropertyByIndex(&self, dwbufferpropertyindex: u32, pguidbufferproperty: *mut windows_core::GUID, pvbufferproperty: *mut core::ffi::c_void, pdwbufferpropertysize: *mut u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for INSSBuffer4 {}
impl INSSBuffer4_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> INSSBuffer4_Vtbl
    where
        Identity: INSSBuffer4_Impl,
    {
        unsafe extern "system" fn GetPropertyCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcbufferproperties: *mut u32) -> windows_core::HRESULT
        where
            Identity: INSSBuffer4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match INSSBuffer4_Impl::GetPropertyCount(this) {
                Ok(ok__) => {
                    pcbufferproperties.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPropertyByIndex<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwbufferpropertyindex: u32, pguidbufferproperty: *mut windows_core::GUID, pvbufferproperty: *mut core::ffi::c_void, pdwbufferpropertysize: *mut u32) -> windows_core::HRESULT
        where
            Identity: INSSBuffer4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            INSSBuffer4_Impl::GetPropertyByIndex(this, core::mem::transmute_copy(&dwbufferpropertyindex), core::mem::transmute_copy(&pguidbufferproperty), core::mem::transmute_copy(&pvbufferproperty), core::mem::transmute_copy(&pdwbufferpropertysize)).into()
        }
        Self {
            base__: INSSBuffer3_Vtbl::new::<Identity, OFFSET>(),
            GetPropertyCount: GetPropertyCount::<Identity, OFFSET>,
            GetPropertyByIndex: GetPropertyByIndex::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<INSSBuffer4 as windows_core::Interface>::IID || iid == &<INSSBuffer as windows_core::Interface>::IID || iid == &<INSSBuffer2 as windows_core::Interface>::IID || iid == &<INSSBuffer3 as windows_core::Interface>::IID
    }
}
pub trait IWMAddressAccess_Impl: Sized {
    fn GetAccessEntryCount(&self, aetype: WM_AETYPE) -> windows_core::Result<u32>;
    fn GetAccessEntry(&self, aetype: WM_AETYPE, dwentrynum: u32) -> windows_core::Result<WM_ADDRESS_ACCESSENTRY>;
    fn AddAccessEntry(&self, aetype: WM_AETYPE, paddraccessentry: *const WM_ADDRESS_ACCESSENTRY) -> windows_core::Result<()>;
    fn RemoveAccessEntry(&self, aetype: WM_AETYPE, dwentrynum: u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWMAddressAccess {}
impl IWMAddressAccess_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWMAddressAccess_Vtbl
    where
        Identity: IWMAddressAccess_Impl,
    {
        unsafe extern "system" fn GetAccessEntryCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, aetype: WM_AETYPE, pcentries: *mut u32) -> windows_core::HRESULT
        where
            Identity: IWMAddressAccess_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMAddressAccess_Impl::GetAccessEntryCount(this, core::mem::transmute_copy(&aetype)) {
                Ok(ok__) => {
                    pcentries.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAccessEntry<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, aetype: WM_AETYPE, dwentrynum: u32, paddraccessentry: *mut WM_ADDRESS_ACCESSENTRY) -> windows_core::HRESULT
        where
            Identity: IWMAddressAccess_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMAddressAccess_Impl::GetAccessEntry(this, core::mem::transmute_copy(&aetype), core::mem::transmute_copy(&dwentrynum)) {
                Ok(ok__) => {
                    paddraccessentry.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddAccessEntry<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, aetype: WM_AETYPE, paddraccessentry: *const WM_ADDRESS_ACCESSENTRY) -> windows_core::HRESULT
        where
            Identity: IWMAddressAccess_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMAddressAccess_Impl::AddAccessEntry(this, core::mem::transmute_copy(&aetype), core::mem::transmute_copy(&paddraccessentry)).into()
        }
        unsafe extern "system" fn RemoveAccessEntry<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, aetype: WM_AETYPE, dwentrynum: u32) -> windows_core::HRESULT
        where
            Identity: IWMAddressAccess_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMAddressAccess_Impl::RemoveAccessEntry(this, core::mem::transmute_copy(&aetype), core::mem::transmute_copy(&dwentrynum)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetAccessEntryCount: GetAccessEntryCount::<Identity, OFFSET>,
            GetAccessEntry: GetAccessEntry::<Identity, OFFSET>,
            AddAccessEntry: AddAccessEntry::<Identity, OFFSET>,
            RemoveAccessEntry: RemoveAccessEntry::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMAddressAccess as windows_core::Interface>::IID
    }
}
pub trait IWMAddressAccess2_Impl: Sized + IWMAddressAccess_Impl {
    fn GetAccessEntryEx(&self, aetype: WM_AETYPE, dwentrynum: u32, pbstraddress: *mut windows_core::BSTR, pbstrmask: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn AddAccessEntryEx(&self, aetype: WM_AETYPE, bstraddress: &windows_core::BSTR, bstrmask: &windows_core::BSTR) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWMAddressAccess2 {}
impl IWMAddressAccess2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWMAddressAccess2_Vtbl
    where
        Identity: IWMAddressAccess2_Impl,
    {
        unsafe extern "system" fn GetAccessEntryEx<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, aetype: WM_AETYPE, dwentrynum: u32, pbstraddress: *mut core::mem::MaybeUninit<windows_core::BSTR>, pbstrmask: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IWMAddressAccess2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMAddressAccess2_Impl::GetAccessEntryEx(this, core::mem::transmute_copy(&aetype), core::mem::transmute_copy(&dwentrynum), core::mem::transmute_copy(&pbstraddress), core::mem::transmute_copy(&pbstrmask)).into()
        }
        unsafe extern "system" fn AddAccessEntryEx<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, aetype: WM_AETYPE, bstraddress: core::mem::MaybeUninit<windows_core::BSTR>, bstrmask: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IWMAddressAccess2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMAddressAccess2_Impl::AddAccessEntryEx(this, core::mem::transmute_copy(&aetype), core::mem::transmute(&bstraddress), core::mem::transmute(&bstrmask)).into()
        }
        Self {
            base__: IWMAddressAccess_Vtbl::new::<Identity, OFFSET>(),
            GetAccessEntryEx: GetAccessEntryEx::<Identity, OFFSET>,
            AddAccessEntryEx: AddAccessEntryEx::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMAddressAccess2 as windows_core::Interface>::IID || iid == &<IWMAddressAccess as windows_core::Interface>::IID
    }
}
pub trait IWMAuthorizer_Impl: Sized {
    fn GetCertCount(&self) -> windows_core::Result<u32>;
    fn GetCert(&self, dwindex: u32) -> windows_core::Result<*mut u8>;
    fn GetSharedData(&self, dwcertindex: u32, pbshareddata: *const u8, pbcert: *const u8) -> windows_core::Result<*mut u8>;
}
impl windows_core::RuntimeName for IWMAuthorizer {}
impl IWMAuthorizer_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWMAuthorizer_Vtbl
    where
        Identity: IWMAuthorizer_Impl,
    {
        unsafe extern "system" fn GetCertCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pccerts: *mut u32) -> windows_core::HRESULT
        where
            Identity: IWMAuthorizer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMAuthorizer_Impl::GetCertCount(this) {
                Ok(ok__) => {
                    pccerts.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCert<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwindex: u32, ppbcertdata: *mut *mut u8) -> windows_core::HRESULT
        where
            Identity: IWMAuthorizer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMAuthorizer_Impl::GetCert(this, core::mem::transmute_copy(&dwindex)) {
                Ok(ok__) => {
                    ppbcertdata.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSharedData<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwcertindex: u32, pbshareddata: *const u8, pbcert: *const u8, ppbshareddata: *mut *mut u8) -> windows_core::HRESULT
        where
            Identity: IWMAuthorizer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMAuthorizer_Impl::GetSharedData(this, core::mem::transmute_copy(&dwcertindex), core::mem::transmute_copy(&pbshareddata), core::mem::transmute_copy(&pbcert)) {
                Ok(ok__) => {
                    ppbshareddata.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCertCount: GetCertCount::<Identity, OFFSET>,
            GetCert: GetCert::<Identity, OFFSET>,
            GetSharedData: GetSharedData::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMAuthorizer as windows_core::Interface>::IID
    }
}
pub trait IWMBackupRestoreProps_Impl: Sized {
    fn GetPropCount(&self) -> windows_core::Result<u16>;
    fn GetPropByIndex(&self, windex: u16, pwszname: windows_core::PWSTR, pcchnamelen: *mut u16, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> windows_core::Result<()>;
    fn GetPropByName(&self, pszname: &windows_core::PCWSTR, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> windows_core::Result<()>;
    fn SetProp(&self, pszname: &windows_core::PCWSTR, r#type: WMT_ATTR_DATATYPE, pvalue: *const u8, cblength: u16) -> windows_core::Result<()>;
    fn RemoveProp(&self, pcwszname: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn RemoveAllProps(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWMBackupRestoreProps {}
impl IWMBackupRestoreProps_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWMBackupRestoreProps_Vtbl
    where
        Identity: IWMBackupRestoreProps_Impl,
    {
        unsafe extern "system" fn GetPropCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcprops: *mut u16) -> windows_core::HRESULT
        where
            Identity: IWMBackupRestoreProps_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMBackupRestoreProps_Impl::GetPropCount(this) {
                Ok(ok__) => {
                    pcprops.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPropByIndex<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, windex: u16, pwszname: windows_core::PWSTR, pcchnamelen: *mut u16, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> windows_core::HRESULT
        where
            Identity: IWMBackupRestoreProps_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMBackupRestoreProps_Impl::GetPropByIndex(this, core::mem::transmute_copy(&windex), core::mem::transmute_copy(&pwszname), core::mem::transmute_copy(&pcchnamelen), core::mem::transmute_copy(&ptype), core::mem::transmute_copy(&pvalue), core::mem::transmute_copy(&pcblength)).into()
        }
        unsafe extern "system" fn GetPropByName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszname: windows_core::PCWSTR, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> windows_core::HRESULT
        where
            Identity: IWMBackupRestoreProps_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMBackupRestoreProps_Impl::GetPropByName(this, core::mem::transmute(&pszname), core::mem::transmute_copy(&ptype), core::mem::transmute_copy(&pvalue), core::mem::transmute_copy(&pcblength)).into()
        }
        unsafe extern "system" fn SetProp<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszname: windows_core::PCWSTR, r#type: WMT_ATTR_DATATYPE, pvalue: *const u8, cblength: u16) -> windows_core::HRESULT
        where
            Identity: IWMBackupRestoreProps_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMBackupRestoreProps_Impl::SetProp(this, core::mem::transmute(&pszname), core::mem::transmute_copy(&r#type), core::mem::transmute_copy(&pvalue), core::mem::transmute_copy(&cblength)).into()
        }
        unsafe extern "system" fn RemoveProp<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcwszname: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IWMBackupRestoreProps_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMBackupRestoreProps_Impl::RemoveProp(this, core::mem::transmute(&pcwszname)).into()
        }
        unsafe extern "system" fn RemoveAllProps<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMBackupRestoreProps_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMBackupRestoreProps_Impl::RemoveAllProps(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetPropCount: GetPropCount::<Identity, OFFSET>,
            GetPropByIndex: GetPropByIndex::<Identity, OFFSET>,
            GetPropByName: GetPropByName::<Identity, OFFSET>,
            SetProp: SetProp::<Identity, OFFSET>,
            RemoveProp: RemoveProp::<Identity, OFFSET>,
            RemoveAllProps: RemoveAllProps::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMBackupRestoreProps as windows_core::Interface>::IID
    }
}
pub trait IWMBandwidthSharing_Impl: Sized + IWMStreamList_Impl {
    fn GetType(&self) -> windows_core::Result<windows_core::GUID>;
    fn SetType(&self, guidtype: *const windows_core::GUID) -> windows_core::Result<()>;
    fn GetBandwidth(&self, pdwbitrate: *mut u32, pmsbufferwindow: *mut u32) -> windows_core::Result<()>;
    fn SetBandwidth(&self, dwbitrate: u32, msbufferwindow: u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWMBandwidthSharing {}
impl IWMBandwidthSharing_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWMBandwidthSharing_Vtbl
    where
        Identity: IWMBandwidthSharing_Impl,
    {
        unsafe extern "system" fn GetType<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pguidtype: *mut windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IWMBandwidthSharing_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMBandwidthSharing_Impl::GetType(this) {
                Ok(ok__) => {
                    pguidtype.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetType<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, guidtype: *const windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IWMBandwidthSharing_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMBandwidthSharing_Impl::SetType(this, core::mem::transmute_copy(&guidtype)).into()
        }
        unsafe extern "system" fn GetBandwidth<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwbitrate: *mut u32, pmsbufferwindow: *mut u32) -> windows_core::HRESULT
        where
            Identity: IWMBandwidthSharing_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMBandwidthSharing_Impl::GetBandwidth(this, core::mem::transmute_copy(&pdwbitrate), core::mem::transmute_copy(&pmsbufferwindow)).into()
        }
        unsafe extern "system" fn SetBandwidth<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwbitrate: u32, msbufferwindow: u32) -> windows_core::HRESULT
        where
            Identity: IWMBandwidthSharing_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMBandwidthSharing_Impl::SetBandwidth(this, core::mem::transmute_copy(&dwbitrate), core::mem::transmute_copy(&msbufferwindow)).into()
        }
        Self {
            base__: IWMStreamList_Vtbl::new::<Identity, OFFSET>(),
            GetType: GetType::<Identity, OFFSET>,
            SetType: SetType::<Identity, OFFSET>,
            GetBandwidth: GetBandwidth::<Identity, OFFSET>,
            SetBandwidth: SetBandwidth::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMBandwidthSharing as windows_core::Interface>::IID || iid == &<IWMStreamList as windows_core::Interface>::IID
    }
}
pub trait IWMClientConnections_Impl: Sized {
    fn GetClientCount(&self) -> windows_core::Result<u32>;
    fn GetClientProperties(&self, dwclientnum: u32) -> windows_core::Result<WM_CLIENT_PROPERTIES>;
}
impl windows_core::RuntimeName for IWMClientConnections {}
impl IWMClientConnections_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWMClientConnections_Vtbl
    where
        Identity: IWMClientConnections_Impl,
    {
        unsafe extern "system" fn GetClientCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcclients: *mut u32) -> windows_core::HRESULT
        where
            Identity: IWMClientConnections_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMClientConnections_Impl::GetClientCount(this) {
                Ok(ok__) => {
                    pcclients.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetClientProperties<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwclientnum: u32, pclientproperties: *mut WM_CLIENT_PROPERTIES) -> windows_core::HRESULT
        where
            Identity: IWMClientConnections_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMClientConnections_Impl::GetClientProperties(this, core::mem::transmute_copy(&dwclientnum)) {
                Ok(ok__) => {
                    pclientproperties.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetClientCount: GetClientCount::<Identity, OFFSET>,
            GetClientProperties: GetClientProperties::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMClientConnections as windows_core::Interface>::IID
    }
}
pub trait IWMClientConnections2_Impl: Sized + IWMClientConnections_Impl {
    fn GetClientInfo(&self, dwclientnum: u32, pwsznetworkaddress: windows_core::PWSTR, pcchnetworkaddress: *mut u32, pwszport: windows_core::PWSTR, pcchport: *mut u32, pwszdnsname: windows_core::PWSTR, pcchdnsname: *mut u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWMClientConnections2 {}
impl IWMClientConnections2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWMClientConnections2_Vtbl
    where
        Identity: IWMClientConnections2_Impl,
    {
        unsafe extern "system" fn GetClientInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwclientnum: u32, pwsznetworkaddress: windows_core::PWSTR, pcchnetworkaddress: *mut u32, pwszport: windows_core::PWSTR, pcchport: *mut u32, pwszdnsname: windows_core::PWSTR, pcchdnsname: *mut u32) -> windows_core::HRESULT
        where
            Identity: IWMClientConnections2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMClientConnections2_Impl::GetClientInfo(this, core::mem::transmute_copy(&dwclientnum), core::mem::transmute_copy(&pwsznetworkaddress), core::mem::transmute_copy(&pcchnetworkaddress), core::mem::transmute_copy(&pwszport), core::mem::transmute_copy(&pcchport), core::mem::transmute_copy(&pwszdnsname), core::mem::transmute_copy(&pcchdnsname)).into()
        }
        Self { base__: IWMClientConnections_Vtbl::new::<Identity, OFFSET>(), GetClientInfo: GetClientInfo::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMClientConnections2 as windows_core::Interface>::IID || iid == &<IWMClientConnections as windows_core::Interface>::IID
    }
}
pub trait IWMCodecInfo_Impl: Sized {
    fn GetCodecInfoCount(&self, guidtype: *const windows_core::GUID) -> windows_core::Result<u32>;
    fn GetCodecFormatCount(&self, guidtype: *const windows_core::GUID, dwcodecindex: u32) -> windows_core::Result<u32>;
    fn GetCodecFormat(&self, guidtype: *const windows_core::GUID, dwcodecindex: u32, dwformatindex: u32) -> windows_core::Result<IWMStreamConfig>;
}
impl windows_core::RuntimeName for IWMCodecInfo {}
impl IWMCodecInfo_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWMCodecInfo_Vtbl
    where
        Identity: IWMCodecInfo_Impl,
    {
        unsafe extern "system" fn GetCodecInfoCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, guidtype: *const windows_core::GUID, pccodecs: *mut u32) -> windows_core::HRESULT
        where
            Identity: IWMCodecInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMCodecInfo_Impl::GetCodecInfoCount(this, core::mem::transmute_copy(&guidtype)) {
                Ok(ok__) => {
                    pccodecs.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCodecFormatCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, guidtype: *const windows_core::GUID, dwcodecindex: u32, pcformat: *mut u32) -> windows_core::HRESULT
        where
            Identity: IWMCodecInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMCodecInfo_Impl::GetCodecFormatCount(this, core::mem::transmute_copy(&guidtype), core::mem::transmute_copy(&dwcodecindex)) {
                Ok(ok__) => {
                    pcformat.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCodecFormat<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, guidtype: *const windows_core::GUID, dwcodecindex: u32, dwformatindex: u32, ppistreamconfig: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMCodecInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMCodecInfo_Impl::GetCodecFormat(this, core::mem::transmute_copy(&guidtype), core::mem::transmute_copy(&dwcodecindex), core::mem::transmute_copy(&dwformatindex)) {
                Ok(ok__) => {
                    ppistreamconfig.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCodecInfoCount: GetCodecInfoCount::<Identity, OFFSET>,
            GetCodecFormatCount: GetCodecFormatCount::<Identity, OFFSET>,
            GetCodecFormat: GetCodecFormat::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMCodecInfo as windows_core::Interface>::IID
    }
}
pub trait IWMCodecInfo2_Impl: Sized + IWMCodecInfo_Impl {
    fn GetCodecName(&self, guidtype: *const windows_core::GUID, dwcodecindex: u32, wszname: windows_core::PWSTR, pcchname: *mut u32) -> windows_core::Result<()>;
    fn GetCodecFormatDesc(&self, guidtype: *const windows_core::GUID, dwcodecindex: u32, dwformatindex: u32, ppistreamconfig: *mut Option<IWMStreamConfig>, wszdesc: windows_core::PWSTR, pcchdesc: *mut u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWMCodecInfo2 {}
impl IWMCodecInfo2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWMCodecInfo2_Vtbl
    where
        Identity: IWMCodecInfo2_Impl,
    {
        unsafe extern "system" fn GetCodecName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, guidtype: *const windows_core::GUID, dwcodecindex: u32, wszname: windows_core::PWSTR, pcchname: *mut u32) -> windows_core::HRESULT
        where
            Identity: IWMCodecInfo2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMCodecInfo2_Impl::GetCodecName(this, core::mem::transmute_copy(&guidtype), core::mem::transmute_copy(&dwcodecindex), core::mem::transmute_copy(&wszname), core::mem::transmute_copy(&pcchname)).into()
        }
        unsafe extern "system" fn GetCodecFormatDesc<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, guidtype: *const windows_core::GUID, dwcodecindex: u32, dwformatindex: u32, ppistreamconfig: *mut *mut core::ffi::c_void, wszdesc: windows_core::PWSTR, pcchdesc: *mut u32) -> windows_core::HRESULT
        where
            Identity: IWMCodecInfo2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMCodecInfo2_Impl::GetCodecFormatDesc(this, core::mem::transmute_copy(&guidtype), core::mem::transmute_copy(&dwcodecindex), core::mem::transmute_copy(&dwformatindex), core::mem::transmute_copy(&ppistreamconfig), core::mem::transmute_copy(&wszdesc), core::mem::transmute_copy(&pcchdesc)).into()
        }
        Self {
            base__: IWMCodecInfo_Vtbl::new::<Identity, OFFSET>(),
            GetCodecName: GetCodecName::<Identity, OFFSET>,
            GetCodecFormatDesc: GetCodecFormatDesc::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMCodecInfo2 as windows_core::Interface>::IID || iid == &<IWMCodecInfo as windows_core::Interface>::IID
    }
}
pub trait IWMCodecInfo3_Impl: Sized + IWMCodecInfo2_Impl {
    fn GetCodecFormatProp(&self, guidtype: *const windows_core::GUID, dwcodecindex: u32, dwformatindex: u32, pszname: &windows_core::PCWSTR, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pdwsize: *mut u32) -> windows_core::Result<()>;
    fn GetCodecProp(&self, guidtype: *const windows_core::GUID, dwcodecindex: u32, pszname: &windows_core::PCWSTR, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pdwsize: *mut u32) -> windows_core::Result<()>;
    fn SetCodecEnumerationSetting(&self, guidtype: *const windows_core::GUID, dwcodecindex: u32, pszname: &windows_core::PCWSTR, r#type: WMT_ATTR_DATATYPE, pvalue: *const u8, dwsize: u32) -> windows_core::Result<()>;
    fn GetCodecEnumerationSetting(&self, guidtype: *const windows_core::GUID, dwcodecindex: u32, pszname: &windows_core::PCWSTR, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pdwsize: *mut u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWMCodecInfo3 {}
impl IWMCodecInfo3_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWMCodecInfo3_Vtbl
    where
        Identity: IWMCodecInfo3_Impl,
    {
        unsafe extern "system" fn GetCodecFormatProp<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, guidtype: *const windows_core::GUID, dwcodecindex: u32, dwformatindex: u32, pszname: windows_core::PCWSTR, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pdwsize: *mut u32) -> windows_core::HRESULT
        where
            Identity: IWMCodecInfo3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMCodecInfo3_Impl::GetCodecFormatProp(this, core::mem::transmute_copy(&guidtype), core::mem::transmute_copy(&dwcodecindex), core::mem::transmute_copy(&dwformatindex), core::mem::transmute(&pszname), core::mem::transmute_copy(&ptype), core::mem::transmute_copy(&pvalue), core::mem::transmute_copy(&pdwsize)).into()
        }
        unsafe extern "system" fn GetCodecProp<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, guidtype: *const windows_core::GUID, dwcodecindex: u32, pszname: windows_core::PCWSTR, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pdwsize: *mut u32) -> windows_core::HRESULT
        where
            Identity: IWMCodecInfo3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMCodecInfo3_Impl::GetCodecProp(this, core::mem::transmute_copy(&guidtype), core::mem::transmute_copy(&dwcodecindex), core::mem::transmute(&pszname), core::mem::transmute_copy(&ptype), core::mem::transmute_copy(&pvalue), core::mem::transmute_copy(&pdwsize)).into()
        }
        unsafe extern "system" fn SetCodecEnumerationSetting<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, guidtype: *const windows_core::GUID, dwcodecindex: u32, pszname: windows_core::PCWSTR, r#type: WMT_ATTR_DATATYPE, pvalue: *const u8, dwsize: u32) -> windows_core::HRESULT
        where
            Identity: IWMCodecInfo3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMCodecInfo3_Impl::SetCodecEnumerationSetting(this, core::mem::transmute_copy(&guidtype), core::mem::transmute_copy(&dwcodecindex), core::mem::transmute(&pszname), core::mem::transmute_copy(&r#type), core::mem::transmute_copy(&pvalue), core::mem::transmute_copy(&dwsize)).into()
        }
        unsafe extern "system" fn GetCodecEnumerationSetting<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, guidtype: *const windows_core::GUID, dwcodecindex: u32, pszname: windows_core::PCWSTR, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pdwsize: *mut u32) -> windows_core::HRESULT
        where
            Identity: IWMCodecInfo3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMCodecInfo3_Impl::GetCodecEnumerationSetting(this, core::mem::transmute_copy(&guidtype), core::mem::transmute_copy(&dwcodecindex), core::mem::transmute(&pszname), core::mem::transmute_copy(&ptype), core::mem::transmute_copy(&pvalue), core::mem::transmute_copy(&pdwsize)).into()
        }
        Self {
            base__: IWMCodecInfo2_Vtbl::new::<Identity, OFFSET>(),
            GetCodecFormatProp: GetCodecFormatProp::<Identity, OFFSET>,
            GetCodecProp: GetCodecProp::<Identity, OFFSET>,
            SetCodecEnumerationSetting: SetCodecEnumerationSetting::<Identity, OFFSET>,
            GetCodecEnumerationSetting: GetCodecEnumerationSetting::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMCodecInfo3 as windows_core::Interface>::IID || iid == &<IWMCodecInfo as windows_core::Interface>::IID || iid == &<IWMCodecInfo2 as windows_core::Interface>::IID
    }
}
pub trait IWMCredentialCallback_Impl: Sized {
    fn AcquireCredentials(&self, pwszrealm: &windows_core::PCWSTR, pwszsite: &windows_core::PCWSTR, pwszuser: windows_core::PWSTR, cchuser: u32, pwszpassword: windows_core::PWSTR, cchpassword: u32, hrstatus: windows_core::HRESULT, pdwflags: *mut u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWMCredentialCallback {}
impl IWMCredentialCallback_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWMCredentialCallback_Vtbl
    where
        Identity: IWMCredentialCallback_Impl,
    {
        unsafe extern "system" fn AcquireCredentials<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszrealm: windows_core::PCWSTR, pwszsite: windows_core::PCWSTR, pwszuser: windows_core::PWSTR, cchuser: u32, pwszpassword: windows_core::PWSTR, cchpassword: u32, hrstatus: windows_core::HRESULT, pdwflags: *mut u32) -> windows_core::HRESULT
        where
            Identity: IWMCredentialCallback_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMCredentialCallback_Impl::AcquireCredentials(this, core::mem::transmute(&pwszrealm), core::mem::transmute(&pwszsite), core::mem::transmute_copy(&pwszuser), core::mem::transmute_copy(&cchuser), core::mem::transmute_copy(&pwszpassword), core::mem::transmute_copy(&cchpassword), core::mem::transmute_copy(&hrstatus), core::mem::transmute_copy(&pdwflags)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), AcquireCredentials: AcquireCredentials::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMCredentialCallback as windows_core::Interface>::IID
    }
}
pub trait IWMDRMEditor_Impl: Sized {
    fn GetDRMProperty(&self, pwstrname: &windows_core::PCWSTR, pdwtype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWMDRMEditor {}
impl IWMDRMEditor_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWMDRMEditor_Vtbl
    where
        Identity: IWMDRMEditor_Impl,
    {
        unsafe extern "system" fn GetDRMProperty<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwstrname: windows_core::PCWSTR, pdwtype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> windows_core::HRESULT
        where
            Identity: IWMDRMEditor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMDRMEditor_Impl::GetDRMProperty(this, core::mem::transmute(&pwstrname), core::mem::transmute_copy(&pdwtype), core::mem::transmute_copy(&pvalue), core::mem::transmute_copy(&pcblength)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetDRMProperty: GetDRMProperty::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMDRMEditor as windows_core::Interface>::IID
    }
}
pub trait IWMDRMMessageParser_Impl: Sized {
    fn ParseRegistrationReqMsg(&self, pbregistrationreqmsg: *const u8, cbregistrationreqmsg: u32, ppdevicecert: *mut Option<INSSBuffer>, pdeviceserialnumber: *mut DRM_VAL16) -> windows_core::Result<()>;
    fn ParseLicenseRequestMsg(&self, pblicenserequestmsg: *const u8, cblicenserequestmsg: u32, ppdevicecert: *mut Option<INSSBuffer>, pdeviceserialnumber: *mut DRM_VAL16, pbstraction: *mut windows_core::BSTR) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWMDRMMessageParser {}
impl IWMDRMMessageParser_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWMDRMMessageParser_Vtbl
    where
        Identity: IWMDRMMessageParser_Impl,
    {
        unsafe extern "system" fn ParseRegistrationReqMsg<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbregistrationreqmsg: *const u8, cbregistrationreqmsg: u32, ppdevicecert: *mut *mut core::ffi::c_void, pdeviceserialnumber: *mut DRM_VAL16) -> windows_core::HRESULT
        where
            Identity: IWMDRMMessageParser_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMDRMMessageParser_Impl::ParseRegistrationReqMsg(this, core::mem::transmute_copy(&pbregistrationreqmsg), core::mem::transmute_copy(&cbregistrationreqmsg), core::mem::transmute_copy(&ppdevicecert), core::mem::transmute_copy(&pdeviceserialnumber)).into()
        }
        unsafe extern "system" fn ParseLicenseRequestMsg<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pblicenserequestmsg: *const u8, cblicenserequestmsg: u32, ppdevicecert: *mut *mut core::ffi::c_void, pdeviceserialnumber: *mut DRM_VAL16, pbstraction: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IWMDRMMessageParser_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMDRMMessageParser_Impl::ParseLicenseRequestMsg(this, core::mem::transmute_copy(&pblicenserequestmsg), core::mem::transmute_copy(&cblicenserequestmsg), core::mem::transmute_copy(&ppdevicecert), core::mem::transmute_copy(&pdeviceserialnumber), core::mem::transmute_copy(&pbstraction)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            ParseRegistrationReqMsg: ParseRegistrationReqMsg::<Identity, OFFSET>,
            ParseLicenseRequestMsg: ParseLicenseRequestMsg::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMDRMMessageParser as windows_core::Interface>::IID
    }
}
pub trait IWMDRMReader_Impl: Sized {
    fn AcquireLicense(&self, dwflags: u32) -> windows_core::Result<()>;
    fn CancelLicenseAcquisition(&self) -> windows_core::Result<()>;
    fn Individualize(&self, dwflags: u32) -> windows_core::Result<()>;
    fn CancelIndividualization(&self) -> windows_core::Result<()>;
    fn MonitorLicenseAcquisition(&self) -> windows_core::Result<()>;
    fn CancelMonitorLicenseAcquisition(&self) -> windows_core::Result<()>;
    fn SetDRMProperty(&self, pwstrname: &windows_core::PCWSTR, dwtype: WMT_ATTR_DATATYPE, pvalue: *const u8, cblength: u16) -> windows_core::Result<()>;
    fn GetDRMProperty(&self, pwstrname: &windows_core::PCWSTR, pdwtype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWMDRMReader {}
impl IWMDRMReader_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWMDRMReader_Vtbl
    where
        Identity: IWMDRMReader_Impl,
    {
        unsafe extern "system" fn AcquireLicense<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: u32) -> windows_core::HRESULT
        where
            Identity: IWMDRMReader_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMDRMReader_Impl::AcquireLicense(this, core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn CancelLicenseAcquisition<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMDRMReader_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMDRMReader_Impl::CancelLicenseAcquisition(this).into()
        }
        unsafe extern "system" fn Individualize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: u32) -> windows_core::HRESULT
        where
            Identity: IWMDRMReader_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMDRMReader_Impl::Individualize(this, core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn CancelIndividualization<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMDRMReader_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMDRMReader_Impl::CancelIndividualization(this).into()
        }
        unsafe extern "system" fn MonitorLicenseAcquisition<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMDRMReader_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMDRMReader_Impl::MonitorLicenseAcquisition(this).into()
        }
        unsafe extern "system" fn CancelMonitorLicenseAcquisition<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMDRMReader_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMDRMReader_Impl::CancelMonitorLicenseAcquisition(this).into()
        }
        unsafe extern "system" fn SetDRMProperty<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwstrname: windows_core::PCWSTR, dwtype: WMT_ATTR_DATATYPE, pvalue: *const u8, cblength: u16) -> windows_core::HRESULT
        where
            Identity: IWMDRMReader_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMDRMReader_Impl::SetDRMProperty(this, core::mem::transmute(&pwstrname), core::mem::transmute_copy(&dwtype), core::mem::transmute_copy(&pvalue), core::mem::transmute_copy(&cblength)).into()
        }
        unsafe extern "system" fn GetDRMProperty<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwstrname: windows_core::PCWSTR, pdwtype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> windows_core::HRESULT
        where
            Identity: IWMDRMReader_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMDRMReader_Impl::GetDRMProperty(this, core::mem::transmute(&pwstrname), core::mem::transmute_copy(&pdwtype), core::mem::transmute_copy(&pvalue), core::mem::transmute_copy(&pcblength)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AcquireLicense: AcquireLicense::<Identity, OFFSET>,
            CancelLicenseAcquisition: CancelLicenseAcquisition::<Identity, OFFSET>,
            Individualize: Individualize::<Identity, OFFSET>,
            CancelIndividualization: CancelIndividualization::<Identity, OFFSET>,
            MonitorLicenseAcquisition: MonitorLicenseAcquisition::<Identity, OFFSET>,
            CancelMonitorLicenseAcquisition: CancelMonitorLicenseAcquisition::<Identity, OFFSET>,
            SetDRMProperty: SetDRMProperty::<Identity, OFFSET>,
            GetDRMProperty: GetDRMProperty::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMDRMReader as windows_core::Interface>::IID
    }
}
pub trait IWMDRMReader2_Impl: Sized + IWMDRMReader_Impl {
    fn SetEvaluateOutputLevelLicenses(&self, fevaluate: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn GetPlayOutputLevels(&self, pplayopl: *mut DRM_PLAY_OPL, pcblength: *mut u32, pdwminappcompliancelevel: *mut u32) -> windows_core::Result<()>;
    fn GetCopyOutputLevels(&self, pcopyopl: *mut DRM_COPY_OPL, pcblength: *mut u32, pdwminappcompliancelevel: *mut u32) -> windows_core::Result<()>;
    fn TryNextLicense(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWMDRMReader2 {}
impl IWMDRMReader2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWMDRMReader2_Vtbl
    where
        Identity: IWMDRMReader2_Impl,
    {
        unsafe extern "system" fn SetEvaluateOutputLevelLicenses<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fevaluate: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IWMDRMReader2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMDRMReader2_Impl::SetEvaluateOutputLevelLicenses(this, core::mem::transmute_copy(&fevaluate)).into()
        }
        unsafe extern "system" fn GetPlayOutputLevels<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pplayopl: *mut DRM_PLAY_OPL, pcblength: *mut u32, pdwminappcompliancelevel: *mut u32) -> windows_core::HRESULT
        where
            Identity: IWMDRMReader2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMDRMReader2_Impl::GetPlayOutputLevels(this, core::mem::transmute_copy(&pplayopl), core::mem::transmute_copy(&pcblength), core::mem::transmute_copy(&pdwminappcompliancelevel)).into()
        }
        unsafe extern "system" fn GetCopyOutputLevels<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcopyopl: *mut DRM_COPY_OPL, pcblength: *mut u32, pdwminappcompliancelevel: *mut u32) -> windows_core::HRESULT
        where
            Identity: IWMDRMReader2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMDRMReader2_Impl::GetCopyOutputLevels(this, core::mem::transmute_copy(&pcopyopl), core::mem::transmute_copy(&pcblength), core::mem::transmute_copy(&pdwminappcompliancelevel)).into()
        }
        unsafe extern "system" fn TryNextLicense<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMDRMReader2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMDRMReader2_Impl::TryNextLicense(this).into()
        }
        Self {
            base__: IWMDRMReader_Vtbl::new::<Identity, OFFSET>(),
            SetEvaluateOutputLevelLicenses: SetEvaluateOutputLevelLicenses::<Identity, OFFSET>,
            GetPlayOutputLevels: GetPlayOutputLevels::<Identity, OFFSET>,
            GetCopyOutputLevels: GetCopyOutputLevels::<Identity, OFFSET>,
            TryNextLicense: TryNextLicense::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMDRMReader2 as windows_core::Interface>::IID || iid == &<IWMDRMReader as windows_core::Interface>::IID
    }
}
pub trait IWMDRMReader3_Impl: Sized + IWMDRMReader2_Impl {
    fn GetInclusionList(&self, ppguids: *mut *mut windows_core::GUID, pcguids: *mut u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWMDRMReader3 {}
impl IWMDRMReader3_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWMDRMReader3_Vtbl
    where
        Identity: IWMDRMReader3_Impl,
    {
        unsafe extern "system" fn GetInclusionList<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppguids: *mut *mut windows_core::GUID, pcguids: *mut u32) -> windows_core::HRESULT
        where
            Identity: IWMDRMReader3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMDRMReader3_Impl::GetInclusionList(this, core::mem::transmute_copy(&ppguids), core::mem::transmute_copy(&pcguids)).into()
        }
        Self { base__: IWMDRMReader2_Vtbl::new::<Identity, OFFSET>(), GetInclusionList: GetInclusionList::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMDRMReader3 as windows_core::Interface>::IID || iid == &<IWMDRMReader as windows_core::Interface>::IID || iid == &<IWMDRMReader2 as windows_core::Interface>::IID
    }
}
pub trait IWMDRMTranscryptionManager_Impl: Sized {
    fn CreateTranscryptor(&self) -> windows_core::Result<IWMDRMTranscryptor>;
}
impl windows_core::RuntimeName for IWMDRMTranscryptionManager {}
impl IWMDRMTranscryptionManager_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWMDRMTranscryptionManager_Vtbl
    where
        Identity: IWMDRMTranscryptionManager_Impl,
    {
        unsafe extern "system" fn CreateTranscryptor<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pptranscryptor: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMDRMTranscryptionManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMDRMTranscryptionManager_Impl::CreateTranscryptor(this) {
                Ok(ok__) => {
                    pptranscryptor.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), CreateTranscryptor: CreateTranscryptor::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMDRMTranscryptionManager as windows_core::Interface>::IID
    }
}
pub trait IWMDRMTranscryptor_Impl: Sized {
    fn Initialize(&self, bstrfilename: &windows_core::BSTR, pblicenserequestmsg: *mut u8, cblicenserequestmsg: u32, pplicenseresponsemsg: *mut Option<INSSBuffer>, pcallback: Option<&IWMStatusCallback>, pvcontext: *const core::ffi::c_void) -> windows_core::Result<()>;
    fn Seek(&self, hnstime: u64) -> windows_core::Result<()>;
    fn Read(&self, pbdata: *const u8, pcbdata: *const u32) -> windows_core::Result<()>;
    fn Close(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWMDRMTranscryptor {}
impl IWMDRMTranscryptor_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWMDRMTranscryptor_Vtbl
    where
        Identity: IWMDRMTranscryptor_Impl,
    {
        unsafe extern "system" fn Initialize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrfilename: core::mem::MaybeUninit<windows_core::BSTR>, pblicenserequestmsg: *mut u8, cblicenserequestmsg: u32, pplicenseresponsemsg: *mut *mut core::ffi::c_void, pcallback: *mut core::ffi::c_void, pvcontext: *const core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMDRMTranscryptor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMDRMTranscryptor_Impl::Initialize(this, core::mem::transmute(&bstrfilename), core::mem::transmute_copy(&pblicenserequestmsg), core::mem::transmute_copy(&cblicenserequestmsg), core::mem::transmute_copy(&pplicenseresponsemsg), windows_core::from_raw_borrowed(&pcallback), core::mem::transmute_copy(&pvcontext)).into()
        }
        unsafe extern "system" fn Seek<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hnstime: u64) -> windows_core::HRESULT
        where
            Identity: IWMDRMTranscryptor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMDRMTranscryptor_Impl::Seek(this, core::mem::transmute_copy(&hnstime)).into()
        }
        unsafe extern "system" fn Read<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbdata: *const u8, pcbdata: *const u32) -> windows_core::HRESULT
        where
            Identity: IWMDRMTranscryptor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMDRMTranscryptor_Impl::Read(this, core::mem::transmute_copy(&pbdata), core::mem::transmute_copy(&pcbdata)).into()
        }
        unsafe extern "system" fn Close<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMDRMTranscryptor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMDRMTranscryptor_Impl::Close(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Initialize: Initialize::<Identity, OFFSET>,
            Seek: Seek::<Identity, OFFSET>,
            Read: Read::<Identity, OFFSET>,
            Close: Close::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMDRMTranscryptor as windows_core::Interface>::IID
    }
}
pub trait IWMDRMTranscryptor2_Impl: Sized + IWMDRMTranscryptor_Impl {
    fn SeekEx(&self, cnsstarttime: u64, cnsduration: u64, flrate: f32, fincludefileheader: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn ZeroAdjustTimestamps(&self, fenable: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn GetSeekStartTime(&self) -> windows_core::Result<u64>;
    fn GetDuration(&self) -> windows_core::Result<u64>;
}
impl windows_core::RuntimeName for IWMDRMTranscryptor2 {}
impl IWMDRMTranscryptor2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWMDRMTranscryptor2_Vtbl
    where
        Identity: IWMDRMTranscryptor2_Impl,
    {
        unsafe extern "system" fn SeekEx<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, cnsstarttime: u64, cnsduration: u64, flrate: f32, fincludefileheader: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IWMDRMTranscryptor2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMDRMTranscryptor2_Impl::SeekEx(this, core::mem::transmute_copy(&cnsstarttime), core::mem::transmute_copy(&cnsduration), core::mem::transmute_copy(&flrate), core::mem::transmute_copy(&fincludefileheader)).into()
        }
        unsafe extern "system" fn ZeroAdjustTimestamps<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fenable: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IWMDRMTranscryptor2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMDRMTranscryptor2_Impl::ZeroAdjustTimestamps(this, core::mem::transmute_copy(&fenable)).into()
        }
        unsafe extern "system" fn GetSeekStartTime<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcnstime: *mut u64) -> windows_core::HRESULT
        where
            Identity: IWMDRMTranscryptor2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMDRMTranscryptor2_Impl::GetSeekStartTime(this) {
                Ok(ok__) => {
                    pcnstime.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDuration<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcnsduration: *mut u64) -> windows_core::HRESULT
        where
            Identity: IWMDRMTranscryptor2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMDRMTranscryptor2_Impl::GetDuration(this) {
                Ok(ok__) => {
                    pcnsduration.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: IWMDRMTranscryptor_Vtbl::new::<Identity, OFFSET>(),
            SeekEx: SeekEx::<Identity, OFFSET>,
            ZeroAdjustTimestamps: ZeroAdjustTimestamps::<Identity, OFFSET>,
            GetSeekStartTime: GetSeekStartTime::<Identity, OFFSET>,
            GetDuration: GetDuration::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMDRMTranscryptor2 as windows_core::Interface>::IID || iid == &<IWMDRMTranscryptor as windows_core::Interface>::IID
    }
}
pub trait IWMDRMWriter_Impl: Sized {
    fn GenerateKeySeed(&self, pwszkeyseed: windows_core::PWSTR, pcwchlength: *mut u32) -> windows_core::Result<()>;
    fn GenerateKeyID(&self, pwszkeyid: windows_core::PWSTR, pcwchlength: *mut u32) -> windows_core::Result<()>;
    fn GenerateSigningKeyPair(&self, pwszprivkey: windows_core::PWSTR, pcwchprivkeylength: *mut u32, pwszpubkey: windows_core::PWSTR, pcwchpubkeylength: *mut u32) -> windows_core::Result<()>;
    fn SetDRMAttribute(&self, wstreamnum: u16, pszname: &windows_core::PCWSTR, r#type: WMT_ATTR_DATATYPE, pvalue: *const u8, cblength: u16) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWMDRMWriter {}
impl IWMDRMWriter_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWMDRMWriter_Vtbl
    where
        Identity: IWMDRMWriter_Impl,
    {
        unsafe extern "system" fn GenerateKeySeed<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszkeyseed: windows_core::PWSTR, pcwchlength: *mut u32) -> windows_core::HRESULT
        where
            Identity: IWMDRMWriter_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMDRMWriter_Impl::GenerateKeySeed(this, core::mem::transmute_copy(&pwszkeyseed), core::mem::transmute_copy(&pcwchlength)).into()
        }
        unsafe extern "system" fn GenerateKeyID<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszkeyid: windows_core::PWSTR, pcwchlength: *mut u32) -> windows_core::HRESULT
        where
            Identity: IWMDRMWriter_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMDRMWriter_Impl::GenerateKeyID(this, core::mem::transmute_copy(&pwszkeyid), core::mem::transmute_copy(&pcwchlength)).into()
        }
        unsafe extern "system" fn GenerateSigningKeyPair<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszprivkey: windows_core::PWSTR, pcwchprivkeylength: *mut u32, pwszpubkey: windows_core::PWSTR, pcwchpubkeylength: *mut u32) -> windows_core::HRESULT
        where
            Identity: IWMDRMWriter_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMDRMWriter_Impl::GenerateSigningKeyPair(this, core::mem::transmute_copy(&pwszprivkey), core::mem::transmute_copy(&pcwchprivkeylength), core::mem::transmute_copy(&pwszpubkey), core::mem::transmute_copy(&pcwchpubkeylength)).into()
        }
        unsafe extern "system" fn SetDRMAttribute<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, wstreamnum: u16, pszname: windows_core::PCWSTR, r#type: WMT_ATTR_DATATYPE, pvalue: *const u8, cblength: u16) -> windows_core::HRESULT
        where
            Identity: IWMDRMWriter_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMDRMWriter_Impl::SetDRMAttribute(this, core::mem::transmute_copy(&wstreamnum), core::mem::transmute(&pszname), core::mem::transmute_copy(&r#type), core::mem::transmute_copy(&pvalue), core::mem::transmute_copy(&cblength)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GenerateKeySeed: GenerateKeySeed::<Identity, OFFSET>,
            GenerateKeyID: GenerateKeyID::<Identity, OFFSET>,
            GenerateSigningKeyPair: GenerateSigningKeyPair::<Identity, OFFSET>,
            SetDRMAttribute: SetDRMAttribute::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMDRMWriter as windows_core::Interface>::IID
    }
}
pub trait IWMDRMWriter2_Impl: Sized + IWMDRMWriter_Impl {
    fn SetWMDRMNetEncryption(&self, fsamplesencrypted: super::super::Foundation::BOOL, pbkeyid: *const u8, cbkeyid: u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWMDRMWriter2 {}
impl IWMDRMWriter2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWMDRMWriter2_Vtbl
    where
        Identity: IWMDRMWriter2_Impl,
    {
        unsafe extern "system" fn SetWMDRMNetEncryption<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fsamplesencrypted: super::super::Foundation::BOOL, pbkeyid: *const u8, cbkeyid: u32) -> windows_core::HRESULT
        where
            Identity: IWMDRMWriter2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMDRMWriter2_Impl::SetWMDRMNetEncryption(this, core::mem::transmute_copy(&fsamplesencrypted), core::mem::transmute_copy(&pbkeyid), core::mem::transmute_copy(&cbkeyid)).into()
        }
        Self { base__: IWMDRMWriter_Vtbl::new::<Identity, OFFSET>(), SetWMDRMNetEncryption: SetWMDRMNetEncryption::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMDRMWriter2 as windows_core::Interface>::IID || iid == &<IWMDRMWriter as windows_core::Interface>::IID
    }
}
pub trait IWMDRMWriter3_Impl: Sized + IWMDRMWriter2_Impl {
    fn SetProtectStreamSamples(&self, pimportinitstruct: *const WMDRM_IMPORT_INIT_STRUCT) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWMDRMWriter3 {}
impl IWMDRMWriter3_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWMDRMWriter3_Vtbl
    where
        Identity: IWMDRMWriter3_Impl,
    {
        unsafe extern "system" fn SetProtectStreamSamples<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pimportinitstruct: *const WMDRM_IMPORT_INIT_STRUCT) -> windows_core::HRESULT
        where
            Identity: IWMDRMWriter3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMDRMWriter3_Impl::SetProtectStreamSamples(this, core::mem::transmute_copy(&pimportinitstruct)).into()
        }
        Self { base__: IWMDRMWriter2_Vtbl::new::<Identity, OFFSET>(), SetProtectStreamSamples: SetProtectStreamSamples::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMDRMWriter3 as windows_core::Interface>::IID || iid == &<IWMDRMWriter as windows_core::Interface>::IID || iid == &<IWMDRMWriter2 as windows_core::Interface>::IID
    }
}
pub trait IWMDeviceRegistration_Impl: Sized {
    fn RegisterDevice(&self, dwregistertype: u32, pbcertificate: *const u8, cbcertificate: u32, serialnumber: &DRM_VAL16) -> windows_core::Result<IWMRegisteredDevice>;
    fn UnregisterDevice(&self, dwregistertype: u32, pbcertificate: *const u8, cbcertificate: u32, serialnumber: &DRM_VAL16) -> windows_core::Result<()>;
    fn GetRegistrationStats(&self, dwregistertype: u32) -> windows_core::Result<u32>;
    fn GetFirstRegisteredDevice(&self, dwregistertype: u32) -> windows_core::Result<IWMRegisteredDevice>;
    fn GetNextRegisteredDevice(&self) -> windows_core::Result<IWMRegisteredDevice>;
    fn GetRegisteredDeviceByID(&self, dwregistertype: u32, pbcertificate: *const u8, cbcertificate: u32, serialnumber: &DRM_VAL16) -> windows_core::Result<IWMRegisteredDevice>;
}
impl windows_core::RuntimeName for IWMDeviceRegistration {}
impl IWMDeviceRegistration_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWMDeviceRegistration_Vtbl
    where
        Identity: IWMDeviceRegistration_Impl,
    {
        unsafe extern "system" fn RegisterDevice<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwregistertype: u32, pbcertificate: *const u8, cbcertificate: u32, serialnumber: DRM_VAL16, ppdevice: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMDeviceRegistration_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMDeviceRegistration_Impl::RegisterDevice(this, core::mem::transmute_copy(&dwregistertype), core::mem::transmute_copy(&pbcertificate), core::mem::transmute_copy(&cbcertificate), core::mem::transmute(&serialnumber)) {
                Ok(ok__) => {
                    ppdevice.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnregisterDevice<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwregistertype: u32, pbcertificate: *const u8, cbcertificate: u32, serialnumber: DRM_VAL16) -> windows_core::HRESULT
        where
            Identity: IWMDeviceRegistration_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMDeviceRegistration_Impl::UnregisterDevice(this, core::mem::transmute_copy(&dwregistertype), core::mem::transmute_copy(&pbcertificate), core::mem::transmute_copy(&cbcertificate), core::mem::transmute(&serialnumber)).into()
        }
        unsafe extern "system" fn GetRegistrationStats<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwregistertype: u32, pcregistereddevices: *mut u32) -> windows_core::HRESULT
        where
            Identity: IWMDeviceRegistration_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMDeviceRegistration_Impl::GetRegistrationStats(this, core::mem::transmute_copy(&dwregistertype)) {
                Ok(ok__) => {
                    pcregistereddevices.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFirstRegisteredDevice<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwregistertype: u32, ppdevice: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMDeviceRegistration_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMDeviceRegistration_Impl::GetFirstRegisteredDevice(this, core::mem::transmute_copy(&dwregistertype)) {
                Ok(ok__) => {
                    ppdevice.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNextRegisteredDevice<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppdevice: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMDeviceRegistration_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMDeviceRegistration_Impl::GetNextRegisteredDevice(this) {
                Ok(ok__) => {
                    ppdevice.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRegisteredDeviceByID<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwregistertype: u32, pbcertificate: *const u8, cbcertificate: u32, serialnumber: DRM_VAL16, ppdevice: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMDeviceRegistration_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMDeviceRegistration_Impl::GetRegisteredDeviceByID(this, core::mem::transmute_copy(&dwregistertype), core::mem::transmute_copy(&pbcertificate), core::mem::transmute_copy(&cbcertificate), core::mem::transmute(&serialnumber)) {
                Ok(ok__) => {
                    ppdevice.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            RegisterDevice: RegisterDevice::<Identity, OFFSET>,
            UnregisterDevice: UnregisterDevice::<Identity, OFFSET>,
            GetRegistrationStats: GetRegistrationStats::<Identity, OFFSET>,
            GetFirstRegisteredDevice: GetFirstRegisteredDevice::<Identity, OFFSET>,
            GetNextRegisteredDevice: GetNextRegisteredDevice::<Identity, OFFSET>,
            GetRegisteredDeviceByID: GetRegisteredDeviceByID::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMDeviceRegistration as windows_core::Interface>::IID
    }
}
pub trait IWMGetSecureChannel_Impl: Sized {
    fn GetPeerSecureChannelInterface(&self) -> windows_core::Result<IWMSecureChannel>;
}
impl windows_core::RuntimeName for IWMGetSecureChannel {}
impl IWMGetSecureChannel_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWMGetSecureChannel_Vtbl
    where
        Identity: IWMGetSecureChannel_Impl,
    {
        unsafe extern "system" fn GetPeerSecureChannelInterface<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pppeer: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMGetSecureChannel_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMGetSecureChannel_Impl::GetPeerSecureChannelInterface(this) {
                Ok(ok__) => {
                    pppeer.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetPeerSecureChannelInterface: GetPeerSecureChannelInterface::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMGetSecureChannel as windows_core::Interface>::IID
    }
}
pub trait IWMHeaderInfo_Impl: Sized {
    fn GetAttributeCount(&self, wstreamnum: u16) -> windows_core::Result<u16>;
    fn GetAttributeByIndex(&self, windex: u16, pwstreamnum: *mut u16, pwszname: windows_core::PWSTR, pcchnamelen: *mut u16, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> windows_core::Result<()>;
    fn GetAttributeByName(&self, pwstreamnum: *mut u16, pszname: &windows_core::PCWSTR, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> windows_core::Result<()>;
    fn SetAttribute(&self, wstreamnum: u16, pszname: &windows_core::PCWSTR, r#type: WMT_ATTR_DATATYPE, pvalue: *const u8, cblength: u16) -> windows_core::Result<()>;
    fn GetMarkerCount(&self) -> windows_core::Result<u16>;
    fn GetMarker(&self, windex: u16, pwszmarkername: windows_core::PWSTR, pcchmarkernamelen: *mut u16, pcnsmarkertime: *mut u64) -> windows_core::Result<()>;
    fn AddMarker(&self, pwszmarkername: &windows_core::PCWSTR, cnsmarkertime: u64) -> windows_core::Result<()>;
    fn RemoveMarker(&self, windex: u16) -> windows_core::Result<()>;
    fn GetScriptCount(&self) -> windows_core::Result<u16>;
    fn GetScript(&self, windex: u16, pwsztype: windows_core::PWSTR, pcchtypelen: *mut u16, pwszcommand: windows_core::PWSTR, pcchcommandlen: *mut u16, pcnsscripttime: *mut u64) -> windows_core::Result<()>;
    fn AddScript(&self, pwsztype: &windows_core::PCWSTR, pwszcommand: &windows_core::PCWSTR, cnsscripttime: u64) -> windows_core::Result<()>;
    fn RemoveScript(&self, windex: u16) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWMHeaderInfo {}
impl IWMHeaderInfo_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWMHeaderInfo_Vtbl
    where
        Identity: IWMHeaderInfo_Impl,
    {
        unsafe extern "system" fn GetAttributeCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, wstreamnum: u16, pcattributes: *mut u16) -> windows_core::HRESULT
        where
            Identity: IWMHeaderInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMHeaderInfo_Impl::GetAttributeCount(this, core::mem::transmute_copy(&wstreamnum)) {
                Ok(ok__) => {
                    pcattributes.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAttributeByIndex<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, windex: u16, pwstreamnum: *mut u16, pwszname: windows_core::PWSTR, pcchnamelen: *mut u16, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> windows_core::HRESULT
        where
            Identity: IWMHeaderInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMHeaderInfo_Impl::GetAttributeByIndex(this, core::mem::transmute_copy(&windex), core::mem::transmute_copy(&pwstreamnum), core::mem::transmute_copy(&pwszname), core::mem::transmute_copy(&pcchnamelen), core::mem::transmute_copy(&ptype), core::mem::transmute_copy(&pvalue), core::mem::transmute_copy(&pcblength)).into()
        }
        unsafe extern "system" fn GetAttributeByName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwstreamnum: *mut u16, pszname: windows_core::PCWSTR, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> windows_core::HRESULT
        where
            Identity: IWMHeaderInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMHeaderInfo_Impl::GetAttributeByName(this, core::mem::transmute_copy(&pwstreamnum), core::mem::transmute(&pszname), core::mem::transmute_copy(&ptype), core::mem::transmute_copy(&pvalue), core::mem::transmute_copy(&pcblength)).into()
        }
        unsafe extern "system" fn SetAttribute<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, wstreamnum: u16, pszname: windows_core::PCWSTR, r#type: WMT_ATTR_DATATYPE, pvalue: *const u8, cblength: u16) -> windows_core::HRESULT
        where
            Identity: IWMHeaderInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMHeaderInfo_Impl::SetAttribute(this, core::mem::transmute_copy(&wstreamnum), core::mem::transmute(&pszname), core::mem::transmute_copy(&r#type), core::mem::transmute_copy(&pvalue), core::mem::transmute_copy(&cblength)).into()
        }
        unsafe extern "system" fn GetMarkerCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcmarkers: *mut u16) -> windows_core::HRESULT
        where
            Identity: IWMHeaderInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMHeaderInfo_Impl::GetMarkerCount(this) {
                Ok(ok__) => {
                    pcmarkers.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMarker<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, windex: u16, pwszmarkername: windows_core::PWSTR, pcchmarkernamelen: *mut u16, pcnsmarkertime: *mut u64) -> windows_core::HRESULT
        where
            Identity: IWMHeaderInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMHeaderInfo_Impl::GetMarker(this, core::mem::transmute_copy(&windex), core::mem::transmute_copy(&pwszmarkername), core::mem::transmute_copy(&pcchmarkernamelen), core::mem::transmute_copy(&pcnsmarkertime)).into()
        }
        unsafe extern "system" fn AddMarker<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszmarkername: windows_core::PCWSTR, cnsmarkertime: u64) -> windows_core::HRESULT
        where
            Identity: IWMHeaderInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMHeaderInfo_Impl::AddMarker(this, core::mem::transmute(&pwszmarkername), core::mem::transmute_copy(&cnsmarkertime)).into()
        }
        unsafe extern "system" fn RemoveMarker<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, windex: u16) -> windows_core::HRESULT
        where
            Identity: IWMHeaderInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMHeaderInfo_Impl::RemoveMarker(this, core::mem::transmute_copy(&windex)).into()
        }
        unsafe extern "system" fn GetScriptCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcscripts: *mut u16) -> windows_core::HRESULT
        where
            Identity: IWMHeaderInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMHeaderInfo_Impl::GetScriptCount(this) {
                Ok(ok__) => {
                    pcscripts.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetScript<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, windex: u16, pwsztype: windows_core::PWSTR, pcchtypelen: *mut u16, pwszcommand: windows_core::PWSTR, pcchcommandlen: *mut u16, pcnsscripttime: *mut u64) -> windows_core::HRESULT
        where
            Identity: IWMHeaderInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMHeaderInfo_Impl::GetScript(this, core::mem::transmute_copy(&windex), core::mem::transmute_copy(&pwsztype), core::mem::transmute_copy(&pcchtypelen), core::mem::transmute_copy(&pwszcommand), core::mem::transmute_copy(&pcchcommandlen), core::mem::transmute_copy(&pcnsscripttime)).into()
        }
        unsafe extern "system" fn AddScript<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwsztype: windows_core::PCWSTR, pwszcommand: windows_core::PCWSTR, cnsscripttime: u64) -> windows_core::HRESULT
        where
            Identity: IWMHeaderInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMHeaderInfo_Impl::AddScript(this, core::mem::transmute(&pwsztype), core::mem::transmute(&pwszcommand), core::mem::transmute_copy(&cnsscripttime)).into()
        }
        unsafe extern "system" fn RemoveScript<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, windex: u16) -> windows_core::HRESULT
        where
            Identity: IWMHeaderInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMHeaderInfo_Impl::RemoveScript(this, core::mem::transmute_copy(&windex)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetAttributeCount: GetAttributeCount::<Identity, OFFSET>,
            GetAttributeByIndex: GetAttributeByIndex::<Identity, OFFSET>,
            GetAttributeByName: GetAttributeByName::<Identity, OFFSET>,
            SetAttribute: SetAttribute::<Identity, OFFSET>,
            GetMarkerCount: GetMarkerCount::<Identity, OFFSET>,
            GetMarker: GetMarker::<Identity, OFFSET>,
            AddMarker: AddMarker::<Identity, OFFSET>,
            RemoveMarker: RemoveMarker::<Identity, OFFSET>,
            GetScriptCount: GetScriptCount::<Identity, OFFSET>,
            GetScript: GetScript::<Identity, OFFSET>,
            AddScript: AddScript::<Identity, OFFSET>,
            RemoveScript: RemoveScript::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMHeaderInfo as windows_core::Interface>::IID
    }
}
pub trait IWMHeaderInfo2_Impl: Sized + IWMHeaderInfo_Impl {
    fn GetCodecInfoCount(&self) -> windows_core::Result<u32>;
    fn GetCodecInfo(&self, windex: u32, pcchname: *mut u16, pwszname: windows_core::PWSTR, pcchdescription: *mut u16, pwszdescription: windows_core::PWSTR, pcodectype: *mut WMT_CODEC_INFO_TYPE, pcbcodecinfo: *mut u16, pbcodecinfo: *mut u8) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWMHeaderInfo2 {}
impl IWMHeaderInfo2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWMHeaderInfo2_Vtbl
    where
        Identity: IWMHeaderInfo2_Impl,
    {
        unsafe extern "system" fn GetCodecInfoCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pccodecinfos: *mut u32) -> windows_core::HRESULT
        where
            Identity: IWMHeaderInfo2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMHeaderInfo2_Impl::GetCodecInfoCount(this) {
                Ok(ok__) => {
                    pccodecinfos.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCodecInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, windex: u32, pcchname: *mut u16, pwszname: windows_core::PWSTR, pcchdescription: *mut u16, pwszdescription: windows_core::PWSTR, pcodectype: *mut WMT_CODEC_INFO_TYPE, pcbcodecinfo: *mut u16, pbcodecinfo: *mut u8) -> windows_core::HRESULT
        where
            Identity: IWMHeaderInfo2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMHeaderInfo2_Impl::GetCodecInfo(this, core::mem::transmute_copy(&windex), core::mem::transmute_copy(&pcchname), core::mem::transmute_copy(&pwszname), core::mem::transmute_copy(&pcchdescription), core::mem::transmute_copy(&pwszdescription), core::mem::transmute_copy(&pcodectype), core::mem::transmute_copy(&pcbcodecinfo), core::mem::transmute_copy(&pbcodecinfo)).into()
        }
        Self {
            base__: IWMHeaderInfo_Vtbl::new::<Identity, OFFSET>(),
            GetCodecInfoCount: GetCodecInfoCount::<Identity, OFFSET>,
            GetCodecInfo: GetCodecInfo::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMHeaderInfo2 as windows_core::Interface>::IID || iid == &<IWMHeaderInfo as windows_core::Interface>::IID
    }
}
pub trait IWMHeaderInfo3_Impl: Sized + IWMHeaderInfo2_Impl {
    fn GetAttributeCountEx(&self, wstreamnum: u16) -> windows_core::Result<u16>;
    fn GetAttributeIndices(&self, wstreamnum: u16, pwszname: &windows_core::PCWSTR, pwlangindex: *const u16, pwindices: *mut u16, pwcount: *mut u16) -> windows_core::Result<()>;
    fn GetAttributeByIndexEx(&self, wstreamnum: u16, windex: u16, pwszname: windows_core::PWSTR, pwnamelen: *mut u16, ptype: *mut WMT_ATTR_DATATYPE, pwlangindex: *mut u16, pvalue: *mut u8, pdwdatalength: *mut u32) -> windows_core::Result<()>;
    fn ModifyAttribute(&self, wstreamnum: u16, windex: u16, r#type: WMT_ATTR_DATATYPE, wlangindex: u16, pvalue: *const u8, dwlength: u32) -> windows_core::Result<()>;
    fn AddAttribute(&self, wstreamnum: u16, pszname: &windows_core::PCWSTR, pwindex: *mut u16, r#type: WMT_ATTR_DATATYPE, wlangindex: u16, pvalue: *const u8, dwlength: u32) -> windows_core::Result<()>;
    fn DeleteAttribute(&self, wstreamnum: u16, windex: u16) -> windows_core::Result<()>;
    fn AddCodecInfo(&self, pwszname: &windows_core::PCWSTR, pwszdescription: &windows_core::PCWSTR, codectype: WMT_CODEC_INFO_TYPE, cbcodecinfo: u16, pbcodecinfo: *const u8) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWMHeaderInfo3 {}
impl IWMHeaderInfo3_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWMHeaderInfo3_Vtbl
    where
        Identity: IWMHeaderInfo3_Impl,
    {
        unsafe extern "system" fn GetAttributeCountEx<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, wstreamnum: u16, pcattributes: *mut u16) -> windows_core::HRESULT
        where
            Identity: IWMHeaderInfo3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMHeaderInfo3_Impl::GetAttributeCountEx(this, core::mem::transmute_copy(&wstreamnum)) {
                Ok(ok__) => {
                    pcattributes.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAttributeIndices<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, wstreamnum: u16, pwszname: windows_core::PCWSTR, pwlangindex: *const u16, pwindices: *mut u16, pwcount: *mut u16) -> windows_core::HRESULT
        where
            Identity: IWMHeaderInfo3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMHeaderInfo3_Impl::GetAttributeIndices(this, core::mem::transmute_copy(&wstreamnum), core::mem::transmute(&pwszname), core::mem::transmute_copy(&pwlangindex), core::mem::transmute_copy(&pwindices), core::mem::transmute_copy(&pwcount)).into()
        }
        unsafe extern "system" fn GetAttributeByIndexEx<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, wstreamnum: u16, windex: u16, pwszname: windows_core::PWSTR, pwnamelen: *mut u16, ptype: *mut WMT_ATTR_DATATYPE, pwlangindex: *mut u16, pvalue: *mut u8, pdwdatalength: *mut u32) -> windows_core::HRESULT
        where
            Identity: IWMHeaderInfo3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMHeaderInfo3_Impl::GetAttributeByIndexEx(this, core::mem::transmute_copy(&wstreamnum), core::mem::transmute_copy(&windex), core::mem::transmute_copy(&pwszname), core::mem::transmute_copy(&pwnamelen), core::mem::transmute_copy(&ptype), core::mem::transmute_copy(&pwlangindex), core::mem::transmute_copy(&pvalue), core::mem::transmute_copy(&pdwdatalength)).into()
        }
        unsafe extern "system" fn ModifyAttribute<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, wstreamnum: u16, windex: u16, r#type: WMT_ATTR_DATATYPE, wlangindex: u16, pvalue: *const u8, dwlength: u32) -> windows_core::HRESULT
        where
            Identity: IWMHeaderInfo3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMHeaderInfo3_Impl::ModifyAttribute(this, core::mem::transmute_copy(&wstreamnum), core::mem::transmute_copy(&windex), core::mem::transmute_copy(&r#type), core::mem::transmute_copy(&wlangindex), core::mem::transmute_copy(&pvalue), core::mem::transmute_copy(&dwlength)).into()
        }
        unsafe extern "system" fn AddAttribute<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, wstreamnum: u16, pszname: windows_core::PCWSTR, pwindex: *mut u16, r#type: WMT_ATTR_DATATYPE, wlangindex: u16, pvalue: *const u8, dwlength: u32) -> windows_core::HRESULT
        where
            Identity: IWMHeaderInfo3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMHeaderInfo3_Impl::AddAttribute(this, core::mem::transmute_copy(&wstreamnum), core::mem::transmute(&pszname), core::mem::transmute_copy(&pwindex), core::mem::transmute_copy(&r#type), core::mem::transmute_copy(&wlangindex), core::mem::transmute_copy(&pvalue), core::mem::transmute_copy(&dwlength)).into()
        }
        unsafe extern "system" fn DeleteAttribute<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, wstreamnum: u16, windex: u16) -> windows_core::HRESULT
        where
            Identity: IWMHeaderInfo3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMHeaderInfo3_Impl::DeleteAttribute(this, core::mem::transmute_copy(&wstreamnum), core::mem::transmute_copy(&windex)).into()
        }
        unsafe extern "system" fn AddCodecInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszname: windows_core::PCWSTR, pwszdescription: windows_core::PCWSTR, codectype: WMT_CODEC_INFO_TYPE, cbcodecinfo: u16, pbcodecinfo: *const u8) -> windows_core::HRESULT
        where
            Identity: IWMHeaderInfo3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMHeaderInfo3_Impl::AddCodecInfo(this, core::mem::transmute(&pwszname), core::mem::transmute(&pwszdescription), core::mem::transmute_copy(&codectype), core::mem::transmute_copy(&cbcodecinfo), core::mem::transmute_copy(&pbcodecinfo)).into()
        }
        Self {
            base__: IWMHeaderInfo2_Vtbl::new::<Identity, OFFSET>(),
            GetAttributeCountEx: GetAttributeCountEx::<Identity, OFFSET>,
            GetAttributeIndices: GetAttributeIndices::<Identity, OFFSET>,
            GetAttributeByIndexEx: GetAttributeByIndexEx::<Identity, OFFSET>,
            ModifyAttribute: ModifyAttribute::<Identity, OFFSET>,
            AddAttribute: AddAttribute::<Identity, OFFSET>,
            DeleteAttribute: DeleteAttribute::<Identity, OFFSET>,
            AddCodecInfo: AddCodecInfo::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMHeaderInfo3 as windows_core::Interface>::IID || iid == &<IWMHeaderInfo as windows_core::Interface>::IID || iid == &<IWMHeaderInfo2 as windows_core::Interface>::IID
    }
}
pub trait IWMIStreamProps_Impl: Sized {
    fn GetProperty(&self, pszname: &windows_core::PCWSTR, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pdwsize: *mut u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWMIStreamProps {}
impl IWMIStreamProps_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWMIStreamProps_Vtbl
    where
        Identity: IWMIStreamProps_Impl,
    {
        unsafe extern "system" fn GetProperty<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszname: windows_core::PCWSTR, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pdwsize: *mut u32) -> windows_core::HRESULT
        where
            Identity: IWMIStreamProps_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMIStreamProps_Impl::GetProperty(this, core::mem::transmute(&pszname), core::mem::transmute_copy(&ptype), core::mem::transmute_copy(&pvalue), core::mem::transmute_copy(&pdwsize)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetProperty: GetProperty::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMIStreamProps as windows_core::Interface>::IID
    }
}
pub trait IWMImageInfo_Impl: Sized {
    fn GetImageCount(&self) -> windows_core::Result<u32>;
    fn GetImage(&self, windex: u32, pcchmimetype: *mut u16, pwszmimetype: windows_core::PWSTR, pcchdescription: *mut u16, pwszdescription: windows_core::PWSTR, pimagetype: *mut u16, pcbimagedata: *mut u32, pbimagedata: *mut u8) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWMImageInfo {}
impl IWMImageInfo_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWMImageInfo_Vtbl
    where
        Identity: IWMImageInfo_Impl,
    {
        unsafe extern "system" fn GetImageCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcimages: *mut u32) -> windows_core::HRESULT
        where
            Identity: IWMImageInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMImageInfo_Impl::GetImageCount(this) {
                Ok(ok__) => {
                    pcimages.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetImage<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, windex: u32, pcchmimetype: *mut u16, pwszmimetype: windows_core::PWSTR, pcchdescription: *mut u16, pwszdescription: windows_core::PWSTR, pimagetype: *mut u16, pcbimagedata: *mut u32, pbimagedata: *mut u8) -> windows_core::HRESULT
        where
            Identity: IWMImageInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMImageInfo_Impl::GetImage(this, core::mem::transmute_copy(&windex), core::mem::transmute_copy(&pcchmimetype), core::mem::transmute_copy(&pwszmimetype), core::mem::transmute_copy(&pcchdescription), core::mem::transmute_copy(&pwszdescription), core::mem::transmute_copy(&pimagetype), core::mem::transmute_copy(&pcbimagedata), core::mem::transmute_copy(&pbimagedata)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetImageCount: GetImageCount::<Identity, OFFSET>,
            GetImage: GetImage::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMImageInfo as windows_core::Interface>::IID
    }
}
pub trait IWMIndexer_Impl: Sized {
    fn StartIndexing(&self, pwszurl: &windows_core::PCWSTR, pcallback: Option<&IWMStatusCallback>, pvcontext: *const core::ffi::c_void) -> windows_core::Result<()>;
    fn Cancel(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWMIndexer {}
impl IWMIndexer_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWMIndexer_Vtbl
    where
        Identity: IWMIndexer_Impl,
    {
        unsafe extern "system" fn StartIndexing<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszurl: windows_core::PCWSTR, pcallback: *mut core::ffi::c_void, pvcontext: *const core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMIndexer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMIndexer_Impl::StartIndexing(this, core::mem::transmute(&pwszurl), windows_core::from_raw_borrowed(&pcallback), core::mem::transmute_copy(&pvcontext)).into()
        }
        unsafe extern "system" fn Cancel<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMIndexer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMIndexer_Impl::Cancel(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            StartIndexing: StartIndexing::<Identity, OFFSET>,
            Cancel: Cancel::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMIndexer as windows_core::Interface>::IID
    }
}
pub trait IWMIndexer2_Impl: Sized + IWMIndexer_Impl {
    fn Configure(&self, wstreamnum: u16, nindexertype: WMT_INDEXER_TYPE, pvinterval: *const core::ffi::c_void, pvindextype: *const core::ffi::c_void) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWMIndexer2 {}
impl IWMIndexer2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWMIndexer2_Vtbl
    where
        Identity: IWMIndexer2_Impl,
    {
        unsafe extern "system" fn Configure<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, wstreamnum: u16, nindexertype: WMT_INDEXER_TYPE, pvinterval: *const core::ffi::c_void, pvindextype: *const core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMIndexer2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMIndexer2_Impl::Configure(this, core::mem::transmute_copy(&wstreamnum), core::mem::transmute_copy(&nindexertype), core::mem::transmute_copy(&pvinterval), core::mem::transmute_copy(&pvindextype)).into()
        }
        Self { base__: IWMIndexer_Vtbl::new::<Identity, OFFSET>(), Configure: Configure::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMIndexer2 as windows_core::Interface>::IID || iid == &<IWMIndexer as windows_core::Interface>::IID
    }
}
pub trait IWMInputMediaProps_Impl: Sized + IWMMediaProps_Impl {
    fn GetConnectionName(&self, pwszname: windows_core::PWSTR, pcchname: *mut u16) -> windows_core::Result<()>;
    fn GetGroupName(&self, pwszname: windows_core::PWSTR, pcchname: *mut u16) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWMInputMediaProps {}
impl IWMInputMediaProps_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWMInputMediaProps_Vtbl
    where
        Identity: IWMInputMediaProps_Impl,
    {
        unsafe extern "system" fn GetConnectionName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszname: windows_core::PWSTR, pcchname: *mut u16) -> windows_core::HRESULT
        where
            Identity: IWMInputMediaProps_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMInputMediaProps_Impl::GetConnectionName(this, core::mem::transmute_copy(&pwszname), core::mem::transmute_copy(&pcchname)).into()
        }
        unsafe extern "system" fn GetGroupName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszname: windows_core::PWSTR, pcchname: *mut u16) -> windows_core::HRESULT
        where
            Identity: IWMInputMediaProps_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMInputMediaProps_Impl::GetGroupName(this, core::mem::transmute_copy(&pwszname), core::mem::transmute_copy(&pcchname)).into()
        }
        Self {
            base__: IWMMediaProps_Vtbl::new::<Identity, OFFSET>(),
            GetConnectionName: GetConnectionName::<Identity, OFFSET>,
            GetGroupName: GetGroupName::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMInputMediaProps as windows_core::Interface>::IID || iid == &<IWMMediaProps as windows_core::Interface>::IID
    }
}
pub trait IWMLanguageList_Impl: Sized {
    fn GetLanguageCount(&self) -> windows_core::Result<u16>;
    fn GetLanguageDetails(&self, windex: u16, pwszlanguagestring: windows_core::PWSTR, pcchlanguagestringlength: *mut u16) -> windows_core::Result<()>;
    fn AddLanguageByRFC1766String(&self, pwszlanguagestring: &windows_core::PCWSTR) -> windows_core::Result<u16>;
}
impl windows_core::RuntimeName for IWMLanguageList {}
impl IWMLanguageList_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWMLanguageList_Vtbl
    where
        Identity: IWMLanguageList_Impl,
    {
        unsafe extern "system" fn GetLanguageCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwcount: *mut u16) -> windows_core::HRESULT
        where
            Identity: IWMLanguageList_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMLanguageList_Impl::GetLanguageCount(this) {
                Ok(ok__) => {
                    pwcount.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLanguageDetails<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, windex: u16, pwszlanguagestring: windows_core::PWSTR, pcchlanguagestringlength: *mut u16) -> windows_core::HRESULT
        where
            Identity: IWMLanguageList_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMLanguageList_Impl::GetLanguageDetails(this, core::mem::transmute_copy(&windex), core::mem::transmute_copy(&pwszlanguagestring), core::mem::transmute_copy(&pcchlanguagestringlength)).into()
        }
        unsafe extern "system" fn AddLanguageByRFC1766String<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszlanguagestring: windows_core::PCWSTR, pwindex: *mut u16) -> windows_core::HRESULT
        where
            Identity: IWMLanguageList_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMLanguageList_Impl::AddLanguageByRFC1766String(this, core::mem::transmute(&pwszlanguagestring)) {
                Ok(ok__) => {
                    pwindex.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetLanguageCount: GetLanguageCount::<Identity, OFFSET>,
            GetLanguageDetails: GetLanguageDetails::<Identity, OFFSET>,
            AddLanguageByRFC1766String: AddLanguageByRFC1766String::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMLanguageList as windows_core::Interface>::IID
    }
}
pub trait IWMLicenseBackup_Impl: Sized {
    fn BackupLicenses(&self, dwflags: u32, pcallback: Option<&IWMStatusCallback>) -> windows_core::Result<()>;
    fn CancelLicenseBackup(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWMLicenseBackup {}
impl IWMLicenseBackup_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWMLicenseBackup_Vtbl
    where
        Identity: IWMLicenseBackup_Impl,
    {
        unsafe extern "system" fn BackupLicenses<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: u32, pcallback: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMLicenseBackup_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMLicenseBackup_Impl::BackupLicenses(this, core::mem::transmute_copy(&dwflags), windows_core::from_raw_borrowed(&pcallback)).into()
        }
        unsafe extern "system" fn CancelLicenseBackup<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMLicenseBackup_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMLicenseBackup_Impl::CancelLicenseBackup(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            BackupLicenses: BackupLicenses::<Identity, OFFSET>,
            CancelLicenseBackup: CancelLicenseBackup::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMLicenseBackup as windows_core::Interface>::IID
    }
}
pub trait IWMLicenseRestore_Impl: Sized {
    fn RestoreLicenses(&self, dwflags: u32, pcallback: Option<&IWMStatusCallback>) -> windows_core::Result<()>;
    fn CancelLicenseRestore(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWMLicenseRestore {}
impl IWMLicenseRestore_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWMLicenseRestore_Vtbl
    where
        Identity: IWMLicenseRestore_Impl,
    {
        unsafe extern "system" fn RestoreLicenses<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: u32, pcallback: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMLicenseRestore_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMLicenseRestore_Impl::RestoreLicenses(this, core::mem::transmute_copy(&dwflags), windows_core::from_raw_borrowed(&pcallback)).into()
        }
        unsafe extern "system" fn CancelLicenseRestore<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMLicenseRestore_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMLicenseRestore_Impl::CancelLicenseRestore(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            RestoreLicenses: RestoreLicenses::<Identity, OFFSET>,
            CancelLicenseRestore: CancelLicenseRestore::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMLicenseRestore as windows_core::Interface>::IID
    }
}
pub trait IWMLicenseRevocationAgent_Impl: Sized {
    fn GetLRBChallenge(&self, pmachineid: *const u8, dwmachineidlength: u32, pchallenge: *const u8, dwchallengelength: u32, pchallengeoutput: *mut u8, pdwchallengeoutputlength: *mut u32) -> windows_core::Result<()>;
    fn ProcessLRB(&self, psignedlrb: *const u8, dwsignedlrblength: u32, psignedack: *mut u8, pdwsignedacklength: *mut u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWMLicenseRevocationAgent {}
impl IWMLicenseRevocationAgent_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWMLicenseRevocationAgent_Vtbl
    where
        Identity: IWMLicenseRevocationAgent_Impl,
    {
        unsafe extern "system" fn GetLRBChallenge<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmachineid: *const u8, dwmachineidlength: u32, pchallenge: *const u8, dwchallengelength: u32, pchallengeoutput: *mut u8, pdwchallengeoutputlength: *mut u32) -> windows_core::HRESULT
        where
            Identity: IWMLicenseRevocationAgent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMLicenseRevocationAgent_Impl::GetLRBChallenge(this, core::mem::transmute_copy(&pmachineid), core::mem::transmute_copy(&dwmachineidlength), core::mem::transmute_copy(&pchallenge), core::mem::transmute_copy(&dwchallengelength), core::mem::transmute_copy(&pchallengeoutput), core::mem::transmute_copy(&pdwchallengeoutputlength)).into()
        }
        unsafe extern "system" fn ProcessLRB<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, psignedlrb: *const u8, dwsignedlrblength: u32, psignedack: *mut u8, pdwsignedacklength: *mut u32) -> windows_core::HRESULT
        where
            Identity: IWMLicenseRevocationAgent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMLicenseRevocationAgent_Impl::ProcessLRB(this, core::mem::transmute_copy(&psignedlrb), core::mem::transmute_copy(&dwsignedlrblength), core::mem::transmute_copy(&psignedack), core::mem::transmute_copy(&pdwsignedacklength)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetLRBChallenge: GetLRBChallenge::<Identity, OFFSET>,
            ProcessLRB: ProcessLRB::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMLicenseRevocationAgent as windows_core::Interface>::IID
    }
}
pub trait IWMMediaProps_Impl: Sized {
    fn GetType(&self) -> windows_core::Result<windows_core::GUID>;
    fn GetMediaType(&self, ptype: *mut WM_MEDIA_TYPE, pcbtype: *mut u32) -> windows_core::Result<()>;
    fn SetMediaType(&self, ptype: *const WM_MEDIA_TYPE) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWMMediaProps {}
impl IWMMediaProps_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWMMediaProps_Vtbl
    where
        Identity: IWMMediaProps_Impl,
    {
        unsafe extern "system" fn GetType<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pguidtype: *mut windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IWMMediaProps_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMMediaProps_Impl::GetType(this) {
                Ok(ok__) => {
                    pguidtype.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMediaType<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptype: *mut WM_MEDIA_TYPE, pcbtype: *mut u32) -> windows_core::HRESULT
        where
            Identity: IWMMediaProps_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMMediaProps_Impl::GetMediaType(this, core::mem::transmute_copy(&ptype), core::mem::transmute_copy(&pcbtype)).into()
        }
        unsafe extern "system" fn SetMediaType<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptype: *const WM_MEDIA_TYPE) -> windows_core::HRESULT
        where
            Identity: IWMMediaProps_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMMediaProps_Impl::SetMediaType(this, core::mem::transmute_copy(&ptype)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetType: GetType::<Identity, OFFSET>,
            GetMediaType: GetMediaType::<Identity, OFFSET>,
            SetMediaType: SetMediaType::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMMediaProps as windows_core::Interface>::IID
    }
}
pub trait IWMMetadataEditor_Impl: Sized {
    fn Open(&self, pwszfilename: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn Close(&self) -> windows_core::Result<()>;
    fn Flush(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWMMetadataEditor {}
impl IWMMetadataEditor_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWMMetadataEditor_Vtbl
    where
        Identity: IWMMetadataEditor_Impl,
    {
        unsafe extern "system" fn Open<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszfilename: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IWMMetadataEditor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMMetadataEditor_Impl::Open(this, core::mem::transmute(&pwszfilename)).into()
        }
        unsafe extern "system" fn Close<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMMetadataEditor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMMetadataEditor_Impl::Close(this).into()
        }
        unsafe extern "system" fn Flush<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMMetadataEditor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMMetadataEditor_Impl::Flush(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Open: Open::<Identity, OFFSET>,
            Close: Close::<Identity, OFFSET>,
            Flush: Flush::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMMetadataEditor as windows_core::Interface>::IID
    }
}
pub trait IWMMetadataEditor2_Impl: Sized + IWMMetadataEditor_Impl {
    fn OpenEx(&self, pwszfilename: &windows_core::PCWSTR, dwdesiredaccess: u32, dwsharemode: u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWMMetadataEditor2 {}
impl IWMMetadataEditor2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWMMetadataEditor2_Vtbl
    where
        Identity: IWMMetadataEditor2_Impl,
    {
        unsafe extern "system" fn OpenEx<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszfilename: windows_core::PCWSTR, dwdesiredaccess: u32, dwsharemode: u32) -> windows_core::HRESULT
        where
            Identity: IWMMetadataEditor2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMMetadataEditor2_Impl::OpenEx(this, core::mem::transmute(&pwszfilename), core::mem::transmute_copy(&dwdesiredaccess), core::mem::transmute_copy(&dwsharemode)).into()
        }
        Self { base__: IWMMetadataEditor_Vtbl::new::<Identity, OFFSET>(), OpenEx: OpenEx::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMMetadataEditor2 as windows_core::Interface>::IID || iid == &<IWMMetadataEditor as windows_core::Interface>::IID
    }
}
pub trait IWMMutualExclusion_Impl: Sized + IWMStreamList_Impl {
    fn GetType(&self) -> windows_core::Result<windows_core::GUID>;
    fn SetType(&self, guidtype: *const windows_core::GUID) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWMMutualExclusion {}
impl IWMMutualExclusion_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWMMutualExclusion_Vtbl
    where
        Identity: IWMMutualExclusion_Impl,
    {
        unsafe extern "system" fn GetType<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pguidtype: *mut windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IWMMutualExclusion_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMMutualExclusion_Impl::GetType(this) {
                Ok(ok__) => {
                    pguidtype.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetType<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, guidtype: *const windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IWMMutualExclusion_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMMutualExclusion_Impl::SetType(this, core::mem::transmute_copy(&guidtype)).into()
        }
        Self { base__: IWMStreamList_Vtbl::new::<Identity, OFFSET>(), GetType: GetType::<Identity, OFFSET>, SetType: SetType::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMMutualExclusion as windows_core::Interface>::IID || iid == &<IWMStreamList as windows_core::Interface>::IID
    }
}
pub trait IWMMutualExclusion2_Impl: Sized + IWMMutualExclusion_Impl {
    fn GetName(&self, pwszname: windows_core::PWSTR, pcchname: *mut u16) -> windows_core::Result<()>;
    fn SetName(&self, pwszname: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetRecordCount(&self) -> windows_core::Result<u16>;
    fn AddRecord(&self) -> windows_core::Result<()>;
    fn RemoveRecord(&self, wrecordnumber: u16) -> windows_core::Result<()>;
    fn GetRecordName(&self, wrecordnumber: u16, pwszrecordname: windows_core::PWSTR, pcchrecordname: *mut u16) -> windows_core::Result<()>;
    fn SetRecordName(&self, wrecordnumber: u16, pwszrecordname: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetStreamsForRecord(&self, wrecordnumber: u16, pwstreamnumarray: *mut u16, pcstreams: *mut u16) -> windows_core::Result<()>;
    fn AddStreamForRecord(&self, wrecordnumber: u16, wstreamnumber: u16) -> windows_core::Result<()>;
    fn RemoveStreamForRecord(&self, wrecordnumber: u16, wstreamnumber: u16) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWMMutualExclusion2 {}
impl IWMMutualExclusion2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWMMutualExclusion2_Vtbl
    where
        Identity: IWMMutualExclusion2_Impl,
    {
        unsafe extern "system" fn GetName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszname: windows_core::PWSTR, pcchname: *mut u16) -> windows_core::HRESULT
        where
            Identity: IWMMutualExclusion2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMMutualExclusion2_Impl::GetName(this, core::mem::transmute_copy(&pwszname), core::mem::transmute_copy(&pcchname)).into()
        }
        unsafe extern "system" fn SetName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszname: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IWMMutualExclusion2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMMutualExclusion2_Impl::SetName(this, core::mem::transmute(&pwszname)).into()
        }
        unsafe extern "system" fn GetRecordCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwrecordcount: *mut u16) -> windows_core::HRESULT
        where
            Identity: IWMMutualExclusion2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMMutualExclusion2_Impl::GetRecordCount(this) {
                Ok(ok__) => {
                    pwrecordcount.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddRecord<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMMutualExclusion2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMMutualExclusion2_Impl::AddRecord(this).into()
        }
        unsafe extern "system" fn RemoveRecord<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, wrecordnumber: u16) -> windows_core::HRESULT
        where
            Identity: IWMMutualExclusion2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMMutualExclusion2_Impl::RemoveRecord(this, core::mem::transmute_copy(&wrecordnumber)).into()
        }
        unsafe extern "system" fn GetRecordName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, wrecordnumber: u16, pwszrecordname: windows_core::PWSTR, pcchrecordname: *mut u16) -> windows_core::HRESULT
        where
            Identity: IWMMutualExclusion2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMMutualExclusion2_Impl::GetRecordName(this, core::mem::transmute_copy(&wrecordnumber), core::mem::transmute_copy(&pwszrecordname), core::mem::transmute_copy(&pcchrecordname)).into()
        }
        unsafe extern "system" fn SetRecordName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, wrecordnumber: u16, pwszrecordname: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IWMMutualExclusion2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMMutualExclusion2_Impl::SetRecordName(this, core::mem::transmute_copy(&wrecordnumber), core::mem::transmute(&pwszrecordname)).into()
        }
        unsafe extern "system" fn GetStreamsForRecord<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, wrecordnumber: u16, pwstreamnumarray: *mut u16, pcstreams: *mut u16) -> windows_core::HRESULT
        where
            Identity: IWMMutualExclusion2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMMutualExclusion2_Impl::GetStreamsForRecord(this, core::mem::transmute_copy(&wrecordnumber), core::mem::transmute_copy(&pwstreamnumarray), core::mem::transmute_copy(&pcstreams)).into()
        }
        unsafe extern "system" fn AddStreamForRecord<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, wrecordnumber: u16, wstreamnumber: u16) -> windows_core::HRESULT
        where
            Identity: IWMMutualExclusion2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMMutualExclusion2_Impl::AddStreamForRecord(this, core::mem::transmute_copy(&wrecordnumber), core::mem::transmute_copy(&wstreamnumber)).into()
        }
        unsafe extern "system" fn RemoveStreamForRecord<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, wrecordnumber: u16, wstreamnumber: u16) -> windows_core::HRESULT
        where
            Identity: IWMMutualExclusion2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMMutualExclusion2_Impl::RemoveStreamForRecord(this, core::mem::transmute_copy(&wrecordnumber), core::mem::transmute_copy(&wstreamnumber)).into()
        }
        Self {
            base__: IWMMutualExclusion_Vtbl::new::<Identity, OFFSET>(),
            GetName: GetName::<Identity, OFFSET>,
            SetName: SetName::<Identity, OFFSET>,
            GetRecordCount: GetRecordCount::<Identity, OFFSET>,
            AddRecord: AddRecord::<Identity, OFFSET>,
            RemoveRecord: RemoveRecord::<Identity, OFFSET>,
            GetRecordName: GetRecordName::<Identity, OFFSET>,
            SetRecordName: SetRecordName::<Identity, OFFSET>,
            GetStreamsForRecord: GetStreamsForRecord::<Identity, OFFSET>,
            AddStreamForRecord: AddStreamForRecord::<Identity, OFFSET>,
            RemoveStreamForRecord: RemoveStreamForRecord::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMMutualExclusion2 as windows_core::Interface>::IID || iid == &<IWMStreamList as windows_core::Interface>::IID || iid == &<IWMMutualExclusion as windows_core::Interface>::IID
    }
}
pub trait IWMOutputMediaProps_Impl: Sized + IWMMediaProps_Impl {
    fn GetStreamGroupName(&self, pwszname: windows_core::PWSTR, pcchname: *mut u16) -> windows_core::Result<()>;
    fn GetConnectionName(&self, pwszname: windows_core::PWSTR, pcchname: *mut u16) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWMOutputMediaProps {}
impl IWMOutputMediaProps_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWMOutputMediaProps_Vtbl
    where
        Identity: IWMOutputMediaProps_Impl,
    {
        unsafe extern "system" fn GetStreamGroupName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszname: windows_core::PWSTR, pcchname: *mut u16) -> windows_core::HRESULT
        where
            Identity: IWMOutputMediaProps_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMOutputMediaProps_Impl::GetStreamGroupName(this, core::mem::transmute_copy(&pwszname), core::mem::transmute_copy(&pcchname)).into()
        }
        unsafe extern "system" fn GetConnectionName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszname: windows_core::PWSTR, pcchname: *mut u16) -> windows_core::HRESULT
        where
            Identity: IWMOutputMediaProps_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMOutputMediaProps_Impl::GetConnectionName(this, core::mem::transmute_copy(&pwszname), core::mem::transmute_copy(&pcchname)).into()
        }
        Self {
            base__: IWMMediaProps_Vtbl::new::<Identity, OFFSET>(),
            GetStreamGroupName: GetStreamGroupName::<Identity, OFFSET>,
            GetConnectionName: GetConnectionName::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMOutputMediaProps as windows_core::Interface>::IID || iid == &<IWMMediaProps as windows_core::Interface>::IID
    }
}
pub trait IWMPacketSize_Impl: Sized {
    fn GetMaxPacketSize(&self) -> windows_core::Result<u32>;
    fn SetMaxPacketSize(&self, dwmaxpacketsize: u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWMPacketSize {}
impl IWMPacketSize_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWMPacketSize_Vtbl
    where
        Identity: IWMPacketSize_Impl,
    {
        unsafe extern "system" fn GetMaxPacketSize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwmaxpacketsize: *mut u32) -> windows_core::HRESULT
        where
            Identity: IWMPacketSize_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMPacketSize_Impl::GetMaxPacketSize(this) {
                Ok(ok__) => {
                    pdwmaxpacketsize.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxPacketSize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwmaxpacketsize: u32) -> windows_core::HRESULT
        where
            Identity: IWMPacketSize_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPacketSize_Impl::SetMaxPacketSize(this, core::mem::transmute_copy(&dwmaxpacketsize)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetMaxPacketSize: GetMaxPacketSize::<Identity, OFFSET>,
            SetMaxPacketSize: SetMaxPacketSize::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMPacketSize as windows_core::Interface>::IID
    }
}
pub trait IWMPacketSize2_Impl: Sized + IWMPacketSize_Impl {
    fn GetMinPacketSize(&self) -> windows_core::Result<u32>;
    fn SetMinPacketSize(&self, dwminpacketsize: u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWMPacketSize2 {}
impl IWMPacketSize2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWMPacketSize2_Vtbl
    where
        Identity: IWMPacketSize2_Impl,
    {
        unsafe extern "system" fn GetMinPacketSize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwminpacketsize: *mut u32) -> windows_core::HRESULT
        where
            Identity: IWMPacketSize2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMPacketSize2_Impl::GetMinPacketSize(this) {
                Ok(ok__) => {
                    pdwminpacketsize.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMinPacketSize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwminpacketsize: u32) -> windows_core::HRESULT
        where
            Identity: IWMPacketSize2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPacketSize2_Impl::SetMinPacketSize(this, core::mem::transmute_copy(&dwminpacketsize)).into()
        }
        Self {
            base__: IWMPacketSize_Vtbl::new::<Identity, OFFSET>(),
            GetMinPacketSize: GetMinPacketSize::<Identity, OFFSET>,
            SetMinPacketSize: SetMinPacketSize::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMPacketSize2 as windows_core::Interface>::IID || iid == &<IWMPacketSize as windows_core::Interface>::IID
    }
}
pub trait IWMPlayerHook_Impl: Sized {
    fn PreDecode(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWMPlayerHook {}
impl IWMPlayerHook_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWMPlayerHook_Vtbl
    where
        Identity: IWMPlayerHook_Impl,
    {
        unsafe extern "system" fn PreDecode<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMPlayerHook_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPlayerHook_Impl::PreDecode(this).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), PreDecode: PreDecode::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMPlayerHook as windows_core::Interface>::IID
    }
}
pub trait IWMPlayerTimestampHook_Impl: Sized {
    fn MapTimestamp(&self, rtin: i64) -> windows_core::Result<i64>;
}
impl windows_core::RuntimeName for IWMPlayerTimestampHook {}
impl IWMPlayerTimestampHook_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWMPlayerTimestampHook_Vtbl
    where
        Identity: IWMPlayerTimestampHook_Impl,
    {
        unsafe extern "system" fn MapTimestamp<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, rtin: i64, prtout: *mut i64) -> windows_core::HRESULT
        where
            Identity: IWMPlayerTimestampHook_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMPlayerTimestampHook_Impl::MapTimestamp(this, core::mem::transmute_copy(&rtin)) {
                Ok(ok__) => {
                    prtout.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), MapTimestamp: MapTimestamp::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMPlayerTimestampHook as windows_core::Interface>::IID
    }
}
pub trait IWMProfile_Impl: Sized {
    fn GetVersion(&self) -> windows_core::Result<WMT_VERSION>;
    fn GetName(&self, pwszname: windows_core::PWSTR, pcchname: *mut u32) -> windows_core::Result<()>;
    fn SetName(&self, pwszname: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetDescription(&self, pwszdescription: windows_core::PWSTR, pcchdescription: *mut u32) -> windows_core::Result<()>;
    fn SetDescription(&self, pwszdescription: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetStreamCount(&self) -> windows_core::Result<u32>;
    fn GetStream(&self, dwstreamindex: u32) -> windows_core::Result<IWMStreamConfig>;
    fn GetStreamByNumber(&self, wstreamnum: u16) -> windows_core::Result<IWMStreamConfig>;
    fn RemoveStream(&self, pconfig: Option<&IWMStreamConfig>) -> windows_core::Result<()>;
    fn RemoveStreamByNumber(&self, wstreamnum: u16) -> windows_core::Result<()>;
    fn AddStream(&self, pconfig: Option<&IWMStreamConfig>) -> windows_core::Result<()>;
    fn ReconfigStream(&self, pconfig: Option<&IWMStreamConfig>) -> windows_core::Result<()>;
    fn CreateNewStream(&self, guidstreamtype: *const windows_core::GUID) -> windows_core::Result<IWMStreamConfig>;
    fn GetMutualExclusionCount(&self) -> windows_core::Result<u32>;
    fn GetMutualExclusion(&self, dwmeindex: u32) -> windows_core::Result<IWMMutualExclusion>;
    fn RemoveMutualExclusion(&self, pme: Option<&IWMMutualExclusion>) -> windows_core::Result<()>;
    fn AddMutualExclusion(&self, pme: Option<&IWMMutualExclusion>) -> windows_core::Result<()>;
    fn CreateNewMutualExclusion(&self) -> windows_core::Result<IWMMutualExclusion>;
}
impl windows_core::RuntimeName for IWMProfile {}
impl IWMProfile_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWMProfile_Vtbl
    where
        Identity: IWMProfile_Impl,
    {
        unsafe extern "system" fn GetVersion<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwversion: *mut WMT_VERSION) -> windows_core::HRESULT
        where
            Identity: IWMProfile_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMProfile_Impl::GetVersion(this) {
                Ok(ok__) => {
                    pdwversion.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszname: windows_core::PWSTR, pcchname: *mut u32) -> windows_core::HRESULT
        where
            Identity: IWMProfile_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMProfile_Impl::GetName(this, core::mem::transmute_copy(&pwszname), core::mem::transmute_copy(&pcchname)).into()
        }
        unsafe extern "system" fn SetName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszname: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IWMProfile_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMProfile_Impl::SetName(this, core::mem::transmute(&pwszname)).into()
        }
        unsafe extern "system" fn GetDescription<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszdescription: windows_core::PWSTR, pcchdescription: *mut u32) -> windows_core::HRESULT
        where
            Identity: IWMProfile_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMProfile_Impl::GetDescription(this, core::mem::transmute_copy(&pwszdescription), core::mem::transmute_copy(&pcchdescription)).into()
        }
        unsafe extern "system" fn SetDescription<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszdescription: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IWMProfile_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMProfile_Impl::SetDescription(this, core::mem::transmute(&pwszdescription)).into()
        }
        unsafe extern "system" fn GetStreamCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcstreams: *mut u32) -> windows_core::HRESULT
        where
            Identity: IWMProfile_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMProfile_Impl::GetStreamCount(this) {
                Ok(ok__) => {
                    pcstreams.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStream<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwstreamindex: u32, ppconfig: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMProfile_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMProfile_Impl::GetStream(this, core::mem::transmute_copy(&dwstreamindex)) {
                Ok(ok__) => {
                    ppconfig.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStreamByNumber<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, wstreamnum: u16, ppconfig: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMProfile_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMProfile_Impl::GetStreamByNumber(this, core::mem::transmute_copy(&wstreamnum)) {
                Ok(ok__) => {
                    ppconfig.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveStream<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pconfig: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMProfile_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMProfile_Impl::RemoveStream(this, windows_core::from_raw_borrowed(&pconfig)).into()
        }
        unsafe extern "system" fn RemoveStreamByNumber<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, wstreamnum: u16) -> windows_core::HRESULT
        where
            Identity: IWMProfile_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMProfile_Impl::RemoveStreamByNumber(this, core::mem::transmute_copy(&wstreamnum)).into()
        }
        unsafe extern "system" fn AddStream<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pconfig: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMProfile_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMProfile_Impl::AddStream(this, windows_core::from_raw_borrowed(&pconfig)).into()
        }
        unsafe extern "system" fn ReconfigStream<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pconfig: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMProfile_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMProfile_Impl::ReconfigStream(this, windows_core::from_raw_borrowed(&pconfig)).into()
        }
        unsafe extern "system" fn CreateNewStream<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, guidstreamtype: *const windows_core::GUID, ppconfig: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMProfile_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMProfile_Impl::CreateNewStream(this, core::mem::transmute_copy(&guidstreamtype)) {
                Ok(ok__) => {
                    ppconfig.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMutualExclusionCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcme: *mut u32) -> windows_core::HRESULT
        where
            Identity: IWMProfile_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMProfile_Impl::GetMutualExclusionCount(this) {
                Ok(ok__) => {
                    pcme.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMutualExclusion<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwmeindex: u32, ppme: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMProfile_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMProfile_Impl::GetMutualExclusion(this, core::mem::transmute_copy(&dwmeindex)) {
                Ok(ok__) => {
                    ppme.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveMutualExclusion<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pme: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMProfile_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMProfile_Impl::RemoveMutualExclusion(this, windows_core::from_raw_borrowed(&pme)).into()
        }
        unsafe extern "system" fn AddMutualExclusion<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pme: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMProfile_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMProfile_Impl::AddMutualExclusion(this, windows_core::from_raw_borrowed(&pme)).into()
        }
        unsafe extern "system" fn CreateNewMutualExclusion<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppme: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMProfile_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMProfile_Impl::CreateNewMutualExclusion(this) {
                Ok(ok__) => {
                    ppme.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetVersion: GetVersion::<Identity, OFFSET>,
            GetName: GetName::<Identity, OFFSET>,
            SetName: SetName::<Identity, OFFSET>,
            GetDescription: GetDescription::<Identity, OFFSET>,
            SetDescription: SetDescription::<Identity, OFFSET>,
            GetStreamCount: GetStreamCount::<Identity, OFFSET>,
            GetStream: GetStream::<Identity, OFFSET>,
            GetStreamByNumber: GetStreamByNumber::<Identity, OFFSET>,
            RemoveStream: RemoveStream::<Identity, OFFSET>,
            RemoveStreamByNumber: RemoveStreamByNumber::<Identity, OFFSET>,
            AddStream: AddStream::<Identity, OFFSET>,
            ReconfigStream: ReconfigStream::<Identity, OFFSET>,
            CreateNewStream: CreateNewStream::<Identity, OFFSET>,
            GetMutualExclusionCount: GetMutualExclusionCount::<Identity, OFFSET>,
            GetMutualExclusion: GetMutualExclusion::<Identity, OFFSET>,
            RemoveMutualExclusion: RemoveMutualExclusion::<Identity, OFFSET>,
            AddMutualExclusion: AddMutualExclusion::<Identity, OFFSET>,
            CreateNewMutualExclusion: CreateNewMutualExclusion::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMProfile as windows_core::Interface>::IID
    }
}
pub trait IWMProfile2_Impl: Sized + IWMProfile_Impl {
    fn GetProfileID(&self) -> windows_core::Result<windows_core::GUID>;
}
impl windows_core::RuntimeName for IWMProfile2 {}
impl IWMProfile2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWMProfile2_Vtbl
    where
        Identity: IWMProfile2_Impl,
    {
        unsafe extern "system" fn GetProfileID<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pguidid: *mut windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IWMProfile2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMProfile2_Impl::GetProfileID(this) {
                Ok(ok__) => {
                    pguidid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: IWMProfile_Vtbl::new::<Identity, OFFSET>(), GetProfileID: GetProfileID::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMProfile2 as windows_core::Interface>::IID || iid == &<IWMProfile as windows_core::Interface>::IID
    }
}
pub trait IWMProfile3_Impl: Sized + IWMProfile2_Impl {
    fn GetStorageFormat(&self) -> windows_core::Result<WMT_STORAGE_FORMAT>;
    fn SetStorageFormat(&self, nstorageformat: WMT_STORAGE_FORMAT) -> windows_core::Result<()>;
    fn GetBandwidthSharingCount(&self) -> windows_core::Result<u32>;
    fn GetBandwidthSharing(&self, dwbsindex: u32) -> windows_core::Result<IWMBandwidthSharing>;
    fn RemoveBandwidthSharing(&self, pbs: Option<&IWMBandwidthSharing>) -> windows_core::Result<()>;
    fn AddBandwidthSharing(&self, pbs: Option<&IWMBandwidthSharing>) -> windows_core::Result<()>;
    fn CreateNewBandwidthSharing(&self) -> windows_core::Result<IWMBandwidthSharing>;
    fn GetStreamPrioritization(&self) -> windows_core::Result<IWMStreamPrioritization>;
    fn SetStreamPrioritization(&self, psp: Option<&IWMStreamPrioritization>) -> windows_core::Result<()>;
    fn RemoveStreamPrioritization(&self) -> windows_core::Result<()>;
    fn CreateNewStreamPrioritization(&self) -> windows_core::Result<IWMStreamPrioritization>;
    fn GetExpectedPacketCount(&self, msduration: u64) -> windows_core::Result<u64>;
}
impl windows_core::RuntimeName for IWMProfile3 {}
impl IWMProfile3_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWMProfile3_Vtbl
    where
        Identity: IWMProfile3_Impl,
    {
        unsafe extern "system" fn GetStorageFormat<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnstorageformat: *mut WMT_STORAGE_FORMAT) -> windows_core::HRESULT
        where
            Identity: IWMProfile3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMProfile3_Impl::GetStorageFormat(this) {
                Ok(ok__) => {
                    pnstorageformat.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStorageFormat<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, nstorageformat: WMT_STORAGE_FORMAT) -> windows_core::HRESULT
        where
            Identity: IWMProfile3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMProfile3_Impl::SetStorageFormat(this, core::mem::transmute_copy(&nstorageformat)).into()
        }
        unsafe extern "system" fn GetBandwidthSharingCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcbs: *mut u32) -> windows_core::HRESULT
        where
            Identity: IWMProfile3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMProfile3_Impl::GetBandwidthSharingCount(this) {
                Ok(ok__) => {
                    pcbs.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBandwidthSharing<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwbsindex: u32, ppbs: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMProfile3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMProfile3_Impl::GetBandwidthSharing(this, core::mem::transmute_copy(&dwbsindex)) {
                Ok(ok__) => {
                    ppbs.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveBandwidthSharing<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbs: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMProfile3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMProfile3_Impl::RemoveBandwidthSharing(this, windows_core::from_raw_borrowed(&pbs)).into()
        }
        unsafe extern "system" fn AddBandwidthSharing<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbs: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMProfile3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMProfile3_Impl::AddBandwidthSharing(this, windows_core::from_raw_borrowed(&pbs)).into()
        }
        unsafe extern "system" fn CreateNewBandwidthSharing<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppbs: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMProfile3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMProfile3_Impl::CreateNewBandwidthSharing(this) {
                Ok(ok__) => {
                    ppbs.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStreamPrioritization<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppsp: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMProfile3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMProfile3_Impl::GetStreamPrioritization(this) {
                Ok(ok__) => {
                    ppsp.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStreamPrioritization<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, psp: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMProfile3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMProfile3_Impl::SetStreamPrioritization(this, windows_core::from_raw_borrowed(&psp)).into()
        }
        unsafe extern "system" fn RemoveStreamPrioritization<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMProfile3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMProfile3_Impl::RemoveStreamPrioritization(this).into()
        }
        unsafe extern "system" fn CreateNewStreamPrioritization<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppsp: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMProfile3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMProfile3_Impl::CreateNewStreamPrioritization(this) {
                Ok(ok__) => {
                    ppsp.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetExpectedPacketCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, msduration: u64, pcpackets: *mut u64) -> windows_core::HRESULT
        where
            Identity: IWMProfile3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMProfile3_Impl::GetExpectedPacketCount(this, core::mem::transmute_copy(&msduration)) {
                Ok(ok__) => {
                    pcpackets.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: IWMProfile2_Vtbl::new::<Identity, OFFSET>(),
            GetStorageFormat: GetStorageFormat::<Identity, OFFSET>,
            SetStorageFormat: SetStorageFormat::<Identity, OFFSET>,
            GetBandwidthSharingCount: GetBandwidthSharingCount::<Identity, OFFSET>,
            GetBandwidthSharing: GetBandwidthSharing::<Identity, OFFSET>,
            RemoveBandwidthSharing: RemoveBandwidthSharing::<Identity, OFFSET>,
            AddBandwidthSharing: AddBandwidthSharing::<Identity, OFFSET>,
            CreateNewBandwidthSharing: CreateNewBandwidthSharing::<Identity, OFFSET>,
            GetStreamPrioritization: GetStreamPrioritization::<Identity, OFFSET>,
            SetStreamPrioritization: SetStreamPrioritization::<Identity, OFFSET>,
            RemoveStreamPrioritization: RemoveStreamPrioritization::<Identity, OFFSET>,
            CreateNewStreamPrioritization: CreateNewStreamPrioritization::<Identity, OFFSET>,
            GetExpectedPacketCount: GetExpectedPacketCount::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMProfile3 as windows_core::Interface>::IID || iid == &<IWMProfile as windows_core::Interface>::IID || iid == &<IWMProfile2 as windows_core::Interface>::IID
    }
}
pub trait IWMProfileManager_Impl: Sized {
    fn CreateEmptyProfile(&self, dwversion: WMT_VERSION) -> windows_core::Result<IWMProfile>;
    fn LoadProfileByID(&self, guidprofile: *const windows_core::GUID) -> windows_core::Result<IWMProfile>;
    fn LoadProfileByData(&self, pwszprofile: &windows_core::PCWSTR) -> windows_core::Result<IWMProfile>;
    fn SaveProfile(&self, piwmprofile: Option<&IWMProfile>, pwszprofile: &windows_core::PCWSTR, pdwlength: *mut u32) -> windows_core::Result<()>;
    fn GetSystemProfileCount(&self) -> windows_core::Result<u32>;
    fn LoadSystemProfile(&self, dwprofileindex: u32) -> windows_core::Result<IWMProfile>;
}
impl windows_core::RuntimeName for IWMProfileManager {}
impl IWMProfileManager_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWMProfileManager_Vtbl
    where
        Identity: IWMProfileManager_Impl,
    {
        unsafe extern "system" fn CreateEmptyProfile<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwversion: WMT_VERSION, ppprofile: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMProfileManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMProfileManager_Impl::CreateEmptyProfile(this, core::mem::transmute_copy(&dwversion)) {
                Ok(ok__) => {
                    ppprofile.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LoadProfileByID<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, guidprofile: *const windows_core::GUID, ppprofile: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMProfileManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMProfileManager_Impl::LoadProfileByID(this, core::mem::transmute_copy(&guidprofile)) {
                Ok(ok__) => {
                    ppprofile.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LoadProfileByData<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszprofile: windows_core::PCWSTR, ppprofile: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMProfileManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMProfileManager_Impl::LoadProfileByData(this, core::mem::transmute(&pwszprofile)) {
                Ok(ok__) => {
                    ppprofile.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SaveProfile<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, piwmprofile: *mut core::ffi::c_void, pwszprofile: windows_core::PCWSTR, pdwlength: *mut u32) -> windows_core::HRESULT
        where
            Identity: IWMProfileManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMProfileManager_Impl::SaveProfile(this, windows_core::from_raw_borrowed(&piwmprofile), core::mem::transmute(&pwszprofile), core::mem::transmute_copy(&pdwlength)).into()
        }
        unsafe extern "system" fn GetSystemProfileCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcprofiles: *mut u32) -> windows_core::HRESULT
        where
            Identity: IWMProfileManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMProfileManager_Impl::GetSystemProfileCount(this) {
                Ok(ok__) => {
                    pcprofiles.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LoadSystemProfile<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwprofileindex: u32, ppprofile: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMProfileManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMProfileManager_Impl::LoadSystemProfile(this, core::mem::transmute_copy(&dwprofileindex)) {
                Ok(ok__) => {
                    ppprofile.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateEmptyProfile: CreateEmptyProfile::<Identity, OFFSET>,
            LoadProfileByID: LoadProfileByID::<Identity, OFFSET>,
            LoadProfileByData: LoadProfileByData::<Identity, OFFSET>,
            SaveProfile: SaveProfile::<Identity, OFFSET>,
            GetSystemProfileCount: GetSystemProfileCount::<Identity, OFFSET>,
            LoadSystemProfile: LoadSystemProfile::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMProfileManager as windows_core::Interface>::IID
    }
}
pub trait IWMProfileManager2_Impl: Sized + IWMProfileManager_Impl {
    fn GetSystemProfileVersion(&self, pdwversion: *mut WMT_VERSION) -> windows_core::Result<()>;
    fn SetSystemProfileVersion(&self, dwversion: WMT_VERSION) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWMProfileManager2 {}
impl IWMProfileManager2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWMProfileManager2_Vtbl
    where
        Identity: IWMProfileManager2_Impl,
    {
        unsafe extern "system" fn GetSystemProfileVersion<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwversion: *mut WMT_VERSION) -> windows_core::HRESULT
        where
            Identity: IWMProfileManager2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMProfileManager2_Impl::GetSystemProfileVersion(this, core::mem::transmute_copy(&pdwversion)).into()
        }
        unsafe extern "system" fn SetSystemProfileVersion<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwversion: WMT_VERSION) -> windows_core::HRESULT
        where
            Identity: IWMProfileManager2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMProfileManager2_Impl::SetSystemProfileVersion(this, core::mem::transmute_copy(&dwversion)).into()
        }
        Self {
            base__: IWMProfileManager_Vtbl::new::<Identity, OFFSET>(),
            GetSystemProfileVersion: GetSystemProfileVersion::<Identity, OFFSET>,
            SetSystemProfileVersion: SetSystemProfileVersion::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMProfileManager2 as windows_core::Interface>::IID || iid == &<IWMProfileManager as windows_core::Interface>::IID
    }
}
pub trait IWMProfileManagerLanguage_Impl: Sized {
    fn GetUserLanguageID(&self, wlangid: *mut u16) -> windows_core::Result<()>;
    fn SetUserLanguageID(&self, wlangid: u16) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWMProfileManagerLanguage {}
impl IWMProfileManagerLanguage_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWMProfileManagerLanguage_Vtbl
    where
        Identity: IWMProfileManagerLanguage_Impl,
    {
        unsafe extern "system" fn GetUserLanguageID<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, wlangid: *mut u16) -> windows_core::HRESULT
        where
            Identity: IWMProfileManagerLanguage_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMProfileManagerLanguage_Impl::GetUserLanguageID(this, core::mem::transmute_copy(&wlangid)).into()
        }
        unsafe extern "system" fn SetUserLanguageID<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, wlangid: u16) -> windows_core::HRESULT
        where
            Identity: IWMProfileManagerLanguage_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMProfileManagerLanguage_Impl::SetUserLanguageID(this, core::mem::transmute_copy(&wlangid)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetUserLanguageID: GetUserLanguageID::<Identity, OFFSET>,
            SetUserLanguageID: SetUserLanguageID::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMProfileManagerLanguage as windows_core::Interface>::IID
    }
}
pub trait IWMPropertyVault_Impl: Sized {
    fn GetPropertyCount(&self, pdwcount: *const u32) -> windows_core::Result<()>;
    fn GetPropertyByName(&self, pszname: &windows_core::PCWSTR, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pdwsize: *mut u32) -> windows_core::Result<()>;
    fn SetProperty(&self, pszname: &windows_core::PCWSTR, ptype: WMT_ATTR_DATATYPE, pvalue: *const u8, dwsize: u32) -> windows_core::Result<()>;
    fn GetPropertyByIndex(&self, dwindex: u32, pszname: windows_core::PWSTR, pdwnamelen: *mut u32, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pdwsize: *mut u32) -> windows_core::Result<()>;
    fn CopyPropertiesFrom(&self, piwmpropertyvault: Option<&IWMPropertyVault>) -> windows_core::Result<()>;
    fn Clear(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWMPropertyVault {}
impl IWMPropertyVault_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWMPropertyVault_Vtbl
    where
        Identity: IWMPropertyVault_Impl,
    {
        unsafe extern "system" fn GetPropertyCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwcount: *const u32) -> windows_core::HRESULT
        where
            Identity: IWMPropertyVault_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPropertyVault_Impl::GetPropertyCount(this, core::mem::transmute_copy(&pdwcount)).into()
        }
        unsafe extern "system" fn GetPropertyByName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszname: windows_core::PCWSTR, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pdwsize: *mut u32) -> windows_core::HRESULT
        where
            Identity: IWMPropertyVault_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPropertyVault_Impl::GetPropertyByName(this, core::mem::transmute(&pszname), core::mem::transmute_copy(&ptype), core::mem::transmute_copy(&pvalue), core::mem::transmute_copy(&pdwsize)).into()
        }
        unsafe extern "system" fn SetProperty<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszname: windows_core::PCWSTR, ptype: WMT_ATTR_DATATYPE, pvalue: *const u8, dwsize: u32) -> windows_core::HRESULT
        where
            Identity: IWMPropertyVault_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPropertyVault_Impl::SetProperty(this, core::mem::transmute(&pszname), core::mem::transmute_copy(&ptype), core::mem::transmute_copy(&pvalue), core::mem::transmute_copy(&dwsize)).into()
        }
        unsafe extern "system" fn GetPropertyByIndex<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwindex: u32, pszname: windows_core::PWSTR, pdwnamelen: *mut u32, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pdwsize: *mut u32) -> windows_core::HRESULT
        where
            Identity: IWMPropertyVault_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPropertyVault_Impl::GetPropertyByIndex(this, core::mem::transmute_copy(&dwindex), core::mem::transmute_copy(&pszname), core::mem::transmute_copy(&pdwnamelen), core::mem::transmute_copy(&ptype), core::mem::transmute_copy(&pvalue), core::mem::transmute_copy(&pdwsize)).into()
        }
        unsafe extern "system" fn CopyPropertiesFrom<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, piwmpropertyvault: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMPropertyVault_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPropertyVault_Impl::CopyPropertiesFrom(this, windows_core::from_raw_borrowed(&piwmpropertyvault)).into()
        }
        unsafe extern "system" fn Clear<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMPropertyVault_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPropertyVault_Impl::Clear(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetPropertyCount: GetPropertyCount::<Identity, OFFSET>,
            GetPropertyByName: GetPropertyByName::<Identity, OFFSET>,
            SetProperty: SetProperty::<Identity, OFFSET>,
            GetPropertyByIndex: GetPropertyByIndex::<Identity, OFFSET>,
            CopyPropertiesFrom: CopyPropertiesFrom::<Identity, OFFSET>,
            Clear: Clear::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMPropertyVault as windows_core::Interface>::IID
    }
}
pub trait IWMProximityDetection_Impl: Sized {
    fn StartDetection(&self, pbregistrationmsg: *const u8, cbregistrationmsg: u32, pblocaladdress: *const u8, cblocaladdress: u32, dwextraportsallowed: u32, ppregistrationresponsemsg: *mut Option<INSSBuffer>, pcallback: Option<&IWMStatusCallback>, pvcontext: *const core::ffi::c_void) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWMProximityDetection {}
impl IWMProximityDetection_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWMProximityDetection_Vtbl
    where
        Identity: IWMProximityDetection_Impl,
    {
        unsafe extern "system" fn StartDetection<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbregistrationmsg: *const u8, cbregistrationmsg: u32, pblocaladdress: *const u8, cblocaladdress: u32, dwextraportsallowed: u32, ppregistrationresponsemsg: *mut *mut core::ffi::c_void, pcallback: *mut core::ffi::c_void, pvcontext: *const core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMProximityDetection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMProximityDetection_Impl::StartDetection(this, core::mem::transmute_copy(&pbregistrationmsg), core::mem::transmute_copy(&cbregistrationmsg), core::mem::transmute_copy(&pblocaladdress), core::mem::transmute_copy(&cblocaladdress), core::mem::transmute_copy(&dwextraportsallowed), core::mem::transmute_copy(&ppregistrationresponsemsg), windows_core::from_raw_borrowed(&pcallback), core::mem::transmute_copy(&pvcontext)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), StartDetection: StartDetection::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMProximityDetection as windows_core::Interface>::IID
    }
}
pub trait IWMReader_Impl: Sized {
    fn Open(&self, pwszurl: &windows_core::PCWSTR, pcallback: Option<&IWMReaderCallback>, pvcontext: *const core::ffi::c_void) -> windows_core::Result<()>;
    fn Close(&self) -> windows_core::Result<()>;
    fn GetOutputCount(&self) -> windows_core::Result<u32>;
    fn GetOutputProps(&self, dwoutputnum: u32) -> windows_core::Result<IWMOutputMediaProps>;
    fn SetOutputProps(&self, dwoutputnum: u32, poutput: Option<&IWMOutputMediaProps>) -> windows_core::Result<()>;
    fn GetOutputFormatCount(&self, dwoutputnumber: u32) -> windows_core::Result<u32>;
    fn GetOutputFormat(&self, dwoutputnumber: u32, dwformatnumber: u32) -> windows_core::Result<IWMOutputMediaProps>;
    fn Start(&self, cnsstart: u64, cnsduration: u64, frate: f32, pvcontext: *const core::ffi::c_void) -> windows_core::Result<()>;
    fn Stop(&self) -> windows_core::Result<()>;
    fn Pause(&self) -> windows_core::Result<()>;
    fn Resume(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWMReader {}
impl IWMReader_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWMReader_Vtbl
    where
        Identity: IWMReader_Impl,
    {
        unsafe extern "system" fn Open<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszurl: windows_core::PCWSTR, pcallback: *mut core::ffi::c_void, pvcontext: *const core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMReader_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMReader_Impl::Open(this, core::mem::transmute(&pwszurl), windows_core::from_raw_borrowed(&pcallback), core::mem::transmute_copy(&pvcontext)).into()
        }
        unsafe extern "system" fn Close<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMReader_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMReader_Impl::Close(this).into()
        }
        unsafe extern "system" fn GetOutputCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcoutputs: *mut u32) -> windows_core::HRESULT
        where
            Identity: IWMReader_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMReader_Impl::GetOutputCount(this) {
                Ok(ok__) => {
                    pcoutputs.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOutputProps<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwoutputnum: u32, ppoutput: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMReader_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMReader_Impl::GetOutputProps(this, core::mem::transmute_copy(&dwoutputnum)) {
                Ok(ok__) => {
                    ppoutput.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOutputProps<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwoutputnum: u32, poutput: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMReader_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMReader_Impl::SetOutputProps(this, core::mem::transmute_copy(&dwoutputnum), windows_core::from_raw_borrowed(&poutput)).into()
        }
        unsafe extern "system" fn GetOutputFormatCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwoutputnumber: u32, pcformats: *mut u32) -> windows_core::HRESULT
        where
            Identity: IWMReader_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMReader_Impl::GetOutputFormatCount(this, core::mem::transmute_copy(&dwoutputnumber)) {
                Ok(ok__) => {
                    pcformats.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOutputFormat<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwoutputnumber: u32, dwformatnumber: u32, ppprops: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMReader_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMReader_Impl::GetOutputFormat(this, core::mem::transmute_copy(&dwoutputnumber), core::mem::transmute_copy(&dwformatnumber)) {
                Ok(ok__) => {
                    ppprops.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Start<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, cnsstart: u64, cnsduration: u64, frate: f32, pvcontext: *const core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMReader_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMReader_Impl::Start(this, core::mem::transmute_copy(&cnsstart), core::mem::transmute_copy(&cnsduration), core::mem::transmute_copy(&frate), core::mem::transmute_copy(&pvcontext)).into()
        }
        unsafe extern "system" fn Stop<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMReader_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMReader_Impl::Stop(this).into()
        }
        unsafe extern "system" fn Pause<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMReader_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMReader_Impl::Pause(this).into()
        }
        unsafe extern "system" fn Resume<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMReader_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMReader_Impl::Resume(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Open: Open::<Identity, OFFSET>,
            Close: Close::<Identity, OFFSET>,
            GetOutputCount: GetOutputCount::<Identity, OFFSET>,
            GetOutputProps: GetOutputProps::<Identity, OFFSET>,
            SetOutputProps: SetOutputProps::<Identity, OFFSET>,
            GetOutputFormatCount: GetOutputFormatCount::<Identity, OFFSET>,
            GetOutputFormat: GetOutputFormat::<Identity, OFFSET>,
            Start: Start::<Identity, OFFSET>,
            Stop: Stop::<Identity, OFFSET>,
            Pause: Pause::<Identity, OFFSET>,
            Resume: Resume::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMReader as windows_core::Interface>::IID
    }
}
pub trait IWMReaderAccelerator_Impl: Sized {
    fn GetCodecInterface(&self, dwoutputnum: u32, riid: *const windows_core::GUID, ppvcodecinterface: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn Notify(&self, dwoutputnum: u32, psubtype: *const WM_MEDIA_TYPE) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWMReaderAccelerator {}
impl IWMReaderAccelerator_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWMReaderAccelerator_Vtbl
    where
        Identity: IWMReaderAccelerator_Impl,
    {
        unsafe extern "system" fn GetCodecInterface<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwoutputnum: u32, riid: *const windows_core::GUID, ppvcodecinterface: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMReaderAccelerator_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMReaderAccelerator_Impl::GetCodecInterface(this, core::mem::transmute_copy(&dwoutputnum), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppvcodecinterface)).into()
        }
        unsafe extern "system" fn Notify<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwoutputnum: u32, psubtype: *const WM_MEDIA_TYPE) -> windows_core::HRESULT
        where
            Identity: IWMReaderAccelerator_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMReaderAccelerator_Impl::Notify(this, core::mem::transmute_copy(&dwoutputnum), core::mem::transmute_copy(&psubtype)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCodecInterface: GetCodecInterface::<Identity, OFFSET>,
            Notify: Notify::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMReaderAccelerator as windows_core::Interface>::IID
    }
}
pub trait IWMReaderAdvanced_Impl: Sized {
    fn SetUserProvidedClock(&self, fuserclock: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn GetUserProvidedClock(&self) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn DeliverTime(&self, cnstime: u64) -> windows_core::Result<()>;
    fn SetManualStreamSelection(&self, fselection: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn GetManualStreamSelection(&self) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn SetStreamsSelected(&self, cstreamcount: u16, pwstreamnumbers: *const u16, pselections: *const WMT_STREAM_SELECTION) -> windows_core::Result<()>;
    fn GetStreamSelected(&self, wstreamnum: u16) -> windows_core::Result<WMT_STREAM_SELECTION>;
    fn SetReceiveSelectionCallbacks(&self, fgetcallbacks: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn GetReceiveSelectionCallbacks(&self) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn SetReceiveStreamSamples(&self, wstreamnum: u16, freceivestreamsamples: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn GetReceiveStreamSamples(&self, wstreamnum: u16) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn SetAllocateForOutput(&self, dwoutputnum: u32, fallocate: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn GetAllocateForOutput(&self, dwoutputnum: u32) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn SetAllocateForStream(&self, wstreamnum: u16, fallocate: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn GetAllocateForStream(&self, dwsreamnum: u16) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn GetStatistics(&self, pstatistics: *mut WM_READER_STATISTICS) -> windows_core::Result<()>;
    fn SetClientInfo(&self, pclientinfo: *const WM_READER_CLIENTINFO) -> windows_core::Result<()>;
    fn GetMaxOutputSampleSize(&self, dwoutput: u32) -> windows_core::Result<u32>;
    fn GetMaxStreamSampleSize(&self, wstream: u16) -> windows_core::Result<u32>;
    fn NotifyLateDelivery(&self, cnslateness: u64) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWMReaderAdvanced {}
impl IWMReaderAdvanced_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWMReaderAdvanced_Vtbl
    where
        Identity: IWMReaderAdvanced_Impl,
    {
        unsafe extern "system" fn SetUserProvidedClock<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fuserclock: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IWMReaderAdvanced_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMReaderAdvanced_Impl::SetUserProvidedClock(this, core::mem::transmute_copy(&fuserclock)).into()
        }
        unsafe extern "system" fn GetUserProvidedClock<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfuserclock: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IWMReaderAdvanced_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMReaderAdvanced_Impl::GetUserProvidedClock(this) {
                Ok(ok__) => {
                    pfuserclock.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeliverTime<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, cnstime: u64) -> windows_core::HRESULT
        where
            Identity: IWMReaderAdvanced_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMReaderAdvanced_Impl::DeliverTime(this, core::mem::transmute_copy(&cnstime)).into()
        }
        unsafe extern "system" fn SetManualStreamSelection<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fselection: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IWMReaderAdvanced_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMReaderAdvanced_Impl::SetManualStreamSelection(this, core::mem::transmute_copy(&fselection)).into()
        }
        unsafe extern "system" fn GetManualStreamSelection<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfselection: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IWMReaderAdvanced_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMReaderAdvanced_Impl::GetManualStreamSelection(this) {
                Ok(ok__) => {
                    pfselection.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStreamsSelected<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, cstreamcount: u16, pwstreamnumbers: *const u16, pselections: *const WMT_STREAM_SELECTION) -> windows_core::HRESULT
        where
            Identity: IWMReaderAdvanced_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMReaderAdvanced_Impl::SetStreamsSelected(this, core::mem::transmute_copy(&cstreamcount), core::mem::transmute_copy(&pwstreamnumbers), core::mem::transmute_copy(&pselections)).into()
        }
        unsafe extern "system" fn GetStreamSelected<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, wstreamnum: u16, pselection: *mut WMT_STREAM_SELECTION) -> windows_core::HRESULT
        where
            Identity: IWMReaderAdvanced_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMReaderAdvanced_Impl::GetStreamSelected(this, core::mem::transmute_copy(&wstreamnum)) {
                Ok(ok__) => {
                    pselection.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetReceiveSelectionCallbacks<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fgetcallbacks: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IWMReaderAdvanced_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMReaderAdvanced_Impl::SetReceiveSelectionCallbacks(this, core::mem::transmute_copy(&fgetcallbacks)).into()
        }
        unsafe extern "system" fn GetReceiveSelectionCallbacks<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfgetcallbacks: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IWMReaderAdvanced_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMReaderAdvanced_Impl::GetReceiveSelectionCallbacks(this) {
                Ok(ok__) => {
                    pfgetcallbacks.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetReceiveStreamSamples<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, wstreamnum: u16, freceivestreamsamples: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IWMReaderAdvanced_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMReaderAdvanced_Impl::SetReceiveStreamSamples(this, core::mem::transmute_copy(&wstreamnum), core::mem::transmute_copy(&freceivestreamsamples)).into()
        }
        unsafe extern "system" fn GetReceiveStreamSamples<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, wstreamnum: u16, pfreceivestreamsamples: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IWMReaderAdvanced_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMReaderAdvanced_Impl::GetReceiveStreamSamples(this, core::mem::transmute_copy(&wstreamnum)) {
                Ok(ok__) => {
                    pfreceivestreamsamples.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAllocateForOutput<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwoutputnum: u32, fallocate: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IWMReaderAdvanced_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMReaderAdvanced_Impl::SetAllocateForOutput(this, core::mem::transmute_copy(&dwoutputnum), core::mem::transmute_copy(&fallocate)).into()
        }
        unsafe extern "system" fn GetAllocateForOutput<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwoutputnum: u32, pfallocate: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IWMReaderAdvanced_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMReaderAdvanced_Impl::GetAllocateForOutput(this, core::mem::transmute_copy(&dwoutputnum)) {
                Ok(ok__) => {
                    pfallocate.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAllocateForStream<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, wstreamnum: u16, fallocate: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IWMReaderAdvanced_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMReaderAdvanced_Impl::SetAllocateForStream(this, core::mem::transmute_copy(&wstreamnum), core::mem::transmute_copy(&fallocate)).into()
        }
        unsafe extern "system" fn GetAllocateForStream<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwsreamnum: u16, pfallocate: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IWMReaderAdvanced_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMReaderAdvanced_Impl::GetAllocateForStream(this, core::mem::transmute_copy(&dwsreamnum)) {
                Ok(ok__) => {
                    pfallocate.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStatistics<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstatistics: *mut WM_READER_STATISTICS) -> windows_core::HRESULT
        where
            Identity: IWMReaderAdvanced_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMReaderAdvanced_Impl::GetStatistics(this, core::mem::transmute_copy(&pstatistics)).into()
        }
        unsafe extern "system" fn SetClientInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pclientinfo: *const WM_READER_CLIENTINFO) -> windows_core::HRESULT
        where
            Identity: IWMReaderAdvanced_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMReaderAdvanced_Impl::SetClientInfo(this, core::mem::transmute_copy(&pclientinfo)).into()
        }
        unsafe extern "system" fn GetMaxOutputSampleSize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwoutput: u32, pcbmax: *mut u32) -> windows_core::HRESULT
        where
            Identity: IWMReaderAdvanced_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMReaderAdvanced_Impl::GetMaxOutputSampleSize(this, core::mem::transmute_copy(&dwoutput)) {
                Ok(ok__) => {
                    pcbmax.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMaxStreamSampleSize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, wstream: u16, pcbmax: *mut u32) -> windows_core::HRESULT
        where
            Identity: IWMReaderAdvanced_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMReaderAdvanced_Impl::GetMaxStreamSampleSize(this, core::mem::transmute_copy(&wstream)) {
                Ok(ok__) => {
                    pcbmax.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NotifyLateDelivery<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, cnslateness: u64) -> windows_core::HRESULT
        where
            Identity: IWMReaderAdvanced_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMReaderAdvanced_Impl::NotifyLateDelivery(this, core::mem::transmute_copy(&cnslateness)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetUserProvidedClock: SetUserProvidedClock::<Identity, OFFSET>,
            GetUserProvidedClock: GetUserProvidedClock::<Identity, OFFSET>,
            DeliverTime: DeliverTime::<Identity, OFFSET>,
            SetManualStreamSelection: SetManualStreamSelection::<Identity, OFFSET>,
            GetManualStreamSelection: GetManualStreamSelection::<Identity, OFFSET>,
            SetStreamsSelected: SetStreamsSelected::<Identity, OFFSET>,
            GetStreamSelected: GetStreamSelected::<Identity, OFFSET>,
            SetReceiveSelectionCallbacks: SetReceiveSelectionCallbacks::<Identity, OFFSET>,
            GetReceiveSelectionCallbacks: GetReceiveSelectionCallbacks::<Identity, OFFSET>,
            SetReceiveStreamSamples: SetReceiveStreamSamples::<Identity, OFFSET>,
            GetReceiveStreamSamples: GetReceiveStreamSamples::<Identity, OFFSET>,
            SetAllocateForOutput: SetAllocateForOutput::<Identity, OFFSET>,
            GetAllocateForOutput: GetAllocateForOutput::<Identity, OFFSET>,
            SetAllocateForStream: SetAllocateForStream::<Identity, OFFSET>,
            GetAllocateForStream: GetAllocateForStream::<Identity, OFFSET>,
            GetStatistics: GetStatistics::<Identity, OFFSET>,
            SetClientInfo: SetClientInfo::<Identity, OFFSET>,
            GetMaxOutputSampleSize: GetMaxOutputSampleSize::<Identity, OFFSET>,
            GetMaxStreamSampleSize: GetMaxStreamSampleSize::<Identity, OFFSET>,
            NotifyLateDelivery: NotifyLateDelivery::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMReaderAdvanced as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWMReaderAdvanced2_Impl: Sized + IWMReaderAdvanced_Impl {
    fn SetPlayMode(&self, mode: WMT_PLAY_MODE) -> windows_core::Result<()>;
    fn GetPlayMode(&self) -> windows_core::Result<WMT_PLAY_MODE>;
    fn GetBufferProgress(&self, pdwpercent: *mut u32, pcnsbuffering: *mut u64) -> windows_core::Result<()>;
    fn GetDownloadProgress(&self, pdwpercent: *mut u32, pqwbytesdownloaded: *mut u64, pcnsdownload: *mut u64) -> windows_core::Result<()>;
    fn GetSaveAsProgress(&self) -> windows_core::Result<u32>;
    fn SaveFileAs(&self, pwszfilename: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetProtocolName(&self, pwszprotocol: windows_core::PWSTR, pcchprotocol: *mut u32) -> windows_core::Result<()>;
    fn StartAtMarker(&self, wmarkerindex: u16, cnsduration: u64, frate: f32, pvcontext: *const core::ffi::c_void) -> windows_core::Result<()>;
    fn GetOutputSetting(&self, dwoutputnum: u32, pszname: &windows_core::PCWSTR, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> windows_core::Result<()>;
    fn SetOutputSetting(&self, dwoutputnum: u32, pszname: &windows_core::PCWSTR, r#type: WMT_ATTR_DATATYPE, pvalue: *const u8, cblength: u16) -> windows_core::Result<()>;
    fn Preroll(&self, cnsstart: u64, cnsduration: u64, frate: f32) -> windows_core::Result<()>;
    fn SetLogClientID(&self, flogclientid: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn GetLogClientID(&self) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn StopBuffering(&self) -> windows_core::Result<()>;
    fn OpenStream(&self, pstream: Option<&super::super::System::Com::IStream>, pcallback: Option<&IWMReaderCallback>, pvcontext: *const core::ffi::c_void) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IWMReaderAdvanced2 {}
#[cfg(feature = "Win32_System_Com")]
impl IWMReaderAdvanced2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWMReaderAdvanced2_Vtbl
    where
        Identity: IWMReaderAdvanced2_Impl,
    {
        unsafe extern "system" fn SetPlayMode<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, mode: WMT_PLAY_MODE) -> windows_core::HRESULT
        where
            Identity: IWMReaderAdvanced2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMReaderAdvanced2_Impl::SetPlayMode(this, core::mem::transmute_copy(&mode)).into()
        }
        unsafe extern "system" fn GetPlayMode<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmode: *mut WMT_PLAY_MODE) -> windows_core::HRESULT
        where
            Identity: IWMReaderAdvanced2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMReaderAdvanced2_Impl::GetPlayMode(this) {
                Ok(ok__) => {
                    pmode.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBufferProgress<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwpercent: *mut u32, pcnsbuffering: *mut u64) -> windows_core::HRESULT
        where
            Identity: IWMReaderAdvanced2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMReaderAdvanced2_Impl::GetBufferProgress(this, core::mem::transmute_copy(&pdwpercent), core::mem::transmute_copy(&pcnsbuffering)).into()
        }
        unsafe extern "system" fn GetDownloadProgress<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwpercent: *mut u32, pqwbytesdownloaded: *mut u64, pcnsdownload: *mut u64) -> windows_core::HRESULT
        where
            Identity: IWMReaderAdvanced2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMReaderAdvanced2_Impl::GetDownloadProgress(this, core::mem::transmute_copy(&pdwpercent), core::mem::transmute_copy(&pqwbytesdownloaded), core::mem::transmute_copy(&pcnsdownload)).into()
        }
        unsafe extern "system" fn GetSaveAsProgress<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwpercent: *mut u32) -> windows_core::HRESULT
        where
            Identity: IWMReaderAdvanced2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMReaderAdvanced2_Impl::GetSaveAsProgress(this) {
                Ok(ok__) => {
                    pdwpercent.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SaveFileAs<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszfilename: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IWMReaderAdvanced2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMReaderAdvanced2_Impl::SaveFileAs(this, core::mem::transmute(&pwszfilename)).into()
        }
        unsafe extern "system" fn GetProtocolName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszprotocol: windows_core::PWSTR, pcchprotocol: *mut u32) -> windows_core::HRESULT
        where
            Identity: IWMReaderAdvanced2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMReaderAdvanced2_Impl::GetProtocolName(this, core::mem::transmute_copy(&pwszprotocol), core::mem::transmute_copy(&pcchprotocol)).into()
        }
        unsafe extern "system" fn StartAtMarker<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, wmarkerindex: u16, cnsduration: u64, frate: f32, pvcontext: *const core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMReaderAdvanced2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMReaderAdvanced2_Impl::StartAtMarker(this, core::mem::transmute_copy(&wmarkerindex), core::mem::transmute_copy(&cnsduration), core::mem::transmute_copy(&frate), core::mem::transmute_copy(&pvcontext)).into()
        }
        unsafe extern "system" fn GetOutputSetting<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwoutputnum: u32, pszname: windows_core::PCWSTR, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> windows_core::HRESULT
        where
            Identity: IWMReaderAdvanced2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMReaderAdvanced2_Impl::GetOutputSetting(this, core::mem::transmute_copy(&dwoutputnum), core::mem::transmute(&pszname), core::mem::transmute_copy(&ptype), core::mem::transmute_copy(&pvalue), core::mem::transmute_copy(&pcblength)).into()
        }
        unsafe extern "system" fn SetOutputSetting<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwoutputnum: u32, pszname: windows_core::PCWSTR, r#type: WMT_ATTR_DATATYPE, pvalue: *const u8, cblength: u16) -> windows_core::HRESULT
        where
            Identity: IWMReaderAdvanced2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMReaderAdvanced2_Impl::SetOutputSetting(this, core::mem::transmute_copy(&dwoutputnum), core::mem::transmute(&pszname), core::mem::transmute_copy(&r#type), core::mem::transmute_copy(&pvalue), core::mem::transmute_copy(&cblength)).into()
        }
        unsafe extern "system" fn Preroll<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, cnsstart: u64, cnsduration: u64, frate: f32) -> windows_core::HRESULT
        where
            Identity: IWMReaderAdvanced2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMReaderAdvanced2_Impl::Preroll(this, core::mem::transmute_copy(&cnsstart), core::mem::transmute_copy(&cnsduration), core::mem::transmute_copy(&frate)).into()
        }
        unsafe extern "system" fn SetLogClientID<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, flogclientid: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IWMReaderAdvanced2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMReaderAdvanced2_Impl::SetLogClientID(this, core::mem::transmute_copy(&flogclientid)).into()
        }
        unsafe extern "system" fn GetLogClientID<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pflogclientid: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IWMReaderAdvanced2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMReaderAdvanced2_Impl::GetLogClientID(this) {
                Ok(ok__) => {
                    pflogclientid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StopBuffering<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMReaderAdvanced2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMReaderAdvanced2_Impl::StopBuffering(this).into()
        }
        unsafe extern "system" fn OpenStream<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstream: *mut core::ffi::c_void, pcallback: *mut core::ffi::c_void, pvcontext: *const core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMReaderAdvanced2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMReaderAdvanced2_Impl::OpenStream(this, windows_core::from_raw_borrowed(&pstream), windows_core::from_raw_borrowed(&pcallback), core::mem::transmute_copy(&pvcontext)).into()
        }
        Self {
            base__: IWMReaderAdvanced_Vtbl::new::<Identity, OFFSET>(),
            SetPlayMode: SetPlayMode::<Identity, OFFSET>,
            GetPlayMode: GetPlayMode::<Identity, OFFSET>,
            GetBufferProgress: GetBufferProgress::<Identity, OFFSET>,
            GetDownloadProgress: GetDownloadProgress::<Identity, OFFSET>,
            GetSaveAsProgress: GetSaveAsProgress::<Identity, OFFSET>,
            SaveFileAs: SaveFileAs::<Identity, OFFSET>,
            GetProtocolName: GetProtocolName::<Identity, OFFSET>,
            StartAtMarker: StartAtMarker::<Identity, OFFSET>,
            GetOutputSetting: GetOutputSetting::<Identity, OFFSET>,
            SetOutputSetting: SetOutputSetting::<Identity, OFFSET>,
            Preroll: Preroll::<Identity, OFFSET>,
            SetLogClientID: SetLogClientID::<Identity, OFFSET>,
            GetLogClientID: GetLogClientID::<Identity, OFFSET>,
            StopBuffering: StopBuffering::<Identity, OFFSET>,
            OpenStream: OpenStream::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMReaderAdvanced2 as windows_core::Interface>::IID || iid == &<IWMReaderAdvanced as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWMReaderAdvanced3_Impl: Sized + IWMReaderAdvanced2_Impl {
    fn StopNetStreaming(&self) -> windows_core::Result<()>;
    fn StartAtPosition(&self, wstreamnum: u16, pvoffsetstart: *const core::ffi::c_void, pvduration: *const core::ffi::c_void, dwoffsetformat: WMT_OFFSET_FORMAT, frate: f32, pvcontext: *const core::ffi::c_void) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IWMReaderAdvanced3 {}
#[cfg(feature = "Win32_System_Com")]
impl IWMReaderAdvanced3_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWMReaderAdvanced3_Vtbl
    where
        Identity: IWMReaderAdvanced3_Impl,
    {
        unsafe extern "system" fn StopNetStreaming<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMReaderAdvanced3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMReaderAdvanced3_Impl::StopNetStreaming(this).into()
        }
        unsafe extern "system" fn StartAtPosition<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, wstreamnum: u16, pvoffsetstart: *const core::ffi::c_void, pvduration: *const core::ffi::c_void, dwoffsetformat: WMT_OFFSET_FORMAT, frate: f32, pvcontext: *const core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMReaderAdvanced3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMReaderAdvanced3_Impl::StartAtPosition(this, core::mem::transmute_copy(&wstreamnum), core::mem::transmute_copy(&pvoffsetstart), core::mem::transmute_copy(&pvduration), core::mem::transmute_copy(&dwoffsetformat), core::mem::transmute_copy(&frate), core::mem::transmute_copy(&pvcontext)).into()
        }
        Self {
            base__: IWMReaderAdvanced2_Vtbl::new::<Identity, OFFSET>(),
            StopNetStreaming: StopNetStreaming::<Identity, OFFSET>,
            StartAtPosition: StartAtPosition::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMReaderAdvanced3 as windows_core::Interface>::IID || iid == &<IWMReaderAdvanced as windows_core::Interface>::IID || iid == &<IWMReaderAdvanced2 as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWMReaderAdvanced4_Impl: Sized + IWMReaderAdvanced3_Impl {
    fn GetLanguageCount(&self, dwoutputnum: u32) -> windows_core::Result<u16>;
    fn GetLanguage(&self, dwoutputnum: u32, wlanguage: u16, pwszlanguagestring: windows_core::PWSTR, pcchlanguagestringlength: *mut u16) -> windows_core::Result<()>;
    fn GetMaxSpeedFactor(&self) -> windows_core::Result<f64>;
    fn IsUsingFastCache(&self) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn AddLogParam(&self, wsznamespace: &windows_core::PCWSTR, wszname: &windows_core::PCWSTR, wszvalue: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn SendLogParams(&self) -> windows_core::Result<()>;
    fn CanSaveFileAs(&self) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn CancelSaveFileAs(&self) -> windows_core::Result<()>;
    fn GetURL(&self, pwszurl: windows_core::PWSTR, pcchurl: *mut u32) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IWMReaderAdvanced4 {}
#[cfg(feature = "Win32_System_Com")]
impl IWMReaderAdvanced4_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWMReaderAdvanced4_Vtbl
    where
        Identity: IWMReaderAdvanced4_Impl,
    {
        unsafe extern "system" fn GetLanguageCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwoutputnum: u32, pwlanguagecount: *mut u16) -> windows_core::HRESULT
        where
            Identity: IWMReaderAdvanced4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMReaderAdvanced4_Impl::GetLanguageCount(this, core::mem::transmute_copy(&dwoutputnum)) {
                Ok(ok__) => {
                    pwlanguagecount.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLanguage<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwoutputnum: u32, wlanguage: u16, pwszlanguagestring: windows_core::PWSTR, pcchlanguagestringlength: *mut u16) -> windows_core::HRESULT
        where
            Identity: IWMReaderAdvanced4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMReaderAdvanced4_Impl::GetLanguage(this, core::mem::transmute_copy(&dwoutputnum), core::mem::transmute_copy(&wlanguage), core::mem::transmute_copy(&pwszlanguagestring), core::mem::transmute_copy(&pcchlanguagestringlength)).into()
        }
        unsafe extern "system" fn GetMaxSpeedFactor<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdblfactor: *mut f64) -> windows_core::HRESULT
        where
            Identity: IWMReaderAdvanced4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMReaderAdvanced4_Impl::GetMaxSpeedFactor(this) {
                Ok(ok__) => {
                    pdblfactor.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsUsingFastCache<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfusingfastcache: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IWMReaderAdvanced4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMReaderAdvanced4_Impl::IsUsingFastCache(this) {
                Ok(ok__) => {
                    pfusingfastcache.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddLogParam<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, wsznamespace: windows_core::PCWSTR, wszname: windows_core::PCWSTR, wszvalue: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IWMReaderAdvanced4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMReaderAdvanced4_Impl::AddLogParam(this, core::mem::transmute(&wsznamespace), core::mem::transmute(&wszname), core::mem::transmute(&wszvalue)).into()
        }
        unsafe extern "system" fn SendLogParams<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMReaderAdvanced4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMReaderAdvanced4_Impl::SendLogParams(this).into()
        }
        unsafe extern "system" fn CanSaveFileAs<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfcansave: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IWMReaderAdvanced4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMReaderAdvanced4_Impl::CanSaveFileAs(this) {
                Ok(ok__) => {
                    pfcansave.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CancelSaveFileAs<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMReaderAdvanced4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMReaderAdvanced4_Impl::CancelSaveFileAs(this).into()
        }
        unsafe extern "system" fn GetURL<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszurl: windows_core::PWSTR, pcchurl: *mut u32) -> windows_core::HRESULT
        where
            Identity: IWMReaderAdvanced4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMReaderAdvanced4_Impl::GetURL(this, core::mem::transmute_copy(&pwszurl), core::mem::transmute_copy(&pcchurl)).into()
        }
        Self {
            base__: IWMReaderAdvanced3_Vtbl::new::<Identity, OFFSET>(),
            GetLanguageCount: GetLanguageCount::<Identity, OFFSET>,
            GetLanguage: GetLanguage::<Identity, OFFSET>,
            GetMaxSpeedFactor: GetMaxSpeedFactor::<Identity, OFFSET>,
            IsUsingFastCache: IsUsingFastCache::<Identity, OFFSET>,
            AddLogParam: AddLogParam::<Identity, OFFSET>,
            SendLogParams: SendLogParams::<Identity, OFFSET>,
            CanSaveFileAs: CanSaveFileAs::<Identity, OFFSET>,
            CancelSaveFileAs: CancelSaveFileAs::<Identity, OFFSET>,
            GetURL: GetURL::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMReaderAdvanced4 as windows_core::Interface>::IID || iid == &<IWMReaderAdvanced as windows_core::Interface>::IID || iid == &<IWMReaderAdvanced2 as windows_core::Interface>::IID || iid == &<IWMReaderAdvanced3 as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWMReaderAdvanced5_Impl: Sized + IWMReaderAdvanced4_Impl {
    fn SetPlayerHook(&self, dwoutputnum: u32, phook: Option<&IWMPlayerHook>) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IWMReaderAdvanced5 {}
#[cfg(feature = "Win32_System_Com")]
impl IWMReaderAdvanced5_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWMReaderAdvanced5_Vtbl
    where
        Identity: IWMReaderAdvanced5_Impl,
    {
        unsafe extern "system" fn SetPlayerHook<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwoutputnum: u32, phook: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMReaderAdvanced5_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMReaderAdvanced5_Impl::SetPlayerHook(this, core::mem::transmute_copy(&dwoutputnum), windows_core::from_raw_borrowed(&phook)).into()
        }
        Self { base__: IWMReaderAdvanced4_Vtbl::new::<Identity, OFFSET>(), SetPlayerHook: SetPlayerHook::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMReaderAdvanced5 as windows_core::Interface>::IID || iid == &<IWMReaderAdvanced as windows_core::Interface>::IID || iid == &<IWMReaderAdvanced2 as windows_core::Interface>::IID || iid == &<IWMReaderAdvanced3 as windows_core::Interface>::IID || iid == &<IWMReaderAdvanced4 as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWMReaderAdvanced6_Impl: Sized + IWMReaderAdvanced5_Impl {
    fn SetProtectStreamSamples(&self, pbcertificate: *const u8, cbcertificate: u32, dwcertificatetype: u32, dwflags: u32, pbinitializationvector: *mut u8, pcbinitializationvector: *mut u32) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IWMReaderAdvanced6 {}
#[cfg(feature = "Win32_System_Com")]
impl IWMReaderAdvanced6_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWMReaderAdvanced6_Vtbl
    where
        Identity: IWMReaderAdvanced6_Impl,
    {
        unsafe extern "system" fn SetProtectStreamSamples<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbcertificate: *const u8, cbcertificate: u32, dwcertificatetype: u32, dwflags: u32, pbinitializationvector: *mut u8, pcbinitializationvector: *mut u32) -> windows_core::HRESULT
        where
            Identity: IWMReaderAdvanced6_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMReaderAdvanced6_Impl::SetProtectStreamSamples(this, core::mem::transmute_copy(&pbcertificate), core::mem::transmute_copy(&cbcertificate), core::mem::transmute_copy(&dwcertificatetype), core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&pbinitializationvector), core::mem::transmute_copy(&pcbinitializationvector)).into()
        }
        Self { base__: IWMReaderAdvanced5_Vtbl::new::<Identity, OFFSET>(), SetProtectStreamSamples: SetProtectStreamSamples::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMReaderAdvanced6 as windows_core::Interface>::IID || iid == &<IWMReaderAdvanced as windows_core::Interface>::IID || iid == &<IWMReaderAdvanced2 as windows_core::Interface>::IID || iid == &<IWMReaderAdvanced3 as windows_core::Interface>::IID || iid == &<IWMReaderAdvanced4 as windows_core::Interface>::IID || iid == &<IWMReaderAdvanced5 as windows_core::Interface>::IID
    }
}
pub trait IWMReaderAllocatorEx_Impl: Sized {
    fn AllocateForStreamEx(&self, wstreamnum: u16, cbbuffer: u32, ppbuffer: *mut Option<INSSBuffer>, dwflags: u32, cnssampletime: u64, cnssampleduration: u64, pvcontext: *const core::ffi::c_void) -> windows_core::Result<()>;
    fn AllocateForOutputEx(&self, dwoutputnum: u32, cbbuffer: u32, ppbuffer: *mut Option<INSSBuffer>, dwflags: u32, cnssampletime: u64, cnssampleduration: u64, pvcontext: *const core::ffi::c_void) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWMReaderAllocatorEx {}
impl IWMReaderAllocatorEx_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWMReaderAllocatorEx_Vtbl
    where
        Identity: IWMReaderAllocatorEx_Impl,
    {
        unsafe extern "system" fn AllocateForStreamEx<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, wstreamnum: u16, cbbuffer: u32, ppbuffer: *mut *mut core::ffi::c_void, dwflags: u32, cnssampletime: u64, cnssampleduration: u64, pvcontext: *const core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMReaderAllocatorEx_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMReaderAllocatorEx_Impl::AllocateForStreamEx(this, core::mem::transmute_copy(&wstreamnum), core::mem::transmute_copy(&cbbuffer), core::mem::transmute_copy(&ppbuffer), core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&cnssampletime), core::mem::transmute_copy(&cnssampleduration), core::mem::transmute_copy(&pvcontext)).into()
        }
        unsafe extern "system" fn AllocateForOutputEx<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwoutputnum: u32, cbbuffer: u32, ppbuffer: *mut *mut core::ffi::c_void, dwflags: u32, cnssampletime: u64, cnssampleduration: u64, pvcontext: *const core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMReaderAllocatorEx_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMReaderAllocatorEx_Impl::AllocateForOutputEx(this, core::mem::transmute_copy(&dwoutputnum), core::mem::transmute_copy(&cbbuffer), core::mem::transmute_copy(&ppbuffer), core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&cnssampletime), core::mem::transmute_copy(&cnssampleduration), core::mem::transmute_copy(&pvcontext)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AllocateForStreamEx: AllocateForStreamEx::<Identity, OFFSET>,
            AllocateForOutputEx: AllocateForOutputEx::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMReaderAllocatorEx as windows_core::Interface>::IID
    }
}
pub trait IWMReaderCallback_Impl: Sized + IWMStatusCallback_Impl {
    fn OnSample(&self, dwoutputnum: u32, cnssampletime: u64, cnssampleduration: u64, dwflags: u32, psample: Option<&INSSBuffer>, pvcontext: *const core::ffi::c_void) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWMReaderCallback {}
impl IWMReaderCallback_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWMReaderCallback_Vtbl
    where
        Identity: IWMReaderCallback_Impl,
    {
        unsafe extern "system" fn OnSample<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwoutputnum: u32, cnssampletime: u64, cnssampleduration: u64, dwflags: u32, psample: *mut core::ffi::c_void, pvcontext: *const core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMReaderCallback_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMReaderCallback_Impl::OnSample(this, core::mem::transmute_copy(&dwoutputnum), core::mem::transmute_copy(&cnssampletime), core::mem::transmute_copy(&cnssampleduration), core::mem::transmute_copy(&dwflags), windows_core::from_raw_borrowed(&psample), core::mem::transmute_copy(&pvcontext)).into()
        }
        Self { base__: IWMStatusCallback_Vtbl::new::<Identity, OFFSET>(), OnSample: OnSample::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMReaderCallback as windows_core::Interface>::IID || iid == &<IWMStatusCallback as windows_core::Interface>::IID
    }
}
pub trait IWMReaderCallbackAdvanced_Impl: Sized {
    fn OnStreamSample(&self, wstreamnum: u16, cnssampletime: u64, cnssampleduration: u64, dwflags: u32, psample: Option<&INSSBuffer>, pvcontext: *const core::ffi::c_void) -> windows_core::Result<()>;
    fn OnTime(&self, cnscurrenttime: u64, pvcontext: *const core::ffi::c_void) -> windows_core::Result<()>;
    fn OnStreamSelection(&self, wstreamcount: u16, pstreamnumbers: *const u16, pselections: *const WMT_STREAM_SELECTION, pvcontext: *const core::ffi::c_void) -> windows_core::Result<()>;
    fn OnOutputPropsChanged(&self, dwoutputnum: u32, pmediatype: *const WM_MEDIA_TYPE, pvcontext: *const core::ffi::c_void) -> windows_core::Result<()>;
    fn AllocateForStream(&self, wstreamnum: u16, cbbuffer: u32, ppbuffer: *mut Option<INSSBuffer>, pvcontext: *const core::ffi::c_void) -> windows_core::Result<()>;
    fn AllocateForOutput(&self, dwoutputnum: u32, cbbuffer: u32, ppbuffer: *mut Option<INSSBuffer>, pvcontext: *const core::ffi::c_void) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWMReaderCallbackAdvanced {}
impl IWMReaderCallbackAdvanced_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWMReaderCallbackAdvanced_Vtbl
    where
        Identity: IWMReaderCallbackAdvanced_Impl,
    {
        unsafe extern "system" fn OnStreamSample<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, wstreamnum: u16, cnssampletime: u64, cnssampleduration: u64, dwflags: u32, psample: *mut core::ffi::c_void, pvcontext: *const core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMReaderCallbackAdvanced_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMReaderCallbackAdvanced_Impl::OnStreamSample(this, core::mem::transmute_copy(&wstreamnum), core::mem::transmute_copy(&cnssampletime), core::mem::transmute_copy(&cnssampleduration), core::mem::transmute_copy(&dwflags), windows_core::from_raw_borrowed(&psample), core::mem::transmute_copy(&pvcontext)).into()
        }
        unsafe extern "system" fn OnTime<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, cnscurrenttime: u64, pvcontext: *const core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMReaderCallbackAdvanced_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMReaderCallbackAdvanced_Impl::OnTime(this, core::mem::transmute_copy(&cnscurrenttime), core::mem::transmute_copy(&pvcontext)).into()
        }
        unsafe extern "system" fn OnStreamSelection<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, wstreamcount: u16, pstreamnumbers: *const u16, pselections: *const WMT_STREAM_SELECTION, pvcontext: *const core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMReaderCallbackAdvanced_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMReaderCallbackAdvanced_Impl::OnStreamSelection(this, core::mem::transmute_copy(&wstreamcount), core::mem::transmute_copy(&pstreamnumbers), core::mem::transmute_copy(&pselections), core::mem::transmute_copy(&pvcontext)).into()
        }
        unsafe extern "system" fn OnOutputPropsChanged<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwoutputnum: u32, pmediatype: *const WM_MEDIA_TYPE, pvcontext: *const core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMReaderCallbackAdvanced_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMReaderCallbackAdvanced_Impl::OnOutputPropsChanged(this, core::mem::transmute_copy(&dwoutputnum), core::mem::transmute_copy(&pmediatype), core::mem::transmute_copy(&pvcontext)).into()
        }
        unsafe extern "system" fn AllocateForStream<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, wstreamnum: u16, cbbuffer: u32, ppbuffer: *mut *mut core::ffi::c_void, pvcontext: *const core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMReaderCallbackAdvanced_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMReaderCallbackAdvanced_Impl::AllocateForStream(this, core::mem::transmute_copy(&wstreamnum), core::mem::transmute_copy(&cbbuffer), core::mem::transmute_copy(&ppbuffer), core::mem::transmute_copy(&pvcontext)).into()
        }
        unsafe extern "system" fn AllocateForOutput<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwoutputnum: u32, cbbuffer: u32, ppbuffer: *mut *mut core::ffi::c_void, pvcontext: *const core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMReaderCallbackAdvanced_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMReaderCallbackAdvanced_Impl::AllocateForOutput(this, core::mem::transmute_copy(&dwoutputnum), core::mem::transmute_copy(&cbbuffer), core::mem::transmute_copy(&ppbuffer), core::mem::transmute_copy(&pvcontext)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnStreamSample: OnStreamSample::<Identity, OFFSET>,
            OnTime: OnTime::<Identity, OFFSET>,
            OnStreamSelection: OnStreamSelection::<Identity, OFFSET>,
            OnOutputPropsChanged: OnOutputPropsChanged::<Identity, OFFSET>,
            AllocateForStream: AllocateForStream::<Identity, OFFSET>,
            AllocateForOutput: AllocateForOutput::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMReaderCallbackAdvanced as windows_core::Interface>::IID
    }
}
pub trait IWMReaderNetworkConfig_Impl: Sized {
    fn GetBufferingTime(&self) -> windows_core::Result<u64>;
    fn SetBufferingTime(&self, cnsbufferingtime: u64) -> windows_core::Result<()>;
    fn GetUDPPortRanges(&self, prangearray: *mut WM_PORT_NUMBER_RANGE, pcranges: *mut u32) -> windows_core::Result<()>;
    fn SetUDPPortRanges(&self, prangearray: *const WM_PORT_NUMBER_RANGE, cranges: u32) -> windows_core::Result<()>;
    fn GetProxySettings(&self, pwszprotocol: &windows_core::PCWSTR) -> windows_core::Result<WMT_PROXY_SETTINGS>;
    fn SetProxySettings(&self, pwszprotocol: &windows_core::PCWSTR, proxysetting: WMT_PROXY_SETTINGS) -> windows_core::Result<()>;
    fn GetProxyHostName(&self, pwszprotocol: &windows_core::PCWSTR, pwszhostname: windows_core::PWSTR, pcchhostname: *mut u32) -> windows_core::Result<()>;
    fn SetProxyHostName(&self, pwszprotocol: &windows_core::PCWSTR, pwszhostname: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetProxyPort(&self, pwszprotocol: &windows_core::PCWSTR) -> windows_core::Result<u32>;
    fn SetProxyPort(&self, pwszprotocol: &windows_core::PCWSTR, dwport: u32) -> windows_core::Result<()>;
    fn GetProxyExceptionList(&self, pwszprotocol: &windows_core::PCWSTR, pwszexceptionlist: windows_core::PWSTR, pcchexceptionlist: *mut u32) -> windows_core::Result<()>;
    fn SetProxyExceptionList(&self, pwszprotocol: &windows_core::PCWSTR, pwszexceptionlist: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetProxyBypassForLocal(&self, pwszprotocol: &windows_core::PCWSTR) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn SetProxyBypassForLocal(&self, pwszprotocol: &windows_core::PCWSTR, fbypassforlocal: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn GetForceRerunAutoProxyDetection(&self) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn SetForceRerunAutoProxyDetection(&self, fforcererundetection: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn GetEnableMulticast(&self) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn SetEnableMulticast(&self, fenablemulticast: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn GetEnableHTTP(&self) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn SetEnableHTTP(&self, fenablehttp: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn GetEnableUDP(&self) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn SetEnableUDP(&self, fenableudp: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn GetEnableTCP(&self) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn SetEnableTCP(&self, fenabletcp: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn ResetProtocolRollover(&self) -> windows_core::Result<()>;
    fn GetConnectionBandwidth(&self) -> windows_core::Result<u32>;
    fn SetConnectionBandwidth(&self, dwconnectionbandwidth: u32) -> windows_core::Result<()>;
    fn GetNumProtocolsSupported(&self) -> windows_core::Result<u32>;
    fn GetSupportedProtocolName(&self, dwprotocolnum: u32, pwszprotocolname: windows_core::PWSTR, pcchprotocolname: *mut u32) -> windows_core::Result<()>;
    fn AddLoggingUrl(&self, pwszurl: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetLoggingUrl(&self, dwindex: u32, pwszurl: windows_core::PWSTR, pcchurl: *mut u32) -> windows_core::Result<()>;
    fn GetLoggingUrlCount(&self) -> windows_core::Result<u32>;
    fn ResetLoggingUrlList(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWMReaderNetworkConfig {}
impl IWMReaderNetworkConfig_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWMReaderNetworkConfig_Vtbl
    where
        Identity: IWMReaderNetworkConfig_Impl,
    {
        unsafe extern "system" fn GetBufferingTime<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcnsbufferingtime: *mut u64) -> windows_core::HRESULT
        where
            Identity: IWMReaderNetworkConfig_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMReaderNetworkConfig_Impl::GetBufferingTime(this) {
                Ok(ok__) => {
                    pcnsbufferingtime.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBufferingTime<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, cnsbufferingtime: u64) -> windows_core::HRESULT
        where
            Identity: IWMReaderNetworkConfig_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMReaderNetworkConfig_Impl::SetBufferingTime(this, core::mem::transmute_copy(&cnsbufferingtime)).into()
        }
        unsafe extern "system" fn GetUDPPortRanges<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, prangearray: *mut WM_PORT_NUMBER_RANGE, pcranges: *mut u32) -> windows_core::HRESULT
        where
            Identity: IWMReaderNetworkConfig_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMReaderNetworkConfig_Impl::GetUDPPortRanges(this, core::mem::transmute_copy(&prangearray), core::mem::transmute_copy(&pcranges)).into()
        }
        unsafe extern "system" fn SetUDPPortRanges<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, prangearray: *const WM_PORT_NUMBER_RANGE, cranges: u32) -> windows_core::HRESULT
        where
            Identity: IWMReaderNetworkConfig_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMReaderNetworkConfig_Impl::SetUDPPortRanges(this, core::mem::transmute_copy(&prangearray), core::mem::transmute_copy(&cranges)).into()
        }
        unsafe extern "system" fn GetProxySettings<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszprotocol: windows_core::PCWSTR, pproxysetting: *mut WMT_PROXY_SETTINGS) -> windows_core::HRESULT
        where
            Identity: IWMReaderNetworkConfig_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMReaderNetworkConfig_Impl::GetProxySettings(this, core::mem::transmute(&pwszprotocol)) {
                Ok(ok__) => {
                    pproxysetting.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProxySettings<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszprotocol: windows_core::PCWSTR, proxysetting: WMT_PROXY_SETTINGS) -> windows_core::HRESULT
        where
            Identity: IWMReaderNetworkConfig_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMReaderNetworkConfig_Impl::SetProxySettings(this, core::mem::transmute(&pwszprotocol), core::mem::transmute_copy(&proxysetting)).into()
        }
        unsafe extern "system" fn GetProxyHostName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszprotocol: windows_core::PCWSTR, pwszhostname: windows_core::PWSTR, pcchhostname: *mut u32) -> windows_core::HRESULT
        where
            Identity: IWMReaderNetworkConfig_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMReaderNetworkConfig_Impl::GetProxyHostName(this, core::mem::transmute(&pwszprotocol), core::mem::transmute_copy(&pwszhostname), core::mem::transmute_copy(&pcchhostname)).into()
        }
        unsafe extern "system" fn SetProxyHostName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszprotocol: windows_core::PCWSTR, pwszhostname: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IWMReaderNetworkConfig_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMReaderNetworkConfig_Impl::SetProxyHostName(this, core::mem::transmute(&pwszprotocol), core::mem::transmute(&pwszhostname)).into()
        }
        unsafe extern "system" fn GetProxyPort<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszprotocol: windows_core::PCWSTR, pdwport: *mut u32) -> windows_core::HRESULT
        where
            Identity: IWMReaderNetworkConfig_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMReaderNetworkConfig_Impl::GetProxyPort(this, core::mem::transmute(&pwszprotocol)) {
                Ok(ok__) => {
                    pdwport.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProxyPort<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszprotocol: windows_core::PCWSTR, dwport: u32) -> windows_core::HRESULT
        where
            Identity: IWMReaderNetworkConfig_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMReaderNetworkConfig_Impl::SetProxyPort(this, core::mem::transmute(&pwszprotocol), core::mem::transmute_copy(&dwport)).into()
        }
        unsafe extern "system" fn GetProxyExceptionList<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszprotocol: windows_core::PCWSTR, pwszexceptionlist: windows_core::PWSTR, pcchexceptionlist: *mut u32) -> windows_core::HRESULT
        where
            Identity: IWMReaderNetworkConfig_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMReaderNetworkConfig_Impl::GetProxyExceptionList(this, core::mem::transmute(&pwszprotocol), core::mem::transmute_copy(&pwszexceptionlist), core::mem::transmute_copy(&pcchexceptionlist)).into()
        }
        unsafe extern "system" fn SetProxyExceptionList<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszprotocol: windows_core::PCWSTR, pwszexceptionlist: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IWMReaderNetworkConfig_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMReaderNetworkConfig_Impl::SetProxyExceptionList(this, core::mem::transmute(&pwszprotocol), core::mem::transmute(&pwszexceptionlist)).into()
        }
        unsafe extern "system" fn GetProxyBypassForLocal<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszprotocol: windows_core::PCWSTR, pfbypassforlocal: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IWMReaderNetworkConfig_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMReaderNetworkConfig_Impl::GetProxyBypassForLocal(this, core::mem::transmute(&pwszprotocol)) {
                Ok(ok__) => {
                    pfbypassforlocal.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProxyBypassForLocal<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszprotocol: windows_core::PCWSTR, fbypassforlocal: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IWMReaderNetworkConfig_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMReaderNetworkConfig_Impl::SetProxyBypassForLocal(this, core::mem::transmute(&pwszprotocol), core::mem::transmute_copy(&fbypassforlocal)).into()
        }
        unsafe extern "system" fn GetForceRerunAutoProxyDetection<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfforcererundetection: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IWMReaderNetworkConfig_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMReaderNetworkConfig_Impl::GetForceRerunAutoProxyDetection(this) {
                Ok(ok__) => {
                    pfforcererundetection.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetForceRerunAutoProxyDetection<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fforcererundetection: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IWMReaderNetworkConfig_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMReaderNetworkConfig_Impl::SetForceRerunAutoProxyDetection(this, core::mem::transmute_copy(&fforcererundetection)).into()
        }
        unsafe extern "system" fn GetEnableMulticast<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfenablemulticast: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IWMReaderNetworkConfig_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMReaderNetworkConfig_Impl::GetEnableMulticast(this) {
                Ok(ok__) => {
                    pfenablemulticast.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEnableMulticast<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fenablemulticast: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IWMReaderNetworkConfig_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMReaderNetworkConfig_Impl::SetEnableMulticast(this, core::mem::transmute_copy(&fenablemulticast)).into()
        }
        unsafe extern "system" fn GetEnableHTTP<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfenablehttp: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IWMReaderNetworkConfig_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMReaderNetworkConfig_Impl::GetEnableHTTP(this) {
                Ok(ok__) => {
                    pfenablehttp.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEnableHTTP<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fenablehttp: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IWMReaderNetworkConfig_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMReaderNetworkConfig_Impl::SetEnableHTTP(this, core::mem::transmute_copy(&fenablehttp)).into()
        }
        unsafe extern "system" fn GetEnableUDP<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfenableudp: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IWMReaderNetworkConfig_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMReaderNetworkConfig_Impl::GetEnableUDP(this) {
                Ok(ok__) => {
                    pfenableudp.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEnableUDP<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fenableudp: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IWMReaderNetworkConfig_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMReaderNetworkConfig_Impl::SetEnableUDP(this, core::mem::transmute_copy(&fenableudp)).into()
        }
        unsafe extern "system" fn GetEnableTCP<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfenabletcp: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IWMReaderNetworkConfig_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMReaderNetworkConfig_Impl::GetEnableTCP(this) {
                Ok(ok__) => {
                    pfenabletcp.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEnableTCP<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fenabletcp: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IWMReaderNetworkConfig_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMReaderNetworkConfig_Impl::SetEnableTCP(this, core::mem::transmute_copy(&fenabletcp)).into()
        }
        unsafe extern "system" fn ResetProtocolRollover<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMReaderNetworkConfig_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMReaderNetworkConfig_Impl::ResetProtocolRollover(this).into()
        }
        unsafe extern "system" fn GetConnectionBandwidth<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwconnectionbandwidth: *mut u32) -> windows_core::HRESULT
        where
            Identity: IWMReaderNetworkConfig_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMReaderNetworkConfig_Impl::GetConnectionBandwidth(this) {
                Ok(ok__) => {
                    pdwconnectionbandwidth.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetConnectionBandwidth<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwconnectionbandwidth: u32) -> windows_core::HRESULT
        where
            Identity: IWMReaderNetworkConfig_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMReaderNetworkConfig_Impl::SetConnectionBandwidth(this, core::mem::transmute_copy(&dwconnectionbandwidth)).into()
        }
        unsafe extern "system" fn GetNumProtocolsSupported<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcprotocols: *mut u32) -> windows_core::HRESULT
        where
            Identity: IWMReaderNetworkConfig_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMReaderNetworkConfig_Impl::GetNumProtocolsSupported(this) {
                Ok(ok__) => {
                    pcprotocols.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSupportedProtocolName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwprotocolnum: u32, pwszprotocolname: windows_core::PWSTR, pcchprotocolname: *mut u32) -> windows_core::HRESULT
        where
            Identity: IWMReaderNetworkConfig_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMReaderNetworkConfig_Impl::GetSupportedProtocolName(this, core::mem::transmute_copy(&dwprotocolnum), core::mem::transmute_copy(&pwszprotocolname), core::mem::transmute_copy(&pcchprotocolname)).into()
        }
        unsafe extern "system" fn AddLoggingUrl<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszurl: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IWMReaderNetworkConfig_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMReaderNetworkConfig_Impl::AddLoggingUrl(this, core::mem::transmute(&pwszurl)).into()
        }
        unsafe extern "system" fn GetLoggingUrl<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwindex: u32, pwszurl: windows_core::PWSTR, pcchurl: *mut u32) -> windows_core::HRESULT
        where
            Identity: IWMReaderNetworkConfig_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMReaderNetworkConfig_Impl::GetLoggingUrl(this, core::mem::transmute_copy(&dwindex), core::mem::transmute_copy(&pwszurl), core::mem::transmute_copy(&pcchurl)).into()
        }
        unsafe extern "system" fn GetLoggingUrlCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwurlcount: *mut u32) -> windows_core::HRESULT
        where
            Identity: IWMReaderNetworkConfig_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMReaderNetworkConfig_Impl::GetLoggingUrlCount(this) {
                Ok(ok__) => {
                    pdwurlcount.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ResetLoggingUrlList<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMReaderNetworkConfig_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMReaderNetworkConfig_Impl::ResetLoggingUrlList(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetBufferingTime: GetBufferingTime::<Identity, OFFSET>,
            SetBufferingTime: SetBufferingTime::<Identity, OFFSET>,
            GetUDPPortRanges: GetUDPPortRanges::<Identity, OFFSET>,
            SetUDPPortRanges: SetUDPPortRanges::<Identity, OFFSET>,
            GetProxySettings: GetProxySettings::<Identity, OFFSET>,
            SetProxySettings: SetProxySettings::<Identity, OFFSET>,
            GetProxyHostName: GetProxyHostName::<Identity, OFFSET>,
            SetProxyHostName: SetProxyHostName::<Identity, OFFSET>,
            GetProxyPort: GetProxyPort::<Identity, OFFSET>,
            SetProxyPort: SetProxyPort::<Identity, OFFSET>,
            GetProxyExceptionList: GetProxyExceptionList::<Identity, OFFSET>,
            SetProxyExceptionList: SetProxyExceptionList::<Identity, OFFSET>,
            GetProxyBypassForLocal: GetProxyBypassForLocal::<Identity, OFFSET>,
            SetProxyBypassForLocal: SetProxyBypassForLocal::<Identity, OFFSET>,
            GetForceRerunAutoProxyDetection: GetForceRerunAutoProxyDetection::<Identity, OFFSET>,
            SetForceRerunAutoProxyDetection: SetForceRerunAutoProxyDetection::<Identity, OFFSET>,
            GetEnableMulticast: GetEnableMulticast::<Identity, OFFSET>,
            SetEnableMulticast: SetEnableMulticast::<Identity, OFFSET>,
            GetEnableHTTP: GetEnableHTTP::<Identity, OFFSET>,
            SetEnableHTTP: SetEnableHTTP::<Identity, OFFSET>,
            GetEnableUDP: GetEnableUDP::<Identity, OFFSET>,
            SetEnableUDP: SetEnableUDP::<Identity, OFFSET>,
            GetEnableTCP: GetEnableTCP::<Identity, OFFSET>,
            SetEnableTCP: SetEnableTCP::<Identity, OFFSET>,
            ResetProtocolRollover: ResetProtocolRollover::<Identity, OFFSET>,
            GetConnectionBandwidth: GetConnectionBandwidth::<Identity, OFFSET>,
            SetConnectionBandwidth: SetConnectionBandwidth::<Identity, OFFSET>,
            GetNumProtocolsSupported: GetNumProtocolsSupported::<Identity, OFFSET>,
            GetSupportedProtocolName: GetSupportedProtocolName::<Identity, OFFSET>,
            AddLoggingUrl: AddLoggingUrl::<Identity, OFFSET>,
            GetLoggingUrl: GetLoggingUrl::<Identity, OFFSET>,
            GetLoggingUrlCount: GetLoggingUrlCount::<Identity, OFFSET>,
            ResetLoggingUrlList: ResetLoggingUrlList::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMReaderNetworkConfig as windows_core::Interface>::IID
    }
}
pub trait IWMReaderNetworkConfig2_Impl: Sized + IWMReaderNetworkConfig_Impl {
    fn GetEnableContentCaching(&self) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn SetEnableContentCaching(&self, fenablecontentcaching: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn GetEnableFastCache(&self) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn SetEnableFastCache(&self, fenablefastcache: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn GetAcceleratedStreamingDuration(&self) -> windows_core::Result<u64>;
    fn SetAcceleratedStreamingDuration(&self, cnsaccelduration: u64) -> windows_core::Result<()>;
    fn GetAutoReconnectLimit(&self) -> windows_core::Result<u32>;
    fn SetAutoReconnectLimit(&self, dwautoreconnectlimit: u32) -> windows_core::Result<()>;
    fn GetEnableResends(&self) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn SetEnableResends(&self, fenableresends: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn GetEnableThinning(&self) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn SetEnableThinning(&self, fenablethinning: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn GetMaxNetPacketSize(&self) -> windows_core::Result<u32>;
}
impl windows_core::RuntimeName for IWMReaderNetworkConfig2 {}
impl IWMReaderNetworkConfig2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWMReaderNetworkConfig2_Vtbl
    where
        Identity: IWMReaderNetworkConfig2_Impl,
    {
        unsafe extern "system" fn GetEnableContentCaching<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfenablecontentcaching: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IWMReaderNetworkConfig2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMReaderNetworkConfig2_Impl::GetEnableContentCaching(this) {
                Ok(ok__) => {
                    pfenablecontentcaching.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEnableContentCaching<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fenablecontentcaching: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IWMReaderNetworkConfig2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMReaderNetworkConfig2_Impl::SetEnableContentCaching(this, core::mem::transmute_copy(&fenablecontentcaching)).into()
        }
        unsafe extern "system" fn GetEnableFastCache<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfenablefastcache: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IWMReaderNetworkConfig2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMReaderNetworkConfig2_Impl::GetEnableFastCache(this) {
                Ok(ok__) => {
                    pfenablefastcache.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEnableFastCache<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fenablefastcache: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IWMReaderNetworkConfig2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMReaderNetworkConfig2_Impl::SetEnableFastCache(this, core::mem::transmute_copy(&fenablefastcache)).into()
        }
        unsafe extern "system" fn GetAcceleratedStreamingDuration<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcnsaccelduration: *mut u64) -> windows_core::HRESULT
        where
            Identity: IWMReaderNetworkConfig2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMReaderNetworkConfig2_Impl::GetAcceleratedStreamingDuration(this) {
                Ok(ok__) => {
                    pcnsaccelduration.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAcceleratedStreamingDuration<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, cnsaccelduration: u64) -> windows_core::HRESULT
        where
            Identity: IWMReaderNetworkConfig2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMReaderNetworkConfig2_Impl::SetAcceleratedStreamingDuration(this, core::mem::transmute_copy(&cnsaccelduration)).into()
        }
        unsafe extern "system" fn GetAutoReconnectLimit<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwautoreconnectlimit: *mut u32) -> windows_core::HRESULT
        where
            Identity: IWMReaderNetworkConfig2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMReaderNetworkConfig2_Impl::GetAutoReconnectLimit(this) {
                Ok(ok__) => {
                    pdwautoreconnectlimit.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAutoReconnectLimit<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwautoreconnectlimit: u32) -> windows_core::HRESULT
        where
            Identity: IWMReaderNetworkConfig2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMReaderNetworkConfig2_Impl::SetAutoReconnectLimit(this, core::mem::transmute_copy(&dwautoreconnectlimit)).into()
        }
        unsafe extern "system" fn GetEnableResends<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfenableresends: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IWMReaderNetworkConfig2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMReaderNetworkConfig2_Impl::GetEnableResends(this) {
                Ok(ok__) => {
                    pfenableresends.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEnableResends<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fenableresends: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IWMReaderNetworkConfig2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMReaderNetworkConfig2_Impl::SetEnableResends(this, core::mem::transmute_copy(&fenableresends)).into()
        }
        unsafe extern "system" fn GetEnableThinning<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfenablethinning: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IWMReaderNetworkConfig2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMReaderNetworkConfig2_Impl::GetEnableThinning(this) {
                Ok(ok__) => {
                    pfenablethinning.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEnableThinning<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fenablethinning: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IWMReaderNetworkConfig2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMReaderNetworkConfig2_Impl::SetEnableThinning(this, core::mem::transmute_copy(&fenablethinning)).into()
        }
        unsafe extern "system" fn GetMaxNetPacketSize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwmaxnetpacketsize: *mut u32) -> windows_core::HRESULT
        where
            Identity: IWMReaderNetworkConfig2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMReaderNetworkConfig2_Impl::GetMaxNetPacketSize(this) {
                Ok(ok__) => {
                    pdwmaxnetpacketsize.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: IWMReaderNetworkConfig_Vtbl::new::<Identity, OFFSET>(),
            GetEnableContentCaching: GetEnableContentCaching::<Identity, OFFSET>,
            SetEnableContentCaching: SetEnableContentCaching::<Identity, OFFSET>,
            GetEnableFastCache: GetEnableFastCache::<Identity, OFFSET>,
            SetEnableFastCache: SetEnableFastCache::<Identity, OFFSET>,
            GetAcceleratedStreamingDuration: GetAcceleratedStreamingDuration::<Identity, OFFSET>,
            SetAcceleratedStreamingDuration: SetAcceleratedStreamingDuration::<Identity, OFFSET>,
            GetAutoReconnectLimit: GetAutoReconnectLimit::<Identity, OFFSET>,
            SetAutoReconnectLimit: SetAutoReconnectLimit::<Identity, OFFSET>,
            GetEnableResends: GetEnableResends::<Identity, OFFSET>,
            SetEnableResends: SetEnableResends::<Identity, OFFSET>,
            GetEnableThinning: GetEnableThinning::<Identity, OFFSET>,
            SetEnableThinning: SetEnableThinning::<Identity, OFFSET>,
            GetMaxNetPacketSize: GetMaxNetPacketSize::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMReaderNetworkConfig2 as windows_core::Interface>::IID || iid == &<IWMReaderNetworkConfig as windows_core::Interface>::IID
    }
}
pub trait IWMReaderPlaylistBurn_Impl: Sized {
    fn InitPlaylistBurn(&self, cfiles: u32, ppwszfilenames: *const windows_core::PCWSTR, pcallback: Option<&IWMStatusCallback>, pvcontext: *const core::ffi::c_void) -> windows_core::Result<()>;
    fn GetInitResults(&self, cfiles: u32) -> windows_core::Result<windows_core::HRESULT>;
    fn Cancel(&self) -> windows_core::Result<()>;
    fn EndPlaylistBurn(&self, hrburnresult: windows_core::HRESULT) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWMReaderPlaylistBurn {}
impl IWMReaderPlaylistBurn_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWMReaderPlaylistBurn_Vtbl
    where
        Identity: IWMReaderPlaylistBurn_Impl,
    {
        unsafe extern "system" fn InitPlaylistBurn<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, cfiles: u32, ppwszfilenames: *const windows_core::PCWSTR, pcallback: *mut core::ffi::c_void, pvcontext: *const core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMReaderPlaylistBurn_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMReaderPlaylistBurn_Impl::InitPlaylistBurn(this, core::mem::transmute_copy(&cfiles), core::mem::transmute_copy(&ppwszfilenames), windows_core::from_raw_borrowed(&pcallback), core::mem::transmute_copy(&pvcontext)).into()
        }
        unsafe extern "system" fn GetInitResults<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, cfiles: u32, phrstati: *mut windows_core::HRESULT) -> windows_core::HRESULT
        where
            Identity: IWMReaderPlaylistBurn_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMReaderPlaylistBurn_Impl::GetInitResults(this, core::mem::transmute_copy(&cfiles)) {
                Ok(ok__) => {
                    phrstati.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Cancel<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMReaderPlaylistBurn_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMReaderPlaylistBurn_Impl::Cancel(this).into()
        }
        unsafe extern "system" fn EndPlaylistBurn<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hrburnresult: windows_core::HRESULT) -> windows_core::HRESULT
        where
            Identity: IWMReaderPlaylistBurn_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMReaderPlaylistBurn_Impl::EndPlaylistBurn(this, core::mem::transmute_copy(&hrburnresult)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            InitPlaylistBurn: InitPlaylistBurn::<Identity, OFFSET>,
            GetInitResults: GetInitResults::<Identity, OFFSET>,
            Cancel: Cancel::<Identity, OFFSET>,
            EndPlaylistBurn: EndPlaylistBurn::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMReaderPlaylistBurn as windows_core::Interface>::IID
    }
}
pub trait IWMReaderStreamClock_Impl: Sized {
    fn GetTime(&self, pcnsnow: *const u64) -> windows_core::Result<()>;
    fn SetTimer(&self, cnswhen: u64, pvparam: *const core::ffi::c_void) -> windows_core::Result<u32>;
    fn KillTimer(&self, dwtimerid: u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWMReaderStreamClock {}
impl IWMReaderStreamClock_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWMReaderStreamClock_Vtbl
    where
        Identity: IWMReaderStreamClock_Impl,
    {
        unsafe extern "system" fn GetTime<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcnsnow: *const u64) -> windows_core::HRESULT
        where
            Identity: IWMReaderStreamClock_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMReaderStreamClock_Impl::GetTime(this, core::mem::transmute_copy(&pcnsnow)).into()
        }
        unsafe extern "system" fn SetTimer<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, cnswhen: u64, pvparam: *const core::ffi::c_void, pdwtimerid: *mut u32) -> windows_core::HRESULT
        where
            Identity: IWMReaderStreamClock_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMReaderStreamClock_Impl::SetTimer(this, core::mem::transmute_copy(&cnswhen), core::mem::transmute_copy(&pvparam)) {
                Ok(ok__) => {
                    pdwtimerid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn KillTimer<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwtimerid: u32) -> windows_core::HRESULT
        where
            Identity: IWMReaderStreamClock_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMReaderStreamClock_Impl::KillTimer(this, core::mem::transmute_copy(&dwtimerid)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetTime: GetTime::<Identity, OFFSET>,
            SetTimer: SetTimer::<Identity, OFFSET>,
            KillTimer: KillTimer::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMReaderStreamClock as windows_core::Interface>::IID
    }
}
pub trait IWMReaderTimecode_Impl: Sized {
    fn GetTimecodeRangeCount(&self, wstreamnum: u16) -> windows_core::Result<u16>;
    fn GetTimecodeRangeBounds(&self, wstreamnum: u16, wrangenum: u16, pstarttimecode: *mut u32, pendtimecode: *mut u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWMReaderTimecode {}
impl IWMReaderTimecode_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWMReaderTimecode_Vtbl
    where
        Identity: IWMReaderTimecode_Impl,
    {
        unsafe extern "system" fn GetTimecodeRangeCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, wstreamnum: u16, pwrangecount: *mut u16) -> windows_core::HRESULT
        where
            Identity: IWMReaderTimecode_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMReaderTimecode_Impl::GetTimecodeRangeCount(this, core::mem::transmute_copy(&wstreamnum)) {
                Ok(ok__) => {
                    pwrangecount.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTimecodeRangeBounds<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, wstreamnum: u16, wrangenum: u16, pstarttimecode: *mut u32, pendtimecode: *mut u32) -> windows_core::HRESULT
        where
            Identity: IWMReaderTimecode_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMReaderTimecode_Impl::GetTimecodeRangeBounds(this, core::mem::transmute_copy(&wstreamnum), core::mem::transmute_copy(&wrangenum), core::mem::transmute_copy(&pstarttimecode), core::mem::transmute_copy(&pendtimecode)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetTimecodeRangeCount: GetTimecodeRangeCount::<Identity, OFFSET>,
            GetTimecodeRangeBounds: GetTimecodeRangeBounds::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMReaderTimecode as windows_core::Interface>::IID
    }
}
pub trait IWMReaderTypeNegotiation_Impl: Sized {
    fn TryOutputProps(&self, dwoutputnum: u32, poutput: Option<&IWMOutputMediaProps>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWMReaderTypeNegotiation {}
impl IWMReaderTypeNegotiation_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWMReaderTypeNegotiation_Vtbl
    where
        Identity: IWMReaderTypeNegotiation_Impl,
    {
        unsafe extern "system" fn TryOutputProps<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwoutputnum: u32, poutput: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMReaderTypeNegotiation_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMReaderTypeNegotiation_Impl::TryOutputProps(this, core::mem::transmute_copy(&dwoutputnum), windows_core::from_raw_borrowed(&poutput)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), TryOutputProps: TryOutputProps::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMReaderTypeNegotiation as windows_core::Interface>::IID
    }
}
pub trait IWMRegisterCallback_Impl: Sized {
    fn Advise(&self, pcallback: Option<&IWMStatusCallback>, pvcontext: *const core::ffi::c_void) -> windows_core::Result<()>;
    fn Unadvise(&self, pcallback: Option<&IWMStatusCallback>, pvcontext: *const core::ffi::c_void) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWMRegisterCallback {}
impl IWMRegisterCallback_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWMRegisterCallback_Vtbl
    where
        Identity: IWMRegisterCallback_Impl,
    {
        unsafe extern "system" fn Advise<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcallback: *mut core::ffi::c_void, pvcontext: *const core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMRegisterCallback_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMRegisterCallback_Impl::Advise(this, windows_core::from_raw_borrowed(&pcallback), core::mem::transmute_copy(&pvcontext)).into()
        }
        unsafe extern "system" fn Unadvise<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcallback: *mut core::ffi::c_void, pvcontext: *const core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMRegisterCallback_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMRegisterCallback_Impl::Unadvise(this, windows_core::from_raw_borrowed(&pcallback), core::mem::transmute_copy(&pvcontext)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Advise: Advise::<Identity, OFFSET>, Unadvise: Unadvise::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMRegisterCallback as windows_core::Interface>::IID
    }
}
pub trait IWMRegisteredDevice_Impl: Sized {
    fn GetDeviceSerialNumber(&self) -> windows_core::Result<DRM_VAL16>;
    fn GetDeviceCertificate(&self) -> windows_core::Result<INSSBuffer>;
    fn GetDeviceType(&self) -> windows_core::Result<u32>;
    fn GetAttributeCount(&self) -> windows_core::Result<u32>;
    fn GetAttributeByIndex(&self, dwindex: u32, pbstrname: *mut windows_core::BSTR, pbstrvalue: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn GetAttributeByName(&self, bstrname: &windows_core::BSTR) -> windows_core::Result<windows_core::BSTR>;
    fn SetAttributeByName(&self, bstrname: &windows_core::BSTR, bstrvalue: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Approve(&self, fapprove: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn IsValid(&self) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn IsApproved(&self) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn IsWmdrmCompliant(&self) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn IsOpened(&self) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn Open(&self) -> windows_core::Result<()>;
    fn Close(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWMRegisteredDevice {}
impl IWMRegisteredDevice_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWMRegisteredDevice_Vtbl
    where
        Identity: IWMRegisteredDevice_Impl,
    {
        unsafe extern "system" fn GetDeviceSerialNumber<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pserialnumber: *mut DRM_VAL16) -> windows_core::HRESULT
        where
            Identity: IWMRegisteredDevice_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMRegisteredDevice_Impl::GetDeviceSerialNumber(this) {
                Ok(ok__) => {
                    pserialnumber.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeviceCertificate<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcertificate: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMRegisteredDevice_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMRegisteredDevice_Impl::GetDeviceCertificate(this) {
                Ok(ok__) => {
                    ppcertificate.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeviceType<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwtype: *mut u32) -> windows_core::HRESULT
        where
            Identity: IWMRegisteredDevice_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMRegisteredDevice_Impl::GetDeviceType(this) {
                Ok(ok__) => {
                    pdwtype.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAttributeCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcattributes: *mut u32) -> windows_core::HRESULT
        where
            Identity: IWMRegisteredDevice_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMRegisteredDevice_Impl::GetAttributeCount(this) {
                Ok(ok__) => {
                    pcattributes.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAttributeByIndex<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwindex: u32, pbstrname: *mut core::mem::MaybeUninit<windows_core::BSTR>, pbstrvalue: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IWMRegisteredDevice_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMRegisteredDevice_Impl::GetAttributeByIndex(this, core::mem::transmute_copy(&dwindex), core::mem::transmute_copy(&pbstrname), core::mem::transmute_copy(&pbstrvalue)).into()
        }
        unsafe extern "system" fn GetAttributeByName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrname: core::mem::MaybeUninit<windows_core::BSTR>, pbstrvalue: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IWMRegisteredDevice_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMRegisteredDevice_Impl::GetAttributeByName(this, core::mem::transmute(&bstrname)) {
                Ok(ok__) => {
                    pbstrvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAttributeByName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrname: core::mem::MaybeUninit<windows_core::BSTR>, bstrvalue: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IWMRegisteredDevice_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMRegisteredDevice_Impl::SetAttributeByName(this, core::mem::transmute(&bstrname), core::mem::transmute(&bstrvalue)).into()
        }
        unsafe extern "system" fn Approve<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fapprove: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IWMRegisteredDevice_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMRegisteredDevice_Impl::Approve(this, core::mem::transmute_copy(&fapprove)).into()
        }
        unsafe extern "system" fn IsValid<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfvalid: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IWMRegisteredDevice_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMRegisteredDevice_Impl::IsValid(this) {
                Ok(ok__) => {
                    pfvalid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsApproved<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfapproved: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IWMRegisteredDevice_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMRegisteredDevice_Impl::IsApproved(this) {
                Ok(ok__) => {
                    pfapproved.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsWmdrmCompliant<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfcompliant: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IWMRegisteredDevice_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMRegisteredDevice_Impl::IsWmdrmCompliant(this) {
                Ok(ok__) => {
                    pfcompliant.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsOpened<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfopened: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IWMRegisteredDevice_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMRegisteredDevice_Impl::IsOpened(this) {
                Ok(ok__) => {
                    pfopened.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Open<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMRegisteredDevice_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMRegisteredDevice_Impl::Open(this).into()
        }
        unsafe extern "system" fn Close<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMRegisteredDevice_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMRegisteredDevice_Impl::Close(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetDeviceSerialNumber: GetDeviceSerialNumber::<Identity, OFFSET>,
            GetDeviceCertificate: GetDeviceCertificate::<Identity, OFFSET>,
            GetDeviceType: GetDeviceType::<Identity, OFFSET>,
            GetAttributeCount: GetAttributeCount::<Identity, OFFSET>,
            GetAttributeByIndex: GetAttributeByIndex::<Identity, OFFSET>,
            GetAttributeByName: GetAttributeByName::<Identity, OFFSET>,
            SetAttributeByName: SetAttributeByName::<Identity, OFFSET>,
            Approve: Approve::<Identity, OFFSET>,
            IsValid: IsValid::<Identity, OFFSET>,
            IsApproved: IsApproved::<Identity, OFFSET>,
            IsWmdrmCompliant: IsWmdrmCompliant::<Identity, OFFSET>,
            IsOpened: IsOpened::<Identity, OFFSET>,
            Open: Open::<Identity, OFFSET>,
            Close: Close::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMRegisteredDevice as windows_core::Interface>::IID
    }
}
pub trait IWMSBufferAllocator_Impl: Sized {
    fn AllocateBuffer(&self, dwmaxbuffersize: u32) -> windows_core::Result<INSSBuffer>;
    fn AllocatePageSizeBuffer(&self, dwmaxbuffersize: u32) -> windows_core::Result<INSSBuffer>;
}
impl windows_core::RuntimeName for IWMSBufferAllocator {}
impl IWMSBufferAllocator_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWMSBufferAllocator_Vtbl
    where
        Identity: IWMSBufferAllocator_Impl,
    {
        unsafe extern "system" fn AllocateBuffer<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwmaxbuffersize: u32, ppbuffer: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMSBufferAllocator_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMSBufferAllocator_Impl::AllocateBuffer(this, core::mem::transmute_copy(&dwmaxbuffersize)) {
                Ok(ok__) => {
                    ppbuffer.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AllocatePageSizeBuffer<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwmaxbuffersize: u32, ppbuffer: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMSBufferAllocator_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMSBufferAllocator_Impl::AllocatePageSizeBuffer(this, core::mem::transmute_copy(&dwmaxbuffersize)) {
                Ok(ok__) => {
                    ppbuffer.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AllocateBuffer: AllocateBuffer::<Identity, OFFSET>,
            AllocatePageSizeBuffer: AllocatePageSizeBuffer::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMSBufferAllocator as windows_core::Interface>::IID
    }
}
pub trait IWMSInternalAdminNetSource_Impl: Sized {
    fn Initialize(&self, psharednamespace: Option<&windows_core::IUnknown>, pnamespacenode: Option<&windows_core::IUnknown>, pnetsourcecreator: Option<&INSNetSourceCreator>, fembeddedinserver: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn GetNetSourceCreator(&self) -> windows_core::Result<INSNetSourceCreator>;
    fn SetCredentials(&self, bstrrealm: &windows_core::BSTR, bstrname: &windows_core::BSTR, bstrpassword: &windows_core::BSTR, fpersist: super::super::Foundation::BOOL, fconfirmedgood: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn GetCredentials(&self, bstrrealm: &windows_core::BSTR, pbstrname: *mut windows_core::BSTR, pbstrpassword: *mut windows_core::BSTR, pfconfirmedgood: *mut super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn DeleteCredentials(&self, bstrrealm: &windows_core::BSTR) -> windows_core::Result<()>;
    fn GetCredentialFlags(&self) -> windows_core::Result<u32>;
    fn SetCredentialFlags(&self, dwflags: u32) -> windows_core::Result<()>;
    fn FindProxyForURL(&self, bstrprotocol: &windows_core::BSTR, bstrhost: &windows_core::BSTR, pfproxyenabled: *mut super::super::Foundation::BOOL, pbstrproxyserver: *mut windows_core::BSTR, pdwproxyport: *mut u32, pdwproxycontext: *mut u32) -> windows_core::Result<()>;
    fn RegisterProxyFailure(&self, hrparam: windows_core::HRESULT, dwproxycontext: u32) -> windows_core::Result<()>;
    fn ShutdownProxyContext(&self, dwproxycontext: u32) -> windows_core::Result<()>;
    fn IsUsingIE(&self, dwproxycontext: u32) -> windows_core::Result<super::super::Foundation::BOOL>;
}
impl windows_core::RuntimeName for IWMSInternalAdminNetSource {}
impl IWMSInternalAdminNetSource_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWMSInternalAdminNetSource_Vtbl
    where
        Identity: IWMSInternalAdminNetSource_Impl,
    {
        unsafe extern "system" fn Initialize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, psharednamespace: *mut core::ffi::c_void, pnamespacenode: *mut core::ffi::c_void, pnetsourcecreator: *mut core::ffi::c_void, fembeddedinserver: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IWMSInternalAdminNetSource_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMSInternalAdminNetSource_Impl::Initialize(this, windows_core::from_raw_borrowed(&psharednamespace), windows_core::from_raw_borrowed(&pnamespacenode), windows_core::from_raw_borrowed(&pnetsourcecreator), core::mem::transmute_copy(&fembeddedinserver)).into()
        }
        unsafe extern "system" fn GetNetSourceCreator<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppnetsourcecreator: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMSInternalAdminNetSource_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMSInternalAdminNetSource_Impl::GetNetSourceCreator(this) {
                Ok(ok__) => {
                    ppnetsourcecreator.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCredentials<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrrealm: core::mem::MaybeUninit<windows_core::BSTR>, bstrname: core::mem::MaybeUninit<windows_core::BSTR>, bstrpassword: core::mem::MaybeUninit<windows_core::BSTR>, fpersist: super::super::Foundation::BOOL, fconfirmedgood: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IWMSInternalAdminNetSource_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMSInternalAdminNetSource_Impl::SetCredentials(this, core::mem::transmute(&bstrrealm), core::mem::transmute(&bstrname), core::mem::transmute(&bstrpassword), core::mem::transmute_copy(&fpersist), core::mem::transmute_copy(&fconfirmedgood)).into()
        }
        unsafe extern "system" fn GetCredentials<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrrealm: core::mem::MaybeUninit<windows_core::BSTR>, pbstrname: *mut core::mem::MaybeUninit<windows_core::BSTR>, pbstrpassword: *mut core::mem::MaybeUninit<windows_core::BSTR>, pfconfirmedgood: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IWMSInternalAdminNetSource_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMSInternalAdminNetSource_Impl::GetCredentials(this, core::mem::transmute(&bstrrealm), core::mem::transmute_copy(&pbstrname), core::mem::transmute_copy(&pbstrpassword), core::mem::transmute_copy(&pfconfirmedgood)).into()
        }
        unsafe extern "system" fn DeleteCredentials<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrrealm: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IWMSInternalAdminNetSource_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMSInternalAdminNetSource_Impl::DeleteCredentials(this, core::mem::transmute(&bstrrealm)).into()
        }
        unsafe extern "system" fn GetCredentialFlags<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpdwflags: *mut u32) -> windows_core::HRESULT
        where
            Identity: IWMSInternalAdminNetSource_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMSInternalAdminNetSource_Impl::GetCredentialFlags(this) {
                Ok(ok__) => {
                    lpdwflags.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCredentialFlags<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: u32) -> windows_core::HRESULT
        where
            Identity: IWMSInternalAdminNetSource_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMSInternalAdminNetSource_Impl::SetCredentialFlags(this, core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn FindProxyForURL<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrprotocol: core::mem::MaybeUninit<windows_core::BSTR>, bstrhost: core::mem::MaybeUninit<windows_core::BSTR>, pfproxyenabled: *mut super::super::Foundation::BOOL, pbstrproxyserver: *mut core::mem::MaybeUninit<windows_core::BSTR>, pdwproxyport: *mut u32, pdwproxycontext: *mut u32) -> windows_core::HRESULT
        where
            Identity: IWMSInternalAdminNetSource_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMSInternalAdminNetSource_Impl::FindProxyForURL(this, core::mem::transmute(&bstrprotocol), core::mem::transmute(&bstrhost), core::mem::transmute_copy(&pfproxyenabled), core::mem::transmute_copy(&pbstrproxyserver), core::mem::transmute_copy(&pdwproxyport), core::mem::transmute_copy(&pdwproxycontext)).into()
        }
        unsafe extern "system" fn RegisterProxyFailure<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hrparam: windows_core::HRESULT, dwproxycontext: u32) -> windows_core::HRESULT
        where
            Identity: IWMSInternalAdminNetSource_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMSInternalAdminNetSource_Impl::RegisterProxyFailure(this, core::mem::transmute_copy(&hrparam), core::mem::transmute_copy(&dwproxycontext)).into()
        }
        unsafe extern "system" fn ShutdownProxyContext<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwproxycontext: u32) -> windows_core::HRESULT
        where
            Identity: IWMSInternalAdminNetSource_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMSInternalAdminNetSource_Impl::ShutdownProxyContext(this, core::mem::transmute_copy(&dwproxycontext)).into()
        }
        unsafe extern "system" fn IsUsingIE<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwproxycontext: u32, pfisusingie: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IWMSInternalAdminNetSource_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMSInternalAdminNetSource_Impl::IsUsingIE(this, core::mem::transmute_copy(&dwproxycontext)) {
                Ok(ok__) => {
                    pfisusingie.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Initialize: Initialize::<Identity, OFFSET>,
            GetNetSourceCreator: GetNetSourceCreator::<Identity, OFFSET>,
            SetCredentials: SetCredentials::<Identity, OFFSET>,
            GetCredentials: GetCredentials::<Identity, OFFSET>,
            DeleteCredentials: DeleteCredentials::<Identity, OFFSET>,
            GetCredentialFlags: GetCredentialFlags::<Identity, OFFSET>,
            SetCredentialFlags: SetCredentialFlags::<Identity, OFFSET>,
            FindProxyForURL: FindProxyForURL::<Identity, OFFSET>,
            RegisterProxyFailure: RegisterProxyFailure::<Identity, OFFSET>,
            ShutdownProxyContext: ShutdownProxyContext::<Identity, OFFSET>,
            IsUsingIE: IsUsingIE::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMSInternalAdminNetSource as windows_core::Interface>::IID
    }
}
pub trait IWMSInternalAdminNetSource2_Impl: Sized {
    fn SetCredentialsEx(&self, bstrrealm: &windows_core::BSTR, bstrurl: &windows_core::BSTR, fproxy: super::super::Foundation::BOOL, bstrname: &windows_core::BSTR, bstrpassword: &windows_core::BSTR, fpersist: super::super::Foundation::BOOL, fconfirmedgood: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn GetCredentialsEx(&self, bstrrealm: &windows_core::BSTR, bstrurl: &windows_core::BSTR, fproxy: super::super::Foundation::BOOL, pdwurlpolicy: *mut NETSOURCE_URLCREDPOLICY_SETTINGS, pbstrname: *mut windows_core::BSTR, pbstrpassword: *mut windows_core::BSTR, pfconfirmedgood: *mut super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn DeleteCredentialsEx(&self, bstrrealm: &windows_core::BSTR, bstrurl: &windows_core::BSTR, fproxy: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn FindProxyForURLEx(&self, bstrprotocol: &windows_core::BSTR, bstrhost: &windows_core::BSTR, bstrurl: &windows_core::BSTR, pfproxyenabled: *mut super::super::Foundation::BOOL, pbstrproxyserver: *mut windows_core::BSTR, pdwproxyport: *mut u32, pdwproxycontext: *mut u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWMSInternalAdminNetSource2 {}
impl IWMSInternalAdminNetSource2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWMSInternalAdminNetSource2_Vtbl
    where
        Identity: IWMSInternalAdminNetSource2_Impl,
    {
        unsafe extern "system" fn SetCredentialsEx<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrrealm: core::mem::MaybeUninit<windows_core::BSTR>, bstrurl: core::mem::MaybeUninit<windows_core::BSTR>, fproxy: super::super::Foundation::BOOL, bstrname: core::mem::MaybeUninit<windows_core::BSTR>, bstrpassword: core::mem::MaybeUninit<windows_core::BSTR>, fpersist: super::super::Foundation::BOOL, fconfirmedgood: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IWMSInternalAdminNetSource2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMSInternalAdminNetSource2_Impl::SetCredentialsEx(this, core::mem::transmute(&bstrrealm), core::mem::transmute(&bstrurl), core::mem::transmute_copy(&fproxy), core::mem::transmute(&bstrname), core::mem::transmute(&bstrpassword), core::mem::transmute_copy(&fpersist), core::mem::transmute_copy(&fconfirmedgood)).into()
        }
        unsafe extern "system" fn GetCredentialsEx<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrrealm: core::mem::MaybeUninit<windows_core::BSTR>, bstrurl: core::mem::MaybeUninit<windows_core::BSTR>, fproxy: super::super::Foundation::BOOL, pdwurlpolicy: *mut NETSOURCE_URLCREDPOLICY_SETTINGS, pbstrname: *mut core::mem::MaybeUninit<windows_core::BSTR>, pbstrpassword: *mut core::mem::MaybeUninit<windows_core::BSTR>, pfconfirmedgood: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IWMSInternalAdminNetSource2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMSInternalAdminNetSource2_Impl::GetCredentialsEx(this, core::mem::transmute(&bstrrealm), core::mem::transmute(&bstrurl), core::mem::transmute_copy(&fproxy), core::mem::transmute_copy(&pdwurlpolicy), core::mem::transmute_copy(&pbstrname), core::mem::transmute_copy(&pbstrpassword), core::mem::transmute_copy(&pfconfirmedgood)).into()
        }
        unsafe extern "system" fn DeleteCredentialsEx<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrrealm: core::mem::MaybeUninit<windows_core::BSTR>, bstrurl: core::mem::MaybeUninit<windows_core::BSTR>, fproxy: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IWMSInternalAdminNetSource2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMSInternalAdminNetSource2_Impl::DeleteCredentialsEx(this, core::mem::transmute(&bstrrealm), core::mem::transmute(&bstrurl), core::mem::transmute_copy(&fproxy)).into()
        }
        unsafe extern "system" fn FindProxyForURLEx<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrprotocol: core::mem::MaybeUninit<windows_core::BSTR>, bstrhost: core::mem::MaybeUninit<windows_core::BSTR>, bstrurl: core::mem::MaybeUninit<windows_core::BSTR>, pfproxyenabled: *mut super::super::Foundation::BOOL, pbstrproxyserver: *mut core::mem::MaybeUninit<windows_core::BSTR>, pdwproxyport: *mut u32, pdwproxycontext: *mut u32) -> windows_core::HRESULT
        where
            Identity: IWMSInternalAdminNetSource2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMSInternalAdminNetSource2_Impl::FindProxyForURLEx(this, core::mem::transmute(&bstrprotocol), core::mem::transmute(&bstrhost), core::mem::transmute(&bstrurl), core::mem::transmute_copy(&pfproxyenabled), core::mem::transmute_copy(&pbstrproxyserver), core::mem::transmute_copy(&pdwproxyport), core::mem::transmute_copy(&pdwproxycontext)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetCredentialsEx: SetCredentialsEx::<Identity, OFFSET>,
            GetCredentialsEx: GetCredentialsEx::<Identity, OFFSET>,
            DeleteCredentialsEx: DeleteCredentialsEx::<Identity, OFFSET>,
            FindProxyForURLEx: FindProxyForURLEx::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMSInternalAdminNetSource2 as windows_core::Interface>::IID
    }
}
pub trait IWMSInternalAdminNetSource3_Impl: Sized + IWMSInternalAdminNetSource2_Impl {
    fn GetNetSourceCreator2(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn FindProxyForURLEx2(&self, bstrprotocol: &windows_core::BSTR, bstrhost: &windows_core::BSTR, bstrurl: &windows_core::BSTR, pfproxyenabled: *mut super::super::Foundation::BOOL, pbstrproxyserver: *mut windows_core::BSTR, pdwproxyport: *mut u32, pqwproxycontext: *mut u64) -> windows_core::Result<()>;
    fn RegisterProxyFailure2(&self, hrparam: windows_core::HRESULT, qwproxycontext: u64) -> windows_core::Result<()>;
    fn ShutdownProxyContext2(&self, qwproxycontext: u64) -> windows_core::Result<()>;
    fn IsUsingIE2(&self, qwproxycontext: u64) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn SetCredentialsEx2(&self, bstrrealm: &windows_core::BSTR, bstrurl: &windows_core::BSTR, fproxy: super::super::Foundation::BOOL, bstrname: &windows_core::BSTR, bstrpassword: &windows_core::BSTR, fpersist: super::super::Foundation::BOOL, fconfirmedgood: super::super::Foundation::BOOL, fcleartextauthentication: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn GetCredentialsEx2(&self, bstrrealm: &windows_core::BSTR, bstrurl: &windows_core::BSTR, fproxy: super::super::Foundation::BOOL, fcleartextauthentication: super::super::Foundation::BOOL, pdwurlpolicy: *mut NETSOURCE_URLCREDPOLICY_SETTINGS, pbstrname: *mut windows_core::BSTR, pbstrpassword: *mut windows_core::BSTR, pfconfirmedgood: *mut super::super::Foundation::BOOL) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWMSInternalAdminNetSource3 {}
impl IWMSInternalAdminNetSource3_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWMSInternalAdminNetSource3_Vtbl
    where
        Identity: IWMSInternalAdminNetSource3_Impl,
    {
        unsafe extern "system" fn GetNetSourceCreator2<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppnetsourcecreator: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMSInternalAdminNetSource3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMSInternalAdminNetSource3_Impl::GetNetSourceCreator2(this) {
                Ok(ok__) => {
                    ppnetsourcecreator.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindProxyForURLEx2<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrprotocol: core::mem::MaybeUninit<windows_core::BSTR>, bstrhost: core::mem::MaybeUninit<windows_core::BSTR>, bstrurl: core::mem::MaybeUninit<windows_core::BSTR>, pfproxyenabled: *mut super::super::Foundation::BOOL, pbstrproxyserver: *mut core::mem::MaybeUninit<windows_core::BSTR>, pdwproxyport: *mut u32, pqwproxycontext: *mut u64) -> windows_core::HRESULT
        where
            Identity: IWMSInternalAdminNetSource3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMSInternalAdminNetSource3_Impl::FindProxyForURLEx2(this, core::mem::transmute(&bstrprotocol), core::mem::transmute(&bstrhost), core::mem::transmute(&bstrurl), core::mem::transmute_copy(&pfproxyenabled), core::mem::transmute_copy(&pbstrproxyserver), core::mem::transmute_copy(&pdwproxyport), core::mem::transmute_copy(&pqwproxycontext)).into()
        }
        unsafe extern "system" fn RegisterProxyFailure2<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hrparam: windows_core::HRESULT, qwproxycontext: u64) -> windows_core::HRESULT
        where
            Identity: IWMSInternalAdminNetSource3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMSInternalAdminNetSource3_Impl::RegisterProxyFailure2(this, core::mem::transmute_copy(&hrparam), core::mem::transmute_copy(&qwproxycontext)).into()
        }
        unsafe extern "system" fn ShutdownProxyContext2<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, qwproxycontext: u64) -> windows_core::HRESULT
        where
            Identity: IWMSInternalAdminNetSource3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMSInternalAdminNetSource3_Impl::ShutdownProxyContext2(this, core::mem::transmute_copy(&qwproxycontext)).into()
        }
        unsafe extern "system" fn IsUsingIE2<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, qwproxycontext: u64, pfisusingie: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IWMSInternalAdminNetSource3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMSInternalAdminNetSource3_Impl::IsUsingIE2(this, core::mem::transmute_copy(&qwproxycontext)) {
                Ok(ok__) => {
                    pfisusingie.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCredentialsEx2<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrrealm: core::mem::MaybeUninit<windows_core::BSTR>, bstrurl: core::mem::MaybeUninit<windows_core::BSTR>, fproxy: super::super::Foundation::BOOL, bstrname: core::mem::MaybeUninit<windows_core::BSTR>, bstrpassword: core::mem::MaybeUninit<windows_core::BSTR>, fpersist: super::super::Foundation::BOOL, fconfirmedgood: super::super::Foundation::BOOL, fcleartextauthentication: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IWMSInternalAdminNetSource3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMSInternalAdminNetSource3_Impl::SetCredentialsEx2(this, core::mem::transmute(&bstrrealm), core::mem::transmute(&bstrurl), core::mem::transmute_copy(&fproxy), core::mem::transmute(&bstrname), core::mem::transmute(&bstrpassword), core::mem::transmute_copy(&fpersist), core::mem::transmute_copy(&fconfirmedgood), core::mem::transmute_copy(&fcleartextauthentication)).into()
        }
        unsafe extern "system" fn GetCredentialsEx2<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrrealm: core::mem::MaybeUninit<windows_core::BSTR>, bstrurl: core::mem::MaybeUninit<windows_core::BSTR>, fproxy: super::super::Foundation::BOOL, fcleartextauthentication: super::super::Foundation::BOOL, pdwurlpolicy: *mut NETSOURCE_URLCREDPOLICY_SETTINGS, pbstrname: *mut core::mem::MaybeUninit<windows_core::BSTR>, pbstrpassword: *mut core::mem::MaybeUninit<windows_core::BSTR>, pfconfirmedgood: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IWMSInternalAdminNetSource3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMSInternalAdminNetSource3_Impl::GetCredentialsEx2(this, core::mem::transmute(&bstrrealm), core::mem::transmute(&bstrurl), core::mem::transmute_copy(&fproxy), core::mem::transmute_copy(&fcleartextauthentication), core::mem::transmute_copy(&pdwurlpolicy), core::mem::transmute_copy(&pbstrname), core::mem::transmute_copy(&pbstrpassword), core::mem::transmute_copy(&pfconfirmedgood)).into()
        }
        Self {
            base__: IWMSInternalAdminNetSource2_Vtbl::new::<Identity, OFFSET>(),
            GetNetSourceCreator2: GetNetSourceCreator2::<Identity, OFFSET>,
            FindProxyForURLEx2: FindProxyForURLEx2::<Identity, OFFSET>,
            RegisterProxyFailure2: RegisterProxyFailure2::<Identity, OFFSET>,
            ShutdownProxyContext2: ShutdownProxyContext2::<Identity, OFFSET>,
            IsUsingIE2: IsUsingIE2::<Identity, OFFSET>,
            SetCredentialsEx2: SetCredentialsEx2::<Identity, OFFSET>,
            GetCredentialsEx2: GetCredentialsEx2::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMSInternalAdminNetSource3 as windows_core::Interface>::IID || iid == &<IWMSInternalAdminNetSource2 as windows_core::Interface>::IID
    }
}
pub trait IWMSecureChannel_Impl: Sized + IWMAuthorizer_Impl {
    fn WMSC_AddCertificate(&self, pcert: Option<&IWMAuthorizer>) -> windows_core::Result<()>;
    fn WMSC_AddSignature(&self, pbcertsig: *const u8, cbcertsig: u32) -> windows_core::Result<()>;
    fn WMSC_Connect(&self, potherside: Option<&IWMSecureChannel>) -> windows_core::Result<()>;
    fn WMSC_IsConnected(&self) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn WMSC_Disconnect(&self) -> windows_core::Result<()>;
    fn WMSC_GetValidCertificate(&self, ppbcertificate: *mut *mut u8, pdwsignature: *mut u32) -> windows_core::Result<()>;
    fn WMSC_Encrypt(&self, pbdata: *const u8, cbdata: u32) -> windows_core::Result<()>;
    fn WMSC_Decrypt(&self, pbdata: *const u8, cbdata: u32) -> windows_core::Result<()>;
    fn WMSC_Lock(&self) -> windows_core::Result<()>;
    fn WMSC_Unlock(&self) -> windows_core::Result<()>;
    fn WMSC_SetSharedData(&self, dwcertindex: u32, pbshareddata: *const u8) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWMSecureChannel {}
impl IWMSecureChannel_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWMSecureChannel_Vtbl
    where
        Identity: IWMSecureChannel_Impl,
    {
        unsafe extern "system" fn WMSC_AddCertificate<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcert: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMSecureChannel_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMSecureChannel_Impl::WMSC_AddCertificate(this, windows_core::from_raw_borrowed(&pcert)).into()
        }
        unsafe extern "system" fn WMSC_AddSignature<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbcertsig: *const u8, cbcertsig: u32) -> windows_core::HRESULT
        where
            Identity: IWMSecureChannel_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMSecureChannel_Impl::WMSC_AddSignature(this, core::mem::transmute_copy(&pbcertsig), core::mem::transmute_copy(&cbcertsig)).into()
        }
        unsafe extern "system" fn WMSC_Connect<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, potherside: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMSecureChannel_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMSecureChannel_Impl::WMSC_Connect(this, windows_core::from_raw_borrowed(&potherside)).into()
        }
        unsafe extern "system" fn WMSC_IsConnected<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfisconnected: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IWMSecureChannel_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMSecureChannel_Impl::WMSC_IsConnected(this) {
                Ok(ok__) => {
                    pfisconnected.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WMSC_Disconnect<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMSecureChannel_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMSecureChannel_Impl::WMSC_Disconnect(this).into()
        }
        unsafe extern "system" fn WMSC_GetValidCertificate<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppbcertificate: *mut *mut u8, pdwsignature: *mut u32) -> windows_core::HRESULT
        where
            Identity: IWMSecureChannel_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMSecureChannel_Impl::WMSC_GetValidCertificate(this, core::mem::transmute_copy(&ppbcertificate), core::mem::transmute_copy(&pdwsignature)).into()
        }
        unsafe extern "system" fn WMSC_Encrypt<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbdata: *const u8, cbdata: u32) -> windows_core::HRESULT
        where
            Identity: IWMSecureChannel_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMSecureChannel_Impl::WMSC_Encrypt(this, core::mem::transmute_copy(&pbdata), core::mem::transmute_copy(&cbdata)).into()
        }
        unsafe extern "system" fn WMSC_Decrypt<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbdata: *const u8, cbdata: u32) -> windows_core::HRESULT
        where
            Identity: IWMSecureChannel_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMSecureChannel_Impl::WMSC_Decrypt(this, core::mem::transmute_copy(&pbdata), core::mem::transmute_copy(&cbdata)).into()
        }
        unsafe extern "system" fn WMSC_Lock<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMSecureChannel_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMSecureChannel_Impl::WMSC_Lock(this).into()
        }
        unsafe extern "system" fn WMSC_Unlock<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMSecureChannel_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMSecureChannel_Impl::WMSC_Unlock(this).into()
        }
        unsafe extern "system" fn WMSC_SetSharedData<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwcertindex: u32, pbshareddata: *const u8) -> windows_core::HRESULT
        where
            Identity: IWMSecureChannel_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMSecureChannel_Impl::WMSC_SetSharedData(this, core::mem::transmute_copy(&dwcertindex), core::mem::transmute_copy(&pbshareddata)).into()
        }
        Self {
            base__: IWMAuthorizer_Vtbl::new::<Identity, OFFSET>(),
            WMSC_AddCertificate: WMSC_AddCertificate::<Identity, OFFSET>,
            WMSC_AddSignature: WMSC_AddSignature::<Identity, OFFSET>,
            WMSC_Connect: WMSC_Connect::<Identity, OFFSET>,
            WMSC_IsConnected: WMSC_IsConnected::<Identity, OFFSET>,
            WMSC_Disconnect: WMSC_Disconnect::<Identity, OFFSET>,
            WMSC_GetValidCertificate: WMSC_GetValidCertificate::<Identity, OFFSET>,
            WMSC_Encrypt: WMSC_Encrypt::<Identity, OFFSET>,
            WMSC_Decrypt: WMSC_Decrypt::<Identity, OFFSET>,
            WMSC_Lock: WMSC_Lock::<Identity, OFFSET>,
            WMSC_Unlock: WMSC_Unlock::<Identity, OFFSET>,
            WMSC_SetSharedData: WMSC_SetSharedData::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMSecureChannel as windows_core::Interface>::IID || iid == &<IWMAuthorizer as windows_core::Interface>::IID
    }
}
pub trait IWMStatusCallback_Impl: Sized {
    fn OnStatus(&self, status: WMT_STATUS, hr: windows_core::HRESULT, dwtype: WMT_ATTR_DATATYPE, pvalue: *const u8, pvcontext: *const core::ffi::c_void) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWMStatusCallback {}
impl IWMStatusCallback_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWMStatusCallback_Vtbl
    where
        Identity: IWMStatusCallback_Impl,
    {
        unsafe extern "system" fn OnStatus<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, status: WMT_STATUS, hr: windows_core::HRESULT, dwtype: WMT_ATTR_DATATYPE, pvalue: *const u8, pvcontext: *const core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMStatusCallback_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMStatusCallback_Impl::OnStatus(this, core::mem::transmute_copy(&status), core::mem::transmute_copy(&hr), core::mem::transmute_copy(&dwtype), core::mem::transmute_copy(&pvalue), core::mem::transmute_copy(&pvcontext)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnStatus: OnStatus::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMStatusCallback as windows_core::Interface>::IID
    }
}
pub trait IWMStreamConfig_Impl: Sized {
    fn GetStreamType(&self) -> windows_core::Result<windows_core::GUID>;
    fn GetStreamNumber(&self) -> windows_core::Result<u16>;
    fn SetStreamNumber(&self, wstreamnum: u16) -> windows_core::Result<()>;
    fn GetStreamName(&self, pwszstreamname: windows_core::PWSTR, pcchstreamname: *mut u16) -> windows_core::Result<()>;
    fn SetStreamName(&self, pwszstreamname: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetConnectionName(&self, pwszinputname: windows_core::PWSTR, pcchinputname: *mut u16) -> windows_core::Result<()>;
    fn SetConnectionName(&self, pwszinputname: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetBitrate(&self) -> windows_core::Result<u32>;
    fn SetBitrate(&self, pdwbitrate: u32) -> windows_core::Result<()>;
    fn GetBufferWindow(&self) -> windows_core::Result<u32>;
    fn SetBufferWindow(&self, msbufferwindow: u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWMStreamConfig {}
impl IWMStreamConfig_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWMStreamConfig_Vtbl
    where
        Identity: IWMStreamConfig_Impl,
    {
        unsafe extern "system" fn GetStreamType<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pguidstreamtype: *mut windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IWMStreamConfig_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMStreamConfig_Impl::GetStreamType(this) {
                Ok(ok__) => {
                    pguidstreamtype.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStreamNumber<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwstreamnum: *mut u16) -> windows_core::HRESULT
        where
            Identity: IWMStreamConfig_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMStreamConfig_Impl::GetStreamNumber(this) {
                Ok(ok__) => {
                    pwstreamnum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStreamNumber<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, wstreamnum: u16) -> windows_core::HRESULT
        where
            Identity: IWMStreamConfig_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMStreamConfig_Impl::SetStreamNumber(this, core::mem::transmute_copy(&wstreamnum)).into()
        }
        unsafe extern "system" fn GetStreamName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszstreamname: windows_core::PWSTR, pcchstreamname: *mut u16) -> windows_core::HRESULT
        where
            Identity: IWMStreamConfig_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMStreamConfig_Impl::GetStreamName(this, core::mem::transmute_copy(&pwszstreamname), core::mem::transmute_copy(&pcchstreamname)).into()
        }
        unsafe extern "system" fn SetStreamName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszstreamname: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IWMStreamConfig_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMStreamConfig_Impl::SetStreamName(this, core::mem::transmute(&pwszstreamname)).into()
        }
        unsafe extern "system" fn GetConnectionName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszinputname: windows_core::PWSTR, pcchinputname: *mut u16) -> windows_core::HRESULT
        where
            Identity: IWMStreamConfig_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMStreamConfig_Impl::GetConnectionName(this, core::mem::transmute_copy(&pwszinputname), core::mem::transmute_copy(&pcchinputname)).into()
        }
        unsafe extern "system" fn SetConnectionName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszinputname: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IWMStreamConfig_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMStreamConfig_Impl::SetConnectionName(this, core::mem::transmute(&pwszinputname)).into()
        }
        unsafe extern "system" fn GetBitrate<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwbitrate: *mut u32) -> windows_core::HRESULT
        where
            Identity: IWMStreamConfig_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMStreamConfig_Impl::GetBitrate(this) {
                Ok(ok__) => {
                    pdwbitrate.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBitrate<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwbitrate: u32) -> windows_core::HRESULT
        where
            Identity: IWMStreamConfig_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMStreamConfig_Impl::SetBitrate(this, core::mem::transmute_copy(&pdwbitrate)).into()
        }
        unsafe extern "system" fn GetBufferWindow<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmsbufferwindow: *mut u32) -> windows_core::HRESULT
        where
            Identity: IWMStreamConfig_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMStreamConfig_Impl::GetBufferWindow(this) {
                Ok(ok__) => {
                    pmsbufferwindow.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBufferWindow<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, msbufferwindow: u32) -> windows_core::HRESULT
        where
            Identity: IWMStreamConfig_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMStreamConfig_Impl::SetBufferWindow(this, core::mem::transmute_copy(&msbufferwindow)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetStreamType: GetStreamType::<Identity, OFFSET>,
            GetStreamNumber: GetStreamNumber::<Identity, OFFSET>,
            SetStreamNumber: SetStreamNumber::<Identity, OFFSET>,
            GetStreamName: GetStreamName::<Identity, OFFSET>,
            SetStreamName: SetStreamName::<Identity, OFFSET>,
            GetConnectionName: GetConnectionName::<Identity, OFFSET>,
            SetConnectionName: SetConnectionName::<Identity, OFFSET>,
            GetBitrate: GetBitrate::<Identity, OFFSET>,
            SetBitrate: SetBitrate::<Identity, OFFSET>,
            GetBufferWindow: GetBufferWindow::<Identity, OFFSET>,
            SetBufferWindow: SetBufferWindow::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMStreamConfig as windows_core::Interface>::IID
    }
}
pub trait IWMStreamConfig2_Impl: Sized + IWMStreamConfig_Impl {
    fn GetTransportType(&self) -> windows_core::Result<WMT_TRANSPORT_TYPE>;
    fn SetTransportType(&self, ntransporttype: WMT_TRANSPORT_TYPE) -> windows_core::Result<()>;
    fn AddDataUnitExtension(&self, guidextensionsystemid: &windows_core::GUID, cbextensiondatasize: u16, pbextensionsysteminfo: *const u8, cbextensionsysteminfo: u32) -> windows_core::Result<()>;
    fn GetDataUnitExtensionCount(&self) -> windows_core::Result<u16>;
    fn GetDataUnitExtension(&self, wdataunitextensionnumber: u16, pguidextensionsystemid: *mut windows_core::GUID, pcbextensiondatasize: *mut u16, pbextensionsysteminfo: *mut u8, pcbextensionsysteminfo: *mut u32) -> windows_core::Result<()>;
    fn RemoveAllDataUnitExtensions(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWMStreamConfig2 {}
impl IWMStreamConfig2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWMStreamConfig2_Vtbl
    where
        Identity: IWMStreamConfig2_Impl,
    {
        unsafe extern "system" fn GetTransportType<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pntransporttype: *mut WMT_TRANSPORT_TYPE) -> windows_core::HRESULT
        where
            Identity: IWMStreamConfig2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMStreamConfig2_Impl::GetTransportType(this) {
                Ok(ok__) => {
                    pntransporttype.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTransportType<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ntransporttype: WMT_TRANSPORT_TYPE) -> windows_core::HRESULT
        where
            Identity: IWMStreamConfig2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMStreamConfig2_Impl::SetTransportType(this, core::mem::transmute_copy(&ntransporttype)).into()
        }
        unsafe extern "system" fn AddDataUnitExtension<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, guidextensionsystemid: windows_core::GUID, cbextensiondatasize: u16, pbextensionsysteminfo: *const u8, cbextensionsysteminfo: u32) -> windows_core::HRESULT
        where
            Identity: IWMStreamConfig2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMStreamConfig2_Impl::AddDataUnitExtension(this, core::mem::transmute(&guidextensionsystemid), core::mem::transmute_copy(&cbextensiondatasize), core::mem::transmute_copy(&pbextensionsysteminfo), core::mem::transmute_copy(&cbextensionsysteminfo)).into()
        }
        unsafe extern "system" fn GetDataUnitExtensionCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcdataunitextensions: *mut u16) -> windows_core::HRESULT
        where
            Identity: IWMStreamConfig2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMStreamConfig2_Impl::GetDataUnitExtensionCount(this) {
                Ok(ok__) => {
                    pcdataunitextensions.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDataUnitExtension<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, wdataunitextensionnumber: u16, pguidextensionsystemid: *mut windows_core::GUID, pcbextensiondatasize: *mut u16, pbextensionsysteminfo: *mut u8, pcbextensionsysteminfo: *mut u32) -> windows_core::HRESULT
        where
            Identity: IWMStreamConfig2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMStreamConfig2_Impl::GetDataUnitExtension(this, core::mem::transmute_copy(&wdataunitextensionnumber), core::mem::transmute_copy(&pguidextensionsystemid), core::mem::transmute_copy(&pcbextensiondatasize), core::mem::transmute_copy(&pbextensionsysteminfo), core::mem::transmute_copy(&pcbextensionsysteminfo)).into()
        }
        unsafe extern "system" fn RemoveAllDataUnitExtensions<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMStreamConfig2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMStreamConfig2_Impl::RemoveAllDataUnitExtensions(this).into()
        }
        Self {
            base__: IWMStreamConfig_Vtbl::new::<Identity, OFFSET>(),
            GetTransportType: GetTransportType::<Identity, OFFSET>,
            SetTransportType: SetTransportType::<Identity, OFFSET>,
            AddDataUnitExtension: AddDataUnitExtension::<Identity, OFFSET>,
            GetDataUnitExtensionCount: GetDataUnitExtensionCount::<Identity, OFFSET>,
            GetDataUnitExtension: GetDataUnitExtension::<Identity, OFFSET>,
            RemoveAllDataUnitExtensions: RemoveAllDataUnitExtensions::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMStreamConfig2 as windows_core::Interface>::IID || iid == &<IWMStreamConfig as windows_core::Interface>::IID
    }
}
pub trait IWMStreamConfig3_Impl: Sized + IWMStreamConfig2_Impl {
    fn GetLanguage(&self, pwszlanguagestring: windows_core::PWSTR, pcchlanguagestringlength: *mut u16) -> windows_core::Result<()>;
    fn SetLanguage(&self, pwszlanguagestring: &windows_core::PCWSTR) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWMStreamConfig3 {}
impl IWMStreamConfig3_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWMStreamConfig3_Vtbl
    where
        Identity: IWMStreamConfig3_Impl,
    {
        unsafe extern "system" fn GetLanguage<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszlanguagestring: windows_core::PWSTR, pcchlanguagestringlength: *mut u16) -> windows_core::HRESULT
        where
            Identity: IWMStreamConfig3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMStreamConfig3_Impl::GetLanguage(this, core::mem::transmute_copy(&pwszlanguagestring), core::mem::transmute_copy(&pcchlanguagestringlength)).into()
        }
        unsafe extern "system" fn SetLanguage<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszlanguagestring: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IWMStreamConfig3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMStreamConfig3_Impl::SetLanguage(this, core::mem::transmute(&pwszlanguagestring)).into()
        }
        Self {
            base__: IWMStreamConfig2_Vtbl::new::<Identity, OFFSET>(),
            GetLanguage: GetLanguage::<Identity, OFFSET>,
            SetLanguage: SetLanguage::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMStreamConfig3 as windows_core::Interface>::IID || iid == &<IWMStreamConfig as windows_core::Interface>::IID || iid == &<IWMStreamConfig2 as windows_core::Interface>::IID
    }
}
pub trait IWMStreamList_Impl: Sized {
    fn GetStreams(&self, pwstreamnumarray: *mut u16, pcstreams: *mut u16) -> windows_core::Result<()>;
    fn AddStream(&self, wstreamnum: u16) -> windows_core::Result<()>;
    fn RemoveStream(&self, wstreamnum: u16) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWMStreamList {}
impl IWMStreamList_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWMStreamList_Vtbl
    where
        Identity: IWMStreamList_Impl,
    {
        unsafe extern "system" fn GetStreams<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwstreamnumarray: *mut u16, pcstreams: *mut u16) -> windows_core::HRESULT
        where
            Identity: IWMStreamList_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMStreamList_Impl::GetStreams(this, core::mem::transmute_copy(&pwstreamnumarray), core::mem::transmute_copy(&pcstreams)).into()
        }
        unsafe extern "system" fn AddStream<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, wstreamnum: u16) -> windows_core::HRESULT
        where
            Identity: IWMStreamList_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMStreamList_Impl::AddStream(this, core::mem::transmute_copy(&wstreamnum)).into()
        }
        unsafe extern "system" fn RemoveStream<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, wstreamnum: u16) -> windows_core::HRESULT
        where
            Identity: IWMStreamList_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMStreamList_Impl::RemoveStream(this, core::mem::transmute_copy(&wstreamnum)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetStreams: GetStreams::<Identity, OFFSET>,
            AddStream: AddStream::<Identity, OFFSET>,
            RemoveStream: RemoveStream::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMStreamList as windows_core::Interface>::IID
    }
}
pub trait IWMStreamPrioritization_Impl: Sized {
    fn GetPriorityRecords(&self, precordarray: *mut WM_STREAM_PRIORITY_RECORD, pcrecords: *mut u16) -> windows_core::Result<()>;
    fn SetPriorityRecords(&self, precordarray: *const WM_STREAM_PRIORITY_RECORD, crecords: u16) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWMStreamPrioritization {}
impl IWMStreamPrioritization_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWMStreamPrioritization_Vtbl
    where
        Identity: IWMStreamPrioritization_Impl,
    {
        unsafe extern "system" fn GetPriorityRecords<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, precordarray: *mut WM_STREAM_PRIORITY_RECORD, pcrecords: *mut u16) -> windows_core::HRESULT
        where
            Identity: IWMStreamPrioritization_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMStreamPrioritization_Impl::GetPriorityRecords(this, core::mem::transmute_copy(&precordarray), core::mem::transmute_copy(&pcrecords)).into()
        }
        unsafe extern "system" fn SetPriorityRecords<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, precordarray: *const WM_STREAM_PRIORITY_RECORD, crecords: u16) -> windows_core::HRESULT
        where
            Identity: IWMStreamPrioritization_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMStreamPrioritization_Impl::SetPriorityRecords(this, core::mem::transmute_copy(&precordarray), core::mem::transmute_copy(&crecords)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetPriorityRecords: GetPriorityRecords::<Identity, OFFSET>,
            SetPriorityRecords: SetPriorityRecords::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMStreamPrioritization as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWMSyncReader_Impl: Sized {
    fn Open(&self, pwszfilename: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn Close(&self) -> windows_core::Result<()>;
    fn SetRange(&self, cnsstarttime: u64, cnsduration: i64) -> windows_core::Result<()>;
    fn SetRangeByFrame(&self, wstreamnum: u16, qwframenumber: u64, cframestoread: i64) -> windows_core::Result<()>;
    fn GetNextSample(&self, wstreamnum: u16, ppsample: *mut Option<INSSBuffer>, pcnssampletime: *mut u64, pcnsduration: *mut u64, pdwflags: *mut u32, pdwoutputnum: *mut u32, pwstreamnum: *mut u16) -> windows_core::Result<()>;
    fn SetStreamsSelected(&self, cstreamcount: u16, pwstreamnumbers: *const u16, pselections: *const WMT_STREAM_SELECTION) -> windows_core::Result<()>;
    fn GetStreamSelected(&self, wstreamnum: u16) -> windows_core::Result<WMT_STREAM_SELECTION>;
    fn SetReadStreamSamples(&self, wstreamnum: u16, fcompressed: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn GetReadStreamSamples(&self, wstreamnum: u16) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn GetOutputSetting(&self, dwoutputnum: u32, pszname: &windows_core::PCWSTR, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> windows_core::Result<()>;
    fn SetOutputSetting(&self, dwoutputnum: u32, pszname: &windows_core::PCWSTR, r#type: WMT_ATTR_DATATYPE, pvalue: *const u8, cblength: u16) -> windows_core::Result<()>;
    fn GetOutputCount(&self) -> windows_core::Result<u32>;
    fn GetOutputProps(&self, dwoutputnum: u32) -> windows_core::Result<IWMOutputMediaProps>;
    fn SetOutputProps(&self, dwoutputnum: u32, poutput: Option<&IWMOutputMediaProps>) -> windows_core::Result<()>;
    fn GetOutputFormatCount(&self, dwoutputnum: u32) -> windows_core::Result<u32>;
    fn GetOutputFormat(&self, dwoutputnum: u32, dwformatnum: u32) -> windows_core::Result<IWMOutputMediaProps>;
    fn GetOutputNumberForStream(&self, wstreamnum: u16) -> windows_core::Result<u32>;
    fn GetStreamNumberForOutput(&self, dwoutputnum: u32) -> windows_core::Result<u16>;
    fn GetMaxOutputSampleSize(&self, dwoutput: u32) -> windows_core::Result<u32>;
    fn GetMaxStreamSampleSize(&self, wstream: u16) -> windows_core::Result<u32>;
    fn OpenStream(&self, pstream: Option<&super::super::System::Com::IStream>) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IWMSyncReader {}
#[cfg(feature = "Win32_System_Com")]
impl IWMSyncReader_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWMSyncReader_Vtbl
    where
        Identity: IWMSyncReader_Impl,
    {
        unsafe extern "system" fn Open<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszfilename: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IWMSyncReader_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMSyncReader_Impl::Open(this, core::mem::transmute(&pwszfilename)).into()
        }
        unsafe extern "system" fn Close<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMSyncReader_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMSyncReader_Impl::Close(this).into()
        }
        unsafe extern "system" fn SetRange<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, cnsstarttime: u64, cnsduration: i64) -> windows_core::HRESULT
        where
            Identity: IWMSyncReader_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMSyncReader_Impl::SetRange(this, core::mem::transmute_copy(&cnsstarttime), core::mem::transmute_copy(&cnsduration)).into()
        }
        unsafe extern "system" fn SetRangeByFrame<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, wstreamnum: u16, qwframenumber: u64, cframestoread: i64) -> windows_core::HRESULT
        where
            Identity: IWMSyncReader_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMSyncReader_Impl::SetRangeByFrame(this, core::mem::transmute_copy(&wstreamnum), core::mem::transmute_copy(&qwframenumber), core::mem::transmute_copy(&cframestoread)).into()
        }
        unsafe extern "system" fn GetNextSample<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, wstreamnum: u16, ppsample: *mut *mut core::ffi::c_void, pcnssampletime: *mut u64, pcnsduration: *mut u64, pdwflags: *mut u32, pdwoutputnum: *mut u32, pwstreamnum: *mut u16) -> windows_core::HRESULT
        where
            Identity: IWMSyncReader_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMSyncReader_Impl::GetNextSample(this, core::mem::transmute_copy(&wstreamnum), core::mem::transmute_copy(&ppsample), core::mem::transmute_copy(&pcnssampletime), core::mem::transmute_copy(&pcnsduration), core::mem::transmute_copy(&pdwflags), core::mem::transmute_copy(&pdwoutputnum), core::mem::transmute_copy(&pwstreamnum)).into()
        }
        unsafe extern "system" fn SetStreamsSelected<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, cstreamcount: u16, pwstreamnumbers: *const u16, pselections: *const WMT_STREAM_SELECTION) -> windows_core::HRESULT
        where
            Identity: IWMSyncReader_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMSyncReader_Impl::SetStreamsSelected(this, core::mem::transmute_copy(&cstreamcount), core::mem::transmute_copy(&pwstreamnumbers), core::mem::transmute_copy(&pselections)).into()
        }
        unsafe extern "system" fn GetStreamSelected<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, wstreamnum: u16, pselection: *mut WMT_STREAM_SELECTION) -> windows_core::HRESULT
        where
            Identity: IWMSyncReader_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMSyncReader_Impl::GetStreamSelected(this, core::mem::transmute_copy(&wstreamnum)) {
                Ok(ok__) => {
                    pselection.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetReadStreamSamples<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, wstreamnum: u16, fcompressed: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IWMSyncReader_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMSyncReader_Impl::SetReadStreamSamples(this, core::mem::transmute_copy(&wstreamnum), core::mem::transmute_copy(&fcompressed)).into()
        }
        unsafe extern "system" fn GetReadStreamSamples<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, wstreamnum: u16, pfcompressed: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IWMSyncReader_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMSyncReader_Impl::GetReadStreamSamples(this, core::mem::transmute_copy(&wstreamnum)) {
                Ok(ok__) => {
                    pfcompressed.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOutputSetting<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwoutputnum: u32, pszname: windows_core::PCWSTR, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> windows_core::HRESULT
        where
            Identity: IWMSyncReader_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMSyncReader_Impl::GetOutputSetting(this, core::mem::transmute_copy(&dwoutputnum), core::mem::transmute(&pszname), core::mem::transmute_copy(&ptype), core::mem::transmute_copy(&pvalue), core::mem::transmute_copy(&pcblength)).into()
        }
        unsafe extern "system" fn SetOutputSetting<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwoutputnum: u32, pszname: windows_core::PCWSTR, r#type: WMT_ATTR_DATATYPE, pvalue: *const u8, cblength: u16) -> windows_core::HRESULT
        where
            Identity: IWMSyncReader_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMSyncReader_Impl::SetOutputSetting(this, core::mem::transmute_copy(&dwoutputnum), core::mem::transmute(&pszname), core::mem::transmute_copy(&r#type), core::mem::transmute_copy(&pvalue), core::mem::transmute_copy(&cblength)).into()
        }
        unsafe extern "system" fn GetOutputCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcoutputs: *mut u32) -> windows_core::HRESULT
        where
            Identity: IWMSyncReader_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMSyncReader_Impl::GetOutputCount(this) {
                Ok(ok__) => {
                    pcoutputs.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOutputProps<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwoutputnum: u32, ppoutput: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMSyncReader_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMSyncReader_Impl::GetOutputProps(this, core::mem::transmute_copy(&dwoutputnum)) {
                Ok(ok__) => {
                    ppoutput.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOutputProps<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwoutputnum: u32, poutput: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMSyncReader_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMSyncReader_Impl::SetOutputProps(this, core::mem::transmute_copy(&dwoutputnum), windows_core::from_raw_borrowed(&poutput)).into()
        }
        unsafe extern "system" fn GetOutputFormatCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwoutputnum: u32, pcformats: *mut u32) -> windows_core::HRESULT
        where
            Identity: IWMSyncReader_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMSyncReader_Impl::GetOutputFormatCount(this, core::mem::transmute_copy(&dwoutputnum)) {
                Ok(ok__) => {
                    pcformats.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOutputFormat<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwoutputnum: u32, dwformatnum: u32, ppprops: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMSyncReader_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMSyncReader_Impl::GetOutputFormat(this, core::mem::transmute_copy(&dwoutputnum), core::mem::transmute_copy(&dwformatnum)) {
                Ok(ok__) => {
                    ppprops.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOutputNumberForStream<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, wstreamnum: u16, pdwoutputnum: *mut u32) -> windows_core::HRESULT
        where
            Identity: IWMSyncReader_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMSyncReader_Impl::GetOutputNumberForStream(this, core::mem::transmute_copy(&wstreamnum)) {
                Ok(ok__) => {
                    pdwoutputnum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStreamNumberForOutput<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwoutputnum: u32, pwstreamnum: *mut u16) -> windows_core::HRESULT
        where
            Identity: IWMSyncReader_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMSyncReader_Impl::GetStreamNumberForOutput(this, core::mem::transmute_copy(&dwoutputnum)) {
                Ok(ok__) => {
                    pwstreamnum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMaxOutputSampleSize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwoutput: u32, pcbmax: *mut u32) -> windows_core::HRESULT
        where
            Identity: IWMSyncReader_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMSyncReader_Impl::GetMaxOutputSampleSize(this, core::mem::transmute_copy(&dwoutput)) {
                Ok(ok__) => {
                    pcbmax.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMaxStreamSampleSize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, wstream: u16, pcbmax: *mut u32) -> windows_core::HRESULT
        where
            Identity: IWMSyncReader_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMSyncReader_Impl::GetMaxStreamSampleSize(this, core::mem::transmute_copy(&wstream)) {
                Ok(ok__) => {
                    pcbmax.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenStream<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstream: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMSyncReader_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMSyncReader_Impl::OpenStream(this, windows_core::from_raw_borrowed(&pstream)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Open: Open::<Identity, OFFSET>,
            Close: Close::<Identity, OFFSET>,
            SetRange: SetRange::<Identity, OFFSET>,
            SetRangeByFrame: SetRangeByFrame::<Identity, OFFSET>,
            GetNextSample: GetNextSample::<Identity, OFFSET>,
            SetStreamsSelected: SetStreamsSelected::<Identity, OFFSET>,
            GetStreamSelected: GetStreamSelected::<Identity, OFFSET>,
            SetReadStreamSamples: SetReadStreamSamples::<Identity, OFFSET>,
            GetReadStreamSamples: GetReadStreamSamples::<Identity, OFFSET>,
            GetOutputSetting: GetOutputSetting::<Identity, OFFSET>,
            SetOutputSetting: SetOutputSetting::<Identity, OFFSET>,
            GetOutputCount: GetOutputCount::<Identity, OFFSET>,
            GetOutputProps: GetOutputProps::<Identity, OFFSET>,
            SetOutputProps: SetOutputProps::<Identity, OFFSET>,
            GetOutputFormatCount: GetOutputFormatCount::<Identity, OFFSET>,
            GetOutputFormat: GetOutputFormat::<Identity, OFFSET>,
            GetOutputNumberForStream: GetOutputNumberForStream::<Identity, OFFSET>,
            GetStreamNumberForOutput: GetStreamNumberForOutput::<Identity, OFFSET>,
            GetMaxOutputSampleSize: GetMaxOutputSampleSize::<Identity, OFFSET>,
            GetMaxStreamSampleSize: GetMaxStreamSampleSize::<Identity, OFFSET>,
            OpenStream: OpenStream::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMSyncReader as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWMSyncReader2_Impl: Sized + IWMSyncReader_Impl {
    fn SetRangeByTimecode(&self, wstreamnum: u16, pstart: *const WMT_TIMECODE_EXTENSION_DATA, pend: *const WMT_TIMECODE_EXTENSION_DATA) -> windows_core::Result<()>;
    fn SetRangeByFrameEx(&self, wstreamnum: u16, qwframenumber: u64, cframestoread: i64) -> windows_core::Result<u64>;
    fn SetAllocateForOutput(&self, dwoutputnum: u32, pallocator: Option<&IWMReaderAllocatorEx>) -> windows_core::Result<()>;
    fn GetAllocateForOutput(&self, dwoutputnum: u32) -> windows_core::Result<IWMReaderAllocatorEx>;
    fn SetAllocateForStream(&self, wstreamnum: u16, pallocator: Option<&IWMReaderAllocatorEx>) -> windows_core::Result<()>;
    fn GetAllocateForStream(&self, dwsreamnum: u16) -> windows_core::Result<IWMReaderAllocatorEx>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IWMSyncReader2 {}
#[cfg(feature = "Win32_System_Com")]
impl IWMSyncReader2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWMSyncReader2_Vtbl
    where
        Identity: IWMSyncReader2_Impl,
    {
        unsafe extern "system" fn SetRangeByTimecode<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, wstreamnum: u16, pstart: *const WMT_TIMECODE_EXTENSION_DATA, pend: *const WMT_TIMECODE_EXTENSION_DATA) -> windows_core::HRESULT
        where
            Identity: IWMSyncReader2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMSyncReader2_Impl::SetRangeByTimecode(this, core::mem::transmute_copy(&wstreamnum), core::mem::transmute_copy(&pstart), core::mem::transmute_copy(&pend)).into()
        }
        unsafe extern "system" fn SetRangeByFrameEx<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, wstreamnum: u16, qwframenumber: u64, cframestoread: i64, pcnsstarttime: *mut u64) -> windows_core::HRESULT
        where
            Identity: IWMSyncReader2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMSyncReader2_Impl::SetRangeByFrameEx(this, core::mem::transmute_copy(&wstreamnum), core::mem::transmute_copy(&qwframenumber), core::mem::transmute_copy(&cframestoread)) {
                Ok(ok__) => {
                    pcnsstarttime.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAllocateForOutput<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwoutputnum: u32, pallocator: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMSyncReader2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMSyncReader2_Impl::SetAllocateForOutput(this, core::mem::transmute_copy(&dwoutputnum), windows_core::from_raw_borrowed(&pallocator)).into()
        }
        unsafe extern "system" fn GetAllocateForOutput<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwoutputnum: u32, ppallocator: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMSyncReader2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMSyncReader2_Impl::GetAllocateForOutput(this, core::mem::transmute_copy(&dwoutputnum)) {
                Ok(ok__) => {
                    ppallocator.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAllocateForStream<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, wstreamnum: u16, pallocator: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMSyncReader2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMSyncReader2_Impl::SetAllocateForStream(this, core::mem::transmute_copy(&wstreamnum), windows_core::from_raw_borrowed(&pallocator)).into()
        }
        unsafe extern "system" fn GetAllocateForStream<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwsreamnum: u16, ppallocator: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMSyncReader2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMSyncReader2_Impl::GetAllocateForStream(this, core::mem::transmute_copy(&dwsreamnum)) {
                Ok(ok__) => {
                    ppallocator.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: IWMSyncReader_Vtbl::new::<Identity, OFFSET>(),
            SetRangeByTimecode: SetRangeByTimecode::<Identity, OFFSET>,
            SetRangeByFrameEx: SetRangeByFrameEx::<Identity, OFFSET>,
            SetAllocateForOutput: SetAllocateForOutput::<Identity, OFFSET>,
            GetAllocateForOutput: GetAllocateForOutput::<Identity, OFFSET>,
            SetAllocateForStream: SetAllocateForStream::<Identity, OFFSET>,
            GetAllocateForStream: GetAllocateForStream::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMSyncReader2 as windows_core::Interface>::IID || iid == &<IWMSyncReader as windows_core::Interface>::IID
    }
}
pub trait IWMVideoMediaProps_Impl: Sized + IWMMediaProps_Impl {
    fn GetMaxKeyFrameSpacing(&self) -> windows_core::Result<i64>;
    fn SetMaxKeyFrameSpacing(&self, lltime: i64) -> windows_core::Result<()>;
    fn GetQuality(&self) -> windows_core::Result<u32>;
    fn SetQuality(&self, dwquality: u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWMVideoMediaProps {}
impl IWMVideoMediaProps_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWMVideoMediaProps_Vtbl
    where
        Identity: IWMVideoMediaProps_Impl,
    {
        unsafe extern "system" fn GetMaxKeyFrameSpacing<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plltime: *mut i64) -> windows_core::HRESULT
        where
            Identity: IWMVideoMediaProps_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMVideoMediaProps_Impl::GetMaxKeyFrameSpacing(this) {
                Ok(ok__) => {
                    plltime.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxKeyFrameSpacing<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lltime: i64) -> windows_core::HRESULT
        where
            Identity: IWMVideoMediaProps_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMVideoMediaProps_Impl::SetMaxKeyFrameSpacing(this, core::mem::transmute_copy(&lltime)).into()
        }
        unsafe extern "system" fn GetQuality<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwquality: *mut u32) -> windows_core::HRESULT
        where
            Identity: IWMVideoMediaProps_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMVideoMediaProps_Impl::GetQuality(this) {
                Ok(ok__) => {
                    pdwquality.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetQuality<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwquality: u32) -> windows_core::HRESULT
        where
            Identity: IWMVideoMediaProps_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMVideoMediaProps_Impl::SetQuality(this, core::mem::transmute_copy(&dwquality)).into()
        }
        Self {
            base__: IWMMediaProps_Vtbl::new::<Identity, OFFSET>(),
            GetMaxKeyFrameSpacing: GetMaxKeyFrameSpacing::<Identity, OFFSET>,
            SetMaxKeyFrameSpacing: SetMaxKeyFrameSpacing::<Identity, OFFSET>,
            GetQuality: GetQuality::<Identity, OFFSET>,
            SetQuality: SetQuality::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMVideoMediaProps as windows_core::Interface>::IID || iid == &<IWMMediaProps as windows_core::Interface>::IID
    }
}
pub trait IWMWatermarkInfo_Impl: Sized {
    fn GetWatermarkEntryCount(&self, wmettype: WMT_WATERMARK_ENTRY_TYPE) -> windows_core::Result<u32>;
    fn GetWatermarkEntry(&self, wmettype: WMT_WATERMARK_ENTRY_TYPE, dwentrynum: u32, pentry: *mut WMT_WATERMARK_ENTRY) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWMWatermarkInfo {}
impl IWMWatermarkInfo_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWMWatermarkInfo_Vtbl
    where
        Identity: IWMWatermarkInfo_Impl,
    {
        unsafe extern "system" fn GetWatermarkEntryCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, wmettype: WMT_WATERMARK_ENTRY_TYPE, pdwcount: *mut u32) -> windows_core::HRESULT
        where
            Identity: IWMWatermarkInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMWatermarkInfo_Impl::GetWatermarkEntryCount(this, core::mem::transmute_copy(&wmettype)) {
                Ok(ok__) => {
                    pdwcount.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetWatermarkEntry<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, wmettype: WMT_WATERMARK_ENTRY_TYPE, dwentrynum: u32, pentry: *mut WMT_WATERMARK_ENTRY) -> windows_core::HRESULT
        where
            Identity: IWMWatermarkInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMWatermarkInfo_Impl::GetWatermarkEntry(this, core::mem::transmute_copy(&wmettype), core::mem::transmute_copy(&dwentrynum), core::mem::transmute_copy(&pentry)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetWatermarkEntryCount: GetWatermarkEntryCount::<Identity, OFFSET>,
            GetWatermarkEntry: GetWatermarkEntry::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMWatermarkInfo as windows_core::Interface>::IID
    }
}
pub trait IWMWriter_Impl: Sized {
    fn SetProfileByID(&self, guidprofile: *const windows_core::GUID) -> windows_core::Result<()>;
    fn SetProfile(&self, pprofile: Option<&IWMProfile>) -> windows_core::Result<()>;
    fn SetOutputFilename(&self, pwszfilename: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetInputCount(&self) -> windows_core::Result<u32>;
    fn GetInputProps(&self, dwinputnum: u32) -> windows_core::Result<IWMInputMediaProps>;
    fn SetInputProps(&self, dwinputnum: u32, pinput: Option<&IWMInputMediaProps>) -> windows_core::Result<()>;
    fn GetInputFormatCount(&self, dwinputnumber: u32) -> windows_core::Result<u32>;
    fn GetInputFormat(&self, dwinputnumber: u32, dwformatnumber: u32) -> windows_core::Result<IWMInputMediaProps>;
    fn BeginWriting(&self) -> windows_core::Result<()>;
    fn EndWriting(&self) -> windows_core::Result<()>;
    fn AllocateSample(&self, dwsamplesize: u32) -> windows_core::Result<INSSBuffer>;
    fn WriteSample(&self, dwinputnum: u32, cnssampletime: u64, dwflags: u32, psample: Option<&INSSBuffer>) -> windows_core::Result<()>;
    fn Flush(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWMWriter {}
impl IWMWriter_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWMWriter_Vtbl
    where
        Identity: IWMWriter_Impl,
    {
        unsafe extern "system" fn SetProfileByID<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, guidprofile: *const windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IWMWriter_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMWriter_Impl::SetProfileByID(this, core::mem::transmute_copy(&guidprofile)).into()
        }
        unsafe extern "system" fn SetProfile<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pprofile: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMWriter_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMWriter_Impl::SetProfile(this, windows_core::from_raw_borrowed(&pprofile)).into()
        }
        unsafe extern "system" fn SetOutputFilename<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszfilename: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IWMWriter_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMWriter_Impl::SetOutputFilename(this, core::mem::transmute(&pwszfilename)).into()
        }
        unsafe extern "system" fn GetInputCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcinputs: *mut u32) -> windows_core::HRESULT
        where
            Identity: IWMWriter_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMWriter_Impl::GetInputCount(this) {
                Ok(ok__) => {
                    pcinputs.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetInputProps<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwinputnum: u32, ppinput: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMWriter_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMWriter_Impl::GetInputProps(this, core::mem::transmute_copy(&dwinputnum)) {
                Ok(ok__) => {
                    ppinput.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInputProps<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwinputnum: u32, pinput: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMWriter_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMWriter_Impl::SetInputProps(this, core::mem::transmute_copy(&dwinputnum), windows_core::from_raw_borrowed(&pinput)).into()
        }
        unsafe extern "system" fn GetInputFormatCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwinputnumber: u32, pcformats: *mut u32) -> windows_core::HRESULT
        where
            Identity: IWMWriter_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMWriter_Impl::GetInputFormatCount(this, core::mem::transmute_copy(&dwinputnumber)) {
                Ok(ok__) => {
                    pcformats.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetInputFormat<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwinputnumber: u32, dwformatnumber: u32, pprops: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMWriter_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMWriter_Impl::GetInputFormat(this, core::mem::transmute_copy(&dwinputnumber), core::mem::transmute_copy(&dwformatnumber)) {
                Ok(ok__) => {
                    pprops.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginWriting<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMWriter_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMWriter_Impl::BeginWriting(this).into()
        }
        unsafe extern "system" fn EndWriting<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMWriter_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMWriter_Impl::EndWriting(this).into()
        }
        unsafe extern "system" fn AllocateSample<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwsamplesize: u32, ppsample: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMWriter_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMWriter_Impl::AllocateSample(this, core::mem::transmute_copy(&dwsamplesize)) {
                Ok(ok__) => {
                    ppsample.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WriteSample<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwinputnum: u32, cnssampletime: u64, dwflags: u32, psample: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMWriter_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMWriter_Impl::WriteSample(this, core::mem::transmute_copy(&dwinputnum), core::mem::transmute_copy(&cnssampletime), core::mem::transmute_copy(&dwflags), windows_core::from_raw_borrowed(&psample)).into()
        }
        unsafe extern "system" fn Flush<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMWriter_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMWriter_Impl::Flush(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetProfileByID: SetProfileByID::<Identity, OFFSET>,
            SetProfile: SetProfile::<Identity, OFFSET>,
            SetOutputFilename: SetOutputFilename::<Identity, OFFSET>,
            GetInputCount: GetInputCount::<Identity, OFFSET>,
            GetInputProps: GetInputProps::<Identity, OFFSET>,
            SetInputProps: SetInputProps::<Identity, OFFSET>,
            GetInputFormatCount: GetInputFormatCount::<Identity, OFFSET>,
            GetInputFormat: GetInputFormat::<Identity, OFFSET>,
            BeginWriting: BeginWriting::<Identity, OFFSET>,
            EndWriting: EndWriting::<Identity, OFFSET>,
            AllocateSample: AllocateSample::<Identity, OFFSET>,
            WriteSample: WriteSample::<Identity, OFFSET>,
            Flush: Flush::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMWriter as windows_core::Interface>::IID
    }
}
pub trait IWMWriterAdvanced_Impl: Sized {
    fn GetSinkCount(&self) -> windows_core::Result<u32>;
    fn GetSink(&self, dwsinknum: u32) -> windows_core::Result<IWMWriterSink>;
    fn AddSink(&self, psink: Option<&IWMWriterSink>) -> windows_core::Result<()>;
    fn RemoveSink(&self, psink: Option<&IWMWriterSink>) -> windows_core::Result<()>;
    fn WriteStreamSample(&self, wstreamnum: u16, cnssampletime: u64, mssamplesendtime: u32, cnssampleduration: u64, dwflags: u32, psample: Option<&INSSBuffer>) -> windows_core::Result<()>;
    fn SetLiveSource(&self, fislivesource: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn IsRealTime(&self) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn GetWriterTime(&self) -> windows_core::Result<u64>;
    fn GetStatistics(&self, wstreamnum: u16, pstats: *mut WM_WRITER_STATISTICS) -> windows_core::Result<()>;
    fn SetSyncTolerance(&self, mswindow: u32) -> windows_core::Result<()>;
    fn GetSyncTolerance(&self) -> windows_core::Result<u32>;
}
impl windows_core::RuntimeName for IWMWriterAdvanced {}
impl IWMWriterAdvanced_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWMWriterAdvanced_Vtbl
    where
        Identity: IWMWriterAdvanced_Impl,
    {
        unsafe extern "system" fn GetSinkCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcsinks: *mut u32) -> windows_core::HRESULT
        where
            Identity: IWMWriterAdvanced_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMWriterAdvanced_Impl::GetSinkCount(this) {
                Ok(ok__) => {
                    pcsinks.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSink<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwsinknum: u32, ppsink: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMWriterAdvanced_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMWriterAdvanced_Impl::GetSink(this, core::mem::transmute_copy(&dwsinknum)) {
                Ok(ok__) => {
                    ppsink.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddSink<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, psink: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMWriterAdvanced_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMWriterAdvanced_Impl::AddSink(this, windows_core::from_raw_borrowed(&psink)).into()
        }
        unsafe extern "system" fn RemoveSink<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, psink: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMWriterAdvanced_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMWriterAdvanced_Impl::RemoveSink(this, windows_core::from_raw_borrowed(&psink)).into()
        }
        unsafe extern "system" fn WriteStreamSample<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, wstreamnum: u16, cnssampletime: u64, mssamplesendtime: u32, cnssampleduration: u64, dwflags: u32, psample: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMWriterAdvanced_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMWriterAdvanced_Impl::WriteStreamSample(this, core::mem::transmute_copy(&wstreamnum), core::mem::transmute_copy(&cnssampletime), core::mem::transmute_copy(&mssamplesendtime), core::mem::transmute_copy(&cnssampleduration), core::mem::transmute_copy(&dwflags), windows_core::from_raw_borrowed(&psample)).into()
        }
        unsafe extern "system" fn SetLiveSource<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fislivesource: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IWMWriterAdvanced_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMWriterAdvanced_Impl::SetLiveSource(this, core::mem::transmute_copy(&fislivesource)).into()
        }
        unsafe extern "system" fn IsRealTime<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfrealtime: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IWMWriterAdvanced_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMWriterAdvanced_Impl::IsRealTime(this) {
                Ok(ok__) => {
                    pfrealtime.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetWriterTime<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcnscurrenttime: *mut u64) -> windows_core::HRESULT
        where
            Identity: IWMWriterAdvanced_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMWriterAdvanced_Impl::GetWriterTime(this) {
                Ok(ok__) => {
                    pcnscurrenttime.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStatistics<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, wstreamnum: u16, pstats: *mut WM_WRITER_STATISTICS) -> windows_core::HRESULT
        where
            Identity: IWMWriterAdvanced_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMWriterAdvanced_Impl::GetStatistics(this, core::mem::transmute_copy(&wstreamnum), core::mem::transmute_copy(&pstats)).into()
        }
        unsafe extern "system" fn SetSyncTolerance<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, mswindow: u32) -> windows_core::HRESULT
        where
            Identity: IWMWriterAdvanced_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMWriterAdvanced_Impl::SetSyncTolerance(this, core::mem::transmute_copy(&mswindow)).into()
        }
        unsafe extern "system" fn GetSyncTolerance<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmswindow: *mut u32) -> windows_core::HRESULT
        where
            Identity: IWMWriterAdvanced_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMWriterAdvanced_Impl::GetSyncTolerance(this) {
                Ok(ok__) => {
                    pmswindow.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetSinkCount: GetSinkCount::<Identity, OFFSET>,
            GetSink: GetSink::<Identity, OFFSET>,
            AddSink: AddSink::<Identity, OFFSET>,
            RemoveSink: RemoveSink::<Identity, OFFSET>,
            WriteStreamSample: WriteStreamSample::<Identity, OFFSET>,
            SetLiveSource: SetLiveSource::<Identity, OFFSET>,
            IsRealTime: IsRealTime::<Identity, OFFSET>,
            GetWriterTime: GetWriterTime::<Identity, OFFSET>,
            GetStatistics: GetStatistics::<Identity, OFFSET>,
            SetSyncTolerance: SetSyncTolerance::<Identity, OFFSET>,
            GetSyncTolerance: GetSyncTolerance::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMWriterAdvanced as windows_core::Interface>::IID
    }
}
pub trait IWMWriterAdvanced2_Impl: Sized + IWMWriterAdvanced_Impl {
    fn GetInputSetting(&self, dwinputnum: u32, pszname: &windows_core::PCWSTR, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> windows_core::Result<()>;
    fn SetInputSetting(&self, dwinputnum: u32, pszname: &windows_core::PCWSTR, r#type: WMT_ATTR_DATATYPE, pvalue: *const u8, cblength: u16) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWMWriterAdvanced2 {}
impl IWMWriterAdvanced2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWMWriterAdvanced2_Vtbl
    where
        Identity: IWMWriterAdvanced2_Impl,
    {
        unsafe extern "system" fn GetInputSetting<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwinputnum: u32, pszname: windows_core::PCWSTR, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> windows_core::HRESULT
        where
            Identity: IWMWriterAdvanced2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMWriterAdvanced2_Impl::GetInputSetting(this, core::mem::transmute_copy(&dwinputnum), core::mem::transmute(&pszname), core::mem::transmute_copy(&ptype), core::mem::transmute_copy(&pvalue), core::mem::transmute_copy(&pcblength)).into()
        }
        unsafe extern "system" fn SetInputSetting<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwinputnum: u32, pszname: windows_core::PCWSTR, r#type: WMT_ATTR_DATATYPE, pvalue: *const u8, cblength: u16) -> windows_core::HRESULT
        where
            Identity: IWMWriterAdvanced2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMWriterAdvanced2_Impl::SetInputSetting(this, core::mem::transmute_copy(&dwinputnum), core::mem::transmute(&pszname), core::mem::transmute_copy(&r#type), core::mem::transmute_copy(&pvalue), core::mem::transmute_copy(&cblength)).into()
        }
        Self {
            base__: IWMWriterAdvanced_Vtbl::new::<Identity, OFFSET>(),
            GetInputSetting: GetInputSetting::<Identity, OFFSET>,
            SetInputSetting: SetInputSetting::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMWriterAdvanced2 as windows_core::Interface>::IID || iid == &<IWMWriterAdvanced as windows_core::Interface>::IID
    }
}
pub trait IWMWriterAdvanced3_Impl: Sized + IWMWriterAdvanced2_Impl {
    fn GetStatisticsEx(&self, wstreamnum: u16, pstats: *mut WM_WRITER_STATISTICS_EX) -> windows_core::Result<()>;
    fn SetNonBlocking(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWMWriterAdvanced3 {}
impl IWMWriterAdvanced3_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWMWriterAdvanced3_Vtbl
    where
        Identity: IWMWriterAdvanced3_Impl,
    {
        unsafe extern "system" fn GetStatisticsEx<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, wstreamnum: u16, pstats: *mut WM_WRITER_STATISTICS_EX) -> windows_core::HRESULT
        where
            Identity: IWMWriterAdvanced3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMWriterAdvanced3_Impl::GetStatisticsEx(this, core::mem::transmute_copy(&wstreamnum), core::mem::transmute_copy(&pstats)).into()
        }
        unsafe extern "system" fn SetNonBlocking<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMWriterAdvanced3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMWriterAdvanced3_Impl::SetNonBlocking(this).into()
        }
        Self {
            base__: IWMWriterAdvanced2_Vtbl::new::<Identity, OFFSET>(),
            GetStatisticsEx: GetStatisticsEx::<Identity, OFFSET>,
            SetNonBlocking: SetNonBlocking::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMWriterAdvanced3 as windows_core::Interface>::IID || iid == &<IWMWriterAdvanced as windows_core::Interface>::IID || iid == &<IWMWriterAdvanced2 as windows_core::Interface>::IID
    }
}
pub trait IWMWriterFileSink_Impl: Sized + IWMWriterSink_Impl {
    fn Open(&self, pwszfilename: &windows_core::PCWSTR) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWMWriterFileSink {}
impl IWMWriterFileSink_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWMWriterFileSink_Vtbl
    where
        Identity: IWMWriterFileSink_Impl,
    {
        unsafe extern "system" fn Open<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszfilename: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IWMWriterFileSink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMWriterFileSink_Impl::Open(this, core::mem::transmute(&pwszfilename)).into()
        }
        Self { base__: IWMWriterSink_Vtbl::new::<Identity, OFFSET>(), Open: Open::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMWriterFileSink as windows_core::Interface>::IID || iid == &<IWMWriterSink as windows_core::Interface>::IID
    }
}
pub trait IWMWriterFileSink2_Impl: Sized + IWMWriterFileSink_Impl {
    fn Start(&self, cnsstarttime: u64) -> windows_core::Result<()>;
    fn Stop(&self, cnsstoptime: u64) -> windows_core::Result<()>;
    fn IsStopped(&self) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn GetFileDuration(&self) -> windows_core::Result<u64>;
    fn GetFileSize(&self) -> windows_core::Result<u64>;
    fn Close(&self) -> windows_core::Result<()>;
    fn IsClosed(&self) -> windows_core::Result<super::super::Foundation::BOOL>;
}
impl windows_core::RuntimeName for IWMWriterFileSink2 {}
impl IWMWriterFileSink2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWMWriterFileSink2_Vtbl
    where
        Identity: IWMWriterFileSink2_Impl,
    {
        unsafe extern "system" fn Start<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, cnsstarttime: u64) -> windows_core::HRESULT
        where
            Identity: IWMWriterFileSink2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMWriterFileSink2_Impl::Start(this, core::mem::transmute_copy(&cnsstarttime)).into()
        }
        unsafe extern "system" fn Stop<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, cnsstoptime: u64) -> windows_core::HRESULT
        where
            Identity: IWMWriterFileSink2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMWriterFileSink2_Impl::Stop(this, core::mem::transmute_copy(&cnsstoptime)).into()
        }
        unsafe extern "system" fn IsStopped<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfstopped: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IWMWriterFileSink2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMWriterFileSink2_Impl::IsStopped(this) {
                Ok(ok__) => {
                    pfstopped.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFileDuration<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcnsduration: *mut u64) -> windows_core::HRESULT
        where
            Identity: IWMWriterFileSink2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMWriterFileSink2_Impl::GetFileDuration(this) {
                Ok(ok__) => {
                    pcnsduration.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFileSize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcbfile: *mut u64) -> windows_core::HRESULT
        where
            Identity: IWMWriterFileSink2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMWriterFileSink2_Impl::GetFileSize(this) {
                Ok(ok__) => {
                    pcbfile.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Close<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMWriterFileSink2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMWriterFileSink2_Impl::Close(this).into()
        }
        unsafe extern "system" fn IsClosed<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfclosed: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IWMWriterFileSink2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMWriterFileSink2_Impl::IsClosed(this) {
                Ok(ok__) => {
                    pfclosed.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: IWMWriterFileSink_Vtbl::new::<Identity, OFFSET>(),
            Start: Start::<Identity, OFFSET>,
            Stop: Stop::<Identity, OFFSET>,
            IsStopped: IsStopped::<Identity, OFFSET>,
            GetFileDuration: GetFileDuration::<Identity, OFFSET>,
            GetFileSize: GetFileSize::<Identity, OFFSET>,
            Close: Close::<Identity, OFFSET>,
            IsClosed: IsClosed::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMWriterFileSink2 as windows_core::Interface>::IID || iid == &<IWMWriterSink as windows_core::Interface>::IID || iid == &<IWMWriterFileSink as windows_core::Interface>::IID
    }
}
pub trait IWMWriterFileSink3_Impl: Sized + IWMWriterFileSink2_Impl {
    fn SetAutoIndexing(&self, fdoautoindexing: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn GetAutoIndexing(&self) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn SetControlStream(&self, wstreamnumber: u16, fshouldcontrolstartandstop: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn GetMode(&self) -> windows_core::Result<u32>;
    fn OnDataUnitEx(&self, pfilesinkdataunit: *const WMT_FILESINK_DATA_UNIT) -> windows_core::Result<()>;
    fn SetUnbufferedIO(&self, funbufferedio: super::super::Foundation::BOOL, frestrictmemusage: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn GetUnbufferedIO(&self) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn CompleteOperations(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWMWriterFileSink3 {}
impl IWMWriterFileSink3_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWMWriterFileSink3_Vtbl
    where
        Identity: IWMWriterFileSink3_Impl,
    {
        unsafe extern "system" fn SetAutoIndexing<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fdoautoindexing: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IWMWriterFileSink3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMWriterFileSink3_Impl::SetAutoIndexing(this, core::mem::transmute_copy(&fdoautoindexing)).into()
        }
        unsafe extern "system" fn GetAutoIndexing<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfautoindexing: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IWMWriterFileSink3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMWriterFileSink3_Impl::GetAutoIndexing(this) {
                Ok(ok__) => {
                    pfautoindexing.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetControlStream<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, wstreamnumber: u16, fshouldcontrolstartandstop: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IWMWriterFileSink3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMWriterFileSink3_Impl::SetControlStream(this, core::mem::transmute_copy(&wstreamnumber), core::mem::transmute_copy(&fshouldcontrolstartandstop)).into()
        }
        unsafe extern "system" fn GetMode<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwfilesinkmode: *mut u32) -> windows_core::HRESULT
        where
            Identity: IWMWriterFileSink3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMWriterFileSink3_Impl::GetMode(this) {
                Ok(ok__) => {
                    pdwfilesinkmode.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnDataUnitEx<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfilesinkdataunit: *const WMT_FILESINK_DATA_UNIT) -> windows_core::HRESULT
        where
            Identity: IWMWriterFileSink3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMWriterFileSink3_Impl::OnDataUnitEx(this, core::mem::transmute_copy(&pfilesinkdataunit)).into()
        }
        unsafe extern "system" fn SetUnbufferedIO<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, funbufferedio: super::super::Foundation::BOOL, frestrictmemusage: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IWMWriterFileSink3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMWriterFileSink3_Impl::SetUnbufferedIO(this, core::mem::transmute_copy(&funbufferedio), core::mem::transmute_copy(&frestrictmemusage)).into()
        }
        unsafe extern "system" fn GetUnbufferedIO<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfunbufferedio: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IWMWriterFileSink3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMWriterFileSink3_Impl::GetUnbufferedIO(this) {
                Ok(ok__) => {
                    pfunbufferedio.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CompleteOperations<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMWriterFileSink3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMWriterFileSink3_Impl::CompleteOperations(this).into()
        }
        Self {
            base__: IWMWriterFileSink2_Vtbl::new::<Identity, OFFSET>(),
            SetAutoIndexing: SetAutoIndexing::<Identity, OFFSET>,
            GetAutoIndexing: GetAutoIndexing::<Identity, OFFSET>,
            SetControlStream: SetControlStream::<Identity, OFFSET>,
            GetMode: GetMode::<Identity, OFFSET>,
            OnDataUnitEx: OnDataUnitEx::<Identity, OFFSET>,
            SetUnbufferedIO: SetUnbufferedIO::<Identity, OFFSET>,
            GetUnbufferedIO: GetUnbufferedIO::<Identity, OFFSET>,
            CompleteOperations: CompleteOperations::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMWriterFileSink3 as windows_core::Interface>::IID || iid == &<IWMWriterSink as windows_core::Interface>::IID || iid == &<IWMWriterFileSink as windows_core::Interface>::IID || iid == &<IWMWriterFileSink2 as windows_core::Interface>::IID
    }
}
pub trait IWMWriterNetworkSink_Impl: Sized + IWMWriterSink_Impl {
    fn SetMaximumClients(&self, dwmaxclients: u32) -> windows_core::Result<()>;
    fn GetMaximumClients(&self) -> windows_core::Result<u32>;
    fn SetNetworkProtocol(&self, protocol: WMT_NET_PROTOCOL) -> windows_core::Result<()>;
    fn GetNetworkProtocol(&self) -> windows_core::Result<WMT_NET_PROTOCOL>;
    fn GetHostURL(&self, pwszurl: windows_core::PWSTR, pcchurl: *mut u32) -> windows_core::Result<()>;
    fn Open(&self, pdwportnum: *mut u32) -> windows_core::Result<()>;
    fn Disconnect(&self) -> windows_core::Result<()>;
    fn Close(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWMWriterNetworkSink {}
impl IWMWriterNetworkSink_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWMWriterNetworkSink_Vtbl
    where
        Identity: IWMWriterNetworkSink_Impl,
    {
        unsafe extern "system" fn SetMaximumClients<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwmaxclients: u32) -> windows_core::HRESULT
        where
            Identity: IWMWriterNetworkSink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMWriterNetworkSink_Impl::SetMaximumClients(this, core::mem::transmute_copy(&dwmaxclients)).into()
        }
        unsafe extern "system" fn GetMaximumClients<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwmaxclients: *mut u32) -> windows_core::HRESULT
        where
            Identity: IWMWriterNetworkSink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMWriterNetworkSink_Impl::GetMaximumClients(this) {
                Ok(ok__) => {
                    pdwmaxclients.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNetworkProtocol<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, protocol: WMT_NET_PROTOCOL) -> windows_core::HRESULT
        where
            Identity: IWMWriterNetworkSink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMWriterNetworkSink_Impl::SetNetworkProtocol(this, core::mem::transmute_copy(&protocol)).into()
        }
        unsafe extern "system" fn GetNetworkProtocol<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pprotocol: *mut WMT_NET_PROTOCOL) -> windows_core::HRESULT
        where
            Identity: IWMWriterNetworkSink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMWriterNetworkSink_Impl::GetNetworkProtocol(this) {
                Ok(ok__) => {
                    pprotocol.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetHostURL<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszurl: windows_core::PWSTR, pcchurl: *mut u32) -> windows_core::HRESULT
        where
            Identity: IWMWriterNetworkSink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMWriterNetworkSink_Impl::GetHostURL(this, core::mem::transmute_copy(&pwszurl), core::mem::transmute_copy(&pcchurl)).into()
        }
        unsafe extern "system" fn Open<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwportnum: *mut u32) -> windows_core::HRESULT
        where
            Identity: IWMWriterNetworkSink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMWriterNetworkSink_Impl::Open(this, core::mem::transmute_copy(&pdwportnum)).into()
        }
        unsafe extern "system" fn Disconnect<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMWriterNetworkSink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMWriterNetworkSink_Impl::Disconnect(this).into()
        }
        unsafe extern "system" fn Close<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMWriterNetworkSink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMWriterNetworkSink_Impl::Close(this).into()
        }
        Self {
            base__: IWMWriterSink_Vtbl::new::<Identity, OFFSET>(),
            SetMaximumClients: SetMaximumClients::<Identity, OFFSET>,
            GetMaximumClients: GetMaximumClients::<Identity, OFFSET>,
            SetNetworkProtocol: SetNetworkProtocol::<Identity, OFFSET>,
            GetNetworkProtocol: GetNetworkProtocol::<Identity, OFFSET>,
            GetHostURL: GetHostURL::<Identity, OFFSET>,
            Open: Open::<Identity, OFFSET>,
            Disconnect: Disconnect::<Identity, OFFSET>,
            Close: Close::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMWriterNetworkSink as windows_core::Interface>::IID || iid == &<IWMWriterSink as windows_core::Interface>::IID
    }
}
pub trait IWMWriterPostView_Impl: Sized {
    fn SetPostViewCallback(&self, pcallback: Option<&IWMWriterPostViewCallback>, pvcontext: *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn SetReceivePostViewSamples(&self, wstreamnum: u16, freceivepostviewsamples: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn GetReceivePostViewSamples(&self, wstreamnum: u16) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn GetPostViewProps(&self, wstreamnumber: u16) -> windows_core::Result<IWMMediaProps>;
    fn SetPostViewProps(&self, wstreamnumber: u16, poutput: Option<&IWMMediaProps>) -> windows_core::Result<()>;
    fn GetPostViewFormatCount(&self, wstreamnumber: u16) -> windows_core::Result<u32>;
    fn GetPostViewFormat(&self, wstreamnumber: u16, dwformatnumber: u32) -> windows_core::Result<IWMMediaProps>;
    fn SetAllocateForPostView(&self, wstreamnumber: u16, fallocate: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn GetAllocateForPostView(&self, wstreamnumber: u16) -> windows_core::Result<super::super::Foundation::BOOL>;
}
impl windows_core::RuntimeName for IWMWriterPostView {}
impl IWMWriterPostView_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWMWriterPostView_Vtbl
    where
        Identity: IWMWriterPostView_Impl,
    {
        unsafe extern "system" fn SetPostViewCallback<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcallback: *mut core::ffi::c_void, pvcontext: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMWriterPostView_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMWriterPostView_Impl::SetPostViewCallback(this, windows_core::from_raw_borrowed(&pcallback), core::mem::transmute_copy(&pvcontext)).into()
        }
        unsafe extern "system" fn SetReceivePostViewSamples<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, wstreamnum: u16, freceivepostviewsamples: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IWMWriterPostView_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMWriterPostView_Impl::SetReceivePostViewSamples(this, core::mem::transmute_copy(&wstreamnum), core::mem::transmute_copy(&freceivepostviewsamples)).into()
        }
        unsafe extern "system" fn GetReceivePostViewSamples<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, wstreamnum: u16, pfreceivepostviewsamples: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IWMWriterPostView_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMWriterPostView_Impl::GetReceivePostViewSamples(this, core::mem::transmute_copy(&wstreamnum)) {
                Ok(ok__) => {
                    pfreceivepostviewsamples.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPostViewProps<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, wstreamnumber: u16, ppoutput: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMWriterPostView_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMWriterPostView_Impl::GetPostViewProps(this, core::mem::transmute_copy(&wstreamnumber)) {
                Ok(ok__) => {
                    ppoutput.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPostViewProps<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, wstreamnumber: u16, poutput: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMWriterPostView_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMWriterPostView_Impl::SetPostViewProps(this, core::mem::transmute_copy(&wstreamnumber), windows_core::from_raw_borrowed(&poutput)).into()
        }
        unsafe extern "system" fn GetPostViewFormatCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, wstreamnumber: u16, pcformats: *mut u32) -> windows_core::HRESULT
        where
            Identity: IWMWriterPostView_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMWriterPostView_Impl::GetPostViewFormatCount(this, core::mem::transmute_copy(&wstreamnumber)) {
                Ok(ok__) => {
                    pcformats.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPostViewFormat<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, wstreamnumber: u16, dwformatnumber: u32, ppprops: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMWriterPostView_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMWriterPostView_Impl::GetPostViewFormat(this, core::mem::transmute_copy(&wstreamnumber), core::mem::transmute_copy(&dwformatnumber)) {
                Ok(ok__) => {
                    ppprops.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAllocateForPostView<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, wstreamnumber: u16, fallocate: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IWMWriterPostView_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMWriterPostView_Impl::SetAllocateForPostView(this, core::mem::transmute_copy(&wstreamnumber), core::mem::transmute_copy(&fallocate)).into()
        }
        unsafe extern "system" fn GetAllocateForPostView<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, wstreamnumber: u16, pfallocate: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IWMWriterPostView_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMWriterPostView_Impl::GetAllocateForPostView(this, core::mem::transmute_copy(&wstreamnumber)) {
                Ok(ok__) => {
                    pfallocate.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetPostViewCallback: SetPostViewCallback::<Identity, OFFSET>,
            SetReceivePostViewSamples: SetReceivePostViewSamples::<Identity, OFFSET>,
            GetReceivePostViewSamples: GetReceivePostViewSamples::<Identity, OFFSET>,
            GetPostViewProps: GetPostViewProps::<Identity, OFFSET>,
            SetPostViewProps: SetPostViewProps::<Identity, OFFSET>,
            GetPostViewFormatCount: GetPostViewFormatCount::<Identity, OFFSET>,
            GetPostViewFormat: GetPostViewFormat::<Identity, OFFSET>,
            SetAllocateForPostView: SetAllocateForPostView::<Identity, OFFSET>,
            GetAllocateForPostView: GetAllocateForPostView::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMWriterPostView as windows_core::Interface>::IID
    }
}
pub trait IWMWriterPostViewCallback_Impl: Sized + IWMStatusCallback_Impl {
    fn OnPostViewSample(&self, wstreamnumber: u16, cnssampletime: u64, cnssampleduration: u64, dwflags: u32, psample: Option<&INSSBuffer>, pvcontext: *const core::ffi::c_void) -> windows_core::Result<()>;
    fn AllocateForPostView(&self, wstreamnum: u16, cbbuffer: u32, ppbuffer: *mut Option<INSSBuffer>, pvcontext: *const core::ffi::c_void) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWMWriterPostViewCallback {}
impl IWMWriterPostViewCallback_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWMWriterPostViewCallback_Vtbl
    where
        Identity: IWMWriterPostViewCallback_Impl,
    {
        unsafe extern "system" fn OnPostViewSample<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, wstreamnumber: u16, cnssampletime: u64, cnssampleduration: u64, dwflags: u32, psample: *mut core::ffi::c_void, pvcontext: *const core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMWriterPostViewCallback_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMWriterPostViewCallback_Impl::OnPostViewSample(this, core::mem::transmute_copy(&wstreamnumber), core::mem::transmute_copy(&cnssampletime), core::mem::transmute_copy(&cnssampleduration), core::mem::transmute_copy(&dwflags), windows_core::from_raw_borrowed(&psample), core::mem::transmute_copy(&pvcontext)).into()
        }
        unsafe extern "system" fn AllocateForPostView<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, wstreamnum: u16, cbbuffer: u32, ppbuffer: *mut *mut core::ffi::c_void, pvcontext: *const core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMWriterPostViewCallback_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMWriterPostViewCallback_Impl::AllocateForPostView(this, core::mem::transmute_copy(&wstreamnum), core::mem::transmute_copy(&cbbuffer), core::mem::transmute_copy(&ppbuffer), core::mem::transmute_copy(&pvcontext)).into()
        }
        Self {
            base__: IWMStatusCallback_Vtbl::new::<Identity, OFFSET>(),
            OnPostViewSample: OnPostViewSample::<Identity, OFFSET>,
            AllocateForPostView: AllocateForPostView::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMWriterPostViewCallback as windows_core::Interface>::IID || iid == &<IWMStatusCallback as windows_core::Interface>::IID
    }
}
pub trait IWMWriterPreprocess_Impl: Sized {
    fn GetMaxPreprocessingPasses(&self, dwinputnum: u32, dwflags: u32) -> windows_core::Result<u32>;
    fn SetNumPreprocessingPasses(&self, dwinputnum: u32, dwflags: u32, dwnumpasses: u32) -> windows_core::Result<()>;
    fn BeginPreprocessingPass(&self, dwinputnum: u32, dwflags: u32) -> windows_core::Result<()>;
    fn PreprocessSample(&self, dwinputnum: u32, cnssampletime: u64, dwflags: u32, psample: Option<&INSSBuffer>) -> windows_core::Result<()>;
    fn EndPreprocessingPass(&self, dwinputnum: u32, dwflags: u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWMWriterPreprocess {}
impl IWMWriterPreprocess_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWMWriterPreprocess_Vtbl
    where
        Identity: IWMWriterPreprocess_Impl,
    {
        unsafe extern "system" fn GetMaxPreprocessingPasses<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwinputnum: u32, dwflags: u32, pdwmaxnumpasses: *mut u32) -> windows_core::HRESULT
        where
            Identity: IWMWriterPreprocess_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMWriterPreprocess_Impl::GetMaxPreprocessingPasses(this, core::mem::transmute_copy(&dwinputnum), core::mem::transmute_copy(&dwflags)) {
                Ok(ok__) => {
                    pdwmaxnumpasses.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNumPreprocessingPasses<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwinputnum: u32, dwflags: u32, dwnumpasses: u32) -> windows_core::HRESULT
        where
            Identity: IWMWriterPreprocess_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMWriterPreprocess_Impl::SetNumPreprocessingPasses(this, core::mem::transmute_copy(&dwinputnum), core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&dwnumpasses)).into()
        }
        unsafe extern "system" fn BeginPreprocessingPass<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwinputnum: u32, dwflags: u32) -> windows_core::HRESULT
        where
            Identity: IWMWriterPreprocess_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMWriterPreprocess_Impl::BeginPreprocessingPass(this, core::mem::transmute_copy(&dwinputnum), core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn PreprocessSample<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwinputnum: u32, cnssampletime: u64, dwflags: u32, psample: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMWriterPreprocess_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMWriterPreprocess_Impl::PreprocessSample(this, core::mem::transmute_copy(&dwinputnum), core::mem::transmute_copy(&cnssampletime), core::mem::transmute_copy(&dwflags), windows_core::from_raw_borrowed(&psample)).into()
        }
        unsafe extern "system" fn EndPreprocessingPass<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwinputnum: u32, dwflags: u32) -> windows_core::HRESULT
        where
            Identity: IWMWriterPreprocess_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMWriterPreprocess_Impl::EndPreprocessingPass(this, core::mem::transmute_copy(&dwinputnum), core::mem::transmute_copy(&dwflags)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetMaxPreprocessingPasses: GetMaxPreprocessingPasses::<Identity, OFFSET>,
            SetNumPreprocessingPasses: SetNumPreprocessingPasses::<Identity, OFFSET>,
            BeginPreprocessingPass: BeginPreprocessingPass::<Identity, OFFSET>,
            PreprocessSample: PreprocessSample::<Identity, OFFSET>,
            EndPreprocessingPass: EndPreprocessingPass::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMWriterPreprocess as windows_core::Interface>::IID
    }
}
pub trait IWMWriterPushSink_Impl: Sized + IWMWriterSink_Impl {
    fn Connect(&self, pwszurl: &windows_core::PCWSTR, pwsztemplateurl: &windows_core::PCWSTR, fautodestroy: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn Disconnect(&self) -> windows_core::Result<()>;
    fn EndSession(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWMWriterPushSink {}
impl IWMWriterPushSink_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWMWriterPushSink_Vtbl
    where
        Identity: IWMWriterPushSink_Impl,
    {
        unsafe extern "system" fn Connect<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszurl: windows_core::PCWSTR, pwsztemplateurl: windows_core::PCWSTR, fautodestroy: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IWMWriterPushSink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMWriterPushSink_Impl::Connect(this, core::mem::transmute(&pwszurl), core::mem::transmute(&pwsztemplateurl), core::mem::transmute_copy(&fautodestroy)).into()
        }
        unsafe extern "system" fn Disconnect<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMWriterPushSink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMWriterPushSink_Impl::Disconnect(this).into()
        }
        unsafe extern "system" fn EndSession<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMWriterPushSink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMWriterPushSink_Impl::EndSession(this).into()
        }
        Self {
            base__: IWMWriterSink_Vtbl::new::<Identity, OFFSET>(),
            Connect: Connect::<Identity, OFFSET>,
            Disconnect: Disconnect::<Identity, OFFSET>,
            EndSession: EndSession::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMWriterPushSink as windows_core::Interface>::IID || iid == &<IWMWriterSink as windows_core::Interface>::IID
    }
}
pub trait IWMWriterSink_Impl: Sized {
    fn OnHeader(&self, pheader: Option<&INSSBuffer>) -> windows_core::Result<()>;
    fn IsRealTime(&self) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn AllocateDataUnit(&self, cbdataunit: u32) -> windows_core::Result<INSSBuffer>;
    fn OnDataUnit(&self, pdataunit: Option<&INSSBuffer>) -> windows_core::Result<()>;
    fn OnEndWriting(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWMWriterSink {}
impl IWMWriterSink_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWMWriterSink_Vtbl
    where
        Identity: IWMWriterSink_Impl,
    {
        unsafe extern "system" fn OnHeader<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pheader: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMWriterSink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMWriterSink_Impl::OnHeader(this, windows_core::from_raw_borrowed(&pheader)).into()
        }
        unsafe extern "system" fn IsRealTime<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfrealtime: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IWMWriterSink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMWriterSink_Impl::IsRealTime(this) {
                Ok(ok__) => {
                    pfrealtime.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AllocateDataUnit<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, cbdataunit: u32, ppdataunit: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMWriterSink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMWriterSink_Impl::AllocateDataUnit(this, core::mem::transmute_copy(&cbdataunit)) {
                Ok(ok__) => {
                    ppdataunit.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnDataUnit<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdataunit: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMWriterSink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMWriterSink_Impl::OnDataUnit(this, windows_core::from_raw_borrowed(&pdataunit)).into()
        }
        unsafe extern "system" fn OnEndWriting<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMWriterSink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMWriterSink_Impl::OnEndWriting(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnHeader: OnHeader::<Identity, OFFSET>,
            IsRealTime: IsRealTime::<Identity, OFFSET>,
            AllocateDataUnit: AllocateDataUnit::<Identity, OFFSET>,
            OnDataUnit: OnDataUnit::<Identity, OFFSET>,
            OnEndWriting: OnEndWriting::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMWriterSink as windows_core::Interface>::IID
    }
}
