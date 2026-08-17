#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ActivationKind(pub i32);
impl ActivationKind {
    pub const Launch: Self = Self(0i32);
    pub const Search: Self = Self(1i32);
    pub const ShareTarget: Self = Self(2i32);
    pub const File: Self = Self(3i32);
    pub const Protocol: Self = Self(4i32);
    pub const FileOpenPicker: Self = Self(5i32);
    pub const FileSavePicker: Self = Self(6i32);
    pub const CachedFileUpdater: Self = Self(7i32);
    pub const ContactPicker: Self = Self(8i32);
    pub const Device: Self = Self(9i32);
    pub const PrintTaskSettings: Self = Self(10i32);
    pub const CameraSettings: Self = Self(11i32);
    pub const RestrictedLaunch: Self = Self(12i32);
    pub const AppointmentsProvider: Self = Self(13i32);
    pub const Contact: Self = Self(14i32);
    pub const LockScreenCall: Self = Self(15i32);
    pub const VoiceCommand: Self = Self(16i32);
    pub const LockScreen: Self = Self(17i32);
    pub const PickerReturned: Self = Self(1000i32);
    pub const WalletAction: Self = Self(1001i32);
    pub const PickFileContinuation: Self = Self(1002i32);
    pub const PickSaveFileContinuation: Self = Self(1003i32);
    pub const PickFolderContinuation: Self = Self(1004i32);
    pub const WebAuthenticationBrokerContinuation: Self = Self(1005i32);
    pub const WebAccountProvider: Self = Self(1006i32);
    pub const ComponentUI: Self = Self(1007i32);
    pub const ProtocolForResults: Self = Self(1009i32);
    pub const ToastNotification: Self = Self(1010i32);
    pub const Print3DWorkflow: Self = Self(1011i32);
    pub const DialReceiver: Self = Self(1012i32);
    pub const DevicePairing: Self = Self(1013i32);
    pub const UserDataAccountsProvider: Self = Self(1014i32);
    pub const FilePickerExperience: Self = Self(1015i32);
    pub const LockScreenComponent: Self = Self(1016i32);
    pub const ContactPanel: Self = Self(1017i32);
    pub const PrintWorkflowForegroundTask: Self = Self(1018i32);
    pub const GameUIProvider: Self = Self(1019i32);
    pub const StartupTask: Self = Self(1020i32);
    pub const CommandLineLaunch: Self = Self(1021i32);
    pub const BarcodeScannerProvider: Self = Self(1022i32);
    pub const PrintSupportJobUI: Self = Self(1023i32);
    pub const PrintSupportSettingsUI: Self = Self(1024i32);
    pub const PhoneCallActivation: Self = Self(1025i32);
    pub const VpnForeground: Self = Self(1026i32);
    pub const PrintSupportEnterpriseManagementUI: Self = Self(1027i32);
}
impl windows_core::TypeKind for ActivationKind {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for ActivationKind {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Activation.ActivationKind;i4)");
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ApplicationExecutionState(pub i32);
impl ApplicationExecutionState {
    pub const NotRunning: Self = Self(0i32);
    pub const Running: Self = Self(1i32);
    pub const Suspended: Self = Self(2i32);
    pub const Terminated: Self = Self(3i32);
    pub const ClosedByUser: Self = Self(4i32);
}
impl windows_core::TypeKind for ApplicationExecutionState {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for ApplicationExecutionState {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Activation.ApplicationExecutionState;i4)");
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AppointmentsProviderAddAppointmentActivatedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(AppointmentsProviderAddAppointmentActivatedEventArgs, windows_core::IUnknown, windows_core::IInspectable, IAppointmentsProviderAddAppointmentActivatedEventArgs);
windows_core::imp::required_hierarchy!(AppointmentsProviderAddAppointmentActivatedEventArgs, IActivatedEventArgs, IActivatedEventArgsWithUser, IAppointmentsProviderActivatedEventArgs);
impl AppointmentsProviderAddAppointmentActivatedEventArgs {
    pub fn Kind(&self) -> windows_core::Result<ActivationKind> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> windows_core::Result<ApplicationExecutionState> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PreviousExecutionState)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SplashScreen(&self) -> windows_core::Result<SplashScreen> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SplashScreen)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "System")]
    pub fn User(&self) -> windows_core::Result<super::super::System::User> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).User)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Verb(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IAppointmentsProviderActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Verb)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(feature = "ApplicationModel_Appointments_AppointmentsProvider")]
    pub fn AddAppointmentOperation(&self) -> windows_core::Result<super::Appointments::AppointmentsProvider::AddAppointmentOperation> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AddAppointmentOperation)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for AppointmentsProviderAddAppointmentActivatedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IAppointmentsProviderAddAppointmentActivatedEventArgs>();
}
unsafe impl windows_core::Interface for AppointmentsProviderAddAppointmentActivatedEventArgs {
    type Vtable = <IAppointmentsProviderAddAppointmentActivatedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IAppointmentsProviderAddAppointmentActivatedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for AppointmentsProviderAddAppointmentActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.AppointmentsProviderAddAppointmentActivatedEventArgs";
}
unsafe impl Send for AppointmentsProviderAddAppointmentActivatedEventArgs {}
unsafe impl Sync for AppointmentsProviderAddAppointmentActivatedEventArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AppointmentsProviderRemoveAppointmentActivatedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(AppointmentsProviderRemoveAppointmentActivatedEventArgs, windows_core::IUnknown, windows_core::IInspectable, IAppointmentsProviderRemoveAppointmentActivatedEventArgs);
windows_core::imp::required_hierarchy!(AppointmentsProviderRemoveAppointmentActivatedEventArgs, IActivatedEventArgs, IActivatedEventArgsWithUser, IAppointmentsProviderActivatedEventArgs);
impl AppointmentsProviderRemoveAppointmentActivatedEventArgs {
    pub fn Kind(&self) -> windows_core::Result<ActivationKind> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> windows_core::Result<ApplicationExecutionState> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PreviousExecutionState)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SplashScreen(&self) -> windows_core::Result<SplashScreen> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SplashScreen)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "System")]
    pub fn User(&self) -> windows_core::Result<super::super::System::User> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).User)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Verb(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IAppointmentsProviderActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Verb)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(feature = "ApplicationModel_Appointments_AppointmentsProvider")]
    pub fn RemoveAppointmentOperation(&self) -> windows_core::Result<super::Appointments::AppointmentsProvider::RemoveAppointmentOperation> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RemoveAppointmentOperation)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for AppointmentsProviderRemoveAppointmentActivatedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IAppointmentsProviderRemoveAppointmentActivatedEventArgs>();
}
unsafe impl windows_core::Interface for AppointmentsProviderRemoveAppointmentActivatedEventArgs {
    type Vtable = <IAppointmentsProviderRemoveAppointmentActivatedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IAppointmentsProviderRemoveAppointmentActivatedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for AppointmentsProviderRemoveAppointmentActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.AppointmentsProviderRemoveAppointmentActivatedEventArgs";
}
unsafe impl Send for AppointmentsProviderRemoveAppointmentActivatedEventArgs {}
unsafe impl Sync for AppointmentsProviderRemoveAppointmentActivatedEventArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AppointmentsProviderReplaceAppointmentActivatedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(AppointmentsProviderReplaceAppointmentActivatedEventArgs, windows_core::IUnknown, windows_core::IInspectable, IAppointmentsProviderReplaceAppointmentActivatedEventArgs);
windows_core::imp::required_hierarchy!(AppointmentsProviderReplaceAppointmentActivatedEventArgs, IActivatedEventArgs, IActivatedEventArgsWithUser, IAppointmentsProviderActivatedEventArgs);
impl AppointmentsProviderReplaceAppointmentActivatedEventArgs {
    pub fn Kind(&self) -> windows_core::Result<ActivationKind> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> windows_core::Result<ApplicationExecutionState> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PreviousExecutionState)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SplashScreen(&self) -> windows_core::Result<SplashScreen> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SplashScreen)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "System")]
    pub fn User(&self) -> windows_core::Result<super::super::System::User> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).User)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Verb(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IAppointmentsProviderActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Verb)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(feature = "ApplicationModel_Appointments_AppointmentsProvider")]
    pub fn ReplaceAppointmentOperation(&self) -> windows_core::Result<super::Appointments::AppointmentsProvider::ReplaceAppointmentOperation> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReplaceAppointmentOperation)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for AppointmentsProviderReplaceAppointmentActivatedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IAppointmentsProviderReplaceAppointmentActivatedEventArgs>();
}
unsafe impl windows_core::Interface for AppointmentsProviderReplaceAppointmentActivatedEventArgs {
    type Vtable = <IAppointmentsProviderReplaceAppointmentActivatedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IAppointmentsProviderReplaceAppointmentActivatedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for AppointmentsProviderReplaceAppointmentActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.AppointmentsProviderReplaceAppointmentActivatedEventArgs";
}
unsafe impl Send for AppointmentsProviderReplaceAppointmentActivatedEventArgs {}
unsafe impl Sync for AppointmentsProviderReplaceAppointmentActivatedEventArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AppointmentsProviderShowAppointmentDetailsActivatedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(AppointmentsProviderShowAppointmentDetailsActivatedEventArgs, windows_core::IUnknown, windows_core::IInspectable, IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs);
windows_core::imp::required_hierarchy!(AppointmentsProviderShowAppointmentDetailsActivatedEventArgs, IActivatedEventArgs, IActivatedEventArgsWithUser, IAppointmentsProviderActivatedEventArgs);
impl AppointmentsProviderShowAppointmentDetailsActivatedEventArgs {
    pub fn Kind(&self) -> windows_core::Result<ActivationKind> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> windows_core::Result<ApplicationExecutionState> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PreviousExecutionState)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SplashScreen(&self) -> windows_core::Result<SplashScreen> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SplashScreen)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "System")]
    pub fn User(&self) -> windows_core::Result<super::super::System::User> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).User)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Verb(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IAppointmentsProviderActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Verb)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn InstanceStartDate(&self) -> windows_core::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).InstanceStartDate)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn LocalId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LocalId)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn RoamingId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RoamingId)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
impl windows_core::RuntimeType for AppointmentsProviderShowAppointmentDetailsActivatedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs>();
}
unsafe impl windows_core::Interface for AppointmentsProviderShowAppointmentDetailsActivatedEventArgs {
    type Vtable = <IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for AppointmentsProviderShowAppointmentDetailsActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.AppointmentsProviderShowAppointmentDetailsActivatedEventArgs";
}
unsafe impl Send for AppointmentsProviderShowAppointmentDetailsActivatedEventArgs {}
unsafe impl Sync for AppointmentsProviderShowAppointmentDetailsActivatedEventArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AppointmentsProviderShowTimeFrameActivatedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(AppointmentsProviderShowTimeFrameActivatedEventArgs, windows_core::IUnknown, windows_core::IInspectable, IAppointmentsProviderShowTimeFrameActivatedEventArgs);
windows_core::imp::required_hierarchy!(AppointmentsProviderShowTimeFrameActivatedEventArgs, IActivatedEventArgs, IActivatedEventArgsWithUser, IAppointmentsProviderActivatedEventArgs);
impl AppointmentsProviderShowTimeFrameActivatedEventArgs {
    pub fn Kind(&self) -> windows_core::Result<ActivationKind> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> windows_core::Result<ApplicationExecutionState> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PreviousExecutionState)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SplashScreen(&self) -> windows_core::Result<SplashScreen> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SplashScreen)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "System")]
    pub fn User(&self) -> windows_core::Result<super::super::System::User> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).User)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Verb(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IAppointmentsProviderActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Verb)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn TimeToShow(&self) -> windows_core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TimeToShow)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Duration(&self) -> windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Duration)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for AppointmentsProviderShowTimeFrameActivatedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IAppointmentsProviderShowTimeFrameActivatedEventArgs>();
}
unsafe impl windows_core::Interface for AppointmentsProviderShowTimeFrameActivatedEventArgs {
    type Vtable = <IAppointmentsProviderShowTimeFrameActivatedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IAppointmentsProviderShowTimeFrameActivatedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for AppointmentsProviderShowTimeFrameActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.AppointmentsProviderShowTimeFrameActivatedEventArgs";
}
unsafe impl Send for AppointmentsProviderShowTimeFrameActivatedEventArgs {}
unsafe impl Sync for AppointmentsProviderShowTimeFrameActivatedEventArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BackgroundActivatedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(BackgroundActivatedEventArgs, windows_core::IUnknown, windows_core::IInspectable, IBackgroundActivatedEventArgs);
impl BackgroundActivatedEventArgs {
    #[cfg(feature = "ApplicationModel_Background")]
    pub fn TaskInstance(&self) -> windows_core::Result<super::Background::IBackgroundTaskInstance> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TaskInstance)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for BackgroundActivatedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IBackgroundActivatedEventArgs>();
}
unsafe impl windows_core::Interface for BackgroundActivatedEventArgs {
    type Vtable = <IBackgroundActivatedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IBackgroundActivatedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for BackgroundActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.BackgroundActivatedEventArgs";
}
unsafe impl Send for BackgroundActivatedEventArgs {}
unsafe impl Sync for BackgroundActivatedEventArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BarcodeScannerPreviewActivatedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(BarcodeScannerPreviewActivatedEventArgs, windows_core::IUnknown, windows_core::IInspectable, IBarcodeScannerPreviewActivatedEventArgs);
windows_core::imp::required_hierarchy!(BarcodeScannerPreviewActivatedEventArgs, IActivatedEventArgs, IActivatedEventArgsWithUser);
impl BarcodeScannerPreviewActivatedEventArgs {
    pub fn Kind(&self) -> windows_core::Result<ActivationKind> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> windows_core::Result<ApplicationExecutionState> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PreviousExecutionState)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SplashScreen(&self) -> windows_core::Result<SplashScreen> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SplashScreen)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "System")]
    pub fn User(&self) -> windows_core::Result<super::super::System::User> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).User)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ConnectionId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ConnectionId)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
