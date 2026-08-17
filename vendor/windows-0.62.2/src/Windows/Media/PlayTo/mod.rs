#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CurrentTimeChangeRequestedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(CurrentTimeChangeRequestedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl CurrentTimeChangeRequestedEventArgs {
    pub fn Time(&self) -> windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Time)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for CurrentTimeChangeRequestedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ICurrentTimeChangeRequestedEventArgs>();
}
unsafe impl windows_core::Interface for CurrentTimeChangeRequestedEventArgs {
    type Vtable = <ICurrentTimeChangeRequestedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ICurrentTimeChangeRequestedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for CurrentTimeChangeRequestedEventArgs {
    const NAME: &'static str = "Windows.Media.PlayTo.CurrentTimeChangeRequestedEventArgs";
}
windows_core::imp::define_interface!(ICurrentTimeChangeRequestedEventArgs, ICurrentTimeChangeRequestedEventArgs_Vtbl, 0x99711324_edc7_4bf5_91f6_3c8627db59e5);
impl windows_core::RuntimeType for ICurrentTimeChangeRequestedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ICurrentTimeChangeRequestedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Time: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::TimeSpan) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMuteChangeRequestedEventArgs, IMuteChangeRequestedEventArgs_Vtbl, 0xe4b4f5f6_af1f_4f1e_b437_7da32400e1d4);
impl windows_core::RuntimeType for IMuteChangeRequestedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IMuteChangeRequestedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Mute: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPlayToConnection, IPlayToConnection_Vtbl, 0x112fbfc8_f235_4fde_8d41_9bf27c9e9a40);
impl windows_core::RuntimeType for IPlayToConnection {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlayToConnection_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub State: unsafe extern "system" fn(*mut core::ffi::c_void, *mut PlayToConnectionState) -> windows_core::HRESULT,
    pub StateChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveStateChanged: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub Transferred: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveTransferred: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub Error: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveError: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPlayToConnectionErrorEventArgs, IPlayToConnectionErrorEventArgs_Vtbl, 0xbf5eada6_88e6_445f_9d40_d9b9f8939896);
impl windows_core::RuntimeType for IPlayToConnectionErrorEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlayToConnectionErrorEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Code: unsafe extern "system" fn(*mut core::ffi::c_void, *mut PlayToConnectionError) -> windows_core::HRESULT,
    pub Message: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPlayToConnectionStateChangedEventArgs, IPlayToConnectionStateChangedEventArgs_Vtbl, 0x68c4b50f_0c20_4980_8602_58c62238d423);
impl windows_core::RuntimeType for IPlayToConnectionStateChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlayToConnectionStateChangedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub PreviousState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut PlayToConnectionState) -> windows_core::HRESULT,
    pub CurrentState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut PlayToConnectionState) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPlayToConnectionTransferredEventArgs, IPlayToConnectionTransferredEventArgs_Vtbl, 0xfae3193a_0683_47d9_8df0_18cbb48984d8);
impl windows_core::RuntimeType for IPlayToConnectionTransferredEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlayToConnectionTransferredEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub PreviousSource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CurrentSource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPlayToManager, IPlayToManager_Vtbl, 0xf56a206e_1b77_42ef_8f0d_b949f8d9b260);
impl windows_core::RuntimeType for IPlayToManager {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlayToManager_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SourceRequested: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveSourceRequested: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub SourceSelected: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveSourceSelected: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub SetDefaultSourceSelection: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub DefaultSourceSelection: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPlayToManagerStatics, IPlayToManagerStatics_Vtbl, 0x64e6a887_3982_4f3b_ba20_6155e435325b);
impl windows_core::RuntimeType for IPlayToManagerStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlayToManagerStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetForCurrentView: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ShowPlayToUI: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPlayToReceiver, IPlayToReceiver_Vtbl, 0xac15cf47_a162_4aa6_af1b_3aa35f3b9069);
impl windows_core::RuntimeType for IPlayToReceiver {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlayToReceiver_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub PlayRequested: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemovePlayRequested: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub PauseRequested: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemovePauseRequested: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub SourceChangeRequested: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveSourceChangeRequested: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub PlaybackRateChangeRequested: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemovePlaybackRateChangeRequested: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub CurrentTimeChangeRequested: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveCurrentTimeChangeRequested: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub MuteChangeRequested: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveMuteChangeRequested: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub VolumeChangeRequested: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveVolumeChangeRequested: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub TimeUpdateRequested: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveTimeUpdateRequested: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub StopRequested: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveStopRequested: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub NotifyVolumeChange: unsafe extern "system" fn(*mut core::ffi::c_void, f64, bool) -> windows_core::HRESULT,
    pub NotifyRateChange: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub NotifyLoadedMetadata: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub NotifyTimeUpdate: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::TimeSpan) -> windows_core::HRESULT,
    pub NotifyDurationChange: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::TimeSpan) -> windows_core::HRESULT,
    pub NotifySeeking: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub NotifySeeked: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub NotifyPaused: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub NotifyPlaying: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub NotifyEnded: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub NotifyError: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub NotifyStopped: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FriendlyName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetFriendlyName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetSupportsImage: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub SupportsImage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetSupportsAudio: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub SupportsAudio: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetSupportsVideo: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub SupportsVideo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Properties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Properties: usize,
    pub StartAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub StopAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPlayToSource, IPlayToSource_Vtbl, 0x7f138a08_fbb7_4b09_8356_aa5f4e335c31);
