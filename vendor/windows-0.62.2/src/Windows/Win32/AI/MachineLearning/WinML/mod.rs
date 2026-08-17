#[inline]
pub unsafe fn MLCreateOperatorRegistry() -> windows_core::Result<IMLOperatorRegistry> {
    windows_core::link!("windows.ai.machinelearning.dll" "system" fn MLCreateOperatorRegistry(registry : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        MLCreateOperatorRegistry(&mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[inline]
pub unsafe fn WinMLCreateRuntime() -> windows_core::Result<IWinMLRuntime> {
    windows_core::link!("winml.dll" "system" fn WinMLCreateRuntime(runtime : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        WinMLCreateRuntime(&mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
windows_core::imp::define_interface!(IMLOperatorAttributes, IMLOperatorAttributes_Vtbl, 0x4b1b1759_ec40_466c_aab4_beb5347fd24c);
windows_core::imp::interface_hierarchy!(IMLOperatorAttributes, windows_core::IUnknown);
impl IMLOperatorAttributes {
    pub unsafe fn GetAttributeElementCount<P0>(&self, name: P0, r#type: MLOperatorAttributeType) -> windows_core::Result<u32>
    where
        P0: windows_core::Param<windows_core::PCSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetAttributeElementCount)(windows_core::Interface::as_raw(self), name.param().abi(), r#type, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetAttribute<P0>(&self, name: P0, r#type: MLOperatorAttributeType, elementcount: u32, elementbytesize: usize, value: *mut core::ffi::c_void) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetAttribute)(windows_core::Interface::as_raw(self), name.param().abi(), r#type, elementcount, elementbytesize, value as _).ok() }
    }
    pub unsafe fn GetStringAttributeElementLength<P0>(&self, name: P0, elementindex: u32) -> windows_core::Result<u32>
    where
        P0: windows_core::Param<windows_core::PCSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetStringAttributeElementLength)(windows_core::Interface::as_raw(self), name.param().abi(), elementindex, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetStringAttributeElement<P0>(&self, name: P0, elementindex: u32, attributeelement: &mut [u8]) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetStringAttributeElement)(windows_core::Interface::as_raw(self), name.param().abi(), elementindex, attributeelement.len().try_into().unwrap(), core::mem::transmute(attributeelement.as_ptr())).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMLOperatorAttributes_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetAttributeElementCount: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCSTR, MLOperatorAttributeType, *mut u32) -> windows_core::HRESULT,
    pub GetAttribute: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCSTR, MLOperatorAttributeType, u32, usize, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetStringAttributeElementLength: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCSTR, u32, *mut u32) -> windows_core::HRESULT,
    pub GetStringAttributeElement: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCSTR, u32, u32, windows_core::PSTR) -> windows_core::HRESULT,
}
pub trait IMLOperatorAttributes_Impl: windows_core::IUnknownImpl {
    fn GetAttributeElementCount(&self, name: &windows_core::PCSTR, r#type: MLOperatorAttributeType) -> windows_core::Result<u32>;
    fn GetAttribute(&self, name: &windows_core::PCSTR, r#type: MLOperatorAttributeType, elementcount: u32, elementbytesize: usize, value: *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn GetStringAttributeElementLength(&self, name: &windows_core::PCSTR, elementindex: u32) -> windows_core::Result<u32>;
    fn GetStringAttributeElement(&self, name: &windows_core::PCSTR, elementindex: u32, attributeelementbytesize: u32, attributeelement: windows_core::PSTR) -> windows_core::Result<()>;
}
impl IMLOperatorAttributes_Vtbl {
    pub const fn new<Identity: IMLOperatorAttributes_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetAttributeElementCount<Identity: IMLOperatorAttributes_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: windows_core::PCSTR, r#type: MLOperatorAttributeType, elementcount: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMLOperatorAttributes_Impl::GetAttributeElementCount(this, core::mem::transmute(&name), core::mem::transmute_copy(&r#type)) {
                    Ok(ok__) => {
                        elementcount.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetAttribute<Identity: IMLOperatorAttributes_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: windows_core::PCSTR, r#type: MLOperatorAttributeType, elementcount: u32, elementbytesize: usize, value: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMLOperatorAttributes_Impl::GetAttribute(this, core::mem::transmute(&name), core::mem::transmute_copy(&r#type), core::mem::transmute_copy(&elementcount), core::mem::transmute_copy(&elementbytesize), core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn GetStringAttributeElementLength<Identity: IMLOperatorAttributes_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: windows_core::PCSTR, elementindex: u32, attributeelementbytesize: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMLOperatorAttributes_Impl::GetStringAttributeElementLength(this, core::mem::transmute(&name), core::mem::transmute_copy(&elementindex)) {
                    Ok(ok__) => {
                        attributeelementbytesize.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetStringAttributeElement<Identity: IMLOperatorAttributes_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: windows_core::PCSTR, elementindex: u32, attributeelementbytesize: u32, attributeelement: windows_core::PSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMLOperatorAttributes_Impl::GetStringAttributeElement(this, core::mem::transmute(&name), core::mem::transmute_copy(&elementindex), core::mem::transmute_copy(&attributeelementbytesize), core::mem::transmute_copy(&attributeelement)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetAttributeElementCount: GetAttributeElementCount::<Identity, OFFSET>,
            GetAttribute: GetAttribute::<Identity, OFFSET>,
            GetStringAttributeElementLength: GetStringAttributeElementLength::<Identity, OFFSET>,
            GetStringAttributeElement: GetStringAttributeElement::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMLOperatorAttributes as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMLOperatorAttributes {}
windows_core::imp::define_interface!(IMLOperatorKernel, IMLOperatorKernel_Vtbl, 0x11c4b4a0_b467_4eaa_a1a6_b961d8d0ed79);
windows_core::imp::interface_hierarchy!(IMLOperatorKernel, windows_core::IUnknown);
impl IMLOperatorKernel {
    pub unsafe fn Compute<P0>(&self, context: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMLOperatorKernelContext>,
    {
        unsafe { (windows_core::Interface::vtable(self).Compute)(windows_core::Interface::as_raw(self), context.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMLOperatorKernel_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Compute: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IMLOperatorKernel_Impl: windows_core::IUnknownImpl {
    fn Compute(&self, context: windows_core::Ref<IMLOperatorKernelContext>) -> windows_core::Result<()>;
}
impl IMLOperatorKernel_Vtbl {
    pub const fn new<Identity: IMLOperatorKernel_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Compute<Identity: IMLOperatorKernel_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, context: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMLOperatorKernel_Impl::Compute(this, core::mem::transmute_copy(&context)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Compute: Compute::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMLOperatorKernel as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMLOperatorKernel {}
windows_core::imp::define_interface!(IMLOperatorKernelContext, IMLOperatorKernelContext_Vtbl, 0x82536a28_f022_4769_9d3f_8b278f84c0c3);
windows_core::imp::interface_hierarchy!(IMLOperatorKernelContext, windows_core::IUnknown);
impl IMLOperatorKernelContext {
    pub unsafe fn GetInputTensor(&self, inputindex: u32) -> windows_core::Result<IMLOperatorTensor> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetInputTensor)(windows_core::Interface::as_raw(self), inputindex, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetOutputTensor(&self, outputindex: u32, dimensionsizes: &[u32]) -> windows_core::Result<IMLOperatorTensor> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetOutputTensor)(windows_core::Interface::as_raw(self), outputindex, dimensionsizes.len().try_into().unwrap(), core::mem::transmute(dimensionsizes.as_ptr()), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetOutputTensor2(&self, outputindex: u32) -> windows_core::Result<IMLOperatorTensor> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetOutputTensor2)(windows_core::Interface::as_raw(self), outputindex, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn AllocateTemporaryData(&self, size: usize) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AllocateTemporaryData)(windows_core::Interface::as_raw(self), size, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetExecutionInterface(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetExecutionInterface)(windows_core::Interface::as_raw(self), &mut result__);
            windows_core::Type::from_abi(result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMLOperatorKernelContext_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetInputTensor: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetOutputTensor: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *const u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetOutputTensor2: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AllocateTemporaryData: unsafe extern "system" fn(*mut core::ffi::c_void, usize, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetExecutionInterface: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void),
}
pub trait IMLOperatorKernelContext_Impl: windows_core::IUnknownImpl {
    fn GetInputTensor(&self, inputindex: u32) -> windows_core::Result<IMLOperatorTensor>;
    fn GetOutputTensor(&self, outputindex: u32, dimensioncount: u32, dimensionsizes: *const u32) -> windows_core::Result<IMLOperatorTensor>;
    fn GetOutputTensor2(&self, outputindex: u32) -> windows_core::Result<IMLOperatorTensor>;
    fn AllocateTemporaryData(&self, size: usize) -> windows_core::Result<windows_core::IUnknown>;
    fn GetExecutionInterface(&self, executionobject: windows_core::OutRef<windows_core::IUnknown>);
}
impl IMLOperatorKernelContext_Vtbl {
    pub const fn new<Identity: IMLOperatorKernelContext_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetInputTensor<Identity: IMLOperatorKernelContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, inputindex: u32, tensor: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMLOperatorKernelContext_Impl::GetInputTensor(this, core::mem::transmute_copy(&inputindex)) {
                    Ok(ok__) => {
                        tensor.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetOutputTensor<Identity: IMLOperatorKernelContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, outputindex: u32, dimensioncount: u32, dimensionsizes: *const u32, tensor: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMLOperatorKernelContext_Impl::GetOutputTensor(this, core::mem::transmute_copy(&outputindex), core::mem::transmute_copy(&dimensioncount), core::mem::transmute_copy(&dimensionsizes)) {
                    Ok(ok__) => {
                        tensor.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetOutputTensor2<Identity: IMLOperatorKernelContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, outputindex: u32, tensor: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMLOperatorKernelContext_Impl::GetOutputTensor2(this, core::mem::transmute_copy(&outputindex)) {
                    Ok(ok__) => {
                        tensor.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn AllocateTemporaryData<Identity: IMLOperatorKernelContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, size: usize, data: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMLOperatorKernelContext_Impl::AllocateTemporaryData(this, core::mem::transmute_copy(&size)) {
                    Ok(ok__) => {
                        data.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetExecutionInterface<Identity: IMLOperatorKernelContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, executionobject: *mut *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMLOperatorKernelContext_Impl::GetExecutionInterface(this, core::mem::transmute_copy(&executionobject))
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetInputTensor: GetInputTensor::<Identity, OFFSET>,
            GetOutputTensor: GetOutputTensor::<Identity, OFFSET>,
            GetOutputTensor2: GetOutputTensor2::<Identity, OFFSET>,
            AllocateTemporaryData: AllocateTemporaryData::<Identity, OFFSET>,
            GetExecutionInterface: GetExecutionInterface::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMLOperatorKernelContext as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMLOperatorKernelContext {}
windows_core::imp::define_interface!(IMLOperatorKernelCreationContext, IMLOperatorKernelCreationContext_Vtbl, 0x5459b53d_a0fc_4665_addd_70171ef7e631);
impl core::ops::Deref for IMLOperatorKernelCreationContext {
    type Target = IMLOperatorAttributes;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IMLOperatorKernelCreationContext, windows_core::IUnknown, IMLOperatorAttributes);
impl IMLOperatorKernelCreationContext {
    pub unsafe fn GetInputCount(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).GetInputCount)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetOutputCount(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).GetOutputCount)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn IsInputValid(&self, inputindex: u32) -> bool {
        unsafe { (windows_core::Interface::vtable(self).IsInputValid)(windows_core::Interface::as_raw(self), inputindex) }
    }
    pub unsafe fn IsOutputValid(&self, outputindex: u32) -> bool {
        unsafe { (windows_core::Interface::vtable(self).IsOutputValid)(windows_core::Interface::as_raw(self), outputindex) }
    }
    pub unsafe fn GetInputEdgeDescription(&self, inputindex: u32) -> windows_core::Result<MLOperatorEdgeDescription> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetInputEdgeDescription)(windows_core::Interface::as_raw(self), inputindex, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetOutputEdgeDescription(&self, outputindex: u32) -> windows_core::Result<MLOperatorEdgeDescription> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetOutputEdgeDescription)(windows_core::Interface::as_raw(self), outputindex, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn HasTensorShapeDescription(&self) -> bool {
        unsafe { (windows_core::Interface::vtable(self).HasTensorShapeDescription)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetTensorShapeDescription(&self) -> windows_core::Result<IMLOperatorTensorShapeDescription> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetTensorShapeDescription)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetExecutionInterface(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetExecutionInterface)(windows_core::Interface::as_raw(self), &mut result__);
            windows_core::Type::from_abi(result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMLOperatorKernelCreationContext_Vtbl {
    pub base__: IMLOperatorAttributes_Vtbl,
    pub GetInputCount: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub GetOutputCount: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub IsInputValid: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> bool,
    pub IsOutputValid: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> bool,
    pub GetInputEdgeDescription: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut MLOperatorEdgeDescription) -> windows_core::HRESULT,
    pub GetOutputEdgeDescription: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut MLOperatorEdgeDescription) -> windows_core::HRESULT,
    pub HasTensorShapeDescription: unsafe extern "system" fn(*mut core::ffi::c_void) -> bool,
    pub GetTensorShapeDescription: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetExecutionInterface: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void),
}
pub trait IMLOperatorKernelCreationContext_Impl: IMLOperatorAttributes_Impl {
    fn GetInputCount(&self) -> u32;
    fn GetOutputCount(&self) -> u32;
    fn IsInputValid(&self, inputindex: u32) -> bool;
    fn IsOutputValid(&self, outputindex: u32) -> bool;
    fn GetInputEdgeDescription(&self, inputindex: u32) -> windows_core::Result<MLOperatorEdgeDescription>;
    fn GetOutputEdgeDescription(&self, outputindex: u32) -> windows_core::Result<MLOperatorEdgeDescription>;
    fn HasTensorShapeDescription(&self) -> bool;
    fn GetTensorShapeDescription(&self) -> windows_core::Result<IMLOperatorTensorShapeDescription>;
    fn GetExecutionInterface(&self, executionobject: windows_core::OutRef<windows_core::IUnknown>);
}
impl IMLOperatorKernelCreationContext_Vtbl {
    pub const fn new<Identity: IMLOperatorKernelCreationContext_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetInputCount<Identity: IMLOperatorKernelCreationContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMLOperatorKernelCreationContext_Impl::GetInputCount(this)
            }
        }
        unsafe extern "system" fn GetOutputCount<Identity: IMLOperatorKernelCreationContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMLOperatorKernelCreationContext_Impl::GetOutputCount(this)
            }
        }
        unsafe extern "system" fn IsInputValid<Identity: IMLOperatorKernelCreationContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, inputindex: u32) -> bool {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMLOperatorKernelCreationContext_Impl::IsInputValid(this, core::mem::transmute_copy(&inputindex))
            }
        }
        unsafe extern "system" fn IsOutputValid<Identity: IMLOperatorKernelCreationContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, outputindex: u32) -> bool {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMLOperatorKernelCreationContext_Impl::IsOutputValid(this, core::mem::transmute_copy(&outputindex))
            }
        }
        unsafe extern "system" fn GetInputEdgeDescription<Identity: IMLOperatorKernelCreationContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, inputindex: u32, edgedescription: *mut MLOperatorEdgeDescription) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMLOperatorKernelCreationContext_Impl::GetInputEdgeDescription(this, core::mem::transmute_copy(&inputindex)) {
                    Ok(ok__) => {
                        edgedescription.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetOutputEdgeDescription<Identity: IMLOperatorKernelCreationContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, outputindex: u32, edgedescription: *mut MLOperatorEdgeDescription) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMLOperatorKernelCreationContext_Impl::GetOutputEdgeDescription(this, core::mem::transmute_copy(&outputindex)) {
                    Ok(ok__) => {
                        edgedescription.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn HasTensorShapeDescription<Identity: IMLOperatorKernelCreationContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> bool {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMLOperatorKernelCreationContext_Impl::HasTensorShapeDescription(this)
            }
        }
        unsafe extern "system" fn GetTensorShapeDescription<Identity: IMLOperatorKernelCreationContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, shapedescription: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMLOperatorKernelCreationContext_Impl::GetTensorShapeDescription(this) {
                    Ok(ok__) => {
                        shapedescription.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetExecutionInterface<Identity: IMLOperatorKernelCreationContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, executionobject: *mut *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMLOperatorKernelCreationContext_Impl::GetExecutionInterface(this, core::mem::transmute_copy(&executionobject))
            }
        }
        Self {
            base__: IMLOperatorAttributes_Vtbl::new::<Identity, OFFSET>(),
            GetInputCount: GetInputCount::<Identity, OFFSET>,
            GetOutputCount: GetOutputCount::<Identity, OFFSET>,
            IsInputValid: IsInputValid::<Identity, OFFSET>,
            IsOutputValid: IsOutputValid::<Identity, OFFSET>,
            GetInputEdgeDescription: GetInputEdgeDescription::<Identity, OFFSET>,
            GetOutputEdgeDescription: GetOutputEdgeDescription::<Identity, OFFSET>,
            HasTensorShapeDescription: HasTensorShapeDescription::<Identity, OFFSET>,
            GetTensorShapeDescription: GetTensorShapeDescription::<Identity, OFFSET>,
            GetExecutionInterface: GetExecutionInterface::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMLOperatorKernelCreationContext as windows_core::Interface>::IID || iid == &<IMLOperatorAttributes as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMLOperatorKernelCreationContext {}
windows_core::imp::define_interface!(IMLOperatorKernelFactory, IMLOperatorKernelFactory_Vtbl, 0xef15ad6f_0dc9_4908_ab35_a575a30dfbf8);
windows_core::imp::interface_hierarchy!(IMLOperatorKernelFactory, windows_core::IUnknown);
impl IMLOperatorKernelFactory {
    pub unsafe fn CreateKernel<P0>(&self, context: P0) -> windows_core::Result<IMLOperatorKernel>
    where
        P0: windows_core::Param<IMLOperatorKernelCreationContext>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateKernel)(windows_core::Interface::as_raw(self), context.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMLOperatorKernelFactory_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub CreateKernel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IMLOperatorKernelFactory_Impl: windows_core::IUnknownImpl {
    fn CreateKernel(&self, context: windows_core::Ref<IMLOperatorKernelCreationContext>) -> windows_core::Result<IMLOperatorKernel>;
}
impl IMLOperatorKernelFactory_Vtbl {
    pub const fn new<Identity: IMLOperatorKernelFactory_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateKernel<Identity: IMLOperatorKernelFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, context: *mut core::ffi::c_void, kernel: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMLOperatorKernelFactory_Impl::CreateKernel(this, core::mem::transmute_copy(&context)) {
                    Ok(ok__) => {
                        kernel.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), CreateKernel: CreateKernel::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMLOperatorKernelFactory as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMLOperatorKernelFactory {}
windows_core::imp::define_interface!(IMLOperatorRegistry, IMLOperatorRegistry_Vtbl, 0x2af9dd2d_b516_4672_9ab5_530c208493ad);
windows_core::imp::interface_hierarchy!(IMLOperatorRegistry, windows_core::IUnknown);
impl IMLOperatorRegistry {
    pub unsafe fn RegisterOperatorSetSchema<P4, P5>(&self, operatorsetid: *const MLOperatorSetId, baselineversion: i32, schema: Option<&[*const MLOperatorSchemaDescription]>, typeinferrer: P4, shapeinferrer: P5) -> windows_core::Result<()>
    where
        P4: windows_core::Param<IMLOperatorTypeInferrer>,
        P5: windows_core::Param<IMLOperatorShapeInferrer>,
    {
        unsafe { (windows_core::Interface::vtable(self).RegisterOperatorSetSchema)(windows_core::Interface::as_raw(self), operatorsetid, baselineversion, core::mem::transmute(schema.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), schema.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), typeinferrer.param().abi(), shapeinferrer.param().abi()).ok() }
    }
    pub unsafe fn RegisterOperatorKernel<P1, P2>(&self, operatorkernel: *const MLOperatorKernelDescription, operatorkernelfactory: P1, shapeinferrer: P2) -> windows_core::Result<()>
    where
        P1: windows_core::Param<IMLOperatorKernelFactory>,
        P2: windows_core::Param<IMLOperatorShapeInferrer>,
    {
        unsafe { (windows_core::Interface::vtable(self).RegisterOperatorKernel)(windows_core::Interface::as_raw(self), operatorkernel, operatorkernelfactory.param().abi(), shapeinferrer.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMLOperatorRegistry_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub RegisterOperatorSetSchema: unsafe extern "system" fn(*mut core::ffi::c_void, *const MLOperatorSetId, i32, *const *const MLOperatorSchemaDescription, u32, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RegisterOperatorKernel: unsafe extern "system" fn(*mut core::ffi::c_void, *const MLOperatorKernelDescription, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IMLOperatorRegistry_Impl: windows_core::IUnknownImpl {
    fn RegisterOperatorSetSchema(&self, operatorsetid: *const MLOperatorSetId, baselineversion: i32, schema: *const *const MLOperatorSchemaDescription, schemacount: u32, typeinferrer: windows_core::Ref<IMLOperatorTypeInferrer>, shapeinferrer: windows_core::Ref<IMLOperatorShapeInferrer>) -> windows_core::Result<()>;
    fn RegisterOperatorKernel(&self, operatorkernel: *const MLOperatorKernelDescription, operatorkernelfactory: windows_core::Ref<IMLOperatorKernelFactory>, shapeinferrer: windows_core::Ref<IMLOperatorShapeInferrer>) -> windows_core::Result<()>;
}
impl IMLOperatorRegistry_Vtbl {
    pub const fn new<Identity: IMLOperatorRegistry_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn RegisterOperatorSetSchema<Identity: IMLOperatorRegistry_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, operatorsetid: *const MLOperatorSetId, baselineversion: i32, schema: *const *const MLOperatorSchemaDescription, schemacount: u32, typeinferrer: *mut core::ffi::c_void, shapeinferrer: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMLOperatorRegistry_Impl::RegisterOperatorSetSchema(this, core::mem::transmute_copy(&operatorsetid), core::mem::transmute_copy(&baselineversion), core::mem::transmute_copy(&schema), core::mem::transmute_copy(&schemacount), core::mem::transmute_copy(&typeinferrer), core::mem::transmute_copy(&shapeinferrer)).into()
            }
        }
        unsafe extern "system" fn RegisterOperatorKernel<Identity: IMLOperatorRegistry_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, operatorkernel: *const MLOperatorKernelDescription, operatorkernelfactory: *mut core::ffi::c_void, shapeinferrer: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMLOperatorRegistry_Impl::RegisterOperatorKernel(this, core::mem::transmute_copy(&operatorkernel), core::mem::transmute_copy(&operatorkernelfactory), core::mem::transmute_copy(&shapeinferrer)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            RegisterOperatorSetSchema: RegisterOperatorSetSchema::<Identity, OFFSET>,
            RegisterOperatorKernel: RegisterOperatorKernel::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMLOperatorRegistry as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMLOperatorRegistry {}
windows_core::imp::define_interface!(IMLOperatorShapeInferenceContext, IMLOperatorShapeInferenceContext_Vtbl, 0x105b6b29_5408_4a68_9959_09b5955a3492);
impl core::ops::Deref for IMLOperatorShapeInferenceContext {
    type Target = IMLOperatorAttributes;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IMLOperatorShapeInferenceContext, windows_core::IUnknown, IMLOperatorAttributes);
impl IMLOperatorShapeInferenceContext {
    pub unsafe fn GetInputCount(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).GetInputCount)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetOutputCount(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).GetOutputCount)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn IsInputValid(&self, inputindex: u32) -> bool {
        unsafe { (windows_core::Interface::vtable(self).IsInputValid)(windows_core::Interface::as_raw(self), inputindex) }
    }
    pub unsafe fn IsOutputValid(&self, outputindex: u32) -> bool {
        unsafe { (windows_core::Interface::vtable(self).IsOutputValid)(windows_core::Interface::as_raw(self), outputindex) }
    }
    pub unsafe fn GetInputEdgeDescription(&self, inputindex: u32) -> windows_core::Result<MLOperatorEdgeDescription> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetInputEdgeDescription)(windows_core::Interface::as_raw(self), inputindex, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetInputTensorDimensionCount(&self, inputindex: u32) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetInputTensorDimensionCount)(windows_core::Interface::as_raw(self), inputindex, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetInputTensorShape(&self, inputindex: u32, dimensions: &mut [u32]) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetInputTensorShape)(windows_core::Interface::as_raw(self), inputindex, dimensions.len().try_into().unwrap(), core::mem::transmute(dimensions.as_ptr())).ok() }
    }
    pub unsafe fn SetOutputTensorShape(&self, outputindex: u32, dimensioncount: u32, dimensions: *const u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetOutputTensorShape)(windows_core::Interface::as_raw(self), outputindex, dimensioncount, dimensions).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMLOperatorShapeInferenceContext_Vtbl {
    pub base__: IMLOperatorAttributes_Vtbl,
    pub GetInputCount: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub GetOutputCount: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub IsInputValid: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> bool,
    pub IsOutputValid: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> bool,
    pub GetInputEdgeDescription: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut MLOperatorEdgeDescription) -> windows_core::HRESULT,
    pub GetInputTensorDimensionCount: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u32) -> windows_core::HRESULT,
    pub GetInputTensorShape: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut u32) -> windows_core::HRESULT,
    pub SetOutputTensorShape: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *const u32) -> windows_core::HRESULT,
}
pub trait IMLOperatorShapeInferenceContext_Impl: IMLOperatorAttributes_Impl {
    fn GetInputCount(&self) -> u32;
    fn GetOutputCount(&self) -> u32;
    fn IsInputValid(&self, inputindex: u32) -> bool;
    fn IsOutputValid(&self, outputindex: u32) -> bool;
    fn GetInputEdgeDescription(&self, inputindex: u32) -> windows_core::Result<MLOperatorEdgeDescription>;
    fn GetInputTensorDimensionCount(&self, inputindex: u32) -> windows_core::Result<u32>;
    fn GetInputTensorShape(&self, inputindex: u32, dimensioncount: u32, dimensions: *mut u32) -> windows_core::Result<()>;
    fn SetOutputTensorShape(&self, outputindex: u32, dimensioncount: u32, dimensions: *const u32) -> windows_core::Result<()>;
}
impl IMLOperatorShapeInferenceContext_Vtbl {
    pub const fn new<Identity: IMLOperatorShapeInferenceContext_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetInputCount<Identity: IMLOperatorShapeInferenceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMLOperatorShapeInferenceContext_Impl::GetInputCount(this)
            }
        }
        unsafe extern "system" fn GetOutputCount<Identity: IMLOperatorShapeInferenceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMLOperatorShapeInferenceContext_Impl::GetOutputCount(this)
            }
        }
        unsafe extern "system" fn IsInputValid<Identity: IMLOperatorShapeInferenceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, inputindex: u32) -> bool {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMLOperatorShapeInferenceContext_Impl::IsInputValid(this, core::mem::transmute_copy(&inputindex))
            }
        }
        unsafe extern "system" fn IsOutputValid<Identity: IMLOperatorShapeInferenceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, outputindex: u32) -> bool {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMLOperatorShapeInferenceContext_Impl::IsOutputValid(this, core::mem::transmute_copy(&outputindex))
            }
        }
        unsafe extern "system" fn GetInputEdgeDescription<Identity: IMLOperatorShapeInferenceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, inputindex: u32, edgedescription: *mut MLOperatorEdgeDescription) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMLOperatorShapeInferenceContext_Impl::GetInputEdgeDescription(this, core::mem::transmute_copy(&inputindex)) {
                    Ok(ok__) => {
                        edgedescription.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetInputTensorDimensionCount<Identity: IMLOperatorShapeInferenceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, inputindex: u32, dimensioncount: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMLOperatorShapeInferenceContext_Impl::GetInputTensorDimensionCount(this, core::mem::transmute_copy(&inputindex)) {
                    Ok(ok__) => {
                        dimensioncount.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetInputTensorShape<Identity: IMLOperatorShapeInferenceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, inputindex: u32, dimensioncount: u32, dimensions: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMLOperatorShapeInferenceContext_Impl::GetInputTensorShape(this, core::mem::transmute_copy(&inputindex), core::mem::transmute_copy(&dimensioncount), core::mem::transmute_copy(&dimensions)).into()
            }
        }
        unsafe extern "system" fn SetOutputTensorShape<Identity: IMLOperatorShapeInferenceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, outputindex: u32, dimensioncount: u32, dimensions: *const u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMLOperatorShapeInferenceContext_Impl::SetOutputTensorShape(this, core::mem::transmute_copy(&outputindex), core::mem::transmute_copy(&dimensioncount), core::mem::transmute_copy(&dimensions)).into()
            }
        }
        Self {
            base__: IMLOperatorAttributes_Vtbl::new::<Identity, OFFSET>(),
            GetInputCount: GetInputCount::<Identity, OFFSET>,
            GetOutputCount: GetOutputCount::<Identity, OFFSET>,
            IsInputValid: IsInputValid::<Identity, OFFSET>,
            IsOutputValid: IsOutputValid::<Identity, OFFSET>,
            GetInputEdgeDescription: GetInputEdgeDescription::<Identity, OFFSET>,
            GetInputTensorDimensionCount: GetInputTensorDimensionCount::<Identity, OFFSET>,
            GetInputTensorShape: GetInputTensorShape::<Identity, OFFSET>,
            SetOutputTensorShape: SetOutputTensorShape::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMLOperatorShapeInferenceContext as windows_core::Interface>::IID || iid == &<IMLOperatorAttributes as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMLOperatorShapeInferenceContext {}
windows_core::imp::define_interface!(IMLOperatorShapeInferrer, IMLOperatorShapeInferrer_Vtbl, 0x540be5be_a6c9_40ee_83f6_d2b8b40a7798);
windows_core::imp::interface_hierarchy!(IMLOperatorShapeInferrer, windows_core::IUnknown);
impl IMLOperatorShapeInferrer {
    pub unsafe fn InferOutputShapes<P0>(&self, context: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMLOperatorShapeInferenceContext>,
    {
        unsafe { (windows_core::Interface::vtable(self).InferOutputShapes)(windows_core::Interface::as_raw(self), context.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMLOperatorShapeInferrer_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub InferOutputShapes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IMLOperatorShapeInferrer_Impl: windows_core::IUnknownImpl {
    fn InferOutputShapes(&self, context: windows_core::Ref<IMLOperatorShapeInferenceContext>) -> windows_core::Result<()>;
}
impl IMLOperatorShapeInferrer_Vtbl {
    pub const fn new<Identity: IMLOperatorShapeInferrer_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn InferOutputShapes<Identity: IMLOperatorShapeInferrer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, context: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMLOperatorShapeInferrer_Impl::InferOutputShapes(this, core::mem::transmute_copy(&context)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), InferOutputShapes: InferOutputShapes::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMLOperatorShapeInferrer as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMLOperatorShapeInferrer {}
windows_core::imp::define_interface!(IMLOperatorTensor, IMLOperatorTensor_Vtbl, 0x7fe41f41_f430_440e_aece_54416dc8b9db);
windows_core::imp::interface_hierarchy!(IMLOperatorTensor, windows_core::IUnknown);
impl IMLOperatorTensor {
    pub unsafe fn GetDimensionCount(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).GetDimensionCount)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetShape(&self, dimensions: &mut [u32]) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetShape)(windows_core::Interface::as_raw(self), dimensions.len().try_into().unwrap(), core::mem::transmute(dimensions.as_ptr())).ok() }
    }
    pub unsafe fn GetTensorDataType(&self) -> MLOperatorTensorDataType {
        unsafe { (windows_core::Interface::vtable(self).GetTensorDataType)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn IsCpuData(&self) -> bool {
        unsafe { (windows_core::Interface::vtable(self).IsCpuData)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn IsDataInterface(&self) -> bool {
        unsafe { (windows_core::Interface::vtable(self).IsDataInterface)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetData(&self) -> *mut core::ffi::c_void {
        unsafe { (windows_core::Interface::vtable(self).GetData)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetDataInterface(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDataInterface)(windows_core::Interface::as_raw(self), &mut result__);
            windows_core::Type::from_abi(result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMLOperatorTensor_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetDimensionCount: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub GetShape: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u32) -> windows_core::HRESULT,
    pub GetTensorDataType: unsafe extern "system" fn(*mut core::ffi::c_void) -> MLOperatorTensorDataType,
    pub IsCpuData: unsafe extern "system" fn(*mut core::ffi::c_void) -> bool,
    pub IsDataInterface: unsafe extern "system" fn(*mut core::ffi::c_void) -> bool,
    pub GetData: unsafe extern "system" fn(*mut core::ffi::c_void) -> *mut core::ffi::c_void,
    pub GetDataInterface: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void),
}
pub trait IMLOperatorTensor_Impl: windows_core::IUnknownImpl {
    fn GetDimensionCount(&self) -> u32;
    fn GetShape(&self, dimensioncount: u32, dimensions: *mut u32) -> windows_core::Result<()>;
    fn GetTensorDataType(&self) -> MLOperatorTensorDataType;
    fn IsCpuData(&self) -> bool;
    fn IsDataInterface(&self) -> bool;
    fn GetData(&self) -> *mut core::ffi::c_void;
    fn GetDataInterface(&self, datainterface: windows_core::OutRef<windows_core::IUnknown>);
}
impl IMLOperatorTensor_Vtbl {
    pub const fn new<Identity: IMLOperatorTensor_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetDimensionCount<Identity: IMLOperatorTensor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMLOperatorTensor_Impl::GetDimensionCount(this)
            }
        }
        unsafe extern "system" fn GetShape<Identity: IMLOperatorTensor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dimensioncount: u32, dimensions: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMLOperatorTensor_Impl::GetShape(this, core::mem::transmute_copy(&dimensioncount), core::mem::transmute_copy(&dimensions)).into()
            }
        }
        unsafe extern "system" fn GetTensorDataType<Identity: IMLOperatorTensor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> MLOperatorTensorDataType {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMLOperatorTensor_Impl::GetTensorDataType(this)
            }
        }
        unsafe extern "system" fn IsCpuData<Identity: IMLOperatorTensor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> bool {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMLOperatorTensor_Impl::IsCpuData(this)
            }
        }
        unsafe extern "system" fn IsDataInterface<Identity: IMLOperatorTensor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> bool {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMLOperatorTensor_Impl::IsDataInterface(this)
            }
        }
        unsafe extern "system" fn GetData<Identity: IMLOperatorTensor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMLOperatorTensor_Impl::GetData(this)
            }
        }
        unsafe extern "system" fn GetDataInterface<Identity: IMLOperatorTensor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, datainterface: *mut *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMLOperatorTensor_Impl::GetDataInterface(this, core::mem::transmute_copy(&datainterface))
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetDimensionCount: GetDimensionCount::<Identity, OFFSET>,
            GetShape: GetShape::<Identity, OFFSET>,
            GetTensorDataType: GetTensorDataType::<Identity, OFFSET>,
            IsCpuData: IsCpuData::<Identity, OFFSET>,
            IsDataInterface: IsDataInterface::<Identity, OFFSET>,
            GetData: GetData::<Identity, OFFSET>,
            GetDataInterface: GetDataInterface::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMLOperatorTensor as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMLOperatorTensor {}
windows_core::imp::define_interface!(IMLOperatorTensorShapeDescription, IMLOperatorTensorShapeDescription_Vtbl, 0xf20e8cbe_3b28_4248_be95_f96fbc6e4643);
windows_core::imp::interface_hierarchy!(IMLOperatorTensorShapeDescription, windows_core::IUnknown);
impl IMLOperatorTensorShapeDescription {
    pub unsafe fn GetInputTensorDimensionCount(&self, inputindex: u32) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetInputTensorDimensionCount)(windows_core::Interface::as_raw(self), inputindex, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetInputTensorShape(&self, inputindex: u32, dimensions: &mut [u32]) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetInputTensorShape)(windows_core::Interface::as_raw(self), inputindex, dimensions.len().try_into().unwrap(), core::mem::transmute(dimensions.as_ptr())).ok() }
    }
    pub unsafe fn HasOutputShapeDescription(&self) -> bool {
        unsafe { (windows_core::Interface::vtable(self).HasOutputShapeDescription)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetOutputTensorDimensionCount(&self, outputindex: u32) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetOutputTensorDimensionCount)(windows_core::Interface::as_raw(self), outputindex, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetOutputTensorShape(&self, outputindex: u32, dimensions: &mut [u32]) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetOutputTensorShape)(windows_core::Interface::as_raw(self), outputindex, dimensions.len().try_into().unwrap(), core::mem::transmute(dimensions.as_ptr())).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMLOperatorTensorShapeDescription_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetInputTensorDimensionCount: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u32) -> windows_core::HRESULT,
    pub GetInputTensorShape: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut u32) -> windows_core::HRESULT,
    pub HasOutputShapeDescription: unsafe extern "system" fn(*mut core::ffi::c_void) -> bool,
    pub GetOutputTensorDimensionCount: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u32) -> windows_core::HRESULT,
    pub GetOutputTensorShape: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut u32) -> windows_core::HRESULT,
}
pub trait IMLOperatorTensorShapeDescription_Impl: windows_core::IUnknownImpl {
    fn GetInputTensorDimensionCount(&self, inputindex: u32) -> windows_core::Result<u32>;
    fn GetInputTensorShape(&self, inputindex: u32, dimensioncount: u32, dimensions: *mut u32) -> windows_core::Result<()>;
    fn HasOutputShapeDescription(&self) -> bool;
    fn GetOutputTensorDimensionCount(&self, outputindex: u32) -> windows_core::Result<u32>;
    fn GetOutputTensorShape(&self, outputindex: u32, dimensioncount: u32, dimensions: *mut u32) -> windows_core::Result<()>;
}
impl IMLOperatorTensorShapeDescription_Vtbl {
    pub const fn new<Identity: IMLOperatorTensorShapeDescription_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetInputTensorDimensionCount<Identity: IMLOperatorTensorShapeDescription_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, inputindex: u32, dimensioncount: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMLOperatorTensorShapeDescription_Impl::GetInputTensorDimensionCount(this, core::mem::transmute_copy(&inputindex)) {
                    Ok(ok__) => {
                        dimensioncount.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetInputTensorShape<Identity: IMLOperatorTensorShapeDescription_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, inputindex: u32, dimensioncount: u32, dimensions: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMLOperatorTensorShapeDescription_Impl::GetInputTensorShape(this, core::mem::transmute_copy(&inputindex), core::mem::transmute_copy(&dimensioncount), core::mem::transmute_copy(&dimensions)).into()
            }
        }
        unsafe extern "system" fn HasOutputShapeDescription<Identity: IMLOperatorTensorShapeDescription_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> bool {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMLOperatorTensorShapeDescription_Impl::HasOutputShapeDescription(this)
            }
        }
        unsafe extern "system" fn GetOutputTensorDimensionCount<Identity: IMLOperatorTensorShapeDescription_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, outputindex: u32, dimensioncount: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMLOperatorTensorShapeDescription_Impl::GetOutputTensorDimensionCount(this, core::mem::transmute_copy(&outputindex)) {
                    Ok(ok__) => {
                        dimensioncount.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetOutputTensorShape<Identity: IMLOperatorTensorShapeDescription_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, outputindex: u32, dimensioncount: u32, dimensions: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMLOperatorTensorShapeDescription_Impl::GetOutputTensorShape(this, core::mem::transmute_copy(&outputindex), core::mem::transmute_copy(&dimensioncount), core::mem::transmute_copy(&dimensions)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetInputTensorDimensionCount: GetInputTensorDimensionCount::<Identity, OFFSET>,
            GetInputTensorShape: GetInputTensorShape::<Identity, OFFSET>,
            HasOutputShapeDescription: HasOutputShapeDescription::<Identity, OFFSET>,
            GetOutputTensorDimensionCount: GetOutputTensorDimensionCount::<Identity, OFFSET>,
            GetOutputTensorShape: GetOutputTensorShape::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMLOperatorTensorShapeDescription as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMLOperatorTensorShapeDescription {}
windows_core::imp::define_interface!(IMLOperatorTypeInferenceContext, IMLOperatorTypeInferenceContext_Vtbl, 0xec893bb1_f938_427b_8488_c8dcf775f138);
impl core::ops::Deref for IMLOperatorTypeInferenceContext {
    type Target = IMLOperatorAttributes;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IMLOperatorTypeInferenceContext, windows_core::IUnknown, IMLOperatorAttributes);
impl IMLOperatorTypeInferenceContext {
    pub unsafe fn GetInputCount(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).GetInputCount)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetOutputCount(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).GetOutputCount)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn IsInputValid(&self, inputindex: u32) -> bool {
        unsafe { (windows_core::Interface::vtable(self).IsInputValid)(windows_core::Interface::as_raw(self), inputindex) }
    }
    pub unsafe fn IsOutputValid(&self, outputindex: u32) -> bool {
        unsafe { (windows_core::Interface::vtable(self).IsOutputValid)(windows_core::Interface::as_raw(self), outputindex) }
    }
    pub unsafe fn GetInputEdgeDescription(&self, inputindex: u32) -> windows_core::Result<MLOperatorEdgeDescription> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetInputEdgeDescription)(windows_core::Interface::as_raw(self), inputindex, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetOutputEdgeDescription(&self, outputindex: u32, edgedescription: *const MLOperatorEdgeDescription) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetOutputEdgeDescription)(windows_core::Interface::as_raw(self), outputindex, edgedescription).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMLOperatorTypeInferenceContext_Vtbl {
    pub base__: IMLOperatorAttributes_Vtbl,
    pub GetInputCount: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub GetOutputCount: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub IsInputValid: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> bool,
    pub IsOutputValid: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> bool,
    pub GetInputEdgeDescription: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut MLOperatorEdgeDescription) -> windows_core::HRESULT,
    pub SetOutputEdgeDescription: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const MLOperatorEdgeDescription) -> windows_core::HRESULT,
}
pub trait IMLOperatorTypeInferenceContext_Impl: IMLOperatorAttributes_Impl {
    fn GetInputCount(&self) -> u32;
    fn GetOutputCount(&self) -> u32;
    fn IsInputValid(&self, inputindex: u32) -> bool;
    fn IsOutputValid(&self, outputindex: u32) -> bool;
    fn GetInputEdgeDescription(&self, inputindex: u32) -> windows_core::Result<MLOperatorEdgeDescription>;
    fn SetOutputEdgeDescription(&self, outputindex: u32, edgedescription: *const MLOperatorEdgeDescription) -> windows_core::Result<()>;
}
impl IMLOperatorTypeInferenceContext_Vtbl {
    pub const fn new<Identity: IMLOperatorTypeInferenceContext_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetInputCount<Identity: IMLOperatorTypeInferenceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMLOperatorTypeInferenceContext_Impl::GetInputCount(this)
            }
        }
        unsafe extern "system" fn GetOutputCount<Identity: IMLOperatorTypeInferenceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMLOperatorTypeInferenceContext_Impl::GetOutputCount(this)
            }
        }
        unsafe extern "system" fn IsInputValid<Identity: IMLOperatorTypeInferenceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, inputindex: u32) -> bool {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMLOperatorTypeInferenceContext_Impl::IsInputValid(this, core::mem::transmute_copy(&inputindex))
            }
        }
        unsafe extern "system" fn IsOutputValid<Identity: IMLOperatorTypeInferenceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, outputindex: u32) -> bool {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMLOperatorTypeInferenceContext_Impl::IsOutputValid(this, core::mem::transmute_copy(&outputindex))
            }
        }
        unsafe extern "system" fn GetInputEdgeDescription<Identity: IMLOperatorTypeInferenceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, inputindex: u32, edgedescription: *mut MLOperatorEdgeDescription) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMLOperatorTypeInferenceContext_Impl::GetInputEdgeDescription(this, core::mem::transmute_copy(&inputindex)) {
                    Ok(ok__) => {
                        edgedescription.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetOutputEdgeDescription<Identity: IMLOperatorTypeInferenceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, outputindex: u32, edgedescription: *const MLOperatorEdgeDescription) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMLOperatorTypeInferenceContext_Impl::SetOutputEdgeDescription(this, core::mem::transmute_copy(&outputindex), core::mem::transmute_copy(&edgedescription)).into()
            }
        }
        Self {
            base__: IMLOperatorAttributes_Vtbl::new::<Identity, OFFSET>(),
            GetInputCount: GetInputCount::<Identity, OFFSET>,
            GetOutputCount: GetOutputCount::<Identity, OFFSET>,
            IsInputValid: IsInputValid::<Identity, OFFSET>,
            IsOutputValid: IsOutputValid::<Identity, OFFSET>,
            GetInputEdgeDescription: GetInputEdgeDescription::<Identity, OFFSET>,
            SetOutputEdgeDescription: SetOutputEdgeDescription::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMLOperatorTypeInferenceContext as windows_core::Interface>::IID || iid == &<IMLOperatorAttributes as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMLOperatorTypeInferenceContext {}
windows_core::imp::define_interface!(IMLOperatorTypeInferrer, IMLOperatorTypeInferrer_Vtbl, 0x781aeb48_9bcb_4797_bf77_8bf455217beb);
windows_core::imp::interface_hierarchy!(IMLOperatorTypeInferrer, windows_core::IUnknown);
impl IMLOperatorTypeInferrer {
    pub unsafe fn InferOutputTypes<P0>(&self, context: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMLOperatorTypeInferenceContext>,
    {
        unsafe { (windows_core::Interface::vtable(self).InferOutputTypes)(windows_core::Interface::as_raw(self), context.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMLOperatorTypeInferrer_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub InferOutputTypes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IMLOperatorTypeInferrer_Impl: windows_core::IUnknownImpl {
    fn InferOutputTypes(&self, context: windows_core::Ref<IMLOperatorTypeInferenceContext>) -> windows_core::Result<()>;
}
impl IMLOperatorTypeInferrer_Vtbl {
    pub const fn new<Identity: IMLOperatorTypeInferrer_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn InferOutputTypes<Identity: IMLOperatorTypeInferrer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, context: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMLOperatorTypeInferrer_Impl::InferOutputTypes(this, core::mem::transmute_copy(&context)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), InferOutputTypes: InferOutputTypes::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMLOperatorTypeInferrer as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMLOperatorTypeInferrer {}
windows_core::imp::define_interface!(IWinMLEvaluationContext, IWinMLEvaluationContext_Vtbl, 0x95848f9e_583d_4054_af12_916387cd8426);
windows_core::imp::interface_hierarchy!(IWinMLEvaluationContext, windows_core::IUnknown);
impl IWinMLEvaluationContext {
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn BindValue(&self, pdescriptor: *const WINML_BINDING_DESC) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).BindValue)(windows_core::Interface::as_raw(self), core::mem::transmute(pdescriptor)).ok() }
    }
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn GetValueByName<P0>(&self, name: P0) -> windows_core::Result<*mut WINML_BINDING_DESC>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetValueByName)(windows_core::Interface::as_raw(self), name.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Clear(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Clear)(windows_core::Interface::as_raw(self)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWinMLEvaluationContext_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub BindValue: unsafe extern "system" fn(*mut core::ffi::c_void, *const WINML_BINDING_DESC) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D12"))]
    BindValue: usize,
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub GetValueByName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut *mut WINML_BINDING_DESC) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D12"))]
    GetValueByName: usize,
    pub Clear: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
pub trait IWinMLEvaluationContext_Impl: windows_core::IUnknownImpl {
    fn BindValue(&self, pdescriptor: *const WINML_BINDING_DESC) -> windows_core::Result<()>;
    fn GetValueByName(&self, name: &windows_core::PCWSTR) -> windows_core::Result<*mut WINML_BINDING_DESC>;
    fn Clear(&self) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl IWinMLEvaluationContext_Vtbl {
    pub const fn new<Identity: IWinMLEvaluationContext_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn BindValue<Identity: IWinMLEvaluationContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdescriptor: *const WINML_BINDING_DESC) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWinMLEvaluationContext_Impl::BindValue(this, core::mem::transmute_copy(&pdescriptor)).into()
            }
        }
        unsafe extern "system" fn GetValueByName<Identity: IWinMLEvaluationContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: windows_core::PCWSTR, pdescriptor: *mut *mut WINML_BINDING_DESC) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWinMLEvaluationContext_Impl::GetValueByName(this, core::mem::transmute(&name)) {
                    Ok(ok__) => {
                        pdescriptor.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Clear<Identity: IWinMLEvaluationContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWinMLEvaluationContext_Impl::Clear(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            BindValue: BindValue::<Identity, OFFSET>,
            GetValueByName: GetValueByName::<Identity, OFFSET>,
            Clear: Clear::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWinMLEvaluationContext as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl windows_core::RuntimeName for IWinMLEvaluationContext {}
windows_core::imp::define_interface!(IWinMLModel, IWinMLModel_Vtbl, 0xe2eeb6a9_f31f_4055_a521_e30b5b33664a);
windows_core::imp::interface_hierarchy!(IWinMLModel, windows_core::IUnknown);
impl IWinMLModel {
    pub unsafe fn GetDescription(&self) -> windows_core::Result<*mut WINML_MODEL_DESC> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDescription)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn EnumerateMetadata(&self, index: u32, pkey: *mut windows_core::PCWSTR, pvalue: *mut windows_core::PCWSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).EnumerateMetadata)(windows_core::Interface::as_raw(self), index, pkey as _, pvalue as _).ok() }
    }
    pub unsafe fn EnumerateModelInputs(&self, index: u32) -> windows_core::Result<*mut WINML_VARIABLE_DESC> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumerateModelInputs)(windows_core::Interface::as_raw(self), index, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn EnumerateModelOutputs(&self, index: u32) -> windows_core::Result<*mut WINML_VARIABLE_DESC> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumerateModelOutputs)(windows_core::Interface::as_raw(self), index, &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWinMLModel_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetDescription: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut WINML_MODEL_DESC) -> windows_core::HRESULT,
    pub EnumerateMetadata: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut windows_core::PCWSTR, *mut windows_core::PCWSTR) -> windows_core::HRESULT,
    pub EnumerateModelInputs: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut WINML_VARIABLE_DESC) -> windows_core::HRESULT,
    pub EnumerateModelOutputs: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut WINML_VARIABLE_DESC) -> windows_core::HRESULT,
}
pub trait IWinMLModel_Impl: windows_core::IUnknownImpl {
    fn GetDescription(&self) -> windows_core::Result<*mut WINML_MODEL_DESC>;
    fn EnumerateMetadata(&self, index: u32, pkey: *mut windows_core::PCWSTR, pvalue: *mut windows_core::PCWSTR) -> windows_core::Result<()>;
    fn EnumerateModelInputs(&self, index: u32) -> windows_core::Result<*mut WINML_VARIABLE_DESC>;
    fn EnumerateModelOutputs(&self, index: u32) -> windows_core::Result<*mut WINML_VARIABLE_DESC>;
}
impl IWinMLModel_Vtbl {
    pub const fn new<Identity: IWinMLModel_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetDescription<Identity: IWinMLModel_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppdescription: *mut *mut WINML_MODEL_DESC) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWinMLModel_Impl::GetDescription(this) {
                    Ok(ok__) => {
                        ppdescription.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn EnumerateMetadata<Identity: IWinMLModel_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, pkey: *mut windows_core::PCWSTR, pvalue: *mut windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWinMLModel_Impl::EnumerateMetadata(this, core::mem::transmute_copy(&index), core::mem::transmute_copy(&pkey), core::mem::transmute_copy(&pvalue)).into()
            }
        }
        unsafe extern "system" fn EnumerateModelInputs<Identity: IWinMLModel_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, ppinputdescriptor: *mut *mut WINML_VARIABLE_DESC) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWinMLModel_Impl::EnumerateModelInputs(this, core::mem::transmute_copy(&index)) {
                    Ok(ok__) => {
                        ppinputdescriptor.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn EnumerateModelOutputs<Identity: IWinMLModel_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, ppoutputdescriptor: *mut *mut WINML_VARIABLE_DESC) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWinMLModel_Impl::EnumerateModelOutputs(this, core::mem::transmute_copy(&index)) {
                    Ok(ok__) => {
                        ppoutputdescriptor.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetDescription: GetDescription::<Identity, OFFSET>,
            EnumerateMetadata: EnumerateMetadata::<Identity, OFFSET>,
            EnumerateModelInputs: EnumerateModelInputs::<Identity, OFFSET>,
            EnumerateModelOutputs: EnumerateModelOutputs::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWinMLModel as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWinMLModel {}
windows_core::imp::define_interface!(IWinMLRuntime, IWinMLRuntime_Vtbl, 0xa0425329_40ae_48d9_bce3_829ef7b8a41a);
windows_core::imp::interface_hierarchy!(IWinMLRuntime, windows_core::IUnknown);
impl IWinMLRuntime {
    pub unsafe fn LoadModel<P0>(&self, path: P0) -> windows_core::Result<IWinMLModel>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).LoadModel)(windows_core::Interface::as_raw(self), path.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn CreateEvaluationContext<P0>(&self, device: P0) -> windows_core::Result<IWinMLEvaluationContext>
    where
        P0: windows_core::Param<super::super::super::Graphics::Direct3D12::ID3D12Device>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateEvaluationContext)(windows_core::Interface::as_raw(self), device.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn EvaluateModel<P0>(&self, pcontext: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IWinMLEvaluationContext>,
    {
        unsafe { (windows_core::Interface::vtable(self).EvaluateModel)(windows_core::Interface::as_raw(self), pcontext.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWinMLRuntime_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub LoadModel: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub CreateEvaluationContext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D12"))]
    CreateEvaluationContext: usize,
    pub EvaluateModel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
pub trait IWinMLRuntime_Impl: windows_core::IUnknownImpl {
    fn LoadModel(&self, path: &windows_core::PCWSTR) -> windows_core::Result<IWinMLModel>;
    fn CreateEvaluationContext(&self, device: windows_core::Ref<super::super::super::Graphics::Direct3D12::ID3D12Device>) -> windows_core::Result<IWinMLEvaluationContext>;
    fn EvaluateModel(&self, pcontext: windows_core::Ref<IWinMLEvaluationContext>) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl IWinMLRuntime_Vtbl {
    pub const fn new<Identity: IWinMLRuntime_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn LoadModel<Identity: IWinMLRuntime_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, path: windows_core::PCWSTR, ppmodel: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWinMLRuntime_Impl::LoadModel(this, core::mem::transmute(&path)) {
                    Ok(ok__) => {
                        ppmodel.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateEvaluationContext<Identity: IWinMLRuntime_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, device: *mut core::ffi::c_void, ppcontext: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWinMLRuntime_Impl::CreateEvaluationContext(this, core::mem::transmute_copy(&device)) {
                    Ok(ok__) => {
                        ppcontext.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn EvaluateModel<Identity: IWinMLRuntime_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcontext: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWinMLRuntime_Impl::EvaluateModel(this, core::mem::transmute_copy(&pcontext)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            LoadModel: LoadModel::<Identity, OFFSET>,
            CreateEvaluationContext: CreateEvaluationContext::<Identity, OFFSET>,
            EvaluateModel: EvaluateModel::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWinMLRuntime as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl windows_core::RuntimeName for IWinMLRuntime {}
windows_core::imp::define_interface!(IWinMLRuntimeFactory, IWinMLRuntimeFactory_Vtbl, 0xa807b84d_4ae5_4bc0_a76a_941aa246bd41);
windows_core::imp::interface_hierarchy!(IWinMLRuntimeFactory, windows_core::IUnknown);
impl IWinMLRuntimeFactory {
    pub unsafe fn CreateRuntime(&self, runtimetype: WINML_RUNTIME_TYPE) -> windows_core::Result<IWinMLRuntime> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateRuntime)(windows_core::Interface::as_raw(self), runtimetype, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWinMLRuntimeFactory_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub CreateRuntime: unsafe extern "system" fn(*mut core::ffi::c_void, WINML_RUNTIME_TYPE, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IWinMLRuntimeFactory_Impl: windows_core::IUnknownImpl {
    fn CreateRuntime(&self, runtimetype: WINML_RUNTIME_TYPE) -> windows_core::Result<IWinMLRuntime>;
}
impl IWinMLRuntimeFactory_Vtbl {
    pub const fn new<Identity: IWinMLRuntimeFactory_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateRuntime<Identity: IWinMLRuntimeFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, runtimetype: WINML_RUNTIME_TYPE, ppruntime: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWinMLRuntimeFactory_Impl::CreateRuntime(this, core::mem::transmute_copy(&runtimetype)) {
                    Ok(ok__) => {
                        ppruntime.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), CreateRuntime: CreateRuntime::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWinMLRuntimeFactory as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWinMLRuntimeFactory {}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MLOperatorAttribute {
    pub name: windows_core::PCSTR,
    pub r#type: MLOperatorAttributeType,
    pub required: u8,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct MLOperatorAttributeNameValue {
    pub name: windows_core::PCSTR,
    pub r#type: MLOperatorAttributeType,
    pub valueCount: u32,
    pub Anonymous: MLOperatorAttributeNameValue_0,
}
impl Default for MLOperatorAttributeNameValue {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union MLOperatorAttributeNameValue_0 {
    pub reserved: *const core::ffi::c_void,
    pub ints: *const i64,
    pub strings: *const *const i8,
    pub floats: *const f32,
}
impl Default for MLOperatorAttributeNameValue_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MLOperatorAttributeType(pub u32);
impl MLOperatorAttributeType {
    pub const Undefined: Self = Self(0u32);
    pub const Float: Self = Self(2u32);
    pub const Int: Self = Self(3u32);
    pub const String: Self = Self(4u32);
    pub const FloatArray: Self = Self(7u32);
    pub const IntArray: Self = Self(8u32);
    pub const StringArray: Self = Self(9u32);
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct MLOperatorEdgeDescription {
    pub edgeType: MLOperatorEdgeType,
    pub Anonymous: MLOperatorEdgeDescription_0,
}
impl Default for MLOperatorEdgeDescription {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union MLOperatorEdgeDescription_0 {
    pub reserved: u64,
    pub tensorDataType: MLOperatorTensorDataType,
}
impl Default for MLOperatorEdgeDescription_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MLOperatorEdgeType(pub u32);
impl MLOperatorEdgeType {
    pub const Undefined: Self = Self(0u32);
    pub const Tensor: Self = Self(1u32);
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MLOperatorEdgeTypeConstraint {
    pub typeLabel: windows_core::PCSTR,
    pub allowedTypes: *const MLOperatorEdgeDescription,
    pub allowedTypeCount: u32,
}
impl Default for MLOperatorEdgeTypeConstraint {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MLOperatorExecutionType(pub u32);
impl MLOperatorExecutionType {
    pub const Undefined: Self = Self(0u32);
    pub const Cpu: Self = Self(1u32);
    pub const D3D12: Self = Self(2u32);
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MLOperatorKernelDescription {
    pub domain: windows_core::PCSTR,
    pub name: windows_core::PCSTR,
    pub minimumOperatorSetVersion: i32,
    pub executionType: MLOperatorExecutionType,
    pub typeConstraints: *const MLOperatorEdgeTypeConstraint,
    pub typeConstraintCount: u32,
    pub defaultAttributes: *const MLOperatorAttributeNameValue,
    pub defaultAttributeCount: u32,
    pub options: MLOperatorKernelOptions,
    pub executionOptions: u32,
}
impl Default for MLOperatorKernelDescription {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MLOperatorKernelOptions(pub u32);
impl MLOperatorKernelOptions {
    pub const None: Self = Self(0u32);
    pub const AllowDynamicInputShapes: Self = Self(1u32);
}
impl MLOperatorKernelOptions {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for MLOperatorKernelOptions {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for MLOperatorKernelOptions {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for MLOperatorKernelOptions {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for MLOperatorKernelOptions {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for MLOperatorKernelOptions {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MLOperatorParameterOptions(pub u32);
impl MLOperatorParameterOptions {
    pub const Single: Self = Self(0u32);
    pub const Optional: Self = Self(1u32);
    pub const Variadic: Self = Self(2u32);
}
impl MLOperatorParameterOptions {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for MLOperatorParameterOptions {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for MLOperatorParameterOptions {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for MLOperatorParameterOptions {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for MLOperatorParameterOptions {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for MLOperatorParameterOptions {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MLOperatorSchemaDescription {
    pub name: windows_core::PCSTR,
    pub operatorSetVersionAtLastChange: i32,
    pub inputs: *const MLOperatorSchemaEdgeDescription,
    pub inputCount: u32,
    pub outputs: *const MLOperatorSchemaEdgeDescription,
    pub outputCount: u32,
    pub typeConstraints: *const MLOperatorEdgeTypeConstraint,
    pub typeConstraintCount: u32,
    pub attributes: *const MLOperatorAttribute,
    pub attributeCount: u32,
    pub defaultAttributes: *const MLOperatorAttributeNameValue,
    pub defaultAttributeCount: u32,
}
impl Default for MLOperatorSchemaDescription {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct MLOperatorSchemaEdgeDescription {
    pub options: MLOperatorParameterOptions,
    pub typeFormat: MLOperatorSchemaEdgeTypeFormat,
    pub Anonymous: MLOperatorSchemaEdgeDescription_0,
}
impl Default for MLOperatorSchemaEdgeDescription {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union MLOperatorSchemaEdgeDescription_0 {
    pub reserved: *const core::ffi::c_void,
    pub typeLabel: windows_core::PCSTR,
    pub edgeDescription: MLOperatorEdgeDescription,
}
impl Default for MLOperatorSchemaEdgeDescription_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MLOperatorSchemaEdgeTypeFormat(pub i32);
impl MLOperatorSchemaEdgeTypeFormat {
    pub const EdgeDescription: Self = Self(0i32);
    pub const Label: Self = Self(1i32);
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MLOperatorSetId {
    pub domain: windows_core::PCSTR,
    pub version: i32,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MLOperatorTensorDataType(pub u32);
impl MLOperatorTensorDataType {
    pub const Undefined: Self = Self(0u32);
    pub const Float: Self = Self(1u32);
    pub const UInt8: Self = Self(2u32);
    pub const Int8: Self = Self(3u32);
    pub const UInt16: Self = Self(4u32);
    pub const Int16: Self = Self(5u32);
    pub const Int32: Self = Self(6u32);
    pub const Int64: Self = Self(7u32);
    pub const String: Self = Self(8u32);
    pub const Bool: Self = Self(9u32);
    pub const Float16: Self = Self(10u32);
    pub const Double: Self = Self(11u32);
    pub const UInt32: Self = Self(12u32);
    pub const UInt64: Self = Self(13u32);
    pub const Complex64: Self = Self(14u32);
    pub const Complex128: Self = Self(15u32);
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Direct3D12")]
pub struct WINML_BINDING_DESC {
    pub Name: windows_core::PCWSTR,
    pub BindType: WINML_BINDING_TYPE,
    pub Anonymous: WINML_BINDING_DESC_0,
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl Clone for WINML_BINDING_DESC {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl Default for WINML_BINDING_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Direct3D12")]
pub union WINML_BINDING_DESC_0 {
    pub Tensor: WINML_TENSOR_BINDING_DESC,
    pub Sequence: WINML_SEQUENCE_BINDING_DESC,
    pub Map: WINML_MAP_BINDING_DESC,
    pub Image: WINML_IMAGE_BINDING_DESC,
    pub Resource: core::mem::ManuallyDrop<WINML_RESOURCE_BINDING_DESC>,
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl Clone for WINML_BINDING_DESC_0 {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl Default for WINML_BINDING_DESC_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const WINML_BINDING_IMAGE: WINML_BINDING_TYPE = WINML_BINDING_TYPE(4i32);
pub const WINML_BINDING_MAP: WINML_BINDING_TYPE = WINML_BINDING_TYPE(3i32);
pub const WINML_BINDING_RESOURCE: WINML_BINDING_TYPE = WINML_BINDING_TYPE(5i32);
pub const WINML_BINDING_SEQUENCE: WINML_BINDING_TYPE = WINML_BINDING_TYPE(2i32);
pub const WINML_BINDING_TENSOR: WINML_BINDING_TYPE = WINML_BINDING_TYPE(1i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WINML_BINDING_TYPE(pub i32);
pub const WINML_BINDING_UNDEFINED: WINML_BINDING_TYPE = WINML_BINDING_TYPE(0i32);
pub const WINML_FEATURE_IMAGE: WINML_FEATURE_TYPE = WINML_FEATURE_TYPE(4i32);
pub const WINML_FEATURE_MAP: WINML_FEATURE_TYPE = WINML_FEATURE_TYPE(3i32);
pub const WINML_FEATURE_SEQUENCE: WINML_FEATURE_TYPE = WINML_FEATURE_TYPE(2i32);
pub const WINML_FEATURE_TENSOR: WINML_FEATURE_TYPE = WINML_FEATURE_TYPE(1i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WINML_FEATURE_TYPE(pub i32);
pub const WINML_FEATURE_UNDEFINED: WINML_FEATURE_TYPE = WINML_FEATURE_TYPE(0i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WINML_IMAGE_BINDING_DESC {
    pub ElementType: WINML_TENSOR_DATA_TYPE,
    pub NumDimensions: u32,
    pub pShape: *mut i64,
    pub DataSize: u32,
    pub pData: *mut core::ffi::c_void,
}
impl Default for WINML_IMAGE_BINDING_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WINML_IMAGE_VARIABLE_DESC {
    pub ElementType: WINML_TENSOR_DATA_TYPE,
    pub NumDimensions: u32,
    pub pShape: *mut i64,
}
impl Default for WINML_IMAGE_VARIABLE_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WINML_MAP_BINDING_DESC {
    pub ElementCount: u32,
    pub KeyType: WINML_TENSOR_DATA_TYPE,
    pub Anonymous1: WINML_MAP_BINDING_DESC_0,
    pub Fields: WINML_TENSOR_DATA_TYPE,
    pub Anonymous2: WINML_MAP_BINDING_DESC_1,
}
impl Default for WINML_MAP_BINDING_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union WINML_MAP_BINDING_DESC_0 {
    pub pStringKeys: *mut windows_core::PWSTR,
    pub pIntKeys: *mut i64,
}
impl Default for WINML_MAP_BINDING_DESC_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union WINML_MAP_BINDING_DESC_1 {
    pub pStringFields: *mut windows_core::PWSTR,
    pub pIntFields: *mut i64,
    pub pFloatFields: *mut f32,
    pub pDoubleFields: *mut f64,
}
impl Default for WINML_MAP_BINDING_DESC_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WINML_MAP_VARIABLE_DESC {
    pub KeyType: WINML_TENSOR_DATA_TYPE,
    pub Fields: WINML_TENSOR_DATA_TYPE,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WINML_MODEL_DESC {
    pub Author: windows_core::PWSTR,
    pub Name: windows_core::PWSTR,
    pub Domain: windows_core::PWSTR,
    pub Description: windows_core::PWSTR,
    pub Version: usize,
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Direct3D12")]
#[derive(Clone, Debug, PartialEq)]
pub struct WINML_RESOURCE_BINDING_DESC {
    pub ElementType: WINML_TENSOR_DATA_TYPE,
    pub NumDimensions: u32,
    pub pShape: *mut i64,
    pub pResource: core::mem::ManuallyDrop<Option<super::super::super::Graphics::Direct3D12::ID3D12Resource>>,
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl Default for WINML_RESOURCE_BINDING_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const WINML_RUNTIME_CNTK: WINML_RUNTIME_TYPE = WINML_RUNTIME_TYPE(0i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WINML_RUNTIME_TYPE(pub i32);
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WINML_SEQUENCE_BINDING_DESC {
    pub ElementCount: u32,
    pub ElementType: WINML_TENSOR_DATA_TYPE,
    pub Anonymous: WINML_SEQUENCE_BINDING_DESC_0,
}
impl Default for WINML_SEQUENCE_BINDING_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union WINML_SEQUENCE_BINDING_DESC_0 {
    pub pStrings: *mut windows_core::PWSTR,
    pub pInts: *mut i64,
    pub pFloats: *mut f32,
    pub pDoubles: *mut f64,
}
impl Default for WINML_SEQUENCE_BINDING_DESC_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WINML_SEQUENCE_VARIABLE_DESC {
    pub ElementType: WINML_TENSOR_DATA_TYPE,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WINML_TENSOR_BINDING_DESC {
    pub DataType: WINML_TENSOR_DATA_TYPE,
    pub NumDimensions: u32,
    pub pShape: *mut i64,
    pub DataSize: u32,
    pub pData: *mut core::ffi::c_void,
}
impl Default for WINML_TENSOR_BINDING_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const WINML_TENSOR_BOOLEAN: WINML_TENSOR_DATA_TYPE = WINML_TENSOR_DATA_TYPE(9i32);
pub const WINML_TENSOR_COMPLEX128: WINML_TENSOR_DATA_TYPE = WINML_TENSOR_DATA_TYPE(15i32);
pub const WINML_TENSOR_COMPLEX64: WINML_TENSOR_DATA_TYPE = WINML_TENSOR_DATA_TYPE(14i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WINML_TENSOR_DATA_TYPE(pub i32);
pub const WINML_TENSOR_DIMENSION_COUNT_MAX: u32 = 4u32;
pub const WINML_TENSOR_DOUBLE: WINML_TENSOR_DATA_TYPE = WINML_TENSOR_DATA_TYPE(11i32);
pub const WINML_TENSOR_FLOAT: WINML_TENSOR_DATA_TYPE = WINML_TENSOR_DATA_TYPE(1i32);
pub const WINML_TENSOR_FLOAT16: WINML_TENSOR_DATA_TYPE = WINML_TENSOR_DATA_TYPE(10i32);
pub const WINML_TENSOR_INT16: WINML_TENSOR_DATA_TYPE = WINML_TENSOR_DATA_TYPE(5i32);
pub const WINML_TENSOR_INT32: WINML_TENSOR_DATA_TYPE = WINML_TENSOR_DATA_TYPE(6i32);
pub const WINML_TENSOR_INT64: WINML_TENSOR_DATA_TYPE = WINML_TENSOR_DATA_TYPE(7i32);
pub const WINML_TENSOR_INT8: WINML_TENSOR_DATA_TYPE = WINML_TENSOR_DATA_TYPE(3i32);
pub const WINML_TENSOR_STRING: WINML_TENSOR_DATA_TYPE = WINML_TENSOR_DATA_TYPE(8i32);
pub const WINML_TENSOR_UINT16: WINML_TENSOR_DATA_TYPE = WINML_TENSOR_DATA_TYPE(4i32);
pub const WINML_TENSOR_UINT32: WINML_TENSOR_DATA_TYPE = WINML_TENSOR_DATA_TYPE(12i32);
pub const WINML_TENSOR_UINT64: WINML_TENSOR_DATA_TYPE = WINML_TENSOR_DATA_TYPE(13i32);
pub const WINML_TENSOR_UINT8: WINML_TENSOR_DATA_TYPE = WINML_TENSOR_DATA_TYPE(2i32);
pub const WINML_TENSOR_UNDEFINED: WINML_TENSOR_DATA_TYPE = WINML_TENSOR_DATA_TYPE(0i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WINML_TENSOR_VARIABLE_DESC {
    pub ElementType: WINML_TENSOR_DATA_TYPE,
    pub NumDimensions: u32,
    pub pShape: *mut i64,
}
impl Default for WINML_TENSOR_VARIABLE_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WINML_VARIABLE_DESC {
    pub Name: windows_core::PWSTR,
    pub Description: windows_core::PWSTR,
    pub FeatureType: WINML_FEATURE_TYPE,
    pub Required: windows_core::BOOL,
    pub Anonymous: WINML_VARIABLE_DESC_0,
}
impl Default for WINML_VARIABLE_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union WINML_VARIABLE_DESC_0 {
    pub Tensor: WINML_TENSOR_VARIABLE_DESC,
    pub Sequence: WINML_SEQUENCE_VARIABLE_DESC,
    pub Map: WINML_MAP_VARIABLE_DESC,
    pub Image: WINML_IMAGE_VARIABLE_DESC,
}
impl Default for WINML_VARIABLE_DESC_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