impl windows_core::RuntimeType for BarcodeScannerPreviewActivatedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IBarcodeScannerPreviewActivatedEventArgs>();
}
unsafe impl windows_core::Interface for BarcodeScannerPreviewActivatedEventArgs {
    type Vtable = <IBarcodeScannerPreviewActivatedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IBarcodeScannerPreviewActivatedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for BarcodeScannerPreviewActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.BarcodeScannerPreviewActivatedEventArgs";
}
unsafe impl Send for BarcodeScannerPreviewActivatedEventArgs {}
unsafe impl Sync for BarcodeScannerPreviewActivatedEventArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CachedFileUpdaterActivatedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(CachedFileUpdaterActivatedEventArgs, windows_core::IUnknown, windows_core::IInspectable, ICachedFileUpdaterActivatedEventArgs);
windows_core::imp::required_hierarchy!(CachedFileUpdaterActivatedEventArgs, IActivatedEventArgs, IActivatedEventArgsWithUser);
impl CachedFileUpdaterActivatedEventArgs {
    pub fn Kind(&self) -> windows_core::Result<ActivationKind> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> windows_core::Result<ApplicationExecutionState> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PreviousExecutionState)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SplashScreen(&self) -> windows_core::Result<SplashScreen> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SplashScreen)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "System")]
    pub fn User(&self) -> windows_core::Result<super::super::System::User> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).User)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage_Provider")]
    pub fn CachedFileUpdaterUI(&self) -> windows_core::Result<super::super::Storage::Provider::CachedFileUpdaterUI> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CachedFileUpdaterUI)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for CachedFileUpdaterActivatedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ICachedFileUpdaterActivatedEventArgs>();
}
unsafe impl windows_core::Interface for CachedFileUpdaterActivatedEventArgs {
    type Vtable = <ICachedFileUpdaterActivatedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ICachedFileUpdaterActivatedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for CachedFileUpdaterActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.CachedFileUpdaterActivatedEventArgs";
}
unsafe impl Send for CachedFileUpdaterActivatedEventArgs {}
unsafe impl Sync for CachedFileUpdaterActivatedEventArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CameraSettingsActivatedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(CameraSettingsActivatedEventArgs, windows_core::IUnknown, windows_core::IInspectable, ICameraSettingsActivatedEventArgs);
windows_core::imp::required_hierarchy!(CameraSettingsActivatedEventArgs, IActivatedEventArgs);
impl CameraSettingsActivatedEventArgs {
    pub fn Kind(&self) -> windows_core::Result<ActivationKind> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> windows_core::Result<ApplicationExecutionState> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PreviousExecutionState)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SplashScreen(&self) -> windows_core::Result<SplashScreen> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SplashScreen)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn VideoDeviceController(&self) -> windows_core::Result<windows_core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).VideoDeviceController)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn VideoDeviceExtension(&self) -> windows_core::Result<windows_core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).VideoDeviceExtension)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for CameraSettingsActivatedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ICameraSettingsActivatedEventArgs>();
}
unsafe impl windows_core::Interface for CameraSettingsActivatedEventArgs {
    type Vtable = <ICameraSettingsActivatedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ICameraSettingsActivatedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for CameraSettingsActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.CameraSettingsActivatedEventArgs";
}
unsafe impl Send for CameraSettingsActivatedEventArgs {}
unsafe impl Sync for CameraSettingsActivatedEventArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CommandLineActivatedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(CommandLineActivatedEventArgs, windows_core::IUnknown, windows_core::IInspectable, ICommandLineActivatedEventArgs);
windows_core::imp::required_hierarchy!(CommandLineActivatedEventArgs, IActivatedEventArgs, IActivatedEventArgsWithUser);
impl CommandLineActivatedEventArgs {
    pub fn Kind(&self) -> windows_core::Result<ActivationKind> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> windows_core::Result<ApplicationExecutionState> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PreviousExecutionState)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SplashScreen(&self) -> windows_core::Result<SplashScreen> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SplashScreen)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "System")]
    pub fn User(&self) -> windows_core::Result<super::super::System::User> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).User)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Operation(&self) -> windows_core::Result<CommandLineActivationOperation> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Operation)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for CommandLineActivatedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ICommandLineActivatedEventArgs>();
}
unsafe impl windows_core::Interface for CommandLineActivatedEventArgs {
    type Vtable = <ICommandLineActivatedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ICommandLineActivatedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for CommandLineActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.CommandLineActivatedEventArgs";
}
unsafe impl Send for CommandLineActivatedEventArgs {}
unsafe impl Sync for CommandLineActivatedEventArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CommandLineActivationOperation(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(CommandLineActivationOperation, windows_core::IUnknown, windows_core::IInspectable);
impl CommandLineActivationOperation {
    pub fn Arguments(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Arguments)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn CurrentDirectoryPath(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CurrentDirectoryPath)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetExitCode(&self, value: i32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetExitCode)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ExitCode(&self) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ExitCode)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn GetDeferral(&self) -> windows_core::Result<super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDeferral)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for CommandLineActivationOperation {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ICommandLineActivationOperation>();
}
unsafe impl windows_core::Interface for CommandLineActivationOperation {
    type Vtable = <ICommandLineActivationOperation as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ICommandLineActivationOperation as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for CommandLineActivationOperation {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.CommandLineActivationOperation";
}
unsafe impl Send for CommandLineActivationOperation {}
unsafe impl Sync for CommandLineActivationOperation {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ContactCallActivatedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ContactCallActivatedEventArgs, windows_core::IUnknown, windows_core::IInspectable, IContactCallActivatedEventArgs);
windows_core::imp::required_hierarchy!(ContactCallActivatedEventArgs, IActivatedEventArgs, IContactActivatedEventArgs);
impl ContactCallActivatedEventArgs {
    pub fn Kind(&self) -> windows_core::Result<ActivationKind> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> windows_core::Result<ApplicationExecutionState> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PreviousExecutionState)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SplashScreen(&self) -> windows_core::Result<SplashScreen> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SplashScreen)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Verb(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IContactActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Verb)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn ServiceId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ServiceId)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn ServiceUserId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ServiceUserId)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(feature = "ApplicationModel_Contacts")]
    pub fn Contact(&self) -> windows_core::Result<super::Contacts::Contact> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Contact)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for ContactCallActivatedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IContactCallActivatedEventArgs>();
}
unsafe impl windows_core::Interface for ContactCallActivatedEventArgs {
    type Vtable = <IContactCallActivatedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IContactCallActivatedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ContactCallActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.ContactCallActivatedEventArgs";
}
unsafe impl Send for ContactCallActivatedEventArgs {}
unsafe impl Sync for ContactCallActivatedEventArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ContactMapActivatedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ContactMapActivatedEventArgs, windows_core::IUnknown, windows_core::IInspectable, IContactMapActivatedEventArgs);
windows_core::imp::required_hierarchy!(ContactMapActivatedEventArgs, IActivatedEventArgs, IContactActivatedEventArgs);
impl ContactMapActivatedEventArgs {
    pub fn Kind(&self) -> windows_core::Result<ActivationKind> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> windows_core::Result<ApplicationExecutionState> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PreviousExecutionState)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SplashScreen(&self) -> windows_core::Result<SplashScreen> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SplashScreen)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Verb(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IContactActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Verb)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(feature = "ApplicationModel_Contacts")]
    pub fn Address(&self) -> windows_core::Result<super::Contacts::ContactAddress> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Address)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "ApplicationModel_Contacts")]
    pub fn Contact(&self) -> windows_core::Result<super::Contacts::Contact> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Contact)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for ContactMapActivatedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IContactMapActivatedEventArgs>();
}
unsafe impl windows_core::Interface for ContactMapActivatedEventArgs {
    type Vtable = <IContactMapActivatedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IContactMapActivatedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ContactMapActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.ContactMapActivatedEventArgs";
}
unsafe impl Send for ContactMapActivatedEventArgs {}
unsafe impl Sync for ContactMapActivatedEventArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ContactMessageActivatedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ContactMessageActivatedEventArgs, windows_core::IUnknown, windows_core::IInspectable, IContactMessageActivatedEventArgs);
windows_core::imp::required_hierarchy!(ContactMessageActivatedEventArgs, IActivatedEventArgs, IContactActivatedEventArgs);
impl ContactMessageActivatedEventArgs {
    pub fn Kind(&self) -> windows_core::Result<ActivationKind> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> windows_core::Result<ApplicationExecutionState> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PreviousExecutionState)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SplashScreen(&self) -> windows_core::Result<SplashScreen> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SplashScreen)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Verb(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IContactActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Verb)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn ServiceId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ServiceId)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn ServiceUserId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ServiceUserId)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(feature = "ApplicationModel_Contacts")]
    pub fn Contact(&self) -> windows_core::Result<super::Contacts::Contact> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Contact)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for ContactMessageActivatedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IContactMessageActivatedEventArgs>();
}
unsafe impl windows_core::Interface for ContactMessageActivatedEventArgs {
    type Vtable = <IContactMessageActivatedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IContactMessageActivatedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ContactMessageActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.ContactMessageActivatedEventArgs";
}
unsafe impl Send for ContactMessageActivatedEventArgs {}
unsafe impl Sync for ContactMessageActivatedEventArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ContactPanelActivatedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ContactPanelActivatedEventArgs, windows_core::IUnknown, windows_core::IInspectable, IContactPanelActivatedEventArgs);
windows_core::imp::required_hierarchy!(ContactPanelActivatedEventArgs, IActivatedEventArgs, IActivatedEventArgsWithUser);
impl ContactPanelActivatedEventArgs {
    pub fn Kind(&self) -> windows_core::Result<ActivationKind> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> windows_core::Result<ApplicationExecutionState> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PreviousExecutionState)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SplashScreen(&self) -> windows_core::Result<SplashScreen> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SplashScreen)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "System")]
    pub fn User(&self) -> windows_core::Result<super::super::System::User> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).User)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "ApplicationModel_Contacts")]
    pub fn ContactPanel(&self) -> windows_core::Result<super::Contacts::ContactPanel> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ContactPanel)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "ApplicationModel_Contacts")]
    pub fn Contact(&self) -> windows_core::Result<super::Contacts::Contact> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Contact)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for ContactPanelActivatedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IContactPanelActivatedEventArgs>();
}
unsafe impl windows_core::Interface for ContactPanelActivatedEventArgs {
    type Vtable = <IContactPanelActivatedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IContactPanelActivatedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ContactPanelActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.ContactPanelActivatedEventArgs";
}
unsafe impl Send for ContactPanelActivatedEventArgs {}
unsafe impl Sync for ContactPanelActivatedEventArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ContactPickerActivatedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ContactPickerActivatedEventArgs, windows_core::IUnknown, windows_core::IInspectable, IContactPickerActivatedEventArgs);
windows_core::imp::required_hierarchy!(ContactPickerActivatedEventArgs, IActivatedEventArgs);
impl ContactPickerActivatedEventArgs {
    pub fn Kind(&self) -> windows_core::Result<ActivationKind> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> windows_core::Result<ApplicationExecutionState> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PreviousExecutionState)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SplashScreen(&self) -> windows_core::Result<SplashScreen> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SplashScreen)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "ApplicationModel_Contacts_Provider")]
    pub fn ContactPickerUI(&self) -> windows_core::Result<super::Contacts::Provider::ContactPickerUI> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ContactPickerUI)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for ContactPickerActivatedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IContactPickerActivatedEventArgs>();
}
unsafe impl windows_core::Interface for ContactPickerActivatedEventArgs {
    type Vtable = <IContactPickerActivatedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IContactPickerActivatedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ContactPickerActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.ContactPickerActivatedEventArgs";
}
unsafe impl Send for ContactPickerActivatedEventArgs {}
unsafe impl Sync for ContactPickerActivatedEventArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ContactPostActivatedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ContactPostActivatedEventArgs, windows_core::IUnknown, windows_core::IInspectable, IContactPostActivatedEventArgs);
windows_core::imp::required_hierarchy!(ContactPostActivatedEventArgs, IActivatedEventArgs, IContactActivatedEventArgs);
impl ContactPostActivatedEventArgs {
    pub fn Kind(&self) -> windows_core::Result<ActivationKind> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> windows_core::Result<ApplicationExecutionState> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PreviousExecutionState)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SplashScreen(&self) -> windows_core::Result<SplashScreen> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SplashScreen)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Verb(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IContactActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Verb)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn ServiceId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ServiceId)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn ServiceUserId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ServiceUserId)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(feature = "ApplicationModel_Contacts")]
    pub fn Contact(&self) -> windows_core::Result<super::Contacts::Contact> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Contact)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for ContactPostActivatedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IContactPostActivatedEventArgs>();
}
unsafe impl windows_core::Interface for ContactPostActivatedEventArgs {
    type Vtable = <IContactPostActivatedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IContactPostActivatedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ContactPostActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.ContactPostActivatedEventArgs";
}
unsafe impl Send for ContactPostActivatedEventArgs {}
unsafe impl Sync for ContactPostActivatedEventArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ContactVideoCallActivatedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ContactVideoCallActivatedEventArgs, windows_core::IUnknown, windows_core::IInspectable, IContactVideoCallActivatedEventArgs);
windows_core::imp::required_hierarchy!(ContactVideoCallActivatedEventArgs, IActivatedEventArgs, IContactActivatedEventArgs);
impl ContactVideoCallActivatedEventArgs {
    pub fn Kind(&self) -> windows_core::Result<ActivationKind> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> windows_core::Result<ApplicationExecutionState> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PreviousExecutionState)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SplashScreen(&self) -> windows_core::Result<SplashScreen> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SplashScreen)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Verb(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IContactActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Verb)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn ServiceId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ServiceId)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn ServiceUserId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ServiceUserId)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(feature = "ApplicationModel_Contacts")]
    pub fn Contact(&self) -> windows_core::Result<super::Contacts::Contact> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Contact)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for ContactVideoCallActivatedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IContactVideoCallActivatedEventArgs>();
}
unsafe impl windows_core::Interface for ContactVideoCallActivatedEventArgs {
    type Vtable = <IContactVideoCallActivatedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IContactVideoCallActivatedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ContactVideoCallActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.ContactVideoCallActivatedEventArgs";
}
unsafe impl Send for ContactVideoCallActivatedEventArgs {}
unsafe impl Sync for ContactVideoCallActivatedEventArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DeviceActivatedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(DeviceActivatedEventArgs, windows_core::IUnknown, windows_core::IInspectable, IDeviceActivatedEventArgs);
windows_core::imp::required_hierarchy!(DeviceActivatedEventArgs, IActivatedEventArgs, IActivatedEventArgsWithUser, IApplicationViewActivatedEventArgs, IViewSwitcherProvider);
impl DeviceActivatedEventArgs {
    pub fn Kind(&self) -> windows_core::Result<ActivationKind> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> windows_core::Result<ApplicationExecutionState> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PreviousExecutionState)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SplashScreen(&self) -> windows_core::Result<SplashScreen> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SplashScreen)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "System")]
    pub fn User(&self) -> windows_core::Result<super::super::System::User> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).User)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CurrentlyShownApplicationViewId(&self) -> windows_core::Result<i32> {
        let this = &windows_core::Interface::cast::<IApplicationViewActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CurrentlyShownApplicationViewId)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn DeviceInformationId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DeviceInformationId)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn Verb(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Verb)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(feature = "UI_ViewManagement")]
    pub fn ViewSwitcher(&self) -> windows_core::Result<super::super::UI::ViewManagement::ActivationViewSwitcher> {
        let this = &windows_core::Interface::cast::<IViewSwitcherProvider>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ViewSwitcher)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for DeviceActivatedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IDeviceActivatedEventArgs>();
}
unsafe impl windows_core::Interface for DeviceActivatedEventArgs {
    type Vtable = <IDeviceActivatedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IDeviceActivatedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for DeviceActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.DeviceActivatedEventArgs";
}
unsafe impl Send for DeviceActivatedEventArgs {}
unsafe impl Sync for DeviceActivatedEventArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DevicePairingActivatedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(DevicePairingActivatedEventArgs, windows_core::IUnknown, windows_core::IInspectable, IDevicePairingActivatedEventArgs);
windows_core::imp::required_hierarchy!(DevicePairingActivatedEventArgs, IActivatedEventArgs, IActivatedEventArgsWithUser);
impl DevicePairingActivatedEventArgs {
    pub fn Kind(&self) -> windows_core::Result<ActivationKind> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> windows_core::Result<ApplicationExecutionState> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PreviousExecutionState)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SplashScreen(&self) -> windows_core::Result<SplashScreen> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SplashScreen)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "System")]
    pub fn User(&self) -> windows_core::Result<super::super::System::User> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).User)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Devices_Enumeration")]
    pub fn DeviceInformation(&self) -> windows_core::Result<super::super::Devices::Enumeration::DeviceInformation> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DeviceInformation)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for DevicePairingActivatedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IDevicePairingActivatedEventArgs>();
}
unsafe impl windows_core::Interface for DevicePairingActivatedEventArgs {
    type Vtable = <IDevicePairingActivatedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IDevicePairingActivatedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for DevicePairingActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.DevicePairingActivatedEventArgs";
}
unsafe impl Send for DevicePairingActivatedEventArgs {}
unsafe impl Sync for DevicePairingActivatedEventArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DialReceiverActivatedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(DialReceiverActivatedEventArgs, windows_core::IUnknown, windows_core::IInspectable, IDialReceiverActivatedEventArgs);
windows_core::imp::required_hierarchy!(DialReceiverActivatedEventArgs, IActivatedEventArgs, IActivatedEventArgsWithUser, IApplicationViewActivatedEventArgs, ILaunchActivatedEventArgs, IViewSwitcherProvider);
impl DialReceiverActivatedEventArgs {
    pub fn Kind(&self) -> windows_core::Result<ActivationKind> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> windows_core::Result<ApplicationExecutionState> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PreviousExecutionState)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SplashScreen(&self) -> windows_core::Result<SplashScreen> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SplashScreen)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "System")]
    pub fn User(&self) -> windows_core::Result<super::super::System::User> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).User)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CurrentlyShownApplicationViewId(&self) -> windows_core::Result<i32> {
        let this = &windows_core::Interface::cast::<IApplicationViewActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CurrentlyShownApplicationViewId)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn AppName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AppName)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn Arguments(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<ILaunchActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Arguments)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn TileId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<ILaunchActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TileId)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(feature = "UI_ViewManagement")]
    pub fn ViewSwitcher(&self) -> windows_core::Result<super::super::UI::ViewManagement::ActivationViewSwitcher> {
        let this = &windows_core::Interface::cast::<IViewSwitcherProvider>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ViewSwitcher)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for DialReceiverActivatedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IDialReceiverActivatedEventArgs>();
}
unsafe impl windows_core::Interface for DialReceiverActivatedEventArgs {
    type Vtable = <IDialReceiverActivatedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IDialReceiverActivatedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for DialReceiverActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.DialReceiverActivatedEventArgs";
}
unsafe impl Send for DialReceiverActivatedEventArgs {}
unsafe impl Sync for DialReceiverActivatedEventArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FileActivatedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(FileActivatedEventArgs, windows_core::IUnknown, windows_core::IInspectable, IFileActivatedEventArgs);
windows_core::imp::required_hierarchy!(FileActivatedEventArgs, IActivatedEventArgs, IActivatedEventArgsWithUser, IApplicationViewActivatedEventArgs, IFileActivatedEventArgsWithCallerPackageFamilyName, IFileActivatedEventArgsWithNeighboringFiles, IViewSwitcherProvider);
impl FileActivatedEventArgs {
    pub fn Kind(&self) -> windows_core::Result<ActivationKind> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> windows_core::Result<ApplicationExecutionState> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PreviousExecutionState)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SplashScreen(&self) -> windows_core::Result<SplashScreen> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SplashScreen)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "System")]
    pub fn User(&self) -> windows_core::Result<super::super::System::User> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).User)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CurrentlyShownApplicationViewId(&self) -> windows_core::Result<i32> {
        let this = &windows_core::Interface::cast::<IApplicationViewActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CurrentlyShownApplicationViewId)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Storage")]
    pub fn Files(&self) -> windows_core::Result<windows_collections::IVectorView<super::super::Storage::IStorageItem>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Files)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Verb(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Verb)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn CallerPackageFamilyName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IFileActivatedEventArgsWithCallerPackageFamilyName>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CallerPackageFamilyName)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(feature = "Storage_Search")]
    pub fn NeighboringFilesQuery(&self) -> windows_core::Result<super::super::Storage::Search::StorageFileQueryResult> {
        let this = &windows_core::Interface::cast::<IFileActivatedEventArgsWithNeighboringFiles>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NeighboringFilesQuery)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "UI_ViewManagement")]
    pub fn ViewSwitcher(&self) -> windows_core::Result<super::super::UI::ViewManagement::ActivationViewSwitcher> {
        let this = &windows_core::Interface::cast::<IViewSwitcherProvider>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ViewSwitcher)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for FileActivatedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IFileActivatedEventArgs>();
}
unsafe impl windows_core::Interface for FileActivatedEventArgs {
    type Vtable = <IFileActivatedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IFileActivatedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for FileActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.FileActivatedEventArgs";
}
unsafe impl Send for FileActivatedEventArgs {}
unsafe impl Sync for FileActivatedEventArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FileOpenPickerActivatedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(FileOpenPickerActivatedEventArgs, windows_core::IUnknown, windows_core::IInspectable, IFileOpenPickerActivatedEventArgs);
windows_core::imp::required_hierarchy!(FileOpenPickerActivatedEventArgs, IActivatedEventArgs, IActivatedEventArgsWithUser, IFileOpenPickerActivatedEventArgs2);
impl FileOpenPickerActivatedEventArgs {
    pub fn Kind(&self) -> windows_core::Result<ActivationKind> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> windows_core::Result<ApplicationExecutionState> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PreviousExecutionState)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SplashScreen(&self) -> windows_core::Result<SplashScreen> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SplashScreen)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "System")]
    pub fn User(&self) -> windows_core::Result<super::super::System::User> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).User)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage_Pickers_Provider")]
    pub fn FileOpenPickerUI(&self) -> windows_core::Result<super::super::Storage::Pickers::Provider::FileOpenPickerUI> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FileOpenPickerUI)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CallerPackageFamilyName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IFileOpenPickerActivatedEventArgs2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CallerPackageFamilyName)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
impl windows_core::RuntimeType for FileOpenPickerActivatedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IFileOpenPickerActivatedEventArgs>();
}
unsafe impl windows_core::Interface for FileOpenPickerActivatedEventArgs {
    type Vtable = <IFileOpenPickerActivatedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IFileOpenPickerActivatedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for FileOpenPickerActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.FileOpenPickerActivatedEventArgs";
}
unsafe impl Send for FileOpenPickerActivatedEventArgs {}
unsafe impl Sync for FileOpenPickerActivatedEventArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FileOpenPickerContinuationEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(FileOpenPickerContinuationEventArgs, windows_core::IUnknown, windows_core::IInspectable, IFileOpenPickerContinuationEventArgs);
windows_core::imp::required_hierarchy!(FileOpenPickerContinuationEventArgs, IActivatedEventArgs, IActivatedEventArgsWithUser, IContinuationActivatedEventArgs);
impl FileOpenPickerContinuationEventArgs {
    pub fn Kind(&self) -> windows_core::Result<ActivationKind> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> windows_core::Result<ApplicationExecutionState> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PreviousExecutionState)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SplashScreen(&self) -> windows_core::Result<SplashScreen> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SplashScreen)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "System")]
    pub fn User(&self) -> windows_core::Result<super::super::System::User> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).User)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn ContinuationData(&self) -> windows_core::Result<super::super::Foundation::Collections::ValueSet> {
        let this = &windows_core::Interface::cast::<IContinuationActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ContinuationData)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn Files(&self) -> windows_core::Result<windows_collections::IVectorView<super::super::Storage::StorageFile>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Files)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for FileOpenPickerContinuationEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IFileOpenPickerContinuationEventArgs>();
}
unsafe impl windows_core::Interface for FileOpenPickerContinuationEventArgs {
    type Vtable = <IFileOpenPickerContinuationEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IFileOpenPickerContinuationEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for FileOpenPickerContinuationEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.FileOpenPickerContinuationEventArgs";
}
unsafe impl Send for FileOpenPickerContinuationEventArgs {}
unsafe impl Sync for FileOpenPickerContinuationEventArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FileSavePickerActivatedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(FileSavePickerActivatedEventArgs, windows_core::IUnknown, windows_core::IInspectable, IFileSavePickerActivatedEventArgs);
windows_core::imp::required_hierarchy!(FileSavePickerActivatedEventArgs, IActivatedEventArgs, IActivatedEventArgsWithUser, IFileSavePickerActivatedEventArgs2);
impl FileSavePickerActivatedEventArgs {
    pub fn Kind(&self) -> windows_core::Result<ActivationKind> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> windows_core::Result<ApplicationExecutionState> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PreviousExecutionState)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SplashScreen(&self) -> windows_core::Result<SplashScreen> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SplashScreen)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "System")]
    pub fn User(&self) -> windows_core::Result<super::super::System::User> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).User)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage_Pickers_Provider")]
    pub fn FileSavePickerUI(&self) -> windows_core::Result<super::super::Storage::Pickers::Provider::FileSavePickerUI> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FileSavePickerUI)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CallerPackageFamilyName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IFileSavePickerActivatedEventArgs2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CallerPackageFamilyName)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn EnterpriseId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IFileSavePickerActivatedEventArgs2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).EnterpriseId)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
impl windows_core::RuntimeType for FileSavePickerActivatedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IFileSavePickerActivatedEventArgs>();
}
unsafe impl windows_core::Interface for FileSavePickerActivatedEventArgs {
    type Vtable = <IFileSavePickerActivatedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IFileSavePickerActivatedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for FileSavePickerActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.FileSavePickerActivatedEventArgs";
}
unsafe impl Send for FileSavePickerActivatedEventArgs {}
unsafe impl Sync for FileSavePickerActivatedEventArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FileSavePickerContinuationEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(FileSavePickerContinuationEventArgs, windows_core::IUnknown, windows_core::IInspectable, IFileSavePickerContinuationEventArgs);
windows_core::imp::required_hierarchy!(FileSavePickerContinuationEventArgs, IActivatedEventArgs, IActivatedEventArgsWithUser, IContinuationActivatedEventArgs);
impl FileSavePickerContinuationEventArgs {
    pub fn Kind(&self) -> windows_core::Result<ActivationKind> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> windows_core::Result<ApplicationExecutionState> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PreviousExecutionState)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SplashScreen(&self) -> windows_core::Result<SplashScreen> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SplashScreen)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "System")]
    pub fn User(&self) -> windows_core::Result<super::super::System::User> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).User)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn ContinuationData(&self) -> windows_core::Result<super::super::Foundation::Collections::ValueSet> {
        let this = &windows_core::Interface::cast::<IContinuationActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ContinuationData)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn File(&self) -> windows_core::Result<super::super::Storage::StorageFile> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).File)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for FileSavePickerContinuationEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IFileSavePickerContinuationEventArgs>();
}
unsafe impl windows_core::Interface for FileSavePickerContinuationEventArgs {
    type Vtable = <IFileSavePickerContinuationEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IFileSavePickerContinuationEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for FileSavePickerContinuationEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.FileSavePickerContinuationEventArgs";
}
unsafe impl Send for FileSavePickerContinuationEventArgs {}
unsafe impl Sync for FileSavePickerContinuationEventArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FolderPickerContinuationEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(FolderPickerContinuationEventArgs, windows_core::IUnknown, windows_core::IInspectable, IFolderPickerContinuationEventArgs);
windows_core::imp::required_hierarchy!(FolderPickerContinuationEventArgs, IActivatedEventArgs, IActivatedEventArgsWithUser, IContinuationActivatedEventArgs);
impl FolderPickerContinuationEventArgs {
    pub fn Kind(&self) -> windows_core::Result<ActivationKind> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> windows_core::Result<ApplicationExecutionState> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PreviousExecutionState)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SplashScreen(&self) -> windows_core::Result<SplashScreen> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SplashScreen)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "System")]
    pub fn User(&self) -> windows_core::Result<super::super::System::User> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).User)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn ContinuationData(&self) -> windows_core::Result<super::super::Foundation::Collections::ValueSet> {
        let this = &windows_core::Interface::cast::<IContinuationActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ContinuationData)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage_Search")]
    pub fn Folder(&self) -> windows_core::Result<super::super::Storage::StorageFolder> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Folder)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for FolderPickerContinuationEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IFolderPickerContinuationEventArgs>();
}
unsafe impl windows_core::Interface for FolderPickerContinuationEventArgs {
    type Vtable = <IFolderPickerContinuationEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IFolderPickerContinuationEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for FolderPickerContinuationEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.FolderPickerContinuationEventArgs";
}
unsafe impl Send for FolderPickerContinuationEventArgs {}
unsafe impl Sync for FolderPickerContinuationEventArgs {}
windows_core::imp::define_interface!(IActivatedEventArgs, IActivatedEventArgs_Vtbl, 0xcf651713_cd08_4fd8_b697_a281b6544e2e);
impl windows_core::RuntimeType for IActivatedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(IActivatedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl IActivatedEventArgs {
    pub fn Kind(&self) -> windows_core::Result<ActivationKind> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> windows_core::Result<ApplicationExecutionState> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PreviousExecutionState)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SplashScreen(&self) -> windows_core::Result<SplashScreen> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SplashScreen)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeName for IActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IActivatedEventArgs";
}
pub trait IActivatedEventArgs_Impl: windows_core::IUnknownImpl {
    fn Kind(&self) -> windows_core::Result<ActivationKind>;
    fn PreviousExecutionState(&self) -> windows_core::Result<ApplicationExecutionState>;
    fn SplashScreen(&self) -> windows_core::Result<SplashScreen>;
}
impl IActivatedEventArgs_Vtbl {
    pub const fn new<Identity: IActivatedEventArgs_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Kind<Identity: IActivatedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut ActivationKind) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IActivatedEventArgs_Impl::Kind(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn PreviousExecutionState<Identity: IActivatedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut ApplicationExecutionState) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IActivatedEventArgs_Impl::PreviousExecutionState(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SplashScreen<Identity: IActivatedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IActivatedEventArgs_Impl::SplashScreen(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IActivatedEventArgs, OFFSET>(),
            Kind: Kind::<Identity, OFFSET>,
            PreviousExecutionState: PreviousExecutionState::<Identity, OFFSET>,
            SplashScreen: SplashScreen::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IActivatedEventArgs as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IActivatedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Kind: unsafe extern "system" fn(*mut core::ffi::c_void, *mut ActivationKind) -> windows_core::HRESULT,
    pub PreviousExecutionState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut ApplicationExecutionState) -> windows_core::HRESULT,
    pub SplashScreen: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IActivatedEventArgsWithUser, IActivatedEventArgsWithUser_Vtbl, 0x1cf09b9e_9962_4936_80ff_afc8e8ae5c8c);
impl windows_core::RuntimeType for IActivatedEventArgsWithUser {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(IActivatedEventArgsWithUser, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(IActivatedEventArgsWithUser, IActivatedEventArgs);
impl IActivatedEventArgsWithUser {
    #[cfg(feature = "System")]
    pub fn User(&self) -> windows_core::Result<super::super::System::User> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).User)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Kind(&self) -> windows_core::Result<ActivationKind> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> windows_core::Result<ApplicationExecutionState> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PreviousExecutionState)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SplashScreen(&self) -> windows_core::Result<SplashScreen> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SplashScreen)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "System")]
