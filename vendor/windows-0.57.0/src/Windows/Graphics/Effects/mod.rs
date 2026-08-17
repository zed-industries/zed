windows_core::imp::define_interface!(IGraphicsEffect, IGraphicsEffect_Vtbl, 0xcb51c0ce_8fe6_4636_b202_861faa07d8f3);
impl core::ops::Deref for IGraphicsEffect {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IGraphicsEffect, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(IGraphicsEffect, IGraphicsEffectSource);
impl IGraphicsEffect {
    pub fn Name(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Name)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetName(&self, name: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetName)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(name)).ok() }
    }
}
impl windows_core::RuntimeType for IGraphicsEffect {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IGraphicsEffect_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IGraphicsEffectSource, IGraphicsEffectSource_Vtbl, 0x2d8f9ddc_4339_4eb9_9216_f9deb75658a2);
impl core::ops::Deref for IGraphicsEffectSource {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IGraphicsEffectSource, windows_core::IUnknown, windows_core::IInspectable);
impl IGraphicsEffectSource {}
impl windows_core::RuntimeType for IGraphicsEffectSource {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IGraphicsEffectSource_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
#[cfg(feature = "implement")]
core::include!("impl.rs");
