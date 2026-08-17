#[cfg(feature = "deprecated")]
windows_core::imp::define_interface!(IBackgroundMediaPlayerStatics, IBackgroundMediaPlayerStatics_Vtbl, 0x856ddbc1_55f7_471f_a0f2_68ac4c904592);
#[cfg(feature = "deprecated")]
impl windows_core::RuntimeType for IBackgroundMediaPlayerStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct IBackgroundMediaPlayerStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "deprecated")]
    pub Current: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Current: usize,
    #[cfg(feature = "deprecated")]
    pub MessageReceivedFromBackground: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    MessageReceivedFromBackground: usize,
    #[cfg(feature = "deprecated")]
    pub RemoveMessageReceivedFromBackground: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    RemoveMessageReceivedFromBackground: usize,
    #[cfg(feature = "deprecated")]
    pub MessageReceivedFromForeground: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    MessageReceivedFromForeground: usize,
    #[cfg(feature = "deprecated")]
    pub RemoveMessageReceivedFromForeground: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    RemoveMessageReceivedFromForeground: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub SendMessageToBackground: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "deprecated")))]
    SendMessageToBackground: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub SendMessageToForeground: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "deprecated")))]
    SendMessageToForeground: usize,
    #[cfg(feature = "deprecated")]
    pub IsMediaPlaying: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    IsMediaPlaying: usize,
    #[cfg(feature = "deprecated")]
    pub Shutdown: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Shutdown: usize,
}
windows_core::imp::define_interface!(ICurrentMediaPlaybackItemChangedEventArgs, ICurrentMediaPlaybackItemChangedEventArgs_Vtbl, 0x1743a892_5c43_4a15_967a_572d2d0f26c6);
impl windows_core::RuntimeType for ICurrentMediaPlaybackItemChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICurrentMediaPlaybackItemChangedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub NewItem: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OldItem: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICurrentMediaPlaybackItemChangedEventArgs2, ICurrentMediaPlaybackItemChangedEventArgs2_Vtbl, 0x1d80a51e_996e_40a9_be48_e66ec90b2b7d);
impl windows_core::RuntimeType for ICurrentMediaPlaybackItemChangedEventArgs2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICurrentMediaPlaybackItemChangedEventArgs2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Reason: unsafe extern "system" fn(*mut core::ffi::c_void, *mut MediaPlaybackItemChangedReason) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMediaBreak, IMediaBreak_Vtbl, 0x714be270_0def_4ebc_a489_6b34930e1558);
impl windows_core::RuntimeType for IMediaBreak {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMediaBreak_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub PlaybackList: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PresentationPosition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub InsertionMethod: unsafe extern "system" fn(*mut core::ffi::c_void, *mut MediaBreakInsertionMethod) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub CustomProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CustomProperties: usize,
    pub CanStart: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetCanStart: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMediaBreakEndedEventArgs, IMediaBreakEndedEventArgs_Vtbl, 0x32b93276_1c5d_4fee_8732_236dc3a88580);
impl windows_core::RuntimeType for IMediaBreakEndedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMediaBreakEndedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub MediaBreak: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMediaBreakFactory, IMediaBreakFactory_Vtbl, 0x4516e002_18e0_4079_8b5f_d33495c15d2e);
impl windows_core::RuntimeType for IMediaBreakFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMediaBreakFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, MediaBreakInsertionMethod, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateWithPresentationPosition: unsafe extern "system" fn(*mut core::ffi::c_void, MediaBreakInsertionMethod, super::super::Foundation::TimeSpan, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMediaBreakManager, IMediaBreakManager_Vtbl, 0xa854ddb1_feb4_4d9b_9d97_0fdbe58e5e39);
impl windows_core::RuntimeType for IMediaBreakManager {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMediaBreakManager_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub BreaksSeekedOver: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveBreaksSeekedOver: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub BreakStarted: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveBreakStarted: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub BreakEnded: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveBreakEnded: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub BreakSkipped: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveBreakSkipped: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub CurrentBreak: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PlaybackSession: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PlayBreak: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SkipCurrentBreak: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMediaBreakSchedule, IMediaBreakSchedule_Vtbl, 0xa19a5813_98b6_41d8_83da_f971d22b7bba);
impl windows_core::RuntimeType for IMediaBreakSchedule {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMediaBreakSchedule_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ScheduleChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveScheduleChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub InsertMidrollBreak: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RemoveMidrollBreak: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub MidrollBreaks: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    MidrollBreaks: usize,
    pub SetPrerollBreak: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PrerollBreak: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetPostrollBreak: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PostrollBreak: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PlaybackItem: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMediaBreakSeekedOverEventArgs, IMediaBreakSeekedOverEventArgs_Vtbl, 0xe5aa6746_0606_4492_b9d3_c3c8fde0a4ea);
impl windows_core::RuntimeType for IMediaBreakSeekedOverEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMediaBreakSeekedOverEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub SeekedOverBreaks: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SeekedOverBreaks: usize,
    pub OldPosition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::TimeSpan) -> windows_core::HRESULT,
    pub NewPosition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::TimeSpan) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMediaBreakSkippedEventArgs, IMediaBreakSkippedEventArgs_Vtbl, 0x6ee94c05_2f54_4a3e_a3ab_24c3b270b4a3);
impl windows_core::RuntimeType for IMediaBreakSkippedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMediaBreakSkippedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub MediaBreak: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMediaBreakStartedEventArgs, IMediaBreakStartedEventArgs_Vtbl, 0xa87efe71_dfd4_454a_956e_0a4a648395f8);
impl windows_core::RuntimeType for IMediaBreakStartedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMediaBreakStartedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub MediaBreak: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "deprecated")]
windows_core::imp::define_interface!(IMediaEnginePlaybackSource, IMediaEnginePlaybackSource_Vtbl, 0x5c1d0ba7_3856_48b9_8dc6_244bf107bf8c);
#[cfg(feature = "deprecated")]
impl core::ops::Deref for IMediaEnginePlaybackSource {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "deprecated")]
windows_core::imp::interface_hierarchy!(IMediaEnginePlaybackSource, windows_core::IUnknown, windows_core::IInspectable);
#[cfg(feature = "deprecated")]
impl IMediaEnginePlaybackSource {
    #[cfg(feature = "deprecated")]
    pub fn CurrentItem(&self) -> windows_core::Result<MediaPlaybackItem> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CurrentItem)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn SetPlaybackSource<P0>(&self, source: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMediaPlaybackSource>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetPlaybackSource)(windows_core::Interface::as_raw(this), source.param().abi()).ok() }
    }
}
#[cfg(feature = "deprecated")]
impl windows_core::RuntimeType for IMediaEnginePlaybackSource {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct IMediaEnginePlaybackSource_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "deprecated")]
    pub CurrentItem: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    CurrentItem: usize,
    #[cfg(feature = "deprecated")]
    pub SetPlaybackSource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SetPlaybackSource: usize,
}
windows_core::imp::define_interface!(IMediaItemDisplayProperties, IMediaItemDisplayProperties_Vtbl, 0x1e3c1b48_7097_4384_a217_c1291dfa8c16);
impl windows_core::RuntimeType for IMediaItemDisplayProperties {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMediaItemDisplayProperties_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Type: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::MediaPlaybackType) -> windows_core::HRESULT,
    pub SetType: unsafe extern "system" fn(*mut core::ffi::c_void, super::MediaPlaybackType) -> windows_core::HRESULT,
    pub MusicProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub VideoProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub Thumbnail: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Thumbnail: usize,
    #[cfg(feature = "Storage_Streams")]
    pub SetThumbnail: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetThumbnail: usize,
    pub ClearAll: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMediaPlaybackCommandManager, IMediaPlaybackCommandManager_Vtbl, 0x5acee5a6_5cb6_4a5a_8521_cc86b1c1ed37);
impl windows_core::RuntimeType for IMediaPlaybackCommandManager {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMediaPlaybackCommandManager_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IsEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetIsEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub MediaPlayer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PlayBehavior: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PauseBehavior: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub NextBehavior: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PreviousBehavior: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FastForwardBehavior: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RewindBehavior: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ShuffleBehavior: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AutoRepeatModeBehavior: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PositionBehavior: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RateBehavior: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PlayReceived: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemovePlayReceived: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub PauseReceived: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemovePauseReceived: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub NextReceived: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveNextReceived: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub PreviousReceived: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemovePreviousReceived: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub FastForwardReceived: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveFastForwardReceived: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RewindReceived: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveRewindReceived: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub ShuffleReceived: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveShuffleReceived: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub AutoRepeatModeReceived: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveAutoRepeatModeReceived: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub PositionReceived: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemovePositionReceived: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RateReceived: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveRateReceived: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMediaPlaybackCommandManagerAutoRepeatModeReceivedEventArgs, IMediaPlaybackCommandManagerAutoRepeatModeReceivedEventArgs_Vtbl, 0x3d6f4f23_5230_4411_a0e9_bad94c2a045c);
impl windows_core::RuntimeType for IMediaPlaybackCommandManagerAutoRepeatModeReceivedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMediaPlaybackCommandManagerAutoRepeatModeReceivedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Handled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub AutoRepeatMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::MediaPlaybackAutoRepeatMode) -> windows_core::HRESULT,
    pub GetDeferral: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMediaPlaybackCommandManagerCommandBehavior, IMediaPlaybackCommandManagerCommandBehavior_Vtbl, 0x786c1e78_ce78_4a10_afd6_843fcbb90c2e);
impl windows_core::RuntimeType for IMediaPlaybackCommandManagerCommandBehavior {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMediaPlaybackCommandManagerCommandBehavior_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CommandManager: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub EnablingRule: unsafe extern "system" fn(*mut core::ffi::c_void, *mut MediaCommandEnablingRule) -> windows_core::HRESULT,
    pub SetEnablingRule: unsafe extern "system" fn(*mut core::ffi::c_void, MediaCommandEnablingRule) -> windows_core::HRESULT,
    pub IsEnabledChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveIsEnabledChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMediaPlaybackCommandManagerFastForwardReceivedEventArgs, IMediaPlaybackCommandManagerFastForwardReceivedEventArgs_Vtbl, 0x30f064d9_b491_4d0a_bc21_3098bd1332e9);
impl windows_core::RuntimeType for IMediaPlaybackCommandManagerFastForwardReceivedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMediaPlaybackCommandManagerFastForwardReceivedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Handled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub GetDeferral: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMediaPlaybackCommandManagerNextReceivedEventArgs, IMediaPlaybackCommandManagerNextReceivedEventArgs_Vtbl, 0xe1504433_a2b0_45d4_b9de_5f42ac14a839);
impl windows_core::RuntimeType for IMediaPlaybackCommandManagerNextReceivedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMediaPlaybackCommandManagerNextReceivedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Handled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub GetDeferral: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMediaPlaybackCommandManagerPauseReceivedEventArgs, IMediaPlaybackCommandManagerPauseReceivedEventArgs_Vtbl, 0x5ceccd1c_c25c_4221_b16c_c3c98ce012d6);
impl windows_core::RuntimeType for IMediaPlaybackCommandManagerPauseReceivedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMediaPlaybackCommandManagerPauseReceivedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Handled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub GetDeferral: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMediaPlaybackCommandManagerPlayReceivedEventArgs, IMediaPlaybackCommandManagerPlayReceivedEventArgs_Vtbl, 0x9af0004e_578b_4c56_a006_16159d888a48);
impl windows_core::RuntimeType for IMediaPlaybackCommandManagerPlayReceivedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMediaPlaybackCommandManagerPlayReceivedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Handled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub GetDeferral: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMediaPlaybackCommandManagerPositionReceivedEventArgs, IMediaPlaybackCommandManagerPositionReceivedEventArgs_Vtbl, 0x5591a754_d627_4bdd_a90d_86a015b24902);
impl windows_core::RuntimeType for IMediaPlaybackCommandManagerPositionReceivedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMediaPlaybackCommandManagerPositionReceivedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Handled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub Position: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::TimeSpan) -> windows_core::HRESULT,
    pub GetDeferral: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMediaPlaybackCommandManagerPreviousReceivedEventArgs, IMediaPlaybackCommandManagerPreviousReceivedEventArgs_Vtbl, 0x525e3081_4632_4f76_99b1_d771623f6287);
impl windows_core::RuntimeType for IMediaPlaybackCommandManagerPreviousReceivedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMediaPlaybackCommandManagerPreviousReceivedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Handled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub GetDeferral: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMediaPlaybackCommandManagerRateReceivedEventArgs, IMediaPlaybackCommandManagerRateReceivedEventArgs_Vtbl, 0x18ea3939_4a16_4169_8b05_3eb9f5ff78eb);
impl windows_core::RuntimeType for IMediaPlaybackCommandManagerRateReceivedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMediaPlaybackCommandManagerRateReceivedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Handled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub PlaybackRate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub GetDeferral: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMediaPlaybackCommandManagerRewindReceivedEventArgs, IMediaPlaybackCommandManagerRewindReceivedEventArgs_Vtbl, 0x9f085947_a3c0_425d_aaef_97ba7898b141);
impl windows_core::RuntimeType for IMediaPlaybackCommandManagerRewindReceivedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMediaPlaybackCommandManagerRewindReceivedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Handled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub GetDeferral: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMediaPlaybackCommandManagerShuffleReceivedEventArgs, IMediaPlaybackCommandManagerShuffleReceivedEventArgs_Vtbl, 0x50a05cef_63ee_4a96_b7b5_fee08b9ff90c);
impl windows_core::RuntimeType for IMediaPlaybackCommandManagerShuffleReceivedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMediaPlaybackCommandManagerShuffleReceivedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Handled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub IsShuffleRequested: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub GetDeferral: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMediaPlaybackItem, IMediaPlaybackItem_Vtbl, 0x047097d2_e4af_48ab_b283_6929e674ece2);
impl windows_core::RuntimeType for IMediaPlaybackItem {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMediaPlaybackItem_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub AudioTracksChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    AudioTracksChanged: usize,
    pub RemoveAudioTracksChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub VideoTracksChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    VideoTracksChanged: usize,
    pub RemoveVideoTracksChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub TimedMetadataTracksChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    TimedMetadataTracksChanged: usize,
    pub RemoveTimedMetadataTracksChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    #[cfg(feature = "Media_Core")]
    pub Source: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Media_Core"))]
    Source: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Core"))]
    pub AudioTracks: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Media_Core")))]
    AudioTracks: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Core"))]
    pub VideoTracks: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Media_Core")))]
    VideoTracks: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Core"))]
    pub TimedMetadataTracks: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Media_Core")))]
    TimedMetadataTracks: usize,
}
windows_core::imp::define_interface!(IMediaPlaybackItem2, IMediaPlaybackItem2_Vtbl, 0xd859d171_d7ef_4b81_ac1f_f40493cbb091);
impl windows_core::RuntimeType for IMediaPlaybackItem2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMediaPlaybackItem2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub BreakSchedule: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub StartTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::TimeSpan) -> windows_core::HRESULT,
    pub DurationLimit: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CanSkip: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetCanSkip: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub GetDisplayProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ApplyDisplayProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMediaPlaybackItem3, IMediaPlaybackItem3_Vtbl, 0x0d328220_b80a_4d09_9ff8_f87094a1c831);
impl windows_core::RuntimeType for IMediaPlaybackItem3 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMediaPlaybackItem3_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IsDisabledInPlaybackList: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetIsDisabledInPlaybackList: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub TotalDownloadProgress: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub AutoLoadedDisplayProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut AutoLoadedDisplayPropertyKind) -> windows_core::HRESULT,
    pub SetAutoLoadedDisplayProperties: unsafe extern "system" fn(*mut core::ffi::c_void, AutoLoadedDisplayPropertyKind) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMediaPlaybackItemError, IMediaPlaybackItemError_Vtbl, 0x69fbef2b_dcd6_4df9_a450_dbf4c6f1c2c2);
impl windows_core::RuntimeType for IMediaPlaybackItemError {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMediaPlaybackItemError_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ErrorCode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut MediaPlaybackItemErrorCode) -> windows_core::HRESULT,
    pub ExtendedError: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::HRESULT) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMediaPlaybackItemFactory, IMediaPlaybackItemFactory_Vtbl, 0x7133fce1_1769_4ff9_a7c1_38d2c4d42360);
impl windows_core::RuntimeType for IMediaPlaybackItemFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMediaPlaybackItemFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Media_Core")]
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Media_Core"))]
    Create: usize,
}
windows_core::imp::define_interface!(IMediaPlaybackItemFactory2, IMediaPlaybackItemFactory2_Vtbl, 0xd77cdf3a_b947_4972_b35d_adfb931a71e6);
impl windows_core::RuntimeType for IMediaPlaybackItemFactory2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMediaPlaybackItemFactory2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Media_Core")]
    pub CreateWithStartTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::Foundation::TimeSpan, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Media_Core"))]
    CreateWithStartTime: usize,
    #[cfg(feature = "Media_Core")]
    pub CreateWithStartTimeAndDurationLimit: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::Foundation::TimeSpan, super::super::Foundation::TimeSpan, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Media_Core"))]
    CreateWithStartTimeAndDurationLimit: usize,
}
windows_core::imp::define_interface!(IMediaPlaybackItemFailedEventArgs, IMediaPlaybackItemFailedEventArgs_Vtbl, 0x7703134a_e9a7_47c3_862c_c656d30683d4);
impl windows_core::RuntimeType for IMediaPlaybackItemFailedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMediaPlaybackItemFailedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Item: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Error: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMediaPlaybackItemOpenedEventArgs, IMediaPlaybackItemOpenedEventArgs_Vtbl, 0xcbd9bd82_3037_4fbe_ae8f_39fc39edf4ef);
impl windows_core::RuntimeType for IMediaPlaybackItemOpenedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMediaPlaybackItemOpenedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Item: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMediaPlaybackItemStatics, IMediaPlaybackItemStatics_Vtbl, 0x4b1be7f4_4345_403c_8a67_f5de91df4c86);
impl windows_core::RuntimeType for IMediaPlaybackItemStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMediaPlaybackItemStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Media_Core")]
    pub FindFromMediaSource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Media_Core"))]
    FindFromMediaSource: usize,
}
windows_core::imp::define_interface!(IMediaPlaybackList, IMediaPlaybackList_Vtbl, 0x7f77ee9c_dc42_4e26_a98d_7850df8ec925);
impl windows_core::RuntimeType for IMediaPlaybackList {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMediaPlaybackList_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ItemFailed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveItemFailed: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub CurrentItemChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveCurrentItemChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub ItemOpened: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveItemOpened: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Items: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Items: usize,
    pub AutoRepeatEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetAutoRepeatEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub ShuffleEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetShuffleEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub CurrentItem: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CurrentItemIndex: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub MoveNext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub MovePrevious: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub MoveTo: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMediaPlaybackList2, IMediaPlaybackList2_Vtbl, 0x0e09b478_600a_4274_a14b_0b6723d0f48b);
