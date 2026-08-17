windows_core::imp::define_interface!(IGraphicsEffect, IGraphicsEffect_Vtbl, 0xcb51c0ce_8fe6_4636_b202_861faa07d8f3);
impl windows_core::RuntimeType for IGraphicsEffect {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(IGraphicsEffect, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(IGraphicsEffect, IGraphicsEffectSource);
impl IGraphicsEffect {
    pub fn Name(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Name)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetName(&self, name: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetName)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(name)).ok() }
    }
}
impl windows_core::RuntimeName for IGraphicsEffect {
    const NAME: &'static str = "Windows.Graphics.Effects.IGraphicsEffect";
}
pub trait IGraphicsEffect_Impl: IGraphicsEffectSource_Impl {
    fn Name(&self) -> windows_core::Result<windows_core::HSTRING>;
    fn SetName(&self, name: &windows_core::HSTRING) -> windows_core::Result<()>;
}
impl IGraphicsEffect_Vtbl {
    pub const fn new<Identity: IGraphicsEffect_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Name<Identity: IGraphicsEffect_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGraphicsEffect_Impl::Name(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetName<Identity: IGraphicsEffect_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGraphicsEffect_Impl::SetName(this, core::mem::transmute(&name)).into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IGraphicsEffect, OFFSET>(),
            Name: Name::<Identity, OFFSET>,
            SetName: SetName::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IGraphicsEffect as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IGraphicsEffect_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IGraphicsEffectSource, IGraphicsEffectSource_Vtbl, 0x2d8f9ddc_4339_4eb9_9216_f9deb75658a2);
impl windows_core::RuntimeType for IGraphicsEffectSource {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(IGraphicsEffectSource, windows_core::IUnknown, windows_core::IInspectable);
impl windows_core::RuntimeName for IGraphicsEffectSource {
    const NAME: &'static str = "Windows.Graphics.Effects.IGraphicsEffectSource";
}
pub trait IGraphicsEffectSource_Impl: windows_core::IUnknownImpl {}
impl IGraphicsEffectSource_Vtbl {
    pub const fn new<Identity: IGraphicsEffectSource_Impl, const OFFSET: isize>() -> Self {
        Self { base__: windows_core::IInspectable_Vtbl::new::<Identity, IGraphicsEffectSource, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IGraphicsEffectSource as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IGraphicsEffectSource_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