impl windows_core::RuntimeName for IActivatedEventArgsWithUser {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IActivatedEventArgsWithUser";
}
#[cfg(feature = "System")]
pub trait IActivatedEventArgsWithUser_Impl: IActivatedEventArgs_Impl {
    fn User(&self) -> windows_core::Result<super::super::System::User>;
}
#[cfg(feature = "System")]
impl IActivatedEventArgsWithUser_Vtbl {
    pub const fn new<Identity: IActivatedEventArgsWithUser_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn User<Identity: IActivatedEventArgsWithUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IActivatedEventArgsWithUser_Impl::User(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IInspectable_Vtbl::new::<Identity, IActivatedEventArgsWithUser, OFFSET>(), User: User::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IActivatedEventArgsWithUser as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IActivatedEventArgsWithUser_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "System")]
    pub User: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    User: usize,
}
windows_core::imp::define_interface!(IApplicationViewActivatedEventArgs, IApplicationViewActivatedEventArgs_Vtbl, 0x930cef4b_b829_40fc_88f4_8513e8a64738);
impl windows_core::RuntimeType for IApplicationViewActivatedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(IApplicationViewActivatedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(IApplicationViewActivatedEventArgs, IActivatedEventArgs);
impl IApplicationViewActivatedEventArgs {
    pub fn CurrentlyShownApplicationViewId(&self) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CurrentlyShownApplicationViewId)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Kind(&self) -> windows_core::Result<ActivationKind> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> windows_core::Result<ApplicationExecutionState> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PreviousExecutionState)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SplashScreen(&self) -> windows_core::Result<SplashScreen> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SplashScreen)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeName for IApplicationViewActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IApplicationViewActivatedEventArgs";
}
pub trait IApplicationViewActivatedEventArgs_Impl: IActivatedEventArgs_Impl {
    fn CurrentlyShownApplicationViewId(&self) -> windows_core::Result<i32>;
}
impl IApplicationViewActivatedEventArgs_Vtbl {
    pub const fn new<Identity: IApplicationViewActivatedEventArgs_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CurrentlyShownApplicationViewId<Identity: IApplicationViewActivatedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IApplicationViewActivatedEventArgs_Impl::CurrentlyShownApplicationViewId(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IApplicationViewActivatedEventArgs, OFFSET>(),
            CurrentlyShownApplicationViewId: CurrentlyShownApplicationViewId::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IApplicationViewActivatedEventArgs as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IApplicationViewActivatedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CurrentlyShownApplicationViewId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAppointmentsProviderActivatedEventArgs, IAppointmentsProviderActivatedEventArgs_Vtbl, 0x3364c405_933c_4e7d_a034_500fb8dcd9f3);
impl windows_core::RuntimeType for IAppointmentsProviderActivatedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(IAppointmentsProviderActivatedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(IAppointmentsProviderActivatedEventArgs, IActivatedEventArgs);
impl IAppointmentsProviderActivatedEventArgs {
    pub fn Verb(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Verb)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn Kind(&self) -> windows_core::Result<ActivationKind> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> windows_core::Result<ApplicationExecutionState> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PreviousExecutionState)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SplashScreen(&self) -> windows_core::Result<SplashScreen> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SplashScreen)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeName for IAppointmentsProviderActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IAppointmentsProviderActivatedEventArgs";
}
pub trait IAppointmentsProviderActivatedEventArgs_Impl: IActivatedEventArgs_Impl {
    fn Verb(&self) -> windows_core::Result<windows_core::HSTRING>;
}
impl IAppointmentsProviderActivatedEventArgs_Vtbl {
    pub const fn new<Identity: IAppointmentsProviderActivatedEventArgs_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Verb<Identity: IAppointmentsProviderActivatedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAppointmentsProviderActivatedEventArgs_Impl::Verb(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IInspectable_Vtbl::new::<Identity, IAppointmentsProviderActivatedEventArgs, OFFSET>(), Verb: Verb::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAppointmentsProviderActivatedEventArgs as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppointmentsProviderActivatedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Verb: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAppointmentsProviderAddAppointmentActivatedEventArgs, IAppointmentsProviderAddAppointmentActivatedEventArgs_Vtbl, 0xa2861367_cee5_4e4d_9ed7_41c34ec18b02);
impl windows_core::RuntimeType for IAppointmentsProviderAddAppointmentActivatedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(IAppointmentsProviderAddAppointmentActivatedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(IAppointmentsProviderAddAppointmentActivatedEventArgs, IActivatedEventArgs, IAppointmentsProviderActivatedEventArgs);
impl IAppointmentsProviderAddAppointmentActivatedEventArgs {
    #[cfg(feature = "ApplicationModel_Appointments_AppointmentsProvider")]
    pub fn AddAppointmentOperation(&self) -> windows_core::Result<super::Appointments::AppointmentsProvider::AddAppointmentOperation> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AddAppointmentOperation)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Kind(&self) -> windows_core::Result<ActivationKind> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> windows_core::Result<ApplicationExecutionState> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PreviousExecutionState)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SplashScreen(&self) -> windows_core::Result<SplashScreen> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SplashScreen)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Verb(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IAppointmentsProviderActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Verb)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[cfg(feature = "ApplicationModel_Appointments_AppointmentsProvider")]
impl windows_core::RuntimeName for IAppointmentsProviderAddAppointmentActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IAppointmentsProviderAddAppointmentActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Appointments_AppointmentsProvider")]
pub trait IAppointmentsProviderAddAppointmentActivatedEventArgs_Impl: IActivatedEventArgs_Impl + IAppointmentsProviderActivatedEventArgs_Impl {
    fn AddAppointmentOperation(&self) -> windows_core::Result<super::Appointments::AppointmentsProvider::AddAppointmentOperation>;
}
#[cfg(feature = "ApplicationModel_Appointments_AppointmentsProvider")]
impl IAppointmentsProviderAddAppointmentActivatedEventArgs_Vtbl {
    pub const fn new<Identity: IAppointmentsProviderAddAppointmentActivatedEventArgs_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AddAppointmentOperation<Identity: IAppointmentsProviderAddAppointmentActivatedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAppointmentsProviderAddAppointmentActivatedEventArgs_Impl::AddAppointmentOperation(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IAppointmentsProviderAddAppointmentActivatedEventArgs, OFFSET>(),
            AddAppointmentOperation: AddAppointmentOperation::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAppointmentsProviderAddAppointmentActivatedEventArgs as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppointmentsProviderAddAppointmentActivatedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "ApplicationModel_Appointments_AppointmentsProvider")]
    pub AddAppointmentOperation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Appointments_AppointmentsProvider"))]
    AddAppointmentOperation: usize,
}
windows_core::imp::define_interface!(IAppointmentsProviderRemoveAppointmentActivatedEventArgs, IAppointmentsProviderRemoveAppointmentActivatedEventArgs_Vtbl, 0x751f3ab8_0b8e_451c_9f15_966e699bac25);
impl windows_core::RuntimeType for IAppointmentsProviderRemoveAppointmentActivatedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(IAppointmentsProviderRemoveAppointmentActivatedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(IAppointmentsProviderRemoveAppointmentActivatedEventArgs, IActivatedEventArgs, IAppointmentsProviderActivatedEventArgs);
impl IAppointmentsProviderRemoveAppointmentActivatedEventArgs {
    #[cfg(feature = "ApplicationModel_Appointments_AppointmentsProvider")]
    pub fn RemoveAppointmentOperation(&self) -> windows_core::Result<super::Appointments::AppointmentsProvider::RemoveAppointmentOperation> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RemoveAppointmentOperation)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Kind(&self) -> windows_core::Result<ActivationKind> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> windows_core::Result<ApplicationExecutionState> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PreviousExecutionState)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SplashScreen(&self) -> windows_core::Result<SplashScreen> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SplashScreen)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Verb(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IAppointmentsProviderActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Verb)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[cfg(feature = "ApplicationModel_Appointments_AppointmentsProvider")]
impl windows_core::RuntimeName for IAppointmentsProviderRemoveAppointmentActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IAppointmentsProviderRemoveAppointmentActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Appointments_AppointmentsProvider")]
pub trait IAppointmentsProviderRemoveAppointmentActivatedEventArgs_Impl: IActivatedEventArgs_Impl + IAppointmentsProviderActivatedEventArgs_Impl {
    fn RemoveAppointmentOperation(&self) -> windows_core::Result<super::Appointments::AppointmentsProvider::RemoveAppointmentOperation>;
}
#[cfg(feature = "ApplicationModel_Appointments_AppointmentsProvider")]
impl IAppointmentsProviderRemoveAppointmentActivatedEventArgs_Vtbl {
    pub const fn new<Identity: IAppointmentsProviderRemoveAppointmentActivatedEventArgs_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn RemoveAppointmentOperation<Identity: IAppointmentsProviderRemoveAppointmentActivatedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAppointmentsProviderRemoveAppointmentActivatedEventArgs_Impl::RemoveAppointmentOperation(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IAppointmentsProviderRemoveAppointmentActivatedEventArgs, OFFSET>(),
            RemoveAppointmentOperation: RemoveAppointmentOperation::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAppointmentsProviderRemoveAppointmentActivatedEventArgs as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppointmentsProviderRemoveAppointmentActivatedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "ApplicationModel_Appointments_AppointmentsProvider")]
    pub RemoveAppointmentOperation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Appointments_AppointmentsProvider"))]
    RemoveAppointmentOperation: usize,
}
windows_core::imp::define_interface!(IAppointmentsProviderReplaceAppointmentActivatedEventArgs, IAppointmentsProviderReplaceAppointmentActivatedEventArgs_Vtbl, 0x1551b7d4_a981_4067_8a62_0524e4ade121);
impl windows_core::RuntimeType for IAppointmentsProviderReplaceAppointmentActivatedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(IAppointmentsProviderReplaceAppointmentActivatedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(IAppointmentsProviderReplaceAppointmentActivatedEventArgs, IActivatedEventArgs, IAppointmentsProviderActivatedEventArgs);
impl IAppointmentsProviderReplaceAppointmentActivatedEventArgs {
    #[cfg(feature = "ApplicationModel_Appointments_AppointmentsProvider")]
    pub fn ReplaceAppointmentOperation(&self) -> windows_core::Result<super::Appointments::AppointmentsProvider::ReplaceAppointmentOperation> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReplaceAppointmentOperation)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Kind(&self) -> windows_core::Result<ActivationKind> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> windows_core::Result<ApplicationExecutionState> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PreviousExecutionState)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SplashScreen(&self) -> windows_core::Result<SplashScreen> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SplashScreen)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Verb(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IAppointmentsProviderActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Verb)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[cfg(feature = "ApplicationModel_Appointments_AppointmentsProvider")]
impl windows_core::RuntimeName for IAppointmentsProviderReplaceAppointmentActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IAppointmentsProviderReplaceAppointmentActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Appointments_AppointmentsProvider")]
pub trait IAppointmentsProviderReplaceAppointmentActivatedEventArgs_Impl: IActivatedEventArgs_Impl + IAppointmentsProviderActivatedEventArgs_Impl {
    fn ReplaceAppointmentOperation(&self) -> windows_core::Result<super::Appointments::AppointmentsProvider::ReplaceAppointmentOperation>;
}
#[cfg(feature = "ApplicationModel_Appointments_AppointmentsProvider")]
impl IAppointmentsProviderReplaceAppointmentActivatedEventArgs_Vtbl {
    pub const fn new<Identity: IAppointmentsProviderReplaceAppointmentActivatedEventArgs_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ReplaceAppointmentOperation<Identity: IAppointmentsProviderReplaceAppointmentActivatedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAppointmentsProviderReplaceAppointmentActivatedEventArgs_Impl::ReplaceAppointmentOperation(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IAppointmentsProviderReplaceAppointmentActivatedEventArgs, OFFSET>(),
            ReplaceAppointmentOperation: ReplaceAppointmentOperation::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAppointmentsProviderReplaceAppointmentActivatedEventArgs as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppointmentsProviderReplaceAppointmentActivatedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "ApplicationModel_Appointments_AppointmentsProvider")]
    pub ReplaceAppointmentOperation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Appointments_AppointmentsProvider"))]
    ReplaceAppointmentOperation: usize,
}
windows_core::imp::define_interface!(IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs, IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs_Vtbl, 0x3958f065_9841_4ca5_999b_885198b9ef2a);
impl windows_core::RuntimeType for IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs, IActivatedEventArgs, IAppointmentsProviderActivatedEventArgs);
impl IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs {
    pub fn InstanceStartDate(&self) -> windows_core::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).InstanceStartDate)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn LocalId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LocalId)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn RoamingId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RoamingId)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn Kind(&self) -> windows_core::Result<ActivationKind> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> windows_core::Result<ApplicationExecutionState> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PreviousExecutionState)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SplashScreen(&self) -> windows_core::Result<SplashScreen> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SplashScreen)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Verb(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IAppointmentsProviderActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Verb)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
impl windows_core::RuntimeName for IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs";
}
pub trait IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs_Impl: IActivatedEventArgs_Impl + IAppointmentsProviderActivatedEventArgs_Impl {
    fn InstanceStartDate(&self) -> windows_core::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>>;
    fn LocalId(&self) -> windows_core::Result<windows_core::HSTRING>;
    fn RoamingId(&self) -> windows_core::Result<windows_core::HSTRING>;
}
impl IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs_Vtbl {
    pub const fn new<Identity: IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn InstanceStartDate<Identity: IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs_Impl::InstanceStartDate(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn LocalId<Identity: IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs_Impl::LocalId(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RoamingId<Identity: IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs_Impl::RoamingId(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs, OFFSET>(),
            InstanceStartDate: InstanceStartDate::<Identity, OFFSET>,
            LocalId: LocalId::<Identity, OFFSET>,
            RoamingId: RoamingId::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub InstanceStartDate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub LocalId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RoamingId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAppointmentsProviderShowTimeFrameActivatedEventArgs, IAppointmentsProviderShowTimeFrameActivatedEventArgs_Vtbl, 0x9baeaba6_0e0b_49aa_babc_12b1dc774986);
impl windows_core::RuntimeType for IAppointmentsProviderShowTimeFrameActivatedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(IAppointmentsProviderShowTimeFrameActivatedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(IAppointmentsProviderShowTimeFrameActivatedEventArgs, IActivatedEventArgs, IAppointmentsProviderActivatedEventArgs);
impl IAppointmentsProviderShowTimeFrameActivatedEventArgs {
    pub fn TimeToShow(&self) -> windows_core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TimeToShow)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Duration(&self) -> windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Duration)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Kind(&self) -> windows_core::Result<ActivationKind> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> windows_core::Result<ApplicationExecutionState> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PreviousExecutionState)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SplashScreen(&self) -> windows_core::Result<SplashScreen> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SplashScreen)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Verb(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IAppointmentsProviderActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Verb)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
impl windows_core::RuntimeName for IAppointmentsProviderShowTimeFrameActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IAppointmentsProviderShowTimeFrameActivatedEventArgs";
}
pub trait IAppointmentsProviderShowTimeFrameActivatedEventArgs_Impl: IActivatedEventArgs_Impl + IAppointmentsProviderActivatedEventArgs_Impl {
    fn TimeToShow(&self) -> windows_core::Result<super::super::Foundation::DateTime>;
    fn Duration(&self) -> windows_core::Result<super::super::Foundation::TimeSpan>;
}
impl IAppointmentsProviderShowTimeFrameActivatedEventArgs_Vtbl {
    pub const fn new<Identity: IAppointmentsProviderShowTimeFrameActivatedEventArgs_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn TimeToShow<Identity: IAppointmentsProviderShowTimeFrameActivatedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAppointmentsProviderShowTimeFrameActivatedEventArgs_Impl::TimeToShow(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Duration<Identity: IAppointmentsProviderShowTimeFrameActivatedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAppointmentsProviderShowTimeFrameActivatedEventArgs_Impl::Duration(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IAppointmentsProviderShowTimeFrameActivatedEventArgs, OFFSET>(),
            TimeToShow: TimeToShow::<Identity, OFFSET>,
            Duration: Duration::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAppointmentsProviderShowTimeFrameActivatedEventArgs as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppointmentsProviderShowTimeFrameActivatedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub TimeToShow: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::DateTime) -> windows_core::HRESULT,
    pub Duration: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::TimeSpan) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IBackgroundActivatedEventArgs, IBackgroundActivatedEventArgs_Vtbl, 0xab14bee0_e760_440e_a91c_44796de3a92d);
impl windows_core::RuntimeType for IBackgroundActivatedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(IBackgroundActivatedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl IBackgroundActivatedEventArgs {
    #[cfg(feature = "ApplicationModel_Background")]
    pub fn TaskInstance(&self) -> windows_core::Result<super::Background::IBackgroundTaskInstance> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TaskInstance)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "ApplicationModel_Background")]
impl windows_core::RuntimeName for IBackgroundActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IBackgroundActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Background")]
pub trait IBackgroundActivatedEventArgs_Impl: windows_core::IUnknownImpl {
    fn TaskInstance(&self) -> windows_core::Result<super::Background::IBackgroundTaskInstance>;
}
#[cfg(feature = "ApplicationModel_Background")]
impl IBackgroundActivatedEventArgs_Vtbl {
    pub const fn new<Identity: IBackgroundActivatedEventArgs_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn TaskInstance<Identity: IBackgroundActivatedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IBackgroundActivatedEventArgs_Impl::TaskInstance(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IBackgroundActivatedEventArgs, OFFSET>(),
            TaskInstance: TaskInstance::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IBackgroundActivatedEventArgs as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundActivatedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "ApplicationModel_Background")]
    pub TaskInstance: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Background"))]
    TaskInstance: usize,
}
windows_core::imp::define_interface!(IBarcodeScannerPreviewActivatedEventArgs, IBarcodeScannerPreviewActivatedEventArgs_Vtbl, 0x6772797c_99bf_4349_af22_e4123560371c);
impl windows_core::RuntimeType for IBarcodeScannerPreviewActivatedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(IBarcodeScannerPreviewActivatedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(IBarcodeScannerPreviewActivatedEventArgs, IActivatedEventArgs);
impl IBarcodeScannerPreviewActivatedEventArgs {
    pub fn ConnectionId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ConnectionId)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn Kind(&self) -> windows_core::Result<ActivationKind> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> windows_core::Result<ApplicationExecutionState> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PreviousExecutionState)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SplashScreen(&self) -> windows_core::Result<SplashScreen> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SplashScreen)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeName for IBarcodeScannerPreviewActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IBarcodeScannerPreviewActivatedEventArgs";
}
pub trait IBarcodeScannerPreviewActivatedEventArgs_Impl: IActivatedEventArgs_Impl {
    fn ConnectionId(&self) -> windows_core::Result<windows_core::HSTRING>;
}
impl IBarcodeScannerPreviewActivatedEventArgs_Vtbl {
    pub const fn new<Identity: IBarcodeScannerPreviewActivatedEventArgs_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ConnectionId<Identity: IBarcodeScannerPreviewActivatedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IBarcodeScannerPreviewActivatedEventArgs_Impl::ConnectionId(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IBarcodeScannerPreviewActivatedEventArgs, OFFSET>(),
            ConnectionId: ConnectionId::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IBarcodeScannerPreviewActivatedEventArgs as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarcodeScannerPreviewActivatedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ConnectionId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICachedFileUpdaterActivatedEventArgs, ICachedFileUpdaterActivatedEventArgs_Vtbl, 0xd06eb1c7_3805_4ecb_b757_6cf15e26fef3);
impl windows_core::RuntimeType for ICachedFileUpdaterActivatedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(ICachedFileUpdaterActivatedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(ICachedFileUpdaterActivatedEventArgs, IActivatedEventArgs);
impl ICachedFileUpdaterActivatedEventArgs {
    #[cfg(feature = "Storage_Provider")]
    pub fn CachedFileUpdaterUI(&self) -> windows_core::Result<super::super::Storage::Provider::CachedFileUpdaterUI> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CachedFileUpdaterUI)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Kind(&self) -> windows_core::Result<ActivationKind> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> windows_core::Result<ApplicationExecutionState> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PreviousExecutionState)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SplashScreen(&self) -> windows_core::Result<SplashScreen> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SplashScreen)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Storage_Provider")]
impl windows_core::RuntimeName for ICachedFileUpdaterActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.ICachedFileUpdaterActivatedEventArgs";
}
#[cfg(feature = "Storage_Provider")]
pub trait ICachedFileUpdaterActivatedEventArgs_Impl: IActivatedEventArgs_Impl {
    fn CachedFileUpdaterUI(&self) -> windows_core::Result<super::super::Storage::Provider::CachedFileUpdaterUI>;
}
#[cfg(feature = "Storage_Provider")]
impl ICachedFileUpdaterActivatedEventArgs_Vtbl {
    pub const fn new<Identity: ICachedFileUpdaterActivatedEventArgs_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CachedFileUpdaterUI<Identity: ICachedFileUpdaterActivatedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICachedFileUpdaterActivatedEventArgs_Impl::CachedFileUpdaterUI(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, ICachedFileUpdaterActivatedEventArgs, OFFSET>(),
            CachedFileUpdaterUI: CachedFileUpdaterUI::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICachedFileUpdaterActivatedEventArgs as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICachedFileUpdaterActivatedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Storage_Provider")]
    pub CachedFileUpdaterUI: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Provider"))]
    CachedFileUpdaterUI: usize,
}
windows_core::imp::define_interface!(ICameraSettingsActivatedEventArgs, ICameraSettingsActivatedEventArgs_Vtbl, 0xfb67a508_2dad_490a_9170_dca036eb114b);
impl windows_core::RuntimeType for ICameraSettingsActivatedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(ICameraSettingsActivatedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(ICameraSettingsActivatedEventArgs, IActivatedEventArgs);
impl ICameraSettingsActivatedEventArgs {
    pub fn VideoDeviceController(&self) -> windows_core::Result<windows_core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).VideoDeviceController)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn VideoDeviceExtension(&self) -> windows_core::Result<windows_core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).VideoDeviceExtension)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Kind(&self) -> windows_core::Result<ActivationKind> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> windows_core::Result<ApplicationExecutionState> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PreviousExecutionState)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SplashScreen(&self) -> windows_core::Result<SplashScreen> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SplashScreen)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeName for ICameraSettingsActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.ICameraSettingsActivatedEventArgs";
}
pub trait ICameraSettingsActivatedEventArgs_Impl: IActivatedEventArgs_Impl {
    fn VideoDeviceController(&self) -> windows_core::Result<windows_core::IInspectable>;
    fn VideoDeviceExtension(&self) -> windows_core::Result<windows_core::IInspectable>;
}
impl ICameraSettingsActivatedEventArgs_Vtbl {
    pub const fn new<Identity: ICameraSettingsActivatedEventArgs_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn VideoDeviceController<Identity: ICameraSettingsActivatedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICameraSettingsActivatedEventArgs_Impl::VideoDeviceController(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn VideoDeviceExtension<Identity: ICameraSettingsActivatedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICameraSettingsActivatedEventArgs_Impl::VideoDeviceExtension(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, ICameraSettingsActivatedEventArgs, OFFSET>(),
            VideoDeviceController: VideoDeviceController::<Identity, OFFSET>,
            VideoDeviceExtension: VideoDeviceExtension::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICameraSettingsActivatedEventArgs as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICameraSettingsActivatedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub VideoDeviceController: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub VideoDeviceExtension: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICommandLineActivatedEventArgs, ICommandLineActivatedEventArgs_Vtbl, 0x4506472c_006a_48eb_8afb_d07ab25e3366);
impl windows_core::RuntimeType for ICommandLineActivatedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(ICommandLineActivatedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(ICommandLineActivatedEventArgs, IActivatedEventArgs);
impl ICommandLineActivatedEventArgs {
    pub fn Operation(&self) -> windows_core::Result<CommandLineActivationOperation> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Operation)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Kind(&self) -> windows_core::Result<ActivationKind> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> windows_core::Result<ApplicationExecutionState> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PreviousExecutionState)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SplashScreen(&self) -> windows_core::Result<SplashScreen> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SplashScreen)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeName for ICommandLineActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.ICommandLineActivatedEventArgs";
}
pub trait ICommandLineActivatedEventArgs_Impl: IActivatedEventArgs_Impl {
    fn Operation(&self) -> windows_core::Result<CommandLineActivationOperation>;
}
impl ICommandLineActivatedEventArgs_Vtbl {
    pub const fn new<Identity: ICommandLineActivatedEventArgs_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Operation<Identity: ICommandLineActivatedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICommandLineActivatedEventArgs_Impl::Operation(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IInspectable_Vtbl::new::<Identity, ICommandLineActivatedEventArgs, OFFSET>(), Operation: Operation::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICommandLineActivatedEventArgs as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICommandLineActivatedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Operation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICommandLineActivationOperation, ICommandLineActivationOperation_Vtbl, 0x994b2841_c59e_4f69_bcfd_b61ed4e622eb);
impl windows_core::RuntimeType for ICommandLineActivationOperation {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ICommandLineActivationOperation_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Arguments: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CurrentDirectoryPath: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetExitCode: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub ExitCode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub GetDeferral: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IContactActivatedEventArgs, IContactActivatedEventArgs_Vtbl, 0xd627a1c4_c025_4c41_9def_f1eafad075e7);
impl windows_core::RuntimeType for IContactActivatedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(IContactActivatedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(IContactActivatedEventArgs, IActivatedEventArgs);
impl IContactActivatedEventArgs {
    pub fn Verb(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Verb)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn Kind(&self) -> windows_core::Result<ActivationKind> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> windows_core::Result<ApplicationExecutionState> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PreviousExecutionState)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SplashScreen(&self) -> windows_core::Result<SplashScreen> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SplashScreen)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeName for IContactActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IContactActivatedEventArgs";
}
pub trait IContactActivatedEventArgs_Impl: IActivatedEventArgs_Impl {
    fn Verb(&self) -> windows_core::Result<windows_core::HSTRING>;
}
impl IContactActivatedEventArgs_Vtbl {
    pub const fn new<Identity: IContactActivatedEventArgs_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Verb<Identity: IContactActivatedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IContactActivatedEventArgs_Impl::Verb(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IInspectable_Vtbl::new::<Identity, IContactActivatedEventArgs, OFFSET>(), Verb: Verb::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IContactActivatedEventArgs as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactActivatedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Verb: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IContactCallActivatedEventArgs, IContactCallActivatedEventArgs_Vtbl, 0xc2df14c7_30eb_41c6_b3bc_5b1694f9dab3);
impl windows_core::RuntimeType for IContactCallActivatedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(IContactCallActivatedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(IContactCallActivatedEventArgs, IActivatedEventArgs, IContactActivatedEventArgs);
impl IContactCallActivatedEventArgs {
    pub fn ServiceId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ServiceId)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn ServiceUserId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ServiceUserId)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(feature = "ApplicationModel_Contacts")]
    pub fn Contact(&self) -> windows_core::Result<super::Contacts::Contact> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Contact)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Kind(&self) -> windows_core::Result<ActivationKind> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> windows_core::Result<ApplicationExecutionState> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PreviousExecutionState)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SplashScreen(&self) -> windows_core::Result<SplashScreen> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SplashScreen)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Verb(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IContactActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Verb)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[cfg(feature = "ApplicationModel_Contacts")]