impl windows_core::RuntimeType for IMediaPlaybackList2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMediaPlaybackList2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub MaxPrefetchTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetMaxPrefetchTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub StartingItem: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetStartingItem: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub ShuffledItems: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ShuffledItems: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub SetShuffledItems: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SetShuffledItems: usize,
}
windows_core::imp::define_interface!(IMediaPlaybackList3, IMediaPlaybackList3_Vtbl, 0xdd24bba9_bc47_4463_aa90_c18b7e5ffde1);
impl windows_core::RuntimeType for IMediaPlaybackList3 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMediaPlaybackList3_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub MaxPlayedItemsToKeepOpen: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetMaxPlayedItemsToKeepOpen: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMediaPlaybackSession, IMediaPlaybackSession_Vtbl, 0xc32b683d_0407_41ba_8946_8b345a5a5435);
impl windows_core::RuntimeType for IMediaPlaybackSession {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMediaPlaybackSession_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub PlaybackStateChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemovePlaybackStateChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub PlaybackRateChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemovePlaybackRateChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub SeekCompleted: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveSeekCompleted: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub BufferingStarted: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveBufferingStarted: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub BufferingEnded: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveBufferingEnded: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub BufferingProgressChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveBufferingProgressChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub DownloadProgressChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveDownloadProgressChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub NaturalDurationChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveNaturalDurationChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub PositionChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemovePositionChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub NaturalVideoSizeChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveNaturalVideoSizeChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub MediaPlayer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub NaturalDuration: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::TimeSpan) -> windows_core::HRESULT,
    pub Position: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::TimeSpan) -> windows_core::HRESULT,
    pub SetPosition: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::TimeSpan) -> windows_core::HRESULT,
    pub PlaybackState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut MediaPlaybackState) -> windows_core::HRESULT,
    pub CanSeek: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub CanPause: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsProtected: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub PlaybackRate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub SetPlaybackRate: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub BufferingProgress: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub DownloadProgress: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub NaturalVideoHeight: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub NaturalVideoWidth: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub NormalizedSourceRect: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::Rect) -> windows_core::HRESULT,
    pub SetNormalizedSourceRect: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::Rect) -> windows_core::HRESULT,
    #[cfg(feature = "Media_MediaProperties")]
    pub StereoscopicVideoPackingMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::MediaProperties::StereoscopicVideoPackingMode) -> windows_core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))]
    StereoscopicVideoPackingMode: usize,
    #[cfg(feature = "Media_MediaProperties")]
    pub SetStereoscopicVideoPackingMode: unsafe extern "system" fn(*mut core::ffi::c_void, super::MediaProperties::StereoscopicVideoPackingMode) -> windows_core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))]
    SetStereoscopicVideoPackingMode: usize,
}
windows_core::imp::define_interface!(IMediaPlaybackSession2, IMediaPlaybackSession2_Vtbl, 0xf8ba7c79_1fc8_4097_ad70_c0fa18cc0050);
impl windows_core::RuntimeType for IMediaPlaybackSession2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMediaPlaybackSession2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub BufferedRangesChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveBufferedRangesChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub PlayedRangesChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemovePlayedRangesChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub SeekableRangesChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveSeekableRangesChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub SupportedPlaybackRatesChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveSupportedPlaybackRatesChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub SphericalVideoProjection: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsMirroring: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetIsMirroring: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetBufferedRanges: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetBufferedRanges: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetPlayedRanges: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetPlayedRanges: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetSeekableRanges: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetSeekableRanges: usize,
    pub IsSupportedPlaybackRateRange: unsafe extern "system" fn(*mut core::ffi::c_void, f64, f64, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMediaPlaybackSession3, IMediaPlaybackSession3_Vtbl, 0x7ba2b41a_a3e2_405f_b77b_a4812c238b66);
impl windows_core::RuntimeType for IMediaPlaybackSession3 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMediaPlaybackSession3_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Media_MediaProperties")]
    pub PlaybackRotation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::MediaProperties::MediaRotation) -> windows_core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))]
    PlaybackRotation: usize,
    #[cfg(feature = "Media_MediaProperties")]
    pub SetPlaybackRotation: unsafe extern "system" fn(*mut core::ffi::c_void, super::MediaProperties::MediaRotation) -> windows_core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))]
    SetPlaybackRotation: usize,
    pub GetOutputDegradationPolicyState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMediaPlaybackSessionBufferingStartedEventArgs, IMediaPlaybackSessionBufferingStartedEventArgs_Vtbl, 0xcd6aafed_74e2_43b5_b115_76236c33791a);
impl windows_core::RuntimeType for IMediaPlaybackSessionBufferingStartedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMediaPlaybackSessionBufferingStartedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IsPlaybackInterruption: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMediaPlaybackSessionOutputDegradationPolicyState, IMediaPlaybackSessionOutputDegradationPolicyState_Vtbl, 0x558e727d_f633_49f9_965a_abaa1db709be);
impl windows_core::RuntimeType for IMediaPlaybackSessionOutputDegradationPolicyState {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMediaPlaybackSessionOutputDegradationPolicyState_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub VideoConstrictionReason: unsafe extern "system" fn(*mut core::ffi::c_void, *mut MediaPlaybackSessionVideoConstrictionReason) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMediaPlaybackSource, IMediaPlaybackSource_Vtbl, 0xef9dc2bc_9317_4696_b051_2bad643177b5);
impl core::ops::Deref for IMediaPlaybackSource {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IMediaPlaybackSource, windows_core::IUnknown, windows_core::IInspectable);
impl IMediaPlaybackSource {}
impl windows_core::RuntimeType for IMediaPlaybackSource {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMediaPlaybackSource_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(IMediaPlaybackSphericalVideoProjection, IMediaPlaybackSphericalVideoProjection_Vtbl, 0xd405b37c_6f0e_4661_b8ee_d487ba9752d5);
impl windows_core::RuntimeType for IMediaPlaybackSphericalVideoProjection {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMediaPlaybackSphericalVideoProjection_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IsEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetIsEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    #[cfg(feature = "Media_MediaProperties")]
    pub FrameFormat: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::MediaProperties::SphericalVideoFrameFormat) -> windows_core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))]
    FrameFormat: usize,
    #[cfg(feature = "Media_MediaProperties")]
    pub SetFrameFormat: unsafe extern "system" fn(*mut core::ffi::c_void, super::MediaProperties::SphericalVideoFrameFormat) -> windows_core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))]
    SetFrameFormat: usize,
    pub HorizontalFieldOfViewInDegrees: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub SetHorizontalFieldOfViewInDegrees: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Numerics")]
    pub ViewOrientation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::Numerics::Quaternion) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    ViewOrientation: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetViewOrientation: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::Numerics::Quaternion) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetViewOrientation: usize,
    pub ProjectionMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SphericalVideoProjectionMode) -> windows_core::HRESULT,
    pub SetProjectionMode: unsafe extern "system" fn(*mut core::ffi::c_void, SphericalVideoProjectionMode) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMediaPlaybackTimedMetadataTrackList, IMediaPlaybackTimedMetadataTrackList_Vtbl, 0x72b41319_bbfb_46a3_9372_9c9c744b9438);
impl windows_core::RuntimeType for IMediaPlaybackTimedMetadataTrackList {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMediaPlaybackTimedMetadataTrackList_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Core"))]
    pub PresentationModeChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Media_Core")))]
    PresentationModeChanged: usize,
    pub RemovePresentationModeChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub GetPresentationMode: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut TimedMetadataTrackPresentationMode) -> windows_core::HRESULT,
    pub SetPresentationMode: unsafe extern "system" fn(*mut core::ffi::c_void, u32, TimedMetadataTrackPresentationMode) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMediaPlayer, IMediaPlayer_Vtbl, 0x381a83cb_6fff_499b_8d64_2885dfc1249e);
impl windows_core::RuntimeType for IMediaPlayer {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMediaPlayer_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub AutoPlay: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetAutoPlay: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    #[cfg(feature = "deprecated")]
    pub NaturalDuration: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::TimeSpan) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    NaturalDuration: usize,
    #[cfg(feature = "deprecated")]
    pub Position: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::TimeSpan) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Position: usize,
    #[cfg(feature = "deprecated")]
    pub SetPosition: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::TimeSpan) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SetPosition: usize,
    #[cfg(feature = "deprecated")]
    pub BufferingProgress: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    BufferingProgress: usize,
    #[cfg(feature = "deprecated")]
    pub CurrentState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut MediaPlayerState) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    CurrentState: usize,
    #[cfg(feature = "deprecated")]
    pub CanSeek: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    CanSeek: usize,
    #[cfg(feature = "deprecated")]
    pub CanPause: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    CanPause: usize,
    pub IsLoopingEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetIsLoopingEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    #[cfg(feature = "deprecated")]
    pub IsProtected: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    IsProtected: usize,
    pub IsMuted: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetIsMuted: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    #[cfg(feature = "deprecated")]
    pub PlaybackRate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    PlaybackRate: usize,
    #[cfg(feature = "deprecated")]
    pub SetPlaybackRate: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SetPlaybackRate: usize,
    pub Volume: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub SetVolume: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    #[cfg(feature = "deprecated")]
    pub PlaybackMediaMarkers: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    PlaybackMediaMarkers: usize,
    pub MediaOpened: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveMediaOpened: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub MediaEnded: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveMediaEnded: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub MediaFailed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveMediaFailed: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    #[cfg(feature = "deprecated")]
    pub CurrentStateChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    CurrentStateChanged: usize,
    #[cfg(feature = "deprecated")]
    pub RemoveCurrentStateChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    RemoveCurrentStateChanged: usize,
    #[cfg(feature = "deprecated")]
    pub PlaybackMediaMarkerReached: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    PlaybackMediaMarkerReached: usize,
    #[cfg(feature = "deprecated")]
    pub RemovePlaybackMediaMarkerReached: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    RemovePlaybackMediaMarkerReached: usize,
    #[cfg(feature = "deprecated")]
    pub MediaPlayerRateChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    MediaPlayerRateChanged: usize,
    #[cfg(feature = "deprecated")]
    pub RemoveMediaPlayerRateChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    RemoveMediaPlayerRateChanged: usize,
    pub VolumeChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveVolumeChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    #[cfg(feature = "deprecated")]
    pub SeekCompleted: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SeekCompleted: usize,
    #[cfg(feature = "deprecated")]
    pub RemoveSeekCompleted: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    RemoveSeekCompleted: usize,
    #[cfg(feature = "deprecated")]
    pub BufferingStarted: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    BufferingStarted: usize,
    #[cfg(feature = "deprecated")]
    pub RemoveBufferingStarted: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    RemoveBufferingStarted: usize,
    #[cfg(feature = "deprecated")]
    pub BufferingEnded: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    BufferingEnded: usize,
    #[cfg(feature = "deprecated")]
    pub RemoveBufferingEnded: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    RemoveBufferingEnded: usize,
    pub Play: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Pause: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "deprecated")]
    pub SetUriSource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SetUriSource: usize,
}
windows_core::imp::define_interface!(IMediaPlayer2, IMediaPlayer2_Vtbl, 0x3c841218_2123_4fc5_9082_2f883f77bdf5);
impl windows_core::RuntimeType for IMediaPlayer2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMediaPlayer2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SystemMediaTransportControls: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AudioCategory: unsafe extern "system" fn(*mut core::ffi::c_void, *mut MediaPlayerAudioCategory) -> windows_core::HRESULT,
    pub SetAudioCategory: unsafe extern "system" fn(*mut core::ffi::c_void, MediaPlayerAudioCategory) -> windows_core::HRESULT,
    pub AudioDeviceType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut MediaPlayerAudioDeviceType) -> windows_core::HRESULT,
    pub SetAudioDeviceType: unsafe extern "system" fn(*mut core::ffi::c_void, MediaPlayerAudioDeviceType) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMediaPlayer3, IMediaPlayer3_Vtbl, 0xee0660da_031b_4feb_bd9b_92e0a0a8d299);
impl windows_core::RuntimeType for IMediaPlayer3 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMediaPlayer3_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IsMutedChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveIsMutedChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub SourceChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveSourceChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub AudioBalance: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub SetAudioBalance: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub RealTimePlayback: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetRealTimePlayback: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub StereoscopicVideoRenderMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut StereoscopicVideoRenderMode) -> windows_core::HRESULT,
    pub SetStereoscopicVideoRenderMode: unsafe extern "system" fn(*mut core::ffi::c_void, StereoscopicVideoRenderMode) -> windows_core::HRESULT,
    pub BreakManager: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CommandManager: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Devices_Enumeration")]
    pub AudioDevice: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Devices_Enumeration"))]
    AudioDevice: usize,
    #[cfg(feature = "Devices_Enumeration")]
    pub SetAudioDevice: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Devices_Enumeration"))]
    SetAudioDevice: usize,
    pub TimelineController: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetTimelineController: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TimelineControllerPositionOffset: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::TimeSpan) -> windows_core::HRESULT,
    pub SetTimelineControllerPositionOffset: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::TimeSpan) -> windows_core::HRESULT,
    pub PlaybackSession: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub StepForwardOneFrame: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub StepBackwardOneFrame: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Media_Casting")]
    pub GetAsCastingSource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Media_Casting"))]
    GetAsCastingSource: usize,
}
windows_core::imp::define_interface!(IMediaPlayer4, IMediaPlayer4_Vtbl, 0x80035db0_7448_4770_afcf_2a57450914c5);
impl windows_core::RuntimeType for IMediaPlayer4 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMediaPlayer4_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SetSurfaceSize: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::Size) -> windows_core::HRESULT,
    #[cfg(feature = "UI_Composition")]
    pub GetSurface: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Composition"))]
    GetSurface: usize,
}
windows_core::imp::define_interface!(IMediaPlayer5, IMediaPlayer5_Vtbl, 0xcfe537fd_f86a_4446_bf4d_c8e792b7b4b3);
impl windows_core::RuntimeType for IMediaPlayer5 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMediaPlayer5_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub VideoFrameAvailable: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveVideoFrameAvailable: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub IsVideoFrameServerEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetIsVideoFrameServerEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    #[cfg(feature = "Graphics_DirectX_Direct3D11")]
    pub CopyFrameToVideoSurface: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX_Direct3D11"))]
    CopyFrameToVideoSurface: usize,
    #[cfg(feature = "Graphics_DirectX_Direct3D11")]
    pub CopyFrameToVideoSurfaceWithTargetRectangle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::Foundation::Rect) -> windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX_Direct3D11"))]
    CopyFrameToVideoSurfaceWithTargetRectangle: usize,
    #[cfg(feature = "Graphics_DirectX_Direct3D11")]
    pub CopyFrameToStereoscopicVideoSurfaces: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX_Direct3D11"))]
    CopyFrameToStereoscopicVideoSurfaces: usize,
}
windows_core::imp::define_interface!(IMediaPlayer6, IMediaPlayer6_Vtbl, 0xe0caa086_ae65_414c_b010_8bc55f00e692);
impl windows_core::RuntimeType for IMediaPlayer6 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMediaPlayer6_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SubtitleFrameChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveSubtitleFrameChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    #[cfg(feature = "Graphics_DirectX_Direct3D11")]
    pub RenderSubtitlesToSurface: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX_Direct3D11"))]
    RenderSubtitlesToSurface: usize,
    #[cfg(feature = "Graphics_DirectX_Direct3D11")]
    pub RenderSubtitlesToSurfaceWithTargetRectangle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::Foundation::Rect, *mut bool) -> windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX_Direct3D11"))]
    RenderSubtitlesToSurfaceWithTargetRectangle: usize,
}
windows_core::imp::define_interface!(IMediaPlayer7, IMediaPlayer7_Vtbl, 0x5d1dc478_4500_4531_b3f4_777a71491f7f);
impl windows_core::RuntimeType for IMediaPlayer7 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMediaPlayer7_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Media_Audio")]
    pub AudioStateMonitor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Media_Audio"))]
    AudioStateMonitor: usize,
}
windows_core::imp::define_interface!(IMediaPlayerDataReceivedEventArgs, IMediaPlayerDataReceivedEventArgs_Vtbl, 0xc75a9405_c801_412a_835b_83fc0e622a8e);
impl windows_core::RuntimeType for IMediaPlayerDataReceivedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMediaPlayerDataReceivedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Data: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Data: usize,
}
windows_core::imp::define_interface!(IMediaPlayerEffects, IMediaPlayerEffects_Vtbl, 0x85a1deda_cab6_4cc0_8be3_6035f4de2591);
impl windows_core::RuntimeType for IMediaPlayerEffects {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMediaPlayerEffects_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub AddAudioEffect: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, bool, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    AddAudioEffect: usize,
    pub RemoveAllEffects: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMediaPlayerEffects2, IMediaPlayerEffects2_Vtbl, 0xfa419a79_1bbe_46c5_ae1f_8ee69fb3c2c7);
