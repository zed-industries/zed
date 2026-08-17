windows_core::imp::define_interface!(ICompositionDebugHeatMaps, ICompositionDebugHeatMaps_Vtbl, 0xe49c90ac_2ff3_5805_718c_b725ee07650f);
impl windows_core::RuntimeType for ICompositionDebugHeatMaps {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICompositionDebugHeatMaps_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Hide: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ShowMemoryUsage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ShowOverdraw: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, CompositionDebugOverdrawContentKinds) -> windows_core::HRESULT,
    pub ShowRedraw: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICompositionDebugSettings, ICompositionDebugSettings_Vtbl, 0x2831987e_1d82_4d38_b7b7_efd11c7bc3d1);
impl windows_core::RuntimeType for ICompositionDebugSettings {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICompositionDebugSettings_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub HeatMaps: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICompositionDebugSettingsStatics, ICompositionDebugSettingsStatics_Vtbl, 0x64ec1f1e_6af8_4af8_b814_c870fd5a9505);
impl windows_core::RuntimeType for ICompositionDebugSettingsStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICompositionDebugSettingsStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub TryGetSettings: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct CompositionDebugHeatMaps(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(CompositionDebugHeatMaps, windows_core::IUnknown, windows_core::IInspectable);
impl CompositionDebugHeatMaps {
    pub fn Hide<P0>(&self, subtree: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Visual>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Hide)(windows_core::Interface::as_raw(this), subtree.param().abi()).ok() }
    }
    pub fn ShowMemoryUsage<P0>(&self, subtree: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Visual>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).ShowMemoryUsage)(windows_core::Interface::as_raw(this), subtree.param().abi()).ok() }
    }
    pub fn ShowOverdraw<P0>(&self, subtree: P0, contentkinds: CompositionDebugOverdrawContentKinds) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Visual>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).ShowOverdraw)(windows_core::Interface::as_raw(this), subtree.param().abi(), contentkinds).ok() }
    }
    pub fn ShowRedraw<P0>(&self, subtree: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Visual>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).ShowRedraw)(windows_core::Interface::as_raw(this), subtree.param().abi()).ok() }
    }
}
impl windows_core::RuntimeType for CompositionDebugHeatMaps {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ICompositionDebugHeatMaps>();
}
unsafe impl windows_core::Interface for CompositionDebugHeatMaps {
    type Vtable = ICompositionDebugHeatMaps_Vtbl;
    const IID: windows_core::GUID = <ICompositionDebugHeatMaps as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for CompositionDebugHeatMaps {
    const NAME: &'static str = "Windows.UI.Composition.Diagnostics.CompositionDebugHeatMaps";
}
unsafe impl Send for CompositionDebugHeatMaps {}
unsafe impl Sync for CompositionDebugHeatMaps {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct CompositionDebugSettings(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(CompositionDebugSettings, windows_core::IUnknown, windows_core::IInspectable);
impl CompositionDebugSettings {
    pub fn HeatMaps(&self) -> windows_core::Result<CompositionDebugHeatMaps> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HeatMaps)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn TryGetSettings<P0>(compositor: P0) -> windows_core::Result<CompositionDebugSettings>
    where
        P0: windows_core::Param<super::Compositor>,
    {
        Self::ICompositionDebugSettingsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryGetSettings)(windows_core::Interface::as_raw(this), compositor.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn ICompositionDebugSettingsStatics<R, F: FnOnce(&ICompositionDebugSettingsStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<CompositionDebugSettings, ICompositionDebugSettingsStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for CompositionDebugSettings {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ICompositionDebugSettings>();
}
unsafe impl windows_core::Interface for CompositionDebugSettings {
    type Vtable = ICompositionDebugSettings_Vtbl;
    const IID: windows_core::GUID = <ICompositionDebugSettings as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for CompositionDebugSettings {
    const NAME: &'static str = "Windows.UI.Composition.Diagnostics.CompositionDebugSettings";
}
unsafe impl Send for CompositionDebugSettings {}
unsafe impl Sync for CompositionDebugSettings {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CompositionDebugOverdrawContentKinds(pub u32);
impl CompositionDebugOverdrawContentKinds {
    pub const None: Self = Self(0u32);
    pub const OffscreenRendered: Self = Self(1u32);
    pub const Colors: Self = Self(2u32);
    pub const Effects: Self = Self(4u32);
    pub const Shadows: Self = Self(8u32);
    pub const Lights: Self = Self(16u32);
    pub const Surfaces: Self = Self(32u32);
    pub const SwapChains: Self = Self(64u32);
    pub const All: Self = Self(4294967295u32);
}
impl windows_core::TypeKind for CompositionDebugOverdrawContentKinds {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CompositionDebugOverdrawContentKinds {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CompositionDebugOverdrawContentKinds").field(&self.0).finish()
    }
}
impl CompositionDebugOverdrawContentKinds {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for CompositionDebugOverdrawContentKinds {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for CompositionDebugOverdrawContentKinds {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for CompositionDebugOverdrawContentKinds {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for CompositionDebugOverdrawContentKinds {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for CompositionDebugOverdrawContentKinds {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl windows_core::RuntimeType for CompositionDebugOverdrawContentKinds {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.UI.Composition.Diagnostics.CompositionDebugOverdrawContentKinds;u4)");
}
