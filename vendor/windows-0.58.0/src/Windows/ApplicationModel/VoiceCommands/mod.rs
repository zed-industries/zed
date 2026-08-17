windows_core::imp::define_interface!(IVoiceCommand, IVoiceCommand_Vtbl, 0x936f5273_ec82_42a6_a55c_d2d79ec6f920);
impl windows_core::RuntimeType for IVoiceCommand {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IVoiceCommand_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CommandName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Properties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Properties: usize,
    #[cfg(feature = "Media_SpeechRecognition")]
    pub SpeechRecognitionResult: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Media_SpeechRecognition"))]
    SpeechRecognitionResult: usize,
}
windows_core::imp::define_interface!(IVoiceCommandCompletedEventArgs, IVoiceCommandCompletedEventArgs_Vtbl, 0xc85e675d_fe42_432c_9907_09df9fcf64e8);
impl windows_core::RuntimeType for IVoiceCommandCompletedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IVoiceCommandCompletedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Reason: unsafe extern "system" fn(*mut core::ffi::c_void, *mut VoiceCommandCompletionReason) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IVoiceCommandConfirmationResult, IVoiceCommandConfirmationResult_Vtbl, 0xa022593e_8221_4526_b083_840972262247);
impl windows_core::RuntimeType for IVoiceCommandConfirmationResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IVoiceCommandConfirmationResult_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Confirmed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IVoiceCommandContentTile, IVoiceCommandContentTile_Vtbl, 0x3eefe9f0_b8c7_4c76_a0de_1607895ee327);
impl windows_core::RuntimeType for IVoiceCommandContentTile {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IVoiceCommandContentTile_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Title: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetTitle: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub TextLine1: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetTextLine1: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub TextLine2: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetTextLine2: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub TextLine3: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetTextLine3: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    #[cfg(feature = "Storage")]
    pub Image: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage"))]
    Image: usize,
    #[cfg(feature = "Storage")]
    pub SetImage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage"))]
    SetImage: usize,
    pub AppContext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetAppContext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AppLaunchArgument: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetAppLaunchArgument: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub ContentTileType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut VoiceCommandContentTileType) -> windows_core::HRESULT,
    pub SetContentTileType: unsafe extern "system" fn(*mut core::ffi::c_void, VoiceCommandContentTileType) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IVoiceCommandDefinition, IVoiceCommandDefinition_Vtbl, 0x7972aad0_0974_4979_984b_cb8959cd61ae);