impl windows_core::RuntimeName for IContactCallActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IContactCallActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Contacts")]
pub trait IContactCallActivatedEventArgs_Impl: IActivatedEventArgs_Impl + IContactActivatedEventArgs_Impl {
    fn ServiceId(&self) -> windows_core::Result<windows_core::HSTRING>;
    fn ServiceUserId(&self) -> windows_core::Result<windows_core::HSTRING>;
    fn Contact(&self) -> windows_core::Result<super::Contacts::Contact>;
}
#[cfg(feature = "ApplicationModel_Contacts")]
impl IContactCallActivatedEventArgs_Vtbl {
    pub const fn new<Identity: IContactCallActivatedEventArgs_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ServiceId<Identity: IContactCallActivatedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IContactCallActivatedEventArgs_Impl::ServiceId(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ServiceUserId<Identity: IContactCallActivatedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IContactCallActivatedEventArgs_Impl::ServiceUserId(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Contact<Identity: IContactCallActivatedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IContactCallActivatedEventArgs_Impl::Contact(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IContactCallActivatedEventArgs, OFFSET>(),
            ServiceId: ServiceId::<Identity, OFFSET>,
            ServiceUserId: ServiceUserId::<Identity, OFFSET>,
            Contact: Contact::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IContactCallActivatedEventArgs as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactCallActivatedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ServiceId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ServiceUserId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "ApplicationModel_Contacts")]
    pub Contact: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Contacts"))]
    Contact: usize,
}
windows_core::imp::define_interface!(IContactMapActivatedEventArgs, IContactMapActivatedEventArgs_Vtbl, 0xb32bf870_eee7_4ad2_aaf1_a87effcf00a4);
impl windows_core::RuntimeType for IContactMapActivatedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(IContactMapActivatedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(IContactMapActivatedEventArgs, IActivatedEventArgs, IContactActivatedEventArgs);
impl IContactMapActivatedEventArgs {
    #[cfg(feature = "ApplicationModel_Contacts")]
    pub fn Address(&self) -> windows_core::Result<super::Contacts::ContactAddress> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Address)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "ApplicationModel_Contacts")]
    pub fn Contact(&self) -> windows_core::Result<super::Contacts::Contact> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Contact)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Kind(&self) -> windows_core::Result<ActivationKind> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> windows_core::Result<ApplicationExecutionState> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PreviousExecutionState)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SplashScreen(&self) -> windows_core::Result<SplashScreen> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SplashScreen)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Verb(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IContactActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Verb)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[cfg(feature = "ApplicationModel_Contacts")]
impl windows_core::RuntimeName for IContactMapActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IContactMapActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Contacts")]
pub trait IContactMapActivatedEventArgs_Impl: IActivatedEventArgs_Impl + IContactActivatedEventArgs_Impl {
    fn Address(&self) -> windows_core::Result<super::Contacts::ContactAddress>;
    fn Contact(&self) -> windows_core::Result<super::Contacts::Contact>;
}
#[cfg(feature = "ApplicationModel_Contacts")]
impl IContactMapActivatedEventArgs_Vtbl {
    pub const fn new<Identity: IContactMapActivatedEventArgs_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Address<Identity: IContactMapActivatedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IContactMapActivatedEventArgs_Impl::Address(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Contact<Identity: IContactMapActivatedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IContactMapActivatedEventArgs_Impl::Contact(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IContactMapActivatedEventArgs, OFFSET>(),
            Address: Address::<Identity, OFFSET>,
            Contact: Contact::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IContactMapActivatedEventArgs as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactMapActivatedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "ApplicationModel_Contacts")]
    pub Address: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Contacts"))]
    Address: usize,
    #[cfg(feature = "ApplicationModel_Contacts")]
    pub Contact: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Contacts"))]
    Contact: usize,
}
windows_core::imp::define_interface!(IContactMessageActivatedEventArgs, IContactMessageActivatedEventArgs_Vtbl, 0xde598db2_0e03_43b0_bf56_bcc40b3162df);
impl windows_core::RuntimeType for IContactMessageActivatedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(IContactMessageActivatedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(IContactMessageActivatedEventArgs, IActivatedEventArgs, IContactActivatedEventArgs);
impl IContactMessageActivatedEventArgs {
    pub fn ServiceId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ServiceId)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn ServiceUserId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ServiceUserId)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(feature = "ApplicationModel_Contacts")]
    pub fn Contact(&self) -> windows_core::Result<super::Contacts::Contact> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Contact)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Kind(&self) -> windows_core::Result<ActivationKind> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> windows_core::Result<ApplicationExecutionState> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PreviousExecutionState)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SplashScreen(&self) -> windows_core::Result<SplashScreen> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SplashScreen)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Verb(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IContactActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Verb)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[cfg(feature = "ApplicationModel_Contacts")]
impl windows_core::RuntimeName for IContactMessageActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IContactMessageActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Contacts")]
pub trait IContactMessageActivatedEventArgs_Impl: IActivatedEventArgs_Impl + IContactActivatedEventArgs_Impl {
    fn ServiceId(&self) -> windows_core::Result<windows_core::HSTRING>;
    fn ServiceUserId(&self) -> windows_core::Result<windows_core::HSTRING>;
    fn Contact(&self) -> windows_core::Result<super::Contacts::Contact>;
}
#[cfg(feature = "ApplicationModel_Contacts")]
impl IContactMessageActivatedEventArgs_Vtbl {
    pub const fn new<Identity: IContactMessageActivatedEventArgs_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ServiceId<Identity: IContactMessageActivatedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IContactMessageActivatedEventArgs_Impl::ServiceId(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ServiceUserId<Identity: IContactMessageActivatedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IContactMessageActivatedEventArgs_Impl::ServiceUserId(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Contact<Identity: IContactMessageActivatedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IContactMessageActivatedEventArgs_Impl::Contact(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IContactMessageActivatedEventArgs, OFFSET>(),
            ServiceId: ServiceId::<Identity, OFFSET>,
            ServiceUserId: ServiceUserId::<Identity, OFFSET>,
            Contact: Contact::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IContactMessageActivatedEventArgs as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactMessageActivatedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ServiceId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ServiceUserId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "ApplicationModel_Contacts")]
    pub Contact: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Contacts"))]
    Contact: usize,
}
windows_core::imp::define_interface!(IContactPanelActivatedEventArgs, IContactPanelActivatedEventArgs_Vtbl, 0x52bb63e4_d3d4_4b63_8051_4af2082cab80);
impl windows_core::RuntimeType for IContactPanelActivatedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(IContactPanelActivatedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl IContactPanelActivatedEventArgs {
    #[cfg(feature = "ApplicationModel_Contacts")]
    pub fn ContactPanel(&self) -> windows_core::Result<super::Contacts::ContactPanel> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ContactPanel)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "ApplicationModel_Contacts")]
    pub fn Contact(&self) -> windows_core::Result<super::Contacts::Contact> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Contact)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "ApplicationModel_Contacts")]
impl windows_core::RuntimeName for IContactPanelActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IContactPanelActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Contacts")]
pub trait IContactPanelActivatedEventArgs_Impl: windows_core::IUnknownImpl {
    fn ContactPanel(&self) -> windows_core::Result<super::Contacts::ContactPanel>;
    fn Contact(&self) -> windows_core::Result<super::Contacts::Contact>;
}
#[cfg(feature = "ApplicationModel_Contacts")]
impl IContactPanelActivatedEventArgs_Vtbl {
    pub const fn new<Identity: IContactPanelActivatedEventArgs_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ContactPanel<Identity: IContactPanelActivatedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IContactPanelActivatedEventArgs_Impl::ContactPanel(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Contact<Identity: IContactPanelActivatedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IContactPanelActivatedEventArgs_Impl::Contact(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IContactPanelActivatedEventArgs, OFFSET>(),
            ContactPanel: ContactPanel::<Identity, OFFSET>,
            Contact: Contact::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IContactPanelActivatedEventArgs as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactPanelActivatedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "ApplicationModel_Contacts")]
    pub ContactPanel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Contacts"))]
    ContactPanel: usize,
    #[cfg(feature = "ApplicationModel_Contacts")]
    pub Contact: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Contacts"))]
    Contact: usize,
}
windows_core::imp::define_interface!(IContactPickerActivatedEventArgs, IContactPickerActivatedEventArgs_Vtbl, 0xce57aae7_6449_45a7_971f_d113be7a8936);
impl windows_core::RuntimeType for IContactPickerActivatedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(IContactPickerActivatedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(IContactPickerActivatedEventArgs, IActivatedEventArgs);
impl IContactPickerActivatedEventArgs {
    #[cfg(feature = "ApplicationModel_Contacts_Provider")]
    pub fn ContactPickerUI(&self) -> windows_core::Result<super::Contacts::Provider::ContactPickerUI> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ContactPickerUI)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Kind(&self) -> windows_core::Result<ActivationKind> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> windows_core::Result<ApplicationExecutionState> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PreviousExecutionState)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SplashScreen(&self) -> windows_core::Result<SplashScreen> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SplashScreen)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "ApplicationModel_Contacts_Provider")]
impl windows_core::RuntimeName for IContactPickerActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IContactPickerActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Contacts_Provider")]
pub trait IContactPickerActivatedEventArgs_Impl: IActivatedEventArgs_Impl {
    fn ContactPickerUI(&self) -> windows_core::Result<super::Contacts::Provider::ContactPickerUI>;
}
#[cfg(feature = "ApplicationModel_Contacts_Provider")]
impl IContactPickerActivatedEventArgs_Vtbl {
    pub const fn new<Identity: IContactPickerActivatedEventArgs_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ContactPickerUI<Identity: IContactPickerActivatedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IContactPickerActivatedEventArgs_Impl::ContactPickerUI(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IContactPickerActivatedEventArgs, OFFSET>(),
            ContactPickerUI: ContactPickerUI::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IContactPickerActivatedEventArgs as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactPickerActivatedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "ApplicationModel_Contacts_Provider")]
    pub ContactPickerUI: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Contacts_Provider"))]
    ContactPickerUI: usize,
}
windows_core::imp::define_interface!(IContactPostActivatedEventArgs, IContactPostActivatedEventArgs_Vtbl, 0xb35a3c67_f1e7_4655_ad6e_4857588f552f);
impl windows_core::RuntimeType for IContactPostActivatedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(IContactPostActivatedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(IContactPostActivatedEventArgs, IActivatedEventArgs, IContactActivatedEventArgs);
impl IContactPostActivatedEventArgs {
    pub fn ServiceId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ServiceId)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn ServiceUserId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ServiceUserId)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(feature = "ApplicationModel_Contacts")]
    pub fn Contact(&self) -> windows_core::Result<super::Contacts::Contact> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Contact)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Kind(&self) -> windows_core::Result<ActivationKind> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> windows_core::Result<ApplicationExecutionState> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PreviousExecutionState)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SplashScreen(&self) -> windows_core::Result<SplashScreen> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SplashScreen)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Verb(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IContactActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Verb)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[cfg(feature = "ApplicationModel_Contacts")]
impl windows_core::RuntimeName for IContactPostActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IContactPostActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Contacts")]
pub trait IContactPostActivatedEventArgs_Impl: IActivatedEventArgs_Impl + IContactActivatedEventArgs_Impl {
    fn ServiceId(&self) -> windows_core::Result<windows_core::HSTRING>;
    fn ServiceUserId(&self) -> windows_core::Result<windows_core::HSTRING>;
    fn Contact(&self) -> windows_core::Result<super::Contacts::Contact>;
}
#[cfg(feature = "ApplicationModel_Contacts")]
impl IContactPostActivatedEventArgs_Vtbl {
    pub const fn new<Identity: IContactPostActivatedEventArgs_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ServiceId<Identity: IContactPostActivatedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IContactPostActivatedEventArgs_Impl::ServiceId(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ServiceUserId<Identity: IContactPostActivatedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IContactPostActivatedEventArgs_Impl::ServiceUserId(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Contact<Identity: IContactPostActivatedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IContactPostActivatedEventArgs_Impl::Contact(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IContactPostActivatedEventArgs, OFFSET>(),
            ServiceId: ServiceId::<Identity, OFFSET>,
            ServiceUserId: ServiceUserId::<Identity, OFFSET>,
            Contact: Contact::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IContactPostActivatedEventArgs as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactPostActivatedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ServiceId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ServiceUserId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "ApplicationModel_Contacts")]
    pub Contact: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Contacts"))]
    Contact: usize,
}
windows_core::imp::define_interface!(IContactVideoCallActivatedEventArgs, IContactVideoCallActivatedEventArgs_Vtbl, 0x61079db8_e3e7_4b4f_858d_5c63a96ef684);
impl windows_core::RuntimeType for IContactVideoCallActivatedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(IContactVideoCallActivatedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(IContactVideoCallActivatedEventArgs, IActivatedEventArgs, IContactActivatedEventArgs);
impl IContactVideoCallActivatedEventArgs {
    pub fn ServiceId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ServiceId)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn ServiceUserId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ServiceUserId)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(feature = "ApplicationModel_Contacts")]
    pub fn Contact(&self) -> windows_core::Result<super::Contacts::Contact> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Contact)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Kind(&self) -> windows_core::Result<ActivationKind> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> windows_core::Result<ApplicationExecutionState> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PreviousExecutionState)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SplashScreen(&self) -> windows_core::Result<SplashScreen> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SplashScreen)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Verb(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IContactActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Verb)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[cfg(feature = "ApplicationModel_Contacts")]
impl windows_core::RuntimeName for IContactVideoCallActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IContactVideoCallActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Contacts")]
pub trait IContactVideoCallActivatedEventArgs_Impl: IActivatedEventArgs_Impl + IContactActivatedEventArgs_Impl {
    fn ServiceId(&self) -> windows_core::Result<windows_core::HSTRING>;
    fn ServiceUserId(&self) -> windows_core::Result<windows_core::HSTRING>;
    fn Contact(&self) -> windows_core::Result<super::Contacts::Contact>;
}
#[cfg(feature = "ApplicationModel_Contacts")]
impl IContactVideoCallActivatedEventArgs_Vtbl {
    pub const fn new<Identity: IContactVideoCallActivatedEventArgs_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ServiceId<Identity: IContactVideoCallActivatedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IContactVideoCallActivatedEventArgs_Impl::ServiceId(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ServiceUserId<Identity: IContactVideoCallActivatedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IContactVideoCallActivatedEventArgs_Impl::ServiceUserId(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Contact<Identity: IContactVideoCallActivatedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IContactVideoCallActivatedEventArgs_Impl::Contact(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IContactVideoCallActivatedEventArgs, OFFSET>(),
            ServiceId: ServiceId::<Identity, OFFSET>,
            ServiceUserId: ServiceUserId::<Identity, OFFSET>,
            Contact: Contact::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IContactVideoCallActivatedEventArgs as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactVideoCallActivatedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ServiceId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ServiceUserId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "ApplicationModel_Contacts")]
    pub Contact: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Contacts"))]
    Contact: usize,
}
windows_core::imp::define_interface!(IContactsProviderActivatedEventArgs, IContactsProviderActivatedEventArgs_Vtbl, 0x4580dca8_5750_4916_aa52_c0829521eb94);
impl windows_core::RuntimeType for IContactsProviderActivatedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(IContactsProviderActivatedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(IContactsProviderActivatedEventArgs, IActivatedEventArgs);
impl IContactsProviderActivatedEventArgs {
    pub fn Verb(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Verb)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn Kind(&self) -> windows_core::Result<ActivationKind> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> windows_core::Result<ApplicationExecutionState> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PreviousExecutionState)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SplashScreen(&self) -> windows_core::Result<SplashScreen> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SplashScreen)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeName for IContactsProviderActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IContactsProviderActivatedEventArgs";
}
pub trait IContactsProviderActivatedEventArgs_Impl: IActivatedEventArgs_Impl {
    fn Verb(&self) -> windows_core::Result<windows_core::HSTRING>;
}
impl IContactsProviderActivatedEventArgs_Vtbl {
    pub const fn new<Identity: IContactsProviderActivatedEventArgs_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Verb<Identity: IContactsProviderActivatedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IContactsProviderActivatedEventArgs_Impl::Verb(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IInspectable_Vtbl::new::<Identity, IContactsProviderActivatedEventArgs, OFFSET>(), Verb: Verb::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IContactsProviderActivatedEventArgs as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactsProviderActivatedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Verb: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IContinuationActivatedEventArgs, IContinuationActivatedEventArgs_Vtbl, 0xe58106b5_155f_4a94_a742_c7e08f4e188c);
impl windows_core::RuntimeType for IContinuationActivatedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(IContinuationActivatedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(IContinuationActivatedEventArgs, IActivatedEventArgs);
impl IContinuationActivatedEventArgs {
    #[cfg(feature = "Foundation_Collections")]
    pub fn ContinuationData(&self) -> windows_core::Result<super::super::Foundation::Collections::ValueSet> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ContinuationData)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Kind(&self) -> windows_core::Result<ActivationKind> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> windows_core::Result<ApplicationExecutionState> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PreviousExecutionState)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SplashScreen(&self) -> windows_core::Result<SplashScreen> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SplashScreen)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Foundation_Collections")]
impl windows_core::RuntimeName for IContinuationActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IContinuationActivatedEventArgs";
}
#[cfg(feature = "Foundation_Collections")]
pub trait IContinuationActivatedEventArgs_Impl: IActivatedEventArgs_Impl {
    fn ContinuationData(&self) -> windows_core::Result<super::super::Foundation::Collections::ValueSet>;
}
#[cfg(feature = "Foundation_Collections")]
impl IContinuationActivatedEventArgs_Vtbl {
    pub const fn new<Identity: IContinuationActivatedEventArgs_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ContinuationData<Identity: IContinuationActivatedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IContinuationActivatedEventArgs_Impl::ContinuationData(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IContinuationActivatedEventArgs, OFFSET>(),
            ContinuationData: ContinuationData::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IContinuationActivatedEventArgs as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IContinuationActivatedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub ContinuationData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ContinuationData: usize,
}
windows_core::imp::define_interface!(IDeviceActivatedEventArgs, IDeviceActivatedEventArgs_Vtbl, 0xcd50b9a9_ce10_44d2_8234_c355a073ef33);
impl windows_core::RuntimeType for IDeviceActivatedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(IDeviceActivatedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(IDeviceActivatedEventArgs, IActivatedEventArgs);
impl IDeviceActivatedEventArgs {
    pub fn DeviceInformationId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DeviceInformationId)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn Verb(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Verb)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn Kind(&self) -> windows_core::Result<ActivationKind> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> windows_core::Result<ApplicationExecutionState> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PreviousExecutionState)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SplashScreen(&self) -> windows_core::Result<SplashScreen> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SplashScreen)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeName for IDeviceActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IDeviceActivatedEventArgs";
}
pub trait IDeviceActivatedEventArgs_Impl: IActivatedEventArgs_Impl {
    fn DeviceInformationId(&self) -> windows_core::Result<windows_core::HSTRING>;
    fn Verb(&self) -> windows_core::Result<windows_core::HSTRING>;
}
impl IDeviceActivatedEventArgs_Vtbl {
    pub const fn new<Identity: IDeviceActivatedEventArgs_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn DeviceInformationId<Identity: IDeviceActivatedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDeviceActivatedEventArgs_Impl::DeviceInformationId(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Verb<Identity: IDeviceActivatedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDeviceActivatedEventArgs_Impl::Verb(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IDeviceActivatedEventArgs, OFFSET>(),
            DeviceInformationId: DeviceInformationId::<Identity, OFFSET>,
            Verb: Verb::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDeviceActivatedEventArgs as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDeviceActivatedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub DeviceInformationId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Verb: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDevicePairingActivatedEventArgs, IDevicePairingActivatedEventArgs_Vtbl, 0xeba0d1e4_ecc6_4148_94ed_f4b37ec05b3e);
impl windows_core::RuntimeType for IDevicePairingActivatedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(IDevicePairingActivatedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(IDevicePairingActivatedEventArgs, IActivatedEventArgs);
impl IDevicePairingActivatedEventArgs {
    #[cfg(feature = "Devices_Enumeration")]
    pub fn DeviceInformation(&self) -> windows_core::Result<super::super::Devices::Enumeration::DeviceInformation> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DeviceInformation)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Kind(&self) -> windows_core::Result<ActivationKind> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> windows_core::Result<ApplicationExecutionState> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PreviousExecutionState)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SplashScreen(&self) -> windows_core::Result<SplashScreen> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SplashScreen)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Devices_Enumeration")]
impl windows_core::RuntimeName for IDevicePairingActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IDevicePairingActivatedEventArgs";
}
#[cfg(feature = "Devices_Enumeration")]
pub trait IDevicePairingActivatedEventArgs_Impl: IActivatedEventArgs_Impl {
    fn DeviceInformation(&self) -> windows_core::Result<super::super::Devices::Enumeration::DeviceInformation>;
}
#[cfg(feature = "Devices_Enumeration")]
impl IDevicePairingActivatedEventArgs_Vtbl {
    pub const fn new<Identity: IDevicePairingActivatedEventArgs_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn DeviceInformation<Identity: IDevicePairingActivatedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDevicePairingActivatedEventArgs_Impl::DeviceInformation(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IDevicePairingActivatedEventArgs, OFFSET>(),
            DeviceInformation: DeviceInformation::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDevicePairingActivatedEventArgs as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDevicePairingActivatedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Devices_Enumeration")]
    pub DeviceInformation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Devices_Enumeration"))]
    DeviceInformation: usize,
}
windows_core::imp::define_interface!(IDialReceiverActivatedEventArgs, IDialReceiverActivatedEventArgs_Vtbl, 0xfb777ed7_85ee_456e_a44d_85d730e70aed);
impl windows_core::RuntimeType for IDialReceiverActivatedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(IDialReceiverActivatedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(IDialReceiverActivatedEventArgs, IActivatedEventArgs, ILaunchActivatedEventArgs);
impl IDialReceiverActivatedEventArgs {
    pub fn AppName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AppName)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn Kind(&self) -> windows_core::Result<ActivationKind> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> windows_core::Result<ApplicationExecutionState> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PreviousExecutionState)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SplashScreen(&self) -> windows_core::Result<SplashScreen> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SplashScreen)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Arguments(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<ILaunchActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Arguments)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn TileId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<ILaunchActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TileId)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
