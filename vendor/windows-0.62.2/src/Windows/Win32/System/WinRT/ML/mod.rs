windows_core::imp::define_interface!(ILearningModelDeviceFactoryNative, ILearningModelDeviceFactoryNative_Vtbl, 0x1e9b31a1_662e_4ae0_af67_f63bb337e634);
windows_core::imp::interface_hierarchy!(ILearningModelDeviceFactoryNative, windows_core::IUnknown);
impl ILearningModelDeviceFactoryNative {
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn CreateFromD3D12CommandQueue<P0>(&self, value: P0) -> windows_core::Result<windows_core::IUnknown>
    where
        P0: windows_core::Param<super::super::super::Graphics::Direct3D12::ID3D12CommandQueue>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateFromD3D12CommandQueue)(windows_core::Interface::as_raw(self), value.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ILearningModelDeviceFactoryNative_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub CreateFromD3D12CommandQueue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D12"))]
    CreateFromD3D12CommandQueue: usize,
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
pub trait ILearningModelDeviceFactoryNative_Impl: windows_core::IUnknownImpl {
    fn CreateFromD3D12CommandQueue(&self, value: windows_core::Ref<super::super::super::Graphics::Direct3D12::ID3D12CommandQueue>) -> windows_core::Result<windows_core::IUnknown>;
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ILearningModelDeviceFactoryNative_Vtbl {
    pub const fn new<Identity: ILearningModelDeviceFactoryNative_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateFromD3D12CommandQueue<Identity: ILearningModelDeviceFactoryNative_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut core::ffi::c_void, result: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ILearningModelDeviceFactoryNative_Impl::CreateFromD3D12CommandQueue(this, core::mem::transmute_copy(&value)) {
                    Ok(ok__) => {
                        result.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), CreateFromD3D12CommandQueue: CreateFromD3D12CommandQueue::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ILearningModelDeviceFactoryNative as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl windows_core::RuntimeName for ILearningModelDeviceFactoryNative {}
windows_core::imp::define_interface!(ILearningModelOperatorProviderNative, ILearningModelOperatorProviderNative_Vtbl, 0x1adaa23a_eb67_41f3_aad8_5d984e9bacd4);
windows_core::imp::interface_hierarchy!(ILearningModelOperatorProviderNative, windows_core::IUnknown);
impl ILearningModelOperatorProviderNative {
    #[cfg(feature = "Win32_AI_MachineLearning_WinML")]
    pub unsafe fn GetRegistry(&self) -> windows_core::Result<super::super::super::AI::MachineLearning::WinML::IMLOperatorRegistry> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetRegistry)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ILearningModelOperatorProviderNative_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_AI_MachineLearning_WinML")]
    pub GetRegistry: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_AI_MachineLearning_WinML"))]
    GetRegistry: usize,
}
#[cfg(feature = "Win32_AI_MachineLearning_WinML")]
pub trait ILearningModelOperatorProviderNative_Impl: windows_core::IUnknownImpl {
    fn GetRegistry(&self) -> windows_core::Result<super::super::super::AI::MachineLearning::WinML::IMLOperatorRegistry>;
}
#[cfg(feature = "Win32_AI_MachineLearning_WinML")]
impl ILearningModelOperatorProviderNative_Vtbl {
    pub const fn new<Identity: ILearningModelOperatorProviderNative_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetRegistry<Identity: ILearningModelOperatorProviderNative_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppoperatorregistry: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ILearningModelOperatorProviderNative_Impl::GetRegistry(this) {
                    Ok(ok__) => {
                        ppoperatorregistry.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetRegistry: GetRegistry::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ILearningModelOperatorProviderNative as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_AI_MachineLearning_WinML")]
impl windows_core::RuntimeName for ILearningModelOperatorProviderNative {}
windows_core::imp::define_interface!(ILearningModelSessionOptionsNative, ILearningModelSessionOptionsNative_Vtbl, 0xc71e953f_37b4_4564_8658_d8396866db0d);
windows_core::imp::interface_hierarchy!(ILearningModelSessionOptionsNative, windows_core::IUnknown);
impl ILearningModelSessionOptionsNative {
    pub unsafe fn SetIntraOpNumThreadsOverride(&self, intraopnumthreads: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetIntraOpNumThreadsOverride)(windows_core::Interface::as_raw(self), intraopnumthreads).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ILearningModelSessionOptionsNative_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetIntraOpNumThreadsOverride: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
pub trait ILearningModelSessionOptionsNative_Impl: windows_core::IUnknownImpl {
    fn SetIntraOpNumThreadsOverride(&self, intraopnumthreads: u32) -> windows_core::Result<()>;
}
impl ILearningModelSessionOptionsNative_Vtbl {
    pub const fn new<Identity: ILearningModelSessionOptionsNative_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetIntraOpNumThreadsOverride<Identity: ILearningModelSessionOptionsNative_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, intraopnumthreads: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ILearningModelSessionOptionsNative_Impl::SetIntraOpNumThreadsOverride(this, core::mem::transmute_copy(&intraopnumthreads)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), SetIntraOpNumThreadsOverride: SetIntraOpNumThreadsOverride::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ILearningModelSessionOptionsNative as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ILearningModelSessionOptionsNative {}
windows_core::imp::define_interface!(ILearningModelSessionOptionsNative1, ILearningModelSessionOptionsNative1_Vtbl, 0x5da37a26_0526_414b_91e4_2a0fa3ddba40);
windows_core::imp::interface_hierarchy!(ILearningModelSessionOptionsNative1, windows_core::IUnknown);
impl ILearningModelSessionOptionsNative1 {
    pub unsafe fn SetIntraOpThreadSpinning(&self, allowspinning: u8) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetIntraOpThreadSpinning)(windows_core::Interface::as_raw(self), allowspinning).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ILearningModelSessionOptionsNative1_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetIntraOpThreadSpinning: unsafe extern "system" fn(*mut core::ffi::c_void, u8) -> windows_core::HRESULT,
}
pub trait ILearningModelSessionOptionsNative1_Impl: windows_core::IUnknownImpl {
    fn SetIntraOpThreadSpinning(&self, allowspinning: u8) -> windows_core::Result<()>;
}
impl ILearningModelSessionOptionsNative1_Vtbl {
    pub const fn new<Identity: ILearningModelSessionOptionsNative1_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetIntraOpThreadSpinning<Identity: ILearningModelSessionOptionsNative1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, allowspinning: u8) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ILearningModelSessionOptionsNative1_Impl::SetIntraOpThreadSpinning(this, core::mem::transmute_copy(&allowspinning)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), SetIntraOpThreadSpinning: SetIntraOpThreadSpinning::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ILearningModelSessionOptionsNative1 as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ILearningModelSessionOptionsNative1 {}
windows_core::imp::define_interface!(ITensorNative, ITensorNative_Vtbl, 0x52f547ef_5b03_49b5_82d6_565f1ee0dd49);
windows_core::imp::interface_hierarchy!(ITensorNative, windows_core::IUnknown);
impl ITensorNative {
    pub unsafe fn GetBuffer(&self, value: *mut *mut u8, capacity: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetBuffer)(windows_core::Interface::as_raw(self), value as _, capacity as _).ok() }
    }
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn GetD3D12Resource(&self) -> windows_core::Result<super::super::super::Graphics::Direct3D12::ID3D12Resource> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetD3D12Resource)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITensorNative_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut u8, *mut u32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub GetD3D12Resource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D12"))]
    GetD3D12Resource: usize,
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
pub trait ITensorNative_Impl: windows_core::IUnknownImpl {
    fn GetBuffer(&self, value: *mut *mut u8, capacity: *mut u32) -> windows_core::Result<()>;
    fn GetD3D12Resource(&self) -> windows_core::Result<super::super::super::Graphics::Direct3D12::ID3D12Resource>;
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ITensorNative_Vtbl {
    pub const fn new<Identity: ITensorNative_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetBuffer<Identity: ITensorNative_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut *mut u8, capacity: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITensorNative_Impl::GetBuffer(this, core::mem::transmute_copy(&value), core::mem::transmute_copy(&capacity)).into()
            }
        }
        unsafe extern "system" fn GetD3D12Resource<Identity: ITensorNative_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITensorNative_Impl::GetD3D12Resource(this) {
                    Ok(ok__) => {
                        result.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetBuffer: GetBuffer::<Identity, OFFSET>,
            GetD3D12Resource: GetD3D12Resource::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITensorNative as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl windows_core::RuntimeName for ITensorNative {}
windows_core::imp::define_interface!(ITensorStaticsNative, ITensorStaticsNative_Vtbl, 0x39d055a4_66f6_4ebc_95d9_7a29ebe7690a);
windows_core::imp::interface_hierarchy!(ITensorStaticsNative, windows_core::IUnknown);
impl ITensorStaticsNative {
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn CreateFromD3D12Resource<P0>(&self, value: P0, shape: *mut i64, shapecount: i32, result: *mut Option<windows_core::IUnknown>) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Graphics::Direct3D12::ID3D12Resource>,
    {
        unsafe { (windows_core::Interface::vtable(self).CreateFromD3D12Resource)(windows_core::Interface::as_raw(self), value.param().abi(), shape as _, shapecount, core::mem::transmute(result)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITensorStaticsNative_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub CreateFromD3D12Resource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D12"))]
    CreateFromD3D12Resource: usize,
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
pub trait ITensorStaticsNative_Impl: windows_core::IUnknownImpl {
    fn CreateFromD3D12Resource(&self, value: windows_core::Ref<super::super::super::Graphics::Direct3D12::ID3D12Resource>, shape: *mut i64, shapecount: i32, result: windows_core::OutRef<windows_core::IUnknown>) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ITensorStaticsNative_Vtbl {
    pub const fn new<Identity: ITensorStaticsNative_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateFromD3D12Resource<Identity: ITensorStaticsNative_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut core::ffi::c_void, shape: *mut i64, shapecount: i32, result: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITensorStaticsNative_Impl::CreateFromD3D12Resource(this, core::mem::transmute_copy(&value), core::mem::transmute_copy(&shape), core::mem::transmute_copy(&shapecount), core::mem::transmute_copy(&result)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), CreateFromD3D12Resource: CreateFromD3D12Resource::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITensorStaticsNative as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl windows_core::RuntimeName for ITensorStaticsNative {}