impl windows_core::RuntimeType for IPlayToSource {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlayToSource_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Connection: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetNext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PlayNext: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPlayToSourceDeferral, IPlayToSourceDeferral_Vtbl, 0x4100891d_278e_4f29_859b_a9e501053e7d);
impl windows_core::RuntimeType for IPlayToSourceDeferral {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlayToSourceDeferral_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Complete: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPlayToSourceRequest, IPlayToSourceRequest_Vtbl, 0xf8584665_64f4_44a0_ac0d_468d2b8fda83);
impl windows_core::RuntimeType for IPlayToSourceRequest {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlayToSourceRequest_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Deadline: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::DateTime) -> windows_core::HRESULT,
    pub DisplayErrorString: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDeferral: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetSource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPlayToSourceRequestedEventArgs, IPlayToSourceRequestedEventArgs_Vtbl, 0xc5cdc330_29df_4ec6_9da9_9fbdfcfc1b3e);
impl windows_core::RuntimeType for IPlayToSourceRequestedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlayToSourceRequestedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SourceRequest: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPlayToSourceSelectedEventArgs, IPlayToSourceSelectedEventArgs_Vtbl, 0x0c9d8511_5202_4dcb_8c67_abda12bb3c12);
impl windows_core::RuntimeType for IPlayToSourceSelectedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlayToSourceSelectedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub FriendlyName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub Icon: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Icon: usize,
    pub SupportsImage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SupportsAudio: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SupportsVideo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPlayToSourceWithPreferredSourceUri, IPlayToSourceWithPreferredSourceUri_Vtbl, 0xaab253eb_3301_4dc4_afba_b2f2ed9635a0);
impl windows_core::RuntimeType for IPlayToSourceWithPreferredSourceUri {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlayToSourceWithPreferredSourceUri_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub PreferredSourceUri: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetPreferredSourceUri: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPlaybackRateChangeRequestedEventArgs, IPlaybackRateChangeRequestedEventArgs_Vtbl, 0x0f5661ae_2c88_4cca_8540_d586095d13a5);
impl windows_core::RuntimeType for IPlaybackRateChangeRequestedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlaybackRateChangeRequestedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Rate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISourceChangeRequestedEventArgs, ISourceChangeRequestedEventArgs_Vtbl, 0xfb3f3a96_7aa6_4a8b_86e7_54f6c6d34f64);
impl windows_core::RuntimeType for ISourceChangeRequestedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ISourceChangeRequestedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Storage_Streams")]
    pub Stream: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Stream: usize,
    pub Title: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Author: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Album: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Genre: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Description: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Date: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub Thumbnail: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Thumbnail: usize,
    pub Rating: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Properties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IVolumeChangeRequestedEventArgs, IVolumeChangeRequestedEventArgs_Vtbl, 0x6f026d5c_cf75_4c2b_913e_6d7c6c329179);
