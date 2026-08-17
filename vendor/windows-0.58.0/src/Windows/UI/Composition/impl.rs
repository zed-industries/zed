pub trait IAnimationObject_Impl: Sized {
    fn PopulatePropertyInfo(&self, propertyname: &windows_core::HSTRING, propertyinfo: Option<&AnimationPropertyInfo>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IAnimationObject {
    const NAME: &'static str = "Windows.UI.Composition.IAnimationObject";
}
impl IAnimationObject_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IAnimationObject_Vtbl
    where
        Identity: IAnimationObject_Impl,
    {
        unsafe extern "system" fn PopulatePropertyInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, propertyname: core::mem::MaybeUninit<windows_core::HSTRING>, propertyinfo: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IAnimationObject_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAnimationObject_Impl::PopulatePropertyInfo(this, core::mem::transmute(&propertyname), windows_core::from_raw_borrowed(&propertyinfo)).into()
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IAnimationObject, OFFSET>(),
            PopulatePropertyInfo: PopulatePropertyInfo::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAnimationObject as windows_core::Interface>::IID
    }
}
pub trait ICompositionAnimationBase_Impl: Sized {}
impl windows_core::RuntimeName for ICompositionAnimationBase {
    const NAME: &'static str = "Windows.UI.Composition.ICompositionAnimationBase";
}
impl ICompositionAnimationBase_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ICompositionAnimationBase_Vtbl
    where
        Identity: ICompositionAnimationBase_Impl,
    {
        Self { base__: windows_core::IInspectable_Vtbl::new::<Identity, ICompositionAnimationBase, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICompositionAnimationBase as windows_core::Interface>::IID
    }
}
pub trait ICompositionSupportsSystemBackdrop_Impl: Sized {
    fn SystemBackdrop(&self) -> windows_core::Result<CompositionBrush>;
    fn SetSystemBackdrop(&self, value: Option<&CompositionBrush>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ICompositionSupportsSystemBackdrop {
    const NAME: &'static str = "Windows.UI.Composition.ICompositionSupportsSystemBackdrop";
}
impl ICompositionSupportsSystemBackdrop_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ICompositionSupportsSystemBackdrop_Vtbl
    where
        Identity: ICompositionSupportsSystemBackdrop_Impl,
    {
        unsafe extern "system" fn SystemBackdrop<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICompositionSupportsSystemBackdrop_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICompositionSupportsSystemBackdrop_Impl::SystemBackdrop(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSystemBackdrop<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICompositionSupportsSystemBackdrop_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICompositionSupportsSystemBackdrop_Impl::SetSystemBackdrop(this, windows_core::from_raw_borrowed(&value)).into()
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, ICompositionSupportsSystemBackdrop, OFFSET>(),
            SystemBackdrop: SystemBackdrop::<Identity, OFFSET>,
            SetSystemBackdrop: SetSystemBackdrop::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICompositionSupportsSystemBackdrop as windows_core::Interface>::IID
    }
}
pub trait ICompositionSurface_Impl: Sized {}
impl windows_core::RuntimeName for ICompositionSurface {
    const NAME: &'static str = "Windows.UI.Composition.ICompositionSurface";
}
impl ICompositionSurface_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ICompositionSurface_Vtbl
    where
        Identity: ICompositionSurface_Impl,
    {
        Self { base__: windows_core::IInspectable_Vtbl::new::<Identity, ICompositionSurface, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICompositionSurface as windows_core::Interface>::IID
    }
}
pub trait ICompositionSurfaceFacade_Impl: Sized {
    fn GetRealSurface(&self) -> windows_core::Result<ICompositionSurface>;
}
impl windows_core::RuntimeName for ICompositionSurfaceFacade {
    const NAME: &'static str = "Windows.UI.Composition.ICompositionSurfaceFacade";
}
impl ICompositionSurfaceFacade_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ICompositionSurfaceFacade_Vtbl
    where
        Identity: ICompositionSurfaceFacade_Impl,
    {
        unsafe extern "system" fn GetRealSurface<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICompositionSurfaceFacade_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICompositionSurfaceFacade_Impl::GetRealSurface(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, ICompositionSurfaceFacade, OFFSET>(),
            GetRealSurface: GetRealSurface::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICompositionSurfaceFacade as windows_core::Interface>::IID
    }
}
pub trait IVisualElement_Impl: Sized {}
impl windows_core::RuntimeName for IVisualElement {
    const NAME: &'static str = "Windows.UI.Composition.IVisualElement";
}
impl IVisualElement_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IVisualElement_Vtbl
    where
        Identity: IVisualElement_Impl,
    {
        Self { base__: windows_core::IInspectable_Vtbl::new::<Identity, IVisualElement, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVisualElement as windows_core::Interface>::IID
    }
}
pub trait IVisualElement2_Impl: Sized {
    fn GetVisualInternal(&self) -> windows_core::Result<Visual>;
}
impl windows_core::RuntimeName for IVisualElement2 {
    const NAME: &'static str = "Windows.UI.Composition.IVisualElement2";
}
impl IVisualElement2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IVisualElement2_Vtbl
    where
        Identity: IVisualElement2_Impl,
    {
        unsafe extern "system" fn GetVisualInternal<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IVisualElement2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IVisualElement2_Impl::GetVisualInternal(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IInspectable_Vtbl::new::<Identity, IVisualElement2, OFFSET>(), GetVisualInternal: GetVisualInternal::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVisualElement2 as windows_core::Interface>::IID
    }
}