impl windows_core::RuntimeType for IVoiceCommandDefinition {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IVoiceCommandDefinition_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Language: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub SetPhraseListAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SetPhraseListAsync: usize,
}
windows_core::imp::define_interface!(IVoiceCommandDefinitionManagerStatics, IVoiceCommandDefinitionManagerStatics_Vtbl, 0x8fe7a69e_067e_4f16_a18c_5b17e9499940);
impl windows_core::RuntimeType for IVoiceCommandDefinitionManagerStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IVoiceCommandDefinitionManagerStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Storage")]
    pub InstallCommandDefinitionsFromStorageFileAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage"))]
    InstallCommandDefinitionsFromStorageFileAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub InstalledCommandDefinitions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    InstalledCommandDefinitions: usize,
}
windows_core::imp::define_interface!(IVoiceCommandDisambiguationResult, IVoiceCommandDisambiguationResult_Vtbl, 0xecc68cfe_c9ac_45df_a8ea_feea08ef9c5e);
impl windows_core::RuntimeType for IVoiceCommandDisambiguationResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IVoiceCommandDisambiguationResult_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SelectedItem: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IVoiceCommandResponse, IVoiceCommandResponse_Vtbl, 0x0284b30e_8a3b_4cc4_a6a1_cad5be2716b5);
impl windows_core::RuntimeType for IVoiceCommandResponse {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IVoiceCommandResponse_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Message: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetMessage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RepeatMessage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetRepeatMessage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AppLaunchArgument: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetAppLaunchArgument: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub VoiceCommandContentTiles: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    VoiceCommandContentTiles: usize,
}
windows_core::imp::define_interface!(IVoiceCommandResponseStatics, IVoiceCommandResponseStatics_Vtbl, 0x2932f813_0d3b_49f2_96dd_625019bd3b5d);
impl windows_core::RuntimeType for IVoiceCommandResponseStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IVoiceCommandResponseStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub MaxSupportedVoiceCommandContentTiles: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub CreateResponse: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateResponseWithTiles: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateResponseWithTiles: usize,
    pub CreateResponseForPrompt: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateResponseForPromptWithTiles: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateResponseForPromptWithTiles: usize,
}
windows_core::imp::define_interface!(IVoiceCommandServiceConnection, IVoiceCommandServiceConnection_Vtbl, 0xd894bb9f_21da_44a4_98a2_fb131920a9cc);
impl windows_core::RuntimeType for IVoiceCommandServiceConnection {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IVoiceCommandServiceConnection_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetVoiceCommandAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RequestConfirmationAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RequestDisambiguationAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ReportProgressAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ReportSuccessAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ReportFailureAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RequestAppLaunchAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Globalization")]
    pub Language: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Globalization"))]
    Language: usize,
    pub VoiceCommandCompleted: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveVoiceCommandCompleted: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IVoiceCommandServiceConnectionStatics, IVoiceCommandServiceConnectionStatics_Vtbl, 0x370ebffb_2d34_42df_8770_074d0f334697);
