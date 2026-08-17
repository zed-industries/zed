windows_core::imp::define_interface!(IGazeDevicePreview, IGazeDevicePreview_Vtbl, 0xe79e7ee9_b389_11e7_b201_c8d3ffb75721);
impl windows_core::RuntimeType for IGazeDevicePreview {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IGazeDevicePreview_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Id: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub CanTrackEyes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub CanTrackHead: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub ConfigurationState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut GazeDeviceConfigurationStatePreview) -> windows_core::HRESULT,
    pub RequestCalibrationAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Devices_HumanInterfaceDevice", feature = "Foundation_Collections"))]
    pub GetNumericControlDescriptions: unsafe extern "system" fn(*mut core::ffi::c_void, u16, u16, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Devices_HumanInterfaceDevice", feature = "Foundation_Collections")))]
    GetNumericControlDescriptions: usize,
    #[cfg(all(feature = "Devices_HumanInterfaceDevice", feature = "Foundation_Collections"))]
    pub GetBooleanControlDescriptions: unsafe extern "system" fn(*mut core::ffi::c_void, u16, u16, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Devices_HumanInterfaceDevice", feature = "Foundation_Collections")))]
    GetBooleanControlDescriptions: usize,
}
windows_core::imp::define_interface!(IGazeDeviceWatcherAddedPreviewEventArgs, IGazeDeviceWatcherAddedPreviewEventArgs_Vtbl, 0xe79e7eed_b389_11e7_b201_c8d3ffb75721);
impl windows_core::RuntimeType for IGazeDeviceWatcherAddedPreviewEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IGazeDeviceWatcherAddedPreviewEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Device: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IGazeDeviceWatcherPreview, IGazeDeviceWatcherPreview_Vtbl, 0xe79e7ee7_b389_11e7_b201_c8d3ffb75721);
impl windows_core::RuntimeType for IGazeDeviceWatcherPreview {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IGazeDeviceWatcherPreview_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Added: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveAdded: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub Removed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveRemoved: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub Updated: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveUpdated: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub EnumerationCompleted: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveEnumerationCompleted: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub Start: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Stop: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IGazeDeviceWatcherRemovedPreviewEventArgs, IGazeDeviceWatcherRemovedPreviewEventArgs_Vtbl, 0xf2631f08_0e3f_431f_a606_50b35af94a1c);
impl windows_core::RuntimeType for IGazeDeviceWatcherRemovedPreviewEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IGazeDeviceWatcherRemovedPreviewEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Device: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IGazeDeviceWatcherUpdatedPreviewEventArgs, IGazeDeviceWatcherUpdatedPreviewEventArgs_Vtbl, 0x7fe830ef_7f08_4737_88e1_4a83ae4e4885);
impl windows_core::RuntimeType for IGazeDeviceWatcherUpdatedPreviewEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IGazeDeviceWatcherUpdatedPreviewEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Device: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IGazeEnteredPreviewEventArgs, IGazeEnteredPreviewEventArgs_Vtbl, 0x2567bf43_1225_489f_9dd1_daa7c50fbf4b);
impl windows_core::RuntimeType for IGazeEnteredPreviewEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IGazeEnteredPreviewEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Handled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub CurrentPoint: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IGazeExitedPreviewEventArgs, IGazeExitedPreviewEventArgs_Vtbl, 0x5d0af07e_7d83_40ef_9f0a_fbc1bbdcc5ac);
impl windows_core::RuntimeType for IGazeExitedPreviewEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IGazeExitedPreviewEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Handled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub CurrentPoint: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IGazeInputSourcePreview, IGazeInputSourcePreview_Vtbl, 0xe79e7ee8_b389_11e7_b201_c8d3ffb75721);
impl windows_core::RuntimeType for IGazeInputSourcePreview {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IGazeInputSourcePreview_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GazeMoved: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveGazeMoved: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub GazeEntered: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveGazeEntered: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub GazeExited: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveGazeExited: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IGazeInputSourcePreviewStatics, IGazeInputSourcePreviewStatics_Vtbl, 0xe79e7ee6_b389_11e7_b201_c8d3ffb75721);
impl windows_core::RuntimeType for IGazeInputSourcePreviewStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IGazeInputSourcePreviewStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetForCurrentView: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateWatcher: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IGazeMovedPreviewEventArgs, IGazeMovedPreviewEventArgs_Vtbl, 0xe79e7eeb_b389_11e7_b201_c8d3ffb75721);
impl windows_core::RuntimeType for IGazeMovedPreviewEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IGazeMovedPreviewEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Handled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub CurrentPoint: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetIntermediatePoints: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetIntermediatePoints: usize,
}
windows_core::imp::define_interface!(IGazePointPreview, IGazePointPreview_Vtbl, 0xe79e7eea_b389_11e7_b201_c8d3ffb75721);
impl windows_core::RuntimeType for IGazePointPreview {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IGazePointPreview_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SourceDevice: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EyeGazePosition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub HeadGazePosition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Timestamp: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u64) -> windows_core::HRESULT,
    #[cfg(feature = "Devices_HumanInterfaceDevice")]
    pub HidInputReport: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Devices_HumanInterfaceDevice"))]
    HidInputReport: usize,
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct GazeDevicePreview(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(GazeDevicePreview, windows_core::IUnknown, windows_core::IInspectable);
impl GazeDevicePreview {
    pub fn Id(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Id)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn CanTrackEyes(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CanTrackEyes)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn CanTrackHead(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CanTrackHead)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ConfigurationState(&self) -> windows_core::Result<GazeDeviceConfigurationStatePreview> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ConfigurationState)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn RequestCalibrationAsync(&self) -> windows_core::Result<super::super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RequestCalibrationAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Devices_HumanInterfaceDevice", feature = "Foundation_Collections"))]
    pub fn GetNumericControlDescriptions(&self, usagepage: u16, usageid: u16) -> windows_core::Result<super::super::super::Foundation::Collections::IVectorView<super::super::HumanInterfaceDevice::HidNumericControlDescription>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetNumericControlDescriptions)(windows_core::Interface::as_raw(this), usagepage, usageid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Devices_HumanInterfaceDevice", feature = "Foundation_Collections"))]
    pub fn GetBooleanControlDescriptions(&self, usagepage: u16, usageid: u16) -> windows_core::Result<super::super::super::Foundation::Collections::IVectorView<super::super::HumanInterfaceDevice::HidBooleanControlDescription>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetBooleanControlDescriptions)(windows_core::Interface::as_raw(this), usagepage, usageid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for GazeDevicePreview {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IGazeDevicePreview>();
}
unsafe impl windows_core::Interface for GazeDevicePreview {
    type Vtable = IGazeDevicePreview_Vtbl;
    const IID: windows_core::GUID = <IGazeDevicePreview as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for GazeDevicePreview {
    const NAME: &'static str = "Windows.Devices.Input.Preview.GazeDevicePreview";
}
unsafe impl Send for GazeDevicePreview {}
unsafe impl Sync for GazeDevicePreview {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct GazeDeviceWatcherAddedPreviewEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(GazeDeviceWatcherAddedPreviewEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl GazeDeviceWatcherAddedPreviewEventArgs {
    pub fn Device(&self) -> windows_core::Result<GazeDevicePreview> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Device)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for GazeDeviceWatcherAddedPreviewEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IGazeDeviceWatcherAddedPreviewEventArgs>();
}
unsafe impl windows_core::Interface for GazeDeviceWatcherAddedPreviewEventArgs {
    type Vtable = IGazeDeviceWatcherAddedPreviewEventArgs_Vtbl;
    const IID: windows_core::GUID = <IGazeDeviceWatcherAddedPreviewEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for GazeDeviceWatcherAddedPreviewEventArgs {
    const NAME: &'static str = "Windows.Devices.Input.Preview.GazeDeviceWatcherAddedPreviewEventArgs";
}
unsafe impl Send for GazeDeviceWatcherAddedPreviewEventArgs {}
unsafe impl Sync for GazeDeviceWatcherAddedPreviewEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct GazeDeviceWatcherPreview(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(GazeDeviceWatcherPreview, windows_core::IUnknown, windows_core::IInspectable);
impl GazeDeviceWatcherPreview {
    pub fn Added<P0>(&self, handler: P0) -> windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::super::Foundation::TypedEventHandler<GazeDeviceWatcherPreview, GazeDeviceWatcherAddedPreviewEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Added)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveAdded(&self, token: super::super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveAdded)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn Removed<P0>(&self, handler: P0) -> windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::super::Foundation::TypedEventHandler<GazeDeviceWatcherPreview, GazeDeviceWatcherRemovedPreviewEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Removed)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveRemoved(&self, token: super::super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveRemoved)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn Updated<P0>(&self, handler: P0) -> windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::super::Foundation::TypedEventHandler<GazeDeviceWatcherPreview, GazeDeviceWatcherUpdatedPreviewEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Updated)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveUpdated(&self, token: super::super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveUpdated)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn EnumerationCompleted<P0>(&self, handler: P0) -> windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::super::Foundation::TypedEventHandler<GazeDeviceWatcherPreview, windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).EnumerationCompleted)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveEnumerationCompleted(&self, token: super::super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveEnumerationCompleted)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn Start(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Start)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Stop(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Stop)(windows_core::Interface::as_raw(this)).ok() }
    }
}
impl windows_core::RuntimeType for GazeDeviceWatcherPreview {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IGazeDeviceWatcherPreview>();
}
unsafe impl windows_core::Interface for GazeDeviceWatcherPreview {
    type Vtable = IGazeDeviceWatcherPreview_Vtbl;
    const IID: windows_core::GUID = <IGazeDeviceWatcherPreview as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for GazeDeviceWatcherPreview {
    const NAME: &'static str = "Windows.Devices.Input.Preview.GazeDeviceWatcherPreview";
}
unsafe impl Send for GazeDeviceWatcherPreview {}
unsafe impl Sync for GazeDeviceWatcherPreview {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct GazeDeviceWatcherRemovedPreviewEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(GazeDeviceWatcherRemovedPreviewEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl GazeDeviceWatcherRemovedPreviewEventArgs {
    pub fn Device(&self) -> windows_core::Result<GazeDevicePreview> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Device)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for GazeDeviceWatcherRemovedPreviewEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IGazeDeviceWatcherRemovedPreviewEventArgs>();
}
unsafe impl windows_core::Interface for GazeDeviceWatcherRemovedPreviewEventArgs {
    type Vtable = IGazeDeviceWatcherRemovedPreviewEventArgs_Vtbl;
    const IID: windows_core::GUID = <IGazeDeviceWatcherRemovedPreviewEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for GazeDeviceWatcherRemovedPreviewEventArgs {
    const NAME: &'static str = "Windows.Devices.Input.Preview.GazeDeviceWatcherRemovedPreviewEventArgs";
}
unsafe impl Send for GazeDeviceWatcherRemovedPreviewEventArgs {}
unsafe impl Sync for GazeDeviceWatcherRemovedPreviewEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct GazeDeviceWatcherUpdatedPreviewEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(GazeDeviceWatcherUpdatedPreviewEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl GazeDeviceWatcherUpdatedPreviewEventArgs {
    pub fn Device(&self) -> windows_core::Result<GazeDevicePreview> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Device)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for GazeDeviceWatcherUpdatedPreviewEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IGazeDeviceWatcherUpdatedPreviewEventArgs>();
}
unsafe impl windows_core::Interface for GazeDeviceWatcherUpdatedPreviewEventArgs {
    type Vtable = IGazeDeviceWatcherUpdatedPreviewEventArgs_Vtbl;
    const IID: windows_core::GUID = <IGazeDeviceWatcherUpdatedPreviewEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for GazeDeviceWatcherUpdatedPreviewEventArgs {
    const NAME: &'static str = "Windows.Devices.Input.Preview.GazeDeviceWatcherUpdatedPreviewEventArgs";
}
unsafe impl Send for GazeDeviceWatcherUpdatedPreviewEventArgs {}
unsafe impl Sync for GazeDeviceWatcherUpdatedPreviewEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct GazeEnteredPreviewEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(GazeEnteredPreviewEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl GazeEnteredPreviewEventArgs {
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
    pub fn CurrentPoint(&self) -> windows_core::Result<GazePointPreview> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CurrentPoint)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for GazeEnteredPreviewEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IGazeEnteredPreviewEventArgs>();
}
unsafe impl windows_core::Interface for GazeEnteredPreviewEventArgs {
    type Vtable = IGazeEnteredPreviewEventArgs_Vtbl;
    const IID: windows_core::GUID = <IGazeEnteredPreviewEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for GazeEnteredPreviewEventArgs {
    const NAME: &'static str = "Windows.Devices.Input.Preview.GazeEnteredPreviewEventArgs";
}
unsafe impl Send for GazeEnteredPreviewEventArgs {}
unsafe impl Sync for GazeEnteredPreviewEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct GazeExitedPreviewEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(GazeExitedPreviewEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl GazeExitedPreviewEventArgs {
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
    pub fn CurrentPoint(&self) -> windows_core::Result<GazePointPreview> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CurrentPoint)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for GazeExitedPreviewEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IGazeExitedPreviewEventArgs>();
}
unsafe impl windows_core::Interface for GazeExitedPreviewEventArgs {
    type Vtable = IGazeExitedPreviewEventArgs_Vtbl;
    const IID: windows_core::GUID = <IGazeExitedPreviewEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for GazeExitedPreviewEventArgs {
    const NAME: &'static str = "Windows.Devices.Input.Preview.GazeExitedPreviewEventArgs";
}
unsafe impl Send for GazeExitedPreviewEventArgs {}
unsafe impl Sync for GazeExitedPreviewEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct GazeInputSourcePreview(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(GazeInputSourcePreview, windows_core::IUnknown, windows_core::IInspectable);
impl GazeInputSourcePreview {
    pub fn GazeMoved<P0>(&self, handler: P0) -> windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::super::Foundation::TypedEventHandler<GazeInputSourcePreview, GazeMovedPreviewEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GazeMoved)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveGazeMoved(&self, token: super::super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveGazeMoved)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn GazeEntered<P0>(&self, handler: P0) -> windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::super::Foundation::TypedEventHandler<GazeInputSourcePreview, GazeEnteredPreviewEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GazeEntered)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveGazeEntered(&self, token: super::super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveGazeEntered)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn GazeExited<P0>(&self, handler: P0) -> windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::super::Foundation::TypedEventHandler<GazeInputSourcePreview, GazeExitedPreviewEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GazeExited)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveGazeExited(&self, token: super::super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveGazeExited)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn GetForCurrentView() -> windows_core::Result<GazeInputSourcePreview> {
        Self::IGazeInputSourcePreviewStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetForCurrentView)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateWatcher() -> windows_core::Result<GazeDeviceWatcherPreview> {
        Self::IGazeInputSourcePreviewStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateWatcher)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IGazeInputSourcePreviewStatics<R, F: FnOnce(&IGazeInputSourcePreviewStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<GazeInputSourcePreview, IGazeInputSourcePreviewStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for GazeInputSourcePreview {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IGazeInputSourcePreview>();
}
unsafe impl windows_core::Interface for GazeInputSourcePreview {
    type Vtable = IGazeInputSourcePreview_Vtbl;
    const IID: windows_core::GUID = <IGazeInputSourcePreview as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for GazeInputSourcePreview {
    const NAME: &'static str = "Windows.Devices.Input.Preview.GazeInputSourcePreview";
}
unsafe impl Send for GazeInputSourcePreview {}
unsafe impl Sync for GazeInputSourcePreview {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct GazeMovedPreviewEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(GazeMovedPreviewEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl GazeMovedPreviewEventArgs {
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
    pub fn CurrentPoint(&self) -> windows_core::Result<GazePointPreview> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CurrentPoint)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetIntermediatePoints(&self) -> windows_core::Result<super::super::super::Foundation::Collections::IVector<GazePointPreview>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetIntermediatePoints)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for GazeMovedPreviewEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IGazeMovedPreviewEventArgs>();
}
unsafe impl windows_core::Interface for GazeMovedPreviewEventArgs {
    type Vtable = IGazeMovedPreviewEventArgs_Vtbl;
    const IID: windows_core::GUID = <IGazeMovedPreviewEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for GazeMovedPreviewEventArgs {
    const NAME: &'static str = "Windows.Devices.Input.Preview.GazeMovedPreviewEventArgs";
}
unsafe impl Send for GazeMovedPreviewEventArgs {}
unsafe impl Sync for GazeMovedPreviewEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct GazePointPreview(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(GazePointPreview, windows_core::IUnknown, windows_core::IInspectable);
impl GazePointPreview {
    pub fn SourceDevice(&self) -> windows_core::Result<GazeDevicePreview> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SourceDevice)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn EyeGazePosition(&self) -> windows_core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::Point>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).EyeGazePosition)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn HeadGazePosition(&self) -> windows_core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::Point>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HeadGazePosition)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Timestamp(&self) -> windows_core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Timestamp)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Devices_HumanInterfaceDevice")]
    pub fn HidInputReport(&self) -> windows_core::Result<super::super::HumanInterfaceDevice::HidInputReport> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HidInputReport)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for GazePointPreview {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IGazePointPreview>();
}
unsafe impl windows_core::Interface for GazePointPreview {
    type Vtable = IGazePointPreview_Vtbl;
    const IID: windows_core::GUID = <IGazePointPreview as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for GazePointPreview {
    const NAME: &'static str = "Windows.Devices.Input.Preview.GazePointPreview";
}
unsafe impl Send for GazePointPreview {}
unsafe impl Sync for GazePointPreview {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct GazeDeviceConfigurationStatePreview(pub i32);
impl GazeDeviceConfigurationStatePreview {
    pub const Unknown: Self = Self(0i32);
    pub const Ready: Self = Self(1i32);
    pub const Configuring: Self = Self(2i32);
    pub const ScreenSetupNeeded: Self = Self(3i32);
    pub const UserCalibrationNeeded: Self = Self(4i32);
}
impl windows_core::TypeKind for GazeDeviceConfigurationStatePreview {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for GazeDeviceConfigurationStatePreview {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("GazeDeviceConfigurationStatePreview").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for GazeDeviceConfigurationStatePreview {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Input.Preview.GazeDeviceConfigurationStatePreview;i4)");
}