impl windows_core::RuntimeName for IDialReceiverActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IDialReceiverActivatedEventArgs";
}
pub trait IDialReceiverActivatedEventArgs_Impl: IActivatedEventArgs_Impl + ILaunchActivatedEventArgs_Impl {
    fn AppName(&self) -> windows_core::Result<windows_core::HSTRING>;
}
impl IDialReceiverActivatedEventArgs_Vtbl {
    pub const fn new<Identity: IDialReceiverActivatedEventArgs_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AppName<Identity: IDialReceiverActivatedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDialReceiverActivatedEventArgs_Impl::AppName(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IInspectable_Vtbl::new::<Identity, IDialReceiverActivatedEventArgs, OFFSET>(), AppName: AppName::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDialReceiverActivatedEventArgs as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDialReceiverActivatedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub AppName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IFileActivatedEventArgs, IFileActivatedEventArgs_Vtbl, 0xbb2afc33_93b1_42ed_8b26_236dd9c78496);
impl windows_core::RuntimeType for IFileActivatedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(IFileActivatedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(IFileActivatedEventArgs, IActivatedEventArgs);
impl IFileActivatedEventArgs {
    #[cfg(feature = "Storage")]
    pub fn Files(&self) -> windows_core::Result<windows_collections::IVectorView<super::super::Storage::IStorageItem>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Files)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Verb(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Verb)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn Kind(&self) -> windows_core::Result<ActivationKind> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> windows_core::Result<ApplicationExecutionState> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PreviousExecutionState)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SplashScreen(&self) -> windows_core::Result<SplashScreen> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SplashScreen)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Storage")]
impl windows_core::RuntimeName for IFileActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IFileActivatedEventArgs";
}
#[cfg(feature = "Storage")]
pub trait IFileActivatedEventArgs_Impl: IActivatedEventArgs_Impl {
    fn Files(&self) -> windows_core::Result<windows_collections::IVectorView<super::super::Storage::IStorageItem>>;
    fn Verb(&self) -> windows_core::Result<windows_core::HSTRING>;
}
#[cfg(feature = "Storage")]
impl IFileActivatedEventArgs_Vtbl {
    pub const fn new<Identity: IFileActivatedEventArgs_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Files<Identity: IFileActivatedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFileActivatedEventArgs_Impl::Files(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Verb<Identity: IFileActivatedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFileActivatedEventArgs_Impl::Verb(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IFileActivatedEventArgs, OFFSET>(),
            Files: Files::<Identity, OFFSET>,
            Verb: Verb::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFileActivatedEventArgs as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFileActivatedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Storage")]
    pub Files: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage"))]
    Files: usize,
    pub Verb: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IFileActivatedEventArgsWithCallerPackageFamilyName, IFileActivatedEventArgsWithCallerPackageFamilyName_Vtbl, 0x2d60f06b_d25f_4d25_8653_e1c5e1108309);
impl windows_core::RuntimeType for IFileActivatedEventArgsWithCallerPackageFamilyName {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(IFileActivatedEventArgsWithCallerPackageFamilyName, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(IFileActivatedEventArgsWithCallerPackageFamilyName, IActivatedEventArgs);
impl IFileActivatedEventArgsWithCallerPackageFamilyName {
    pub fn CallerPackageFamilyName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CallerPackageFamilyName)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn Kind(&self) -> windows_core::Result<ActivationKind> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> windows_core::Result<ApplicationExecutionState> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PreviousExecutionState)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SplashScreen(&self) -> windows_core::Result<SplashScreen> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SplashScreen)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeName for IFileActivatedEventArgsWithCallerPackageFamilyName {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IFileActivatedEventArgsWithCallerPackageFamilyName";
}
pub trait IFileActivatedEventArgsWithCallerPackageFamilyName_Impl: IActivatedEventArgs_Impl {
    fn CallerPackageFamilyName(&self) -> windows_core::Result<windows_core::HSTRING>;
}
impl IFileActivatedEventArgsWithCallerPackageFamilyName_Vtbl {
    pub const fn new<Identity: IFileActivatedEventArgsWithCallerPackageFamilyName_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CallerPackageFamilyName<Identity: IFileActivatedEventArgsWithCallerPackageFamilyName_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFileActivatedEventArgsWithCallerPackageFamilyName_Impl::CallerPackageFamilyName(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IFileActivatedEventArgsWithCallerPackageFamilyName, OFFSET>(),
            CallerPackageFamilyName: CallerPackageFamilyName::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFileActivatedEventArgsWithCallerPackageFamilyName as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFileActivatedEventArgsWithCallerPackageFamilyName_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CallerPackageFamilyName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IFileActivatedEventArgsWithNeighboringFiles, IFileActivatedEventArgsWithNeighboringFiles_Vtbl, 0x433ba1a4_e1e2_48fd_b7fc_b5d6eee65033);
impl windows_core::RuntimeType for IFileActivatedEventArgsWithNeighboringFiles {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(IFileActivatedEventArgsWithNeighboringFiles, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(IFileActivatedEventArgsWithNeighboringFiles, IActivatedEventArgs, IFileActivatedEventArgs);
impl IFileActivatedEventArgsWithNeighboringFiles {
    #[cfg(feature = "Storage_Search")]
    pub fn NeighboringFilesQuery(&self) -> windows_core::Result<super::super::Storage::Search::StorageFileQueryResult> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NeighboringFilesQuery)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Kind(&self) -> windows_core::Result<ActivationKind> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> windows_core::Result<ApplicationExecutionState> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PreviousExecutionState)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SplashScreen(&self) -> windows_core::Result<SplashScreen> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SplashScreen)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage")]
    pub fn Files(&self) -> windows_core::Result<windows_collections::IVectorView<super::super::Storage::IStorageItem>> {
        let this = &windows_core::Interface::cast::<IFileActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Files)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Verb(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IFileActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Verb)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[cfg(feature = "Storage_Search")]
impl windows_core::RuntimeName for IFileActivatedEventArgsWithNeighboringFiles {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IFileActivatedEventArgsWithNeighboringFiles";
}
#[cfg(feature = "Storage_Search")]
pub trait IFileActivatedEventArgsWithNeighboringFiles_Impl: IActivatedEventArgs_Impl + IFileActivatedEventArgs_Impl {
    fn NeighboringFilesQuery(&self) -> windows_core::Result<super::super::Storage::Search::StorageFileQueryResult>;
}
#[cfg(feature = "Storage_Search")]
impl IFileActivatedEventArgsWithNeighboringFiles_Vtbl {
    pub const fn new<Identity: IFileActivatedEventArgsWithNeighboringFiles_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn NeighboringFilesQuery<Identity: IFileActivatedEventArgsWithNeighboringFiles_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFileActivatedEventArgsWithNeighboringFiles_Impl::NeighboringFilesQuery(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IFileActivatedEventArgsWithNeighboringFiles, OFFSET>(),
            NeighboringFilesQuery: NeighboringFilesQuery::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFileActivatedEventArgsWithNeighboringFiles as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFileActivatedEventArgsWithNeighboringFiles_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Storage_Search")]
    pub NeighboringFilesQuery: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Search"))]
    NeighboringFilesQuery: usize,
}
windows_core::imp::define_interface!(IFileOpenPickerActivatedEventArgs, IFileOpenPickerActivatedEventArgs_Vtbl, 0x72827082_5525_4bf2_bc09_1f5095d4964d);
impl windows_core::RuntimeType for IFileOpenPickerActivatedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(IFileOpenPickerActivatedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(IFileOpenPickerActivatedEventArgs, IActivatedEventArgs);
impl IFileOpenPickerActivatedEventArgs {
    #[cfg(feature = "Storage_Pickers_Provider")]
    pub fn FileOpenPickerUI(&self) -> windows_core::Result<super::super::Storage::Pickers::Provider::FileOpenPickerUI> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FileOpenPickerUI)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Kind(&self) -> windows_core::Result<ActivationKind> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> windows_core::Result<ApplicationExecutionState> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PreviousExecutionState)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SplashScreen(&self) -> windows_core::Result<SplashScreen> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SplashScreen)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Storage_Pickers_Provider")]
impl windows_core::RuntimeName for IFileOpenPickerActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IFileOpenPickerActivatedEventArgs";
}
#[cfg(feature = "Storage_Pickers_Provider")]
pub trait IFileOpenPickerActivatedEventArgs_Impl: IActivatedEventArgs_Impl {
    fn FileOpenPickerUI(&self) -> windows_core::Result<super::super::Storage::Pickers::Provider::FileOpenPickerUI>;
}
#[cfg(feature = "Storage_Pickers_Provider")]
impl IFileOpenPickerActivatedEventArgs_Vtbl {
    pub const fn new<Identity: IFileOpenPickerActivatedEventArgs_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn FileOpenPickerUI<Identity: IFileOpenPickerActivatedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFileOpenPickerActivatedEventArgs_Impl::FileOpenPickerUI(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IFileOpenPickerActivatedEventArgs, OFFSET>(),
            FileOpenPickerUI: FileOpenPickerUI::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFileOpenPickerActivatedEventArgs as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFileOpenPickerActivatedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Storage_Pickers_Provider")]
    pub FileOpenPickerUI: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Pickers_Provider"))]
    FileOpenPickerUI: usize,
}
windows_core::imp::define_interface!(IFileOpenPickerActivatedEventArgs2, IFileOpenPickerActivatedEventArgs2_Vtbl, 0x5e731f66_8d1f_45fb_af1d_73205c8fc7a1);
impl windows_core::RuntimeType for IFileOpenPickerActivatedEventArgs2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(IFileOpenPickerActivatedEventArgs2, windows_core::IUnknown, windows_core::IInspectable);
impl IFileOpenPickerActivatedEventArgs2 {
    pub fn CallerPackageFamilyName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CallerPackageFamilyName)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
impl windows_core::RuntimeName for IFileOpenPickerActivatedEventArgs2 {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IFileOpenPickerActivatedEventArgs2";
}
pub trait IFileOpenPickerActivatedEventArgs2_Impl: windows_core::IUnknownImpl {
    fn CallerPackageFamilyName(&self) -> windows_core::Result<windows_core::HSTRING>;
}
impl IFileOpenPickerActivatedEventArgs2_Vtbl {
    pub const fn new<Identity: IFileOpenPickerActivatedEventArgs2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CallerPackageFamilyName<Identity: IFileOpenPickerActivatedEventArgs2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFileOpenPickerActivatedEventArgs2_Impl::CallerPackageFamilyName(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IFileOpenPickerActivatedEventArgs2, OFFSET>(),
            CallerPackageFamilyName: CallerPackageFamilyName::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFileOpenPickerActivatedEventArgs2 as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFileOpenPickerActivatedEventArgs2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CallerPackageFamilyName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IFileOpenPickerContinuationEventArgs, IFileOpenPickerContinuationEventArgs_Vtbl, 0xf0fa3f3a_d4e8_4ad3_9c34_2308f32fcec9);
impl windows_core::RuntimeType for IFileOpenPickerContinuationEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(IFileOpenPickerContinuationEventArgs, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(IFileOpenPickerContinuationEventArgs, IActivatedEventArgs, IContinuationActivatedEventArgs);
impl IFileOpenPickerContinuationEventArgs {
    #[cfg(feature = "Storage_Streams")]
    pub fn Files(&self) -> windows_core::Result<windows_collections::IVectorView<super::super::Storage::StorageFile>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Files)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Kind(&self) -> windows_core::Result<ActivationKind> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> windows_core::Result<ApplicationExecutionState> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PreviousExecutionState)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SplashScreen(&self) -> windows_core::Result<SplashScreen> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SplashScreen)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn ContinuationData(&self) -> windows_core::Result<super::super::Foundation::Collections::ValueSet> {
        let this = &windows_core::Interface::cast::<IContinuationActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ContinuationData)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))]
impl windows_core::RuntimeName for IFileOpenPickerContinuationEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IFileOpenPickerContinuationEventArgs";
}
#[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))]
pub trait IFileOpenPickerContinuationEventArgs_Impl: IActivatedEventArgs_Impl + IContinuationActivatedEventArgs_Impl {
    fn Files(&self) -> windows_core::Result<windows_collections::IVectorView<super::super::Storage::StorageFile>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))]
impl IFileOpenPickerContinuationEventArgs_Vtbl {
    pub const fn new<Identity: IFileOpenPickerContinuationEventArgs_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Files<Identity: IFileOpenPickerContinuationEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFileOpenPickerContinuationEventArgs_Impl::Files(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IInspectable_Vtbl::new::<Identity, IFileOpenPickerContinuationEventArgs, OFFSET>(), Files: Files::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFileOpenPickerContinuationEventArgs as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFileOpenPickerContinuationEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Storage_Streams")]
    pub Files: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Files: usize,
}
windows_core::imp::define_interface!(IFileSavePickerActivatedEventArgs, IFileSavePickerActivatedEventArgs_Vtbl, 0x81c19cf1_74e6_4387_82eb_bb8fd64b4346);
impl windows_core::RuntimeType for IFileSavePickerActivatedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(IFileSavePickerActivatedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(IFileSavePickerActivatedEventArgs, IActivatedEventArgs);
impl IFileSavePickerActivatedEventArgs {
    #[cfg(feature = "Storage_Pickers_Provider")]
    pub fn FileSavePickerUI(&self) -> windows_core::Result<super::super::Storage::Pickers::Provider::FileSavePickerUI> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FileSavePickerUI)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Kind(&self) -> windows_core::Result<ActivationKind> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> windows_core::Result<ApplicationExecutionState> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PreviousExecutionState)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SplashScreen(&self) -> windows_core::Result<SplashScreen> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SplashScreen)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Storage_Pickers_Provider")]
impl windows_core::RuntimeName for IFileSavePickerActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IFileSavePickerActivatedEventArgs";
}
#[cfg(feature = "Storage_Pickers_Provider")]
pub trait IFileSavePickerActivatedEventArgs_Impl: IActivatedEventArgs_Impl {
    fn FileSavePickerUI(&self) -> windows_core::Result<super::super::Storage::Pickers::Provider::FileSavePickerUI>;
}
#[cfg(feature = "Storage_Pickers_Provider")]
impl IFileSavePickerActivatedEventArgs_Vtbl {
    pub const fn new<Identity: IFileSavePickerActivatedEventArgs_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn FileSavePickerUI<Identity: IFileSavePickerActivatedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFileSavePickerActivatedEventArgs_Impl::FileSavePickerUI(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IFileSavePickerActivatedEventArgs, OFFSET>(),
            FileSavePickerUI: FileSavePickerUI::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFileSavePickerActivatedEventArgs as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFileSavePickerActivatedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Storage_Pickers_Provider")]
    pub FileSavePickerUI: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Pickers_Provider"))]
    FileSavePickerUI: usize,
}
windows_core::imp::define_interface!(IFileSavePickerActivatedEventArgs2, IFileSavePickerActivatedEventArgs2_Vtbl, 0x6b73fe13_2cf2_4d48_8cbc_af67d23f1ce7);
impl windows_core::RuntimeType for IFileSavePickerActivatedEventArgs2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(IFileSavePickerActivatedEventArgs2, windows_core::IUnknown, windows_core::IInspectable);
impl IFileSavePickerActivatedEventArgs2 {
    pub fn CallerPackageFamilyName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CallerPackageFamilyName)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn EnterpriseId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).EnterpriseId)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
impl windows_core::RuntimeName for IFileSavePickerActivatedEventArgs2 {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IFileSavePickerActivatedEventArgs2";
}
pub trait IFileSavePickerActivatedEventArgs2_Impl: windows_core::IUnknownImpl {
    fn CallerPackageFamilyName(&self) -> windows_core::Result<windows_core::HSTRING>;
    fn EnterpriseId(&self) -> windows_core::Result<windows_core::HSTRING>;
}
impl IFileSavePickerActivatedEventArgs2_Vtbl {
    pub const fn new<Identity: IFileSavePickerActivatedEventArgs2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CallerPackageFamilyName<Identity: IFileSavePickerActivatedEventArgs2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFileSavePickerActivatedEventArgs2_Impl::CallerPackageFamilyName(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn EnterpriseId<Identity: IFileSavePickerActivatedEventArgs2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFileSavePickerActivatedEventArgs2_Impl::EnterpriseId(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IFileSavePickerActivatedEventArgs2, OFFSET>(),
            CallerPackageFamilyName: CallerPackageFamilyName::<Identity, OFFSET>,
            EnterpriseId: EnterpriseId::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFileSavePickerActivatedEventArgs2 as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFileSavePickerActivatedEventArgs2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CallerPackageFamilyName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EnterpriseId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IFileSavePickerContinuationEventArgs, IFileSavePickerContinuationEventArgs_Vtbl, 0x2c846fe1_3bad_4f33_8c8b_e46fae824b4b);
impl windows_core::RuntimeType for IFileSavePickerContinuationEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(IFileSavePickerContinuationEventArgs, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(IFileSavePickerContinuationEventArgs, IActivatedEventArgs, IContinuationActivatedEventArgs);
impl IFileSavePickerContinuationEventArgs {
    #[cfg(feature = "Storage_Streams")]
    pub fn File(&self) -> windows_core::Result<super::super::Storage::StorageFile> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).File)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Kind(&self) -> windows_core::Result<ActivationKind> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> windows_core::Result<ApplicationExecutionState> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PreviousExecutionState)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SplashScreen(&self) -> windows_core::Result<SplashScreen> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SplashScreen)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn ContinuationData(&self) -> windows_core::Result<super::super::Foundation::Collections::ValueSet> {
        let this = &windows_core::Interface::cast::<IContinuationActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ContinuationData)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))]
impl windows_core::RuntimeName for IFileSavePickerContinuationEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IFileSavePickerContinuationEventArgs";
}
#[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))]
pub trait IFileSavePickerContinuationEventArgs_Impl: IActivatedEventArgs_Impl + IContinuationActivatedEventArgs_Impl {
    fn File(&self) -> windows_core::Result<super::super::Storage::StorageFile>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))]
impl IFileSavePickerContinuationEventArgs_Vtbl {
    pub const fn new<Identity: IFileSavePickerContinuationEventArgs_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn File<Identity: IFileSavePickerContinuationEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFileSavePickerContinuationEventArgs_Impl::File(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IInspectable_Vtbl::new::<Identity, IFileSavePickerContinuationEventArgs, OFFSET>(), File: File::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFileSavePickerContinuationEventArgs as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFileSavePickerContinuationEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Storage_Streams")]
    pub File: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    File: usize,
}
windows_core::imp::define_interface!(IFolderPickerContinuationEventArgs, IFolderPickerContinuationEventArgs_Vtbl, 0x51882366_9f4b_498f_beb0_42684f6e1c29);
impl windows_core::RuntimeType for IFolderPickerContinuationEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(IFolderPickerContinuationEventArgs, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(IFolderPickerContinuationEventArgs, IActivatedEventArgs, IContinuationActivatedEventArgs);
impl IFolderPickerContinuationEventArgs {
    #[cfg(feature = "Storage_Search")]
    pub fn Folder(&self) -> windows_core::Result<super::super::Storage::StorageFolder> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Folder)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Kind(&self) -> windows_core::Result<ActivationKind> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> windows_core::Result<ApplicationExecutionState> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PreviousExecutionState)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SplashScreen(&self) -> windows_core::Result<SplashScreen> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SplashScreen)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn ContinuationData(&self) -> windows_core::Result<super::super::Foundation::Collections::ValueSet> {
        let this = &windows_core::Interface::cast::<IContinuationActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ContinuationData)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "Storage_Search"))]
impl windows_core::RuntimeName for IFolderPickerContinuationEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IFolderPickerContinuationEventArgs";
}
#[cfg(all(feature = "Foundation_Collections", feature = "Storage_Search"))]
pub trait IFolderPickerContinuationEventArgs_Impl: IActivatedEventArgs_Impl + IContinuationActivatedEventArgs_Impl {
    fn Folder(&self) -> windows_core::Result<super::super::Storage::StorageFolder>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "Storage_Search"))]
impl IFolderPickerContinuationEventArgs_Vtbl {
    pub const fn new<Identity: IFolderPickerContinuationEventArgs_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Folder<Identity: IFolderPickerContinuationEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFolderPickerContinuationEventArgs_Impl::Folder(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IInspectable_Vtbl::new::<Identity, IFolderPickerContinuationEventArgs, OFFSET>(), Folder: Folder::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFolderPickerContinuationEventArgs as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFolderPickerContinuationEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Storage_Search")]
    pub Folder: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Search"))]
    Folder: usize,
}
windows_core::imp::define_interface!(ILaunchActivatedEventArgs, ILaunchActivatedEventArgs_Vtbl, 0xfbc93e26_a14a_4b4f_82b0_33bed920af52);
impl windows_core::RuntimeType for ILaunchActivatedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(ILaunchActivatedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(ILaunchActivatedEventArgs, IActivatedEventArgs);
impl ILaunchActivatedEventArgs {
    pub fn Arguments(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Arguments)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn TileId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TileId)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn Kind(&self) -> windows_core::Result<ActivationKind> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> windows_core::Result<ApplicationExecutionState> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PreviousExecutionState)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SplashScreen(&self) -> windows_core::Result<SplashScreen> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SplashScreen)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeName for ILaunchActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.ILaunchActivatedEventArgs";
}
pub trait ILaunchActivatedEventArgs_Impl: IActivatedEventArgs_Impl {
    fn Arguments(&self) -> windows_core::Result<windows_core::HSTRING>;
    fn TileId(&self) -> windows_core::Result<windows_core::HSTRING>;
}
impl ILaunchActivatedEventArgs_Vtbl {
    pub const fn new<Identity: ILaunchActivatedEventArgs_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Arguments<Identity: ILaunchActivatedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ILaunchActivatedEventArgs_Impl::Arguments(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn TileId<Identity: ILaunchActivatedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ILaunchActivatedEventArgs_Impl::TileId(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, ILaunchActivatedEventArgs, OFFSET>(),
            Arguments: Arguments::<Identity, OFFSET>,
            TileId: TileId::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ILaunchActivatedEventArgs as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ILaunchActivatedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Arguments: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TileId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ILaunchActivatedEventArgs2, ILaunchActivatedEventArgs2_Vtbl, 0x0fd37ebc_9dc9_46b5_9ace_bd95d4565345);
impl windows_core::RuntimeType for ILaunchActivatedEventArgs2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(ILaunchActivatedEventArgs2, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(ILaunchActivatedEventArgs2, IActivatedEventArgs, ILaunchActivatedEventArgs);
impl ILaunchActivatedEventArgs2 {
    pub fn TileActivatedInfo(&self) -> windows_core::Result<TileActivatedInfo> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TileActivatedInfo)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Kind(&self) -> windows_core::Result<ActivationKind> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> windows_core::Result<ApplicationExecutionState> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PreviousExecutionState)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SplashScreen(&self) -> windows_core::Result<SplashScreen> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SplashScreen)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Arguments(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<ILaunchActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Arguments)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn TileId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<ILaunchActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TileId)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
