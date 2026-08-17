windows_core::imp::define_interface!(ILearningModelDeviceFactoryNative, ILearningModelDeviceFactoryNative_Vtbl, 0x1e9b31a1_662e_4ae0_af67_f63bb337e634);
impl core::ops::Deref for ILearningModelDeviceFactoryNative {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ILearningModelDeviceFactoryNative, windows_core::IUnknown);
impl ILearningModelDeviceFactoryNative {
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn CreateFromD3D12CommandQueue<P0>(&self, value: P0) -> windows_core::Result<windows_core::IUnknown>
    where
        P0: windows_core::Param<super::super::super::Graphics::Direct3D12::ID3D12CommandQueue>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateFromD3D12CommandQueue)(windows_core::Interface::as_raw(self), value.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct ILearningModelDeviceFactoryNative_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub CreateFromD3D12CommandQueue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D12"))]
    CreateFromD3D12CommandQueue: usize,
}
windows_core::imp::define_interface!(ILearningModelOperatorProviderNative, ILearningModelOperatorProviderNative_Vtbl, 0x1adaa23a_eb67_41f3_aad8_5d984e9bacd4);
impl core::ops::Deref for ILearningModelOperatorProviderNative {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ILearningModelOperatorProviderNative, windows_core::IUnknown);
impl ILearningModelOperatorProviderNative {
    #[cfg(feature = "Win32_AI_MachineLearning_WinML")]
    pub unsafe fn GetRegistry(&self) -> windows_core::Result<super::super::super::AI::MachineLearning::WinML::IMLOperatorRegistry> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetRegistry)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct ILearningModelOperatorProviderNative_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_AI_MachineLearning_WinML")]
    pub GetRegistry: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_AI_MachineLearning_WinML"))]
    GetRegistry: usize,
}
windows_core::imp::define_interface!(ILearningModelSessionOptionsNative, ILearningModelSessionOptionsNative_Vtbl, 0xc71e953f_37b4_4564_8658_d8396866db0d);
impl core::ops::Deref for ILearningModelSessionOptionsNative {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ILearningModelSessionOptionsNative, windows_core::IUnknown);
impl ILearningModelSessionOptionsNative {
    pub unsafe fn SetIntraOpNumThreadsOverride(&self, intraopnumthreads: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetIntraOpNumThreadsOverride)(windows_core::Interface::as_raw(self), intraopnumthreads).ok()
    }
}
#[repr(C)]
pub struct ILearningModelSessionOptionsNative_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetIntraOpNumThreadsOverride: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ILearningModelSessionOptionsNative1, ILearningModelSessionOptionsNative1_Vtbl, 0x5da37a26_0526_414b_91e4_2a0fa3ddba40);
impl core::ops::Deref for ILearningModelSessionOptionsNative1 {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ILearningModelSessionOptionsNative1, windows_core::IUnknown);
impl ILearningModelSessionOptionsNative1 {
    pub unsafe fn SetIntraOpThreadSpinning(&self, allowspinning: u8) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetIntraOpThreadSpinning)(windows_core::Interface::as_raw(self), allowspinning).ok()
    }
}
#[repr(C)]
pub struct ILearningModelSessionOptionsNative1_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetIntraOpThreadSpinning: unsafe extern "system" fn(*mut core::ffi::c_void, u8) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ITensorNative, ITensorNative_Vtbl, 0x52f547ef_5b03_49b5_82d6_565f1ee0dd49);
impl core::ops::Deref for ITensorNative {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ITensorNative, windows_core::IUnknown);
impl ITensorNative {
    pub unsafe fn GetBuffer(&self, value: *mut *mut u8, capacity: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetBuffer)(windows_core::Interface::as_raw(self), value, capacity).ok()
    }
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn GetD3D12Resource(&self) -> windows_core::Result<super::super::super::Graphics::Direct3D12::ID3D12Resource> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetD3D12Resource)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct ITensorNative_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut u8, *mut u32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub GetD3D12Resource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D12"))]
    GetD3D12Resource: usize,
}
windows_core::imp::define_interface!(ITensorStaticsNative, ITensorStaticsNative_Vtbl, 0x39d055a4_66f6_4ebc_95d9_7a29ebe7690a);
impl core::ops::Deref for ITensorStaticsNative {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ITensorStaticsNative, windows_core::IUnknown);
impl ITensorStaticsNative {
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn CreateFromD3D12Resource<P0>(&self, value: P0, shape: *mut i64, shapecount: i32, result: *mut Option<windows_core::IUnknown>) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Graphics::Direct3D12::ID3D12Resource>,
    {
        (windows_core::Interface::vtable(self).CreateFromD3D12Resource)(windows_core::Interface::as_raw(self), value.param().abi(), shape, shapecount, core::mem::transmute(result)).ok()
    }
}
#[repr(C)]
pub struct ITensorStaticsNative_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub CreateFromD3D12Resource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D12"))]
    CreateFromD3D12Resource: usize,
}
#[cfg(feature = "implement")]
core::include!("impl.rs");
