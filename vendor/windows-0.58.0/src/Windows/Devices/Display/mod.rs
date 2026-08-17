#[cfg(feature = "Devices_Display_Core")]
pub mod Core;
windows_core::imp::define_interface!(IDisplayMonitor, IDisplayMonitor_Vtbl, 0x1f6b15d4_1d01_4c51_87e2_6f954a772b59);
impl windows_core::RuntimeType for IDisplayMonitor {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IDisplayMonitor_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub DeviceId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub DisplayName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub ConnectionKind: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DisplayMonitorConnectionKind) -> windows_core::HRESULT,
    pub PhysicalConnector: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DisplayMonitorPhysicalConnectorKind) -> windows_core::HRESULT,
    pub DisplayAdapterDeviceId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    #[cfg(feature = "Graphics")]
    pub DisplayAdapterId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Graphics::DisplayAdapterId) -> windows_core::HRESULT,
    #[cfg(not(feature = "Graphics"))]
    DisplayAdapterId: usize,
    pub DisplayAdapterTargetId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub UsageKind: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DisplayMonitorUsageKind) -> windows_core::HRESULT,
    #[cfg(feature = "Graphics")]
    pub NativeResolutionInRawPixels: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Graphics::SizeInt32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Graphics"))]
    NativeResolutionInRawPixels: usize,
    pub PhysicalSizeInInches: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RawDpiX: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub RawDpiY: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub RedPrimary: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::Point) -> windows_core::HRESULT,
    pub GreenPrimary: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::Point) -> windows_core::HRESULT,
    pub BluePrimary: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::Point) -> windows_core::HRESULT,
    pub WhitePoint: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::Point) -> windows_core::HRESULT,
    pub MaxLuminanceInNits: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub MinLuminanceInNits: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub MaxAverageFullFrameLuminanceInNits: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub GetDescriptor: unsafe extern "system" fn(*mut core::ffi::c_void, DisplayMonitorDescriptorKind, *mut u32, *mut *mut u8) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDisplayMonitor2, IDisplayMonitor2_Vtbl, 0x023018e6_cb23_5830_96df_a7bf6e602577);
