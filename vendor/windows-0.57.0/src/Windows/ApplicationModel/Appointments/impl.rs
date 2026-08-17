pub trait IAppointmentParticipant_Impl: Sized {
    fn DisplayName(&self) -> windows_core::Result<windows_core::HSTRING>;
    fn SetDisplayName(&self, value: &windows_core::HSTRING) -> windows_core::Result<()>;
    fn Address(&self) -> windows_core::Result<windows_core::HSTRING>;
    fn SetAddress(&self, value: &windows_core::HSTRING) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IAppointmentParticipant {
    const NAME: &'static str = "Windows.ApplicationModel.Appointments.IAppointmentParticipant";
}
impl IAppointmentParticipant_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IAppointmentParticipant_Impl, const OFFSET: isize>() -> IAppointmentParticipant_Vtbl {
        unsafe extern "system" fn DisplayName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IAppointmentParticipant_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IAppointmentParticipant_Impl::DisplayName(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDisplayName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IAppointmentParticipant_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IAppointmentParticipant_Impl::SetDisplayName(this, core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn Address<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IAppointmentParticipant_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IAppointmentParticipant_Impl::Address(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAddress<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IAppointmentParticipant_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IAppointmentParticipant_Impl::SetAddress(this, core::mem::transmute(&value)).into()
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IAppointmentParticipant, OFFSET>(),
            DisplayName: DisplayName::<Identity, Impl, OFFSET>,
            SetDisplayName: SetDisplayName::<Identity, Impl, OFFSET>,
            Address: Address::<Identity, Impl, OFFSET>,
            SetAddress: SetAddress::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAppointmentParticipant as windows_core::Interface>::IID
    }
}