impl windows_core::RuntimeType for IVolumeChangeRequestedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IVolumeChangeRequestedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Volume: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MuteChangeRequestedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(MuteChangeRequestedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl MuteChangeRequestedEventArgs {
    pub fn Mute(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Mute)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for MuteChangeRequestedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IMuteChangeRequestedEventArgs>();
}
unsafe impl windows_core::Interface for MuteChangeRequestedEventArgs {
    type Vtable = <IMuteChangeRequestedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IMuteChangeRequestedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for MuteChangeRequestedEventArgs {
    const NAME: &'static str = "Windows.Media.PlayTo.MuteChangeRequestedEventArgs";
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlayToConnection(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PlayToConnection, windows_core::IUnknown, windows_core::IInspectable);
impl PlayToConnection {
    pub fn State(&self) -> windows_core::Result<PlayToConnectionState> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).State)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn StateChanged<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<PlayToConnection, PlayToConnectionStateChangedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).StateChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveStateChanged(&self, token: i64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveStateChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn Transferred<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<PlayToConnection, PlayToConnectionTransferredEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Transferred)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveTransferred(&self, token: i64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveTransferred)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn Error<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<PlayToConnection, PlayToConnectionErrorEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Error)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveError(&self, token: i64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveError)(windows_core::Interface::as_raw(this), token).ok() }
    }
}
impl windows_core::RuntimeType for PlayToConnection {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPlayToConnection>();
}
unsafe impl windows_core::Interface for PlayToConnection {
    type Vtable = <IPlayToConnection as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IPlayToConnection as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PlayToConnection {
    const NAME: &'static str = "Windows.Media.PlayTo.PlayToConnection";
}
unsafe impl Send for PlayToConnection {}
unsafe impl Sync for PlayToConnection {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PlayToConnectionError(pub i32);
impl PlayToConnectionError {
    pub const None: Self = Self(0i32);
    pub const DeviceNotResponding: Self = Self(1i32);
    pub const DeviceError: Self = Self(2i32);
    pub const DeviceLocked: Self = Self(3i32);
    pub const ProtectedPlaybackFailed: Self = Self(4i32);
}
impl windows_core::TypeKind for PlayToConnectionError {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for PlayToConnectionError {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.PlayTo.PlayToConnectionError;i4)");
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlayToConnectionErrorEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PlayToConnectionErrorEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl PlayToConnectionErrorEventArgs {
    pub fn Code(&self) -> windows_core::Result<PlayToConnectionError> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Code)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Message(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Message)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
impl windows_core::RuntimeType for PlayToConnectionErrorEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPlayToConnectionErrorEventArgs>();
}
unsafe impl windows_core::Interface for PlayToConnectionErrorEventArgs {
    type Vtable = <IPlayToConnectionErrorEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IPlayToConnectionErrorEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PlayToConnectionErrorEventArgs {
    const NAME: &'static str = "Windows.Media.PlayTo.PlayToConnectionErrorEventArgs";
}
unsafe impl Send for PlayToConnectionErrorEventArgs {}
unsafe impl Sync for PlayToConnectionErrorEventArgs {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PlayToConnectionState(pub i32);
impl PlayToConnectionState {
    pub const Disconnected: Self = Self(0i32);
    pub const Connected: Self = Self(1i32);
    pub const Rendering: Self = Self(2i32);
}
impl windows_core::TypeKind for PlayToConnectionState {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for PlayToConnectionState {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.PlayTo.PlayToConnectionState;i4)");
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlayToConnectionStateChangedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PlayToConnectionStateChangedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl PlayToConnectionStateChangedEventArgs {
    pub fn PreviousState(&self) -> windows_core::Result<PlayToConnectionState> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PreviousState)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn CurrentState(&self) -> windows_core::Result<PlayToConnectionState> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CurrentState)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for PlayToConnectionStateChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPlayToConnectionStateChangedEventArgs>();
}
unsafe impl windows_core::Interface for PlayToConnectionStateChangedEventArgs {
    type Vtable = <IPlayToConnectionStateChangedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IPlayToConnectionStateChangedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PlayToConnectionStateChangedEventArgs {
    const NAME: &'static str = "Windows.Media.PlayTo.PlayToConnectionStateChangedEventArgs";
}
unsafe impl Send for PlayToConnectionStateChangedEventArgs {}
unsafe impl Sync for PlayToConnectionStateChangedEventArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlayToConnectionTransferredEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PlayToConnectionTransferredEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl PlayToConnectionTransferredEventArgs {
    pub fn PreviousSource(&self) -> windows_core::Result<PlayToSource> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PreviousSource)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CurrentSource(&self) -> windows_core::Result<PlayToSource> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CurrentSource)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for PlayToConnectionTransferredEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPlayToConnectionTransferredEventArgs>();
}
unsafe impl windows_core::Interface for PlayToConnectionTransferredEventArgs {
    type Vtable = <IPlayToConnectionTransferredEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IPlayToConnectionTransferredEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PlayToConnectionTransferredEventArgs {
    const NAME: &'static str = "Windows.Media.PlayTo.PlayToConnectionTransferredEventArgs";
}
unsafe impl Send for PlayToConnectionTransferredEventArgs {}
unsafe impl Sync for PlayToConnectionTransferredEventArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlayToManager(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PlayToManager, windows_core::IUnknown, windows_core::IInspectable);
impl PlayToManager {
    pub fn SourceRequested<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<PlayToManager, PlayToSourceRequestedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SourceRequested)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveSourceRequested(&self, token: i64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveSourceRequested)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn SourceSelected<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<PlayToManager, PlayToSourceSelectedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SourceSelected)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveSourceSelected(&self, token: i64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveSourceSelected)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn SetDefaultSourceSelection(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetDefaultSourceSelection)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn DefaultSourceSelection(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DefaultSourceSelection)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn GetForCurrentView() -> windows_core::Result<PlayToManager> {
        Self::IPlayToManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetForCurrentView)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn ShowPlayToUI() -> windows_core::Result<()> {
        Self::IPlayToManagerStatics(|this| unsafe { (windows_core::Interface::vtable(this).ShowPlayToUI)(windows_core::Interface::as_raw(this)).ok() })
    }
    fn IPlayToManagerStatics<R, F: FnOnce(&IPlayToManagerStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<PlayToManager, IPlayToManagerStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for PlayToManager {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPlayToManager>();
}
unsafe impl windows_core::Interface for PlayToManager {
    type Vtable = <IPlayToManager as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IPlayToManager as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PlayToManager {
    const NAME: &'static str = "Windows.Media.PlayTo.PlayToManager";
}
unsafe impl Send for PlayToManager {}
unsafe impl Sync for PlayToManager {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlayToReceiver(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PlayToReceiver, windows_core::IUnknown, windows_core::IInspectable);
impl PlayToReceiver {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<PlayToReceiver, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn PlayRequested<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<PlayToReceiver, windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PlayRequested)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemovePlayRequested(&self, token: i64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemovePlayRequested)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn PauseRequested<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<PlayToReceiver, windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PauseRequested)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemovePauseRequested(&self, token: i64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemovePauseRequested)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn SourceChangeRequested<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<PlayToReceiver, SourceChangeRequestedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SourceChangeRequested)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveSourceChangeRequested(&self, token: i64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveSourceChangeRequested)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn PlaybackRateChangeRequested<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<PlayToReceiver, PlaybackRateChangeRequestedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PlaybackRateChangeRequested)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemovePlaybackRateChangeRequested(&self, token: i64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemovePlaybackRateChangeRequested)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn CurrentTimeChangeRequested<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<PlayToReceiver, CurrentTimeChangeRequestedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CurrentTimeChangeRequested)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveCurrentTimeChangeRequested(&self, token: i64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveCurrentTimeChangeRequested)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn MuteChangeRequested<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<PlayToReceiver, MuteChangeRequestedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MuteChangeRequested)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveMuteChangeRequested(&self, token: i64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveMuteChangeRequested)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn VolumeChangeRequested<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<PlayToReceiver, VolumeChangeRequestedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).VolumeChangeRequested)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveVolumeChangeRequested(&self, token: i64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveVolumeChangeRequested)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn TimeUpdateRequested<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<PlayToReceiver, windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TimeUpdateRequested)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveTimeUpdateRequested(&self, token: i64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveTimeUpdateRequested)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn StopRequested<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<PlayToReceiver, windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).StopRequested)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveStopRequested(&self, token: i64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveStopRequested)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn NotifyVolumeChange(&self, volume: f64, mute: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).NotifyVolumeChange)(windows_core::Interface::as_raw(this), volume, mute).ok() }
    }
    pub fn NotifyRateChange(&self, rate: f64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).NotifyRateChange)(windows_core::Interface::as_raw(this), rate).ok() }
    }
    pub fn NotifyLoadedMetadata(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).NotifyLoadedMetadata)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn NotifyTimeUpdate(&self, currenttime: super::super::Foundation::TimeSpan) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).NotifyTimeUpdate)(windows_core::Interface::as_raw(this), currenttime).ok() }
    }
    pub fn NotifyDurationChange(&self, duration: super::super::Foundation::TimeSpan) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).NotifyDurationChange)(windows_core::Interface::as_raw(this), duration).ok() }
    }
    pub fn NotifySeeking(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).NotifySeeking)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn NotifySeeked(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).NotifySeeked)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn NotifyPaused(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).NotifyPaused)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn NotifyPlaying(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).NotifyPlaying)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn NotifyEnded(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).NotifyEnded)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn NotifyError(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).NotifyError)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn NotifyStopped(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).NotifyStopped)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn FriendlyName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FriendlyName)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetFriendlyName(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetFriendlyName)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn SetSupportsImage(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetSupportsImage)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn SupportsImage(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SupportsImage)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetSupportsAudio(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetSupportsAudio)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn SupportsAudio(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SupportsAudio)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetSupportsVideo(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetSupportsVideo)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn SupportsVideo(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SupportsVideo)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Properties(&self) -> windows_core::Result<super::super::Foundation::Collections::IPropertySet> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Properties)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn StartAsync(&self) -> windows_core::Result<windows_future::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).StartAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn StopAsync(&self) -> windows_core::Result<windows_future::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).StopAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for PlayToReceiver {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPlayToReceiver>();
}
unsafe impl windows_core::Interface for PlayToReceiver {
    type Vtable = <IPlayToReceiver as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IPlayToReceiver as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PlayToReceiver {
    const NAME: &'static str = "Windows.Media.PlayTo.PlayToReceiver";
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlayToSource(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PlayToSource, windows_core::IUnknown, windows_core::IInspectable);
impl PlayToSource {
    pub fn Connection(&self) -> windows_core::Result<PlayToConnection> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Connection)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Next(&self) -> windows_core::Result<PlayToSource> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Next)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetNext<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<PlayToSource>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetNext)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn PlayNext(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).PlayNext)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn PreferredSourceUri(&self) -> windows_core::Result<super::super::Foundation::Uri> {
        let this = &windows_core::Interface::cast::<IPlayToSourceWithPreferredSourceUri>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PreferredSourceUri)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetPreferredSourceUri<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::Uri>,
    {
        let this = &windows_core::Interface::cast::<IPlayToSourceWithPreferredSourceUri>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetPreferredSourceUri)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
}
impl windows_core::RuntimeType for PlayToSource {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPlayToSource>();
}
unsafe impl windows_core::Interface for PlayToSource {
    type Vtable = <IPlayToSource as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IPlayToSource as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PlayToSource {
    const NAME: &'static str = "Windows.Media.PlayTo.PlayToSource";
}
unsafe impl Send for PlayToSource {}
unsafe impl Sync for PlayToSource {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlayToSourceDeferral(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PlayToSourceDeferral, windows_core::IUnknown, windows_core::IInspectable);
impl PlayToSourceDeferral {
    pub fn Complete(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Complete)(windows_core::Interface::as_raw(this)).ok() }
    }
}
impl windows_core::RuntimeType for PlayToSourceDeferral {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPlayToSourceDeferral>();
}
unsafe impl windows_core::Interface for PlayToSourceDeferral {
    type Vtable = <IPlayToSourceDeferral as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IPlayToSourceDeferral as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PlayToSourceDeferral {
    const NAME: &'static str = "Windows.Media.PlayTo.PlayToSourceDeferral";
}
unsafe impl Send for PlayToSourceDeferral {}
unsafe impl Sync for PlayToSourceDeferral {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlayToSourceRequest(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PlayToSourceRequest, windows_core::IUnknown, windows_core::IInspectable);
impl PlayToSourceRequest {
    pub fn Deadline(&self) -> windows_core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Deadline)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn DisplayErrorString(&self, errorstring: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).DisplayErrorString)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(errorstring)).ok() }
    }
    pub fn GetDeferral(&self) -> windows_core::Result<PlayToSourceDeferral> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDeferral)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetSource<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<PlayToSource>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetSource)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
}
impl windows_core::RuntimeType for PlayToSourceRequest {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPlayToSourceRequest>();
}
unsafe impl windows_core::Interface for PlayToSourceRequest {
    type Vtable = <IPlayToSourceRequest as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IPlayToSourceRequest as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PlayToSourceRequest {
    const NAME: &'static str = "Windows.Media.PlayTo.PlayToSourceRequest";
}
unsafe impl Send for PlayToSourceRequest {}
unsafe impl Sync for PlayToSourceRequest {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlayToSourceRequestedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PlayToSourceRequestedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl PlayToSourceRequestedEventArgs {
    pub fn SourceRequest(&self) -> windows_core::Result<PlayToSourceRequest> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SourceRequest)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for PlayToSourceRequestedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPlayToSourceRequestedEventArgs>();
}
unsafe impl windows_core::Interface for PlayToSourceRequestedEventArgs {
    type Vtable = <IPlayToSourceRequestedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IPlayToSourceRequestedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PlayToSourceRequestedEventArgs {
    const NAME: &'static str = "Windows.Media.PlayTo.PlayToSourceRequestedEventArgs";
}
unsafe impl Send for PlayToSourceRequestedEventArgs {}
unsafe impl Sync for PlayToSourceRequestedEventArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlayToSourceSelectedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PlayToSourceSelectedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl PlayToSourceSelectedEventArgs {
    pub fn FriendlyName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FriendlyName)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn Icon(&self) -> windows_core::Result<super::super::Storage::Streams::IRandomAccessStreamWithContentType> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Icon)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SupportsImage(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SupportsImage)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SupportsAudio(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SupportsAudio)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SupportsVideo(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SupportsVideo)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for PlayToSourceSelectedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPlayToSourceSelectedEventArgs>();
}
unsafe impl windows_core::Interface for PlayToSourceSelectedEventArgs {
    type Vtable = <IPlayToSourceSelectedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IPlayToSourceSelectedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PlayToSourceSelectedEventArgs {
    const NAME: &'static str = "Windows.Media.PlayTo.PlayToSourceSelectedEventArgs";
}
unsafe impl Send for PlayToSourceSelectedEventArgs {}
unsafe impl Sync for PlayToSourceSelectedEventArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlaybackRateChangeRequestedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PlaybackRateChangeRequestedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl PlaybackRateChangeRequestedEventArgs {
    pub fn Rate(&self) -> windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Rate)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for PlaybackRateChangeRequestedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPlaybackRateChangeRequestedEventArgs>();
}
unsafe impl windows_core::Interface for PlaybackRateChangeRequestedEventArgs {
    type Vtable = <IPlaybackRateChangeRequestedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IPlaybackRateChangeRequestedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PlaybackRateChangeRequestedEventArgs {
    const NAME: &'static str = "Windows.Media.PlayTo.PlaybackRateChangeRequestedEventArgs";
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SourceChangeRequestedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(SourceChangeRequestedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl SourceChangeRequestedEventArgs {
    #[cfg(feature = "Storage_Streams")]
    pub fn Stream(&self) -> windows_core::Result<super::super::Storage::Streams::IRandomAccessStreamWithContentType> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Stream)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Title(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Title)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn Author(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Author)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn Album(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Album)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn Genre(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Genre)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn Description(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Description)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn Date(&self) -> windows_core::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Date)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn Thumbnail(&self) -> windows_core::Result<super::super::Storage::Streams::IRandomAccessStreamReference> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Thumbnail)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Rating(&self) -> windows_core::Result<super::super::Foundation::IReference<u32>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Rating)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Properties(&self) -> windows_core::Result<windows_collections::IMapView<windows_core::HSTRING, windows_core::IInspectable>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Properties)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for SourceChangeRequestedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ISourceChangeRequestedEventArgs>();
}
unsafe impl windows_core::Interface for SourceChangeRequestedEventArgs {
    type Vtable = <ISourceChangeRequestedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ISourceChangeRequestedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for SourceChangeRequestedEventArgs {
    const NAME: &'static str = "Windows.Media.PlayTo.SourceChangeRequestedEventArgs";
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VolumeChangeRequestedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(VolumeChangeRequestedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl VolumeChangeRequestedEventArgs {
    pub fn Volume(&self) -> windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Volume)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for VolumeChangeRequestedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IVolumeChangeRequestedEventArgs>();
}
unsafe impl windows_core::Interface for VolumeChangeRequestedEventArgs {
    type Vtable = <IVolumeChangeRequestedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IVolumeChangeRequestedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for VolumeChangeRequestedEventArgs {
    const NAME: &'static str = "Windows.Media.PlayTo.VolumeChangeRequestedEventArgs";
}