impl windows_core::RuntimeName for ILaunchActivatedEventArgs2 {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.ILaunchActivatedEventArgs2";
}
pub trait ILaunchActivatedEventArgs2_Impl: IActivatedEventArgs_Impl + ILaunchActivatedEventArgs_Impl {
    fn TileActivatedInfo(&self) -> windows_core::Result<TileActivatedInfo>;
}
impl ILaunchActivatedEventArgs2_Vtbl {
    pub const fn new<Identity: ILaunchActivatedEventArgs2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn TileActivatedInfo<Identity: ILaunchActivatedEventArgs2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ILaunchActivatedEventArgs2_Impl::TileActivatedInfo(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, ILaunchActivatedEventArgs2, OFFSET>(),
            TileActivatedInfo: TileActivatedInfo::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ILaunchActivatedEventArgs2 as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ILaunchActivatedEventArgs2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub TileActivatedInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ILockScreenActivatedEventArgs, ILockScreenActivatedEventArgs_Vtbl, 0x3ca77966_6108_4a41_8220_ee7d133c8532);
impl windows_core::RuntimeType for ILockScreenActivatedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(ILockScreenActivatedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(ILockScreenActivatedEventArgs, IActivatedEventArgs);
impl ILockScreenActivatedEventArgs {
    pub fn Info(&self) -> windows_core::Result<windows_core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Info)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Kind(&self) -> windows_core::Result<ActivationKind> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> windows_core::Result<ApplicationExecutionState> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PreviousExecutionState)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SplashScreen(&self) -> windows_core::Result<SplashScreen> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SplashScreen)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeName for ILockScreenActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.ILockScreenActivatedEventArgs";
}
pub trait ILockScreenActivatedEventArgs_Impl: IActivatedEventArgs_Impl {
    fn Info(&self) -> windows_core::Result<windows_core::IInspectable>;
}
impl ILockScreenActivatedEventArgs_Vtbl {
    pub const fn new<Identity: ILockScreenActivatedEventArgs_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Info<Identity: ILockScreenActivatedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ILockScreenActivatedEventArgs_Impl::Info(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IInspectable_Vtbl::new::<Identity, ILockScreenActivatedEventArgs, OFFSET>(), Info: Info::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ILockScreenActivatedEventArgs as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ILockScreenActivatedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Info: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ILockScreenCallActivatedEventArgs, ILockScreenCallActivatedEventArgs_Vtbl, 0x06f37fbe_b5f2_448b_b13e_e328ac1c516a);
impl windows_core::RuntimeType for ILockScreenCallActivatedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(ILockScreenCallActivatedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(ILockScreenCallActivatedEventArgs, IActivatedEventArgs, ILaunchActivatedEventArgs);
impl ILockScreenCallActivatedEventArgs {
    #[cfg(feature = "ApplicationModel_Calls")]
    pub fn CallUI(&self) -> windows_core::Result<super::Calls::LockScreenCallUI> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CallUI)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Kind(&self) -> windows_core::Result<ActivationKind> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> windows_core::Result<ApplicationExecutionState> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PreviousExecutionState)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SplashScreen(&self) -> windows_core::Result<SplashScreen> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SplashScreen)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Arguments(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<ILaunchActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Arguments)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn TileId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<ILaunchActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TileId)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[cfg(feature = "ApplicationModel_Calls")]
impl windows_core::RuntimeName for ILockScreenCallActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.ILockScreenCallActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Calls")]
pub trait ILockScreenCallActivatedEventArgs_Impl: IActivatedEventArgs_Impl + ILaunchActivatedEventArgs_Impl {
    fn CallUI(&self) -> windows_core::Result<super::Calls::LockScreenCallUI>;
}
#[cfg(feature = "ApplicationModel_Calls")]
impl ILockScreenCallActivatedEventArgs_Vtbl {
    pub const fn new<Identity: ILockScreenCallActivatedEventArgs_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CallUI<Identity: ILockScreenCallActivatedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ILockScreenCallActivatedEventArgs_Impl::CallUI(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IInspectable_Vtbl::new::<Identity, ILockScreenCallActivatedEventArgs, OFFSET>(), CallUI: CallUI::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ILockScreenCallActivatedEventArgs as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ILockScreenCallActivatedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "ApplicationModel_Calls")]
    pub CallUI: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Calls"))]
    CallUI: usize,
}
windows_core::imp::define_interface!(IPhoneCallActivatedEventArgs, IPhoneCallActivatedEventArgs_Vtbl, 0x54615221_a3c1_4ced_b62f_8c60523619ad);
impl windows_core::RuntimeType for IPhoneCallActivatedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(IPhoneCallActivatedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(IPhoneCallActivatedEventArgs, IActivatedEventArgs);
impl IPhoneCallActivatedEventArgs {
    pub fn LineId(&self) -> windows_core::Result<windows_core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LineId)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Kind(&self) -> windows_core::Result<ActivationKind> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> windows_core::Result<ApplicationExecutionState> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PreviousExecutionState)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SplashScreen(&self) -> windows_core::Result<SplashScreen> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SplashScreen)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeName for IPhoneCallActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IPhoneCallActivatedEventArgs";
}
pub trait IPhoneCallActivatedEventArgs_Impl: IActivatedEventArgs_Impl {
    fn LineId(&self) -> windows_core::Result<windows_core::GUID>;
}
impl IPhoneCallActivatedEventArgs_Vtbl {
    pub const fn new<Identity: IPhoneCallActivatedEventArgs_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn LineId<Identity: IPhoneCallActivatedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPhoneCallActivatedEventArgs_Impl::LineId(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IInspectable_Vtbl::new::<Identity, IPhoneCallActivatedEventArgs, OFFSET>(), LineId: LineId::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPhoneCallActivatedEventArgs as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhoneCallActivatedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub LineId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPickerReturnedActivatedEventArgs, IPickerReturnedActivatedEventArgs_Vtbl, 0x360defb9_a9d3_4984_a4ed_9ec734604921);
impl windows_core::RuntimeType for IPickerReturnedActivatedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(IPickerReturnedActivatedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(IPickerReturnedActivatedEventArgs, IActivatedEventArgs);
impl IPickerReturnedActivatedEventArgs {
    pub fn PickerOperationId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PickerOperationId)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn Kind(&self) -> windows_core::Result<ActivationKind> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> windows_core::Result<ApplicationExecutionState> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PreviousExecutionState)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SplashScreen(&self) -> windows_core::Result<SplashScreen> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SplashScreen)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeName for IPickerReturnedActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IPickerReturnedActivatedEventArgs";
}
pub trait IPickerReturnedActivatedEventArgs_Impl: IActivatedEventArgs_Impl {
    fn PickerOperationId(&self) -> windows_core::Result<windows_core::HSTRING>;
}
impl IPickerReturnedActivatedEventArgs_Vtbl {
    pub const fn new<Identity: IPickerReturnedActivatedEventArgs_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn PickerOperationId<Identity: IPickerReturnedActivatedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPickerReturnedActivatedEventArgs_Impl::PickerOperationId(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IPickerReturnedActivatedEventArgs, OFFSET>(),
            PickerOperationId: PickerOperationId::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPickerReturnedActivatedEventArgs as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPickerReturnedActivatedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub PickerOperationId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPrelaunchActivatedEventArgs, IPrelaunchActivatedEventArgs_Vtbl, 0x0c44717b_19f7_48d6_b046_cf22826eaa74);
impl windows_core::RuntimeType for IPrelaunchActivatedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(IPrelaunchActivatedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(IPrelaunchActivatedEventArgs, IActivatedEventArgs);
impl IPrelaunchActivatedEventArgs {
    pub fn PrelaunchActivated(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PrelaunchActivated)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Kind(&self) -> windows_core::Result<ActivationKind> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> windows_core::Result<ApplicationExecutionState> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PreviousExecutionState)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SplashScreen(&self) -> windows_core::Result<SplashScreen> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SplashScreen)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeName for IPrelaunchActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IPrelaunchActivatedEventArgs";
}
pub trait IPrelaunchActivatedEventArgs_Impl: IActivatedEventArgs_Impl {
    fn PrelaunchActivated(&self) -> windows_core::Result<bool>;
}
impl IPrelaunchActivatedEventArgs_Vtbl {
    pub const fn new<Identity: IPrelaunchActivatedEventArgs_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn PrelaunchActivated<Identity: IPrelaunchActivatedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPrelaunchActivatedEventArgs_Impl::PrelaunchActivated(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IPrelaunchActivatedEventArgs, OFFSET>(),
            PrelaunchActivated: PrelaunchActivated::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPrelaunchActivatedEventArgs as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrelaunchActivatedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub PrelaunchActivated: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPrint3DWorkflowActivatedEventArgs, IPrint3DWorkflowActivatedEventArgs_Vtbl, 0x3f57e78b_f2ac_4619_8302_ef855e1c9b90);
impl windows_core::RuntimeType for IPrint3DWorkflowActivatedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(IPrint3DWorkflowActivatedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(IPrint3DWorkflowActivatedEventArgs, IActivatedEventArgs);
impl IPrint3DWorkflowActivatedEventArgs {
    #[cfg(feature = "Devices_Printers_Extensions")]
    pub fn Workflow(&self) -> windows_core::Result<super::super::Devices::Printers::Extensions::Print3DWorkflow> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Workflow)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Kind(&self) -> windows_core::Result<ActivationKind> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> windows_core::Result<ApplicationExecutionState> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PreviousExecutionState)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SplashScreen(&self) -> windows_core::Result<SplashScreen> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SplashScreen)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Devices_Printers_Extensions")]
impl windows_core::RuntimeName for IPrint3DWorkflowActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IPrint3DWorkflowActivatedEventArgs";
}
#[cfg(feature = "Devices_Printers_Extensions")]
pub trait IPrint3DWorkflowActivatedEventArgs_Impl: IActivatedEventArgs_Impl {
    fn Workflow(&self) -> windows_core::Result<super::super::Devices::Printers::Extensions::Print3DWorkflow>;
}
#[cfg(feature = "Devices_Printers_Extensions")]
impl IPrint3DWorkflowActivatedEventArgs_Vtbl {
    pub const fn new<Identity: IPrint3DWorkflowActivatedEventArgs_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Workflow<Identity: IPrint3DWorkflowActivatedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPrint3DWorkflowActivatedEventArgs_Impl::Workflow(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IInspectable_Vtbl::new::<Identity, IPrint3DWorkflowActivatedEventArgs, OFFSET>(), Workflow: Workflow::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPrint3DWorkflowActivatedEventArgs as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrint3DWorkflowActivatedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Devices_Printers_Extensions")]
    pub Workflow: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Devices_Printers_Extensions"))]
    Workflow: usize,
}
windows_core::imp::define_interface!(IPrintTaskSettingsActivatedEventArgs, IPrintTaskSettingsActivatedEventArgs_Vtbl, 0xee30a0c9_ce56_4865_ba8e_8954ac271107);
impl windows_core::RuntimeType for IPrintTaskSettingsActivatedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(IPrintTaskSettingsActivatedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(IPrintTaskSettingsActivatedEventArgs, IActivatedEventArgs);
impl IPrintTaskSettingsActivatedEventArgs {
    #[cfg(feature = "Devices_Printers_Extensions")]
    pub fn Configuration(&self) -> windows_core::Result<super::super::Devices::Printers::Extensions::PrintTaskConfiguration> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Configuration)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Kind(&self) -> windows_core::Result<ActivationKind> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> windows_core::Result<ApplicationExecutionState> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PreviousExecutionState)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SplashScreen(&self) -> windows_core::Result<SplashScreen> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SplashScreen)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Devices_Printers_Extensions")]
impl windows_core::RuntimeName for IPrintTaskSettingsActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IPrintTaskSettingsActivatedEventArgs";
}
#[cfg(feature = "Devices_Printers_Extensions")]
pub trait IPrintTaskSettingsActivatedEventArgs_Impl: IActivatedEventArgs_Impl {
    fn Configuration(&self) -> windows_core::Result<super::super::Devices::Printers::Extensions::PrintTaskConfiguration>;
}
#[cfg(feature = "Devices_Printers_Extensions")]
impl IPrintTaskSettingsActivatedEventArgs_Vtbl {
    pub const fn new<Identity: IPrintTaskSettingsActivatedEventArgs_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Configuration<Identity: IPrintTaskSettingsActivatedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPrintTaskSettingsActivatedEventArgs_Impl::Configuration(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IPrintTaskSettingsActivatedEventArgs, OFFSET>(),
            Configuration: Configuration::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPrintTaskSettingsActivatedEventArgs as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintTaskSettingsActivatedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Devices_Printers_Extensions")]
    pub Configuration: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Devices_Printers_Extensions"))]
    Configuration: usize,
}
windows_core::imp::define_interface!(IProtocolActivatedEventArgs, IProtocolActivatedEventArgs_Vtbl, 0x6095f4dd_b7c0_46ab_81fe_d90f36d00d24);
impl windows_core::RuntimeType for IProtocolActivatedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(IProtocolActivatedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(IProtocolActivatedEventArgs, IActivatedEventArgs);
impl IProtocolActivatedEventArgs {
    pub fn Uri(&self) -> windows_core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Uri)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Kind(&self) -> windows_core::Result<ActivationKind> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> windows_core::Result<ApplicationExecutionState> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PreviousExecutionState)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SplashScreen(&self) -> windows_core::Result<SplashScreen> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SplashScreen)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeName for IProtocolActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IProtocolActivatedEventArgs";
}
pub trait IProtocolActivatedEventArgs_Impl: IActivatedEventArgs_Impl {
    fn Uri(&self) -> windows_core::Result<super::super::Foundation::Uri>;
}
impl IProtocolActivatedEventArgs_Vtbl {
    pub const fn new<Identity: IProtocolActivatedEventArgs_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Uri<Identity: IProtocolActivatedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IProtocolActivatedEventArgs_Impl::Uri(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IInspectable_Vtbl::new::<Identity, IProtocolActivatedEventArgs, OFFSET>(), Uri: Uri::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IProtocolActivatedEventArgs as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IProtocolActivatedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Uri: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData, IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData_Vtbl, 0xd84a0c12_5c8f_438c_83cb_c28fcc0b2fdb);
impl windows_core::RuntimeType for IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData, IActivatedEventArgs);
impl IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData {
    pub fn CallerPackageFamilyName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CallerPackageFamilyName)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Data(&self) -> windows_core::Result<super::super::Foundation::Collections::ValueSet> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Data)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Kind(&self) -> windows_core::Result<ActivationKind> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> windows_core::Result<ApplicationExecutionState> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PreviousExecutionState)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SplashScreen(&self) -> windows_core::Result<SplashScreen> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SplashScreen)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Foundation_Collections")]
impl windows_core::RuntimeName for IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData";
}
#[cfg(feature = "Foundation_Collections")]
pub trait IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData_Impl: IActivatedEventArgs_Impl {
    fn CallerPackageFamilyName(&self) -> windows_core::Result<windows_core::HSTRING>;
    fn Data(&self) -> windows_core::Result<super::super::Foundation::Collections::ValueSet>;
}
#[cfg(feature = "Foundation_Collections")]
impl IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData_Vtbl {
    pub const fn new<Identity: IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CallerPackageFamilyName<Identity: IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData_Impl::CallerPackageFamilyName(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Data<Identity: IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData_Impl::Data(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData, OFFSET>(),
            CallerPackageFamilyName: CallerPackageFamilyName::<Identity, OFFSET>,
            Data: Data::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CallerPackageFamilyName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Data: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Data: usize,
}
windows_core::imp::define_interface!(IProtocolForResultsActivatedEventArgs, IProtocolForResultsActivatedEventArgs_Vtbl, 0xe75132c2_7ae7_4517_80ac_dbe8d7cc5b9c);
impl windows_core::RuntimeType for IProtocolForResultsActivatedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(IProtocolForResultsActivatedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(IProtocolForResultsActivatedEventArgs, IActivatedEventArgs);
impl IProtocolForResultsActivatedEventArgs {
    #[cfg(feature = "System")]
    pub fn ProtocolForResultsOperation(&self) -> windows_core::Result<super::super::System::ProtocolForResultsOperation> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ProtocolForResultsOperation)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Kind(&self) -> windows_core::Result<ActivationKind> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> windows_core::Result<ApplicationExecutionState> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PreviousExecutionState)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SplashScreen(&self) -> windows_core::Result<SplashScreen> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SplashScreen)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "System")]
impl windows_core::RuntimeName for IProtocolForResultsActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IProtocolForResultsActivatedEventArgs";
}
#[cfg(feature = "System")]
pub trait IProtocolForResultsActivatedEventArgs_Impl: IActivatedEventArgs_Impl {
    fn ProtocolForResultsOperation(&self) -> windows_core::Result<super::super::System::ProtocolForResultsOperation>;
}
#[cfg(feature = "System")]
impl IProtocolForResultsActivatedEventArgs_Vtbl {
    pub const fn new<Identity: IProtocolForResultsActivatedEventArgs_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ProtocolForResultsOperation<Identity: IProtocolForResultsActivatedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IProtocolForResultsActivatedEventArgs_Impl::ProtocolForResultsOperation(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IProtocolForResultsActivatedEventArgs, OFFSET>(),
            ProtocolForResultsOperation: ProtocolForResultsOperation::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IProtocolForResultsActivatedEventArgs as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IProtocolForResultsActivatedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "System")]
    pub ProtocolForResultsOperation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    ProtocolForResultsOperation: usize,
}
windows_core::imp::define_interface!(IRestrictedLaunchActivatedEventArgs, IRestrictedLaunchActivatedEventArgs_Vtbl, 0xe0b7ac81_bfc3_4344_a5da_19fd5a27baae);
impl windows_core::RuntimeType for IRestrictedLaunchActivatedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(IRestrictedLaunchActivatedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(IRestrictedLaunchActivatedEventArgs, IActivatedEventArgs);
impl IRestrictedLaunchActivatedEventArgs {
    pub fn SharedContext(&self) -> windows_core::Result<windows_core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SharedContext)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Kind(&self) -> windows_core::Result<ActivationKind> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> windows_core::Result<ApplicationExecutionState> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PreviousExecutionState)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SplashScreen(&self) -> windows_core::Result<SplashScreen> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SplashScreen)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeName for IRestrictedLaunchActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IRestrictedLaunchActivatedEventArgs";
}
pub trait IRestrictedLaunchActivatedEventArgs_Impl: IActivatedEventArgs_Impl {
    fn SharedContext(&self) -> windows_core::Result<windows_core::IInspectable>;
}
impl IRestrictedLaunchActivatedEventArgs_Vtbl {
    pub const fn new<Identity: IRestrictedLaunchActivatedEventArgs_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SharedContext<Identity: IRestrictedLaunchActivatedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRestrictedLaunchActivatedEventArgs_Impl::SharedContext(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IRestrictedLaunchActivatedEventArgs, OFFSET>(),
            SharedContext: SharedContext::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRestrictedLaunchActivatedEventArgs as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRestrictedLaunchActivatedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SharedContext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISearchActivatedEventArgs, ISearchActivatedEventArgs_Vtbl, 0x8cb36951_58c8_43e3_94bc_41d33f8b630e);
impl windows_core::RuntimeType for ISearchActivatedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(ISearchActivatedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(ISearchActivatedEventArgs, IActivatedEventArgs);
impl ISearchActivatedEventArgs {
    pub fn QueryText(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).QueryText)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn Language(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Language)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn Kind(&self) -> windows_core::Result<ActivationKind> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> windows_core::Result<ApplicationExecutionState> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PreviousExecutionState)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SplashScreen(&self) -> windows_core::Result<SplashScreen> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SplashScreen)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeName for ISearchActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.ISearchActivatedEventArgs";
}
pub trait ISearchActivatedEventArgs_Impl: IActivatedEventArgs_Impl {
    fn QueryText(&self) -> windows_core::Result<windows_core::HSTRING>;
    fn Language(&self) -> windows_core::Result<windows_core::HSTRING>;
}
impl ISearchActivatedEventArgs_Vtbl {
    pub const fn new<Identity: ISearchActivatedEventArgs_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn QueryText<Identity: ISearchActivatedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISearchActivatedEventArgs_Impl::QueryText(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Language<Identity: ISearchActivatedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISearchActivatedEventArgs_Impl::Language(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, ISearchActivatedEventArgs, OFFSET>(),
            QueryText: QueryText::<Identity, OFFSET>,
            Language: Language::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISearchActivatedEventArgs as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISearchActivatedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub QueryText: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Language: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISearchActivatedEventArgsWithLinguisticDetails, ISearchActivatedEventArgsWithLinguisticDetails_Vtbl, 0xc09f33da_08ab_4931_9b7c_451025f21f81);
impl windows_core::RuntimeType for ISearchActivatedEventArgsWithLinguisticDetails {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(ISearchActivatedEventArgsWithLinguisticDetails, windows_core::IUnknown, windows_core::IInspectable);
impl ISearchActivatedEventArgsWithLinguisticDetails {
    #[cfg(feature = "ApplicationModel_Search")]
    pub fn LinguisticDetails(&self) -> windows_core::Result<super::Search::SearchPaneQueryLinguisticDetails> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LinguisticDetails)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "ApplicationModel_Search")]
impl windows_core::RuntimeName for ISearchActivatedEventArgsWithLinguisticDetails {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.ISearchActivatedEventArgsWithLinguisticDetails";
}
#[cfg(feature = "ApplicationModel_Search")]
pub trait ISearchActivatedEventArgsWithLinguisticDetails_Impl: windows_core::IUnknownImpl {
    fn LinguisticDetails(&self) -> windows_core::Result<super::Search::SearchPaneQueryLinguisticDetails>;
}
#[cfg(feature = "ApplicationModel_Search")]
impl ISearchActivatedEventArgsWithLinguisticDetails_Vtbl {
    pub const fn new<Identity: ISearchActivatedEventArgsWithLinguisticDetails_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn LinguisticDetails<Identity: ISearchActivatedEventArgsWithLinguisticDetails_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISearchActivatedEventArgsWithLinguisticDetails_Impl::LinguisticDetails(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, ISearchActivatedEventArgsWithLinguisticDetails, OFFSET>(),
            LinguisticDetails: LinguisticDetails::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISearchActivatedEventArgsWithLinguisticDetails as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISearchActivatedEventArgsWithLinguisticDetails_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "ApplicationModel_Search")]
    pub LinguisticDetails: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Search"))]
    LinguisticDetails: usize,
}
windows_core::imp::define_interface!(IShareTargetActivatedEventArgs, IShareTargetActivatedEventArgs_Vtbl, 0x4bdaf9c8_cdb2_4acb_bfc3_6648563378ec);
impl windows_core::RuntimeType for IShareTargetActivatedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(IShareTargetActivatedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(IShareTargetActivatedEventArgs, IActivatedEventArgs);
impl IShareTargetActivatedEventArgs {
    #[cfg(feature = "ApplicationModel_DataTransfer_ShareTarget")]
    pub fn ShareOperation(&self) -> windows_core::Result<super::DataTransfer::ShareTarget::ShareOperation> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ShareOperation)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Kind(&self) -> windows_core::Result<ActivationKind> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> windows_core::Result<ApplicationExecutionState> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PreviousExecutionState)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SplashScreen(&self) -> windows_core::Result<SplashScreen> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SplashScreen)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "ApplicationModel_DataTransfer_ShareTarget")]
impl windows_core::RuntimeName for IShareTargetActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IShareTargetActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_DataTransfer_ShareTarget")]
pub trait IShareTargetActivatedEventArgs_Impl: IActivatedEventArgs_Impl {
    fn ShareOperation(&self) -> windows_core::Result<super::DataTransfer::ShareTarget::ShareOperation>;
}
#[cfg(feature = "ApplicationModel_DataTransfer_ShareTarget")]
impl IShareTargetActivatedEventArgs_Vtbl {
    pub const fn new<Identity: IShareTargetActivatedEventArgs_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ShareOperation<Identity: IShareTargetActivatedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IShareTargetActivatedEventArgs_Impl::ShareOperation(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IShareTargetActivatedEventArgs, OFFSET>(),
            ShareOperation: ShareOperation::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IShareTargetActivatedEventArgs as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IShareTargetActivatedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "ApplicationModel_DataTransfer_ShareTarget")]
    pub ShareOperation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_DataTransfer_ShareTarget"))]
    ShareOperation: usize,
}
windows_core::imp::define_interface!(ISplashScreen, ISplashScreen_Vtbl, 0xca4d975c_d4d6_43f0_97c0_0833c6391c24);
impl windows_core::RuntimeType for ISplashScreen {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ISplashScreen_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ImageLocation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::Rect) -> windows_core::HRESULT,
    pub Dismissed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveDismissed: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IStartupTaskActivatedEventArgs, IStartupTaskActivatedEventArgs_Vtbl, 0x03b11a58_5276_4d91_8621_54611864d5fa);
