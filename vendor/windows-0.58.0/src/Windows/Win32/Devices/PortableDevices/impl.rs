pub trait IConnectionRequestCallback_Impl: Sized {
    fn OnComplete(&self, hrstatus: windows_core::HRESULT) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IConnectionRequestCallback {}
impl IConnectionRequestCallback_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IConnectionRequestCallback_Vtbl
    where
        Identity: IConnectionRequestCallback_Impl,
    {
        unsafe extern "system" fn OnComplete<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hrstatus: windows_core::HRESULT) -> windows_core::HRESULT
        where
            Identity: IConnectionRequestCallback_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IConnectionRequestCallback_Impl::OnComplete(this, core::mem::transmute_copy(&hrstatus)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnComplete: OnComplete::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IConnectionRequestCallback as windows_core::Interface>::IID
    }
}
pub trait IEnumPortableDeviceConnectors_Impl: Sized {
    fn Next(&self, crequested: u32, pconnectors: *mut Option<IPortableDeviceConnector>, pcfetched: *mut u32) -> windows_core::Result<()>;
    fn Skip(&self, cconnectors: u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IEnumPortableDeviceConnectors>;
}
impl windows_core::RuntimeName for IEnumPortableDeviceConnectors {}
impl IEnumPortableDeviceConnectors_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IEnumPortableDeviceConnectors_Vtbl
    where
        Identity: IEnumPortableDeviceConnectors_Impl,
    {
        unsafe extern "system" fn Next<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, crequested: u32, pconnectors: *mut *mut core::ffi::c_void, pcfetched: *mut u32) -> windows_core::HRESULT
        where
            Identity: IEnumPortableDeviceConnectors_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumPortableDeviceConnectors_Impl::Next(this, core::mem::transmute_copy(&crequested), core::mem::transmute_copy(&pconnectors), core::mem::transmute_copy(&pcfetched)).into()
        }
        unsafe extern "system" fn Skip<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, cconnectors: u32) -> windows_core::HRESULT
        where
            Identity: IEnumPortableDeviceConnectors_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumPortableDeviceConnectors_Impl::Skip(this, core::mem::transmute_copy(&cconnectors)).into()
        }
        unsafe extern "system" fn Reset<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IEnumPortableDeviceConnectors_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumPortableDeviceConnectors_Impl::Reset(this).into()
        }
        unsafe extern "system" fn Clone<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IEnumPortableDeviceConnectors_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEnumPortableDeviceConnectors_Impl::Clone(this) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
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
        iid == &<IEnumPortableDeviceConnectors as windows_core::Interface>::IID
    }
}
pub trait IEnumPortableDeviceObjectIDs_Impl: Sized {
    fn Next(&self, cobjects: u32, pobjids: *mut windows_core::PWSTR, pcfetched: *mut u32) -> windows_core::HRESULT;
    fn Skip(&self, cobjects: u32) -> windows_core::HRESULT;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IEnumPortableDeviceObjectIDs>;
    fn Cancel(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IEnumPortableDeviceObjectIDs {}
impl IEnumPortableDeviceObjectIDs_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IEnumPortableDeviceObjectIDs_Vtbl
    where
        Identity: IEnumPortableDeviceObjectIDs_Impl,
    {
        unsafe extern "system" fn Next<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, cobjects: u32, pobjids: *mut windows_core::PWSTR, pcfetched: *mut u32) -> windows_core::HRESULT
        where
            Identity: IEnumPortableDeviceObjectIDs_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumPortableDeviceObjectIDs_Impl::Next(this, core::mem::transmute_copy(&cobjects), core::mem::transmute_copy(&pobjids), core::mem::transmute_copy(&pcfetched))
        }
        unsafe extern "system" fn Skip<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, cobjects: u32) -> windows_core::HRESULT
        where
            Identity: IEnumPortableDeviceObjectIDs_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumPortableDeviceObjectIDs_Impl::Skip(this, core::mem::transmute_copy(&cobjects))
        }
        unsafe extern "system" fn Reset<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IEnumPortableDeviceObjectIDs_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumPortableDeviceObjectIDs_Impl::Reset(this).into()
        }
        unsafe extern "system" fn Clone<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IEnumPortableDeviceObjectIDs_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEnumPortableDeviceObjectIDs_Impl::Clone(this) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Cancel<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IEnumPortableDeviceObjectIDs_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumPortableDeviceObjectIDs_Impl::Cancel(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, OFFSET>,
            Skip: Skip::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
            Cancel: Cancel::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnumPortableDeviceObjectIDs as windows_core::Interface>::IID
    }
}
pub trait IMediaRadioManager_Impl: Sized {
    fn GetRadioInstances(&self) -> windows_core::Result<IRadioInstanceCollection>;
    fn OnSystemRadioStateChange(&self, sysradiostate: SYSTEM_RADIO_STATE, utimeoutsec: u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IMediaRadioManager {}
impl IMediaRadioManager_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IMediaRadioManager_Vtbl
    where
        Identity: IMediaRadioManager_Impl,
    {
        unsafe extern "system" fn GetRadioInstances<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcollection: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMediaRadioManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMediaRadioManager_Impl::GetRadioInstances(this) {
                Ok(ok__) => {
                    ppcollection.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnSystemRadioStateChange<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, sysradiostate: SYSTEM_RADIO_STATE, utimeoutsec: u32) -> windows_core::HRESULT
        where
            Identity: IMediaRadioManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMediaRadioManager_Impl::OnSystemRadioStateChange(this, core::mem::transmute_copy(&sysradiostate), core::mem::transmute_copy(&utimeoutsec)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetRadioInstances: GetRadioInstances::<Identity, OFFSET>,
            OnSystemRadioStateChange: OnSystemRadioStateChange::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMediaRadioManager as windows_core::Interface>::IID
    }
}
pub trait IMediaRadioManagerNotifySink_Impl: Sized {
    fn OnInstanceAdd(&self, pradioinstance: Option<&IRadioInstance>) -> windows_core::Result<()>;
    fn OnInstanceRemove(&self, bstrradioinstanceid: &windows_core::BSTR) -> windows_core::Result<()>;
    fn OnInstanceRadioChange(&self, bstrradioinstanceid: &windows_core::BSTR, radiostate: DEVICE_RADIO_STATE) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IMediaRadioManagerNotifySink {}
impl IMediaRadioManagerNotifySink_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IMediaRadioManagerNotifySink_Vtbl
    where
        Identity: IMediaRadioManagerNotifySink_Impl,
    {
        unsafe extern "system" fn OnInstanceAdd<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pradioinstance: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMediaRadioManagerNotifySink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMediaRadioManagerNotifySink_Impl::OnInstanceAdd(this, windows_core::from_raw_borrowed(&pradioinstance)).into()
        }
        unsafe extern "system" fn OnInstanceRemove<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrradioinstanceid: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IMediaRadioManagerNotifySink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMediaRadioManagerNotifySink_Impl::OnInstanceRemove(this, core::mem::transmute(&bstrradioinstanceid)).into()
        }
        unsafe extern "system" fn OnInstanceRadioChange<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrradioinstanceid: core::mem::MaybeUninit<windows_core::BSTR>, radiostate: DEVICE_RADIO_STATE) -> windows_core::HRESULT
        where
            Identity: IMediaRadioManagerNotifySink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMediaRadioManagerNotifySink_Impl::OnInstanceRadioChange(this, core::mem::transmute(&bstrradioinstanceid), core::mem::transmute_copy(&radiostate)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnInstanceAdd: OnInstanceAdd::<Identity, OFFSET>,
            OnInstanceRemove: OnInstanceRemove::<Identity, OFFSET>,
            OnInstanceRadioChange: OnInstanceRadioChange::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMediaRadioManagerNotifySink as windows_core::Interface>::IID
    }
}
pub trait IPortableDevice_Impl: Sized {
    fn Open(&self, pszpnpdeviceid: &windows_core::PCWSTR, pclientinfo: Option<&IPortableDeviceValues>) -> windows_core::Result<()>;
    fn SendCommand(&self, dwflags: u32, pparameters: Option<&IPortableDeviceValues>) -> windows_core::Result<IPortableDeviceValues>;
    fn Content(&self) -> windows_core::Result<IPortableDeviceContent>;
    fn Capabilities(&self) -> windows_core::Result<IPortableDeviceCapabilities>;
    fn Cancel(&self) -> windows_core::Result<()>;
    fn Close(&self) -> windows_core::Result<()>;
    fn Advise(&self, dwflags: u32, pcallback: Option<&IPortableDeviceEventCallback>, pparameters: Option<&IPortableDeviceValues>) -> windows_core::Result<windows_core::PWSTR>;
    fn Unadvise(&self, pszcookie: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetPnPDeviceID(&self) -> windows_core::Result<windows_core::PWSTR>;
}
impl windows_core::RuntimeName for IPortableDevice {}
impl IPortableDevice_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IPortableDevice_Vtbl
    where
        Identity: IPortableDevice_Impl,
    {
        unsafe extern "system" fn Open<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszpnpdeviceid: windows_core::PCWSTR, pclientinfo: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPortableDevice_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPortableDevice_Impl::Open(this, core::mem::transmute(&pszpnpdeviceid), windows_core::from_raw_borrowed(&pclientinfo)).into()
        }
        unsafe extern "system" fn SendCommand<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: u32, pparameters: *mut core::ffi::c_void, ppresults: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPortableDevice_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPortableDevice_Impl::SendCommand(this, core::mem::transmute_copy(&dwflags), windows_core::from_raw_borrowed(&pparameters)) {
                Ok(ok__) => {
                    ppresults.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Content<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcontent: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPortableDevice_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPortableDevice_Impl::Content(this) {
                Ok(ok__) => {
                    ppcontent.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Capabilities<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcapabilities: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPortableDevice_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPortableDevice_Impl::Capabilities(this) {
                Ok(ok__) => {
                    ppcapabilities.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Cancel<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPortableDevice_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPortableDevice_Impl::Cancel(this).into()
        }
        unsafe extern "system" fn Close<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPortableDevice_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPortableDevice_Impl::Close(this).into()
        }
        unsafe extern "system" fn Advise<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: u32, pcallback: *mut core::ffi::c_void, pparameters: *mut core::ffi::c_void, ppszcookie: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IPortableDevice_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPortableDevice_Impl::Advise(this, core::mem::transmute_copy(&dwflags), windows_core::from_raw_borrowed(&pcallback), windows_core::from_raw_borrowed(&pparameters)) {
                Ok(ok__) => {
                    ppszcookie.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Unadvise<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszcookie: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IPortableDevice_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPortableDevice_Impl::Unadvise(this, core::mem::transmute(&pszcookie)).into()
        }
        unsafe extern "system" fn GetPnPDeviceID<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppszpnpdeviceid: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IPortableDevice_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPortableDevice_Impl::GetPnPDeviceID(this) {
                Ok(ok__) => {
                    ppszpnpdeviceid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Open: Open::<Identity, OFFSET>,
            SendCommand: SendCommand::<Identity, OFFSET>,
            Content: Content::<Identity, OFFSET>,
            Capabilities: Capabilities::<Identity, OFFSET>,
            Cancel: Cancel::<Identity, OFFSET>,
            Close: Close::<Identity, OFFSET>,
            Advise: Advise::<Identity, OFFSET>,
            Unadvise: Unadvise::<Identity, OFFSET>,
            GetPnPDeviceID: GetPnPDeviceID::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPortableDevice as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub trait IPortableDeviceCapabilities_Impl: Sized {
    fn GetSupportedCommands(&self) -> windows_core::Result<IPortableDeviceKeyCollection>;
    fn GetCommandOptions(&self, command: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> windows_core::Result<IPortableDeviceValues>;
    fn GetFunctionalCategories(&self) -> windows_core::Result<IPortableDevicePropVariantCollection>;
    fn GetFunctionalObjects(&self, category: *const windows_core::GUID) -> windows_core::Result<IPortableDevicePropVariantCollection>;
    fn GetSupportedContentTypes(&self, category: *const windows_core::GUID) -> windows_core::Result<IPortableDevicePropVariantCollection>;
    fn GetSupportedFormats(&self, contenttype: *const windows_core::GUID) -> windows_core::Result<IPortableDevicePropVariantCollection>;
    fn GetSupportedFormatProperties(&self, format: *const windows_core::GUID) -> windows_core::Result<IPortableDeviceKeyCollection>;
    fn GetFixedPropertyAttributes(&self, format: *const windows_core::GUID, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> windows_core::Result<IPortableDeviceValues>;
    fn Cancel(&self) -> windows_core::Result<()>;
    fn GetSupportedEvents(&self) -> windows_core::Result<IPortableDevicePropVariantCollection>;
    fn GetEventOptions(&self, event: *const windows_core::GUID) -> windows_core::Result<IPortableDeviceValues>;
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl windows_core::RuntimeName for IPortableDeviceCapabilities {}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl IPortableDeviceCapabilities_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IPortableDeviceCapabilities_Vtbl
    where
        Identity: IPortableDeviceCapabilities_Impl,
    {
        unsafe extern "system" fn GetSupportedCommands<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcommands: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPortableDeviceCapabilities_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPortableDeviceCapabilities_Impl::GetSupportedCommands(this) {
                Ok(ok__) => {
                    ppcommands.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCommandOptions<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, command: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, ppoptions: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPortableDeviceCapabilities_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPortableDeviceCapabilities_Impl::GetCommandOptions(this, core::mem::transmute_copy(&command)) {
                Ok(ok__) => {
                    ppoptions.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFunctionalCategories<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcategories: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPortableDeviceCapabilities_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPortableDeviceCapabilities_Impl::GetFunctionalCategories(this) {
                Ok(ok__) => {
                    ppcategories.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFunctionalObjects<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, category: *const windows_core::GUID, ppobjectids: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPortableDeviceCapabilities_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPortableDeviceCapabilities_Impl::GetFunctionalObjects(this, core::mem::transmute_copy(&category)) {
                Ok(ok__) => {
                    ppobjectids.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSupportedContentTypes<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, category: *const windows_core::GUID, ppcontenttypes: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPortableDeviceCapabilities_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPortableDeviceCapabilities_Impl::GetSupportedContentTypes(this, core::mem::transmute_copy(&category)) {
                Ok(ok__) => {
                    ppcontenttypes.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSupportedFormats<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, contenttype: *const windows_core::GUID, ppformats: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPortableDeviceCapabilities_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPortableDeviceCapabilities_Impl::GetSupportedFormats(this, core::mem::transmute_copy(&contenttype)) {
                Ok(ok__) => {
                    ppformats.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSupportedFormatProperties<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, format: *const windows_core::GUID, ppkeys: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPortableDeviceCapabilities_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPortableDeviceCapabilities_Impl::GetSupportedFormatProperties(this, core::mem::transmute_copy(&format)) {
                Ok(ok__) => {
                    ppkeys.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFixedPropertyAttributes<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, format: *const windows_core::GUID, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, ppattributes: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPortableDeviceCapabilities_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPortableDeviceCapabilities_Impl::GetFixedPropertyAttributes(this, core::mem::transmute_copy(&format), core::mem::transmute_copy(&key)) {
                Ok(ok__) => {
                    ppattributes.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Cancel<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPortableDeviceCapabilities_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPortableDeviceCapabilities_Impl::Cancel(this).into()
        }
        unsafe extern "system" fn GetSupportedEvents<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppevents: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPortableDeviceCapabilities_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPortableDeviceCapabilities_Impl::GetSupportedEvents(this) {
                Ok(ok__) => {
                    ppevents.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetEventOptions<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, event: *const windows_core::GUID, ppoptions: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPortableDeviceCapabilities_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPortableDeviceCapabilities_Impl::GetEventOptions(this, core::mem::transmute_copy(&event)) {
                Ok(ok__) => {
                    ppoptions.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetSupportedCommands: GetSupportedCommands::<Identity, OFFSET>,
            GetCommandOptions: GetCommandOptions::<Identity, OFFSET>,
            GetFunctionalCategories: GetFunctionalCategories::<Identity, OFFSET>,
            GetFunctionalObjects: GetFunctionalObjects::<Identity, OFFSET>,
            GetSupportedContentTypes: GetSupportedContentTypes::<Identity, OFFSET>,
            GetSupportedFormats: GetSupportedFormats::<Identity, OFFSET>,
            GetSupportedFormatProperties: GetSupportedFormatProperties::<Identity, OFFSET>,
            GetFixedPropertyAttributes: GetFixedPropertyAttributes::<Identity, OFFSET>,
            Cancel: Cancel::<Identity, OFFSET>,
            GetSupportedEvents: GetSupportedEvents::<Identity, OFFSET>,
            GetEventOptions: GetEventOptions::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPortableDeviceCapabilities as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Devices_Properties")]
pub trait IPortableDeviceConnector_Impl: Sized {
    fn Connect(&self, pcallback: Option<&IConnectionRequestCallback>) -> windows_core::Result<()>;
    fn Disconnect(&self, pcallback: Option<&IConnectionRequestCallback>) -> windows_core::Result<()>;
    fn Cancel(&self, pcallback: Option<&IConnectionRequestCallback>) -> windows_core::Result<()>;
    fn GetProperty(&self, ppropertykey: *const super::Properties::DEVPROPKEY, ppropertytype: *mut super::Properties::DEVPROPTYPE, ppdata: *mut *mut u8, pcbdata: *mut u32) -> windows_core::Result<()>;
    fn SetProperty(&self, ppropertykey: *const super::Properties::DEVPROPKEY, propertytype: super::Properties::DEVPROPTYPE, pdata: *const u8, cbdata: u32) -> windows_core::Result<()>;
    fn GetPnPID(&self) -> windows_core::Result<windows_core::PWSTR>;
}
#[cfg(feature = "Win32_Devices_Properties")]
impl windows_core::RuntimeName for IPortableDeviceConnector {}
#[cfg(feature = "Win32_Devices_Properties")]
impl IPortableDeviceConnector_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IPortableDeviceConnector_Vtbl
    where
        Identity: IPortableDeviceConnector_Impl,
    {
        unsafe extern "system" fn Connect<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcallback: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPortableDeviceConnector_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPortableDeviceConnector_Impl::Connect(this, windows_core::from_raw_borrowed(&pcallback)).into()
        }
        unsafe extern "system" fn Disconnect<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcallback: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPortableDeviceConnector_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPortableDeviceConnector_Impl::Disconnect(this, windows_core::from_raw_borrowed(&pcallback)).into()
        }
        unsafe extern "system" fn Cancel<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcallback: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPortableDeviceConnector_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPortableDeviceConnector_Impl::Cancel(this, windows_core::from_raw_borrowed(&pcallback)).into()
        }
        unsafe extern "system" fn GetProperty<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppropertykey: *const super::Properties::DEVPROPKEY, ppropertytype: *mut super::Properties::DEVPROPTYPE, ppdata: *mut *mut u8, pcbdata: *mut u32) -> windows_core::HRESULT
        where
            Identity: IPortableDeviceConnector_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPortableDeviceConnector_Impl::GetProperty(this, core::mem::transmute_copy(&ppropertykey), core::mem::transmute_copy(&ppropertytype), core::mem::transmute_copy(&ppdata), core::mem::transmute_copy(&pcbdata)).into()
        }
        unsafe extern "system" fn SetProperty<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppropertykey: *const super::Properties::DEVPROPKEY, propertytype: super::Properties::DEVPROPTYPE, pdata: *const u8, cbdata: u32) -> windows_core::HRESULT
        where
            Identity: IPortableDeviceConnector_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPortableDeviceConnector_Impl::SetProperty(this, core::mem::transmute_copy(&ppropertykey), core::mem::transmute_copy(&propertytype), core::mem::transmute_copy(&pdata), core::mem::transmute_copy(&cbdata)).into()
        }
        unsafe extern "system" fn GetPnPID<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppwszpnpid: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IPortableDeviceConnector_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPortableDeviceConnector_Impl::GetPnPID(this) {
                Ok(ok__) => {
                    ppwszpnpid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Connect: Connect::<Identity, OFFSET>,
            Disconnect: Disconnect::<Identity, OFFSET>,
            Cancel: Cancel::<Identity, OFFSET>,
            GetProperty: GetProperty::<Identity, OFFSET>,
            SetProperty: SetProperty::<Identity, OFFSET>,
            GetPnPID: GetPnPID::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPortableDeviceConnector as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IPortableDeviceContent_Impl: Sized {
    fn EnumObjects(&self, dwflags: u32, pszparentobjectid: &windows_core::PCWSTR, pfilter: Option<&IPortableDeviceValues>) -> windows_core::Result<IEnumPortableDeviceObjectIDs>;
    fn Properties(&self) -> windows_core::Result<IPortableDeviceProperties>;
    fn Transfer(&self) -> windows_core::Result<IPortableDeviceResources>;
    fn CreateObjectWithPropertiesOnly(&self, pvalues: Option<&IPortableDeviceValues>, ppszobjectid: *mut windows_core::PWSTR) -> windows_core::Result<()>;
    fn CreateObjectWithPropertiesAndData(&self, pvalues: Option<&IPortableDeviceValues>, ppdata: *mut Option<super::super::System::Com::IStream>, pdwoptimalwritebuffersize: *mut u32, ppszcookie: *mut windows_core::PWSTR) -> windows_core::Result<()>;
    fn Delete(&self, dwoptions: u32, pobjectids: Option<&IPortableDevicePropVariantCollection>, ppresults: *mut Option<IPortableDevicePropVariantCollection>) -> windows_core::Result<()>;
    fn GetObjectIDsFromPersistentUniqueIDs(&self, ppersistentuniqueids: Option<&IPortableDevicePropVariantCollection>) -> windows_core::Result<IPortableDevicePropVariantCollection>;
    fn Cancel(&self) -> windows_core::Result<()>;
    fn Move(&self, pobjectids: Option<&IPortableDevicePropVariantCollection>, pszdestinationfolderobjectid: &windows_core::PCWSTR, ppresults: *mut Option<IPortableDevicePropVariantCollection>) -> windows_core::Result<()>;
    fn Copy(&self, pobjectids: Option<&IPortableDevicePropVariantCollection>, pszdestinationfolderobjectid: &windows_core::PCWSTR, ppresults: *mut Option<IPortableDevicePropVariantCollection>) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IPortableDeviceContent {}
#[cfg(feature = "Win32_System_Com")]
impl IPortableDeviceContent_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IPortableDeviceContent_Vtbl
    where
        Identity: IPortableDeviceContent_Impl,
    {
        unsafe extern "system" fn EnumObjects<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: u32, pszparentobjectid: windows_core::PCWSTR, pfilter: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPortableDeviceContent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPortableDeviceContent_Impl::EnumObjects(this, core::mem::transmute_copy(&dwflags), core::mem::transmute(&pszparentobjectid), windows_core::from_raw_borrowed(&pfilter)) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Properties<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppproperties: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPortableDeviceContent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPortableDeviceContent_Impl::Properties(this) {
                Ok(ok__) => {
                    ppproperties.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Transfer<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppresources: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPortableDeviceContent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPortableDeviceContent_Impl::Transfer(this) {
                Ok(ok__) => {
                    ppresources.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateObjectWithPropertiesOnly<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalues: *mut core::ffi::c_void, ppszobjectid: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IPortableDeviceContent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPortableDeviceContent_Impl::CreateObjectWithPropertiesOnly(this, windows_core::from_raw_borrowed(&pvalues), core::mem::transmute_copy(&ppszobjectid)).into()
        }
        unsafe extern "system" fn CreateObjectWithPropertiesAndData<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalues: *mut core::ffi::c_void, ppdata: *mut *mut core::ffi::c_void, pdwoptimalwritebuffersize: *mut u32, ppszcookie: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IPortableDeviceContent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPortableDeviceContent_Impl::CreateObjectWithPropertiesAndData(this, windows_core::from_raw_borrowed(&pvalues), core::mem::transmute_copy(&ppdata), core::mem::transmute_copy(&pdwoptimalwritebuffersize), core::mem::transmute_copy(&ppszcookie)).into()
        }
        unsafe extern "system" fn Delete<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwoptions: u32, pobjectids: *mut core::ffi::c_void, ppresults: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPortableDeviceContent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPortableDeviceContent_Impl::Delete(this, core::mem::transmute_copy(&dwoptions), windows_core::from_raw_borrowed(&pobjectids), core::mem::transmute_copy(&ppresults)).into()
        }
        unsafe extern "system" fn GetObjectIDsFromPersistentUniqueIDs<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppersistentuniqueids: *mut core::ffi::c_void, ppobjectids: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPortableDeviceContent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPortableDeviceContent_Impl::GetObjectIDsFromPersistentUniqueIDs(this, windows_core::from_raw_borrowed(&ppersistentuniqueids)) {
                Ok(ok__) => {
                    ppobjectids.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Cancel<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPortableDeviceContent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPortableDeviceContent_Impl::Cancel(this).into()
        }
        unsafe extern "system" fn Move<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pobjectids: *mut core::ffi::c_void, pszdestinationfolderobjectid: windows_core::PCWSTR, ppresults: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPortableDeviceContent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPortableDeviceContent_Impl::Move(this, windows_core::from_raw_borrowed(&pobjectids), core::mem::transmute(&pszdestinationfolderobjectid), core::mem::transmute_copy(&ppresults)).into()
        }
        unsafe extern "system" fn Copy<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pobjectids: *mut core::ffi::c_void, pszdestinationfolderobjectid: windows_core::PCWSTR, ppresults: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPortableDeviceContent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPortableDeviceContent_Impl::Copy(this, windows_core::from_raw_borrowed(&pobjectids), core::mem::transmute(&pszdestinationfolderobjectid), core::mem::transmute_copy(&ppresults)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            EnumObjects: EnumObjects::<Identity, OFFSET>,
            Properties: Properties::<Identity, OFFSET>,
            Transfer: Transfer::<Identity, OFFSET>,
            CreateObjectWithPropertiesOnly: CreateObjectWithPropertiesOnly::<Identity, OFFSET>,
            CreateObjectWithPropertiesAndData: CreateObjectWithPropertiesAndData::<Identity, OFFSET>,
            Delete: Delete::<Identity, OFFSET>,
            GetObjectIDsFromPersistentUniqueIDs: GetObjectIDsFromPersistentUniqueIDs::<Identity, OFFSET>,
            Cancel: Cancel::<Identity, OFFSET>,
            Move: Move::<Identity, OFFSET>,
            Copy: Copy::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPortableDeviceContent as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IPortableDeviceContent2_Impl: Sized + IPortableDeviceContent_Impl {
    fn UpdateObjectWithPropertiesAndData(&self, pszobjectid: &windows_core::PCWSTR, pproperties: Option<&IPortableDeviceValues>, ppdata: *mut Option<super::super::System::Com::IStream>, pdwoptimalwritebuffersize: *mut u32) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IPortableDeviceContent2 {}
#[cfg(feature = "Win32_System_Com")]
impl IPortableDeviceContent2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IPortableDeviceContent2_Vtbl
    where
        Identity: IPortableDeviceContent2_Impl,
    {
        unsafe extern "system" fn UpdateObjectWithPropertiesAndData<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszobjectid: windows_core::PCWSTR, pproperties: *mut core::ffi::c_void, ppdata: *mut *mut core::ffi::c_void, pdwoptimalwritebuffersize: *mut u32) -> windows_core::HRESULT
        where
            Identity: IPortableDeviceContent2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPortableDeviceContent2_Impl::UpdateObjectWithPropertiesAndData(this, core::mem::transmute(&pszobjectid), windows_core::from_raw_borrowed(&pproperties), core::mem::transmute_copy(&ppdata), core::mem::transmute_copy(&pdwoptimalwritebuffersize)).into()
        }
        Self {
            base__: IPortableDeviceContent_Vtbl::new::<Identity, OFFSET>(),
            UpdateObjectWithPropertiesAndData: UpdateObjectWithPropertiesAndData::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPortableDeviceContent2 as windows_core::Interface>::IID || iid == &<IPortableDeviceContent as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IPortableDeviceDataStream_Impl: Sized + super::super::System::Com::IStream_Impl {
    fn GetObjectID(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn Cancel(&self) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IPortableDeviceDataStream {}
#[cfg(feature = "Win32_System_Com")]
impl IPortableDeviceDataStream_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IPortableDeviceDataStream_Vtbl
    where
        Identity: IPortableDeviceDataStream_Impl,
    {
        unsafe extern "system" fn GetObjectID<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppszobjectid: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IPortableDeviceDataStream_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPortableDeviceDataStream_Impl::GetObjectID(this) {
                Ok(ok__) => {
                    ppszobjectid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Cancel<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPortableDeviceDataStream_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPortableDeviceDataStream_Impl::Cancel(this).into()
        }
        Self {
            base__: super::super::System::Com::IStream_Vtbl::new::<Identity, OFFSET>(),
            GetObjectID: GetObjectID::<Identity, OFFSET>,
            Cancel: Cancel::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPortableDeviceDataStream as windows_core::Interface>::IID || iid == &<super::super::System::Com::ISequentialStream as windows_core::Interface>::IID || iid == &<super::super::System::Com::IStream as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IPortableDeviceDispatchFactory_Impl: Sized {
    fn GetDeviceDispatch(&self, pszpnpdeviceid: &windows_core::PCWSTR) -> windows_core::Result<super::super::System::Com::IDispatch>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IPortableDeviceDispatchFactory {}
#[cfg(feature = "Win32_System_Com")]
impl IPortableDeviceDispatchFactory_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IPortableDeviceDispatchFactory_Vtbl
    where
        Identity: IPortableDeviceDispatchFactory_Impl,
    {
        unsafe extern "system" fn GetDeviceDispatch<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszpnpdeviceid: windows_core::PCWSTR, ppdevicedispatch: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPortableDeviceDispatchFactory_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPortableDeviceDispatchFactory_Impl::GetDeviceDispatch(this, core::mem::transmute(&pszpnpdeviceid)) {
                Ok(ok__) => {
                    ppdevicedispatch.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetDeviceDispatch: GetDeviceDispatch::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPortableDeviceDispatchFactory as windows_core::Interface>::IID
    }
}
pub trait IPortableDeviceEventCallback_Impl: Sized {
    fn OnEvent(&self, peventparameters: Option<&IPortableDeviceValues>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IPortableDeviceEventCallback {}
impl IPortableDeviceEventCallback_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IPortableDeviceEventCallback_Vtbl
    where
        Identity: IPortableDeviceEventCallback_Impl,
    {
        unsafe extern "system" fn OnEvent<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, peventparameters: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPortableDeviceEventCallback_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPortableDeviceEventCallback_Impl::OnEvent(this, windows_core::from_raw_borrowed(&peventparameters)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnEvent: OnEvent::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPortableDeviceEventCallback as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub trait IPortableDeviceKeyCollection_Impl: Sized {
    fn GetCount(&self, pcelems: *const u32) -> windows_core::Result<()>;
    fn GetAt(&self, dwindex: u32, pkey: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> windows_core::Result<()>;
    fn Add(&self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> windows_core::Result<()>;
    fn Clear(&self) -> windows_core::Result<()>;
    fn RemoveAt(&self, dwindex: u32) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl windows_core::RuntimeName for IPortableDeviceKeyCollection {}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl IPortableDeviceKeyCollection_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IPortableDeviceKeyCollection_Vtbl
    where
        Identity: IPortableDeviceKeyCollection_Impl,
    {
        unsafe extern "system" fn GetCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcelems: *const u32) -> windows_core::HRESULT
        where
            Identity: IPortableDeviceKeyCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPortableDeviceKeyCollection_Impl::GetCount(this, core::mem::transmute_copy(&pcelems)).into()
        }
        unsafe extern "system" fn GetAt<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwindex: u32, pkey: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> windows_core::HRESULT
        where
            Identity: IPortableDeviceKeyCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPortableDeviceKeyCollection_Impl::GetAt(this, core::mem::transmute_copy(&dwindex), core::mem::transmute_copy(&pkey)).into()
        }
        unsafe extern "system" fn Add<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> windows_core::HRESULT
        where
            Identity: IPortableDeviceKeyCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPortableDeviceKeyCollection_Impl::Add(this, core::mem::transmute_copy(&key)).into()
        }
        unsafe extern "system" fn Clear<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPortableDeviceKeyCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPortableDeviceKeyCollection_Impl::Clear(this).into()
        }
        unsafe extern "system" fn RemoveAt<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwindex: u32) -> windows_core::HRESULT
        where
            Identity: IPortableDeviceKeyCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPortableDeviceKeyCollection_Impl::RemoveAt(this, core::mem::transmute_copy(&dwindex)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCount: GetCount::<Identity, OFFSET>,
            GetAt: GetAt::<Identity, OFFSET>,
            Add: Add::<Identity, OFFSET>,
            Clear: Clear::<Identity, OFFSET>,
            RemoveAt: RemoveAt::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPortableDeviceKeyCollection as windows_core::Interface>::IID
    }
}
pub trait IPortableDeviceManager_Impl: Sized {
    fn GetDevices(&self, ppnpdeviceids: *mut windows_core::PWSTR, pcpnpdeviceids: *mut u32) -> windows_core::Result<()>;
    fn RefreshDeviceList(&self) -> windows_core::Result<()>;
    fn GetDeviceFriendlyName(&self, pszpnpdeviceid: &windows_core::PCWSTR, pdevicefriendlyname: &windows_core::PWSTR, pcchdevicefriendlyname: *mut u32) -> windows_core::Result<()>;
    fn GetDeviceDescription(&self, pszpnpdeviceid: &windows_core::PCWSTR, pdevicedescription: &windows_core::PWSTR, pcchdevicedescription: *mut u32) -> windows_core::Result<()>;
    fn GetDeviceManufacturer(&self, pszpnpdeviceid: &windows_core::PCWSTR, pdevicemanufacturer: &windows_core::PWSTR, pcchdevicemanufacturer: *mut u32) -> windows_core::Result<()>;
    fn GetDeviceProperty(&self, pszpnpdeviceid: &windows_core::PCWSTR, pszdevicepropertyname: &windows_core::PCWSTR, pdata: *mut u8, pcbdata: *mut u32, pdwtype: *mut u32) -> windows_core::Result<()>;
    fn GetPrivateDevices(&self, ppnpdeviceids: *mut windows_core::PWSTR, pcpnpdeviceids: *mut u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IPortableDeviceManager {}
impl IPortableDeviceManager_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IPortableDeviceManager_Vtbl
    where
        Identity: IPortableDeviceManager_Impl,
    {
        unsafe extern "system" fn GetDevices<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppnpdeviceids: *mut windows_core::PWSTR, pcpnpdeviceids: *mut u32) -> windows_core::HRESULT
        where
            Identity: IPortableDeviceManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPortableDeviceManager_Impl::GetDevices(this, core::mem::transmute_copy(&ppnpdeviceids), core::mem::transmute_copy(&pcpnpdeviceids)).into()
        }
        unsafe extern "system" fn RefreshDeviceList<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPortableDeviceManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPortableDeviceManager_Impl::RefreshDeviceList(this).into()
        }
        unsafe extern "system" fn GetDeviceFriendlyName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszpnpdeviceid: windows_core::PCWSTR, pdevicefriendlyname: windows_core::PWSTR, pcchdevicefriendlyname: *mut u32) -> windows_core::HRESULT
        where
            Identity: IPortableDeviceManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPortableDeviceManager_Impl::GetDeviceFriendlyName(this, core::mem::transmute(&pszpnpdeviceid), core::mem::transmute(&pdevicefriendlyname), core::mem::transmute_copy(&pcchdevicefriendlyname)).into()
        }
        unsafe extern "system" fn GetDeviceDescription<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszpnpdeviceid: windows_core::PCWSTR, pdevicedescription: windows_core::PWSTR, pcchdevicedescription: *mut u32) -> windows_core::HRESULT
        where
            Identity: IPortableDeviceManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPortableDeviceManager_Impl::GetDeviceDescription(this, core::mem::transmute(&pszpnpdeviceid), core::mem::transmute(&pdevicedescription), core::mem::transmute_copy(&pcchdevicedescription)).into()
        }
        unsafe extern "system" fn GetDeviceManufacturer<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszpnpdeviceid: windows_core::PCWSTR, pdevicemanufacturer: windows_core::PWSTR, pcchdevicemanufacturer: *mut u32) -> windows_core::HRESULT
        where
            Identity: IPortableDeviceManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPortableDeviceManager_Impl::GetDeviceManufacturer(this, core::mem::transmute(&pszpnpdeviceid), core::mem::transmute(&pdevicemanufacturer), core::mem::transmute_copy(&pcchdevicemanufacturer)).into()
        }
        unsafe extern "system" fn GetDeviceProperty<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszpnpdeviceid: windows_core::PCWSTR, pszdevicepropertyname: windows_core::PCWSTR, pdata: *mut u8, pcbdata: *mut u32, pdwtype: *mut u32) -> windows_core::HRESULT
        where
            Identity: IPortableDeviceManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPortableDeviceManager_Impl::GetDeviceProperty(this, core::mem::transmute(&pszpnpdeviceid), core::mem::transmute(&pszdevicepropertyname), core::mem::transmute_copy(&pdata), core::mem::transmute_copy(&pcbdata), core::mem::transmute_copy(&pdwtype)).into()
        }
        unsafe extern "system" fn GetPrivateDevices<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppnpdeviceids: *mut windows_core::PWSTR, pcpnpdeviceids: *mut u32) -> windows_core::HRESULT
        where
            Identity: IPortableDeviceManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPortableDeviceManager_Impl::GetPrivateDevices(this, core::mem::transmute_copy(&ppnpdeviceids), core::mem::transmute_copy(&pcpnpdeviceids)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetDevices: GetDevices::<Identity, OFFSET>,
            RefreshDeviceList: RefreshDeviceList::<Identity, OFFSET>,
            GetDeviceFriendlyName: GetDeviceFriendlyName::<Identity, OFFSET>,
            GetDeviceDescription: GetDeviceDescription::<Identity, OFFSET>,
            GetDeviceManufacturer: GetDeviceManufacturer::<Identity, OFFSET>,
            GetDeviceProperty: GetDeviceProperty::<Identity, OFFSET>,
            GetPrivateDevices: GetPrivateDevices::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPortableDeviceManager as windows_core::Interface>::IID
    }
}
pub trait IPortableDevicePropVariantCollection_Impl: Sized {
    fn GetCount(&self, pcelems: *const u32) -> windows_core::Result<()>;
    fn GetAt(&self, dwindex: u32, pvalue: *const windows_core::PROPVARIANT) -> windows_core::Result<()>;
    fn Add(&self, pvalue: *const windows_core::PROPVARIANT) -> windows_core::Result<()>;
    fn GetType(&self) -> windows_core::Result<u16>;
    fn ChangeType(&self, vt: u16) -> windows_core::Result<()>;
    fn Clear(&self) -> windows_core::Result<()>;
    fn RemoveAt(&self, dwindex: u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IPortableDevicePropVariantCollection {}
impl IPortableDevicePropVariantCollection_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IPortableDevicePropVariantCollection_Vtbl
    where
        Identity: IPortableDevicePropVariantCollection_Impl,
    {
        unsafe extern "system" fn GetCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcelems: *const u32) -> windows_core::HRESULT
        where
            Identity: IPortableDevicePropVariantCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPortableDevicePropVariantCollection_Impl::GetCount(this, core::mem::transmute_copy(&pcelems)).into()
        }
        unsafe extern "system" fn GetAt<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwindex: u32, pvalue: *const core::mem::MaybeUninit<windows_core::PROPVARIANT>) -> windows_core::HRESULT
        where
            Identity: IPortableDevicePropVariantCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPortableDevicePropVariantCollection_Impl::GetAt(this, core::mem::transmute_copy(&dwindex), core::mem::transmute_copy(&pvalue)).into()
        }
        unsafe extern "system" fn Add<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *const core::mem::MaybeUninit<windows_core::PROPVARIANT>) -> windows_core::HRESULT
        where
            Identity: IPortableDevicePropVariantCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPortableDevicePropVariantCollection_Impl::Add(this, core::mem::transmute_copy(&pvalue)).into()
        }
        unsafe extern "system" fn GetType<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvt: *mut u16) -> windows_core::HRESULT
        where
            Identity: IPortableDevicePropVariantCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPortableDevicePropVariantCollection_Impl::GetType(this) {
                Ok(ok__) => {
                    pvt.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ChangeType<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, vt: u16) -> windows_core::HRESULT
        where
            Identity: IPortableDevicePropVariantCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPortableDevicePropVariantCollection_Impl::ChangeType(this, core::mem::transmute_copy(&vt)).into()
        }
        unsafe extern "system" fn Clear<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPortableDevicePropVariantCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPortableDevicePropVariantCollection_Impl::Clear(this).into()
        }
        unsafe extern "system" fn RemoveAt<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwindex: u32) -> windows_core::HRESULT
        where
            Identity: IPortableDevicePropVariantCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPortableDevicePropVariantCollection_Impl::RemoveAt(this, core::mem::transmute_copy(&dwindex)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCount: GetCount::<Identity, OFFSET>,
            GetAt: GetAt::<Identity, OFFSET>,
            Add: Add::<Identity, OFFSET>,
            GetType: GetType::<Identity, OFFSET>,
            ChangeType: ChangeType::<Identity, OFFSET>,
            Clear: Clear::<Identity, OFFSET>,
            RemoveAt: RemoveAt::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPortableDevicePropVariantCollection as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub trait IPortableDeviceProperties_Impl: Sized {
    fn GetSupportedProperties(&self, pszobjectid: &windows_core::PCWSTR) -> windows_core::Result<IPortableDeviceKeyCollection>;
    fn GetPropertyAttributes(&self, pszobjectid: &windows_core::PCWSTR, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> windows_core::Result<IPortableDeviceValues>;
    fn GetValues(&self, pszobjectid: &windows_core::PCWSTR, pkeys: Option<&IPortableDeviceKeyCollection>) -> windows_core::Result<IPortableDeviceValues>;
    fn SetValues(&self, pszobjectid: &windows_core::PCWSTR, pvalues: Option<&IPortableDeviceValues>) -> windows_core::Result<IPortableDeviceValues>;
    fn Delete(&self, pszobjectid: &windows_core::PCWSTR, pkeys: Option<&IPortableDeviceKeyCollection>) -> windows_core::Result<()>;
    fn Cancel(&self) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl windows_core::RuntimeName for IPortableDeviceProperties {}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl IPortableDeviceProperties_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IPortableDeviceProperties_Vtbl
    where
        Identity: IPortableDeviceProperties_Impl,
    {
        unsafe extern "system" fn GetSupportedProperties<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszobjectid: windows_core::PCWSTR, ppkeys: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPortableDeviceProperties_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPortableDeviceProperties_Impl::GetSupportedProperties(this, core::mem::transmute(&pszobjectid)) {
                Ok(ok__) => {
                    ppkeys.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPropertyAttributes<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszobjectid: windows_core::PCWSTR, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, ppattributes: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPortableDeviceProperties_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPortableDeviceProperties_Impl::GetPropertyAttributes(this, core::mem::transmute(&pszobjectid), core::mem::transmute_copy(&key)) {
                Ok(ok__) => {
                    ppattributes.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetValues<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszobjectid: windows_core::PCWSTR, pkeys: *mut core::ffi::c_void, ppvalues: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPortableDeviceProperties_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPortableDeviceProperties_Impl::GetValues(this, core::mem::transmute(&pszobjectid), windows_core::from_raw_borrowed(&pkeys)) {
                Ok(ok__) => {
                    ppvalues.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetValues<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszobjectid: windows_core::PCWSTR, pvalues: *mut core::ffi::c_void, ppresults: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPortableDeviceProperties_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPortableDeviceProperties_Impl::SetValues(this, core::mem::transmute(&pszobjectid), windows_core::from_raw_borrowed(&pvalues)) {
                Ok(ok__) => {
                    ppresults.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Delete<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszobjectid: windows_core::PCWSTR, pkeys: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPortableDeviceProperties_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPortableDeviceProperties_Impl::Delete(this, core::mem::transmute(&pszobjectid), windows_core::from_raw_borrowed(&pkeys)).into()
        }
        unsafe extern "system" fn Cancel<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPortableDeviceProperties_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPortableDeviceProperties_Impl::Cancel(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetSupportedProperties: GetSupportedProperties::<Identity, OFFSET>,
            GetPropertyAttributes: GetPropertyAttributes::<Identity, OFFSET>,
            GetValues: GetValues::<Identity, OFFSET>,
            SetValues: SetValues::<Identity, OFFSET>,
            Delete: Delete::<Identity, OFFSET>,
            Cancel: Cancel::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPortableDeviceProperties as windows_core::Interface>::IID
    }
}
pub trait IPortableDevicePropertiesBulk_Impl: Sized {
    fn QueueGetValuesByObjectList(&self, pobjectids: Option<&IPortableDevicePropVariantCollection>, pkeys: Option<&IPortableDeviceKeyCollection>, pcallback: Option<&IPortableDevicePropertiesBulkCallback>) -> windows_core::Result<windows_core::GUID>;
    fn QueueGetValuesByObjectFormat(&self, pguidobjectformat: *const windows_core::GUID, pszparentobjectid: &windows_core::PCWSTR, dwdepth: u32, pkeys: Option<&IPortableDeviceKeyCollection>, pcallback: Option<&IPortableDevicePropertiesBulkCallback>) -> windows_core::Result<windows_core::GUID>;
    fn QueueSetValuesByObjectList(&self, pobjectvalues: Option<&IPortableDeviceValuesCollection>, pcallback: Option<&IPortableDevicePropertiesBulkCallback>) -> windows_core::Result<windows_core::GUID>;
    fn Start(&self, pcontext: *const windows_core::GUID) -> windows_core::Result<()>;
    fn Cancel(&self, pcontext: *const windows_core::GUID) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IPortableDevicePropertiesBulk {}
impl IPortableDevicePropertiesBulk_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IPortableDevicePropertiesBulk_Vtbl
    where
        Identity: IPortableDevicePropertiesBulk_Impl,
    {
        unsafe extern "system" fn QueueGetValuesByObjectList<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pobjectids: *mut core::ffi::c_void, pkeys: *mut core::ffi::c_void, pcallback: *mut core::ffi::c_void, pcontext: *mut windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IPortableDevicePropertiesBulk_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPortableDevicePropertiesBulk_Impl::QueueGetValuesByObjectList(this, windows_core::from_raw_borrowed(&pobjectids), windows_core::from_raw_borrowed(&pkeys), windows_core::from_raw_borrowed(&pcallback)) {
                Ok(ok__) => {
                    pcontext.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueueGetValuesByObjectFormat<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pguidobjectformat: *const windows_core::GUID, pszparentobjectid: windows_core::PCWSTR, dwdepth: u32, pkeys: *mut core::ffi::c_void, pcallback: *mut core::ffi::c_void, pcontext: *mut windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IPortableDevicePropertiesBulk_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPortableDevicePropertiesBulk_Impl::QueueGetValuesByObjectFormat(this, core::mem::transmute_copy(&pguidobjectformat), core::mem::transmute(&pszparentobjectid), core::mem::transmute_copy(&dwdepth), windows_core::from_raw_borrowed(&pkeys), windows_core::from_raw_borrowed(&pcallback)) {
                Ok(ok__) => {
                    pcontext.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueueSetValuesByObjectList<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pobjectvalues: *mut core::ffi::c_void, pcallback: *mut core::ffi::c_void, pcontext: *mut windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IPortableDevicePropertiesBulk_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPortableDevicePropertiesBulk_Impl::QueueSetValuesByObjectList(this, windows_core::from_raw_borrowed(&pobjectvalues), windows_core::from_raw_borrowed(&pcallback)) {
                Ok(ok__) => {
                    pcontext.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Start<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcontext: *const windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IPortableDevicePropertiesBulk_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPortableDevicePropertiesBulk_Impl::Start(this, core::mem::transmute_copy(&pcontext)).into()
        }
        unsafe extern "system" fn Cancel<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcontext: *const windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IPortableDevicePropertiesBulk_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPortableDevicePropertiesBulk_Impl::Cancel(this, core::mem::transmute_copy(&pcontext)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            QueueGetValuesByObjectList: QueueGetValuesByObjectList::<Identity, OFFSET>,
            QueueGetValuesByObjectFormat: QueueGetValuesByObjectFormat::<Identity, OFFSET>,
            QueueSetValuesByObjectList: QueueSetValuesByObjectList::<Identity, OFFSET>,
            Start: Start::<Identity, OFFSET>,
            Cancel: Cancel::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPortableDevicePropertiesBulk as windows_core::Interface>::IID
    }
}
pub trait IPortableDevicePropertiesBulkCallback_Impl: Sized {
    fn OnStart(&self, pcontext: *const windows_core::GUID) -> windows_core::Result<()>;
    fn OnProgress(&self, pcontext: *const windows_core::GUID, presults: Option<&IPortableDeviceValuesCollection>) -> windows_core::Result<()>;
    fn OnEnd(&self, pcontext: *const windows_core::GUID, hrstatus: windows_core::HRESULT) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IPortableDevicePropertiesBulkCallback {}
impl IPortableDevicePropertiesBulkCallback_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IPortableDevicePropertiesBulkCallback_Vtbl
    where
        Identity: IPortableDevicePropertiesBulkCallback_Impl,
    {
        unsafe extern "system" fn OnStart<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcontext: *const windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IPortableDevicePropertiesBulkCallback_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPortableDevicePropertiesBulkCallback_Impl::OnStart(this, core::mem::transmute_copy(&pcontext)).into()
        }
        unsafe extern "system" fn OnProgress<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcontext: *const windows_core::GUID, presults: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPortableDevicePropertiesBulkCallback_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPortableDevicePropertiesBulkCallback_Impl::OnProgress(this, core::mem::transmute_copy(&pcontext), windows_core::from_raw_borrowed(&presults)).into()
        }
        unsafe extern "system" fn OnEnd<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcontext: *const windows_core::GUID, hrstatus: windows_core::HRESULT) -> windows_core::HRESULT
        where
            Identity: IPortableDevicePropertiesBulkCallback_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPortableDevicePropertiesBulkCallback_Impl::OnEnd(this, core::mem::transmute_copy(&pcontext), core::mem::transmute_copy(&hrstatus)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnStart: OnStart::<Identity, OFFSET>,
            OnProgress: OnProgress::<Identity, OFFSET>,
            OnEnd: OnEnd::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPortableDevicePropertiesBulkCallback as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_UI_Shell_PropertiesSystem"))]
pub trait IPortableDeviceResources_Impl: Sized {
    fn GetSupportedResources(&self, pszobjectid: &windows_core::PCWSTR) -> windows_core::Result<IPortableDeviceKeyCollection>;
    fn GetResourceAttributes(&self, pszobjectid: &windows_core::PCWSTR, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> windows_core::Result<IPortableDeviceValues>;
    fn GetStream(&self, pszobjectid: &windows_core::PCWSTR, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, dwmode: u32, pdwoptimalbuffersize: *mut u32, ppstream: *mut Option<super::super::System::Com::IStream>) -> windows_core::Result<()>;
    fn Delete(&self, pszobjectid: &windows_core::PCWSTR, pkeys: Option<&IPortableDeviceKeyCollection>) -> windows_core::Result<()>;
    fn Cancel(&self) -> windows_core::Result<()>;
    fn CreateResource(&self, presourceattributes: Option<&IPortableDeviceValues>, ppdata: *mut Option<super::super::System::Com::IStream>, pdwoptimalwritebuffersize: *mut u32, ppszcookie: *mut windows_core::PWSTR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl windows_core::RuntimeName for IPortableDeviceResources {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl IPortableDeviceResources_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IPortableDeviceResources_Vtbl
    where
        Identity: IPortableDeviceResources_Impl,
    {
        unsafe extern "system" fn GetSupportedResources<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszobjectid: windows_core::PCWSTR, ppkeys: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPortableDeviceResources_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPortableDeviceResources_Impl::GetSupportedResources(this, core::mem::transmute(&pszobjectid)) {
                Ok(ok__) => {
                    ppkeys.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetResourceAttributes<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszobjectid: windows_core::PCWSTR, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, ppresourceattributes: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPortableDeviceResources_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPortableDeviceResources_Impl::GetResourceAttributes(this, core::mem::transmute(&pszobjectid), core::mem::transmute_copy(&key)) {
                Ok(ok__) => {
                    ppresourceattributes.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStream<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszobjectid: windows_core::PCWSTR, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, dwmode: u32, pdwoptimalbuffersize: *mut u32, ppstream: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPortableDeviceResources_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPortableDeviceResources_Impl::GetStream(this, core::mem::transmute(&pszobjectid), core::mem::transmute_copy(&key), core::mem::transmute_copy(&dwmode), core::mem::transmute_copy(&pdwoptimalbuffersize), core::mem::transmute_copy(&ppstream)).into()
        }
        unsafe extern "system" fn Delete<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszobjectid: windows_core::PCWSTR, pkeys: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPortableDeviceResources_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPortableDeviceResources_Impl::Delete(this, core::mem::transmute(&pszobjectid), windows_core::from_raw_borrowed(&pkeys)).into()
        }
        unsafe extern "system" fn Cancel<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPortableDeviceResources_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPortableDeviceResources_Impl::Cancel(this).into()
        }
        unsafe extern "system" fn CreateResource<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, presourceattributes: *mut core::ffi::c_void, ppdata: *mut *mut core::ffi::c_void, pdwoptimalwritebuffersize: *mut u32, ppszcookie: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IPortableDeviceResources_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPortableDeviceResources_Impl::CreateResource(this, windows_core::from_raw_borrowed(&presourceattributes), core::mem::transmute_copy(&ppdata), core::mem::transmute_copy(&pdwoptimalwritebuffersize), core::mem::transmute_copy(&ppszcookie)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetSupportedResources: GetSupportedResources::<Identity, OFFSET>,
            GetResourceAttributes: GetResourceAttributes::<Identity, OFFSET>,
            GetStream: GetStream::<Identity, OFFSET>,
            Delete: Delete::<Identity, OFFSET>,
            Cancel: Cancel::<Identity, OFFSET>,
            CreateResource: CreateResource::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPortableDeviceResources as windows_core::Interface>::IID
    }
}
pub trait IPortableDeviceService_Impl: Sized {
    fn Open(&self, pszpnpserviceid: &windows_core::PCWSTR, pclientinfo: Option<&IPortableDeviceValues>) -> windows_core::Result<()>;
    fn Capabilities(&self) -> windows_core::Result<IPortableDeviceServiceCapabilities>;
    fn Content(&self) -> windows_core::Result<IPortableDeviceContent2>;
    fn Methods(&self) -> windows_core::Result<IPortableDeviceServiceMethods>;
    fn Cancel(&self) -> windows_core::Result<()>;
    fn Close(&self) -> windows_core::Result<()>;
    fn GetServiceObjectID(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn GetPnPServiceID(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn Advise(&self, dwflags: u32, pcallback: Option<&IPortableDeviceEventCallback>, pparameters: Option<&IPortableDeviceValues>) -> windows_core::Result<windows_core::PWSTR>;
    fn Unadvise(&self, pszcookie: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn SendCommand(&self, dwflags: u32, pparameters: Option<&IPortableDeviceValues>) -> windows_core::Result<IPortableDeviceValues>;
}
impl windows_core::RuntimeName for IPortableDeviceService {}
impl IPortableDeviceService_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IPortableDeviceService_Vtbl
    where
        Identity: IPortableDeviceService_Impl,
    {
        unsafe extern "system" fn Open<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszpnpserviceid: windows_core::PCWSTR, pclientinfo: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPortableDeviceService_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPortableDeviceService_Impl::Open(this, core::mem::transmute(&pszpnpserviceid), windows_core::from_raw_borrowed(&pclientinfo)).into()
        }
        unsafe extern "system" fn Capabilities<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcapabilities: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPortableDeviceService_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPortableDeviceService_Impl::Capabilities(this) {
                Ok(ok__) => {
                    ppcapabilities.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Content<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcontent: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPortableDeviceService_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPortableDeviceService_Impl::Content(this) {
                Ok(ok__) => {
                    ppcontent.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Methods<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppmethods: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPortableDeviceService_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPortableDeviceService_Impl::Methods(this) {
                Ok(ok__) => {
                    ppmethods.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Cancel<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPortableDeviceService_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPortableDeviceService_Impl::Cancel(this).into()
        }
        unsafe extern "system" fn Close<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPortableDeviceService_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPortableDeviceService_Impl::Close(this).into()
        }
        unsafe extern "system" fn GetServiceObjectID<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppszserviceobjectid: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IPortableDeviceService_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPortableDeviceService_Impl::GetServiceObjectID(this) {
                Ok(ok__) => {
                    ppszserviceobjectid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPnPServiceID<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppszpnpserviceid: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IPortableDeviceService_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPortableDeviceService_Impl::GetPnPServiceID(this) {
                Ok(ok__) => {
                    ppszpnpserviceid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Advise<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: u32, pcallback: *mut core::ffi::c_void, pparameters: *mut core::ffi::c_void, ppszcookie: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IPortableDeviceService_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPortableDeviceService_Impl::Advise(this, core::mem::transmute_copy(&dwflags), windows_core::from_raw_borrowed(&pcallback), windows_core::from_raw_borrowed(&pparameters)) {
                Ok(ok__) => {
                    ppszcookie.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Unadvise<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszcookie: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IPortableDeviceService_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPortableDeviceService_Impl::Unadvise(this, core::mem::transmute(&pszcookie)).into()
        }
        unsafe extern "system" fn SendCommand<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: u32, pparameters: *mut core::ffi::c_void, ppresults: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPortableDeviceService_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPortableDeviceService_Impl::SendCommand(this, core::mem::transmute_copy(&dwflags), windows_core::from_raw_borrowed(&pparameters)) {
                Ok(ok__) => {
                    ppresults.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Open: Open::<Identity, OFFSET>,
            Capabilities: Capabilities::<Identity, OFFSET>,
            Content: Content::<Identity, OFFSET>,
            Methods: Methods::<Identity, OFFSET>,
            Cancel: Cancel::<Identity, OFFSET>,
            Close: Close::<Identity, OFFSET>,
            GetServiceObjectID: GetServiceObjectID::<Identity, OFFSET>,
            GetPnPServiceID: GetPnPServiceID::<Identity, OFFSET>,
            Advise: Advise::<Identity, OFFSET>,
            Unadvise: Unadvise::<Identity, OFFSET>,
            SendCommand: SendCommand::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPortableDeviceService as windows_core::Interface>::IID
    }
}
pub trait IPortableDeviceServiceActivation_Impl: Sized {
    fn OpenAsync(&self, pszpnpserviceid: &windows_core::PCWSTR, pclientinfo: Option<&IPortableDeviceValues>, pcallback: Option<&IPortableDeviceServiceOpenCallback>) -> windows_core::Result<()>;
    fn CancelOpenAsync(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IPortableDeviceServiceActivation {}
impl IPortableDeviceServiceActivation_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IPortableDeviceServiceActivation_Vtbl
    where
        Identity: IPortableDeviceServiceActivation_Impl,
    {
        unsafe extern "system" fn OpenAsync<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszpnpserviceid: windows_core::PCWSTR, pclientinfo: *mut core::ffi::c_void, pcallback: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPortableDeviceServiceActivation_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPortableDeviceServiceActivation_Impl::OpenAsync(this, core::mem::transmute(&pszpnpserviceid), windows_core::from_raw_borrowed(&pclientinfo), windows_core::from_raw_borrowed(&pcallback)).into()
        }
        unsafe extern "system" fn CancelOpenAsync<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPortableDeviceServiceActivation_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPortableDeviceServiceActivation_Impl::CancelOpenAsync(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OpenAsync: OpenAsync::<Identity, OFFSET>,
            CancelOpenAsync: CancelOpenAsync::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPortableDeviceServiceActivation as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub trait IPortableDeviceServiceCapabilities_Impl: Sized {
    fn GetSupportedMethods(&self) -> windows_core::Result<IPortableDevicePropVariantCollection>;
    fn GetSupportedMethodsByFormat(&self, format: *const windows_core::GUID) -> windows_core::Result<IPortableDevicePropVariantCollection>;
    fn GetMethodAttributes(&self, method: *const windows_core::GUID) -> windows_core::Result<IPortableDeviceValues>;
    fn GetMethodParameterAttributes(&self, method: *const windows_core::GUID, parameter: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> windows_core::Result<IPortableDeviceValues>;
    fn GetSupportedFormats(&self) -> windows_core::Result<IPortableDevicePropVariantCollection>;
    fn GetFormatAttributes(&self, format: *const windows_core::GUID) -> windows_core::Result<IPortableDeviceValues>;
    fn GetSupportedFormatProperties(&self, format: *const windows_core::GUID) -> windows_core::Result<IPortableDeviceKeyCollection>;
    fn GetFormatPropertyAttributes(&self, format: *const windows_core::GUID, property: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> windows_core::Result<IPortableDeviceValues>;
    fn GetSupportedEvents(&self) -> windows_core::Result<IPortableDevicePropVariantCollection>;
    fn GetEventAttributes(&self, event: *const windows_core::GUID) -> windows_core::Result<IPortableDeviceValues>;
    fn GetEventParameterAttributes(&self, event: *const windows_core::GUID, parameter: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> windows_core::Result<IPortableDeviceValues>;
    fn GetInheritedServices(&self, dwinheritancetype: u32) -> windows_core::Result<IPortableDevicePropVariantCollection>;
    fn GetFormatRenderingProfiles(&self, format: *const windows_core::GUID) -> windows_core::Result<IPortableDeviceValuesCollection>;
    fn GetSupportedCommands(&self) -> windows_core::Result<IPortableDeviceKeyCollection>;
    fn GetCommandOptions(&self, command: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> windows_core::Result<IPortableDeviceValues>;
    fn Cancel(&self) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl windows_core::RuntimeName for IPortableDeviceServiceCapabilities {}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl IPortableDeviceServiceCapabilities_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IPortableDeviceServiceCapabilities_Vtbl
    where
        Identity: IPortableDeviceServiceCapabilities_Impl,
    {
        unsafe extern "system" fn GetSupportedMethods<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppmethods: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPortableDeviceServiceCapabilities_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPortableDeviceServiceCapabilities_Impl::GetSupportedMethods(this) {
                Ok(ok__) => {
                    ppmethods.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSupportedMethodsByFormat<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, format: *const windows_core::GUID, ppmethods: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPortableDeviceServiceCapabilities_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPortableDeviceServiceCapabilities_Impl::GetSupportedMethodsByFormat(this, core::mem::transmute_copy(&format)) {
                Ok(ok__) => {
                    ppmethods.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMethodAttributes<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, method: *const windows_core::GUID, ppattributes: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPortableDeviceServiceCapabilities_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPortableDeviceServiceCapabilities_Impl::GetMethodAttributes(this, core::mem::transmute_copy(&method)) {
                Ok(ok__) => {
                    ppattributes.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMethodParameterAttributes<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, method: *const windows_core::GUID, parameter: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, ppattributes: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPortableDeviceServiceCapabilities_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPortableDeviceServiceCapabilities_Impl::GetMethodParameterAttributes(this, core::mem::transmute_copy(&method), core::mem::transmute_copy(&parameter)) {
                Ok(ok__) => {
                    ppattributes.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSupportedFormats<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppformats: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPortableDeviceServiceCapabilities_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPortableDeviceServiceCapabilities_Impl::GetSupportedFormats(this) {
                Ok(ok__) => {
                    ppformats.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFormatAttributes<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, format: *const windows_core::GUID, ppattributes: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPortableDeviceServiceCapabilities_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPortableDeviceServiceCapabilities_Impl::GetFormatAttributes(this, core::mem::transmute_copy(&format)) {
                Ok(ok__) => {
                    ppattributes.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSupportedFormatProperties<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, format: *const windows_core::GUID, ppkeys: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPortableDeviceServiceCapabilities_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPortableDeviceServiceCapabilities_Impl::GetSupportedFormatProperties(this, core::mem::transmute_copy(&format)) {
                Ok(ok__) => {
                    ppkeys.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFormatPropertyAttributes<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, format: *const windows_core::GUID, property: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, ppattributes: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPortableDeviceServiceCapabilities_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPortableDeviceServiceCapabilities_Impl::GetFormatPropertyAttributes(this, core::mem::transmute_copy(&format), core::mem::transmute_copy(&property)) {
                Ok(ok__) => {
                    ppattributes.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSupportedEvents<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppevents: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPortableDeviceServiceCapabilities_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPortableDeviceServiceCapabilities_Impl::GetSupportedEvents(this) {
                Ok(ok__) => {
                    ppevents.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetEventAttributes<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, event: *const windows_core::GUID, ppattributes: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPortableDeviceServiceCapabilities_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPortableDeviceServiceCapabilities_Impl::GetEventAttributes(this, core::mem::transmute_copy(&event)) {
                Ok(ok__) => {
                    ppattributes.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetEventParameterAttributes<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, event: *const windows_core::GUID, parameter: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, ppattributes: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPortableDeviceServiceCapabilities_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPortableDeviceServiceCapabilities_Impl::GetEventParameterAttributes(this, core::mem::transmute_copy(&event), core::mem::transmute_copy(&parameter)) {
                Ok(ok__) => {
                    ppattributes.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetInheritedServices<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwinheritancetype: u32, ppservices: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPortableDeviceServiceCapabilities_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPortableDeviceServiceCapabilities_Impl::GetInheritedServices(this, core::mem::transmute_copy(&dwinheritancetype)) {
                Ok(ok__) => {
                    ppservices.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFormatRenderingProfiles<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, format: *const windows_core::GUID, pprenderingprofiles: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPortableDeviceServiceCapabilities_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPortableDeviceServiceCapabilities_Impl::GetFormatRenderingProfiles(this, core::mem::transmute_copy(&format)) {
                Ok(ok__) => {
                    pprenderingprofiles.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSupportedCommands<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcommands: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPortableDeviceServiceCapabilities_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPortableDeviceServiceCapabilities_Impl::GetSupportedCommands(this) {
                Ok(ok__) => {
                    ppcommands.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCommandOptions<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, command: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, ppoptions: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPortableDeviceServiceCapabilities_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPortableDeviceServiceCapabilities_Impl::GetCommandOptions(this, core::mem::transmute_copy(&command)) {
                Ok(ok__) => {
                    ppoptions.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Cancel<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPortableDeviceServiceCapabilities_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPortableDeviceServiceCapabilities_Impl::Cancel(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetSupportedMethods: GetSupportedMethods::<Identity, OFFSET>,
            GetSupportedMethodsByFormat: GetSupportedMethodsByFormat::<Identity, OFFSET>,
            GetMethodAttributes: GetMethodAttributes::<Identity, OFFSET>,
            GetMethodParameterAttributes: GetMethodParameterAttributes::<Identity, OFFSET>,
            GetSupportedFormats: GetSupportedFormats::<Identity, OFFSET>,
            GetFormatAttributes: GetFormatAttributes::<Identity, OFFSET>,
            GetSupportedFormatProperties: GetSupportedFormatProperties::<Identity, OFFSET>,
            GetFormatPropertyAttributes: GetFormatPropertyAttributes::<Identity, OFFSET>,
            GetSupportedEvents: GetSupportedEvents::<Identity, OFFSET>,
            GetEventAttributes: GetEventAttributes::<Identity, OFFSET>,
            GetEventParameterAttributes: GetEventParameterAttributes::<Identity, OFFSET>,
            GetInheritedServices: GetInheritedServices::<Identity, OFFSET>,
            GetFormatRenderingProfiles: GetFormatRenderingProfiles::<Identity, OFFSET>,
            GetSupportedCommands: GetSupportedCommands::<Identity, OFFSET>,
            GetCommandOptions: GetCommandOptions::<Identity, OFFSET>,
            Cancel: Cancel::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPortableDeviceServiceCapabilities as windows_core::Interface>::IID
    }
}
pub trait IPortableDeviceServiceManager_Impl: Sized {
    fn GetDeviceServices(&self, pszpnpdeviceid: &windows_core::PCWSTR, guidservicecategory: *const windows_core::GUID, pservices: *mut windows_core::PWSTR, pcservices: *mut u32) -> windows_core::Result<()>;
    fn GetDeviceForService(&self, pszpnpserviceid: &windows_core::PCWSTR) -> windows_core::Result<windows_core::PWSTR>;
}
impl windows_core::RuntimeName for IPortableDeviceServiceManager {}
impl IPortableDeviceServiceManager_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IPortableDeviceServiceManager_Vtbl
    where
        Identity: IPortableDeviceServiceManager_Impl,
    {
        unsafe extern "system" fn GetDeviceServices<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszpnpdeviceid: windows_core::PCWSTR, guidservicecategory: *const windows_core::GUID, pservices: *mut windows_core::PWSTR, pcservices: *mut u32) -> windows_core::HRESULT
        where
            Identity: IPortableDeviceServiceManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPortableDeviceServiceManager_Impl::GetDeviceServices(this, core::mem::transmute(&pszpnpdeviceid), core::mem::transmute_copy(&guidservicecategory), core::mem::transmute_copy(&pservices), core::mem::transmute_copy(&pcservices)).into()
        }
        unsafe extern "system" fn GetDeviceForService<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszpnpserviceid: windows_core::PCWSTR, ppszpnpdeviceid: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IPortableDeviceServiceManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPortableDeviceServiceManager_Impl::GetDeviceForService(this, core::mem::transmute(&pszpnpserviceid)) {
                Ok(ok__) => {
                    ppszpnpdeviceid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetDeviceServices: GetDeviceServices::<Identity, OFFSET>,
            GetDeviceForService: GetDeviceForService::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPortableDeviceServiceManager as windows_core::Interface>::IID
    }
}
pub trait IPortableDeviceServiceMethodCallback_Impl: Sized {
    fn OnComplete(&self, hrstatus: windows_core::HRESULT, presults: Option<&IPortableDeviceValues>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IPortableDeviceServiceMethodCallback {}
impl IPortableDeviceServiceMethodCallback_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IPortableDeviceServiceMethodCallback_Vtbl
    where
        Identity: IPortableDeviceServiceMethodCallback_Impl,
    {
        unsafe extern "system" fn OnComplete<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hrstatus: windows_core::HRESULT, presults: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPortableDeviceServiceMethodCallback_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPortableDeviceServiceMethodCallback_Impl::OnComplete(this, core::mem::transmute_copy(&hrstatus), windows_core::from_raw_borrowed(&presults)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnComplete: OnComplete::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPortableDeviceServiceMethodCallback as windows_core::Interface>::IID
    }
}
pub trait IPortableDeviceServiceMethods_Impl: Sized {
    fn Invoke(&self, method: *const windows_core::GUID, pparameters: Option<&IPortableDeviceValues>, ppresults: *mut Option<IPortableDeviceValues>) -> windows_core::Result<()>;
    fn InvokeAsync(&self, method: *const windows_core::GUID, pparameters: Option<&IPortableDeviceValues>, pcallback: Option<&IPortableDeviceServiceMethodCallback>) -> windows_core::Result<()>;
    fn Cancel(&self, pcallback: Option<&IPortableDeviceServiceMethodCallback>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IPortableDeviceServiceMethods {}
impl IPortableDeviceServiceMethods_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IPortableDeviceServiceMethods_Vtbl
    where
        Identity: IPortableDeviceServiceMethods_Impl,
    {
        unsafe extern "system" fn Invoke<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, method: *const windows_core::GUID, pparameters: *mut core::ffi::c_void, ppresults: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPortableDeviceServiceMethods_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPortableDeviceServiceMethods_Impl::Invoke(this, core::mem::transmute_copy(&method), windows_core::from_raw_borrowed(&pparameters), core::mem::transmute_copy(&ppresults)).into()
        }
        unsafe extern "system" fn InvokeAsync<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, method: *const windows_core::GUID, pparameters: *mut core::ffi::c_void, pcallback: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPortableDeviceServiceMethods_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPortableDeviceServiceMethods_Impl::InvokeAsync(this, core::mem::transmute_copy(&method), windows_core::from_raw_borrowed(&pparameters), windows_core::from_raw_borrowed(&pcallback)).into()
        }
        unsafe extern "system" fn Cancel<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcallback: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPortableDeviceServiceMethods_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPortableDeviceServiceMethods_Impl::Cancel(this, windows_core::from_raw_borrowed(&pcallback)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Invoke: Invoke::<Identity, OFFSET>,
            InvokeAsync: InvokeAsync::<Identity, OFFSET>,
            Cancel: Cancel::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPortableDeviceServiceMethods as windows_core::Interface>::IID
    }
}
pub trait IPortableDeviceServiceOpenCallback_Impl: Sized {
    fn OnComplete(&self, hrstatus: windows_core::HRESULT) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IPortableDeviceServiceOpenCallback {}
impl IPortableDeviceServiceOpenCallback_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IPortableDeviceServiceOpenCallback_Vtbl
    where
        Identity: IPortableDeviceServiceOpenCallback_Impl,
    {
        unsafe extern "system" fn OnComplete<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hrstatus: windows_core::HRESULT) -> windows_core::HRESULT
        where
            Identity: IPortableDeviceServiceOpenCallback_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPortableDeviceServiceOpenCallback_Impl::OnComplete(this, core::mem::transmute_copy(&hrstatus)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnComplete: OnComplete::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPortableDeviceServiceOpenCallback as windows_core::Interface>::IID
    }
}
pub trait IPortableDeviceUnitsStream_Impl: Sized {
    fn SeekInUnits(&self, dlibmove: i64, units: WPD_STREAM_UNITS, dworigin: u32, plibnewposition: *mut u64) -> windows_core::Result<()>;
    fn Cancel(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IPortableDeviceUnitsStream {}
impl IPortableDeviceUnitsStream_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IPortableDeviceUnitsStream_Vtbl
    where
        Identity: IPortableDeviceUnitsStream_Impl,
    {
        unsafe extern "system" fn SeekInUnits<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dlibmove: i64, units: WPD_STREAM_UNITS, dworigin: u32, plibnewposition: *mut u64) -> windows_core::HRESULT
        where
            Identity: IPortableDeviceUnitsStream_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPortableDeviceUnitsStream_Impl::SeekInUnits(this, core::mem::transmute_copy(&dlibmove), core::mem::transmute_copy(&units), core::mem::transmute_copy(&dworigin), core::mem::transmute_copy(&plibnewposition)).into()
        }
        unsafe extern "system" fn Cancel<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPortableDeviceUnitsStream_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPortableDeviceUnitsStream_Impl::Cancel(this).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), SeekInUnits: SeekInUnits::<Identity, OFFSET>, Cancel: Cancel::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPortableDeviceUnitsStream as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub trait IPortableDeviceValues_Impl: Sized {
    fn GetCount(&self, pcelt: *const u32) -> windows_core::Result<()>;
    fn GetAt(&self, index: u32, pkey: *mut super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: *mut windows_core::PROPVARIANT) -> windows_core::Result<()>;
    fn SetValue(&self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: *const windows_core::PROPVARIANT) -> windows_core::Result<()>;
    fn GetValue(&self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> windows_core::Result<windows_core::PROPVARIANT>;
    fn SetStringValue(&self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, value: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetStringValue(&self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> windows_core::Result<windows_core::PWSTR>;
    fn SetUnsignedIntegerValue(&self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, value: u32) -> windows_core::Result<()>;
    fn GetUnsignedIntegerValue(&self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> windows_core::Result<u32>;
    fn SetSignedIntegerValue(&self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, value: i32) -> windows_core::Result<()>;
    fn GetSignedIntegerValue(&self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> windows_core::Result<i32>;
    fn SetUnsignedLargeIntegerValue(&self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, value: u64) -> windows_core::Result<()>;
    fn GetUnsignedLargeIntegerValue(&self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> windows_core::Result<u64>;
    fn SetSignedLargeIntegerValue(&self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, value: i64) -> windows_core::Result<()>;
    fn GetSignedLargeIntegerValue(&self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> windows_core::Result<i64>;
    fn SetFloatValue(&self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, value: f32) -> windows_core::Result<()>;
    fn GetFloatValue(&self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> windows_core::Result<f32>;
    fn SetErrorValue(&self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, value: windows_core::HRESULT) -> windows_core::Result<()>;
    fn GetErrorValue(&self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> windows_core::Result<windows_core::HRESULT>;
    fn SetKeyValue(&self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, value: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> windows_core::Result<()>;
    fn GetKeyValue(&self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: *mut super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> windows_core::Result<()>;
    fn SetBoolValue(&self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, value: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn GetBoolValue(&self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn SetIUnknownValue(&self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: Option<&windows_core::IUnknown>) -> windows_core::Result<()>;
    fn GetIUnknownValue(&self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> windows_core::Result<windows_core::IUnknown>;
    fn SetGuidValue(&self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, value: *const windows_core::GUID) -> windows_core::Result<()>;
    fn GetGuidValue(&self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> windows_core::Result<windows_core::GUID>;
    fn SetBufferValue(&self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: *const u8, cbvalue: u32) -> windows_core::Result<()>;
    fn GetBufferValue(&self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, ppvalue: *mut *mut u8, pcbvalue: *mut u32) -> windows_core::Result<()>;
    fn SetIPortableDeviceValuesValue(&self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: Option<&IPortableDeviceValues>) -> windows_core::Result<()>;
    fn GetIPortableDeviceValuesValue(&self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> windows_core::Result<IPortableDeviceValues>;
    fn SetIPortableDevicePropVariantCollectionValue(&self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: Option<&IPortableDevicePropVariantCollection>) -> windows_core::Result<()>;
    fn GetIPortableDevicePropVariantCollectionValue(&self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> windows_core::Result<IPortableDevicePropVariantCollection>;
    fn SetIPortableDeviceKeyCollectionValue(&self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: Option<&IPortableDeviceKeyCollection>) -> windows_core::Result<()>;
    fn GetIPortableDeviceKeyCollectionValue(&self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> windows_core::Result<IPortableDeviceKeyCollection>;
    fn SetIPortableDeviceValuesCollectionValue(&self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: Option<&IPortableDeviceValuesCollection>) -> windows_core::Result<()>;
    fn GetIPortableDeviceValuesCollectionValue(&self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> windows_core::Result<IPortableDeviceValuesCollection>;
    fn RemoveValue(&self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> windows_core::Result<()>;
    fn CopyValuesFromPropertyStore(&self, pstore: Option<&super::super::UI::Shell::PropertiesSystem::IPropertyStore>) -> windows_core::Result<()>;
    fn CopyValuesToPropertyStore(&self, pstore: Option<&super::super::UI::Shell::PropertiesSystem::IPropertyStore>) -> windows_core::Result<()>;
    fn Clear(&self) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl windows_core::RuntimeName for IPortableDeviceValues {}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl IPortableDeviceValues_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IPortableDeviceValues_Vtbl
    where
        Identity: IPortableDeviceValues_Impl,
    {
        unsafe extern "system" fn GetCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcelt: *const u32) -> windows_core::HRESULT
        where
            Identity: IPortableDeviceValues_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPortableDeviceValues_Impl::GetCount(this, core::mem::transmute_copy(&pcelt)).into()
        }
        unsafe extern "system" fn GetAt<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, pkey: *mut super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: *mut core::mem::MaybeUninit<windows_core::PROPVARIANT>) -> windows_core::HRESULT
        where
            Identity: IPortableDeviceValues_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPortableDeviceValues_Impl::GetAt(this, core::mem::transmute_copy(&index), core::mem::transmute_copy(&pkey), core::mem::transmute_copy(&pvalue)).into()
        }
        unsafe extern "system" fn SetValue<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: *const core::mem::MaybeUninit<windows_core::PROPVARIANT>) -> windows_core::HRESULT
        where
            Identity: IPortableDeviceValues_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPortableDeviceValues_Impl::SetValue(this, core::mem::transmute_copy(&key), core::mem::transmute_copy(&pvalue)).into()
        }
        unsafe extern "system" fn GetValue<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: *mut core::mem::MaybeUninit<windows_core::PROPVARIANT>) -> windows_core::HRESULT
        where
            Identity: IPortableDeviceValues_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPortableDeviceValues_Impl::GetValue(this, core::mem::transmute_copy(&key)) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStringValue<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, value: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IPortableDeviceValues_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPortableDeviceValues_Impl::SetStringValue(this, core::mem::transmute_copy(&key), core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn GetStringValue<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IPortableDeviceValues_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPortableDeviceValues_Impl::GetStringValue(this, core::mem::transmute_copy(&key)) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUnsignedIntegerValue<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, value: u32) -> windows_core::HRESULT
        where
            Identity: IPortableDeviceValues_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPortableDeviceValues_Impl::SetUnsignedIntegerValue(this, core::mem::transmute_copy(&key), core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetUnsignedIntegerValue<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: *mut u32) -> windows_core::HRESULT
        where
            Identity: IPortableDeviceValues_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPortableDeviceValues_Impl::GetUnsignedIntegerValue(this, core::mem::transmute_copy(&key)) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSignedIntegerValue<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, value: i32) -> windows_core::HRESULT
        where
            Identity: IPortableDeviceValues_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPortableDeviceValues_Impl::SetSignedIntegerValue(this, core::mem::transmute_copy(&key), core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetSignedIntegerValue<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: *mut i32) -> windows_core::HRESULT
        where
            Identity: IPortableDeviceValues_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPortableDeviceValues_Impl::GetSignedIntegerValue(this, core::mem::transmute_copy(&key)) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUnsignedLargeIntegerValue<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, value: u64) -> windows_core::HRESULT
        where
            Identity: IPortableDeviceValues_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPortableDeviceValues_Impl::SetUnsignedLargeIntegerValue(this, core::mem::transmute_copy(&key), core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetUnsignedLargeIntegerValue<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: *mut u64) -> windows_core::HRESULT
        where
            Identity: IPortableDeviceValues_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPortableDeviceValues_Impl::GetUnsignedLargeIntegerValue(this, core::mem::transmute_copy(&key)) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSignedLargeIntegerValue<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, value: i64) -> windows_core::HRESULT
        where
            Identity: IPortableDeviceValues_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPortableDeviceValues_Impl::SetSignedLargeIntegerValue(this, core::mem::transmute_copy(&key), core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetSignedLargeIntegerValue<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: *mut i64) -> windows_core::HRESULT
        where
            Identity: IPortableDeviceValues_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPortableDeviceValues_Impl::GetSignedLargeIntegerValue(this, core::mem::transmute_copy(&key)) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFloatValue<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, value: f32) -> windows_core::HRESULT
        where
            Identity: IPortableDeviceValues_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPortableDeviceValues_Impl::SetFloatValue(this, core::mem::transmute_copy(&key), core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetFloatValue<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: *mut f32) -> windows_core::HRESULT
        where
            Identity: IPortableDeviceValues_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPortableDeviceValues_Impl::GetFloatValue(this, core::mem::transmute_copy(&key)) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetErrorValue<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, value: windows_core::HRESULT) -> windows_core::HRESULT
        where
            Identity: IPortableDeviceValues_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPortableDeviceValues_Impl::SetErrorValue(this, core::mem::transmute_copy(&key), core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetErrorValue<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: *mut windows_core::HRESULT) -> windows_core::HRESULT
        where
            Identity: IPortableDeviceValues_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPortableDeviceValues_Impl::GetErrorValue(this, core::mem::transmute_copy(&key)) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetKeyValue<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, value: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> windows_core::HRESULT
        where
            Identity: IPortableDeviceValues_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPortableDeviceValues_Impl::SetKeyValue(this, core::mem::transmute_copy(&key), core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetKeyValue<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: *mut super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> windows_core::HRESULT
        where
            Identity: IPortableDeviceValues_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPortableDeviceValues_Impl::GetKeyValue(this, core::mem::transmute_copy(&key), core::mem::transmute_copy(&pvalue)).into()
        }
        unsafe extern "system" fn SetBoolValue<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, value: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IPortableDeviceValues_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPortableDeviceValues_Impl::SetBoolValue(this, core::mem::transmute_copy(&key), core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetBoolValue<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IPortableDeviceValues_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPortableDeviceValues_Impl::GetBoolValue(this, core::mem::transmute_copy(&key)) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIUnknownValue<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPortableDeviceValues_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPortableDeviceValues_Impl::SetIUnknownValue(this, core::mem::transmute_copy(&key), windows_core::from_raw_borrowed(&pvalue)).into()
        }
        unsafe extern "system" fn GetIUnknownValue<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, ppvalue: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPortableDeviceValues_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPortableDeviceValues_Impl::GetIUnknownValue(this, core::mem::transmute_copy(&key)) {
                Ok(ok__) => {
                    ppvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetGuidValue<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, value: *const windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IPortableDeviceValues_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPortableDeviceValues_Impl::SetGuidValue(this, core::mem::transmute_copy(&key), core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetGuidValue<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: *mut windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IPortableDeviceValues_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPortableDeviceValues_Impl::GetGuidValue(this, core::mem::transmute_copy(&key)) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBufferValue<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: *const u8, cbvalue: u32) -> windows_core::HRESULT
        where
            Identity: IPortableDeviceValues_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPortableDeviceValues_Impl::SetBufferValue(this, core::mem::transmute_copy(&key), core::mem::transmute_copy(&pvalue), core::mem::transmute_copy(&cbvalue)).into()
        }
        unsafe extern "system" fn GetBufferValue<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, ppvalue: *mut *mut u8, pcbvalue: *mut u32) -> windows_core::HRESULT
        where
            Identity: IPortableDeviceValues_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPortableDeviceValues_Impl::GetBufferValue(this, core::mem::transmute_copy(&key), core::mem::transmute_copy(&ppvalue), core::mem::transmute_copy(&pcbvalue)).into()
        }
        unsafe extern "system" fn SetIPortableDeviceValuesValue<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPortableDeviceValues_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPortableDeviceValues_Impl::SetIPortableDeviceValuesValue(this, core::mem::transmute_copy(&key), windows_core::from_raw_borrowed(&pvalue)).into()
        }
        unsafe extern "system" fn GetIPortableDeviceValuesValue<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, ppvalue: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPortableDeviceValues_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPortableDeviceValues_Impl::GetIPortableDeviceValuesValue(this, core::mem::transmute_copy(&key)) {
                Ok(ok__) => {
                    ppvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIPortableDevicePropVariantCollectionValue<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPortableDeviceValues_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPortableDeviceValues_Impl::SetIPortableDevicePropVariantCollectionValue(this, core::mem::transmute_copy(&key), windows_core::from_raw_borrowed(&pvalue)).into()
        }
        unsafe extern "system" fn GetIPortableDevicePropVariantCollectionValue<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, ppvalue: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPortableDeviceValues_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPortableDeviceValues_Impl::GetIPortableDevicePropVariantCollectionValue(this, core::mem::transmute_copy(&key)) {
                Ok(ok__) => {
                    ppvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIPortableDeviceKeyCollectionValue<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPortableDeviceValues_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPortableDeviceValues_Impl::SetIPortableDeviceKeyCollectionValue(this, core::mem::transmute_copy(&key), windows_core::from_raw_borrowed(&pvalue)).into()
        }
        unsafe extern "system" fn GetIPortableDeviceKeyCollectionValue<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, ppvalue: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPortableDeviceValues_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPortableDeviceValues_Impl::GetIPortableDeviceKeyCollectionValue(this, core::mem::transmute_copy(&key)) {
                Ok(ok__) => {
                    ppvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIPortableDeviceValuesCollectionValue<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPortableDeviceValues_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPortableDeviceValues_Impl::SetIPortableDeviceValuesCollectionValue(this, core::mem::transmute_copy(&key), windows_core::from_raw_borrowed(&pvalue)).into()
        }
        unsafe extern "system" fn GetIPortableDeviceValuesCollectionValue<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, ppvalue: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPortableDeviceValues_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPortableDeviceValues_Impl::GetIPortableDeviceValuesCollectionValue(this, core::mem::transmute_copy(&key)) {
                Ok(ok__) => {
                    ppvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveValue<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> windows_core::HRESULT
        where
            Identity: IPortableDeviceValues_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPortableDeviceValues_Impl::RemoveValue(this, core::mem::transmute_copy(&key)).into()
        }
        unsafe extern "system" fn CopyValuesFromPropertyStore<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstore: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPortableDeviceValues_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPortableDeviceValues_Impl::CopyValuesFromPropertyStore(this, windows_core::from_raw_borrowed(&pstore)).into()
        }
        unsafe extern "system" fn CopyValuesToPropertyStore<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstore: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPortableDeviceValues_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPortableDeviceValues_Impl::CopyValuesToPropertyStore(this, windows_core::from_raw_borrowed(&pstore)).into()
        }
        unsafe extern "system" fn Clear<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPortableDeviceValues_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPortableDeviceValues_Impl::Clear(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCount: GetCount::<Identity, OFFSET>,
            GetAt: GetAt::<Identity, OFFSET>,
            SetValue: SetValue::<Identity, OFFSET>,
            GetValue: GetValue::<Identity, OFFSET>,
            SetStringValue: SetStringValue::<Identity, OFFSET>,
            GetStringValue: GetStringValue::<Identity, OFFSET>,
            SetUnsignedIntegerValue: SetUnsignedIntegerValue::<Identity, OFFSET>,
            GetUnsignedIntegerValue: GetUnsignedIntegerValue::<Identity, OFFSET>,
            SetSignedIntegerValue: SetSignedIntegerValue::<Identity, OFFSET>,
            GetSignedIntegerValue: GetSignedIntegerValue::<Identity, OFFSET>,
            SetUnsignedLargeIntegerValue: SetUnsignedLargeIntegerValue::<Identity, OFFSET>,
            GetUnsignedLargeIntegerValue: GetUnsignedLargeIntegerValue::<Identity, OFFSET>,
            SetSignedLargeIntegerValue: SetSignedLargeIntegerValue::<Identity, OFFSET>,
            GetSignedLargeIntegerValue: GetSignedLargeIntegerValue::<Identity, OFFSET>,
            SetFloatValue: SetFloatValue::<Identity, OFFSET>,
            GetFloatValue: GetFloatValue::<Identity, OFFSET>,
            SetErrorValue: SetErrorValue::<Identity, OFFSET>,
            GetErrorValue: GetErrorValue::<Identity, OFFSET>,
            SetKeyValue: SetKeyValue::<Identity, OFFSET>,
            GetKeyValue: GetKeyValue::<Identity, OFFSET>,
            SetBoolValue: SetBoolValue::<Identity, OFFSET>,
            GetBoolValue: GetBoolValue::<Identity, OFFSET>,
            SetIUnknownValue: SetIUnknownValue::<Identity, OFFSET>,
            GetIUnknownValue: GetIUnknownValue::<Identity, OFFSET>,
            SetGuidValue: SetGuidValue::<Identity, OFFSET>,
            GetGuidValue: GetGuidValue::<Identity, OFFSET>,
            SetBufferValue: SetBufferValue::<Identity, OFFSET>,
            GetBufferValue: GetBufferValue::<Identity, OFFSET>,
            SetIPortableDeviceValuesValue: SetIPortableDeviceValuesValue::<Identity, OFFSET>,
            GetIPortableDeviceValuesValue: GetIPortableDeviceValuesValue::<Identity, OFFSET>,
            SetIPortableDevicePropVariantCollectionValue: SetIPortableDevicePropVariantCollectionValue::<Identity, OFFSET>,
            GetIPortableDevicePropVariantCollectionValue: GetIPortableDevicePropVariantCollectionValue::<Identity, OFFSET>,
            SetIPortableDeviceKeyCollectionValue: SetIPortableDeviceKeyCollectionValue::<Identity, OFFSET>,
            GetIPortableDeviceKeyCollectionValue: GetIPortableDeviceKeyCollectionValue::<Identity, OFFSET>,
            SetIPortableDeviceValuesCollectionValue: SetIPortableDeviceValuesCollectionValue::<Identity, OFFSET>,
            GetIPortableDeviceValuesCollectionValue: GetIPortableDeviceValuesCollectionValue::<Identity, OFFSET>,
            RemoveValue: RemoveValue::<Identity, OFFSET>,
            CopyValuesFromPropertyStore: CopyValuesFromPropertyStore::<Identity, OFFSET>,
            CopyValuesToPropertyStore: CopyValuesToPropertyStore::<Identity, OFFSET>,
            Clear: Clear::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPortableDeviceValues as windows_core::Interface>::IID
    }
}
pub trait IPortableDeviceValuesCollection_Impl: Sized {
    fn GetCount(&self, pcelems: *const u32) -> windows_core::Result<()>;
    fn GetAt(&self, dwindex: u32) -> windows_core::Result<IPortableDeviceValues>;
    fn Add(&self, pvalues: Option<&IPortableDeviceValues>) -> windows_core::Result<()>;
    fn Clear(&self) -> windows_core::Result<()>;
    fn RemoveAt(&self, dwindex: u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IPortableDeviceValuesCollection {}
impl IPortableDeviceValuesCollection_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IPortableDeviceValuesCollection_Vtbl
    where
        Identity: IPortableDeviceValuesCollection_Impl,
    {
        unsafe extern "system" fn GetCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcelems: *const u32) -> windows_core::HRESULT
        where
            Identity: IPortableDeviceValuesCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPortableDeviceValuesCollection_Impl::GetCount(this, core::mem::transmute_copy(&pcelems)).into()
        }
        unsafe extern "system" fn GetAt<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwindex: u32, ppvalues: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPortableDeviceValuesCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPortableDeviceValuesCollection_Impl::GetAt(this, core::mem::transmute_copy(&dwindex)) {
                Ok(ok__) => {
                    ppvalues.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Add<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalues: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPortableDeviceValuesCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPortableDeviceValuesCollection_Impl::Add(this, windows_core::from_raw_borrowed(&pvalues)).into()
        }
        unsafe extern "system" fn Clear<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPortableDeviceValuesCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPortableDeviceValuesCollection_Impl::Clear(this).into()
        }
        unsafe extern "system" fn RemoveAt<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwindex: u32) -> windows_core::HRESULT
        where
            Identity: IPortableDeviceValuesCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPortableDeviceValuesCollection_Impl::RemoveAt(this, core::mem::transmute_copy(&dwindex)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCount: GetCount::<Identity, OFFSET>,
            GetAt: GetAt::<Identity, OFFSET>,
            Add: Add::<Identity, OFFSET>,
            Clear: Clear::<Identity, OFFSET>,
            RemoveAt: RemoveAt::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPortableDeviceValuesCollection as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IPortableDeviceWebControl_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn GetDeviceFromId(&self, deviceid: &windows_core::BSTR) -> windows_core::Result<super::super::System::Com::IDispatch>;
    fn GetDeviceFromIdAsync(&self, deviceid: &windows_core::BSTR, pcompletionhandler: Option<&super::super::System::Com::IDispatch>, perrorhandler: Option<&super::super::System::Com::IDispatch>) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IPortableDeviceWebControl {}
#[cfg(feature = "Win32_System_Com")]
impl IPortableDeviceWebControl_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IPortableDeviceWebControl_Vtbl
    where
        Identity: IPortableDeviceWebControl_Impl,
    {
        unsafe extern "system" fn GetDeviceFromId<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, deviceid: core::mem::MaybeUninit<windows_core::BSTR>, ppdevice: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPortableDeviceWebControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPortableDeviceWebControl_Impl::GetDeviceFromId(this, core::mem::transmute(&deviceid)) {
                Ok(ok__) => {
                    ppdevice.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeviceFromIdAsync<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, deviceid: core::mem::MaybeUninit<windows_core::BSTR>, pcompletionhandler: *mut core::ffi::c_void, perrorhandler: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPortableDeviceWebControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPortableDeviceWebControl_Impl::GetDeviceFromIdAsync(this, core::mem::transmute(&deviceid), windows_core::from_raw_borrowed(&pcompletionhandler), windows_core::from_raw_borrowed(&perrorhandler)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            GetDeviceFromId: GetDeviceFromId::<Identity, OFFSET>,
            GetDeviceFromIdAsync: GetDeviceFromIdAsync::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPortableDeviceWebControl as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
pub trait IRadioInstance_Impl: Sized {
    fn GetRadioManagerSignature(&self) -> windows_core::Result<windows_core::GUID>;
    fn GetInstanceSignature(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetFriendlyName(&self, lcid: u32) -> windows_core::Result<windows_core::BSTR>;
    fn GetRadioState(&self) -> windows_core::Result<DEVICE_RADIO_STATE>;
    fn SetRadioState(&self, radiostate: DEVICE_RADIO_STATE, utimeoutsec: u32) -> windows_core::Result<()>;
    fn IsMultiComm(&self) -> super::super::Foundation::BOOL;
    fn IsAssociatingDevice(&self) -> super::super::Foundation::BOOL;
}
impl windows_core::RuntimeName for IRadioInstance {}
impl IRadioInstance_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IRadioInstance_Vtbl
    where
        Identity: IRadioInstance_Impl,
    {
        unsafe extern "system" fn GetRadioManagerSignature<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pguidsignature: *mut windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IRadioInstance_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRadioInstance_Impl::GetRadioManagerSignature(this) {
                Ok(ok__) => {
                    pguidsignature.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetInstanceSignature<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrid: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IRadioInstance_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRadioInstance_Impl::GetInstanceSignature(this) {
                Ok(ok__) => {
                    pbstrid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFriendlyName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lcid: u32, pbstrname: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IRadioInstance_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRadioInstance_Impl::GetFriendlyName(this, core::mem::transmute_copy(&lcid)) {
                Ok(ok__) => {
                    pbstrname.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRadioState<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pradiostate: *mut DEVICE_RADIO_STATE) -> windows_core::HRESULT
        where
            Identity: IRadioInstance_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRadioInstance_Impl::GetRadioState(this) {
                Ok(ok__) => {
                    pradiostate.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRadioState<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, radiostate: DEVICE_RADIO_STATE, utimeoutsec: u32) -> windows_core::HRESULT
        where
            Identity: IRadioInstance_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRadioInstance_Impl::SetRadioState(this, core::mem::transmute_copy(&radiostate), core::mem::transmute_copy(&utimeoutsec)).into()
        }
        unsafe extern "system" fn IsMultiComm<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> super::super::Foundation::BOOL
        where
            Identity: IRadioInstance_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRadioInstance_Impl::IsMultiComm(this)
        }
        unsafe extern "system" fn IsAssociatingDevice<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> super::super::Foundation::BOOL
        where
            Identity: IRadioInstance_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRadioInstance_Impl::IsAssociatingDevice(this)
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetRadioManagerSignature: GetRadioManagerSignature::<Identity, OFFSET>,
            GetInstanceSignature: GetInstanceSignature::<Identity, OFFSET>,
            GetFriendlyName: GetFriendlyName::<Identity, OFFSET>,
            GetRadioState: GetRadioState::<Identity, OFFSET>,
            SetRadioState: SetRadioState::<Identity, OFFSET>,
            IsMultiComm: IsMultiComm::<Identity, OFFSET>,
            IsAssociatingDevice: IsAssociatingDevice::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRadioInstance as windows_core::Interface>::IID
    }
}
pub trait IRadioInstanceCollection_Impl: Sized {
    fn GetCount(&self) -> windows_core::Result<u32>;
    fn GetAt(&self, uindex: u32) -> windows_core::Result<IRadioInstance>;
}
impl windows_core::RuntimeName for IRadioInstanceCollection {}
impl IRadioInstanceCollection_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IRadioInstanceCollection_Vtbl
    where
        Identity: IRadioInstanceCollection_Impl,
    {
        unsafe extern "system" fn GetCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcinstance: *mut u32) -> windows_core::HRESULT
        where
            Identity: IRadioInstanceCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRadioInstanceCollection_Impl::GetCount(this) {
                Ok(ok__) => {
                    pcinstance.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAt<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, uindex: u32, ppradioinstance: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IRadioInstanceCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRadioInstanceCollection_Impl::GetAt(this, core::mem::transmute_copy(&uindex)) {
                Ok(ok__) => {
                    ppradioinstance.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetCount: GetCount::<Identity, OFFSET>, GetAt: GetAt::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRadioInstanceCollection as windows_core::Interface>::IID
    }
}
pub trait IWpdSerializer_Impl: Sized {
    fn GetIPortableDeviceValuesFromBuffer(&self, pbuffer: *const u8, dwinputbufferlength: u32) -> windows_core::Result<IPortableDeviceValues>;
    fn WriteIPortableDeviceValuesToBuffer(&self, dwoutputbufferlength: u32, presults: Option<&IPortableDeviceValues>, pbuffer: *mut u8, pdwbyteswritten: *mut u32) -> windows_core::Result<()>;
    fn GetBufferFromIPortableDeviceValues(&self, psource: Option<&IPortableDeviceValues>, ppbuffer: *mut *mut u8, pdwbuffersize: *mut u32) -> windows_core::Result<()>;
    fn GetSerializedSize(&self, psource: Option<&IPortableDeviceValues>) -> windows_core::Result<u32>;
}
impl windows_core::RuntimeName for IWpdSerializer {}
impl IWpdSerializer_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWpdSerializer_Vtbl
    where
        Identity: IWpdSerializer_Impl,
    {
        unsafe extern "system" fn GetIPortableDeviceValuesFromBuffer<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbuffer: *const u8, dwinputbufferlength: u32, ppparams: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWpdSerializer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWpdSerializer_Impl::GetIPortableDeviceValuesFromBuffer(this, core::mem::transmute_copy(&pbuffer), core::mem::transmute_copy(&dwinputbufferlength)) {
                Ok(ok__) => {
                    ppparams.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WriteIPortableDeviceValuesToBuffer<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwoutputbufferlength: u32, presults: *mut core::ffi::c_void, pbuffer: *mut u8, pdwbyteswritten: *mut u32) -> windows_core::HRESULT
        where
            Identity: IWpdSerializer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWpdSerializer_Impl::WriteIPortableDeviceValuesToBuffer(this, core::mem::transmute_copy(&dwoutputbufferlength), windows_core::from_raw_borrowed(&presults), core::mem::transmute_copy(&pbuffer), core::mem::transmute_copy(&pdwbyteswritten)).into()
        }
        unsafe extern "system" fn GetBufferFromIPortableDeviceValues<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, psource: *mut core::ffi::c_void, ppbuffer: *mut *mut u8, pdwbuffersize: *mut u32) -> windows_core::HRESULT
        where
            Identity: IWpdSerializer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWpdSerializer_Impl::GetBufferFromIPortableDeviceValues(this, windows_core::from_raw_borrowed(&psource), core::mem::transmute_copy(&ppbuffer), core::mem::transmute_copy(&pdwbuffersize)).into()
        }
        unsafe extern "system" fn GetSerializedSize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, psource: *mut core::ffi::c_void, pdwsize: *mut u32) -> windows_core::HRESULT
        where
            Identity: IWpdSerializer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWpdSerializer_Impl::GetSerializedSize(this, windows_core::from_raw_borrowed(&psource)) {
                Ok(ok__) => {
                    pdwsize.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetIPortableDeviceValuesFromBuffer: GetIPortableDeviceValuesFromBuffer::<Identity, OFFSET>,
            WriteIPortableDeviceValuesToBuffer: WriteIPortableDeviceValuesToBuffer::<Identity, OFFSET>,
            GetBufferFromIPortableDeviceValues: GetBufferFromIPortableDeviceValues::<Identity, OFFSET>,
            GetSerializedSize: GetSerializedSize::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWpdSerializer as windows_core::Interface>::IID
    }
}
