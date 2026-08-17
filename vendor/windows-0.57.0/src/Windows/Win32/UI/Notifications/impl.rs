pub trait INotificationActivationCallback_Impl: Sized {
    fn Activate(&self, appusermodelid: &windows_core::PCWSTR, invokedargs: &windows_core::PCWSTR, data: *const NOTIFICATION_USER_INPUT_DATA, count: u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for INotificationActivationCallback {}
impl INotificationActivationCallback_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INotificationActivationCallback_Impl, const OFFSET: isize>() -> INotificationActivationCallback_Vtbl {
        unsafe extern "system" fn Activate<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INotificationActivationCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, appusermodelid: windows_core::PCWSTR, invokedargs: windows_core::PCWSTR, data: *const NOTIFICATION_USER_INPUT_DATA, count: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            INotificationActivationCallback_Impl::Activate(this, core::mem::transmute(&appusermodelid), core::mem::transmute(&invokedargs), core::mem::transmute_copy(&data), core::mem::transmute_copy(&count)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Activate: Activate::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<INotificationActivationCallback as windows_core::Interface>::IID
    }
}