impl windows_core::RuntimeType for IStartupTaskActivatedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(IStartupTaskActivatedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(IStartupTaskActivatedEventArgs, IActivatedEventArgs);
impl IStartupTaskActivatedEventArgs {
    pub fn TaskId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TaskId)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn Kind(&self) -> windows_core::Result<ActivationKind> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> windows_core::Result<ApplicationExecutionState> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PreviousExecutionState)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SplashScreen(&self) -> windows_core::Result<SplashScreen> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SplashScreen)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeName for IStartupTaskActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IStartupTaskActivatedEventArgs";
}
pub trait IStartupTaskActivatedEventArgs_Impl: IActivatedEventArgs_Impl {
    fn TaskId(&self) -> windows_core::Result<windows_core::HSTRING>;
}
impl IStartupTaskActivatedEventArgs_Vtbl {
    pub const fn new<Identity: IStartupTaskActivatedEventArgs_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn TaskId<Identity: IStartupTaskActivatedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IStartupTaskActivatedEventArgs_Impl::TaskId(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IInspectable_Vtbl::new::<Identity, IStartupTaskActivatedEventArgs, OFFSET>(), TaskId: TaskId::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IStartupTaskActivatedEventArgs as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IStartupTaskActivatedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub TaskId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ITileActivatedInfo, ITileActivatedInfo_Vtbl, 0x80e4a3b1_3980_4f17_b738_89194e0b8f65);
impl windows_core::RuntimeType for ITileActivatedInfo {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ITileActivatedInfo_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "UI_Notifications")]
    pub RecentlyShownNotifications: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Notifications"))]
    RecentlyShownNotifications: usize,
}
windows_core::imp::define_interface!(IToastNotificationActivatedEventArgs, IToastNotificationActivatedEventArgs_Vtbl, 0x92a86f82_5290_431d_be85_c4aaeeb8685f);
impl windows_core::RuntimeType for IToastNotificationActivatedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(IToastNotificationActivatedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(IToastNotificationActivatedEventArgs, IActivatedEventArgs);
impl IToastNotificationActivatedEventArgs {
    pub fn Argument(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Argument)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn UserInput(&self) -> windows_core::Result<super::super::Foundation::Collections::ValueSet> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UserInput)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Kind(&self) -> windows_core::Result<ActivationKind> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> windows_core::Result<ApplicationExecutionState> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PreviousExecutionState)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SplashScreen(&self) -> windows_core::Result<SplashScreen> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SplashScreen)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Foundation_Collections")]
impl windows_core::RuntimeName for IToastNotificationActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IToastNotificationActivatedEventArgs";
}
#[cfg(feature = "Foundation_Collections")]
pub trait IToastNotificationActivatedEventArgs_Impl: IActivatedEventArgs_Impl {
    fn Argument(&self) -> windows_core::Result<windows_core::HSTRING>;
    fn UserInput(&self) -> windows_core::Result<super::super::Foundation::Collections::ValueSet>;
}
#[cfg(feature = "Foundation_Collections")]
impl IToastNotificationActivatedEventArgs_Vtbl {
    pub const fn new<Identity: IToastNotificationActivatedEventArgs_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Argument<Identity: IToastNotificationActivatedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IToastNotificationActivatedEventArgs_Impl::Argument(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn UserInput<Identity: IToastNotificationActivatedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IToastNotificationActivatedEventArgs_Impl::UserInput(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IToastNotificationActivatedEventArgs, OFFSET>(),
            Argument: Argument::<Identity, OFFSET>,
            UserInput: UserInput::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IToastNotificationActivatedEventArgs as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IToastNotificationActivatedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Argument: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub UserInput: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    UserInput: usize,
}
windows_core::imp::define_interface!(IUserDataAccountProviderActivatedEventArgs, IUserDataAccountProviderActivatedEventArgs_Vtbl, 0x1bc9f723_8ef1_4a51_a63a_fe711eeab607);
impl windows_core::RuntimeType for IUserDataAccountProviderActivatedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(IUserDataAccountProviderActivatedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(IUserDataAccountProviderActivatedEventArgs, IActivatedEventArgs);
impl IUserDataAccountProviderActivatedEventArgs {
    #[cfg(feature = "ApplicationModel_UserDataAccounts_Provider")]
    pub fn Operation(&self) -> windows_core::Result<super::UserDataAccounts::Provider::IUserDataAccountProviderOperation> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Operation)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Kind(&self) -> windows_core::Result<ActivationKind> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> windows_core::Result<ApplicationExecutionState> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PreviousExecutionState)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SplashScreen(&self) -> windows_core::Result<SplashScreen> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SplashScreen)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "ApplicationModel_UserDataAccounts_Provider")]
impl windows_core::RuntimeName for IUserDataAccountProviderActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IUserDataAccountProviderActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_UserDataAccounts_Provider")]
pub trait IUserDataAccountProviderActivatedEventArgs_Impl: IActivatedEventArgs_Impl {
    fn Operation(&self) -> windows_core::Result<super::UserDataAccounts::Provider::IUserDataAccountProviderOperation>;
}
#[cfg(feature = "ApplicationModel_UserDataAccounts_Provider")]
impl IUserDataAccountProviderActivatedEventArgs_Vtbl {
    pub const fn new<Identity: IUserDataAccountProviderActivatedEventArgs_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Operation<Identity: IUserDataAccountProviderActivatedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUserDataAccountProviderActivatedEventArgs_Impl::Operation(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IUserDataAccountProviderActivatedEventArgs, OFFSET>(),
            Operation: Operation::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUserDataAccountProviderActivatedEventArgs as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserDataAccountProviderActivatedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "ApplicationModel_UserDataAccounts_Provider")]
    pub Operation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_UserDataAccounts_Provider"))]
    Operation: usize,
}
windows_core::imp::define_interface!(IViewSwitcherProvider, IViewSwitcherProvider_Vtbl, 0x33f288a6_5c2c_4d27_bac7_7536088f1219);
impl windows_core::RuntimeType for IViewSwitcherProvider {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(IViewSwitcherProvider, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(IViewSwitcherProvider, IActivatedEventArgs);
impl IViewSwitcherProvider {
    #[cfg(feature = "UI_ViewManagement")]
    pub fn ViewSwitcher(&self) -> windows_core::Result<super::super::UI::ViewManagement::ActivationViewSwitcher> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ViewSwitcher)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Kind(&self) -> windows_core::Result<ActivationKind> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> windows_core::Result<ApplicationExecutionState> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PreviousExecutionState)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SplashScreen(&self) -> windows_core::Result<SplashScreen> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SplashScreen)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "UI_ViewManagement")]
impl windows_core::RuntimeName for IViewSwitcherProvider {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IViewSwitcherProvider";
}
#[cfg(feature = "UI_ViewManagement")]
pub trait IViewSwitcherProvider_Impl: IActivatedEventArgs_Impl {
    fn ViewSwitcher(&self) -> windows_core::Result<super::super::UI::ViewManagement::ActivationViewSwitcher>;
}
#[cfg(feature = "UI_ViewManagement")]
impl IViewSwitcherProvider_Vtbl {
    pub const fn new<Identity: IViewSwitcherProvider_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ViewSwitcher<Identity: IViewSwitcherProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IViewSwitcherProvider_Impl::ViewSwitcher(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IInspectable_Vtbl::new::<Identity, IViewSwitcherProvider, OFFSET>(), ViewSwitcher: ViewSwitcher::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IViewSwitcherProvider as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IViewSwitcherProvider_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "UI_ViewManagement")]
    pub ViewSwitcher: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_ViewManagement"))]
    ViewSwitcher: usize,
}
windows_core::imp::define_interface!(IVoiceCommandActivatedEventArgs, IVoiceCommandActivatedEventArgs_Vtbl, 0xab92dcfd_8d43_4de6_9775_20704b581b00);
impl windows_core::RuntimeType for IVoiceCommandActivatedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(IVoiceCommandActivatedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(IVoiceCommandActivatedEventArgs, IActivatedEventArgs);
impl IVoiceCommandActivatedEventArgs {
    #[cfg(feature = "Media_SpeechRecognition")]
    pub fn Result(&self) -> windows_core::Result<super::super::Media::SpeechRecognition::SpeechRecognitionResult> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Result)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Kind(&self) -> windows_core::Result<ActivationKind> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> windows_core::Result<ApplicationExecutionState> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PreviousExecutionState)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SplashScreen(&self) -> windows_core::Result<SplashScreen> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SplashScreen)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Media_SpeechRecognition")]
impl windows_core::RuntimeName for IVoiceCommandActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IVoiceCommandActivatedEventArgs";
}
#[cfg(feature = "Media_SpeechRecognition")]
pub trait IVoiceCommandActivatedEventArgs_Impl: IActivatedEventArgs_Impl {
    fn Result(&self) -> windows_core::Result<super::super::Media::SpeechRecognition::SpeechRecognitionResult>;
}
#[cfg(feature = "Media_SpeechRecognition")]
impl IVoiceCommandActivatedEventArgs_Vtbl {
    pub const fn new<Identity: IVoiceCommandActivatedEventArgs_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Result<Identity: IVoiceCommandActivatedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IVoiceCommandActivatedEventArgs_Impl::Result(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IInspectable_Vtbl::new::<Identity, IVoiceCommandActivatedEventArgs, OFFSET>(), Result: Result::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVoiceCommandActivatedEventArgs as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IVoiceCommandActivatedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Media_SpeechRecognition")]
    pub Result: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Media_SpeechRecognition"))]
    Result: usize,
}
windows_core::imp::define_interface!(IWalletActionActivatedEventArgs, IWalletActionActivatedEventArgs_Vtbl, 0xfcfc027b_1a1a_4d22_923f_ae6f45fa52d9);
impl windows_core::RuntimeType for IWalletActionActivatedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(IWalletActionActivatedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(IWalletActionActivatedEventArgs, IActivatedEventArgs);
impl IWalletActionActivatedEventArgs {
    pub fn ItemId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ItemId)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(feature = "ApplicationModel_Wallet")]
    pub fn ActionKind(&self) -> windows_core::Result<super::Wallet::WalletActionKind> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ActionKind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ActionId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ActionId)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn Kind(&self) -> windows_core::Result<ActivationKind> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> windows_core::Result<ApplicationExecutionState> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PreviousExecutionState)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SplashScreen(&self) -> windows_core::Result<SplashScreen> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SplashScreen)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "ApplicationModel_Wallet")]
impl windows_core::RuntimeName for IWalletActionActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IWalletActionActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Wallet")]
pub trait IWalletActionActivatedEventArgs_Impl: IActivatedEventArgs_Impl {
    fn ItemId(&self) -> windows_core::Result<windows_core::HSTRING>;
    fn ActionKind(&self) -> windows_core::Result<super::Wallet::WalletActionKind>;
    fn ActionId(&self) -> windows_core::Result<windows_core::HSTRING>;
}
#[cfg(feature = "ApplicationModel_Wallet")]
impl IWalletActionActivatedEventArgs_Vtbl {
    pub const fn new<Identity: IWalletActionActivatedEventArgs_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ItemId<Identity: IWalletActionActivatedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWalletActionActivatedEventArgs_Impl::ItemId(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ActionKind<Identity: IWalletActionActivatedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut super::Wallet::WalletActionKind) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWalletActionActivatedEventArgs_Impl::ActionKind(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ActionId<Identity: IWalletActionActivatedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWalletActionActivatedEventArgs_Impl::ActionId(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IWalletActionActivatedEventArgs, OFFSET>(),
            ItemId: ItemId::<Identity, OFFSET>,
            ActionKind: ActionKind::<Identity, OFFSET>,
            ActionId: ActionId::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWalletActionActivatedEventArgs as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWalletActionActivatedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ItemId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "ApplicationModel_Wallet")]
    pub ActionKind: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::Wallet::WalletActionKind) -> windows_core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Wallet"))]
    ActionKind: usize,
    pub ActionId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWebAccountProviderActivatedEventArgs, IWebAccountProviderActivatedEventArgs_Vtbl, 0x72b71774_98ea_4ccf_9752_46d9051004f1);
impl windows_core::RuntimeType for IWebAccountProviderActivatedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(IWebAccountProviderActivatedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(IWebAccountProviderActivatedEventArgs, IActivatedEventArgs);
impl IWebAccountProviderActivatedEventArgs {
    #[cfg(feature = "Security_Authentication_Web_Provider")]
    pub fn Operation(&self) -> windows_core::Result<super::super::Security::Authentication::Web::Provider::IWebAccountProviderOperation> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Operation)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Kind(&self) -> windows_core::Result<ActivationKind> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> windows_core::Result<ApplicationExecutionState> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PreviousExecutionState)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SplashScreen(&self) -> windows_core::Result<SplashScreen> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SplashScreen)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Security_Authentication_Web_Provider")]
impl windows_core::RuntimeName for IWebAccountProviderActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IWebAccountProviderActivatedEventArgs";
}
#[cfg(feature = "Security_Authentication_Web_Provider")]
pub trait IWebAccountProviderActivatedEventArgs_Impl: IActivatedEventArgs_Impl {
    fn Operation(&self) -> windows_core::Result<super::super::Security::Authentication::Web::Provider::IWebAccountProviderOperation>;
}
#[cfg(feature = "Security_Authentication_Web_Provider")]
impl IWebAccountProviderActivatedEventArgs_Vtbl {
    pub const fn new<Identity: IWebAccountProviderActivatedEventArgs_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Operation<Identity: IWebAccountProviderActivatedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWebAccountProviderActivatedEventArgs_Impl::Operation(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IWebAccountProviderActivatedEventArgs, OFFSET>(),
            Operation: Operation::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWebAccountProviderActivatedEventArgs as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebAccountProviderActivatedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Security_Authentication_Web_Provider")]
    pub Operation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Security_Authentication_Web_Provider"))]
    Operation: usize,
}
windows_core::imp::define_interface!(IWebAuthenticationBrokerContinuationEventArgs, IWebAuthenticationBrokerContinuationEventArgs_Vtbl, 0x75dda3d4_7714_453d_b7ff_b95e3a1709da);
impl windows_core::RuntimeType for IWebAuthenticationBrokerContinuationEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(IWebAuthenticationBrokerContinuationEventArgs, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(IWebAuthenticationBrokerContinuationEventArgs, IActivatedEventArgs, IContinuationActivatedEventArgs);
impl IWebAuthenticationBrokerContinuationEventArgs {
    #[cfg(feature = "Security_Authentication_Web")]
    pub fn WebAuthenticationResult(&self) -> windows_core::Result<super::super::Security::Authentication::Web::WebAuthenticationResult> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).WebAuthenticationResult)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Kind(&self) -> windows_core::Result<ActivationKind> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> windows_core::Result<ApplicationExecutionState> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PreviousExecutionState)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SplashScreen(&self) -> windows_core::Result<SplashScreen> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SplashScreen)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn ContinuationData(&self) -> windows_core::Result<super::super::Foundation::Collections::ValueSet> {
        let this = &windows_core::Interface::cast::<IContinuationActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ContinuationData)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "Security_Authentication_Web"))]
impl windows_core::RuntimeName for IWebAuthenticationBrokerContinuationEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IWebAuthenticationBrokerContinuationEventArgs";
}
#[cfg(all(feature = "Foundation_Collections", feature = "Security_Authentication_Web"))]
pub trait IWebAuthenticationBrokerContinuationEventArgs_Impl: IActivatedEventArgs_Impl + IContinuationActivatedEventArgs_Impl {
    fn WebAuthenticationResult(&self) -> windows_core::Result<super::super::Security::Authentication::Web::WebAuthenticationResult>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "Security_Authentication_Web"))]
impl IWebAuthenticationBrokerContinuationEventArgs_Vtbl {
    pub const fn new<Identity: IWebAuthenticationBrokerContinuationEventArgs_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn WebAuthenticationResult<Identity: IWebAuthenticationBrokerContinuationEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWebAuthenticationBrokerContinuationEventArgs_Impl::WebAuthenticationResult(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IWebAuthenticationBrokerContinuationEventArgs, OFFSET>(),
            WebAuthenticationResult: WebAuthenticationResult::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWebAuthenticationBrokerContinuationEventArgs as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebAuthenticationBrokerContinuationEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Security_Authentication_Web")]
    pub WebAuthenticationResult: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Security_Authentication_Web"))]
    WebAuthenticationResult: usize,
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LaunchActivatedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(LaunchActivatedEventArgs, windows_core::IUnknown, windows_core::IInspectable, ILaunchActivatedEventArgs);
windows_core::imp::required_hierarchy!(LaunchActivatedEventArgs, IActivatedEventArgs, IActivatedEventArgsWithUser, IApplicationViewActivatedEventArgs, ILaunchActivatedEventArgs2, IPrelaunchActivatedEventArgs, IViewSwitcherProvider);
impl LaunchActivatedEventArgs {
    pub fn Kind(&self) -> windows_core::Result<ActivationKind> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> windows_core::Result<ApplicationExecutionState> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PreviousExecutionState)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SplashScreen(&self) -> windows_core::Result<SplashScreen> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SplashScreen)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "System")]
    pub fn User(&self) -> windows_core::Result<super::super::System::User> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).User)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CurrentlyShownApplicationViewId(&self) -> windows_core::Result<i32> {
        let this = &windows_core::Interface::cast::<IApplicationViewActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CurrentlyShownApplicationViewId)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Arguments(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Arguments)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn TileId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TileId)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn TileActivatedInfo(&self) -> windows_core::Result<TileActivatedInfo> {
        let this = &windows_core::Interface::cast::<ILaunchActivatedEventArgs2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TileActivatedInfo)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn PrelaunchActivated(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IPrelaunchActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PrelaunchActivated)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "UI_ViewManagement")]
    pub fn ViewSwitcher(&self) -> windows_core::Result<super::super::UI::ViewManagement::ActivationViewSwitcher> {
        let this = &windows_core::Interface::cast::<IViewSwitcherProvider>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ViewSwitcher)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for LaunchActivatedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ILaunchActivatedEventArgs>();
}
unsafe impl windows_core::Interface for LaunchActivatedEventArgs {
    type Vtable = <ILaunchActivatedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ILaunchActivatedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for LaunchActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.LaunchActivatedEventArgs";
}
unsafe impl Send for LaunchActivatedEventArgs {}
unsafe impl Sync for LaunchActivatedEventArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LockScreenActivatedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(LockScreenActivatedEventArgs, windows_core::IUnknown, windows_core::IInspectable, ILockScreenActivatedEventArgs);
windows_core::imp::required_hierarchy!(LockScreenActivatedEventArgs, IActivatedEventArgs, IActivatedEventArgsWithUser);
impl LockScreenActivatedEventArgs {
    pub fn Kind(&self) -> windows_core::Result<ActivationKind> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> windows_core::Result<ApplicationExecutionState> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PreviousExecutionState)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SplashScreen(&self) -> windows_core::Result<SplashScreen> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SplashScreen)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "System")]
    pub fn User(&self) -> windows_core::Result<super::super::System::User> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).User)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Info(&self) -> windows_core::Result<windows_core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Info)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for LockScreenActivatedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ILockScreenActivatedEventArgs>();
}
unsafe impl windows_core::Interface for LockScreenActivatedEventArgs {
    type Vtable = <ILockScreenActivatedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ILockScreenActivatedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for LockScreenActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.LockScreenActivatedEventArgs";
}
unsafe impl Send for LockScreenActivatedEventArgs {}
unsafe impl Sync for LockScreenActivatedEventArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LockScreenCallActivatedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(LockScreenCallActivatedEventArgs, windows_core::IUnknown, windows_core::IInspectable, ILockScreenCallActivatedEventArgs);
windows_core::imp::required_hierarchy!(LockScreenCallActivatedEventArgs, IActivatedEventArgs, IApplicationViewActivatedEventArgs, ILaunchActivatedEventArgs, IViewSwitcherProvider);
impl LockScreenCallActivatedEventArgs {
    pub fn Kind(&self) -> windows_core::Result<ActivationKind> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> windows_core::Result<ApplicationExecutionState> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PreviousExecutionState)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SplashScreen(&self) -> windows_core::Result<SplashScreen> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SplashScreen)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CurrentlyShownApplicationViewId(&self) -> windows_core::Result<i32> {
        let this = &windows_core::Interface::cast::<IApplicationViewActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CurrentlyShownApplicationViewId)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Arguments(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<ILaunchActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Arguments)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn TileId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<ILaunchActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TileId)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(feature = "ApplicationModel_Calls")]
    pub fn CallUI(&self) -> windows_core::Result<super::Calls::LockScreenCallUI> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CallUI)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "UI_ViewManagement")]
    pub fn ViewSwitcher(&self) -> windows_core::Result<super::super::UI::ViewManagement::ActivationViewSwitcher> {
        let this = &windows_core::Interface::cast::<IViewSwitcherProvider>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ViewSwitcher)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for LockScreenCallActivatedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ILockScreenCallActivatedEventArgs>();
}
unsafe impl windows_core::Interface for LockScreenCallActivatedEventArgs {
    type Vtable = <ILockScreenCallActivatedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ILockScreenCallActivatedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for LockScreenCallActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.LockScreenCallActivatedEventArgs";
}
unsafe impl Send for LockScreenCallActivatedEventArgs {}
unsafe impl Sync for LockScreenCallActivatedEventArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LockScreenComponentActivatedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(LockScreenComponentActivatedEventArgs, windows_core::IUnknown, windows_core::IInspectable, IActivatedEventArgs);
impl LockScreenComponentActivatedEventArgs {
    pub fn Kind(&self) -> windows_core::Result<ActivationKind> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> windows_core::Result<ApplicationExecutionState> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PreviousExecutionState)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SplashScreen(&self) -> windows_core::Result<SplashScreen> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SplashScreen)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for LockScreenComponentActivatedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IActivatedEventArgs>();
}
unsafe impl windows_core::Interface for LockScreenComponentActivatedEventArgs {
    type Vtable = <IActivatedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IActivatedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for LockScreenComponentActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.LockScreenComponentActivatedEventArgs";
}
unsafe impl Send for LockScreenComponentActivatedEventArgs {}
unsafe impl Sync for LockScreenComponentActivatedEventArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PhoneCallActivatedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PhoneCallActivatedEventArgs, windows_core::IUnknown, windows_core::IInspectable, IPhoneCallActivatedEventArgs);
windows_core::imp::required_hierarchy!(PhoneCallActivatedEventArgs, IActivatedEventArgs, IActivatedEventArgsWithUser);
impl PhoneCallActivatedEventArgs {
    pub fn Kind(&self) -> windows_core::Result<ActivationKind> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> windows_core::Result<ApplicationExecutionState> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PreviousExecutionState)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SplashScreen(&self) -> windows_core::Result<SplashScreen> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SplashScreen)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "System")]
    pub fn User(&self) -> windows_core::Result<super::super::System::User> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).User)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn LineId(&self) -> windows_core::Result<windows_core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LineId)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for PhoneCallActivatedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPhoneCallActivatedEventArgs>();
}
unsafe impl windows_core::Interface for PhoneCallActivatedEventArgs {
    type Vtable = <IPhoneCallActivatedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IPhoneCallActivatedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PhoneCallActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.PhoneCallActivatedEventArgs";
}
unsafe impl Send for PhoneCallActivatedEventArgs {}
unsafe impl Sync for PhoneCallActivatedEventArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PickerReturnedActivatedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PickerReturnedActivatedEventArgs, windows_core::IUnknown, windows_core::IInspectable, IPickerReturnedActivatedEventArgs);
windows_core::imp::required_hierarchy!(PickerReturnedActivatedEventArgs, IActivatedEventArgs);
impl PickerReturnedActivatedEventArgs {
    pub fn Kind(&self) -> windows_core::Result<ActivationKind> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> windows_core::Result<ApplicationExecutionState> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PreviousExecutionState)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SplashScreen(&self) -> windows_core::Result<SplashScreen> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SplashScreen)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn PickerOperationId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PickerOperationId)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
