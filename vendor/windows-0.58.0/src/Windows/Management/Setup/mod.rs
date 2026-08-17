windows_core::imp::define_interface!(IAgentProvisioningProgressReport, IAgentProvisioningProgressReport_Vtbl, 0x5097398a_70cc_5181_a7af_d31c167323d1);
impl windows_core::RuntimeType for IAgentProvisioningProgressReport {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAgentProvisioningProgressReport_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub State: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DeploymentAgentProgressState) -> windows_core::HRESULT,
    pub SetState: unsafe extern "system" fn(*mut core::ffi::c_void, DeploymentAgentProgressState) -> windows_core::HRESULT,
    pub ProgressPercentage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub SetProgressPercentage: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub EstimatedTimeRemaining: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::TimeSpan) -> windows_core::HRESULT,
    pub SetEstimatedTimeRemaining: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::TimeSpan) -> windows_core::HRESULT,
    pub DisplayProgress: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetDisplayProgress: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub DisplayProgressSecondary: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetDisplayProgressSecondary: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Batches: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Batches: usize,
    pub CurrentBatchIndex: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetCurrentBatchIndex: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDeploymentSessionConnectionChangedEventArgs, IDeploymentSessionConnectionChangedEventArgs_Vtbl, 0x8d40c631_6e4b_5d59_92f8_0de54c2a3c6b);
impl windows_core::RuntimeType for IDeploymentSessionConnectionChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IDeploymentSessionConnectionChangedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SessionId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Change: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DeploymentSessionConnectionChange) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDeploymentSessionHeartbeatRequestedEventArgs, IDeploymentSessionHeartbeatRequestedEventArgs_Vtbl, 0x09d81fa0_1036_58e6_b63b_fe343c45005f);
impl windows_core::RuntimeType for IDeploymentSessionHeartbeatRequestedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IDeploymentSessionHeartbeatRequestedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Handled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDeploymentSessionStateChangedEventArgs, IDeploymentSessionStateChangedEventArgs_Vtbl, 0xfbd3b7f3_88cb_5703_b8a5_0218de8fed81);
impl windows_core::RuntimeType for IDeploymentSessionStateChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IDeploymentSessionStateChangedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SessionId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Change: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DeploymentSessionStateChange) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDeploymentWorkload, IDeploymentWorkload_Vtbl, 0x1cefd3d4_456c_50d1_9312_cc5c818fc12e);
impl windows_core::RuntimeType for IDeploymentWorkload {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IDeploymentWorkload_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Id: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub DisplayFriendlyName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetDisplayFriendlyName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub StartTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetStartTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EndTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetEndTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ErrorCode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetErrorCode: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub ErrorMessage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetErrorMessage: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub PossibleCause: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetPossibleCause: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub PossibleResolution: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetPossibleResolution: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub State: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DeploymentWorkloadState) -> windows_core::HRESULT,
    pub SetState: unsafe extern "system" fn(*mut core::ffi::c_void, DeploymentWorkloadState) -> windows_core::HRESULT,
    pub StateDetails: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetStateDetails: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDeploymentWorkloadBatch, IDeploymentWorkloadBatch_Vtbl, 0x5e56e3df_b9c0_5fee_ba3f_e89d800a9bf2);
impl windows_core::RuntimeType for IDeploymentWorkloadBatch {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IDeploymentWorkloadBatch_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Id: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub DisplayCategoryTitle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetDisplayCategoryTitle: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub BatchWorkloads: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    BatchWorkloads: usize,
}
windows_core::imp::define_interface!(IDeploymentWorkloadBatchFactory, IDeploymentWorkloadBatchFactory_Vtbl, 0xd0209697_9560_5a05_bdf6_f1af535cb0d4);
impl windows_core::RuntimeType for IDeploymentWorkloadBatchFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IDeploymentWorkloadBatchFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDeploymentWorkloadFactory, IDeploymentWorkloadFactory_Vtbl, 0x41426c72_22a3_5339_bdf1_51268169aa61);
impl windows_core::RuntimeType for IDeploymentWorkloadFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IDeploymentWorkloadFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDevicePreparationExecutionContext, IDevicePreparationExecutionContext_Vtbl, 0x084f221b_2484_5e81_a4e7_83f6caf19dc4);
impl windows_core::RuntimeType for IDevicePreparationExecutionContext {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IDevicePreparationExecutionContext_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Context: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMachineProvisioningProgressReporter, IMachineProvisioningProgressReporter_Vtbl, 0xebd8677f_dfd2_59da_ac3d_753ee1667cbb);
impl windows_core::RuntimeType for IMachineProvisioningProgressReporter {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMachineProvisioningProgressReporter_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SessionId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub SessionConnection: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DeploymentSessionConnectionChange) -> windows_core::HRESULT,
    pub SessionState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DeploymentSessionStateChange) -> windows_core::HRESULT,
    pub SessionStateChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveSessionStateChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub SessionConnectionChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveSessionConnectionChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub ReportProgress: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDevicePreparationExecutionContextAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMachineProvisioningProgressReporterStatics, IMachineProvisioningProgressReporterStatics_Vtbl, 0x77682c17_5da3_51fc_a042_c7b53458ddb5);