impl windows_core::RuntimeType for IDisplayMonitor2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IDisplayMonitor2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IsDolbyVisionSupportedInHdrMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDisplayMonitorStatics, IDisplayMonitorStatics_Vtbl, 0x6eae698f_a228_4c05_821d_b695d667de8e);
impl windows_core::RuntimeType for IDisplayMonitorStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IDisplayMonitorStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetDeviceSelector: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub FromIdAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FromInterfaceIdAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct DisplayMonitor(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(DisplayMonitor, windows_core::IUnknown, windows_core::IInspectable);
impl DisplayMonitor {
    pub fn DeviceId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DeviceId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn DisplayName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DisplayName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ConnectionKind(&self) -> windows_core::Result<DisplayMonitorConnectionKind> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ConnectionKind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PhysicalConnector(&self) -> windows_core::Result<DisplayMonitorPhysicalConnectorKind> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PhysicalConnector)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn DisplayAdapterDeviceId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DisplayAdapterDeviceId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Graphics")]
    pub fn DisplayAdapterId(&self) -> windows_core::Result<super::super::Graphics::DisplayAdapterId> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DisplayAdapterId)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn DisplayAdapterTargetId(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DisplayAdapterTargetId)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn UsageKind(&self) -> windows_core::Result<DisplayMonitorUsageKind> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UsageKind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Graphics")]
    pub fn NativeResolutionInRawPixels(&self) -> windows_core::Result<super::super::Graphics::SizeInt32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NativeResolutionInRawPixels)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PhysicalSizeInInches(&self) -> windows_core::Result<super::super::Foundation::IReference<super::super::Foundation::Size>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PhysicalSizeInInches)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn RawDpiX(&self) -> windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RawDpiX)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn RawDpiY(&self) -> windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RawDpiY)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn RedPrimary(&self) -> windows_core::Result<super::super::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RedPrimary)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn GreenPrimary(&self) -> windows_core::Result<super::super::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GreenPrimary)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn BluePrimary(&self) -> windows_core::Result<super::super::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BluePrimary)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn WhitePoint(&self) -> windows_core::Result<super::super::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).WhitePoint)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn MaxLuminanceInNits(&self) -> windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MaxLuminanceInNits)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn MinLuminanceInNits(&self) -> windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MinLuminanceInNits)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn MaxAverageFullFrameLuminanceInNits(&self) -> windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MaxAverageFullFrameLuminanceInNits)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn GetDescriptor(&self, descriptorkind: DisplayMonitorDescriptorKind) -> windows_core::Result<windows_core::Array<u8>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::MaybeUninit::zeroed();
            (windows_core::Interface::vtable(this).GetDescriptor)(windows_core::Interface::as_raw(this), descriptorkind, windows_core::Array::<u8>::set_abi_len(core::mem::transmute(&mut result__)), result__.as_mut_ptr() as *mut _ as _).map(|| result__.assume_init())
        }
    }
    pub fn IsDolbyVisionSupportedInHdrMode(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IDisplayMonitor2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsDolbyVisionSupportedInHdrMode)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn GetDeviceSelector() -> windows_core::Result<windows_core::HSTRING> {
        Self::IDisplayMonitorStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDeviceSelector)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn FromIdAsync(deviceid: &windows_core::HSTRING) -> windows_core::Result<super::super::Foundation::IAsyncOperation<DisplayMonitor>> {
        Self::IDisplayMonitorStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FromIdAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(deviceid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn FromInterfaceIdAsync(deviceinterfaceid: &windows_core::HSTRING) -> windows_core::Result<super::super::Foundation::IAsyncOperation<DisplayMonitor>> {
        Self::IDisplayMonitorStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FromInterfaceIdAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(deviceinterfaceid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IDisplayMonitorStatics<R, F: FnOnce(&IDisplayMonitorStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<DisplayMonitor, IDisplayMonitorStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for DisplayMonitor {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IDisplayMonitor>();
}
unsafe impl windows_core::Interface for DisplayMonitor {
    type Vtable = IDisplayMonitor_Vtbl;
    const IID: windows_core::GUID = <IDisplayMonitor as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for DisplayMonitor {
    const NAME: &'static str = "Windows.Devices.Display.DisplayMonitor";
}
unsafe impl Send for DisplayMonitor {}
unsafe impl Sync for DisplayMonitor {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DisplayMonitorConnectionKind(pub i32);
impl DisplayMonitorConnectionKind {
    pub const Internal: Self = Self(0i32);
    pub const Wired: Self = Self(1i32);
    pub const Wireless: Self = Self(2i32);
    pub const Virtual: Self = Self(3i32);
}
impl windows_core::TypeKind for DisplayMonitorConnectionKind {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DisplayMonitorConnectionKind {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DisplayMonitorConnectionKind").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for DisplayMonitorConnectionKind {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Display.DisplayMonitorConnectionKind;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DisplayMonitorDescriptorKind(pub i32);
impl DisplayMonitorDescriptorKind {
    pub const Edid: Self = Self(0i32);
    pub const DisplayId: Self = Self(1i32);
}
impl windows_core::TypeKind for DisplayMonitorDescriptorKind {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DisplayMonitorDescriptorKind {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DisplayMonitorDescriptorKind").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for DisplayMonitorDescriptorKind {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Display.DisplayMonitorDescriptorKind;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DisplayMonitorPhysicalConnectorKind(pub i32);
impl DisplayMonitorPhysicalConnectorKind {
    pub const Unknown: Self = Self(0i32);
    pub const HD15: Self = Self(1i32);
    pub const AnalogTV: Self = Self(2i32);
    pub const Dvi: Self = Self(3i32);
    pub const Hdmi: Self = Self(4i32);
    pub const Lvds: Self = Self(5i32);
    pub const Sdi: Self = Self(6i32);
    pub const DisplayPort: Self = Self(7i32);
}
impl windows_core::TypeKind for DisplayMonitorPhysicalConnectorKind {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DisplayMonitorPhysicalConnectorKind {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DisplayMonitorPhysicalConnectorKind").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for DisplayMonitorPhysicalConnectorKind {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Display.DisplayMonitorPhysicalConnectorKind;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DisplayMonitorUsageKind(pub i32);
impl DisplayMonitorUsageKind {
    pub const Standard: Self = Self(0i32);
    pub const HeadMounted: Self = Self(1i32);
    pub const SpecialPurpose: Self = Self(2i32);
}
impl windows_core::TypeKind for DisplayMonitorUsageKind {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DisplayMonitorUsageKind {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DisplayMonitorUsageKind").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for DisplayMonitorUsageKind {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Display.DisplayMonitorUsageKind;i4)");
}
