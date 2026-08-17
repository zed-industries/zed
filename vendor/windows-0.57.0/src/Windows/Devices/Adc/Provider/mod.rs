windows_core::imp::define_interface!(IAdcControllerProvider, IAdcControllerProvider_Vtbl, 0xbe545828_816d_4de5_a048_aba06958aaa8);
impl core::ops::Deref for IAdcControllerProvider {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
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
impl windows_core::RuntimeType for IAdcControllerProvider {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
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
impl core::ops::Deref for IAdcProvider {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IAdcProvider, windows_core::IUnknown, windows_core::IInspectable);
impl IAdcProvider {
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetControllers(&self) -> windows_core::Result<super::super::super::Foundation::Collections::IVectorView<IAdcControllerProvider>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetControllers)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for IAdcProvider {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAdcProvider_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub GetControllers: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetControllers: usize,
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct ProviderAdcChannelMode(pub i32);
impl ProviderAdcChannelMode {
    pub const SingleEnded: Self = Self(0i32);
    pub const Differential: Self = Self(1i32);
}
impl windows_core::TypeKind for ProviderAdcChannelMode {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for ProviderAdcChannelMode {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ProviderAdcChannelMode").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for ProviderAdcChannelMode {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Adc.Provider.ProviderAdcChannelMode;i4)");
}
#[cfg(feature = "implement")]
core::include!("impl.rs");
