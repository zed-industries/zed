pub trait IPropertyAnimation_Impl: Sized {
    fn Type(&self) -> windows_core::Result<PropertyAnimationType>;
    fn Delay(&self) -> windows_core::Result<super::super::super::Foundation::TimeSpan>;
    fn Duration(&self) -> windows_core::Result<super::super::super::Foundation::TimeSpan>;
    fn Control1(&self) -> windows_core::Result<super::super::super::Foundation::Point>;
    fn Control2(&self) -> windows_core::Result<super::super::super::Foundation::Point>;
}
impl windows_core::RuntimeName for IPropertyAnimation {
    const NAME: &'static str = "Windows.UI.Core.AnimationMetrics.IPropertyAnimation";
}
impl IPropertyAnimation_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPropertyAnimation_Impl, const OFFSET: isize>() -> IPropertyAnimation_Vtbl {
        unsafe extern "system" fn Type<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPropertyAnimation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut PropertyAnimationType) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IPropertyAnimation_Impl::Type(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Delay<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPropertyAnimation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut super::super::super::Foundation::TimeSpan) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IPropertyAnimation_Impl::Delay(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Duration<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPropertyAnimation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut super::super::super::Foundation::TimeSpan) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IPropertyAnimation_Impl::Duration(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Control1<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPropertyAnimation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut super::super::super::Foundation::Point) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IPropertyAnimation_Impl::Control1(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Control2<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPropertyAnimation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut super::super::super::Foundation::Point) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IPropertyAnimation_Impl::Control2(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IPropertyAnimation, OFFSET>(),
            Type: Type::<Identity, Impl, OFFSET>,
            Delay: Delay::<Identity, Impl, OFFSET>,
            Duration: Duration::<Identity, Impl, OFFSET>,
            Control1: Control1::<Identity, Impl, OFFSET>,
            Control2: Control2::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPropertyAnimation as windows_core::Interface>::IID
    }
}
