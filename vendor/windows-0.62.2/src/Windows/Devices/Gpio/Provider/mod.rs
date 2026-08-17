#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GpioPinProviderValueChangedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(GpioPinProviderValueChangedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl GpioPinProviderValueChangedEventArgs {
    pub fn Edge(&self) -> windows_core::Result<ProviderGpioPinEdge> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Edge)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Create(edge: ProviderGpioPinEdge) -> windows_core::Result<GpioPinProviderValueChangedEventArgs> {
        Self::IGpioPinProviderValueChangedEventArgsFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Create)(windows_core::Interface::as_raw(this), edge, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    fn IGpioPinProviderValueChangedEventArgsFactory<R, F: FnOnce(&IGpioPinProviderValueChangedEventArgsFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<GpioPinProviderValueChangedEventArgs, IGpioPinProviderValueChangedEventArgsFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for GpioPinProviderValueChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IGpioPinProviderValueChangedEventArgs>();
}
unsafe impl windows_core::Interface for GpioPinProviderValueChangedEventArgs {
    type Vtable = <IGpioPinProviderValueChangedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IGpioPinProviderValueChangedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for GpioPinProviderValueChangedEventArgs {
    const NAME: &'static str = "Windows.Devices.Gpio.Provider.GpioPinProviderValueChangedEventArgs";
}
unsafe impl Send for GpioPinProviderValueChangedEventArgs {}
unsafe impl Sync for GpioPinProviderValueChangedEventArgs {}
windows_core::imp::define_interface!(IGpioControllerProvider, IGpioControllerProvider_Vtbl, 0xad11cec7_19ea_4b21_874f_b91aed4a25db);
impl windows_core::RuntimeType for IGpioControllerProvider {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(IGpioControllerProvider, windows_core::IUnknown, windows_core::IInspectable);
impl IGpioControllerProvider {
    pub fn PinCount(&self) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PinCount)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn OpenPinProvider(&self, pin: i32, sharingmode: ProviderGpioSharingMode) -> windows_core::Result<IGpioPinProvider> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OpenPinProvider)(windows_core::Interface::as_raw(this), pin, sharingmode, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeName for IGpioControllerProvider {
    const NAME: &'static str = "Windows.Devices.Gpio.Provider.IGpioControllerProvider";
}
pub trait IGpioControllerProvider_Impl: windows_core::IUnknownImpl {
    fn PinCount(&self) -> windows_core::Result<i32>;
    fn OpenPinProvider(&self, pin: i32, sharingMode: ProviderGpioSharingMode) -> windows_core::Result<IGpioPinProvider>;
}
impl IGpioControllerProvider_Vtbl {
    pub const fn new<Identity: IGpioControllerProvider_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn PinCount<Identity: IGpioControllerProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGpioControllerProvider_Impl::PinCount(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn OpenPinProvider<Identity: IGpioControllerProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pin: i32, sharingmode: ProviderGpioSharingMode, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGpioControllerProvider_Impl::OpenPinProvider(this, pin, sharingmode) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IGpioControllerProvider, OFFSET>(),
            PinCount: PinCount::<Identity, OFFSET>,
            OpenPinProvider: OpenPinProvider::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IGpioControllerProvider as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IGpioControllerProvider_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub PinCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub OpenPinProvider: unsafe extern "system" fn(*mut core::ffi::c_void, i32, ProviderGpioSharingMode, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IGpioPinProvider, IGpioPinProvider_Vtbl, 0x42344cb7_6abc_40ff_9ce7_73b85301b900);
impl windows_core::RuntimeType for IGpioPinProvider {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(IGpioPinProvider, windows_core::IUnknown, windows_core::IInspectable);
impl IGpioPinProvider {
    pub fn ValueChanged<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<super::super::super::Foundation::TypedEventHandler<IGpioPinProvider, GpioPinProviderValueChangedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ValueChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveValueChanged(&self, token: i64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveValueChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn DebounceTimeout(&self) -> windows_core::Result<super::super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DebounceTimeout)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetDebounceTimeout(&self, value: super::super::super::Foundation::TimeSpan) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetDebounceTimeout)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn PinNumber(&self) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PinNumber)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SharingMode(&self) -> windows_core::Result<ProviderGpioSharingMode> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SharingMode)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsDriveModeSupported(&self, drivemode: ProviderGpioPinDriveMode) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsDriveModeSupported)(windows_core::Interface::as_raw(this), drivemode, &mut result__).map(|| result__)
        }
    }
    pub fn GetDriveMode(&self) -> windows_core::Result<ProviderGpioPinDriveMode> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDriveMode)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetDriveMode(&self, value: ProviderGpioPinDriveMode) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetDriveMode)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Write(&self, value: ProviderGpioPinValue) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Write)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Read(&self) -> windows_core::Result<ProviderGpioPinValue> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Read)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeName for IGpioPinProvider {
    const NAME: &'static str = "Windows.Devices.Gpio.Provider.IGpioPinProvider";
}
pub trait IGpioPinProvider_Impl: windows_core::IUnknownImpl {
    fn ValueChanged(&self, handler: windows_core::Ref<super::super::super::Foundation::TypedEventHandler<IGpioPinProvider, GpioPinProviderValueChangedEventArgs>>) -> windows_core::Result<i64>;
    fn RemoveValueChanged(&self, token: i64) -> windows_core::Result<()>;
    fn DebounceTimeout(&self) -> windows_core::Result<super::super::super::Foundation::TimeSpan>;
    fn SetDebounceTimeout(&self, value: &super::super::super::Foundation::TimeSpan) -> windows_core::Result<()>;
    fn PinNumber(&self) -> windows_core::Result<i32>;
    fn SharingMode(&self) -> windows_core::Result<ProviderGpioSharingMode>;
    fn IsDriveModeSupported(&self, driveMode: ProviderGpioPinDriveMode) -> windows_core::Result<bool>;
    fn GetDriveMode(&self) -> windows_core::Result<ProviderGpioPinDriveMode>;
    fn SetDriveMode(&self, value: ProviderGpioPinDriveMode) -> windows_core::Result<()>;
    fn Write(&self, value: ProviderGpioPinValue) -> windows_core::Result<()>;
    fn Read(&self) -> windows_core::Result<ProviderGpioPinValue>;
}
impl IGpioPinProvider_Vtbl {
    pub const fn new<Identity: IGpioPinProvider_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ValueChanged<Identity: IGpioPinProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, handler: *mut core::ffi::c_void, result__: *mut i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGpioPinProvider_Impl::ValueChanged(this, core::mem::transmute_copy(&handler)) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RemoveValueChanged<Identity: IGpioPinProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, token: i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGpioPinProvider_Impl::RemoveValueChanged(this, token).into()
            }
        }
        unsafe extern "system" fn DebounceTimeout<Identity: IGpioPinProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut super::super::super::Foundation::TimeSpan) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGpioPinProvider_Impl::DebounceTimeout(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetDebounceTimeout<Identity: IGpioPinProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: super::super::super::Foundation::TimeSpan) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGpioPinProvider_Impl::SetDebounceTimeout(this, core::mem::transmute(&value)).into()
            }
        }
        unsafe extern "system" fn PinNumber<Identity: IGpioPinProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGpioPinProvider_Impl::PinNumber(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SharingMode<Identity: IGpioPinProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut ProviderGpioSharingMode) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGpioPinProvider_Impl::SharingMode(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsDriveModeSupported<Identity: IGpioPinProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, drivemode: ProviderGpioPinDriveMode, result__: *mut bool) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGpioPinProvider_Impl::IsDriveModeSupported(this, drivemode) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetDriveMode<Identity: IGpioPinProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut ProviderGpioPinDriveMode) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGpioPinProvider_Impl::GetDriveMode(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetDriveMode<Identity: IGpioPinProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: ProviderGpioPinDriveMode) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGpioPinProvider_Impl::SetDriveMode(this, value).into()
            }
        }
        unsafe extern "system" fn Write<Identity: IGpioPinProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: ProviderGpioPinValue) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGpioPinProvider_Impl::Write(this, value).into()
            }
        }
        unsafe extern "system" fn Read<Identity: IGpioPinProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut ProviderGpioPinValue) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGpioPinProvider_Impl::Read(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IGpioPinProvider, OFFSET>(),
            ValueChanged: ValueChanged::<Identity, OFFSET>,
            RemoveValueChanged: RemoveValueChanged::<Identity, OFFSET>,
            DebounceTimeout: DebounceTimeout::<Identity, OFFSET>,
            SetDebounceTimeout: SetDebounceTimeout::<Identity, OFFSET>,
            PinNumber: PinNumber::<Identity, OFFSET>,
            SharingMode: SharingMode::<Identity, OFFSET>,
            IsDriveModeSupported: IsDriveModeSupported::<Identity, OFFSET>,
            GetDriveMode: GetDriveMode::<Identity, OFFSET>,
            SetDriveMode: SetDriveMode::<Identity, OFFSET>,
            Write: Write::<Identity, OFFSET>,
            Read: Read::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IGpioPinProvider as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IGpioPinProvider_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ValueChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveValueChanged: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub DebounceTimeout: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::TimeSpan) -> windows_core::HRESULT,
    pub SetDebounceTimeout: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::TimeSpan) -> windows_core::HRESULT,
    pub PinNumber: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SharingMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut ProviderGpioSharingMode) -> windows_core::HRESULT,
    pub IsDriveModeSupported: unsafe extern "system" fn(*mut core::ffi::c_void, ProviderGpioPinDriveMode, *mut bool) -> windows_core::HRESULT,
    pub GetDriveMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut ProviderGpioPinDriveMode) -> windows_core::HRESULT,
    pub SetDriveMode: unsafe extern "system" fn(*mut core::ffi::c_void, ProviderGpioPinDriveMode) -> windows_core::HRESULT,
    pub Write: unsafe extern "system" fn(*mut core::ffi::c_void, ProviderGpioPinValue) -> windows_core::HRESULT,
    pub Read: unsafe extern "system" fn(*mut core::ffi::c_void, *mut ProviderGpioPinValue) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IGpioPinProviderValueChangedEventArgs, IGpioPinProviderValueChangedEventArgs_Vtbl, 0x32a6d6f2_3d5b_44cd_8fbe_13a69f2edb24);