impl windows_core::RuntimeType for PickerReturnedActivatedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPickerReturnedActivatedEventArgs>();
}
unsafe impl windows_core::Interface for PickerReturnedActivatedEventArgs {
    type Vtable = <IPickerReturnedActivatedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IPickerReturnedActivatedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PickerReturnedActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.PickerReturnedActivatedEventArgs";
}
unsafe impl Send for PickerReturnedActivatedEventArgs {}
unsafe impl Sync for PickerReturnedActivatedEventArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Print3DWorkflowActivatedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(Print3DWorkflowActivatedEventArgs, windows_core::IUnknown, windows_core::IInspectable, IPrint3DWorkflowActivatedEventArgs);
windows_core::imp::required_hierarchy!(Print3DWorkflowActivatedEventArgs, IActivatedEventArgs);
impl Print3DWorkflowActivatedEventArgs {
    pub fn Kind(&self) -> windows_core::Result<ActivationKind> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> windows_core::Result<ApplicationExecutionState> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PreviousExecutionState)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SplashScreen(&self) -> windows_core::Result<SplashScreen> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SplashScreen)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Devices_Printers_Extensions")]
    pub fn Workflow(&self) -> windows_core::Result<super::super::Devices::Printers::Extensions::Print3DWorkflow> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Workflow)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for Print3DWorkflowActivatedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPrint3DWorkflowActivatedEventArgs>();
}
unsafe impl windows_core::Interface for Print3DWorkflowActivatedEventArgs {
    type Vtable = <IPrint3DWorkflowActivatedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IPrint3DWorkflowActivatedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for Print3DWorkflowActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.Print3DWorkflowActivatedEventArgs";
}
unsafe impl Send for Print3DWorkflowActivatedEventArgs {}
unsafe impl Sync for Print3DWorkflowActivatedEventArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PrintTaskSettingsActivatedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PrintTaskSettingsActivatedEventArgs, windows_core::IUnknown, windows_core::IInspectable, IPrintTaskSettingsActivatedEventArgs);
windows_core::imp::required_hierarchy!(PrintTaskSettingsActivatedEventArgs, IActivatedEventArgs);
impl PrintTaskSettingsActivatedEventArgs {
    pub fn Kind(&self) -> windows_core::Result<ActivationKind> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> windows_core::Result<ApplicationExecutionState> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PreviousExecutionState)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SplashScreen(&self) -> windows_core::Result<SplashScreen> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SplashScreen)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Devices_Printers_Extensions")]
    pub fn Configuration(&self) -> windows_core::Result<super::super::Devices::Printers::Extensions::PrintTaskConfiguration> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Configuration)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for PrintTaskSettingsActivatedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPrintTaskSettingsActivatedEventArgs>();
}
unsafe impl windows_core::Interface for PrintTaskSettingsActivatedEventArgs {
    type Vtable = <IPrintTaskSettingsActivatedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IPrintTaskSettingsActivatedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PrintTaskSettingsActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.PrintTaskSettingsActivatedEventArgs";
}
unsafe impl Send for PrintTaskSettingsActivatedEventArgs {}
unsafe impl Sync for PrintTaskSettingsActivatedEventArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProtocolActivatedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ProtocolActivatedEventArgs, windows_core::IUnknown, windows_core::IInspectable, IProtocolActivatedEventArgs);
windows_core::imp::required_hierarchy!(ProtocolActivatedEventArgs, IActivatedEventArgs, IActivatedEventArgsWithUser, IApplicationViewActivatedEventArgs, IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData, IViewSwitcherProvider);
impl ProtocolActivatedEventArgs {
    pub fn Kind(&self) -> windows_core::Result<ActivationKind> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> windows_core::Result<ApplicationExecutionState> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PreviousExecutionState)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SplashScreen(&self) -> windows_core::Result<SplashScreen> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SplashScreen)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "System")]
    pub fn User(&self) -> windows_core::Result<super::super::System::User> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).User)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CurrentlyShownApplicationViewId(&self) -> windows_core::Result<i32> {
        let this = &windows_core::Interface::cast::<IApplicationViewActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CurrentlyShownApplicationViewId)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Uri(&self) -> windows_core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Uri)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CallerPackageFamilyName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CallerPackageFamilyName)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Data(&self) -> windows_core::Result<super::super::Foundation::Collections::ValueSet> {
        let this = &windows_core::Interface::cast::<IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Data)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "UI_ViewManagement")]
    pub fn ViewSwitcher(&self) -> windows_core::Result<super::super::UI::ViewManagement::ActivationViewSwitcher> {
        let this = &windows_core::Interface::cast::<IViewSwitcherProvider>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ViewSwitcher)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for ProtocolActivatedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IProtocolActivatedEventArgs>();
}
unsafe impl windows_core::Interface for ProtocolActivatedEventArgs {
    type Vtable = <IProtocolActivatedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IProtocolActivatedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ProtocolActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.ProtocolActivatedEventArgs";
}
unsafe impl Send for ProtocolActivatedEventArgs {}
unsafe impl Sync for ProtocolActivatedEventArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProtocolForResultsActivatedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ProtocolForResultsActivatedEventArgs, windows_core::IUnknown, windows_core::IInspectable, IProtocolForResultsActivatedEventArgs);
windows_core::imp::required_hierarchy!(ProtocolForResultsActivatedEventArgs, IActivatedEventArgs, IActivatedEventArgsWithUser, IApplicationViewActivatedEventArgs, IProtocolActivatedEventArgs, IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData, IViewSwitcherProvider);
impl ProtocolForResultsActivatedEventArgs {
    pub fn Kind(&self) -> windows_core::Result<ActivationKind> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> windows_core::Result<ApplicationExecutionState> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PreviousExecutionState)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SplashScreen(&self) -> windows_core::Result<SplashScreen> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SplashScreen)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "System")]
    pub fn User(&self) -> windows_core::Result<super::super::System::User> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).User)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CurrentlyShownApplicationViewId(&self) -> windows_core::Result<i32> {
        let this = &windows_core::Interface::cast::<IApplicationViewActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CurrentlyShownApplicationViewId)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Uri(&self) -> windows_core::Result<super::super::Foundation::Uri> {
        let this = &windows_core::Interface::cast::<IProtocolActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Uri)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CallerPackageFamilyName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CallerPackageFamilyName)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Data(&self) -> windows_core::Result<super::super::Foundation::Collections::ValueSet> {
        let this = &windows_core::Interface::cast::<IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Data)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "System")]
    pub fn ProtocolForResultsOperation(&self) -> windows_core::Result<super::super::System::ProtocolForResultsOperation> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ProtocolForResultsOperation)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "UI_ViewManagement")]
    pub fn ViewSwitcher(&self) -> windows_core::Result<super::super::UI::ViewManagement::ActivationViewSwitcher> {
        let this = &windows_core::Interface::cast::<IViewSwitcherProvider>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ViewSwitcher)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for ProtocolForResultsActivatedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IProtocolForResultsActivatedEventArgs>();
}
unsafe impl windows_core::Interface for ProtocolForResultsActivatedEventArgs {
    type Vtable = <IProtocolForResultsActivatedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IProtocolForResultsActivatedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ProtocolForResultsActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.ProtocolForResultsActivatedEventArgs";
}
unsafe impl Send for ProtocolForResultsActivatedEventArgs {}
unsafe impl Sync for ProtocolForResultsActivatedEventArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RestrictedLaunchActivatedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(RestrictedLaunchActivatedEventArgs, windows_core::IUnknown, windows_core::IInspectable, IRestrictedLaunchActivatedEventArgs);
windows_core::imp::required_hierarchy!(RestrictedLaunchActivatedEventArgs, IActivatedEventArgs, IActivatedEventArgsWithUser);
impl RestrictedLaunchActivatedEventArgs {
    pub fn Kind(&self) -> windows_core::Result<ActivationKind> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> windows_core::Result<ApplicationExecutionState> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PreviousExecutionState)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SplashScreen(&self) -> windows_core::Result<SplashScreen> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SplashScreen)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "System")]
    pub fn User(&self) -> windows_core::Result<super::super::System::User> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).User)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SharedContext(&self) -> windows_core::Result<windows_core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SharedContext)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for RestrictedLaunchActivatedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IRestrictedLaunchActivatedEventArgs>();
}
unsafe impl windows_core::Interface for RestrictedLaunchActivatedEventArgs {
    type Vtable = <IRestrictedLaunchActivatedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IRestrictedLaunchActivatedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for RestrictedLaunchActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.RestrictedLaunchActivatedEventArgs";
}
unsafe impl Send for RestrictedLaunchActivatedEventArgs {}
unsafe impl Sync for RestrictedLaunchActivatedEventArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SearchActivatedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(SearchActivatedEventArgs, windows_core::IUnknown, windows_core::IInspectable, ISearchActivatedEventArgs);
windows_core::imp::required_hierarchy!(SearchActivatedEventArgs, IActivatedEventArgs, IActivatedEventArgsWithUser, IApplicationViewActivatedEventArgs, ISearchActivatedEventArgsWithLinguisticDetails, IViewSwitcherProvider);
impl SearchActivatedEventArgs {
    pub fn Kind(&self) -> windows_core::Result<ActivationKind> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> windows_core::Result<ApplicationExecutionState> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PreviousExecutionState)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SplashScreen(&self) -> windows_core::Result<SplashScreen> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SplashScreen)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "System")]
    pub fn User(&self) -> windows_core::Result<super::super::System::User> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).User)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CurrentlyShownApplicationViewId(&self) -> windows_core::Result<i32> {
        let this = &windows_core::Interface::cast::<IApplicationViewActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CurrentlyShownApplicationViewId)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn QueryText(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).QueryText)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn Language(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Language)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(feature = "ApplicationModel_Search")]
    pub fn LinguisticDetails(&self) -> windows_core::Result<super::Search::SearchPaneQueryLinguisticDetails> {
        let this = &windows_core::Interface::cast::<ISearchActivatedEventArgsWithLinguisticDetails>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LinguisticDetails)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "UI_ViewManagement")]
    pub fn ViewSwitcher(&self) -> windows_core::Result<super::super::UI::ViewManagement::ActivationViewSwitcher> {
        let this = &windows_core::Interface::cast::<IViewSwitcherProvider>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ViewSwitcher)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for SearchActivatedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ISearchActivatedEventArgs>();
}
unsafe impl windows_core::Interface for SearchActivatedEventArgs {
    type Vtable = <ISearchActivatedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ISearchActivatedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for SearchActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.SearchActivatedEventArgs";
}
unsafe impl Send for SearchActivatedEventArgs {}
unsafe impl Sync for SearchActivatedEventArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ShareTargetActivatedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ShareTargetActivatedEventArgs, windows_core::IUnknown, windows_core::IInspectable, IShareTargetActivatedEventArgs);
windows_core::imp::required_hierarchy!(ShareTargetActivatedEventArgs, IActivatedEventArgs, IActivatedEventArgsWithUser);
impl ShareTargetActivatedEventArgs {
    pub fn Kind(&self) -> windows_core::Result<ActivationKind> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> windows_core::Result<ApplicationExecutionState> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PreviousExecutionState)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SplashScreen(&self) -> windows_core::Result<SplashScreen> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SplashScreen)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "System")]
    pub fn User(&self) -> windows_core::Result<super::super::System::User> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).User)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "ApplicationModel_DataTransfer_ShareTarget")]
    pub fn ShareOperation(&self) -> windows_core::Result<super::DataTransfer::ShareTarget::ShareOperation> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ShareOperation)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for ShareTargetActivatedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IShareTargetActivatedEventArgs>();
}
unsafe impl windows_core::Interface for ShareTargetActivatedEventArgs {
    type Vtable = <IShareTargetActivatedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IShareTargetActivatedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ShareTargetActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.ShareTargetActivatedEventArgs";
}
unsafe impl Send for ShareTargetActivatedEventArgs {}
unsafe impl Sync for ShareTargetActivatedEventArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SplashScreen(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(SplashScreen, windows_core::IUnknown, windows_core::IInspectable);
impl SplashScreen {
    pub fn ImageLocation(&self) -> windows_core::Result<super::super::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ImageLocation)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Dismissed<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<SplashScreen, windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Dismissed)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveDismissed(&self, cookie: i64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveDismissed)(windows_core::Interface::as_raw(this), cookie).ok() }
    }
}
impl windows_core::RuntimeType for SplashScreen {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ISplashScreen>();
}
unsafe impl windows_core::Interface for SplashScreen {
    type Vtable = <ISplashScreen as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ISplashScreen as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for SplashScreen {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.SplashScreen";
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StartupTaskActivatedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(StartupTaskActivatedEventArgs, windows_core::IUnknown, windows_core::IInspectable, IStartupTaskActivatedEventArgs);
windows_core::imp::required_hierarchy!(StartupTaskActivatedEventArgs, IActivatedEventArgs, IActivatedEventArgsWithUser);
impl StartupTaskActivatedEventArgs {
    pub fn Kind(&self) -> windows_core::Result<ActivationKind> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> windows_core::Result<ApplicationExecutionState> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PreviousExecutionState)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SplashScreen(&self) -> windows_core::Result<SplashScreen> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SplashScreen)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "System")]
    pub fn User(&self) -> windows_core::Result<super::super::System::User> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).User)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn TaskId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TaskId)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
impl windows_core::RuntimeType for StartupTaskActivatedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IStartupTaskActivatedEventArgs>();
}
unsafe impl windows_core::Interface for StartupTaskActivatedEventArgs {
    type Vtable = <IStartupTaskActivatedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IStartupTaskActivatedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for StartupTaskActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.StartupTaskActivatedEventArgs";
}
unsafe impl Send for StartupTaskActivatedEventArgs {}
unsafe impl Sync for StartupTaskActivatedEventArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TileActivatedInfo(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(TileActivatedInfo, windows_core::IUnknown, windows_core::IInspectable);
impl TileActivatedInfo {
    #[cfg(feature = "UI_Notifications")]
    pub fn RecentlyShownNotifications(&self) -> windows_core::Result<windows_collections::IVectorView<super::super::UI::Notifications::ShownTileNotification>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RecentlyShownNotifications)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for TileActivatedInfo {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ITileActivatedInfo>();
}
unsafe impl windows_core::Interface for TileActivatedInfo {
    type Vtable = <ITileActivatedInfo as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ITileActivatedInfo as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for TileActivatedInfo {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.TileActivatedInfo";
}
unsafe impl Send for TileActivatedInfo {}
unsafe impl Sync for TileActivatedInfo {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ToastNotificationActivatedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ToastNotificationActivatedEventArgs, windows_core::IUnknown, windows_core::IInspectable, IToastNotificationActivatedEventArgs);
windows_core::imp::required_hierarchy!(ToastNotificationActivatedEventArgs, IActivatedEventArgs, IActivatedEventArgsWithUser, IApplicationViewActivatedEventArgs);
impl ToastNotificationActivatedEventArgs {
    pub fn Kind(&self) -> windows_core::Result<ActivationKind> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> windows_core::Result<ApplicationExecutionState> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PreviousExecutionState)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SplashScreen(&self) -> windows_core::Result<SplashScreen> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SplashScreen)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "System")]
    pub fn User(&self) -> windows_core::Result<super::super::System::User> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).User)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CurrentlyShownApplicationViewId(&self) -> windows_core::Result<i32> {
        let this = &windows_core::Interface::cast::<IApplicationViewActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CurrentlyShownApplicationViewId)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Argument(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Argument)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn UserInput(&self) -> windows_core::Result<super::super::Foundation::Collections::ValueSet> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UserInput)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for ToastNotificationActivatedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IToastNotificationActivatedEventArgs>();
}
unsafe impl windows_core::Interface for ToastNotificationActivatedEventArgs {
    type Vtable = <IToastNotificationActivatedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IToastNotificationActivatedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ToastNotificationActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.ToastNotificationActivatedEventArgs";
}
unsafe impl Send for ToastNotificationActivatedEventArgs {}
unsafe impl Sync for ToastNotificationActivatedEventArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UserDataAccountProviderActivatedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(UserDataAccountProviderActivatedEventArgs, windows_core::IUnknown, windows_core::IInspectable, IUserDataAccountProviderActivatedEventArgs);
windows_core::imp::required_hierarchy!(UserDataAccountProviderActivatedEventArgs, IActivatedEventArgs);
impl UserDataAccountProviderActivatedEventArgs {
    pub fn Kind(&self) -> windows_core::Result<ActivationKind> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> windows_core::Result<ApplicationExecutionState> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PreviousExecutionState)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SplashScreen(&self) -> windows_core::Result<SplashScreen> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SplashScreen)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "ApplicationModel_UserDataAccounts_Provider")]
    pub fn Operation(&self) -> windows_core::Result<super::UserDataAccounts::Provider::IUserDataAccountProviderOperation> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Operation)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for UserDataAccountProviderActivatedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IUserDataAccountProviderActivatedEventArgs>();
}
unsafe impl windows_core::Interface for UserDataAccountProviderActivatedEventArgs {
    type Vtable = <IUserDataAccountProviderActivatedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IUserDataAccountProviderActivatedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for UserDataAccountProviderActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.UserDataAccountProviderActivatedEventArgs";
}
unsafe impl Send for UserDataAccountProviderActivatedEventArgs {}
unsafe impl Sync for UserDataAccountProviderActivatedEventArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VoiceCommandActivatedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(VoiceCommandActivatedEventArgs, windows_core::IUnknown, windows_core::IInspectable, IVoiceCommandActivatedEventArgs);
windows_core::imp::required_hierarchy!(VoiceCommandActivatedEventArgs, IActivatedEventArgs, IActivatedEventArgsWithUser);
impl VoiceCommandActivatedEventArgs {
    pub fn Kind(&self) -> windows_core::Result<ActivationKind> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> windows_core::Result<ApplicationExecutionState> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PreviousExecutionState)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SplashScreen(&self) -> windows_core::Result<SplashScreen> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SplashScreen)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "System")]
    pub fn User(&self) -> windows_core::Result<super::super::System::User> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).User)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Media_SpeechRecognition")]
    pub fn Result(&self) -> windows_core::Result<super::super::Media::SpeechRecognition::SpeechRecognitionResult> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Result)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for VoiceCommandActivatedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IVoiceCommandActivatedEventArgs>();
}
unsafe impl windows_core::Interface for VoiceCommandActivatedEventArgs {
    type Vtable = <IVoiceCommandActivatedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IVoiceCommandActivatedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for VoiceCommandActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.VoiceCommandActivatedEventArgs";
}
unsafe impl Send for VoiceCommandActivatedEventArgs {}
unsafe impl Sync for VoiceCommandActivatedEventArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WalletActionActivatedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(WalletActionActivatedEventArgs, windows_core::IUnknown, windows_core::IInspectable, IWalletActionActivatedEventArgs);
windows_core::imp::required_hierarchy!(WalletActionActivatedEventArgs, IActivatedEventArgs);
impl WalletActionActivatedEventArgs {
    pub fn Kind(&self) -> windows_core::Result<ActivationKind> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> windows_core::Result<ApplicationExecutionState> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PreviousExecutionState)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SplashScreen(&self) -> windows_core::Result<SplashScreen> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SplashScreen)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ItemId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ItemId)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(feature = "ApplicationModel_Wallet")]
    pub fn ActionKind(&self) -> windows_core::Result<super::Wallet::WalletActionKind> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ActionKind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ActionId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ActionId)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
impl windows_core::RuntimeType for WalletActionActivatedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IWalletActionActivatedEventArgs>();
}
unsafe impl windows_core::Interface for WalletActionActivatedEventArgs {
    type Vtable = <IWalletActionActivatedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IWalletActionActivatedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for WalletActionActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.WalletActionActivatedEventArgs";
}
unsafe impl Send for WalletActionActivatedEventArgs {}
unsafe impl Sync for WalletActionActivatedEventArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WebAccountProviderActivatedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(WebAccountProviderActivatedEventArgs, windows_core::IUnknown, windows_core::IInspectable, IWebAccountProviderActivatedEventArgs);
windows_core::imp::required_hierarchy!(WebAccountProviderActivatedEventArgs, IActivatedEventArgs, IActivatedEventArgsWithUser);
impl WebAccountProviderActivatedEventArgs {
    pub fn Kind(&self) -> windows_core::Result<ActivationKind> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> windows_core::Result<ApplicationExecutionState> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PreviousExecutionState)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SplashScreen(&self) -> windows_core::Result<SplashScreen> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SplashScreen)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "System")]
    pub fn User(&self) -> windows_core::Result<super::super::System::User> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).User)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Security_Authentication_Web_Provider")]
    pub fn Operation(&self) -> windows_core::Result<super::super::Security::Authentication::Web::Provider::IWebAccountProviderOperation> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Operation)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for WebAccountProviderActivatedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IWebAccountProviderActivatedEventArgs>();
}
unsafe impl windows_core::Interface for WebAccountProviderActivatedEventArgs {
    type Vtable = <IWebAccountProviderActivatedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IWebAccountProviderActivatedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for WebAccountProviderActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.WebAccountProviderActivatedEventArgs";
}
unsafe impl Send for WebAccountProviderActivatedEventArgs {}
unsafe impl Sync for WebAccountProviderActivatedEventArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WebAuthenticationBrokerContinuationEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(WebAuthenticationBrokerContinuationEventArgs, windows_core::IUnknown, windows_core::IInspectable, IWebAuthenticationBrokerContinuationEventArgs);
windows_core::imp::required_hierarchy!(WebAuthenticationBrokerContinuationEventArgs, IActivatedEventArgs, IContinuationActivatedEventArgs);
impl WebAuthenticationBrokerContinuationEventArgs {
    pub fn Kind(&self) -> windows_core::Result<ActivationKind> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> windows_core::Result<ApplicationExecutionState> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PreviousExecutionState)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SplashScreen(&self) -> windows_core::Result<SplashScreen> {
        let this = &windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SplashScreen)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn ContinuationData(&self) -> windows_core::Result<super::super::Foundation::Collections::ValueSet> {
        let this = &windows_core::Interface::cast::<IContinuationActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ContinuationData)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Security_Authentication_Web")]
    pub fn WebAuthenticationResult(&self) -> windows_core::Result<super::super::Security::Authentication::Web::WebAuthenticationResult> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).WebAuthenticationResult)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for WebAuthenticationBrokerContinuationEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IWebAuthenticationBrokerContinuationEventArgs>();
}
unsafe impl windows_core::Interface for WebAuthenticationBrokerContinuationEventArgs {
    type Vtable = <IWebAuthenticationBrokerContinuationEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IWebAuthenticationBrokerContinuationEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for WebAuthenticationBrokerContinuationEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.WebAuthenticationBrokerContinuationEventArgs";
}
unsafe impl Send for WebAuthenticationBrokerContinuationEventArgs {}
unsafe impl Sync for WebAuthenticationBrokerContinuationEventArgs {}