impl windows_core::RuntimeType for IVoiceCommandServiceConnectionStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IVoiceCommandServiceConnectionStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "ApplicationModel_AppService")]
    pub FromAppServiceTriggerDetails: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_AppService"))]
    FromAppServiceTriggerDetails: usize,
}
windows_core::imp::define_interface!(IVoiceCommandUserMessage, IVoiceCommandUserMessage_Vtbl, 0x674eb3c0_44f6_4f07_b979_4c723fc08597);
impl windows_core::RuntimeType for IVoiceCommandUserMessage {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IVoiceCommandUserMessage_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub DisplayMessage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetDisplayMessage: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SpokenMessage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetSpokenMessage: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct VoiceCommand(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(VoiceCommand, windows_core::IUnknown, windows_core::IInspectable);
impl VoiceCommand {
    pub fn CommandName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CommandName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Properties(&self) -> windows_core::Result<super::super::Foundation::Collections::IMapView<windows_core::HSTRING, super::super::Foundation::Collections::IVectorView<windows_core::HSTRING>>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Properties)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Media_SpeechRecognition")]
    pub fn SpeechRecognitionResult(&self) -> windows_core::Result<super::super::Media::SpeechRecognition::SpeechRecognitionResult> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SpeechRecognitionResult)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for VoiceCommand {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IVoiceCommand>();
}
unsafe impl windows_core::Interface for VoiceCommand {
    type Vtable = IVoiceCommand_Vtbl;
    const IID: windows_core::GUID = <IVoiceCommand as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for VoiceCommand {
    const NAME: &'static str = "Windows.ApplicationModel.VoiceCommands.VoiceCommand";
}
unsafe impl Send for VoiceCommand {}
unsafe impl Sync for VoiceCommand {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct VoiceCommandCompletedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(VoiceCommandCompletedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl VoiceCommandCompletedEventArgs {
    pub fn Reason(&self) -> windows_core::Result<VoiceCommandCompletionReason> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Reason)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for VoiceCommandCompletedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IVoiceCommandCompletedEventArgs>();
}
unsafe impl windows_core::Interface for VoiceCommandCompletedEventArgs {
    type Vtable = IVoiceCommandCompletedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IVoiceCommandCompletedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for VoiceCommandCompletedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.VoiceCommands.VoiceCommandCompletedEventArgs";
}
unsafe impl Send for VoiceCommandCompletedEventArgs {}
unsafe impl Sync for VoiceCommandCompletedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct VoiceCommandConfirmationResult(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(VoiceCommandConfirmationResult, windows_core::IUnknown, windows_core::IInspectable);
impl VoiceCommandConfirmationResult {
    pub fn Confirmed(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Confirmed)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for VoiceCommandConfirmationResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IVoiceCommandConfirmationResult>();
}
unsafe impl windows_core::Interface for VoiceCommandConfirmationResult {
    type Vtable = IVoiceCommandConfirmationResult_Vtbl;
    const IID: windows_core::GUID = <IVoiceCommandConfirmationResult as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for VoiceCommandConfirmationResult {
    const NAME: &'static str = "Windows.ApplicationModel.VoiceCommands.VoiceCommandConfirmationResult";
}
unsafe impl Send for VoiceCommandConfirmationResult {}
unsafe impl Sync for VoiceCommandConfirmationResult {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct VoiceCommandContentTile(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(VoiceCommandContentTile, windows_core::IUnknown, windows_core::IInspectable);
impl VoiceCommandContentTile {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<VoiceCommandContentTile, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Title(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Title)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetTitle(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetTitle)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn TextLine1(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TextLine1)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetTextLine1(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetTextLine1)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn TextLine2(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TextLine2)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetTextLine2(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetTextLine2)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn TextLine3(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TextLine3)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetTextLine3(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetTextLine3)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    #[cfg(feature = "Storage")]
    pub fn Image(&self) -> windows_core::Result<super::super::Storage::IStorageFile> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Image)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage")]
    pub fn SetImage<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Storage::IStorageFile>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetImage)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn AppContext(&self) -> windows_core::Result<windows_core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AppContext)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetAppContext<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetAppContext)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn AppLaunchArgument(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AppLaunchArgument)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetAppLaunchArgument(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetAppLaunchArgument)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn ContentTileType(&self) -> windows_core::Result<VoiceCommandContentTileType> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ContentTileType)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetContentTileType(&self, value: VoiceCommandContentTileType) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetContentTileType)(windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl windows_core::RuntimeType for VoiceCommandContentTile {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IVoiceCommandContentTile>();
}
unsafe impl windows_core::Interface for VoiceCommandContentTile {
    type Vtable = IVoiceCommandContentTile_Vtbl;
    const IID: windows_core::GUID = <IVoiceCommandContentTile as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for VoiceCommandContentTile {
    const NAME: &'static str = "Windows.ApplicationModel.VoiceCommands.VoiceCommandContentTile";
}
unsafe impl Send for VoiceCommandContentTile {}
unsafe impl Sync for VoiceCommandContentTile {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct VoiceCommandDefinition(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(VoiceCommandDefinition, windows_core::IUnknown, windows_core::IInspectable);
impl VoiceCommandDefinition {
    pub fn Language(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Language)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Name(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Name)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn SetPhraseListAsync<P0>(&self, phraselistname: &windows_core::HSTRING, phraselist: P0) -> windows_core::Result<super::super::Foundation::IAsyncAction>
    where
        P0: windows_core::Param<super::super::Foundation::Collections::IIterable<windows_core::HSTRING>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SetPhraseListAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(phraselistname), phraselist.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for VoiceCommandDefinition {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IVoiceCommandDefinition>();
}
unsafe impl windows_core::Interface for VoiceCommandDefinition {
    type Vtable = IVoiceCommandDefinition_Vtbl;
    const IID: windows_core::GUID = <IVoiceCommandDefinition as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for VoiceCommandDefinition {
    const NAME: &'static str = "Windows.ApplicationModel.VoiceCommands.VoiceCommandDefinition";
}
unsafe impl Send for VoiceCommandDefinition {}
unsafe impl Sync for VoiceCommandDefinition {}
pub struct VoiceCommandDefinitionManager;
impl VoiceCommandDefinitionManager {
    #[cfg(feature = "Storage")]
    pub fn InstallCommandDefinitionsFromStorageFileAsync<P0>(file: P0) -> windows_core::Result<super::super::Foundation::IAsyncAction>
    where
        P0: windows_core::Param<super::super::Storage::StorageFile>,
    {
        Self::IVoiceCommandDefinitionManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).InstallCommandDefinitionsFromStorageFileAsync)(windows_core::Interface::as_raw(this), file.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn InstalledCommandDefinitions() -> windows_core::Result<super::super::Foundation::Collections::IMapView<windows_core::HSTRING, VoiceCommandDefinition>> {
        Self::IVoiceCommandDefinitionManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).InstalledCommandDefinitions)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IVoiceCommandDefinitionManagerStatics<R, F: FnOnce(&IVoiceCommandDefinitionManagerStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<VoiceCommandDefinitionManager, IVoiceCommandDefinitionManagerStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeName for VoiceCommandDefinitionManager {
    const NAME: &'static str = "Windows.ApplicationModel.VoiceCommands.VoiceCommandDefinitionManager";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct VoiceCommandDisambiguationResult(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(VoiceCommandDisambiguationResult, windows_core::IUnknown, windows_core::IInspectable);
impl VoiceCommandDisambiguationResult {
    pub fn SelectedItem(&self) -> windows_core::Result<VoiceCommandContentTile> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SelectedItem)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for VoiceCommandDisambiguationResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IVoiceCommandDisambiguationResult>();
}
unsafe impl windows_core::Interface for VoiceCommandDisambiguationResult {
    type Vtable = IVoiceCommandDisambiguationResult_Vtbl;
    const IID: windows_core::GUID = <IVoiceCommandDisambiguationResult as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for VoiceCommandDisambiguationResult {
    const NAME: &'static str = "Windows.ApplicationModel.VoiceCommands.VoiceCommandDisambiguationResult";
}
unsafe impl Send for VoiceCommandDisambiguationResult {}
unsafe impl Sync for VoiceCommandDisambiguationResult {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct VoiceCommandResponse(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(VoiceCommandResponse, windows_core::IUnknown, windows_core::IInspectable);
impl VoiceCommandResponse {
    pub fn Message(&self) -> windows_core::Result<VoiceCommandUserMessage> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Message)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetMessage<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<VoiceCommandUserMessage>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetMessage)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn RepeatMessage(&self) -> windows_core::Result<VoiceCommandUserMessage> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RepeatMessage)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetRepeatMessage<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<VoiceCommandUserMessage>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetRepeatMessage)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn AppLaunchArgument(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AppLaunchArgument)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetAppLaunchArgument(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetAppLaunchArgument)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn VoiceCommandContentTiles(&self) -> windows_core::Result<super::super::Foundation::Collections::IVector<VoiceCommandContentTile>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).VoiceCommandContentTiles)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn MaxSupportedVoiceCommandContentTiles() -> windows_core::Result<u32> {
        Self::IVoiceCommandResponseStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MaxSupportedVoiceCommandContentTiles)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn CreateResponse<P0>(usermessage: P0) -> windows_core::Result<VoiceCommandResponse>
    where
        P0: windows_core::Param<VoiceCommandUserMessage>,
    {
        Self::IVoiceCommandResponseStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateResponse)(windows_core::Interface::as_raw(this), usermessage.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateResponseWithTiles<P0, P1>(message: P0, contenttiles: P1) -> windows_core::Result<VoiceCommandResponse>
    where
        P0: windows_core::Param<VoiceCommandUserMessage>,
        P1: windows_core::Param<super::super::Foundation::Collections::IIterable<VoiceCommandContentTile>>,
    {
        Self::IVoiceCommandResponseStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateResponseWithTiles)(windows_core::Interface::as_raw(this), message.param().abi(), contenttiles.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateResponseForPrompt<P0, P1>(message: P0, repeatmessage: P1) -> windows_core::Result<VoiceCommandResponse>
    where
        P0: windows_core::Param<VoiceCommandUserMessage>,
        P1: windows_core::Param<VoiceCommandUserMessage>,
    {
        Self::IVoiceCommandResponseStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateResponseForPrompt)(windows_core::Interface::as_raw(this), message.param().abi(), repeatmessage.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateResponseForPromptWithTiles<P0, P1, P2>(message: P0, repeatmessage: P1, contenttiles: P2) -> windows_core::Result<VoiceCommandResponse>
    where
        P0: windows_core::Param<VoiceCommandUserMessage>,
        P1: windows_core::Param<VoiceCommandUserMessage>,
        P2: windows_core::Param<super::super::Foundation::Collections::IIterable<VoiceCommandContentTile>>,
    {
        Self::IVoiceCommandResponseStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateResponseForPromptWithTiles)(windows_core::Interface::as_raw(this), message.param().abi(), repeatmessage.param().abi(), contenttiles.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IVoiceCommandResponseStatics<R, F: FnOnce(&IVoiceCommandResponseStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<VoiceCommandResponse, IVoiceCommandResponseStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for VoiceCommandResponse {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IVoiceCommandResponse>();
}
unsafe impl windows_core::Interface for VoiceCommandResponse {
    type Vtable = IVoiceCommandResponse_Vtbl;
    const IID: windows_core::GUID = <IVoiceCommandResponse as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for VoiceCommandResponse {
    const NAME: &'static str = "Windows.ApplicationModel.VoiceCommands.VoiceCommandResponse";
}
unsafe impl Send for VoiceCommandResponse {}
unsafe impl Sync for VoiceCommandResponse {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct VoiceCommandServiceConnection(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(VoiceCommandServiceConnection, windows_core::IUnknown, windows_core::IInspectable);
impl VoiceCommandServiceConnection {
    pub fn GetVoiceCommandAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncOperation<VoiceCommand>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetVoiceCommandAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn RequestConfirmationAsync<P0>(&self, response: P0) -> windows_core::Result<super::super::Foundation::IAsyncOperation<VoiceCommandConfirmationResult>>
    where
        P0: windows_core::Param<VoiceCommandResponse>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RequestConfirmationAsync)(windows_core::Interface::as_raw(this), response.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn RequestDisambiguationAsync<P0>(&self, response: P0) -> windows_core::Result<super::super::Foundation::IAsyncOperation<VoiceCommandDisambiguationResult>>
    where
        P0: windows_core::Param<VoiceCommandResponse>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RequestDisambiguationAsync)(windows_core::Interface::as_raw(this), response.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ReportProgressAsync<P0>(&self, response: P0) -> windows_core::Result<super::super::Foundation::IAsyncAction>
    where
        P0: windows_core::Param<VoiceCommandResponse>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReportProgressAsync)(windows_core::Interface::as_raw(this), response.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ReportSuccessAsync<P0>(&self, response: P0) -> windows_core::Result<super::super::Foundation::IAsyncAction>
    where
        P0: windows_core::Param<VoiceCommandResponse>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReportSuccessAsync)(windows_core::Interface::as_raw(this), response.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ReportFailureAsync<P0>(&self, response: P0) -> windows_core::Result<super::super::Foundation::IAsyncAction>
    where
        P0: windows_core::Param<VoiceCommandResponse>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReportFailureAsync)(windows_core::Interface::as_raw(this), response.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn RequestAppLaunchAsync<P0>(&self, response: P0) -> windows_core::Result<super::super::Foundation::IAsyncAction>
    where
        P0: windows_core::Param<VoiceCommandResponse>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RequestAppLaunchAsync)(windows_core::Interface::as_raw(this), response.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Globalization")]
    pub fn Language(&self) -> windows_core::Result<super::super::Globalization::Language> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Language)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn VoiceCommandCompleted<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<VoiceCommandServiceConnection, VoiceCommandCompletedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).VoiceCommandCompleted)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveVoiceCommandCompleted(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveVoiceCommandCompleted)(windows_core::Interface::as_raw(this), token).ok() }
    }
    #[cfg(feature = "ApplicationModel_AppService")]
    pub fn FromAppServiceTriggerDetails<P0>(triggerdetails: P0) -> windows_core::Result<VoiceCommandServiceConnection>
    where
        P0: windows_core::Param<super::AppService::AppServiceTriggerDetails>,
    {
        Self::IVoiceCommandServiceConnectionStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FromAppServiceTriggerDetails)(windows_core::Interface::as_raw(this), triggerdetails.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IVoiceCommandServiceConnectionStatics<R, F: FnOnce(&IVoiceCommandServiceConnectionStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<VoiceCommandServiceConnection, IVoiceCommandServiceConnectionStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for VoiceCommandServiceConnection {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IVoiceCommandServiceConnection>();
}
unsafe impl windows_core::Interface for VoiceCommandServiceConnection {
    type Vtable = IVoiceCommandServiceConnection_Vtbl;
    const IID: windows_core::GUID = <IVoiceCommandServiceConnection as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for VoiceCommandServiceConnection {
    const NAME: &'static str = "Windows.ApplicationModel.VoiceCommands.VoiceCommandServiceConnection";
}
unsafe impl Send for VoiceCommandServiceConnection {}
unsafe impl Sync for VoiceCommandServiceConnection {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct VoiceCommandUserMessage(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(VoiceCommandUserMessage, windows_core::IUnknown, windows_core::IInspectable);
impl VoiceCommandUserMessage {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<VoiceCommandUserMessage, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn DisplayMessage(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DisplayMessage)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetDisplayMessage(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetDisplayMessage)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn SpokenMessage(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SpokenMessage)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetSpokenMessage(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetSpokenMessage)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
}
impl windows_core::RuntimeType for VoiceCommandUserMessage {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IVoiceCommandUserMessage>();
}
unsafe impl windows_core::Interface for VoiceCommandUserMessage {
    type Vtable = IVoiceCommandUserMessage_Vtbl;
    const IID: windows_core::GUID = <IVoiceCommandUserMessage as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for VoiceCommandUserMessage {
    const NAME: &'static str = "Windows.ApplicationModel.VoiceCommands.VoiceCommandUserMessage";
}
unsafe impl Send for VoiceCommandUserMessage {}
unsafe impl Sync for VoiceCommandUserMessage {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct VoiceCommandCompletionReason(pub i32);
impl VoiceCommandCompletionReason {
    pub const Unknown: Self = Self(0i32);
    pub const CommunicationFailed: Self = Self(1i32);
    pub const ResourceLimitsExceeded: Self = Self(2i32);
    pub const Canceled: Self = Self(3i32);
    pub const TimeoutExceeded: Self = Self(4i32);
    pub const AppLaunched: Self = Self(5i32);
    pub const Completed: Self = Self(6i32);
}
impl windows_core::TypeKind for VoiceCommandCompletionReason {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for VoiceCommandCompletionReason {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("VoiceCommandCompletionReason").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for VoiceCommandCompletionReason {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.VoiceCommands.VoiceCommandCompletionReason;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct VoiceCommandContentTileType(pub i32);
impl VoiceCommandContentTileType {
    pub const TitleOnly: Self = Self(0i32);
    pub const TitleWithText: Self = Self(1i32);
    pub const TitleWith68x68Icon: Self = Self(2i32);
    pub const TitleWith68x68IconAndText: Self = Self(3i32);
    pub const TitleWith68x92Icon: Self = Self(4i32);
    pub const TitleWith68x92IconAndText: Self = Self(5i32);
    pub const TitleWith280x140Icon: Self = Self(6i32);
    pub const TitleWith280x140IconAndText: Self = Self(7i32);
}
impl windows_core::TypeKind for VoiceCommandContentTileType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for VoiceCommandContentTileType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("VoiceCommandContentTileType").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for VoiceCommandContentTileType {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.VoiceCommands.VoiceCommandContentTileType;i4)");
}
