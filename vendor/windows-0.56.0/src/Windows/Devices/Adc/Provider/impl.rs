pub trait IAdcControllerProvider_Impl: Sized {
    fn ChannelCount(&self) -> windows_core::Result<i32>;
    fn ResolutionInBits(&self) -> windows_core::Result<i32>;
    fn MinValue(&self) -> windows_core::Result<i32>;
    fn MaxValue(&self) -> windows_core::Result<i32>;
    fn ChannelMode(&self) -> windows_core::Result<ProviderAdcChannelMode>;
    fn SetChannelMode(&self, value: ProviderAdcChannelMode) -> windows_core::Result<()>;
    fn IsChannelModeSupported(&self, channelmode: ProviderAdcChannelMode) -> windows_core::Result<bool>;
    fn AcquireChannel(&self, channel: i32) -> windows_core::Result<()>;
    fn ReleaseChannel(&self, channel: i32) -> windows_core::Result<()>;
    fn ReadValue(&self, channelnumber: i32) -> windows_core::Result<i32>;
}
impl windows_core::RuntimeName for IAdcControllerProvider {
    const NAME: &'static str = "Windows.Devices.Adc.Provider.IAdcControllerProvider";
}
impl IAdcControllerProvider_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IAdcControllerProvider_Impl, const OFFSET: isize>() -> IAdcControllerProvider_Vtbl {
        unsafe extern "system" fn ChannelCount<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IAdcControllerProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IAdcControllerProvider_Impl::ChannelCount(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ResolutionInBits<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IAdcControllerProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IAdcControllerProvider_Impl::ResolutionInBits(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MinValue<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IAdcControllerProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IAdcControllerProvider_Impl::MinValue(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MaxValue<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IAdcControllerProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IAdcControllerProvider_Impl::MaxValue(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ChannelMode<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IAdcControllerProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut ProviderAdcChannelMode) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IAdcControllerProvider_Impl::ChannelMode(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetChannelMode<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IAdcControllerProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: ProviderAdcChannelMode) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IAdcControllerProvider_Impl::SetChannelMode(this, value).into()
        }
        unsafe extern "system" fn IsChannelModeSupported<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IAdcControllerProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, channelmode: ProviderAdcChannelMode, result__: *mut bool) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IAdcControllerProvider_Impl::IsChannelModeSupported(this, channelmode) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AcquireChannel<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IAdcControllerProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, channel: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IAdcControllerProvider_Impl::AcquireChannel(this, channel).into()
        }
        unsafe extern "system" fn ReleaseChannel<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IAdcControllerProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, channel: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IAdcControllerProvider_Impl::ReleaseChannel(this, channel).into()
        }
        unsafe extern "system" fn ReadValue<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IAdcControllerProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, channelnumber: i32, result__: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IAdcControllerProvider_Impl::ReadValue(this, channelnumber) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IAdcControllerProvider, OFFSET>(),
            ChannelCount: ChannelCount::<Identity, Impl, OFFSET>,
            ResolutionInBits: ResolutionInBits::<Identity, Impl, OFFSET>,
            MinValue: MinValue::<Identity, Impl, OFFSET>,
            MaxValue: MaxValue::<Identity, Impl, OFFSET>,
            ChannelMode: ChannelMode::<Identity, Impl, OFFSET>,
            SetChannelMode: SetChannelMode::<Identity, Impl, OFFSET>,
            IsChannelModeSupported: IsChannelModeSupported::<Identity, Impl, OFFSET>,
            AcquireChannel: AcquireChannel::<Identity, Impl, OFFSET>,
            ReleaseChannel: ReleaseChannel::<Identity, Impl, OFFSET>,
            ReadValue: ReadValue::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAdcControllerProvider as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Foundation_Collections")]
pub trait IAdcProvider_Impl: Sized {
    fn GetControllers(&self) -> windows_core::Result<super::super::super::Foundation::Collections::IVectorView<IAdcControllerProvider>>;
}
#[cfg(feature = "Foundation_Collections")]
impl windows_core::RuntimeName for IAdcProvider {
    const NAME: &'static str = "Windows.Devices.Adc.Provider.IAdcProvider";
}
#[cfg(feature = "Foundation_Collections")]
impl IAdcProvider_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IAdcProvider_Impl, const OFFSET: isize>() -> IAdcProvider_Vtbl {
        unsafe extern "system" fn GetControllers<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IAdcProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IAdcProvider_Impl::GetControllers(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IInspectable_Vtbl::new::<Identity, IAdcProvider, OFFSET>(), GetControllers: GetControllers::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAdcProvider as windows_core::Interface>::IID
    }
}