impl windows_core::RuntimeType for IMediaPlayerEffects2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMediaPlayerEffects2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub AddVideoEffect: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, bool, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    AddVideoEffect: usize,
}
windows_core::imp::define_interface!(IMediaPlayerFailedEventArgs, IMediaPlayerFailedEventArgs_Vtbl, 0x2744e9b9_a7e3_4f16_bac4_7914ebc08301);
impl windows_core::RuntimeType for IMediaPlayerFailedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMediaPlayerFailedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Error: unsafe extern "system" fn(*mut core::ffi::c_void, *mut MediaPlayerError) -> windows_core::HRESULT,
    pub ExtendedErrorCode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::HRESULT) -> windows_core::HRESULT,
    pub ErrorMessage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMediaPlayerRateChangedEventArgs, IMediaPlayerRateChangedEventArgs_Vtbl, 0x40600d58_3b61_4bb2_989f_fc65608b6cab);
impl windows_core::RuntimeType for IMediaPlayerRateChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMediaPlayerRateChangedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub NewRate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMediaPlayerSource, IMediaPlayerSource_Vtbl, 0xbd4f8897_1423_4c3e_82c5_0fb1af94f715);
impl windows_core::RuntimeType for IMediaPlayerSource {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMediaPlayerSource_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Media_Protection")]
    pub ProtectionManager: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Media_Protection"))]
    ProtectionManager: usize,
    #[cfg(feature = "Media_Protection")]
    pub SetProtectionManager: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Media_Protection"))]
    SetProtectionManager: usize,
    #[cfg(all(feature = "Storage", feature = "deprecated"))]
    pub SetFileSource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Storage", feature = "deprecated")))]
    SetFileSource: usize,
    #[cfg(all(feature = "Storage_Streams", feature = "deprecated"))]
    pub SetStreamSource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Storage_Streams", feature = "deprecated")))]
    SetStreamSource: usize,
    #[cfg(all(feature = "Media_Core", feature = "deprecated"))]
    pub SetMediaSource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Media_Core", feature = "deprecated")))]
    SetMediaSource: usize,
}
windows_core::imp::define_interface!(IMediaPlayerSource2, IMediaPlayerSource2_Vtbl, 0x82449b9f_7322_4c0b_b03b_3e69a48260c5);
impl windows_core::RuntimeType for IMediaPlayerSource2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMediaPlayerSource2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Source: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetSource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMediaPlayerSurface, IMediaPlayerSurface_Vtbl, 0x0ed653bc_b736_49c3_830b_764a3845313a);
impl windows_core::RuntimeType for IMediaPlayerSurface {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMediaPlayerSurface_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "UI_Composition")]
    pub CompositionSurface: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Composition"))]
    CompositionSurface: usize,
    #[cfg(feature = "UI_Composition")]
    pub Compositor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Composition"))]
    Compositor: usize,
    pub MediaPlayer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPlaybackMediaMarker, IPlaybackMediaMarker_Vtbl, 0xc4d22f5c_3c1c_4444_b6b9_778b0422d41a);
impl windows_core::RuntimeType for IPlaybackMediaMarker {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPlaybackMediaMarker_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Time: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::TimeSpan) -> windows_core::HRESULT,
    pub MediaMarkerType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Text: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPlaybackMediaMarkerFactory, IPlaybackMediaMarkerFactory_Vtbl, 0x8c530a78_e0ae_4e1a_a8c8_e23f982a937b);
