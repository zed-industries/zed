pub trait IWCNConnectNotify_Impl: Sized {
    fn ConnectSucceeded(&self) -> windows_core::Result<()>;
    fn ConnectFailed(&self, hrfailure: windows_core::HRESULT) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWCNConnectNotify {}
impl IWCNConnectNotify_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWCNConnectNotify_Impl, const OFFSET: isize>() -> IWCNConnectNotify_Vtbl {
        unsafe extern "system" fn ConnectSucceeded<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWCNConnectNotify_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IWCNConnectNotify_Impl::ConnectSucceeded(this).into()
        }
        unsafe extern "system" fn ConnectFailed<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWCNConnectNotify_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hrfailure: windows_core::HRESULT) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IWCNConnectNotify_Impl::ConnectFailed(this, core::mem::transmute_copy(&hrfailure)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            ConnectSucceeded: ConnectSucceeded::<Identity, Impl, OFFSET>,
            ConnectFailed: ConnectFailed::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWCNConnectNotify as windows_core::Interface>::IID
    }
}
pub trait IWCNDevice_Impl: Sized {
    fn SetPassword(&self, r#type: WCN_PASSWORD_TYPE, dwpasswordlength: u32, pbpassword: *const u8) -> windows_core::Result<()>;
    fn Connect(&self, pnotify: Option<&IWCNConnectNotify>) -> windows_core::Result<()>;
    fn GetAttribute(&self, attributetype: WCN_ATTRIBUTE_TYPE, dwmaxbuffersize: u32, pbbuffer: *mut u8, pdwbufferused: *mut u32) -> windows_core::Result<()>;
    fn GetIntegerAttribute(&self, attributetype: WCN_ATTRIBUTE_TYPE) -> windows_core::Result<u32>;
    fn GetStringAttribute(&self, attributetype: WCN_ATTRIBUTE_TYPE, cchmaxstring: u32, wszstring: windows_core::PWSTR) -> windows_core::Result<()>;
    fn GetNetworkProfile(&self, cchmaxstringlength: u32, wszprofile: windows_core::PWSTR) -> windows_core::Result<()>;
    fn SetNetworkProfile(&self, pszprofilexml: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetVendorExtension(&self, pvendorextspec: *const WCN_VENDOR_EXTENSION_SPEC, dwmaxbuffersize: u32, pbbuffer: *mut u8, pdwbufferused: *mut u32) -> windows_core::Result<()>;
    fn SetVendorExtension(&self, pvendorextspec: *const WCN_VENDOR_EXTENSION_SPEC, cbbuffer: u32, pbbuffer: *const u8) -> windows_core::Result<()>;
    fn Unadvise(&self) -> windows_core::Result<()>;
    fn SetNFCPasswordParams(&self, r#type: WCN_PASSWORD_TYPE, dwoobpasswordid: u32, dwpasswordlength: u32, pbpassword: *const u8, dwremotepublickeyhashlength: u32, pbremotepublickeyhash: *const u8, dwdhkeybloblength: u32, pbdhkeyblob: *const u8) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWCNDevice {}
impl IWCNDevice_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWCNDevice_Impl, const OFFSET: isize>() -> IWCNDevice_Vtbl {
        unsafe extern "system" fn SetPassword<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWCNDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, r#type: WCN_PASSWORD_TYPE, dwpasswordlength: u32, pbpassword: *const u8) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IWCNDevice_Impl::SetPassword(this, core::mem::transmute_copy(&r#type), core::mem::transmute_copy(&dwpasswordlength), core::mem::transmute_copy(&pbpassword)).into()
        }
        unsafe extern "system" fn Connect<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWCNDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnotify: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IWCNDevice_Impl::Connect(this, windows_core::from_raw_borrowed(&pnotify)).into()
        }
        unsafe extern "system" fn GetAttribute<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWCNDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, attributetype: WCN_ATTRIBUTE_TYPE, dwmaxbuffersize: u32, pbbuffer: *mut u8, pdwbufferused: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IWCNDevice_Impl::GetAttribute(this, core::mem::transmute_copy(&attributetype), core::mem::transmute_copy(&dwmaxbuffersize), core::mem::transmute_copy(&pbbuffer), core::mem::transmute_copy(&pdwbufferused)).into()
        }
        unsafe extern "system" fn GetIntegerAttribute<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWCNDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, attributetype: WCN_ATTRIBUTE_TYPE, puinteger: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IWCNDevice_Impl::GetIntegerAttribute(this, core::mem::transmute_copy(&attributetype)) {
                Ok(ok__) => {
                    core::ptr::write(puinteger, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStringAttribute<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWCNDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, attributetype: WCN_ATTRIBUTE_TYPE, cchmaxstring: u32, wszstring: windows_core::PWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IWCNDevice_Impl::GetStringAttribute(this, core::mem::transmute_copy(&attributetype), core::mem::transmute_copy(&cchmaxstring), core::mem::transmute_copy(&wszstring)).into()
        }
        unsafe extern "system" fn GetNetworkProfile<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWCNDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cchmaxstringlength: u32, wszprofile: windows_core::PWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IWCNDevice_Impl::GetNetworkProfile(this, core::mem::transmute_copy(&cchmaxstringlength), core::mem::transmute_copy(&wszprofile)).into()
        }
        unsafe extern "system" fn SetNetworkProfile<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWCNDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszprofilexml: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IWCNDevice_Impl::SetNetworkProfile(this, core::mem::transmute(&pszprofilexml)).into()
        }
        unsafe extern "system" fn GetVendorExtension<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWCNDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvendorextspec: *const WCN_VENDOR_EXTENSION_SPEC, dwmaxbuffersize: u32, pbbuffer: *mut u8, pdwbufferused: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IWCNDevice_Impl::GetVendorExtension(this, core::mem::transmute_copy(&pvendorextspec), core::mem::transmute_copy(&dwmaxbuffersize), core::mem::transmute_copy(&pbbuffer), core::mem::transmute_copy(&pdwbufferused)).into()
        }
        unsafe extern "system" fn SetVendorExtension<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWCNDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvendorextspec: *const WCN_VENDOR_EXTENSION_SPEC, cbbuffer: u32, pbbuffer: *const u8) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IWCNDevice_Impl::SetVendorExtension(this, core::mem::transmute_copy(&pvendorextspec), core::mem::transmute_copy(&cbbuffer), core::mem::transmute_copy(&pbbuffer)).into()
        }
        unsafe extern "system" fn Unadvise<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWCNDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IWCNDevice_Impl::Unadvise(this).into()
        }
        unsafe extern "system" fn SetNFCPasswordParams<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWCNDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, r#type: WCN_PASSWORD_TYPE, dwoobpasswordid: u32, dwpasswordlength: u32, pbpassword: *const u8, dwremotepublickeyhashlength: u32, pbremotepublickeyhash: *const u8, dwdhkeybloblength: u32, pbdhkeyblob: *const u8) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IWCNDevice_Impl::SetNFCPasswordParams(this, core::mem::transmute_copy(&r#type), core::mem::transmute_copy(&dwoobpasswordid), core::mem::transmute_copy(&dwpasswordlength), core::mem::transmute_copy(&pbpassword), core::mem::transmute_copy(&dwremotepublickeyhashlength), core::mem::transmute_copy(&pbremotepublickeyhash), core::mem::transmute_copy(&dwdhkeybloblength), core::mem::transmute_copy(&pbdhkeyblob)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetPassword: SetPassword::<Identity, Impl, OFFSET>,
            Connect: Connect::<Identity, Impl, OFFSET>,
            GetAttribute: GetAttribute::<Identity, Impl, OFFSET>,
            GetIntegerAttribute: GetIntegerAttribute::<Identity, Impl, OFFSET>,
            GetStringAttribute: GetStringAttribute::<Identity, Impl, OFFSET>,
            GetNetworkProfile: GetNetworkProfile::<Identity, Impl, OFFSET>,
            SetNetworkProfile: SetNetworkProfile::<Identity, Impl, OFFSET>,
            GetVendorExtension: GetVendorExtension::<Identity, Impl, OFFSET>,
            SetVendorExtension: SetVendorExtension::<Identity, Impl, OFFSET>,
            Unadvise: Unadvise::<Identity, Impl, OFFSET>,
            SetNFCPasswordParams: SetNFCPasswordParams::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWCNDevice as windows_core::Interface>::IID
    }
}
