pub trait IMLOperatorAttributes_Impl: Sized {
    fn GetAttributeElementCount(&self, name: &windows_core::PCSTR, r#type: MLOperatorAttributeType) -> windows_core::Result<u32>;
    fn GetAttribute(&self, name: &windows_core::PCSTR, r#type: MLOperatorAttributeType, elementcount: u32, elementbytesize: usize, value: *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn GetStringAttributeElementLength(&self, name: &windows_core::PCSTR, elementindex: u32) -> windows_core::Result<u32>;
    fn GetStringAttributeElement(&self, name: &windows_core::PCSTR, elementindex: u32, attributeelementbytesize: u32, attributeelement: windows_core::PSTR) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IMLOperatorAttributes {}
impl IMLOperatorAttributes_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMLOperatorAttributes_Impl, const OFFSET: isize>() -> IMLOperatorAttributes_Vtbl {
        unsafe extern "system" fn GetAttributeElementCount<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMLOperatorAttributes_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: windows_core::PCSTR, r#type: MLOperatorAttributeType, elementcount: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IMLOperatorAttributes_Impl::GetAttributeElementCount(this, core::mem::transmute(&name), core::mem::transmute_copy(&r#type)) {
                Ok(ok__) => {
                    core::ptr::write(elementcount, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAttribute<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMLOperatorAttributes_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: windows_core::PCSTR, r#type: MLOperatorAttributeType, elementcount: u32, elementbytesize: usize, value: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IMLOperatorAttributes_Impl::GetAttribute(this, core::mem::transmute(&name), core::mem::transmute_copy(&r#type), core::mem::transmute_copy(&elementcount), core::mem::transmute_copy(&elementbytesize), core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetStringAttributeElementLength<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMLOperatorAttributes_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: windows_core::PCSTR, elementindex: u32, attributeelementbytesize: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IMLOperatorAttributes_Impl::GetStringAttributeElementLength(this, core::mem::transmute(&name), core::mem::transmute_copy(&elementindex)) {
                Ok(ok__) => {
                    core::ptr::write(attributeelementbytesize, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStringAttributeElement<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMLOperatorAttributes_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: windows_core::PCSTR, elementindex: u32, attributeelementbytesize: u32, attributeelement: windows_core::PSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IMLOperatorAttributes_Impl::GetStringAttributeElement(this, core::mem::transmute(&name), core::mem::transmute_copy(&elementindex), core::mem::transmute_copy(&attributeelementbytesize), core::mem::transmute_copy(&attributeelement)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetAttributeElementCount: GetAttributeElementCount::<Identity, Impl, OFFSET>,
            GetAttribute: GetAttribute::<Identity, Impl, OFFSET>,
            GetStringAttributeElementLength: GetStringAttributeElementLength::<Identity, Impl, OFFSET>,
            GetStringAttributeElement: GetStringAttributeElement::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMLOperatorAttributes as windows_core::Interface>::IID
    }
}
pub trait IMLOperatorKernel_Impl: Sized {
    fn Compute(&self, context: Option<&IMLOperatorKernelContext>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IMLOperatorKernel {}
impl IMLOperatorKernel_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMLOperatorKernel_Impl, const OFFSET: isize>() -> IMLOperatorKernel_Vtbl {
        unsafe extern "system" fn Compute<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMLOperatorKernel_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, context: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IMLOperatorKernel_Impl::Compute(this, windows_core::from_raw_borrowed(&context)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Compute: Compute::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMLOperatorKernel as windows_core::Interface>::IID
    }
}
pub trait IMLOperatorKernelContext_Impl: Sized {
    fn GetInputTensor(&self, inputindex: u32) -> windows_core::Result<IMLOperatorTensor>;
    fn GetOutputTensor(&self, outputindex: u32, dimensioncount: u32, dimensionsizes: *const u32) -> windows_core::Result<IMLOperatorTensor>;
    fn GetOutputTensor2(&self, outputindex: u32) -> windows_core::Result<IMLOperatorTensor>;
    fn AllocateTemporaryData(&self, size: usize) -> windows_core::Result<windows_core::IUnknown>;
    fn GetExecutionInterface(&self, executionobject: *mut Option<windows_core::IUnknown>);
}
impl windows_core::RuntimeName for IMLOperatorKernelContext {}
impl IMLOperatorKernelContext_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMLOperatorKernelContext_Impl, const OFFSET: isize>() -> IMLOperatorKernelContext_Vtbl {
        unsafe extern "system" fn GetInputTensor<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMLOperatorKernelContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, inputindex: u32, tensor: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IMLOperatorKernelContext_Impl::GetInputTensor(this, core::mem::transmute_copy(&inputindex)) {
                Ok(ok__) => {
                    core::ptr::write(tensor, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOutputTensor<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMLOperatorKernelContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, outputindex: u32, dimensioncount: u32, dimensionsizes: *const u32, tensor: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IMLOperatorKernelContext_Impl::GetOutputTensor(this, core::mem::transmute_copy(&outputindex), core::mem::transmute_copy(&dimensioncount), core::mem::transmute_copy(&dimensionsizes)) {
                Ok(ok__) => {
                    core::ptr::write(tensor, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOutputTensor2<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMLOperatorKernelContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, outputindex: u32, tensor: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IMLOperatorKernelContext_Impl::GetOutputTensor2(this, core::mem::transmute_copy(&outputindex)) {
                Ok(ok__) => {
                    core::ptr::write(tensor, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AllocateTemporaryData<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMLOperatorKernelContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, size: usize, data: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IMLOperatorKernelContext_Impl::AllocateTemporaryData(this, core::mem::transmute_copy(&size)) {
                Ok(ok__) => {
                    core::ptr::write(data, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetExecutionInterface<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMLOperatorKernelContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, executionobject: *mut *mut core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IMLOperatorKernelContext_Impl::GetExecutionInterface(this, core::mem::transmute_copy(&executionobject))
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetInputTensor: GetInputTensor::<Identity, Impl, OFFSET>,
            GetOutputTensor: GetOutputTensor::<Identity, Impl, OFFSET>,
            GetOutputTensor2: GetOutputTensor2::<Identity, Impl, OFFSET>,
            AllocateTemporaryData: AllocateTemporaryData::<Identity, Impl, OFFSET>,
            GetExecutionInterface: GetExecutionInterface::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMLOperatorKernelContext as windows_core::Interface>::IID
    }
}
pub trait IMLOperatorKernelCreationContext_Impl: Sized + IMLOperatorAttributes_Impl {
    fn GetInputCount(&self) -> u32;
    fn GetOutputCount(&self) -> u32;
    fn IsInputValid(&self, inputindex: u32) -> bool;
    fn IsOutputValid(&self, outputindex: u32) -> bool;
    fn GetInputEdgeDescription(&self, inputindex: u32) -> windows_core::Result<MLOperatorEdgeDescription>;
    fn GetOutputEdgeDescription(&self, outputindex: u32) -> windows_core::Result<MLOperatorEdgeDescription>;
    fn HasTensorShapeDescription(&self) -> bool;
    fn GetTensorShapeDescription(&self) -> windows_core::Result<IMLOperatorTensorShapeDescription>;
    fn GetExecutionInterface(&self, executionobject: *mut Option<windows_core::IUnknown>);
}
impl windows_core::RuntimeName for IMLOperatorKernelCreationContext {}
impl IMLOperatorKernelCreationContext_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMLOperatorKernelCreationContext_Impl, const OFFSET: isize>() -> IMLOperatorKernelCreationContext_Vtbl {
        unsafe extern "system" fn GetInputCount<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMLOperatorKernelCreationContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IMLOperatorKernelCreationContext_Impl::GetInputCount(this)
        }
        unsafe extern "system" fn GetOutputCount<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMLOperatorKernelCreationContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IMLOperatorKernelCreationContext_Impl::GetOutputCount(this)
        }
        unsafe extern "system" fn IsInputValid<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMLOperatorKernelCreationContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, inputindex: u32) -> bool {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IMLOperatorKernelCreationContext_Impl::IsInputValid(this, core::mem::transmute_copy(&inputindex))
        }
        unsafe extern "system" fn IsOutputValid<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMLOperatorKernelCreationContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, outputindex: u32) -> bool {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IMLOperatorKernelCreationContext_Impl::IsOutputValid(this, core::mem::transmute_copy(&outputindex))
        }
        unsafe extern "system" fn GetInputEdgeDescription<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMLOperatorKernelCreationContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, inputindex: u32, edgedescription: *mut MLOperatorEdgeDescription) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IMLOperatorKernelCreationContext_Impl::GetInputEdgeDescription(this, core::mem::transmute_copy(&inputindex)) {
                Ok(ok__) => {
                    core::ptr::write(edgedescription, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOutputEdgeDescription<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMLOperatorKernelCreationContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, outputindex: u32, edgedescription: *mut MLOperatorEdgeDescription) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IMLOperatorKernelCreationContext_Impl::GetOutputEdgeDescription(this, core::mem::transmute_copy(&outputindex)) {
                Ok(ok__) => {
                    core::ptr::write(edgedescription, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HasTensorShapeDescription<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMLOperatorKernelCreationContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> bool {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IMLOperatorKernelCreationContext_Impl::HasTensorShapeDescription(this)
        }
        unsafe extern "system" fn GetTensorShapeDescription<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMLOperatorKernelCreationContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, shapedescription: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IMLOperatorKernelCreationContext_Impl::GetTensorShapeDescription(this) {
                Ok(ok__) => {
                    core::ptr::write(shapedescription, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetExecutionInterface<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMLOperatorKernelCreationContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, executionobject: *mut *mut core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IMLOperatorKernelCreationContext_Impl::GetExecutionInterface(this, core::mem::transmute_copy(&executionobject))
        }
        Self {
            base__: IMLOperatorAttributes_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetInputCount: GetInputCount::<Identity, Impl, OFFSET>,
            GetOutputCount: GetOutputCount::<Identity, Impl, OFFSET>,
            IsInputValid: IsInputValid::<Identity, Impl, OFFSET>,
            IsOutputValid: IsOutputValid::<Identity, Impl, OFFSET>,
            GetInputEdgeDescription: GetInputEdgeDescription::<Identity, Impl, OFFSET>,
            GetOutputEdgeDescription: GetOutputEdgeDescription::<Identity, Impl, OFFSET>,
            HasTensorShapeDescription: HasTensorShapeDescription::<Identity, Impl, OFFSET>,
            GetTensorShapeDescription: GetTensorShapeDescription::<Identity, Impl, OFFSET>,
            GetExecutionInterface: GetExecutionInterface::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMLOperatorKernelCreationContext as windows_core::Interface>::IID || iid == &<IMLOperatorAttributes as windows_core::Interface>::IID
    }
}
pub trait IMLOperatorKernelFactory_Impl: Sized {
    fn CreateKernel(&self, context: Option<&IMLOperatorKernelCreationContext>) -> windows_core::Result<IMLOperatorKernel>;
}
impl windows_core::RuntimeName for IMLOperatorKernelFactory {}
impl IMLOperatorKernelFactory_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMLOperatorKernelFactory_Impl, const OFFSET: isize>() -> IMLOperatorKernelFactory_Vtbl {
        unsafe extern "system" fn CreateKernel<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMLOperatorKernelFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, context: *mut core::ffi::c_void, kernel: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IMLOperatorKernelFactory_Impl::CreateKernel(this, windows_core::from_raw_borrowed(&context)) {
                Ok(ok__) => {
                    core::ptr::write(kernel, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), CreateKernel: CreateKernel::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMLOperatorKernelFactory as windows_core::Interface>::IID
    }
}
pub trait IMLOperatorRegistry_Impl: Sized {
    fn RegisterOperatorSetSchema(&self, operatorsetid: *const MLOperatorSetId, baselineversion: i32, schema: *const *const MLOperatorSchemaDescription, schemacount: u32, typeinferrer: Option<&IMLOperatorTypeInferrer>, shapeinferrer: Option<&IMLOperatorShapeInferrer>) -> windows_core::Result<()>;
    fn RegisterOperatorKernel(&self, operatorkernel: *const MLOperatorKernelDescription, operatorkernelfactory: Option<&IMLOperatorKernelFactory>, shapeinferrer: Option<&IMLOperatorShapeInferrer>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IMLOperatorRegistry {}
impl IMLOperatorRegistry_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMLOperatorRegistry_Impl, const OFFSET: isize>() -> IMLOperatorRegistry_Vtbl {
        unsafe extern "system" fn RegisterOperatorSetSchema<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMLOperatorRegistry_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, operatorsetid: *const MLOperatorSetId, baselineversion: i32, schema: *const *const MLOperatorSchemaDescription, schemacount: u32, typeinferrer: *mut core::ffi::c_void, shapeinferrer: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IMLOperatorRegistry_Impl::RegisterOperatorSetSchema(this, core::mem::transmute_copy(&operatorsetid), core::mem::transmute_copy(&baselineversion), core::mem::transmute_copy(&schema), core::mem::transmute_copy(&schemacount), windows_core::from_raw_borrowed(&typeinferrer), windows_core::from_raw_borrowed(&shapeinferrer)).into()
        }
        unsafe extern "system" fn RegisterOperatorKernel<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMLOperatorRegistry_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, operatorkernel: *const MLOperatorKernelDescription, operatorkernelfactory: *mut core::ffi::c_void, shapeinferrer: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IMLOperatorRegistry_Impl::RegisterOperatorKernel(this, core::mem::transmute_copy(&operatorkernel), windows_core::from_raw_borrowed(&operatorkernelfactory), windows_core::from_raw_borrowed(&shapeinferrer)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            RegisterOperatorSetSchema: RegisterOperatorSetSchema::<Identity, Impl, OFFSET>,
            RegisterOperatorKernel: RegisterOperatorKernel::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMLOperatorRegistry as windows_core::Interface>::IID
    }
}
pub trait IMLOperatorShapeInferenceContext_Impl: Sized + IMLOperatorAttributes_Impl {
    fn GetInputCount(&self) -> u32;
    fn GetOutputCount(&self) -> u32;
    fn IsInputValid(&self, inputindex: u32) -> bool;
    fn IsOutputValid(&self, outputindex: u32) -> bool;
    fn GetInputEdgeDescription(&self, inputindex: u32) -> windows_core::Result<MLOperatorEdgeDescription>;
    fn GetInputTensorDimensionCount(&self, inputindex: u32) -> windows_core::Result<u32>;
    fn GetInputTensorShape(&self, inputindex: u32, dimensioncount: u32, dimensions: *mut u32) -> windows_core::Result<()>;
    fn SetOutputTensorShape(&self, outputindex: u32, dimensioncount: u32, dimensions: *const u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IMLOperatorShapeInferenceContext {}
impl IMLOperatorShapeInferenceContext_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMLOperatorShapeInferenceContext_Impl, const OFFSET: isize>() -> IMLOperatorShapeInferenceContext_Vtbl {
        unsafe extern "system" fn GetInputCount<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMLOperatorShapeInferenceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IMLOperatorShapeInferenceContext_Impl::GetInputCount(this)
        }
        unsafe extern "system" fn GetOutputCount<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMLOperatorShapeInferenceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IMLOperatorShapeInferenceContext_Impl::GetOutputCount(this)
        }
        unsafe extern "system" fn IsInputValid<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMLOperatorShapeInferenceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, inputindex: u32) -> bool {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IMLOperatorShapeInferenceContext_Impl::IsInputValid(this, core::mem::transmute_copy(&inputindex))
        }
        unsafe extern "system" fn IsOutputValid<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMLOperatorShapeInferenceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, outputindex: u32) -> bool {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IMLOperatorShapeInferenceContext_Impl::IsOutputValid(this, core::mem::transmute_copy(&outputindex))
        }
        unsafe extern "system" fn GetInputEdgeDescription<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMLOperatorShapeInferenceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, inputindex: u32, edgedescription: *mut MLOperatorEdgeDescription) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IMLOperatorShapeInferenceContext_Impl::GetInputEdgeDescription(this, core::mem::transmute_copy(&inputindex)) {
                Ok(ok__) => {
                    core::ptr::write(edgedescription, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetInputTensorDimensionCount<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMLOperatorShapeInferenceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, inputindex: u32, dimensioncount: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IMLOperatorShapeInferenceContext_Impl::GetInputTensorDimensionCount(this, core::mem::transmute_copy(&inputindex)) {
                Ok(ok__) => {
                    core::ptr::write(dimensioncount, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetInputTensorShape<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMLOperatorShapeInferenceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, inputindex: u32, dimensioncount: u32, dimensions: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IMLOperatorShapeInferenceContext_Impl::GetInputTensorShape(this, core::mem::transmute_copy(&inputindex), core::mem::transmute_copy(&dimensioncount), core::mem::transmute_copy(&dimensions)).into()
        }
        unsafe extern "system" fn SetOutputTensorShape<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMLOperatorShapeInferenceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, outputindex: u32, dimensioncount: u32, dimensions: *const u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IMLOperatorShapeInferenceContext_Impl::SetOutputTensorShape(this, core::mem::transmute_copy(&outputindex), core::mem::transmute_copy(&dimensioncount), core::mem::transmute_copy(&dimensions)).into()
        }
        Self {
            base__: IMLOperatorAttributes_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetInputCount: GetInputCount::<Identity, Impl, OFFSET>,
            GetOutputCount: GetOutputCount::<Identity, Impl, OFFSET>,
            IsInputValid: IsInputValid::<Identity, Impl, OFFSET>,
            IsOutputValid: IsOutputValid::<Identity, Impl, OFFSET>,
            GetInputEdgeDescription: GetInputEdgeDescription::<Identity, Impl, OFFSET>,
            GetInputTensorDimensionCount: GetInputTensorDimensionCount::<Identity, Impl, OFFSET>,
            GetInputTensorShape: GetInputTensorShape::<Identity, Impl, OFFSET>,
            SetOutputTensorShape: SetOutputTensorShape::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMLOperatorShapeInferenceContext as windows_core::Interface>::IID || iid == &<IMLOperatorAttributes as windows_core::Interface>::IID
    }
}
pub trait IMLOperatorShapeInferrer_Impl: Sized {
    fn InferOutputShapes(&self, context: Option<&IMLOperatorShapeInferenceContext>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IMLOperatorShapeInferrer {}
impl IMLOperatorShapeInferrer_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMLOperatorShapeInferrer_Impl, const OFFSET: isize>() -> IMLOperatorShapeInferrer_Vtbl {
        unsafe extern "system" fn InferOutputShapes<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMLOperatorShapeInferrer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, context: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IMLOperatorShapeInferrer_Impl::InferOutputShapes(this, windows_core::from_raw_borrowed(&context)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), InferOutputShapes: InferOutputShapes::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMLOperatorShapeInferrer as windows_core::Interface>::IID
    }
}
pub trait IMLOperatorTensor_Impl: Sized {
    fn GetDimensionCount(&self) -> u32;
    fn GetShape(&self, dimensioncount: u32, dimensions: *mut u32) -> windows_core::Result<()>;
    fn GetTensorDataType(&self) -> MLOperatorTensorDataType;
    fn IsCpuData(&self) -> bool;
    fn IsDataInterface(&self) -> bool;
    fn GetData(&self) -> *mut core::ffi::c_void;
    fn GetDataInterface(&self, datainterface: *mut Option<windows_core::IUnknown>);
}
impl windows_core::RuntimeName for IMLOperatorTensor {}
impl IMLOperatorTensor_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMLOperatorTensor_Impl, const OFFSET: isize>() -> IMLOperatorTensor_Vtbl {
        unsafe extern "system" fn GetDimensionCount<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMLOperatorTensor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IMLOperatorTensor_Impl::GetDimensionCount(this)
        }
        unsafe extern "system" fn GetShape<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMLOperatorTensor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dimensioncount: u32, dimensions: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IMLOperatorTensor_Impl::GetShape(this, core::mem::transmute_copy(&dimensioncount), core::mem::transmute_copy(&dimensions)).into()
        }
        unsafe extern "system" fn GetTensorDataType<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMLOperatorTensor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> MLOperatorTensorDataType {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IMLOperatorTensor_Impl::GetTensorDataType(this)
        }
        unsafe extern "system" fn IsCpuData<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMLOperatorTensor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> bool {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IMLOperatorTensor_Impl::IsCpuData(this)
        }
        unsafe extern "system" fn IsDataInterface<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMLOperatorTensor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> bool {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IMLOperatorTensor_Impl::IsDataInterface(this)
        }
        unsafe extern "system" fn GetData<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMLOperatorTensor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IMLOperatorTensor_Impl::GetData(this)
        }
        unsafe extern "system" fn GetDataInterface<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMLOperatorTensor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, datainterface: *mut *mut core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IMLOperatorTensor_Impl::GetDataInterface(this, core::mem::transmute_copy(&datainterface))
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetDimensionCount: GetDimensionCount::<Identity, Impl, OFFSET>,
            GetShape: GetShape::<Identity, Impl, OFFSET>,
            GetTensorDataType: GetTensorDataType::<Identity, Impl, OFFSET>,
            IsCpuData: IsCpuData::<Identity, Impl, OFFSET>,
            IsDataInterface: IsDataInterface::<Identity, Impl, OFFSET>,
            GetData: GetData::<Identity, Impl, OFFSET>,
            GetDataInterface: GetDataInterface::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMLOperatorTensor as windows_core::Interface>::IID
    }
}
pub trait IMLOperatorTensorShapeDescription_Impl: Sized {
    fn GetInputTensorDimensionCount(&self, inputindex: u32) -> windows_core::Result<u32>;
    fn GetInputTensorShape(&self, inputindex: u32, dimensioncount: u32, dimensions: *mut u32) -> windows_core::Result<()>;
    fn HasOutputShapeDescription(&self) -> bool;
    fn GetOutputTensorDimensionCount(&self, outputindex: u32) -> windows_core::Result<u32>;
    fn GetOutputTensorShape(&self, outputindex: u32, dimensioncount: u32, dimensions: *mut u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IMLOperatorTensorShapeDescription {}
impl IMLOperatorTensorShapeDescription_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMLOperatorTensorShapeDescription_Impl, const OFFSET: isize>() -> IMLOperatorTensorShapeDescription_Vtbl {
        unsafe extern "system" fn GetInputTensorDimensionCount<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMLOperatorTensorShapeDescription_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, inputindex: u32, dimensioncount: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IMLOperatorTensorShapeDescription_Impl::GetInputTensorDimensionCount(this, core::mem::transmute_copy(&inputindex)) {
                Ok(ok__) => {
                    core::ptr::write(dimensioncount, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetInputTensorShape<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMLOperatorTensorShapeDescription_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, inputindex: u32, dimensioncount: u32, dimensions: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IMLOperatorTensorShapeDescription_Impl::GetInputTensorShape(this, core::mem::transmute_copy(&inputindex), core::mem::transmute_copy(&dimensioncount), core::mem::transmute_copy(&dimensions)).into()
        }
        unsafe extern "system" fn HasOutputShapeDescription<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMLOperatorTensorShapeDescription_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> bool {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IMLOperatorTensorShapeDescription_Impl::HasOutputShapeDescription(this)
        }
        unsafe extern "system" fn GetOutputTensorDimensionCount<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMLOperatorTensorShapeDescription_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, outputindex: u32, dimensioncount: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IMLOperatorTensorShapeDescription_Impl::GetOutputTensorDimensionCount(this, core::mem::transmute_copy(&outputindex)) {
                Ok(ok__) => {
                    core::ptr::write(dimensioncount, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOutputTensorShape<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMLOperatorTensorShapeDescription_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, outputindex: u32, dimensioncount: u32, dimensions: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IMLOperatorTensorShapeDescription_Impl::GetOutputTensorShape(this, core::mem::transmute_copy(&outputindex), core::mem::transmute_copy(&dimensioncount), core::mem::transmute_copy(&dimensions)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetInputTensorDimensionCount: GetInputTensorDimensionCount::<Identity, Impl, OFFSET>,
            GetInputTensorShape: GetInputTensorShape::<Identity, Impl, OFFSET>,
            HasOutputShapeDescription: HasOutputShapeDescription::<Identity, Impl, OFFSET>,
            GetOutputTensorDimensionCount: GetOutputTensorDimensionCount::<Identity, Impl, OFFSET>,
            GetOutputTensorShape: GetOutputTensorShape::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMLOperatorTensorShapeDescription as windows_core::Interface>::IID
    }
}
pub trait IMLOperatorTypeInferenceContext_Impl: Sized + IMLOperatorAttributes_Impl {
    fn GetInputCount(&self) -> u32;
    fn GetOutputCount(&self) -> u32;
    fn IsInputValid(&self, inputindex: u32) -> bool;
    fn IsOutputValid(&self, outputindex: u32) -> bool;
    fn GetInputEdgeDescription(&self, inputindex: u32) -> windows_core::Result<MLOperatorEdgeDescription>;
    fn SetOutputEdgeDescription(&self, outputindex: u32, edgedescription: *const MLOperatorEdgeDescription) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IMLOperatorTypeInferenceContext {}
impl IMLOperatorTypeInferenceContext_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMLOperatorTypeInferenceContext_Impl, const OFFSET: isize>() -> IMLOperatorTypeInferenceContext_Vtbl {
        unsafe extern "system" fn GetInputCount<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMLOperatorTypeInferenceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IMLOperatorTypeInferenceContext_Impl::GetInputCount(this)
        }
        unsafe extern "system" fn GetOutputCount<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMLOperatorTypeInferenceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IMLOperatorTypeInferenceContext_Impl::GetOutputCount(this)
        }
        unsafe extern "system" fn IsInputValid<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMLOperatorTypeInferenceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, inputindex: u32) -> bool {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IMLOperatorTypeInferenceContext_Impl::IsInputValid(this, core::mem::transmute_copy(&inputindex))
        }
        unsafe extern "system" fn IsOutputValid<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMLOperatorTypeInferenceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, outputindex: u32) -> bool {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IMLOperatorTypeInferenceContext_Impl::IsOutputValid(this, core::mem::transmute_copy(&outputindex))
        }
        unsafe extern "system" fn GetInputEdgeDescription<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMLOperatorTypeInferenceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, inputindex: u32, edgedescription: *mut MLOperatorEdgeDescription) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IMLOperatorTypeInferenceContext_Impl::GetInputEdgeDescription(this, core::mem::transmute_copy(&inputindex)) {
                Ok(ok__) => {
                    core::ptr::write(edgedescription, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOutputEdgeDescription<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMLOperatorTypeInferenceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, outputindex: u32, edgedescription: *const MLOperatorEdgeDescription) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IMLOperatorTypeInferenceContext_Impl::SetOutputEdgeDescription(this, core::mem::transmute_copy(&outputindex), core::mem::transmute_copy(&edgedescription)).into()
        }
        Self {
            base__: IMLOperatorAttributes_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetInputCount: GetInputCount::<Identity, Impl, OFFSET>,
            GetOutputCount: GetOutputCount::<Identity, Impl, OFFSET>,
            IsInputValid: IsInputValid::<Identity, Impl, OFFSET>,
            IsOutputValid: IsOutputValid::<Identity, Impl, OFFSET>,
            GetInputEdgeDescription: GetInputEdgeDescription::<Identity, Impl, OFFSET>,
            SetOutputEdgeDescription: SetOutputEdgeDescription::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMLOperatorTypeInferenceContext as windows_core::Interface>::IID || iid == &<IMLOperatorAttributes as windows_core::Interface>::IID
    }
}
pub trait IMLOperatorTypeInferrer_Impl: Sized {
    fn InferOutputTypes(&self, context: Option<&IMLOperatorTypeInferenceContext>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IMLOperatorTypeInferrer {}
impl IMLOperatorTypeInferrer_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMLOperatorTypeInferrer_Impl, const OFFSET: isize>() -> IMLOperatorTypeInferrer_Vtbl {
        unsafe extern "system" fn InferOutputTypes<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMLOperatorTypeInferrer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, context: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IMLOperatorTypeInferrer_Impl::InferOutputTypes(this, windows_core::from_raw_borrowed(&context)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), InferOutputTypes: InferOutputTypes::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMLOperatorTypeInferrer as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
pub trait IWinMLEvaluationContext_Impl: Sized {
    fn BindValue(&self, pdescriptor: *const WINML_BINDING_DESC) -> windows_core::Result<()>;
    fn GetValueByName(&self, name: &windows_core::PCWSTR) -> windows_core::Result<*mut WINML_BINDING_DESC>;
    fn Clear(&self) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl windows_core::RuntimeName for IWinMLEvaluationContext {}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl IWinMLEvaluationContext_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWinMLEvaluationContext_Impl, const OFFSET: isize>() -> IWinMLEvaluationContext_Vtbl {
        unsafe extern "system" fn BindValue<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWinMLEvaluationContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdescriptor: *const WINML_BINDING_DESC) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IWinMLEvaluationContext_Impl::BindValue(this, core::mem::transmute_copy(&pdescriptor)).into()
        }
        unsafe extern "system" fn GetValueByName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWinMLEvaluationContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: windows_core::PCWSTR, pdescriptor: *mut *mut WINML_BINDING_DESC) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IWinMLEvaluationContext_Impl::GetValueByName(this, core::mem::transmute(&name)) {
                Ok(ok__) => {
                    core::ptr::write(pdescriptor, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clear<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWinMLEvaluationContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IWinMLEvaluationContext_Impl::Clear(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            BindValue: BindValue::<Identity, Impl, OFFSET>,
            GetValueByName: GetValueByName::<Identity, Impl, OFFSET>,
            Clear: Clear::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWinMLEvaluationContext as windows_core::Interface>::IID
    }
}
pub trait IWinMLModel_Impl: Sized {
    fn GetDescription(&self) -> windows_core::Result<*mut WINML_MODEL_DESC>;
    fn EnumerateMetadata(&self, index: u32, pkey: *mut windows_core::PCWSTR, pvalue: *mut windows_core::PCWSTR) -> windows_core::Result<()>;
    fn EnumerateModelInputs(&self, index: u32) -> windows_core::Result<*mut WINML_VARIABLE_DESC>;
    fn EnumerateModelOutputs(&self, index: u32) -> windows_core::Result<*mut WINML_VARIABLE_DESC>;
}
impl windows_core::RuntimeName for IWinMLModel {}
impl IWinMLModel_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWinMLModel_Impl, const OFFSET: isize>() -> IWinMLModel_Vtbl {
        unsafe extern "system" fn GetDescription<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWinMLModel_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppdescription: *mut *mut WINML_MODEL_DESC) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IWinMLModel_Impl::GetDescription(this) {
                Ok(ok__) => {
                    core::ptr::write(ppdescription, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumerateMetadata<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWinMLModel_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, pkey: *mut windows_core::PCWSTR, pvalue: *mut windows_core::PCWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IWinMLModel_Impl::EnumerateMetadata(this, core::mem::transmute_copy(&index), core::mem::transmute_copy(&pkey), core::mem::transmute_copy(&pvalue)).into()
        }
        unsafe extern "system" fn EnumerateModelInputs<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWinMLModel_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, ppinputdescriptor: *mut *mut WINML_VARIABLE_DESC) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IWinMLModel_Impl::EnumerateModelInputs(this, core::mem::transmute_copy(&index)) {
                Ok(ok__) => {
                    core::ptr::write(ppinputdescriptor, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumerateModelOutputs<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWinMLModel_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, ppoutputdescriptor: *mut *mut WINML_VARIABLE_DESC) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IWinMLModel_Impl::EnumerateModelOutputs(this, core::mem::transmute_copy(&index)) {
                Ok(ok__) => {
                    core::ptr::write(ppoutputdescriptor, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetDescription: GetDescription::<Identity, Impl, OFFSET>,
            EnumerateMetadata: EnumerateMetadata::<Identity, Impl, OFFSET>,
            EnumerateModelInputs: EnumerateModelInputs::<Identity, Impl, OFFSET>,
            EnumerateModelOutputs: EnumerateModelOutputs::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWinMLModel as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
pub trait IWinMLRuntime_Impl: Sized {
    fn LoadModel(&self, path: &windows_core::PCWSTR) -> windows_core::Result<IWinMLModel>;
    fn CreateEvaluationContext(&self, device: Option<&super::super::super::Graphics::Direct3D12::ID3D12Device>) -> windows_core::Result<IWinMLEvaluationContext>;
    fn EvaluateModel(&self, pcontext: Option<&IWinMLEvaluationContext>) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl windows_core::RuntimeName for IWinMLRuntime {}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl IWinMLRuntime_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWinMLRuntime_Impl, const OFFSET: isize>() -> IWinMLRuntime_Vtbl {
        unsafe extern "system" fn LoadModel<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWinMLRuntime_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, path: windows_core::PCWSTR, ppmodel: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IWinMLRuntime_Impl::LoadModel(this, core::mem::transmute(&path)) {
                Ok(ok__) => {
                    core::ptr::write(ppmodel, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateEvaluationContext<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWinMLRuntime_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, device: *mut core::ffi::c_void, ppcontext: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IWinMLRuntime_Impl::CreateEvaluationContext(this, windows_core::from_raw_borrowed(&device)) {
                Ok(ok__) => {
                    core::ptr::write(ppcontext, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EvaluateModel<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWinMLRuntime_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcontext: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IWinMLRuntime_Impl::EvaluateModel(this, windows_core::from_raw_borrowed(&pcontext)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            LoadModel: LoadModel::<Identity, Impl, OFFSET>,
            CreateEvaluationContext: CreateEvaluationContext::<Identity, Impl, OFFSET>,
            EvaluateModel: EvaluateModel::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWinMLRuntime as windows_core::Interface>::IID
    }
}
pub trait IWinMLRuntimeFactory_Impl: Sized {
    fn CreateRuntime(&self, runtimetype: WINML_RUNTIME_TYPE) -> windows_core::Result<IWinMLRuntime>;
}
impl windows_core::RuntimeName for IWinMLRuntimeFactory {}
impl IWinMLRuntimeFactory_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWinMLRuntimeFactory_Impl, const OFFSET: isize>() -> IWinMLRuntimeFactory_Vtbl {
        unsafe extern "system" fn CreateRuntime<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWinMLRuntimeFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, runtimetype: WINML_RUNTIME_TYPE, ppruntime: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IWinMLRuntimeFactory_Impl::CreateRuntime(this, core::mem::transmute_copy(&runtimetype)) {
                Ok(ok__) => {
                    core::ptr::write(ppruntime, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), CreateRuntime: CreateRuntime::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWinMLRuntimeFactory as windows_core::Interface>::IID
    }
}
