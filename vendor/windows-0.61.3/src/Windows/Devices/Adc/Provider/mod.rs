windows_core::imp::define_interface!(IAdcControllerProvider, IAdcControllerProvider_Vtbl, 0xbe545828_816d_4de5_a048_aba06958aaa8);
impl windows_core::RuntimeType for IAdcControllerProvider {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(IAdcControllerProvider, windows_core::IUnknown, windows_core::IInspectable);
impl IAdcControllerProvider {
    pub fn ChannelCount(&self) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ChannelCount)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ResolutionInBits(&self) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ResolutionInBits)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn MinValue(&self) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MinValue)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn MaxValue(&self) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MaxValue)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ChannelMode(&self) -> windows_core::Result<ProviderAdcChannelMode> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ChannelMode)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetChannelMode(&self, value: ProviderAdcChannelMode) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetChannelMode)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsChannelModeSupported(&self, channelmode: ProviderAdcChannelMode) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsChannelModeSupported)(windows_core::Interface::as_raw(this), channelmode, &mut result__).map(|| result__)
        }
    }
    pub fn AcquireChannel(&self, channel: i32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).AcquireChannel)(windows_core::Interface::as_raw(this), channel).ok() }
    }
    pub fn ReleaseChannel(&self, channel: i32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).ReleaseChannel)(windows_core::Interface::as_raw(this), channel).ok() }
    }
    pub fn ReadValue(&self, channelnumber: i32) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReadValue)(windows_core::Interface::as_raw(this), channelnumber, &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeName for IAdcControllerProvider {
    const NAME: &'static str = "Windows.Devices.Adc.Provider.IAdcControllerProvider";
}
pub trait IAdcControllerProvider_Impl: windows_core::IUnknownImpl {
    fn ChannelCount(&self) -> windows_core::Result<i32>;
    fn ResolutionInBits(&self) -> windows_core::Result<i32>;
    fn MinValue(&self) -> windows_core::Result<i32>;
    fn MaxValue(&self) -> windows_core::Result<i32>;
    fn ChannelMode(&self) -> windows_core::Result<ProviderAdcChannelMode>;
    fn SetChannelMode(&self, value: ProviderAdcChannelMode) -> windows_core::Result<()>;
    fn IsChannelModeSupported(&self, channelMode: ProviderAdcChannelMode) -> windows_core::Result<bool>;
    fn AcquireChannel(&self, channel: i32) -> windows_core::Result<()>;
    fn ReleaseChannel(&self, channel: i32) -> windows_core::Result<()>;
    fn ReadValue(&self, channelNumber: i32) -> windows_core::Result<i32>;
}
impl IAdcControllerProvider_Vtbl {
    pub const fn new<Identity: IAdcControllerProvider_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ChannelCount<Identity: IAdcControllerProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAdcControllerProvider_Impl::ChannelCount(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ResolutionInBits<Identity: IAdcControllerProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAdcControllerProvider_Impl::ResolutionInBits(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn MinValue<Identity: IAdcControllerProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAdcControllerProvider_Impl::MinValue(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn MaxValue<Identity: IAdcControllerProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAdcControllerProvider_Impl::MaxValue(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ChannelMode<Identity: IAdcControllerProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut ProviderAdcChannelMode) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAdcControllerProvider_Impl::ChannelMode(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetChannelMode<Identity: IAdcControllerProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: ProviderAdcChannelMode) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAdcControllerProvider_Impl::SetChannelMode(this, value).into()
            }
        }
        unsafe extern "system" fn IsChannelModeSupported<Identity: IAdcControllerProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, channelmode: ProviderAdcChannelMode, result__: *mut bool) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAdcControllerProvider_Impl::IsChannelModeSupported(this, channelmode) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn AcquireChannel<Identity: IAdcControllerProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, channel: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAdcControllerProvider_Impl::AcquireChannel(this, channel).into()
            }
        }
        unsafe extern "system" fn ReleaseChannel<Identity: IAdcControllerProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, channel: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAdcControllerProvider_Impl::ReleaseChannel(this, channel).into()
            }
        }
        unsafe extern "system" fn ReadValue<Identity: IAdcControllerProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, channelnumber: i32, result__: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAdcControllerProvider_Impl::ReadValue(this, channelnumber) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IAdcControllerProvider, OFFSET>(),
            ChannelCount: ChannelCount::<Identity, OFFSET>,
            ResolutionInBits: ResolutionInBits::<Identity, OFFSET>,
            MinValue: MinValue::<Identity, OFFSET>,
            MaxValue: MaxValue::<Identity, OFFSET>,
            ChannelMode: ChannelMode::<Identity, OFFSET>,
            SetChannelMode: SetChannelMode::<Identity, OFFSET>,
            IsChannelModeSupported: IsChannelModeSupported::<Identity, OFFSET>,
            AcquireChannel: AcquireChannel::<Identity, OFFSET>,
            ReleaseChannel: ReleaseChannel::<Identity, OFFSET>,
            ReadValue: ReadValue::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAdcControllerProvider as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAdcControllerProvider_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ChannelCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub ResolutionInBits: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub MinValue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub MaxValue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub ChannelMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut ProviderAdcChannelMode) -> windows_core::HRESULT,
    pub SetChannelMode: unsafe extern "system" fn(*mut core::ffi::c_void, ProviderAdcChannelMode) -> windows_core::HRESULT,
    pub IsChannelModeSupported: unsafe extern "system" fn(*mut core::ffi::c_void, ProviderAdcChannelMode, *mut bool) -> windows_core::HRESULT,
    pub AcquireChannel: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub ReleaseChannel: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub ReadValue: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut i32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAdcProvider, IAdcProvider_Vtbl, 0x28953668_9359_4c57_bc88_e275e81638c9);
impl windows_core::RuntimeType for IAdcProvider {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(IAdcProvider, windows_core::IUnknown, windows_core::IInspectable);
impl IAdcProvider {
    pub fn GetControllers(&self) -> windows_core::Result<windows_collections::IVectorView<IAdcControllerProvider>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetControllers)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeName for IAdcProvider {
    const NAME: &'static str = "Windows.Devices.Adc.Provider.IAdcProvider";
}
pub trait IAdcProvider_Impl: windows_core::IUnknownImpl {
    fn GetControllers(&self) -> windows_core::Result<windows_collections::IVectorView<IAdcControllerProvider>>;
}
impl IAdcProvider_Vtbl {
    pub const fn new<Identity: IAdcProvider_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetControllers<Identity: IAdcProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAdcProvider_Impl::GetControllers(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IInspectable_Vtbl::new::<Identity, IAdcProvider, OFFSET>(), GetControllers: GetControllers::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAdcProvider as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAdcProvider_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetControllers: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ProviderAdcChannelMode(pub i32);
impl ProviderAdcChannelMode {
    pub const SingleEnded: Self = Self(0i32);
    pub const Differential: Self = Self(1i32);
}
impl windows_core::TypeKind for ProviderAdcChannelMode {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for ProviderAdcChannelMode {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Adc.Provider.ProviderAdcChannelMode;i4)");
}