impl windows_core::RuntimeType for IPlaybackMediaMarkerFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPlaybackMediaMarkerFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateFromTime: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::TimeSpan, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::TimeSpan, core::mem::MaybeUninit<windows_core::HSTRING>, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPlaybackMediaMarkerReachedEventArgs, IPlaybackMediaMarkerReachedEventArgs_Vtbl, 0x578cd1b9_90e2_4e60_abc4_8740b01f6196);
impl windows_core::RuntimeType for IPlaybackMediaMarkerReachedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPlaybackMediaMarkerReachedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub PlaybackMediaMarker: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPlaybackMediaMarkerSequence, IPlaybackMediaMarkerSequence_Vtbl, 0xf2810cee_638b_46cf_8817_1d111fe9d8c4);
impl windows_core::RuntimeType for IPlaybackMediaMarkerSequence {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPlaybackMediaMarkerSequence_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Size: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Insert: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clear: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ITimedMetadataPresentationModeChangedEventArgs, ITimedMetadataPresentationModeChangedEventArgs_Vtbl, 0xd1636099_65df_45ae_8cef_dc0b53fdc2bb);
impl windows_core::RuntimeType for ITimedMetadataPresentationModeChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ITimedMetadataPresentationModeChangedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Media_Core")]
    pub Track: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Media_Core"))]
    Track: usize,
    pub OldPresentationMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut TimedMetadataTrackPresentationMode) -> windows_core::HRESULT,
    pub NewPresentationMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut TimedMetadataTrackPresentationMode) -> windows_core::HRESULT,
}
#[cfg(feature = "deprecated")]
pub struct BackgroundMediaPlayer;
#[cfg(feature = "deprecated")]
impl BackgroundMediaPlayer {
    #[cfg(feature = "deprecated")]
    pub fn Current() -> windows_core::Result<MediaPlayer> {
        Self::IBackgroundMediaPlayerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Current)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "deprecated")]
    pub fn MessageReceivedFromBackground<P0>(value: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::EventHandler<MediaPlayerDataReceivedEventArgs>>,
    {
        Self::IBackgroundMediaPlayerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MessageReceivedFromBackground)(windows_core::Interface::as_raw(this), value.param().abi(), &mut result__).map(|| result__)
        })
    }
    #[cfg(feature = "deprecated")]
    pub fn RemoveMessageReceivedFromBackground(token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        Self::IBackgroundMediaPlayerStatics(|this| unsafe { (windows_core::Interface::vtable(this).RemoveMessageReceivedFromBackground)(windows_core::Interface::as_raw(this), token).ok() })
    }
    #[cfg(feature = "deprecated")]
    pub fn MessageReceivedFromForeground<P0>(value: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::EventHandler<MediaPlayerDataReceivedEventArgs>>,
    {
        Self::IBackgroundMediaPlayerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MessageReceivedFromForeground)(windows_core::Interface::as_raw(this), value.param().abi(), &mut result__).map(|| result__)
        })
    }
    #[cfg(feature = "deprecated")]
    pub fn RemoveMessageReceivedFromForeground(token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        Self::IBackgroundMediaPlayerStatics(|this| unsafe { (windows_core::Interface::vtable(this).RemoveMessageReceivedFromForeground)(windows_core::Interface::as_raw(this), token).ok() })
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub fn SendMessageToBackground<P0>(value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::Collections::ValueSet>,
    {
        Self::IBackgroundMediaPlayerStatics(|this| unsafe { (windows_core::Interface::vtable(this).SendMessageToBackground)(windows_core::Interface::as_raw(this), value.param().abi()).ok() })
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub fn SendMessageToForeground<P0>(value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::Collections::ValueSet>,
    {
        Self::IBackgroundMediaPlayerStatics(|this| unsafe { (windows_core::Interface::vtable(this).SendMessageToForeground)(windows_core::Interface::as_raw(this), value.param().abi()).ok() })
    }
    #[cfg(feature = "deprecated")]
    pub fn IsMediaPlaying() -> windows_core::Result<bool> {
        Self::IBackgroundMediaPlayerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsMediaPlaying)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    #[cfg(feature = "deprecated")]
    pub fn Shutdown() -> windows_core::Result<()> {
        Self::IBackgroundMediaPlayerStatics(|this| unsafe { (windows_core::Interface::vtable(this).Shutdown)(windows_core::Interface::as_raw(this)).ok() })
    }
    #[doc(hidden)]
    #[cfg(feature = "deprecated")]
    pub fn IBackgroundMediaPlayerStatics<R, F: FnOnce(&IBackgroundMediaPlayerStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<BackgroundMediaPlayer, IBackgroundMediaPlayerStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
#[cfg(feature = "deprecated")]
impl windows_core::RuntimeName for BackgroundMediaPlayer {
    const NAME: &'static str = "Windows.Media.Playback.BackgroundMediaPlayer";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct CurrentMediaPlaybackItemChangedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(CurrentMediaPlaybackItemChangedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl CurrentMediaPlaybackItemChangedEventArgs {
    pub fn NewItem(&self) -> windows_core::Result<MediaPlaybackItem> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NewItem)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn OldItem(&self) -> windows_core::Result<MediaPlaybackItem> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OldItem)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Reason(&self) -> windows_core::Result<MediaPlaybackItemChangedReason> {
        let this = &windows_core::Interface::cast::<ICurrentMediaPlaybackItemChangedEventArgs2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Reason)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for CurrentMediaPlaybackItemChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ICurrentMediaPlaybackItemChangedEventArgs>();
}
unsafe impl windows_core::Interface for CurrentMediaPlaybackItemChangedEventArgs {
    type Vtable = ICurrentMediaPlaybackItemChangedEventArgs_Vtbl;
    const IID: windows_core::GUID = <ICurrentMediaPlaybackItemChangedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for CurrentMediaPlaybackItemChangedEventArgs {
    const NAME: &'static str = "Windows.Media.Playback.CurrentMediaPlaybackItemChangedEventArgs";
}
unsafe impl Send for CurrentMediaPlaybackItemChangedEventArgs {}
unsafe impl Sync for CurrentMediaPlaybackItemChangedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct MediaBreak(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(MediaBreak, windows_core::IUnknown, windows_core::IInspectable);
impl MediaBreak {
    pub fn PlaybackList(&self) -> windows_core::Result<MediaPlaybackList> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PlaybackList)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn PresentationPosition(&self) -> windows_core::Result<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PresentationPosition)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn InsertionMethod(&self) -> windows_core::Result<MediaBreakInsertionMethod> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).InsertionMethod)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn CustomProperties(&self) -> windows_core::Result<super::super::Foundation::Collections::ValueSet> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CustomProperties)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CanStart(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CanStart)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetCanStart(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetCanStart)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Create(insertionmethod: MediaBreakInsertionMethod) -> windows_core::Result<MediaBreak> {
        Self::IMediaBreakFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Create)(windows_core::Interface::as_raw(this), insertionmethod, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateWithPresentationPosition(insertionmethod: MediaBreakInsertionMethod, presentationposition: super::super::Foundation::TimeSpan) -> windows_core::Result<MediaBreak> {
        Self::IMediaBreakFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateWithPresentationPosition)(windows_core::Interface::as_raw(this), insertionmethod, presentationposition, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IMediaBreakFactory<R, F: FnOnce(&IMediaBreakFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<MediaBreak, IMediaBreakFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for MediaBreak {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IMediaBreak>();
}
unsafe impl windows_core::Interface for MediaBreak {
    type Vtable = IMediaBreak_Vtbl;
    const IID: windows_core::GUID = <IMediaBreak as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for MediaBreak {
    const NAME: &'static str = "Windows.Media.Playback.MediaBreak";
}
unsafe impl Send for MediaBreak {}
unsafe impl Sync for MediaBreak {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct MediaBreakEndedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(MediaBreakEndedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl MediaBreakEndedEventArgs {
    pub fn MediaBreak(&self) -> windows_core::Result<MediaBreak> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MediaBreak)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for MediaBreakEndedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IMediaBreakEndedEventArgs>();
}
unsafe impl windows_core::Interface for MediaBreakEndedEventArgs {
    type Vtable = IMediaBreakEndedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IMediaBreakEndedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for MediaBreakEndedEventArgs {
    const NAME: &'static str = "Windows.Media.Playback.MediaBreakEndedEventArgs";
}
unsafe impl Send for MediaBreakEndedEventArgs {}
unsafe impl Sync for MediaBreakEndedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct MediaBreakManager(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(MediaBreakManager, windows_core::IUnknown, windows_core::IInspectable);
impl MediaBreakManager {
    pub fn BreaksSeekedOver<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<MediaBreakManager, MediaBreakSeekedOverEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BreaksSeekedOver)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveBreaksSeekedOver(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveBreaksSeekedOver)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn BreakStarted<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<MediaBreakManager, MediaBreakStartedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BreakStarted)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveBreakStarted(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveBreakStarted)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn BreakEnded<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<MediaBreakManager, MediaBreakEndedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BreakEnded)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveBreakEnded(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveBreakEnded)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn BreakSkipped<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<MediaBreakManager, MediaBreakSkippedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BreakSkipped)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveBreakSkipped(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveBreakSkipped)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn CurrentBreak(&self) -> windows_core::Result<MediaBreak> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CurrentBreak)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn PlaybackSession(&self) -> windows_core::Result<MediaPlaybackSession> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PlaybackSession)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn PlayBreak<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<MediaBreak>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).PlayBreak)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn SkipCurrentBreak(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SkipCurrentBreak)(windows_core::Interface::as_raw(this)).ok() }
    }
}
impl windows_core::RuntimeType for MediaBreakManager {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IMediaBreakManager>();
}
unsafe impl windows_core::Interface for MediaBreakManager {
    type Vtable = IMediaBreakManager_Vtbl;
    const IID: windows_core::GUID = <IMediaBreakManager as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for MediaBreakManager {
    const NAME: &'static str = "Windows.Media.Playback.MediaBreakManager";
}
unsafe impl Send for MediaBreakManager {}
unsafe impl Sync for MediaBreakManager {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct MediaBreakSchedule(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(MediaBreakSchedule, windows_core::IUnknown, windows_core::IInspectable);
impl MediaBreakSchedule {
    pub fn ScheduleChanged<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<MediaBreakSchedule, windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ScheduleChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveScheduleChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveScheduleChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn InsertMidrollBreak<P0>(&self, mediabreak: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<MediaBreak>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).InsertMidrollBreak)(windows_core::Interface::as_raw(this), mediabreak.param().abi()).ok() }
    }
    pub fn RemoveMidrollBreak<P0>(&self, mediabreak: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<MediaBreak>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveMidrollBreak)(windows_core::Interface::as_raw(this), mediabreak.param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn MidrollBreaks(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<MediaBreak>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MidrollBreaks)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetPrerollBreak<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<MediaBreak>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetPrerollBreak)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn PrerollBreak(&self) -> windows_core::Result<MediaBreak> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PrerollBreak)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetPostrollBreak<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<MediaBreak>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetPostrollBreak)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn PostrollBreak(&self) -> windows_core::Result<MediaBreak> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PostrollBreak)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn PlaybackItem(&self) -> windows_core::Result<MediaPlaybackItem> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PlaybackItem)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for MediaBreakSchedule {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IMediaBreakSchedule>();
}
unsafe impl windows_core::Interface for MediaBreakSchedule {
    type Vtable = IMediaBreakSchedule_Vtbl;
    const IID: windows_core::GUID = <IMediaBreakSchedule as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for MediaBreakSchedule {
    const NAME: &'static str = "Windows.Media.Playback.MediaBreakSchedule";
}
unsafe impl Send for MediaBreakSchedule {}
unsafe impl Sync for MediaBreakSchedule {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct MediaBreakSeekedOverEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(MediaBreakSeekedOverEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl MediaBreakSeekedOverEventArgs {
    #[cfg(feature = "Foundation_Collections")]
    pub fn SeekedOverBreaks(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<MediaBreak>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SeekedOverBreaks)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn OldPosition(&self) -> windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OldPosition)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn NewPosition(&self) -> windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NewPosition)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for MediaBreakSeekedOverEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IMediaBreakSeekedOverEventArgs>();
}
unsafe impl windows_core::Interface for MediaBreakSeekedOverEventArgs {
    type Vtable = IMediaBreakSeekedOverEventArgs_Vtbl;
    const IID: windows_core::GUID = <IMediaBreakSeekedOverEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for MediaBreakSeekedOverEventArgs {
    const NAME: &'static str = "Windows.Media.Playback.MediaBreakSeekedOverEventArgs";
}
unsafe impl Send for MediaBreakSeekedOverEventArgs {}
unsafe impl Sync for MediaBreakSeekedOverEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct MediaBreakSkippedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(MediaBreakSkippedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl MediaBreakSkippedEventArgs {
    pub fn MediaBreak(&self) -> windows_core::Result<MediaBreak> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MediaBreak)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for MediaBreakSkippedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IMediaBreakSkippedEventArgs>();
}
unsafe impl windows_core::Interface for MediaBreakSkippedEventArgs {
    type Vtable = IMediaBreakSkippedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IMediaBreakSkippedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for MediaBreakSkippedEventArgs {
    const NAME: &'static str = "Windows.Media.Playback.MediaBreakSkippedEventArgs";
}
unsafe impl Send for MediaBreakSkippedEventArgs {}
unsafe impl Sync for MediaBreakSkippedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct MediaBreakStartedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(MediaBreakStartedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl MediaBreakStartedEventArgs {
    pub fn MediaBreak(&self) -> windows_core::Result<MediaBreak> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MediaBreak)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for MediaBreakStartedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IMediaBreakStartedEventArgs>();
}
unsafe impl windows_core::Interface for MediaBreakStartedEventArgs {
    type Vtable = IMediaBreakStartedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IMediaBreakStartedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for MediaBreakStartedEventArgs {
    const NAME: &'static str = "Windows.Media.Playback.MediaBreakStartedEventArgs";
}
unsafe impl Send for MediaBreakStartedEventArgs {}
unsafe impl Sync for MediaBreakStartedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct MediaItemDisplayProperties(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(MediaItemDisplayProperties, windows_core::IUnknown, windows_core::IInspectable);
impl MediaItemDisplayProperties {
    pub fn Type(&self) -> windows_core::Result<super::MediaPlaybackType> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Type)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetType(&self, value: super::MediaPlaybackType) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetType)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn MusicProperties(&self) -> windows_core::Result<super::MusicDisplayProperties> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MusicProperties)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn VideoProperties(&self) -> windows_core::Result<super::VideoDisplayProperties> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).VideoProperties)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn Thumbnail(&self) -> windows_core::Result<super::super::Storage::Streams::RandomAccessStreamReference> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Thumbnail)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn SetThumbnail<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Storage::Streams::RandomAccessStreamReference>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetThumbnail)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn ClearAll(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).ClearAll)(windows_core::Interface::as_raw(this)).ok() }
    }
}
impl windows_core::RuntimeType for MediaItemDisplayProperties {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IMediaItemDisplayProperties>();
}
unsafe impl windows_core::Interface for MediaItemDisplayProperties {
    type Vtable = IMediaItemDisplayProperties_Vtbl;
    const IID: windows_core::GUID = <IMediaItemDisplayProperties as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for MediaItemDisplayProperties {
    const NAME: &'static str = "Windows.Media.Playback.MediaItemDisplayProperties";
}
unsafe impl Send for MediaItemDisplayProperties {}
unsafe impl Sync for MediaItemDisplayProperties {}
#[cfg(all(feature = "Foundation_Collections", feature = "Media_Core"))]
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct MediaPlaybackAudioTrackList(windows_core::IUnknown);
#[cfg(all(feature = "Foundation_Collections", feature = "Media_Core"))]
windows_core::imp::interface_hierarchy!(MediaPlaybackAudioTrackList, windows_core::IUnknown, windows_core::IInspectable);
#[cfg(all(feature = "Foundation_Collections", feature = "Media_Core"))]
windows_core::imp::required_hierarchy!(MediaPlaybackAudioTrackList, super::super::Foundation::Collections::IIterable::<super::Core::AudioTrack>, super::Core::ISingleSelectMediaTrackList, super::super::Foundation::Collections::IVectorView::<super::Core::AudioTrack>);
#[cfg(all(feature = "Foundation_Collections", feature = "Media_Core"))]
impl MediaPlaybackAudioTrackList {
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Core"))]
    pub fn First(&self) -> windows_core::Result<super::super::Foundation::Collections::IIterator<super::Core::AudioTrack>> {
        let this = &windows_core::Interface::cast::<super::super::Foundation::Collections::IIterable<super::Core::AudioTrack>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).First)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Media_Core")]
    pub fn SelectedIndexChanged<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<super::Core::ISingleSelectMediaTrackList, windows_core::IInspectable>>,
    {
        let this = &windows_core::Interface::cast::<super::Core::ISingleSelectMediaTrackList>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SelectedIndexChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Media_Core")]
    pub fn RemoveSelectedIndexChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::Core::ISingleSelectMediaTrackList>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemoveSelectedIndexChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
    #[cfg(feature = "Media_Core")]
    pub fn SetSelectedIndex(&self, value: i32) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::Core::ISingleSelectMediaTrackList>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetSelectedIndex)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "Media_Core")]
    pub fn SelectedIndex(&self) -> windows_core::Result<i32> {
        let this = &windows_core::Interface::cast::<super::Core::ISingleSelectMediaTrackList>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SelectedIndex)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Core"))]
    pub fn GetAt(&self, index: u32) -> windows_core::Result<super::Core::AudioTrack> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetAt)(windows_core::Interface::as_raw(this), index, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Core"))]
    pub fn Size(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Size)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Core"))]
    pub fn IndexOf<P0>(&self, value: P0, index: &mut u32) -> windows_core::Result<bool>
    where
        P0: windows_core::Param<super::Core::AudioTrack>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IndexOf)(windows_core::Interface::as_raw(this), value.param().abi(), index, &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Core"))]
    pub fn GetMany(&self, startindex: u32, items: &mut [Option<super::Core::AudioTrack>]) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetMany)(windows_core::Interface::as_raw(this), startindex, items.len().try_into().unwrap(), core::mem::transmute_copy(&items), &mut result__).map(|| result__)
        }
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "Media_Core"))]
impl windows_core::RuntimeType for MediaPlaybackAudioTrackList {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, super::super::Foundation::Collections::IVectorView<super::Core::AudioTrack>>();
}
#[cfg(all(feature = "Foundation_Collections", feature = "Media_Core"))]
unsafe impl windows_core::Interface for MediaPlaybackAudioTrackList {
    type Vtable = super::super::Foundation::Collections::IVectorView_Vtbl<super::Core::AudioTrack>;
    const IID: windows_core::GUID = <super::super::Foundation::Collections::IVectorView<super::Core::AudioTrack> as windows_core::Interface>::IID;
}
#[cfg(all(feature = "Foundation_Collections", feature = "Media_Core"))]
impl windows_core::RuntimeName for MediaPlaybackAudioTrackList {
    const NAME: &'static str = "Windows.Media.Playback.MediaPlaybackAudioTrackList";
}
#[cfg(all(feature = "Foundation_Collections", feature = "Media_Core"))]
impl IntoIterator for MediaPlaybackAudioTrackList {
    type Item = super::Core::AudioTrack;
    type IntoIter = super::super::Foundation::Collections::VectorViewIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        IntoIterator::into_iter(&self)
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "Media_Core"))]
impl IntoIterator for &MediaPlaybackAudioTrackList {
    type Item = super::Core::AudioTrack;
    type IntoIter = super::super::Foundation::Collections::VectorViewIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        super::super::Foundation::Collections::VectorViewIterator::new(windows_core::Interface::cast(self).ok())
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "Media_Core"))]
unsafe impl Send for MediaPlaybackAudioTrackList {}
#[cfg(all(feature = "Foundation_Collections", feature = "Media_Core"))]
unsafe impl Sync for MediaPlaybackAudioTrackList {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct MediaPlaybackCommandManager(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(MediaPlaybackCommandManager, windows_core::IUnknown, windows_core::IInspectable);
impl MediaPlaybackCommandManager {
    pub fn IsEnabled(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsEnabled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsEnabled(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetIsEnabled)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn MediaPlayer(&self) -> windows_core::Result<MediaPlayer> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MediaPlayer)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn PlayBehavior(&self) -> windows_core::Result<MediaPlaybackCommandManagerCommandBehavior> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PlayBehavior)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn PauseBehavior(&self) -> windows_core::Result<MediaPlaybackCommandManagerCommandBehavior> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PauseBehavior)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn NextBehavior(&self) -> windows_core::Result<MediaPlaybackCommandManagerCommandBehavior> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NextBehavior)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn PreviousBehavior(&self) -> windows_core::Result<MediaPlaybackCommandManagerCommandBehavior> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PreviousBehavior)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn FastForwardBehavior(&self) -> windows_core::Result<MediaPlaybackCommandManagerCommandBehavior> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FastForwardBehavior)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn RewindBehavior(&self) -> windows_core::Result<MediaPlaybackCommandManagerCommandBehavior> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RewindBehavior)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ShuffleBehavior(&self) -> windows_core::Result<MediaPlaybackCommandManagerCommandBehavior> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ShuffleBehavior)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn AutoRepeatModeBehavior(&self) -> windows_core::Result<MediaPlaybackCommandManagerCommandBehavior> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AutoRepeatModeBehavior)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn PositionBehavior(&self) -> windows_core::Result<MediaPlaybackCommandManagerCommandBehavior> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PositionBehavior)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn RateBehavior(&self) -> windows_core::Result<MediaPlaybackCommandManagerCommandBehavior> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RateBehavior)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn PlayReceived<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<MediaPlaybackCommandManager, MediaPlaybackCommandManagerPlayReceivedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PlayReceived)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemovePlayReceived(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemovePlayReceived)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn PauseReceived<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<MediaPlaybackCommandManager, MediaPlaybackCommandManagerPauseReceivedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PauseReceived)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemovePauseReceived(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemovePauseReceived)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn NextReceived<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<MediaPlaybackCommandManager, MediaPlaybackCommandManagerNextReceivedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NextReceived)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveNextReceived(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveNextReceived)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn PreviousReceived<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<MediaPlaybackCommandManager, MediaPlaybackCommandManagerPreviousReceivedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PreviousReceived)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemovePreviousReceived(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemovePreviousReceived)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn FastForwardReceived<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<MediaPlaybackCommandManager, MediaPlaybackCommandManagerFastForwardReceivedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FastForwardReceived)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveFastForwardReceived(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveFastForwardReceived)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn RewindReceived<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<MediaPlaybackCommandManager, MediaPlaybackCommandManagerRewindReceivedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RewindReceived)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveRewindReceived(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveRewindReceived)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn ShuffleReceived<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<MediaPlaybackCommandManager, MediaPlaybackCommandManagerShuffleReceivedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ShuffleReceived)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveShuffleReceived(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveShuffleReceived)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn AutoRepeatModeReceived<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<MediaPlaybackCommandManager, MediaPlaybackCommandManagerAutoRepeatModeReceivedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AutoRepeatModeReceived)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveAutoRepeatModeReceived(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveAutoRepeatModeReceived)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn PositionReceived<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<MediaPlaybackCommandManager, MediaPlaybackCommandManagerPositionReceivedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PositionReceived)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemovePositionReceived(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemovePositionReceived)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn RateReceived<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<MediaPlaybackCommandManager, MediaPlaybackCommandManagerRateReceivedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RateReceived)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveRateReceived(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveRateReceived)(windows_core::Interface::as_raw(this), token).ok() }
    }
}
impl windows_core::RuntimeType for MediaPlaybackCommandManager {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IMediaPlaybackCommandManager>();
}
unsafe impl windows_core::Interface for MediaPlaybackCommandManager {
    type Vtable = IMediaPlaybackCommandManager_Vtbl;
    const IID: windows_core::GUID = <IMediaPlaybackCommandManager as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for MediaPlaybackCommandManager {
    const NAME: &'static str = "Windows.Media.Playback.MediaPlaybackCommandManager";
}
unsafe impl Send for MediaPlaybackCommandManager {}
unsafe impl Sync for MediaPlaybackCommandManager {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct MediaPlaybackCommandManagerAutoRepeatModeReceivedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(MediaPlaybackCommandManagerAutoRepeatModeReceivedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl MediaPlaybackCommandManagerAutoRepeatModeReceivedEventArgs {
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
    pub fn AutoRepeatMode(&self) -> windows_core::Result<super::MediaPlaybackAutoRepeatMode> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AutoRepeatMode)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
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
impl windows_core::RuntimeType for MediaPlaybackCommandManagerAutoRepeatModeReceivedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IMediaPlaybackCommandManagerAutoRepeatModeReceivedEventArgs>();
}
unsafe impl windows_core::Interface for MediaPlaybackCommandManagerAutoRepeatModeReceivedEventArgs {
    type Vtable = IMediaPlaybackCommandManagerAutoRepeatModeReceivedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IMediaPlaybackCommandManagerAutoRepeatModeReceivedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for MediaPlaybackCommandManagerAutoRepeatModeReceivedEventArgs {
    const NAME: &'static str = "Windows.Media.Playback.MediaPlaybackCommandManagerAutoRepeatModeReceivedEventArgs";
}
unsafe impl Send for MediaPlaybackCommandManagerAutoRepeatModeReceivedEventArgs {}
unsafe impl Sync for MediaPlaybackCommandManagerAutoRepeatModeReceivedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct MediaPlaybackCommandManagerCommandBehavior(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(MediaPlaybackCommandManagerCommandBehavior, windows_core::IUnknown, windows_core::IInspectable);
impl MediaPlaybackCommandManagerCommandBehavior {
    pub fn CommandManager(&self) -> windows_core::Result<MediaPlaybackCommandManager> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CommandManager)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn IsEnabled(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsEnabled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn EnablingRule(&self) -> windows_core::Result<MediaCommandEnablingRule> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).EnablingRule)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetEnablingRule(&self, value: MediaCommandEnablingRule) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetEnablingRule)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsEnabledChanged<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<MediaPlaybackCommandManagerCommandBehavior, windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsEnabledChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveIsEnabledChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveIsEnabledChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
}
impl windows_core::RuntimeType for MediaPlaybackCommandManagerCommandBehavior {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IMediaPlaybackCommandManagerCommandBehavior>();
}
unsafe impl windows_core::Interface for MediaPlaybackCommandManagerCommandBehavior {
    type Vtable = IMediaPlaybackCommandManagerCommandBehavior_Vtbl;
    const IID: windows_core::GUID = <IMediaPlaybackCommandManagerCommandBehavior as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for MediaPlaybackCommandManagerCommandBehavior {
    const NAME: &'static str = "Windows.Media.Playback.MediaPlaybackCommandManagerCommandBehavior";
}
unsafe impl Send for MediaPlaybackCommandManagerCommandBehavior {}
unsafe impl Sync for MediaPlaybackCommandManagerCommandBehavior {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct MediaPlaybackCommandManagerFastForwardReceivedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(MediaPlaybackCommandManagerFastForwardReceivedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl MediaPlaybackCommandManagerFastForwardReceivedEventArgs {
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
    pub fn GetDeferral(&self) -> windows_core::Result<super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDeferral)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for MediaPlaybackCommandManagerFastForwardReceivedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IMediaPlaybackCommandManagerFastForwardReceivedEventArgs>();
}
unsafe impl windows_core::Interface for MediaPlaybackCommandManagerFastForwardReceivedEventArgs {
    type Vtable = IMediaPlaybackCommandManagerFastForwardReceivedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IMediaPlaybackCommandManagerFastForwardReceivedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for MediaPlaybackCommandManagerFastForwardReceivedEventArgs {
    const NAME: &'static str = "Windows.Media.Playback.MediaPlaybackCommandManagerFastForwardReceivedEventArgs";
}
unsafe impl Send for MediaPlaybackCommandManagerFastForwardReceivedEventArgs {}
unsafe impl Sync for MediaPlaybackCommandManagerFastForwardReceivedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct MediaPlaybackCommandManagerNextReceivedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(MediaPlaybackCommandManagerNextReceivedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl MediaPlaybackCommandManagerNextReceivedEventArgs {
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
    pub fn GetDeferral(&self) -> windows_core::Result<super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDeferral)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for MediaPlaybackCommandManagerNextReceivedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IMediaPlaybackCommandManagerNextReceivedEventArgs>();
}
unsafe impl windows_core::Interface for MediaPlaybackCommandManagerNextReceivedEventArgs {
    type Vtable = IMediaPlaybackCommandManagerNextReceivedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IMediaPlaybackCommandManagerNextReceivedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for MediaPlaybackCommandManagerNextReceivedEventArgs {
    const NAME: &'static str = "Windows.Media.Playback.MediaPlaybackCommandManagerNextReceivedEventArgs";
}
unsafe impl Send for MediaPlaybackCommandManagerNextReceivedEventArgs {}
unsafe impl Sync for MediaPlaybackCommandManagerNextReceivedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct MediaPlaybackCommandManagerPauseReceivedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(MediaPlaybackCommandManagerPauseReceivedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl MediaPlaybackCommandManagerPauseReceivedEventArgs {
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
    pub fn GetDeferral(&self) -> windows_core::Result<super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDeferral)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for MediaPlaybackCommandManagerPauseReceivedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IMediaPlaybackCommandManagerPauseReceivedEventArgs>();
}
unsafe impl windows_core::Interface for MediaPlaybackCommandManagerPauseReceivedEventArgs {
    type Vtable = IMediaPlaybackCommandManagerPauseReceivedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IMediaPlaybackCommandManagerPauseReceivedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for MediaPlaybackCommandManagerPauseReceivedEventArgs {
    const NAME: &'static str = "Windows.Media.Playback.MediaPlaybackCommandManagerPauseReceivedEventArgs";
}
unsafe impl Send for MediaPlaybackCommandManagerPauseReceivedEventArgs {}
unsafe impl Sync for MediaPlaybackCommandManagerPauseReceivedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct MediaPlaybackCommandManagerPlayReceivedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(MediaPlaybackCommandManagerPlayReceivedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl MediaPlaybackCommandManagerPlayReceivedEventArgs {
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
    pub fn GetDeferral(&self) -> windows_core::Result<super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDeferral)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for MediaPlaybackCommandManagerPlayReceivedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IMediaPlaybackCommandManagerPlayReceivedEventArgs>();
}
unsafe impl windows_core::Interface for MediaPlaybackCommandManagerPlayReceivedEventArgs {
    type Vtable = IMediaPlaybackCommandManagerPlayReceivedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IMediaPlaybackCommandManagerPlayReceivedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for MediaPlaybackCommandManagerPlayReceivedEventArgs {
    const NAME: &'static str = "Windows.Media.Playback.MediaPlaybackCommandManagerPlayReceivedEventArgs";
}
unsafe impl Send for MediaPlaybackCommandManagerPlayReceivedEventArgs {}
unsafe impl Sync for MediaPlaybackCommandManagerPlayReceivedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct MediaPlaybackCommandManagerPositionReceivedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(MediaPlaybackCommandManagerPositionReceivedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl MediaPlaybackCommandManagerPositionReceivedEventArgs {
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
    pub fn Position(&self) -> windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Position)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
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
impl windows_core::RuntimeType for MediaPlaybackCommandManagerPositionReceivedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IMediaPlaybackCommandManagerPositionReceivedEventArgs>();
}
unsafe impl windows_core::Interface for MediaPlaybackCommandManagerPositionReceivedEventArgs {
    type Vtable = IMediaPlaybackCommandManagerPositionReceivedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IMediaPlaybackCommandManagerPositionReceivedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for MediaPlaybackCommandManagerPositionReceivedEventArgs {
    const NAME: &'static str = "Windows.Media.Playback.MediaPlaybackCommandManagerPositionReceivedEventArgs";
}
unsafe impl Send for MediaPlaybackCommandManagerPositionReceivedEventArgs {}
unsafe impl Sync for MediaPlaybackCommandManagerPositionReceivedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct MediaPlaybackCommandManagerPreviousReceivedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(MediaPlaybackCommandManagerPreviousReceivedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl MediaPlaybackCommandManagerPreviousReceivedEventArgs {
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
    pub fn GetDeferral(&self) -> windows_core::Result<super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDeferral)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for MediaPlaybackCommandManagerPreviousReceivedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IMediaPlaybackCommandManagerPreviousReceivedEventArgs>();
}
unsafe impl windows_core::Interface for MediaPlaybackCommandManagerPreviousReceivedEventArgs {
    type Vtable = IMediaPlaybackCommandManagerPreviousReceivedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IMediaPlaybackCommandManagerPreviousReceivedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for MediaPlaybackCommandManagerPreviousReceivedEventArgs {
    const NAME: &'static str = "Windows.Media.Playback.MediaPlaybackCommandManagerPreviousReceivedEventArgs";
}
unsafe impl Send for MediaPlaybackCommandManagerPreviousReceivedEventArgs {}
unsafe impl Sync for MediaPlaybackCommandManagerPreviousReceivedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct MediaPlaybackCommandManagerRateReceivedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(MediaPlaybackCommandManagerRateReceivedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl MediaPlaybackCommandManagerRateReceivedEventArgs {
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
    pub fn PlaybackRate(&self) -> windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PlaybackRate)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
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
impl windows_core::RuntimeType for MediaPlaybackCommandManagerRateReceivedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IMediaPlaybackCommandManagerRateReceivedEventArgs>();
}
unsafe impl windows_core::Interface for MediaPlaybackCommandManagerRateReceivedEventArgs {
    type Vtable = IMediaPlaybackCommandManagerRateReceivedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IMediaPlaybackCommandManagerRateReceivedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for MediaPlaybackCommandManagerRateReceivedEventArgs {
    const NAME: &'static str = "Windows.Media.Playback.MediaPlaybackCommandManagerRateReceivedEventArgs";
}
unsafe impl Send for MediaPlaybackCommandManagerRateReceivedEventArgs {}
unsafe impl Sync for MediaPlaybackCommandManagerRateReceivedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct MediaPlaybackCommandManagerRewindReceivedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(MediaPlaybackCommandManagerRewindReceivedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl MediaPlaybackCommandManagerRewindReceivedEventArgs {
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
    pub fn GetDeferral(&self) -> windows_core::Result<super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDeferral)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for MediaPlaybackCommandManagerRewindReceivedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IMediaPlaybackCommandManagerRewindReceivedEventArgs>();
}
unsafe impl windows_core::Interface for MediaPlaybackCommandManagerRewindReceivedEventArgs {
    type Vtable = IMediaPlaybackCommandManagerRewindReceivedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IMediaPlaybackCommandManagerRewindReceivedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for MediaPlaybackCommandManagerRewindReceivedEventArgs {
    const NAME: &'static str = "Windows.Media.Playback.MediaPlaybackCommandManagerRewindReceivedEventArgs";
}
unsafe impl Send for MediaPlaybackCommandManagerRewindReceivedEventArgs {}
unsafe impl Sync for MediaPlaybackCommandManagerRewindReceivedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct MediaPlaybackCommandManagerShuffleReceivedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(MediaPlaybackCommandManagerShuffleReceivedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl MediaPlaybackCommandManagerShuffleReceivedEventArgs {
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
    pub fn IsShuffleRequested(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsShuffleRequested)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
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
impl windows_core::RuntimeType for MediaPlaybackCommandManagerShuffleReceivedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IMediaPlaybackCommandManagerShuffleReceivedEventArgs>();
}
unsafe impl windows_core::Interface for MediaPlaybackCommandManagerShuffleReceivedEventArgs {
    type Vtable = IMediaPlaybackCommandManagerShuffleReceivedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IMediaPlaybackCommandManagerShuffleReceivedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for MediaPlaybackCommandManagerShuffleReceivedEventArgs {
    const NAME: &'static str = "Windows.Media.Playback.MediaPlaybackCommandManagerShuffleReceivedEventArgs";
}
unsafe impl Send for MediaPlaybackCommandManagerShuffleReceivedEventArgs {}
unsafe impl Sync for MediaPlaybackCommandManagerShuffleReceivedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct MediaPlaybackItem(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(MediaPlaybackItem, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(MediaPlaybackItem, IMediaPlaybackSource);
impl MediaPlaybackItem {
    #[cfg(feature = "Foundation_Collections")]
    pub fn AudioTracksChanged<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<MediaPlaybackItem, super::super::Foundation::Collections::IVectorChangedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AudioTracksChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveAudioTracksChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveAudioTracksChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn VideoTracksChanged<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<MediaPlaybackItem, super::super::Foundation::Collections::IVectorChangedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).VideoTracksChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveVideoTracksChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveVideoTracksChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn TimedMetadataTracksChanged<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<MediaPlaybackItem, super::super::Foundation::Collections::IVectorChangedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TimedMetadataTracksChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveTimedMetadataTracksChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveTimedMetadataTracksChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
    #[cfg(feature = "Media_Core")]
    pub fn Source(&self) -> windows_core::Result<super::Core::MediaSource> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Source)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Core"))]
    pub fn AudioTracks(&self) -> windows_core::Result<MediaPlaybackAudioTrackList> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AudioTracks)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Core"))]
    pub fn VideoTracks(&self) -> windows_core::Result<MediaPlaybackVideoTrackList> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).VideoTracks)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Core"))]
    pub fn TimedMetadataTracks(&self) -> windows_core::Result<MediaPlaybackTimedMetadataTrackList> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TimedMetadataTracks)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn BreakSchedule(&self) -> windows_core::Result<MediaBreakSchedule> {
        let this = &windows_core::Interface::cast::<IMediaPlaybackItem2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BreakSchedule)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn StartTime(&self) -> windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = &windows_core::Interface::cast::<IMediaPlaybackItem2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).StartTime)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn DurationLimit(&self) -> windows_core::Result<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>> {
        let this = &windows_core::Interface::cast::<IMediaPlaybackItem2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DurationLimit)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CanSkip(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IMediaPlaybackItem2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CanSkip)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetCanSkip(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IMediaPlaybackItem2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetCanSkip)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn GetDisplayProperties(&self) -> windows_core::Result<MediaItemDisplayProperties> {
        let this = &windows_core::Interface::cast::<IMediaPlaybackItem2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDisplayProperties)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ApplyDisplayProperties<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<MediaItemDisplayProperties>,
    {
        let this = &windows_core::Interface::cast::<IMediaPlaybackItem2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).ApplyDisplayProperties)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn IsDisabledInPlaybackList(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IMediaPlaybackItem3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsDisabledInPlaybackList)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsDisabledInPlaybackList(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IMediaPlaybackItem3>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetIsDisabledInPlaybackList)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn TotalDownloadProgress(&self) -> windows_core::Result<f64> {
        let this = &windows_core::Interface::cast::<IMediaPlaybackItem3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TotalDownloadProgress)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn AutoLoadedDisplayProperties(&self) -> windows_core::Result<AutoLoadedDisplayPropertyKind> {
        let this = &windows_core::Interface::cast::<IMediaPlaybackItem3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AutoLoadedDisplayProperties)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetAutoLoadedDisplayProperties(&self, value: AutoLoadedDisplayPropertyKind) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IMediaPlaybackItem3>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetAutoLoadedDisplayProperties)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "Media_Core")]
    pub fn Create<P0>(source: P0) -> windows_core::Result<MediaPlaybackItem>
    where
        P0: windows_core::Param<super::Core::MediaSource>,
    {
        Self::IMediaPlaybackItemFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Create)(windows_core::Interface::as_raw(this), source.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Media_Core")]
    pub fn CreateWithStartTime<P0>(source: P0, starttime: super::super::Foundation::TimeSpan) -> windows_core::Result<MediaPlaybackItem>
    where
        P0: windows_core::Param<super::Core::MediaSource>,
    {
        Self::IMediaPlaybackItemFactory2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateWithStartTime)(windows_core::Interface::as_raw(this), source.param().abi(), starttime, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Media_Core")]
    pub fn CreateWithStartTimeAndDurationLimit<P0>(source: P0, starttime: super::super::Foundation::TimeSpan, durationlimit: super::super::Foundation::TimeSpan) -> windows_core::Result<MediaPlaybackItem>
    where
        P0: windows_core::Param<super::Core::MediaSource>,
    {
        Self::IMediaPlaybackItemFactory2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateWithStartTimeAndDurationLimit)(windows_core::Interface::as_raw(this), source.param().abi(), starttime, durationlimit, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Media_Core")]
    pub fn FindFromMediaSource<P0>(source: P0) -> windows_core::Result<MediaPlaybackItem>
    where
        P0: windows_core::Param<super::Core::MediaSource>,
    {
        Self::IMediaPlaybackItemStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FindFromMediaSource)(windows_core::Interface::as_raw(this), source.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IMediaPlaybackItemFactory<R, F: FnOnce(&IMediaPlaybackItemFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<MediaPlaybackItem, IMediaPlaybackItemFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IMediaPlaybackItemFactory2<R, F: FnOnce(&IMediaPlaybackItemFactory2) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<MediaPlaybackItem, IMediaPlaybackItemFactory2> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IMediaPlaybackItemStatics<R, F: FnOnce(&IMediaPlaybackItemStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<MediaPlaybackItem, IMediaPlaybackItemStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for MediaPlaybackItem {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IMediaPlaybackItem>();
}
unsafe impl windows_core::Interface for MediaPlaybackItem {
    type Vtable = IMediaPlaybackItem_Vtbl;
    const IID: windows_core::GUID = <IMediaPlaybackItem as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for MediaPlaybackItem {
    const NAME: &'static str = "Windows.Media.Playback.MediaPlaybackItem";
}
unsafe impl Send for MediaPlaybackItem {}
unsafe impl Sync for MediaPlaybackItem {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct MediaPlaybackItemError(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(MediaPlaybackItemError, windows_core::IUnknown, windows_core::IInspectable);
impl MediaPlaybackItemError {
    pub fn ErrorCode(&self) -> windows_core::Result<MediaPlaybackItemErrorCode> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ErrorCode)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ExtendedError(&self) -> windows_core::Result<windows_core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ExtendedError)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for MediaPlaybackItemError {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IMediaPlaybackItemError>();
}
unsafe impl windows_core::Interface for MediaPlaybackItemError {
    type Vtable = IMediaPlaybackItemError_Vtbl;
    const IID: windows_core::GUID = <IMediaPlaybackItemError as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for MediaPlaybackItemError {
    const NAME: &'static str = "Windows.Media.Playback.MediaPlaybackItemError";
}
unsafe impl Send for MediaPlaybackItemError {}
unsafe impl Sync for MediaPlaybackItemError {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct MediaPlaybackItemFailedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(MediaPlaybackItemFailedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl MediaPlaybackItemFailedEventArgs {
    pub fn Item(&self) -> windows_core::Result<MediaPlaybackItem> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Item)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Error(&self) -> windows_core::Result<MediaPlaybackItemError> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Error)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for MediaPlaybackItemFailedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IMediaPlaybackItemFailedEventArgs>();
}
unsafe impl windows_core::Interface for MediaPlaybackItemFailedEventArgs {
    type Vtable = IMediaPlaybackItemFailedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IMediaPlaybackItemFailedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for MediaPlaybackItemFailedEventArgs {
    const NAME: &'static str = "Windows.Media.Playback.MediaPlaybackItemFailedEventArgs";
}
unsafe impl Send for MediaPlaybackItemFailedEventArgs {}
unsafe impl Sync for MediaPlaybackItemFailedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct MediaPlaybackItemOpenedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(MediaPlaybackItemOpenedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl MediaPlaybackItemOpenedEventArgs {
    pub fn Item(&self) -> windows_core::Result<MediaPlaybackItem> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Item)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for MediaPlaybackItemOpenedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IMediaPlaybackItemOpenedEventArgs>();
}
unsafe impl windows_core::Interface for MediaPlaybackItemOpenedEventArgs {
    type Vtable = IMediaPlaybackItemOpenedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IMediaPlaybackItemOpenedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for MediaPlaybackItemOpenedEventArgs {
    const NAME: &'static str = "Windows.Media.Playback.MediaPlaybackItemOpenedEventArgs";
}
unsafe impl Send for MediaPlaybackItemOpenedEventArgs {}
unsafe impl Sync for MediaPlaybackItemOpenedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct MediaPlaybackList(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(MediaPlaybackList, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(MediaPlaybackList, IMediaPlaybackSource);
impl MediaPlaybackList {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<MediaPlaybackList, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn ItemFailed<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<MediaPlaybackList, MediaPlaybackItemFailedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ItemFailed)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveItemFailed(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveItemFailed)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn CurrentItemChanged<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<MediaPlaybackList, CurrentMediaPlaybackItemChangedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CurrentItemChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveCurrentItemChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveCurrentItemChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn ItemOpened<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<MediaPlaybackList, MediaPlaybackItemOpenedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ItemOpened)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveItemOpened(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveItemOpened)(windows_core::Interface::as_raw(this), token).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Items(&self) -> windows_core::Result<super::super::Foundation::Collections::IObservableVector<MediaPlaybackItem>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Items)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn AutoRepeatEnabled(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AutoRepeatEnabled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetAutoRepeatEnabled(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetAutoRepeatEnabled)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ShuffleEnabled(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ShuffleEnabled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetShuffleEnabled(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetShuffleEnabled)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn CurrentItem(&self) -> windows_core::Result<MediaPlaybackItem> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CurrentItem)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CurrentItemIndex(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CurrentItemIndex)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn MoveNext(&self) -> windows_core::Result<MediaPlaybackItem> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MoveNext)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn MovePrevious(&self) -> windows_core::Result<MediaPlaybackItem> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MovePrevious)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn MoveTo(&self, itemindex: u32) -> windows_core::Result<MediaPlaybackItem> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MoveTo)(windows_core::Interface::as_raw(this), itemindex, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn MaxPrefetchTime(&self) -> windows_core::Result<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>> {
        let this = &windows_core::Interface::cast::<IMediaPlaybackList2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MaxPrefetchTime)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetMaxPrefetchTime<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>,
    {
        let this = &windows_core::Interface::cast::<IMediaPlaybackList2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetMaxPrefetchTime)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn StartingItem(&self) -> windows_core::Result<MediaPlaybackItem> {
        let this = &windows_core::Interface::cast::<IMediaPlaybackList2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).StartingItem)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetStartingItem<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<MediaPlaybackItem>,
    {
        let this = &windows_core::Interface::cast::<IMediaPlaybackList2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetStartingItem)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn ShuffledItems(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<MediaPlaybackItem>> {
        let this = &windows_core::Interface::cast::<IMediaPlaybackList2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ShuffledItems)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn SetShuffledItems<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::Collections::IIterable<MediaPlaybackItem>>,
    {
        let this = &windows_core::Interface::cast::<IMediaPlaybackList2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetShuffledItems)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn MaxPlayedItemsToKeepOpen(&self) -> windows_core::Result<super::super::Foundation::IReference<u32>> {
        let this = &windows_core::Interface::cast::<IMediaPlaybackList3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MaxPlayedItemsToKeepOpen)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetMaxPlayedItemsToKeepOpen<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::IReference<u32>>,
    {
        let this = &windows_core::Interface::cast::<IMediaPlaybackList3>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetMaxPlayedItemsToKeepOpen)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
}
impl windows_core::RuntimeType for MediaPlaybackList {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IMediaPlaybackList>();
}
unsafe impl windows_core::Interface for MediaPlaybackList {
    type Vtable = IMediaPlaybackList_Vtbl;
    const IID: windows_core::GUID = <IMediaPlaybackList as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for MediaPlaybackList {
    const NAME: &'static str = "Windows.Media.Playback.MediaPlaybackList";
}
unsafe impl Send for MediaPlaybackList {}
unsafe impl Sync for MediaPlaybackList {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct MediaPlaybackSession(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(MediaPlaybackSession, windows_core::IUnknown, windows_core::IInspectable);
impl MediaPlaybackSession {
    pub fn PlaybackStateChanged<P0>(&self, value: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<MediaPlaybackSession, windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PlaybackStateChanged)(windows_core::Interface::as_raw(this), value.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemovePlaybackStateChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemovePlaybackStateChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn PlaybackRateChanged<P0>(&self, value: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<MediaPlaybackSession, windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PlaybackRateChanged)(windows_core::Interface::as_raw(this), value.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemovePlaybackRateChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemovePlaybackRateChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn SeekCompleted<P0>(&self, value: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<MediaPlaybackSession, windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SeekCompleted)(windows_core::Interface::as_raw(this), value.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveSeekCompleted(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveSeekCompleted)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn BufferingStarted<P0>(&self, value: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<MediaPlaybackSession, windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BufferingStarted)(windows_core::Interface::as_raw(this), value.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveBufferingStarted(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveBufferingStarted)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn BufferingEnded<P0>(&self, value: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<MediaPlaybackSession, windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BufferingEnded)(windows_core::Interface::as_raw(this), value.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveBufferingEnded(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveBufferingEnded)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn BufferingProgressChanged<P0>(&self, value: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<MediaPlaybackSession, windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BufferingProgressChanged)(windows_core::Interface::as_raw(this), value.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveBufferingProgressChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveBufferingProgressChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn DownloadProgressChanged<P0>(&self, value: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<MediaPlaybackSession, windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DownloadProgressChanged)(windows_core::Interface::as_raw(this), value.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveDownloadProgressChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveDownloadProgressChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn NaturalDurationChanged<P0>(&self, value: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<MediaPlaybackSession, windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NaturalDurationChanged)(windows_core::Interface::as_raw(this), value.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveNaturalDurationChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveNaturalDurationChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn PositionChanged<P0>(&self, value: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<MediaPlaybackSession, windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PositionChanged)(windows_core::Interface::as_raw(this), value.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemovePositionChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemovePositionChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn NaturalVideoSizeChanged<P0>(&self, value: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<MediaPlaybackSession, windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NaturalVideoSizeChanged)(windows_core::Interface::as_raw(this), value.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveNaturalVideoSizeChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveNaturalVideoSizeChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn MediaPlayer(&self) -> windows_core::Result<MediaPlayer> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MediaPlayer)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn NaturalDuration(&self) -> windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NaturalDuration)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Position(&self) -> windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Position)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetPosition(&self, value: super::super::Foundation::TimeSpan) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetPosition)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn PlaybackState(&self) -> windows_core::Result<MediaPlaybackState> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PlaybackState)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn CanSeek(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CanSeek)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn CanPause(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CanPause)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsProtected(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsProtected)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PlaybackRate(&self) -> windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PlaybackRate)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetPlaybackRate(&self, value: f64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetPlaybackRate)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn BufferingProgress(&self) -> windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BufferingProgress)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn DownloadProgress(&self) -> windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DownloadProgress)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn NaturalVideoHeight(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NaturalVideoHeight)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn NaturalVideoWidth(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NaturalVideoWidth)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn NormalizedSourceRect(&self) -> windows_core::Result<super::super::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NormalizedSourceRect)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetNormalizedSourceRect(&self, value: super::super::Foundation::Rect) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetNormalizedSourceRect)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "Media_MediaProperties")]
    pub fn StereoscopicVideoPackingMode(&self) -> windows_core::Result<super::MediaProperties::StereoscopicVideoPackingMode> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).StereoscopicVideoPackingMode)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Media_MediaProperties")]
    pub fn SetStereoscopicVideoPackingMode(&self, value: super::MediaProperties::StereoscopicVideoPackingMode) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetStereoscopicVideoPackingMode)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn BufferedRangesChanged<P0>(&self, value: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<MediaPlaybackSession, windows_core::IInspectable>>,
    {
        let this = &windows_core::Interface::cast::<IMediaPlaybackSession2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BufferedRangesChanged)(windows_core::Interface::as_raw(this), value.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveBufferedRangesChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IMediaPlaybackSession2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemoveBufferedRangesChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn PlayedRangesChanged<P0>(&self, value: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<MediaPlaybackSession, windows_core::IInspectable>>,
    {
        let this = &windows_core::Interface::cast::<IMediaPlaybackSession2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PlayedRangesChanged)(windows_core::Interface::as_raw(this), value.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemovePlayedRangesChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IMediaPlaybackSession2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemovePlayedRangesChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn SeekableRangesChanged<P0>(&self, value: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<MediaPlaybackSession, windows_core::IInspectable>>,
    {
        let this = &windows_core::Interface::cast::<IMediaPlaybackSession2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SeekableRangesChanged)(windows_core::Interface::as_raw(this), value.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveSeekableRangesChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IMediaPlaybackSession2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemoveSeekableRangesChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn SupportedPlaybackRatesChanged<P0>(&self, value: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<MediaPlaybackSession, windows_core::IInspectable>>,
    {
        let this = &windows_core::Interface::cast::<IMediaPlaybackSession2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SupportedPlaybackRatesChanged)(windows_core::Interface::as_raw(this), value.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveSupportedPlaybackRatesChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IMediaPlaybackSession2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemoveSupportedPlaybackRatesChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn SphericalVideoProjection(&self) -> windows_core::Result<MediaPlaybackSphericalVideoProjection> {
        let this = &windows_core::Interface::cast::<IMediaPlaybackSession2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SphericalVideoProjection)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn IsMirroring(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IMediaPlaybackSession2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsMirroring)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsMirroring(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IMediaPlaybackSession2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetIsMirroring)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetBufferedRanges(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<super::MediaTimeRange>> {
        let this = &windows_core::Interface::cast::<IMediaPlaybackSession2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetBufferedRanges)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetPlayedRanges(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<super::MediaTimeRange>> {
        let this = &windows_core::Interface::cast::<IMediaPlaybackSession2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetPlayedRanges)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetSeekableRanges(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<super::MediaTimeRange>> {
        let this = &windows_core::Interface::cast::<IMediaPlaybackSession2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetSeekableRanges)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn IsSupportedPlaybackRateRange(&self, rate1: f64, rate2: f64) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IMediaPlaybackSession2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsSupportedPlaybackRateRange)(windows_core::Interface::as_raw(this), rate1, rate2, &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Media_MediaProperties")]
    pub fn PlaybackRotation(&self) -> windows_core::Result<super::MediaProperties::MediaRotation> {
        let this = &windows_core::Interface::cast::<IMediaPlaybackSession3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PlaybackRotation)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Media_MediaProperties")]
    pub fn SetPlaybackRotation(&self, value: super::MediaProperties::MediaRotation) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IMediaPlaybackSession3>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetPlaybackRotation)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn GetOutputDegradationPolicyState(&self) -> windows_core::Result<MediaPlaybackSessionOutputDegradationPolicyState> {
        let this = &windows_core::Interface::cast::<IMediaPlaybackSession3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetOutputDegradationPolicyState)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for MediaPlaybackSession {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IMediaPlaybackSession>();
}
unsafe impl windows_core::Interface for MediaPlaybackSession {
    type Vtable = IMediaPlaybackSession_Vtbl;
    const IID: windows_core::GUID = <IMediaPlaybackSession as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for MediaPlaybackSession {
    const NAME: &'static str = "Windows.Media.Playback.MediaPlaybackSession";
}
unsafe impl Send for MediaPlaybackSession {}
unsafe impl Sync for MediaPlaybackSession {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct MediaPlaybackSessionBufferingStartedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(MediaPlaybackSessionBufferingStartedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl MediaPlaybackSessionBufferingStartedEventArgs {
    pub fn IsPlaybackInterruption(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsPlaybackInterruption)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for MediaPlaybackSessionBufferingStartedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IMediaPlaybackSessionBufferingStartedEventArgs>();
}
unsafe impl windows_core::Interface for MediaPlaybackSessionBufferingStartedEventArgs {
    type Vtable = IMediaPlaybackSessionBufferingStartedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IMediaPlaybackSessionBufferingStartedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for MediaPlaybackSessionBufferingStartedEventArgs {
    const NAME: &'static str = "Windows.Media.Playback.MediaPlaybackSessionBufferingStartedEventArgs";
}
unsafe impl Send for MediaPlaybackSessionBufferingStartedEventArgs {}
unsafe impl Sync for MediaPlaybackSessionBufferingStartedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct MediaPlaybackSessionOutputDegradationPolicyState(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(MediaPlaybackSessionOutputDegradationPolicyState, windows_core::IUnknown, windows_core::IInspectable);
impl MediaPlaybackSessionOutputDegradationPolicyState {
    pub fn VideoConstrictionReason(&self) -> windows_core::Result<MediaPlaybackSessionVideoConstrictionReason> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).VideoConstrictionReason)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for MediaPlaybackSessionOutputDegradationPolicyState {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IMediaPlaybackSessionOutputDegradationPolicyState>();
}
unsafe impl windows_core::Interface for MediaPlaybackSessionOutputDegradationPolicyState {
    type Vtable = IMediaPlaybackSessionOutputDegradationPolicyState_Vtbl;
    const IID: windows_core::GUID = <IMediaPlaybackSessionOutputDegradationPolicyState as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for MediaPlaybackSessionOutputDegradationPolicyState {
    const NAME: &'static str = "Windows.Media.Playback.MediaPlaybackSessionOutputDegradationPolicyState";
}
unsafe impl Send for MediaPlaybackSessionOutputDegradationPolicyState {}
unsafe impl Sync for MediaPlaybackSessionOutputDegradationPolicyState {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct MediaPlaybackSphericalVideoProjection(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(MediaPlaybackSphericalVideoProjection, windows_core::IUnknown, windows_core::IInspectable);
impl MediaPlaybackSphericalVideoProjection {
    pub fn IsEnabled(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsEnabled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsEnabled(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetIsEnabled)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "Media_MediaProperties")]
    pub fn FrameFormat(&self) -> windows_core::Result<super::MediaProperties::SphericalVideoFrameFormat> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FrameFormat)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Media_MediaProperties")]
    pub fn SetFrameFormat(&self, value: super::MediaProperties::SphericalVideoFrameFormat) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetFrameFormat)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn HorizontalFieldOfViewInDegrees(&self) -> windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HorizontalFieldOfViewInDegrees)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetHorizontalFieldOfViewInDegrees(&self, value: f64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetHorizontalFieldOfViewInDegrees)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn ViewOrientation(&self) -> windows_core::Result<super::super::Foundation::Numerics::Quaternion> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ViewOrientation)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn SetViewOrientation(&self, value: super::super::Foundation::Numerics::Quaternion) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetViewOrientation)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ProjectionMode(&self) -> windows_core::Result<SphericalVideoProjectionMode> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ProjectionMode)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetProjectionMode(&self, value: SphericalVideoProjectionMode) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetProjectionMode)(windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl windows_core::RuntimeType for MediaPlaybackSphericalVideoProjection {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IMediaPlaybackSphericalVideoProjection>();
}
unsafe impl windows_core::Interface for MediaPlaybackSphericalVideoProjection {
    type Vtable = IMediaPlaybackSphericalVideoProjection_Vtbl;
    const IID: windows_core::GUID = <IMediaPlaybackSphericalVideoProjection as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for MediaPlaybackSphericalVideoProjection {
    const NAME: &'static str = "Windows.Media.Playback.MediaPlaybackSphericalVideoProjection";
}
unsafe impl Send for MediaPlaybackSphericalVideoProjection {}
unsafe impl Sync for MediaPlaybackSphericalVideoProjection {}
#[cfg(all(feature = "Foundation_Collections", feature = "Media_Core"))]
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct MediaPlaybackTimedMetadataTrackList(windows_core::IUnknown);
#[cfg(all(feature = "Foundation_Collections", feature = "Media_Core"))]
windows_core::imp::interface_hierarchy!(MediaPlaybackTimedMetadataTrackList, windows_core::IUnknown, windows_core::IInspectable);
#[cfg(all(feature = "Foundation_Collections", feature = "Media_Core"))]
windows_core::imp::required_hierarchy!(MediaPlaybackTimedMetadataTrackList, super::super::Foundation::Collections::IIterable::<super::Core::TimedMetadataTrack>, super::super::Foundation::Collections::IVectorView::<super::Core::TimedMetadataTrack>);
#[cfg(all(feature = "Foundation_Collections", feature = "Media_Core"))]
impl MediaPlaybackTimedMetadataTrackList {
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Core"))]
    pub fn First(&self) -> windows_core::Result<super::super::Foundation::Collections::IIterator<super::Core::TimedMetadataTrack>> {
        let this = &windows_core::Interface::cast::<super::super::Foundation::Collections::IIterable<super::Core::TimedMetadataTrack>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).First)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Core"))]
    pub fn PresentationModeChanged<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<MediaPlaybackTimedMetadataTrackList, TimedMetadataPresentationModeChangedEventArgs>>,
    {
        let this = &windows_core::Interface::cast::<IMediaPlaybackTimedMetadataTrackList>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PresentationModeChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemovePresentationModeChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IMediaPlaybackTimedMetadataTrackList>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemovePresentationModeChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn GetPresentationMode(&self, index: u32) -> windows_core::Result<TimedMetadataTrackPresentationMode> {
        let this = &windows_core::Interface::cast::<IMediaPlaybackTimedMetadataTrackList>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetPresentationMode)(windows_core::Interface::as_raw(this), index, &mut result__).map(|| result__)
        }
    }
    pub fn SetPresentationMode(&self, index: u32, value: TimedMetadataTrackPresentationMode) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IMediaPlaybackTimedMetadataTrackList>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetPresentationMode)(windows_core::Interface::as_raw(this), index, value).ok() }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Core"))]
    pub fn GetAt(&self, index: u32) -> windows_core::Result<super::Core::TimedMetadataTrack> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetAt)(windows_core::Interface::as_raw(this), index, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Core"))]
    pub fn Size(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Size)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Core"))]
    pub fn IndexOf<P0>(&self, value: P0, index: &mut u32) -> windows_core::Result<bool>
    where
        P0: windows_core::Param<super::Core::TimedMetadataTrack>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IndexOf)(windows_core::Interface::as_raw(this), value.param().abi(), index, &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Core"))]
    pub fn GetMany(&self, startindex: u32, items: &mut [Option<super::Core::TimedMetadataTrack>]) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetMany)(windows_core::Interface::as_raw(this), startindex, items.len().try_into().unwrap(), core::mem::transmute_copy(&items), &mut result__).map(|| result__)
        }
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "Media_Core"))]
impl windows_core::RuntimeType for MediaPlaybackTimedMetadataTrackList {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, super::super::Foundation::Collections::IVectorView<super::Core::TimedMetadataTrack>>();
}
#[cfg(all(feature = "Foundation_Collections", feature = "Media_Core"))]
unsafe impl windows_core::Interface for MediaPlaybackTimedMetadataTrackList {
    type Vtable = super::super::Foundation::Collections::IVectorView_Vtbl<super::Core::TimedMetadataTrack>;
    const IID: windows_core::GUID = <super::super::Foundation::Collections::IVectorView<super::Core::TimedMetadataTrack> as windows_core::Interface>::IID;
}
#[cfg(all(feature = "Foundation_Collections", feature = "Media_Core"))]
impl windows_core::RuntimeName for MediaPlaybackTimedMetadataTrackList {
    const NAME: &'static str = "Windows.Media.Playback.MediaPlaybackTimedMetadataTrackList";
}
#[cfg(all(feature = "Foundation_Collections", feature = "Media_Core"))]
impl IntoIterator for MediaPlaybackTimedMetadataTrackList {
    type Item = super::Core::TimedMetadataTrack;
    type IntoIter = super::super::Foundation::Collections::VectorViewIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        IntoIterator::into_iter(&self)
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "Media_Core"))]
impl IntoIterator for &MediaPlaybackTimedMetadataTrackList {
    type Item = super::Core::TimedMetadataTrack;
    type IntoIter = super::super::Foundation::Collections::VectorViewIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        super::super::Foundation::Collections::VectorViewIterator::new(windows_core::Interface::cast(self).ok())
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "Media_Core"))]
unsafe impl Send for MediaPlaybackTimedMetadataTrackList {}
#[cfg(all(feature = "Foundation_Collections", feature = "Media_Core"))]
unsafe impl Sync for MediaPlaybackTimedMetadataTrackList {}
#[cfg(all(feature = "Foundation_Collections", feature = "Media_Core"))]
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct MediaPlaybackVideoTrackList(windows_core::IUnknown);
#[cfg(all(feature = "Foundation_Collections", feature = "Media_Core"))]
windows_core::imp::interface_hierarchy!(MediaPlaybackVideoTrackList, windows_core::IUnknown, windows_core::IInspectable);
#[cfg(all(feature = "Foundation_Collections", feature = "Media_Core"))]
windows_core::imp::required_hierarchy!(MediaPlaybackVideoTrackList, super::super::Foundation::Collections::IIterable::<super::Core::VideoTrack>, super::Core::ISingleSelectMediaTrackList, super::super::Foundation::Collections::IVectorView::<super::Core::VideoTrack>);
#[cfg(all(feature = "Foundation_Collections", feature = "Media_Core"))]
impl MediaPlaybackVideoTrackList {
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Core"))]
    pub fn First(&self) -> windows_core::Result<super::super::Foundation::Collections::IIterator<super::Core::VideoTrack>> {
        let this = &windows_core::Interface::cast::<super::super::Foundation::Collections::IIterable<super::Core::VideoTrack>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).First)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Media_Core")]
    pub fn SelectedIndexChanged<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<super::Core::ISingleSelectMediaTrackList, windows_core::IInspectable>>,
    {
        let this = &windows_core::Interface::cast::<super::Core::ISingleSelectMediaTrackList>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SelectedIndexChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Media_Core")]
    pub fn RemoveSelectedIndexChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::Core::ISingleSelectMediaTrackList>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemoveSelectedIndexChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
    #[cfg(feature = "Media_Core")]
    pub fn SetSelectedIndex(&self, value: i32) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::Core::ISingleSelectMediaTrackList>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetSelectedIndex)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "Media_Core")]
    pub fn SelectedIndex(&self) -> windows_core::Result<i32> {
        let this = &windows_core::Interface::cast::<super::Core::ISingleSelectMediaTrackList>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SelectedIndex)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Core"))]
    pub fn GetAt(&self, index: u32) -> windows_core::Result<super::Core::VideoTrack> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetAt)(windows_core::Interface::as_raw(this), index, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Core"))]
    pub fn Size(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Size)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Core"))]
    pub fn IndexOf<P0>(&self, value: P0, index: &mut u32) -> windows_core::Result<bool>
    where
        P0: windows_core::Param<super::Core::VideoTrack>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IndexOf)(windows_core::Interface::as_raw(this), value.param().abi(), index, &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Core"))]
    pub fn GetMany(&self, startindex: u32, items: &mut [Option<super::Core::VideoTrack>]) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetMany)(windows_core::Interface::as_raw(this), startindex, items.len().try_into().unwrap(), core::mem::transmute_copy(&items), &mut result__).map(|| result__)
        }
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "Media_Core"))]
impl windows_core::RuntimeType for MediaPlaybackVideoTrackList {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, super::super::Foundation::Collections::IVectorView<super::Core::VideoTrack>>();
}
#[cfg(all(feature = "Foundation_Collections", feature = "Media_Core"))]
unsafe impl windows_core::Interface for MediaPlaybackVideoTrackList {
    type Vtable = super::super::Foundation::Collections::IVectorView_Vtbl<super::Core::VideoTrack>;
    const IID: windows_core::GUID = <super::super::Foundation::Collections::IVectorView<super::Core::VideoTrack> as windows_core::Interface>::IID;
}
#[cfg(all(feature = "Foundation_Collections", feature = "Media_Core"))]
impl windows_core::RuntimeName for MediaPlaybackVideoTrackList {
    const NAME: &'static str = "Windows.Media.Playback.MediaPlaybackVideoTrackList";
}
#[cfg(all(feature = "Foundation_Collections", feature = "Media_Core"))]
impl IntoIterator for MediaPlaybackVideoTrackList {
    type Item = super::Core::VideoTrack;
    type IntoIter = super::super::Foundation::Collections::VectorViewIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        IntoIterator::into_iter(&self)
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "Media_Core"))]
impl IntoIterator for &MediaPlaybackVideoTrackList {
    type Item = super::Core::VideoTrack;
    type IntoIter = super::super::Foundation::Collections::VectorViewIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        super::super::Foundation::Collections::VectorViewIterator::new(windows_core::Interface::cast(self).ok())
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "Media_Core"))]
unsafe impl Send for MediaPlaybackVideoTrackList {}
#[cfg(all(feature = "Foundation_Collections", feature = "Media_Core"))]
unsafe impl Sync for MediaPlaybackVideoTrackList {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct MediaPlayer(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(MediaPlayer, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(MediaPlayer, super::super::Foundation::IClosable);
impl MediaPlayer {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<MediaPlayer, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn AutoPlay(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AutoPlay)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetAutoPlay(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetAutoPlay)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "deprecated")]
    pub fn NaturalDuration(&self) -> windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NaturalDuration)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn Position(&self) -> windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Position)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn SetPosition(&self, value: super::super::Foundation::TimeSpan) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetPosition)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "deprecated")]
    pub fn BufferingProgress(&self) -> windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BufferingProgress)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn CurrentState(&self) -> windows_core::Result<MediaPlayerState> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CurrentState)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn CanSeek(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CanSeek)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn CanPause(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CanPause)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsLoopingEnabled(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsLoopingEnabled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsLoopingEnabled(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetIsLoopingEnabled)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "deprecated")]
    pub fn IsProtected(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsProtected)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsMuted(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsMuted)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsMuted(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetIsMuted)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "deprecated")]
    pub fn PlaybackRate(&self) -> windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PlaybackRate)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn SetPlaybackRate(&self, value: f64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetPlaybackRate)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Volume(&self) -> windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Volume)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetVolume(&self, value: f64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetVolume)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "deprecated")]
    pub fn PlaybackMediaMarkers(&self) -> windows_core::Result<PlaybackMediaMarkerSequence> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PlaybackMediaMarkers)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn MediaOpened<P0>(&self, value: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<MediaPlayer, windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MediaOpened)(windows_core::Interface::as_raw(this), value.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveMediaOpened(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveMediaOpened)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn MediaEnded<P0>(&self, value: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<MediaPlayer, windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MediaEnded)(windows_core::Interface::as_raw(this), value.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveMediaEnded(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveMediaEnded)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn MediaFailed<P0>(&self, value: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<MediaPlayer, MediaPlayerFailedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MediaFailed)(windows_core::Interface::as_raw(this), value.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveMediaFailed(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveMediaFailed)(windows_core::Interface::as_raw(this), token).ok() }
    }
    #[cfg(feature = "deprecated")]
    pub fn CurrentStateChanged<P0>(&self, value: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<MediaPlayer, windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CurrentStateChanged)(windows_core::Interface::as_raw(this), value.param().abi(), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn RemoveCurrentStateChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveCurrentStateChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
    #[cfg(feature = "deprecated")]
    pub fn PlaybackMediaMarkerReached<P0>(&self, value: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<MediaPlayer, PlaybackMediaMarkerReachedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PlaybackMediaMarkerReached)(windows_core::Interface::as_raw(this), value.param().abi(), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn RemovePlaybackMediaMarkerReached(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemovePlaybackMediaMarkerReached)(windows_core::Interface::as_raw(this), token).ok() }
    }
    #[cfg(feature = "deprecated")]
    pub fn MediaPlayerRateChanged<P0>(&self, value: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<MediaPlayer, MediaPlayerRateChangedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MediaPlayerRateChanged)(windows_core::Interface::as_raw(this), value.param().abi(), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn RemoveMediaPlayerRateChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveMediaPlayerRateChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn VolumeChanged<P0>(&self, value: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<MediaPlayer, windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).VolumeChanged)(windows_core::Interface::as_raw(this), value.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveVolumeChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveVolumeChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
    #[cfg(feature = "deprecated")]
    pub fn SeekCompleted<P0>(&self, value: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<MediaPlayer, windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SeekCompleted)(windows_core::Interface::as_raw(this), value.param().abi(), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn RemoveSeekCompleted(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveSeekCompleted)(windows_core::Interface::as_raw(this), token).ok() }
    }
    #[cfg(feature = "deprecated")]
    pub fn BufferingStarted<P0>(&self, value: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<MediaPlayer, windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BufferingStarted)(windows_core::Interface::as_raw(this), value.param().abi(), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn RemoveBufferingStarted(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveBufferingStarted)(windows_core::Interface::as_raw(this), token).ok() }
    }
    #[cfg(feature = "deprecated")]
    pub fn BufferingEnded<P0>(&self, value: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<MediaPlayer, windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BufferingEnded)(windows_core::Interface::as_raw(this), value.param().abi(), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn RemoveBufferingEnded(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveBufferingEnded)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn Play(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Play)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Pause(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Pause)(windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "deprecated")]
    pub fn SetUriSource<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::Uri>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetUriSource)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn SystemMediaTransportControls(&self) -> windows_core::Result<super::SystemMediaTransportControls> {
        let this = &windows_core::Interface::cast::<IMediaPlayer2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SystemMediaTransportControls)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn AudioCategory(&self) -> windows_core::Result<MediaPlayerAudioCategory> {
        let this = &windows_core::Interface::cast::<IMediaPlayer2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AudioCategory)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetAudioCategory(&self, value: MediaPlayerAudioCategory) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IMediaPlayer2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetAudioCategory)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn AudioDeviceType(&self) -> windows_core::Result<MediaPlayerAudioDeviceType> {
        let this = &windows_core::Interface::cast::<IMediaPlayer2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AudioDeviceType)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetAudioDeviceType(&self, value: MediaPlayerAudioDeviceType) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IMediaPlayer2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetAudioDeviceType)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsMutedChanged<P0>(&self, value: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<MediaPlayer, windows_core::IInspectable>>,
    {
        let this = &windows_core::Interface::cast::<IMediaPlayer3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsMutedChanged)(windows_core::Interface::as_raw(this), value.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveIsMutedChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IMediaPlayer3>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemoveIsMutedChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn SourceChanged<P0>(&self, value: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<MediaPlayer, windows_core::IInspectable>>,
    {
        let this = &windows_core::Interface::cast::<IMediaPlayer3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SourceChanged)(windows_core::Interface::as_raw(this), value.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveSourceChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IMediaPlayer3>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemoveSourceChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn AudioBalance(&self) -> windows_core::Result<f64> {
        let this = &windows_core::Interface::cast::<IMediaPlayer3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AudioBalance)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetAudioBalance(&self, value: f64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IMediaPlayer3>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetAudioBalance)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn RealTimePlayback(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IMediaPlayer3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RealTimePlayback)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetRealTimePlayback(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IMediaPlayer3>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetRealTimePlayback)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn StereoscopicVideoRenderMode(&self) -> windows_core::Result<StereoscopicVideoRenderMode> {
        let this = &windows_core::Interface::cast::<IMediaPlayer3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).StereoscopicVideoRenderMode)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetStereoscopicVideoRenderMode(&self, value: StereoscopicVideoRenderMode) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IMediaPlayer3>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetStereoscopicVideoRenderMode)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn BreakManager(&self) -> windows_core::Result<MediaBreakManager> {
        let this = &windows_core::Interface::cast::<IMediaPlayer3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BreakManager)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CommandManager(&self) -> windows_core::Result<MediaPlaybackCommandManager> {
        let this = &windows_core::Interface::cast::<IMediaPlayer3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CommandManager)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Devices_Enumeration")]
    pub fn AudioDevice(&self) -> windows_core::Result<super::super::Devices::Enumeration::DeviceInformation> {
        let this = &windows_core::Interface::cast::<IMediaPlayer3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AudioDevice)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Devices_Enumeration")]
    pub fn SetAudioDevice<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Devices::Enumeration::DeviceInformation>,
    {
        let this = &windows_core::Interface::cast::<IMediaPlayer3>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetAudioDevice)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn TimelineController(&self) -> windows_core::Result<super::MediaTimelineController> {
        let this = &windows_core::Interface::cast::<IMediaPlayer3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TimelineController)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetTimelineController<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::MediaTimelineController>,
    {
        let this = &windows_core::Interface::cast::<IMediaPlayer3>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetTimelineController)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn TimelineControllerPositionOffset(&self) -> windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = &windows_core::Interface::cast::<IMediaPlayer3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TimelineControllerPositionOffset)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetTimelineControllerPositionOffset(&self, value: super::super::Foundation::TimeSpan) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IMediaPlayer3>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetTimelineControllerPositionOffset)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn PlaybackSession(&self) -> windows_core::Result<MediaPlaybackSession> {
        let this = &windows_core::Interface::cast::<IMediaPlayer3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PlaybackSession)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn StepForwardOneFrame(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IMediaPlayer3>(self)?;
        unsafe { (windows_core::Interface::vtable(this).StepForwardOneFrame)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn StepBackwardOneFrame(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IMediaPlayer3>(self)?;
        unsafe { (windows_core::Interface::vtable(this).StepBackwardOneFrame)(windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "Media_Casting")]
    pub fn GetAsCastingSource(&self) -> windows_core::Result<super::Casting::CastingSource> {
        let this = &windows_core::Interface::cast::<IMediaPlayer3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetAsCastingSource)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetSurfaceSize(&self, size: super::super::Foundation::Size) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IMediaPlayer4>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetSurfaceSize)(windows_core::Interface::as_raw(this), size).ok() }
    }
    #[cfg(feature = "UI_Composition")]
    pub fn GetSurface<P0>(&self, compositor: P0) -> windows_core::Result<MediaPlayerSurface>
    where
        P0: windows_core::Param<super::super::UI::Composition::Compositor>,
    {
        let this = &windows_core::Interface::cast::<IMediaPlayer4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetSurface)(windows_core::Interface::as_raw(this), compositor.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn VideoFrameAvailable<P0>(&self, value: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<MediaPlayer, windows_core::IInspectable>>,
    {
        let this = &windows_core::Interface::cast::<IMediaPlayer5>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).VideoFrameAvailable)(windows_core::Interface::as_raw(this), value.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveVideoFrameAvailable(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IMediaPlayer5>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemoveVideoFrameAvailable)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn IsVideoFrameServerEnabled(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IMediaPlayer5>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsVideoFrameServerEnabled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsVideoFrameServerEnabled(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IMediaPlayer5>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetIsVideoFrameServerEnabled)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "Graphics_DirectX_Direct3D11")]
    pub fn CopyFrameToVideoSurface<P0>(&self, destination: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Graphics::DirectX::Direct3D11::IDirect3DSurface>,
    {
        let this = &windows_core::Interface::cast::<IMediaPlayer5>(self)?;
        unsafe { (windows_core::Interface::vtable(this).CopyFrameToVideoSurface)(windows_core::Interface::as_raw(this), destination.param().abi()).ok() }
    }
    #[cfg(feature = "Graphics_DirectX_Direct3D11")]
    pub fn CopyFrameToVideoSurfaceWithTargetRectangle<P0>(&self, destination: P0, targetrectangle: super::super::Foundation::Rect) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Graphics::DirectX::Direct3D11::IDirect3DSurface>,
    {
        let this = &windows_core::Interface::cast::<IMediaPlayer5>(self)?;
        unsafe { (windows_core::Interface::vtable(this).CopyFrameToVideoSurfaceWithTargetRectangle)(windows_core::Interface::as_raw(this), destination.param().abi(), targetrectangle).ok() }
    }
    #[cfg(feature = "Graphics_DirectX_Direct3D11")]
    pub fn CopyFrameToStereoscopicVideoSurfaces<P0, P1>(&self, destinationlefteye: P0, destinationrighteye: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Graphics::DirectX::Direct3D11::IDirect3DSurface>,
        P1: windows_core::Param<super::super::Graphics::DirectX::Direct3D11::IDirect3DSurface>,
    {
        let this = &windows_core::Interface::cast::<IMediaPlayer5>(self)?;
        unsafe { (windows_core::Interface::vtable(this).CopyFrameToStereoscopicVideoSurfaces)(windows_core::Interface::as_raw(this), destinationlefteye.param().abi(), destinationrighteye.param().abi()).ok() }
    }
    pub fn SubtitleFrameChanged<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<MediaPlayer, windows_core::IInspectable>>,
    {
        let this = &windows_core::Interface::cast::<IMediaPlayer6>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SubtitleFrameChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveSubtitleFrameChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IMediaPlayer6>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemoveSubtitleFrameChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
    #[cfg(feature = "Graphics_DirectX_Direct3D11")]
    pub fn RenderSubtitlesToSurface<P0>(&self, destination: P0) -> windows_core::Result<bool>
    where
        P0: windows_core::Param<super::super::Graphics::DirectX::Direct3D11::IDirect3DSurface>,
    {
        let this = &windows_core::Interface::cast::<IMediaPlayer6>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RenderSubtitlesToSurface)(windows_core::Interface::as_raw(this), destination.param().abi(), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Graphics_DirectX_Direct3D11")]
    pub fn RenderSubtitlesToSurfaceWithTargetRectangle<P0>(&self, destination: P0, targetrectangle: super::super::Foundation::Rect) -> windows_core::Result<bool>
    where
        P0: windows_core::Param<super::super::Graphics::DirectX::Direct3D11::IDirect3DSurface>,
    {
        let this = &windows_core::Interface::cast::<IMediaPlayer6>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RenderSubtitlesToSurfaceWithTargetRectangle)(windows_core::Interface::as_raw(this), destination.param().abi(), targetrectangle, &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Media_Audio")]
    pub fn AudioStateMonitor(&self) -> windows_core::Result<super::Audio::AudioStateMonitor> {
        let this = &windows_core::Interface::cast::<IMediaPlayer7>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AudioStateMonitor)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn AddAudioEffect<P0>(&self, activatableclassid: &windows_core::HSTRING, effectoptional: bool, configuration: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::Collections::IPropertySet>,
    {
        let this = &windows_core::Interface::cast::<IMediaPlayerEffects>(self)?;
        unsafe { (windows_core::Interface::vtable(this).AddAudioEffect)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(activatableclassid), effectoptional, configuration.param().abi()).ok() }
    }
    pub fn RemoveAllEffects(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IMediaPlayerEffects>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemoveAllEffects)(windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn AddVideoEffect<P0>(&self, activatableclassid: &windows_core::HSTRING, effectoptional: bool, effectconfiguration: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::Collections::IPropertySet>,
    {
        let this = &windows_core::Interface::cast::<IMediaPlayerEffects2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).AddVideoEffect)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(activatableclassid), effectoptional, effectconfiguration.param().abi()).ok() }
    }
    #[cfg(feature = "Media_Protection")]
    pub fn ProtectionManager(&self) -> windows_core::Result<super::Protection::MediaProtectionManager> {
        let this = &windows_core::Interface::cast::<IMediaPlayerSource>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ProtectionManager)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Media_Protection")]
    pub fn SetProtectionManager<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Protection::MediaProtectionManager>,
    {
        let this = &windows_core::Interface::cast::<IMediaPlayerSource>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetProtectionManager)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    #[cfg(all(feature = "Storage", feature = "deprecated"))]
    pub fn SetFileSource<P0>(&self, file: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Storage::IStorageFile>,
    {
        let this = &windows_core::Interface::cast::<IMediaPlayerSource>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetFileSource)(windows_core::Interface::as_raw(this), file.param().abi()).ok() }
    }
    #[cfg(all(feature = "Storage_Streams", feature = "deprecated"))]
    pub fn SetStreamSource<P0>(&self, stream: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Storage::Streams::IRandomAccessStream>,
    {
        let this = &windows_core::Interface::cast::<IMediaPlayerSource>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetStreamSource)(windows_core::Interface::as_raw(this), stream.param().abi()).ok() }
    }
    #[cfg(all(feature = "Media_Core", feature = "deprecated"))]
    pub fn SetMediaSource<P0>(&self, source: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Core::IMediaSource>,
    {
        let this = &windows_core::Interface::cast::<IMediaPlayerSource>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetMediaSource)(windows_core::Interface::as_raw(this), source.param().abi()).ok() }
    }
    pub fn Source(&self) -> windows_core::Result<IMediaPlaybackSource> {
        let this = &windows_core::Interface::cast::<IMediaPlayerSource2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Source)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetSource<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMediaPlaybackSource>,
    {
        let this = &windows_core::Interface::cast::<IMediaPlayerSource2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetSource)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
}
impl windows_core::RuntimeType for MediaPlayer {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IMediaPlayer>();
}
unsafe impl windows_core::Interface for MediaPlayer {
    type Vtable = IMediaPlayer_Vtbl;
    const IID: windows_core::GUID = <IMediaPlayer as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for MediaPlayer {
    const NAME: &'static str = "Windows.Media.Playback.MediaPlayer";
}
unsafe impl Send for MediaPlayer {}
unsafe impl Sync for MediaPlayer {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct MediaPlayerDataReceivedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(MediaPlayerDataReceivedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl MediaPlayerDataReceivedEventArgs {
    #[cfg(feature = "Foundation_Collections")]
    pub fn Data(&self) -> windows_core::Result<super::super::Foundation::Collections::ValueSet> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Data)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for MediaPlayerDataReceivedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IMediaPlayerDataReceivedEventArgs>();
}
unsafe impl windows_core::Interface for MediaPlayerDataReceivedEventArgs {
    type Vtable = IMediaPlayerDataReceivedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IMediaPlayerDataReceivedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for MediaPlayerDataReceivedEventArgs {
    const NAME: &'static str = "Windows.Media.Playback.MediaPlayerDataReceivedEventArgs";
}
unsafe impl Send for MediaPlayerDataReceivedEventArgs {}
unsafe impl Sync for MediaPlayerDataReceivedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct MediaPlayerFailedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(MediaPlayerFailedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl MediaPlayerFailedEventArgs {
    pub fn Error(&self) -> windows_core::Result<MediaPlayerError> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Error)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ExtendedErrorCode(&self) -> windows_core::Result<windows_core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ExtendedErrorCode)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ErrorMessage(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ErrorMessage)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for MediaPlayerFailedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IMediaPlayerFailedEventArgs>();
}
unsafe impl windows_core::Interface for MediaPlayerFailedEventArgs {
    type Vtable = IMediaPlayerFailedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IMediaPlayerFailedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for MediaPlayerFailedEventArgs {
    const NAME: &'static str = "Windows.Media.Playback.MediaPlayerFailedEventArgs";
}
unsafe impl Send for MediaPlayerFailedEventArgs {}
unsafe impl Sync for MediaPlayerFailedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct MediaPlayerRateChangedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(MediaPlayerRateChangedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl MediaPlayerRateChangedEventArgs {
    pub fn NewRate(&self) -> windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NewRate)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for MediaPlayerRateChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IMediaPlayerRateChangedEventArgs>();
}
unsafe impl windows_core::Interface for MediaPlayerRateChangedEventArgs {
    type Vtable = IMediaPlayerRateChangedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IMediaPlayerRateChangedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for MediaPlayerRateChangedEventArgs {
    const NAME: &'static str = "Windows.Media.Playback.MediaPlayerRateChangedEventArgs";
}
unsafe impl Send for MediaPlayerRateChangedEventArgs {}
unsafe impl Sync for MediaPlayerRateChangedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct MediaPlayerSurface(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(MediaPlayerSurface, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(MediaPlayerSurface, super::super::Foundation::IClosable);
impl MediaPlayerSurface {
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "UI_Composition")]
    pub fn CompositionSurface(&self) -> windows_core::Result<super::super::UI::Composition::ICompositionSurface> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CompositionSurface)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "UI_Composition")]
    pub fn Compositor(&self) -> windows_core::Result<super::super::UI::Composition::Compositor> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Compositor)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn MediaPlayer(&self) -> windows_core::Result<MediaPlayer> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MediaPlayer)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for MediaPlayerSurface {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IMediaPlayerSurface>();
}
unsafe impl windows_core::Interface for MediaPlayerSurface {
    type Vtable = IMediaPlayerSurface_Vtbl;
    const IID: windows_core::GUID = <IMediaPlayerSurface as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for MediaPlayerSurface {
    const NAME: &'static str = "Windows.Media.Playback.MediaPlayerSurface";
}
unsafe impl Send for MediaPlayerSurface {}
unsafe impl Sync for MediaPlayerSurface {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct PlaybackMediaMarker(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PlaybackMediaMarker, windows_core::IUnknown, windows_core::IInspectable);
impl PlaybackMediaMarker {
    pub fn Time(&self) -> windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Time)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn MediaMarkerType(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MediaMarkerType)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Text(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Text)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CreateFromTime(value: super::super::Foundation::TimeSpan) -> windows_core::Result<PlaybackMediaMarker> {
        Self::IPlaybackMediaMarkerFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFromTime)(windows_core::Interface::as_raw(this), value, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn Create(value: super::super::Foundation::TimeSpan, mediamarkettype: &windows_core::HSTRING, text: &windows_core::HSTRING) -> windows_core::Result<PlaybackMediaMarker> {
        Self::IPlaybackMediaMarkerFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Create)(windows_core::Interface::as_raw(this), value, core::mem::transmute_copy(mediamarkettype), core::mem::transmute_copy(text), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IPlaybackMediaMarkerFactory<R, F: FnOnce(&IPlaybackMediaMarkerFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<PlaybackMediaMarker, IPlaybackMediaMarkerFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for PlaybackMediaMarker {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPlaybackMediaMarker>();
}
unsafe impl windows_core::Interface for PlaybackMediaMarker {
    type Vtable = IPlaybackMediaMarker_Vtbl;
    const IID: windows_core::GUID = <IPlaybackMediaMarker as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PlaybackMediaMarker {
    const NAME: &'static str = "Windows.Media.Playback.PlaybackMediaMarker";
}
unsafe impl Send for PlaybackMediaMarker {}
unsafe impl Sync for PlaybackMediaMarker {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct PlaybackMediaMarkerReachedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PlaybackMediaMarkerReachedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl PlaybackMediaMarkerReachedEventArgs {
    pub fn PlaybackMediaMarker(&self) -> windows_core::Result<PlaybackMediaMarker> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PlaybackMediaMarker)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for PlaybackMediaMarkerReachedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPlaybackMediaMarkerReachedEventArgs>();
}
unsafe impl windows_core::Interface for PlaybackMediaMarkerReachedEventArgs {
    type Vtable = IPlaybackMediaMarkerReachedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IPlaybackMediaMarkerReachedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PlaybackMediaMarkerReachedEventArgs {
    const NAME: &'static str = "Windows.Media.Playback.PlaybackMediaMarkerReachedEventArgs";
}
unsafe impl Send for PlaybackMediaMarkerReachedEventArgs {}
unsafe impl Sync for PlaybackMediaMarkerReachedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct PlaybackMediaMarkerSequence(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PlaybackMediaMarkerSequence, windows_core::IUnknown, windows_core::IInspectable);
#[cfg(feature = "Foundation_Collections")]
windows_core::imp::required_hierarchy!(PlaybackMediaMarkerSequence, super::super::Foundation::Collections::IIterable::<PlaybackMediaMarker>);
impl PlaybackMediaMarkerSequence {
    #[cfg(feature = "Foundation_Collections")]
    pub fn First(&self) -> windows_core::Result<super::super::Foundation::Collections::IIterator<PlaybackMediaMarker>> {
        let this = &windows_core::Interface::cast::<super::super::Foundation::Collections::IIterable<PlaybackMediaMarker>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).First)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Size(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Size)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Insert<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<PlaybackMediaMarker>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Insert)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn Clear(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Clear)(windows_core::Interface::as_raw(this)).ok() }
    }
}
impl windows_core::RuntimeType for PlaybackMediaMarkerSequence {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPlaybackMediaMarkerSequence>();
}
unsafe impl windows_core::Interface for PlaybackMediaMarkerSequence {
    type Vtable = IPlaybackMediaMarkerSequence_Vtbl;
    const IID: windows_core::GUID = <IPlaybackMediaMarkerSequence as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PlaybackMediaMarkerSequence {
    const NAME: &'static str = "Windows.Media.Playback.PlaybackMediaMarkerSequence";
}
#[cfg(feature = "Foundation_Collections")]
impl IntoIterator for PlaybackMediaMarkerSequence {
    type Item = PlaybackMediaMarker;
    type IntoIter = super::super::Foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        IntoIterator::into_iter(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl IntoIterator for &PlaybackMediaMarkerSequence {
    type Item = PlaybackMediaMarker;
    type IntoIter = super::super::Foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.First().unwrap()
    }
}
unsafe impl Send for PlaybackMediaMarkerSequence {}
unsafe impl Sync for PlaybackMediaMarkerSequence {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct TimedMetadataPresentationModeChangedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(TimedMetadataPresentationModeChangedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl TimedMetadataPresentationModeChangedEventArgs {
    #[cfg(feature = "Media_Core")]
    pub fn Track(&self) -> windows_core::Result<super::Core::TimedMetadataTrack> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Track)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn OldPresentationMode(&self) -> windows_core::Result<TimedMetadataTrackPresentationMode> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OldPresentationMode)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn NewPresentationMode(&self) -> windows_core::Result<TimedMetadataTrackPresentationMode> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NewPresentationMode)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for TimedMetadataPresentationModeChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ITimedMetadataPresentationModeChangedEventArgs>();
}
unsafe impl windows_core::Interface for TimedMetadataPresentationModeChangedEventArgs {
    type Vtable = ITimedMetadataPresentationModeChangedEventArgs_Vtbl;
    const IID: windows_core::GUID = <ITimedMetadataPresentationModeChangedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for TimedMetadataPresentationModeChangedEventArgs {
    const NAME: &'static str = "Windows.Media.Playback.TimedMetadataPresentationModeChangedEventArgs";
}
unsafe impl Send for TimedMetadataPresentationModeChangedEventArgs {}
unsafe impl Sync for TimedMetadataPresentationModeChangedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct AutoLoadedDisplayPropertyKind(pub i32);
impl AutoLoadedDisplayPropertyKind {
    pub const None: Self = Self(0i32);
    pub const MusicOrVideo: Self = Self(1i32);
    pub const Music: Self = Self(2i32);
    pub const Video: Self = Self(3i32);
}
impl windows_core::TypeKind for AutoLoadedDisplayPropertyKind {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for AutoLoadedDisplayPropertyKind {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("AutoLoadedDisplayPropertyKind").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for AutoLoadedDisplayPropertyKind {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Playback.AutoLoadedDisplayPropertyKind;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct FailedMediaStreamKind(pub i32);
impl FailedMediaStreamKind {
    pub const Unknown: Self = Self(0i32);
    pub const Audio: Self = Self(1i32);
    pub const Video: Self = Self(2i32);
}
impl windows_core::TypeKind for FailedMediaStreamKind {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for FailedMediaStreamKind {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("FailedMediaStreamKind").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for FailedMediaStreamKind {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Playback.FailedMediaStreamKind;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct MediaBreakInsertionMethod(pub i32);
impl MediaBreakInsertionMethod {
    pub const Interrupt: Self = Self(0i32);
    pub const Replace: Self = Self(1i32);
}
impl windows_core::TypeKind for MediaBreakInsertionMethod {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for MediaBreakInsertionMethod {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MediaBreakInsertionMethod").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for MediaBreakInsertionMethod {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Playback.MediaBreakInsertionMethod;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct MediaCommandEnablingRule(pub i32);
impl MediaCommandEnablingRule {
    pub const Auto: Self = Self(0i32);
    pub const Always: Self = Self(1i32);
    pub const Never: Self = Self(2i32);
}
impl windows_core::TypeKind for MediaCommandEnablingRule {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for MediaCommandEnablingRule {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MediaCommandEnablingRule").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for MediaCommandEnablingRule {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Playback.MediaCommandEnablingRule;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct MediaPlaybackItemChangedReason(pub i32);
impl MediaPlaybackItemChangedReason {
    pub const InitialItem: Self = Self(0i32);
    pub const EndOfStream: Self = Self(1i32);
    pub const Error: Self = Self(2i32);
    pub const AppRequested: Self = Self(3i32);
}
impl windows_core::TypeKind for MediaPlaybackItemChangedReason {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for MediaPlaybackItemChangedReason {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MediaPlaybackItemChangedReason").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for MediaPlaybackItemChangedReason {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Playback.MediaPlaybackItemChangedReason;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct MediaPlaybackItemErrorCode(pub i32);
impl MediaPlaybackItemErrorCode {
    pub const None: Self = Self(0i32);
    pub const Aborted: Self = Self(1i32);
    pub const NetworkError: Self = Self(2i32);
    pub const DecodeError: Self = Self(3i32);
    pub const SourceNotSupportedError: Self = Self(4i32);
    pub const EncryptionError: Self = Self(5i32);
}
impl windows_core::TypeKind for MediaPlaybackItemErrorCode {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for MediaPlaybackItemErrorCode {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MediaPlaybackItemErrorCode").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for MediaPlaybackItemErrorCode {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Playback.MediaPlaybackItemErrorCode;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct MediaPlaybackSessionVideoConstrictionReason(pub i32);
impl MediaPlaybackSessionVideoConstrictionReason {
    pub const None: Self = Self(0i32);
    pub const VirtualMachine: Self = Self(1i32);
    pub const UnsupportedDisplayAdapter: Self = Self(2i32);
    pub const UnsignedDriver: Self = Self(3i32);
    pub const FrameServerEnabled: Self = Self(4i32);
    pub const OutputProtectionFailed: Self = Self(5i32);
    pub const Unknown: Self = Self(6i32);
}
impl windows_core::TypeKind for MediaPlaybackSessionVideoConstrictionReason {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for MediaPlaybackSessionVideoConstrictionReason {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MediaPlaybackSessionVideoConstrictionReason").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for MediaPlaybackSessionVideoConstrictionReason {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Playback.MediaPlaybackSessionVideoConstrictionReason;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct MediaPlaybackState(pub i32);
impl MediaPlaybackState {
    pub const None: Self = Self(0i32);
    pub const Opening: Self = Self(1i32);
    pub const Buffering: Self = Self(2i32);
    pub const Playing: Self = Self(3i32);
    pub const Paused: Self = Self(4i32);
}
impl windows_core::TypeKind for MediaPlaybackState {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for MediaPlaybackState {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MediaPlaybackState").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for MediaPlaybackState {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Playback.MediaPlaybackState;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct MediaPlayerAudioCategory(pub i32);
impl MediaPlayerAudioCategory {
    pub const Other: Self = Self(0i32);
    pub const Communications: Self = Self(3i32);
    pub const Alerts: Self = Self(4i32);
    pub const SoundEffects: Self = Self(5i32);
    pub const GameEffects: Self = Self(6i32);
    pub const GameMedia: Self = Self(7i32);
    pub const GameChat: Self = Self(8i32);
    pub const Speech: Self = Self(9i32);
    pub const Movie: Self = Self(10i32);
    pub const Media: Self = Self(11i32);
}
impl windows_core::TypeKind for MediaPlayerAudioCategory {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for MediaPlayerAudioCategory {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MediaPlayerAudioCategory").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for MediaPlayerAudioCategory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Playback.MediaPlayerAudioCategory;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct MediaPlayerAudioDeviceType(pub i32);
impl MediaPlayerAudioDeviceType {
    pub const Console: Self = Self(0i32);
    pub const Multimedia: Self = Self(1i32);
    pub const Communications: Self = Self(2i32);
}
impl windows_core::TypeKind for MediaPlayerAudioDeviceType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for MediaPlayerAudioDeviceType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MediaPlayerAudioDeviceType").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for MediaPlayerAudioDeviceType {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Playback.MediaPlayerAudioDeviceType;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct MediaPlayerError(pub i32);
impl MediaPlayerError {
    pub const Unknown: Self = Self(0i32);
    pub const Aborted: Self = Self(1i32);
    pub const NetworkError: Self = Self(2i32);
    pub const DecodingError: Self = Self(3i32);
    pub const SourceNotSupported: Self = Self(4i32);
}
impl windows_core::TypeKind for MediaPlayerError {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for MediaPlayerError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MediaPlayerError").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for MediaPlayerError {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Playback.MediaPlayerError;i4)");
}
#[cfg(feature = "deprecated")]
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct MediaPlayerState(pub i32);
#[cfg(feature = "deprecated")]
impl MediaPlayerState {
    pub const Closed: Self = Self(0i32);
    pub const Opening: Self = Self(1i32);
    pub const Buffering: Self = Self(2i32);
    pub const Playing: Self = Self(3i32);
    pub const Paused: Self = Self(4i32);
    pub const Stopped: Self = Self(5i32);
}
#[cfg(feature = "deprecated")]
impl windows_core::TypeKind for MediaPlayerState {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "deprecated")]
impl core::fmt::Debug for MediaPlayerState {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MediaPlayerState").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
impl windows_core::RuntimeType for MediaPlayerState {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Playback.MediaPlayerState;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SphericalVideoProjectionMode(pub i32);
impl SphericalVideoProjectionMode {
    pub const Spherical: Self = Self(0i32);
    pub const Flat: Self = Self(1i32);
}
impl windows_core::TypeKind for SphericalVideoProjectionMode {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SphericalVideoProjectionMode {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SphericalVideoProjectionMode").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for SphericalVideoProjectionMode {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Playback.SphericalVideoProjectionMode;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct StereoscopicVideoRenderMode(pub i32);
impl StereoscopicVideoRenderMode {
    pub const Mono: Self = Self(0i32);
    pub const Stereo: Self = Self(1i32);
}
impl windows_core::TypeKind for StereoscopicVideoRenderMode {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for StereoscopicVideoRenderMode {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("StereoscopicVideoRenderMode").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for StereoscopicVideoRenderMode {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Playback.StereoscopicVideoRenderMode;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct TimedMetadataTrackPresentationMode(pub i32);
impl TimedMetadataTrackPresentationMode {
    pub const Disabled: Self = Self(0i32);
    pub const Hidden: Self = Self(1i32);
    pub const ApplicationPresented: Self = Self(2i32);
    pub const PlatformPresented: Self = Self(3i32);
}
impl windows_core::TypeKind for TimedMetadataTrackPresentationMode {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for TimedMetadataTrackPresentationMode {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("TimedMetadataTrackPresentationMode").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for TimedMetadataTrackPresentationMode {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Playback.TimedMetadataTrackPresentationMode;i4)");
}
#[cfg(feature = "implement")]
core::include!("impl.rs");