impl windows_core::RuntimeType for IGpioPinProviderValueChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IGpioPinProviderValueChangedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Edge: unsafe extern "system" fn(*mut core::ffi::c_void, *mut ProviderGpioPinEdge) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IGpioPinProviderValueChangedEventArgsFactory, IGpioPinProviderValueChangedEventArgsFactory_Vtbl, 0x3ecb0b59_568c_4392_b24a_8a59a902b1f1);
impl windows_core::RuntimeType for IGpioPinProviderValueChangedEventArgsFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IGpioPinProviderValueChangedEventArgsFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, ProviderGpioPinEdge, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IGpioProvider, IGpioProvider_Vtbl, 0x44e82707_08ca_434a_afe0_d61580446f7e);
impl windows_core::RuntimeType for IGpioProvider {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(IGpioProvider, windows_core::IUnknown, windows_core::IInspectable);
impl IGpioProvider {
    pub fn GetControllers(&self) -> windows_core::Result<windows_collections::IVectorView<IGpioControllerProvider>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetControllers)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeName for IGpioProvider {
    const NAME: &'static str = "Windows.Devices.Gpio.Provider.IGpioProvider";
}
pub trait IGpioProvider_Impl: windows_core::IUnknownImpl {
    fn GetControllers(&self) -> windows_core::Result<windows_collections::IVectorView<IGpioControllerProvider>>;
}
impl IGpioProvider_Vtbl {
    pub const fn new<Identity: IGpioProvider_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetControllers<Identity: IGpioProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGpioProvider_Impl::GetControllers(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IInspectable_Vtbl::new::<Identity, IGpioProvider, OFFSET>(), GetControllers: GetControllers::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IGpioProvider as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IGpioProvider_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetControllers: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ProviderGpioPinDriveMode(pub i32);
impl ProviderGpioPinDriveMode {
    pub const Input: Self = Self(0i32);
    pub const Output: Self = Self(1i32);
    pub const InputPullUp: Self = Self(2i32);
    pub const InputPullDown: Self = Self(3i32);
    pub const OutputOpenDrain: Self = Self(4i32);
    pub const OutputOpenDrainPullUp: Self = Self(5i32);
    pub const OutputOpenSource: Self = Self(6i32);
    pub const OutputOpenSourcePullDown: Self = Self(7i32);
}
impl windows_core::TypeKind for ProviderGpioPinDriveMode {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for ProviderGpioPinDriveMode {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Gpio.Provider.ProviderGpioPinDriveMode;i4)");
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ProviderGpioPinEdge(pub i32);
impl ProviderGpioPinEdge {
    pub const FallingEdge: Self = Self(0i32);
    pub const RisingEdge: Self = Self(1i32);
}
impl windows_core::TypeKind for ProviderGpioPinEdge {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for ProviderGpioPinEdge {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Gpio.Provider.ProviderGpioPinEdge;i4)");
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ProviderGpioPinValue(pub i32);
impl ProviderGpioPinValue {
    pub const Low: Self = Self(0i32);
    pub const High: Self = Self(1i32);
}
impl windows_core::TypeKind for ProviderGpioPinValue {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for ProviderGpioPinValue {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Gpio.Provider.ProviderGpioPinValue;i4)");
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ProviderGpioSharingMode(pub i32);
impl ProviderGpioSharingMode {
    pub const Exclusive: Self = Self(0i32);
    pub const SharedReadOnly: Self = Self(1i32);
}
impl windows_core::TypeKind for ProviderGpioSharingMode {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for ProviderGpioSharingMode {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Gpio.Provider.ProviderGpioSharingMode;i4)");
}