impl windows_core::RuntimeType for IMachineProvisioningProgressReporterStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMachineProvisioningProgressReporterStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetForLaunchUri: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct AgentProvisioningProgressReport(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(AgentProvisioningProgressReport, windows_core::IUnknown, windows_core::IInspectable);
impl AgentProvisioningProgressReport {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<AgentProvisioningProgressReport, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn State(&self) -> windows_core::Result<DeploymentAgentProgressState> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).State)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetState(&self, value: DeploymentAgentProgressState) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetState)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ProgressPercentage(&self) -> windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ProgressPercentage)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetProgressPercentage(&self, value: f64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetProgressPercentage)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn EstimatedTimeRemaining(&self) -> windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).EstimatedTimeRemaining)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetEstimatedTimeRemaining(&self, value: super::super::Foundation::TimeSpan) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetEstimatedTimeRemaining)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn DisplayProgress(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DisplayProgress)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetDisplayProgress(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetDisplayProgress)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn DisplayProgressSecondary(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DisplayProgressSecondary)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetDisplayProgressSecondary(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetDisplayProgressSecondary)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Batches(&self) -> windows_core::Result<super::super::Foundation::Collections::IVector<DeploymentWorkloadBatch>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Batches)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CurrentBatchIndex(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CurrentBatchIndex)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetCurrentBatchIndex(&self, value: u32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetCurrentBatchIndex)(windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl windows_core::RuntimeType for AgentProvisioningProgressReport {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IAgentProvisioningProgressReport>();
}
unsafe impl windows_core::Interface for AgentProvisioningProgressReport {
    type Vtable = IAgentProvisioningProgressReport_Vtbl;
    const IID: windows_core::GUID = <IAgentProvisioningProgressReport as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for AgentProvisioningProgressReport {
    const NAME: &'static str = "Windows.Management.Setup.AgentProvisioningProgressReport";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct DeploymentSessionConnectionChangedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(DeploymentSessionConnectionChangedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl DeploymentSessionConnectionChangedEventArgs {
    pub fn SessionId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SessionId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Change(&self) -> windows_core::Result<DeploymentSessionConnectionChange> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Change)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for DeploymentSessionConnectionChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IDeploymentSessionConnectionChangedEventArgs>();
}
unsafe impl windows_core::Interface for DeploymentSessionConnectionChangedEventArgs {
    type Vtable = IDeploymentSessionConnectionChangedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IDeploymentSessionConnectionChangedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for DeploymentSessionConnectionChangedEventArgs {
    const NAME: &'static str = "Windows.Management.Setup.DeploymentSessionConnectionChangedEventArgs";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct DeploymentSessionHeartbeatRequestedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(DeploymentSessionHeartbeatRequestedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl DeploymentSessionHeartbeatRequestedEventArgs {
    pub fn Handled(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Handled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetHandled(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetHandled)(windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl windows_core::RuntimeType for DeploymentSessionHeartbeatRequestedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IDeploymentSessionHeartbeatRequestedEventArgs>();
}
unsafe impl windows_core::Interface for DeploymentSessionHeartbeatRequestedEventArgs {
    type Vtable = IDeploymentSessionHeartbeatRequestedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IDeploymentSessionHeartbeatRequestedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for DeploymentSessionHeartbeatRequestedEventArgs {
    const NAME: &'static str = "Windows.Management.Setup.DeploymentSessionHeartbeatRequestedEventArgs";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct DeploymentSessionStateChangedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(DeploymentSessionStateChangedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl DeploymentSessionStateChangedEventArgs {
    pub fn SessionId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SessionId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Change(&self) -> windows_core::Result<DeploymentSessionStateChange> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Change)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for DeploymentSessionStateChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IDeploymentSessionStateChangedEventArgs>();
}
unsafe impl windows_core::Interface for DeploymentSessionStateChangedEventArgs {
    type Vtable = IDeploymentSessionStateChangedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IDeploymentSessionStateChangedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for DeploymentSessionStateChangedEventArgs {
    const NAME: &'static str = "Windows.Management.Setup.DeploymentSessionStateChangedEventArgs";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct DeploymentWorkload(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(DeploymentWorkload, windows_core::IUnknown, windows_core::IInspectable);
impl DeploymentWorkload {
    pub fn Id(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Id)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn DisplayFriendlyName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DisplayFriendlyName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetDisplayFriendlyName(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetDisplayFriendlyName)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn StartTime(&self) -> windows_core::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).StartTime)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetStartTime<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::IReference<super::super::Foundation::DateTime>>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetStartTime)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn EndTime(&self) -> windows_core::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).EndTime)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetEndTime<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::IReference<super::super::Foundation::DateTime>>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetEndTime)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn ErrorCode(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ErrorCode)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetErrorCode(&self, value: u32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetErrorCode)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ErrorMessage(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ErrorMessage)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetErrorMessage(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetErrorMessage)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn PossibleCause(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PossibleCause)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetPossibleCause(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetPossibleCause)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn PossibleResolution(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PossibleResolution)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetPossibleResolution(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetPossibleResolution)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn State(&self) -> windows_core::Result<DeploymentWorkloadState> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).State)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetState(&self, value: DeploymentWorkloadState) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetState)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn StateDetails(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).StateDetails)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetStateDetails(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetStateDetails)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn CreateInstance(id: &windows_core::HSTRING) -> windows_core::Result<DeploymentWorkload> {
        Self::IDeploymentWorkloadFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(id), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IDeploymentWorkloadFactory<R, F: FnOnce(&IDeploymentWorkloadFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<DeploymentWorkload, IDeploymentWorkloadFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for DeploymentWorkload {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IDeploymentWorkload>();
}
unsafe impl windows_core::Interface for DeploymentWorkload {
    type Vtable = IDeploymentWorkload_Vtbl;
    const IID: windows_core::GUID = <IDeploymentWorkload as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for DeploymentWorkload {
    const NAME: &'static str = "Windows.Management.Setup.DeploymentWorkload";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct DeploymentWorkloadBatch(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(DeploymentWorkloadBatch, windows_core::IUnknown, windows_core::IInspectable);
impl DeploymentWorkloadBatch {
    pub fn Id(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Id)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn DisplayCategoryTitle(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DisplayCategoryTitle)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetDisplayCategoryTitle(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetDisplayCategoryTitle)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn BatchWorkloads(&self) -> windows_core::Result<super::super::Foundation::Collections::IVector<DeploymentWorkload>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BatchWorkloads)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CreateInstance(id: u32) -> windows_core::Result<DeploymentWorkloadBatch> {
        Self::IDeploymentWorkloadBatchFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(windows_core::Interface::as_raw(this), id, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IDeploymentWorkloadBatchFactory<R, F: FnOnce(&IDeploymentWorkloadBatchFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<DeploymentWorkloadBatch, IDeploymentWorkloadBatchFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for DeploymentWorkloadBatch {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IDeploymentWorkloadBatch>();
}
unsafe impl windows_core::Interface for DeploymentWorkloadBatch {
    type Vtable = IDeploymentWorkloadBatch_Vtbl;
    const IID: windows_core::GUID = <IDeploymentWorkloadBatch as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for DeploymentWorkloadBatch {
    const NAME: &'static str = "Windows.Management.Setup.DeploymentWorkloadBatch";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct DevicePreparationExecutionContext(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(DevicePreparationExecutionContext, windows_core::IUnknown, windows_core::IInspectable);
impl DevicePreparationExecutionContext {
    pub fn Context(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Context)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for DevicePreparationExecutionContext {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IDevicePreparationExecutionContext>();
}
unsafe impl windows_core::Interface for DevicePreparationExecutionContext {
    type Vtable = IDevicePreparationExecutionContext_Vtbl;
    const IID: windows_core::GUID = <IDevicePreparationExecutionContext as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for DevicePreparationExecutionContext {
    const NAME: &'static str = "Windows.Management.Setup.DevicePreparationExecutionContext";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct MachineProvisioningProgressReporter(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(MachineProvisioningProgressReporter, windows_core::IUnknown, windows_core::IInspectable);
impl MachineProvisioningProgressReporter {
    pub fn SessionId(&self) -> windows_core::Result<windows_core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SessionId)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SessionConnection(&self) -> windows_core::Result<DeploymentSessionConnectionChange> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SessionConnection)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SessionState(&self) -> windows_core::Result<DeploymentSessionStateChange> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SessionState)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SessionStateChanged<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<MachineProvisioningProgressReporter, DeploymentSessionStateChangedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SessionStateChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveSessionStateChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveSessionStateChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn SessionConnectionChanged<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<MachineProvisioningProgressReporter, DeploymentSessionConnectionChangedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SessionConnectionChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveSessionConnectionChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveSessionConnectionChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn ReportProgress<P0>(&self, updatereport: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<AgentProvisioningProgressReport>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).ReportProgress)(windows_core::Interface::as_raw(this), updatereport.param().abi()).ok() }
    }
    pub fn GetDevicePreparationExecutionContextAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncOperation<DevicePreparationExecutionContext>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDevicePreparationExecutionContextAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetForLaunchUri<P0, P1>(launchuri: P0, heartbeathandler: P1) -> windows_core::Result<MachineProvisioningProgressReporter>
    where
        P0: windows_core::Param<super::super::Foundation::Uri>,
        P1: windows_core::Param<DeploymentSessionHeartbeatRequested>,
    {
        Self::IMachineProvisioningProgressReporterStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetForLaunchUri)(windows_core::Interface::as_raw(this), launchuri.param().abi(), heartbeathandler.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IMachineProvisioningProgressReporterStatics<R, F: FnOnce(&IMachineProvisioningProgressReporterStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<MachineProvisioningProgressReporter, IMachineProvisioningProgressReporterStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for MachineProvisioningProgressReporter {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IMachineProvisioningProgressReporter>();
}
unsafe impl windows_core::Interface for MachineProvisioningProgressReporter {
    type Vtable = IMachineProvisioningProgressReporter_Vtbl;
    const IID: windows_core::GUID = <IMachineProvisioningProgressReporter as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for MachineProvisioningProgressReporter {
    const NAME: &'static str = "Windows.Management.Setup.MachineProvisioningProgressReporter";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DeploymentAgentProgressState(pub i32);
impl DeploymentAgentProgressState {
    pub const NotStarted: Self = Self(0i32);
    pub const Initializing: Self = Self(1i32);
    pub const InProgress: Self = Self(2i32);
    pub const Completed: Self = Self(3i32);
    pub const ErrorOccurred: Self = Self(4i32);
    pub const RebootRequired: Self = Self(5i32);
    pub const Canceled: Self = Self(6i32);
}
impl windows_core::TypeKind for DeploymentAgentProgressState {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DeploymentAgentProgressState {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DeploymentAgentProgressState").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for DeploymentAgentProgressState {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Management.Setup.DeploymentAgentProgressState;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DeploymentSessionConnectionChange(pub i32);
impl DeploymentSessionConnectionChange {
    pub const NoChange: Self = Self(0i32);
    pub const HostConnectionLost: Self = Self(1i32);
    pub const HostConnectionRestored: Self = Self(2i32);
    pub const AgentConnectionLost: Self = Self(3i32);
    pub const AgentConnectionRestored: Self = Self(4i32);
    pub const InternetConnectionLost: Self = Self(5i32);
    pub const InternetConnectionRestored: Self = Self(6i32);
}
impl windows_core::TypeKind for DeploymentSessionConnectionChange {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DeploymentSessionConnectionChange {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DeploymentSessionConnectionChange").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for DeploymentSessionConnectionChange {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Management.Setup.DeploymentSessionConnectionChange;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DeploymentSessionStateChange(pub i32);
impl DeploymentSessionStateChange {
    pub const NoChange: Self = Self(0i32);
    pub const CancelRequestedByUser: Self = Self(1i32);
    pub const RetryRequestedByUser: Self = Self(2i32);
}
impl windows_core::TypeKind for DeploymentSessionStateChange {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DeploymentSessionStateChange {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DeploymentSessionStateChange").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for DeploymentSessionStateChange {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Management.Setup.DeploymentSessionStateChange;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DeploymentWorkloadState(pub i32);
impl DeploymentWorkloadState {
    pub const NotStarted: Self = Self(0i32);
    pub const InProgress: Self = Self(1i32);
    pub const Completed: Self = Self(2i32);
    pub const Failed: Self = Self(3i32);
    pub const Canceled: Self = Self(4i32);
    pub const Skipped: Self = Self(5i32);
    pub const Uninstalled: Self = Self(6i32);
    pub const RebootRequired: Self = Self(7i32);
}
impl windows_core::TypeKind for DeploymentWorkloadState {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DeploymentWorkloadState {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DeploymentWorkloadState").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for DeploymentWorkloadState {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Management.Setup.DeploymentWorkloadState;i4)");
}
windows_core::imp::define_interface!(DeploymentSessionHeartbeatRequested, DeploymentSessionHeartbeatRequested_Vtbl, 0xc94a770b_5b05_4595_9e69_79070484377e);
impl DeploymentSessionHeartbeatRequested {
    pub fn new<F: FnMut(Option<&DeploymentSessionHeartbeatRequestedEventArgs>) -> windows_core::Result<()> + Send + 'static>(invoke: F) -> Self {
        let com = DeploymentSessionHeartbeatRequestedBox::<F> { vtable: &DeploymentSessionHeartbeatRequestedBox::<F>::VTABLE, count: windows_core::imp::RefCount::new(1), invoke };
        unsafe { core::mem::transmute(Box::new(com)) }
    }
    pub fn Invoke<P0>(&self, eventargs: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<DeploymentSessionHeartbeatRequestedEventArgs>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Invoke)(windows_core::Interface::as_raw(this), eventargs.param().abi()).ok() }
    }
}
#[repr(C)]
struct DeploymentSessionHeartbeatRequestedBox<F: FnMut(Option<&DeploymentSessionHeartbeatRequestedEventArgs>) -> windows_core::Result<()> + Send + 'static> {
    vtable: *const DeploymentSessionHeartbeatRequested_Vtbl,
    invoke: F,
    count: windows_core::imp::RefCount,
}
impl<F: FnMut(Option<&DeploymentSessionHeartbeatRequestedEventArgs>) -> windows_core::Result<()> + Send + 'static> DeploymentSessionHeartbeatRequestedBox<F> {
    const VTABLE: DeploymentSessionHeartbeatRequested_Vtbl = DeploymentSessionHeartbeatRequested_Vtbl { base__: windows_core::IUnknown_Vtbl { QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release }, Invoke: Self::Invoke };
    unsafe extern "system" fn QueryInterface(this: *mut core::ffi::c_void, iid: *const windows_core::GUID, interface: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
        let this = this as *mut *mut core::ffi::c_void as *mut Self;
        if iid.is_null() || interface.is_null() {
            return windows_core::HRESULT(-2147467261);
        }
        *interface = if *iid == <DeploymentSessionHeartbeatRequested as windows_core::Interface>::IID || *iid == <windows_core::IUnknown as windows_core::Interface>::IID || *iid == <windows_core::imp::IAgileObject as windows_core::Interface>::IID { &mut (*this).vtable as *mut _ as _ } else { core::ptr::null_mut() };
        if (*interface).is_null() {
            windows_core::HRESULT(-2147467262)
        } else {
            (*this).count.add_ref();
            windows_core::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: *mut core::ffi::c_void) -> u32 {
        let this = this as *mut *mut core::ffi::c_void as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: *mut core::ffi::c_void) -> u32 {
        let this = this as *mut *mut core::ffi::c_void as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            let _ = Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(this: *mut core::ffi::c_void, eventargs: *mut core::ffi::c_void) -> windows_core::HRESULT {
        let this = &mut *(this as *mut *mut core::ffi::c_void as *mut Self);
        (this.invoke)(windows_core::from_raw_borrowed(&eventargs)).into()
    }
}
impl windows_core::RuntimeType for DeploymentSessionHeartbeatRequested {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct DeploymentSessionHeartbeatRequested_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Invoke: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
