pub trait ILearningModelFeatureDescriptor_Impl: Sized {
    fn Name(&self) -> windows_core::Result<windows_core::HSTRING>;
    fn Description(&self) -> windows_core::Result<windows_core::HSTRING>;
    fn Kind(&self) -> windows_core::Result<LearningModelFeatureKind>;
    fn IsRequired(&self) -> windows_core::Result<bool>;
}
impl windows_core::RuntimeName for ILearningModelFeatureDescriptor {
    const NAME: &'static str = "Windows.AI.MachineLearning.ILearningModelFeatureDescriptor";
}
impl ILearningModelFeatureDescriptor_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ILearningModelFeatureDescriptor_Vtbl
    where
        Identity: ILearningModelFeatureDescriptor_Impl,
    {
        unsafe extern "system" fn Name<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT
        where
            Identity: ILearningModelFeatureDescriptor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ILearningModelFeatureDescriptor_Impl::Name(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Description<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT
        where
            Identity: ILearningModelFeatureDescriptor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ILearningModelFeatureDescriptor_Impl::Description(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Kind<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut LearningModelFeatureKind) -> windows_core::HRESULT
        where
            Identity: ILearningModelFeatureDescriptor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ILearningModelFeatureDescriptor_Impl::Kind(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsRequired<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT
        where
            Identity: ILearningModelFeatureDescriptor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ILearningModelFeatureDescriptor_Impl::IsRequired(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, ILearningModelFeatureDescriptor, OFFSET>(),
            Name: Name::<Identity, OFFSET>,
            Description: Description::<Identity, OFFSET>,
            Kind: Kind::<Identity, OFFSET>,
            IsRequired: IsRequired::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ILearningModelFeatureDescriptor as windows_core::Interface>::IID
    }
}
pub trait ILearningModelFeatureValue_Impl: Sized {
    fn Kind(&self) -> windows_core::Result<LearningModelFeatureKind>;
}
impl windows_core::RuntimeName for ILearningModelFeatureValue {
    const NAME: &'static str = "Windows.AI.MachineLearning.ILearningModelFeatureValue";
}
impl ILearningModelFeatureValue_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ILearningModelFeatureValue_Vtbl
    where
        Identity: ILearningModelFeatureValue_Impl,
    {
        unsafe extern "system" fn Kind<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut LearningModelFeatureKind) -> windows_core::HRESULT
        where
            Identity: ILearningModelFeatureValue_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ILearningModelFeatureValue_Impl::Kind(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IInspectable_Vtbl::new::<Identity, ILearningModelFeatureValue, OFFSET>(), Kind: Kind::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ILearningModelFeatureValue as windows_core::Interface>::IID
    }
}
pub trait ILearningModelOperatorProvider_Impl: Sized {}
impl windows_core::RuntimeName for ILearningModelOperatorProvider {
    const NAME: &'static str = "Windows.AI.MachineLearning.ILearningModelOperatorProvider";
}
impl ILearningModelOperatorProvider_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ILearningModelOperatorProvider_Vtbl
    where
        Identity: ILearningModelOperatorProvider_Impl,
    {
        Self { base__: windows_core::IInspectable_Vtbl::new::<Identity, ILearningModelOperatorProvider, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ILearningModelOperatorProvider as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Foundation_Collections")]
pub trait ITensor_Impl: Sized + ILearningModelFeatureValue_Impl {
    fn TensorKind(&self) -> windows_core::Result<TensorKind>;
    fn Shape(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<i64>>;
}
#[cfg(feature = "Foundation_Collections")]
impl windows_core::RuntimeName for ITensor {
    const NAME: &'static str = "Windows.AI.MachineLearning.ITensor";
}
#[cfg(feature = "Foundation_Collections")]
impl ITensor_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITensor_Vtbl
    where
        Identity: ITensor_Impl,
    {
        unsafe extern "system" fn TensorKind<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut TensorKind) -> windows_core::HRESULT
        where
            Identity: ITensor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITensor_Impl::TensorKind(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Shape<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITensor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITensor_Impl::Shape(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, ITensor, OFFSET>(),
            TensorKind: TensorKind::<Identity, OFFSET>,
            Shape: Shape::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITensor as windows_core::Interface>::IID
    }
}
