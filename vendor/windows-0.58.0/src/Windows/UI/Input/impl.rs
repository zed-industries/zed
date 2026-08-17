pub trait IPointerPointTransform_Impl: Sized {
    fn Inverse(&self) -> windows_core::Result<IPointerPointTransform>;
    fn TryTransform(&self, inpoint: &super::super::Foundation::Point, outpoint: &mut super::super::Foundation::Point) -> windows_core::Result<bool>;
    fn TransformBounds(&self, rect: &super::super::Foundation::Rect) -> windows_core::Result<super::super::Foundation::Rect>;
}
impl windows_core::RuntimeName for IPointerPointTransform {
    const NAME: &'static str = "Windows.UI.Input.IPointerPointTransform";
}
impl IPointerPointTransform_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IPointerPointTransform_Vtbl
    where
        Identity: IPointerPointTransform_Impl,
    {
        unsafe extern "system" fn Inverse<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPointerPointTransform_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPointerPointTransform_Impl::Inverse(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryTransform<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, inpoint: super::super::Foundation::Point, outpoint: *mut super::super::Foundation::Point, result__: *mut bool) -> windows_core::HRESULT
        where
            Identity: IPointerPointTransform_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPointerPointTransform_Impl::TryTransform(this, core::mem::transmute(&inpoint), core::mem::transmute_copy(&outpoint)) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TransformBounds<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, rect: super::super::Foundation::Rect, result__: *mut super::super::Foundation::Rect) -> windows_core::HRESULT
        where
            Identity: IPointerPointTransform_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPointerPointTransform_Impl::TransformBounds(this, core::mem::transmute(&rect)) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IPointerPointTransform, OFFSET>(),
            Inverse: Inverse::<Identity, OFFSET>,
            TryTransform: TryTransform::<Identity, OFFSET>,
            TransformBounds: TransformBounds::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPointerPointTransform as windows_core::Interface>::IID
    }
}
