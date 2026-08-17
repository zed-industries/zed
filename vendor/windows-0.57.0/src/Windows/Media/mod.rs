#[cfg(feature = "Media_AppBroadcasting")]
pub mod AppBroadcasting;
#[cfg(feature = "Media_AppRecording")]
pub mod AppRecording;
#[cfg(feature = "Media_Audio")]
pub mod Audio;
#[cfg(feature = "Media_Capture")]
pub mod Capture;
#[cfg(feature = "Media_Casting")]
pub mod Casting;
#[cfg(feature = "Media_ClosedCaptioning")]
pub mod ClosedCaptioning;
#[cfg(feature = "Media_ContentRestrictions")]
pub mod ContentRestrictions;
#[cfg(feature = "Media_Control")]
pub mod Control;
#[cfg(feature = "Media_Core")]
pub mod Core;
#[cfg(feature = "Media_Devices")]
pub mod Devices;
#[cfg(feature = "Media_DialProtocol")]
pub mod DialProtocol;
#[cfg(feature = "Media_Editing")]
pub mod Editing;
#[cfg(feature = "Media_Effects")]
pub mod Effects;
#[cfg(feature = "Media_FaceAnalysis")]
pub mod FaceAnalysis;
#[cfg(feature = "Media_Import")]
pub mod Import;
#[cfg(feature = "Media_MediaProperties")]
pub mod MediaProperties;
#[cfg(feature = "Media_Miracast")]
pub mod Miracast;
#[cfg(feature = "Media_Ocr")]
pub mod Ocr;
#[cfg(feature = "Media_PlayTo")]
pub mod PlayTo;
#[cfg(feature = "Media_Playback")]
pub mod Playback;
#[cfg(feature = "Media_Playlists")]
pub mod Playlists;
#[cfg(feature = "Media_Protection")]
pub mod Protection;
#[cfg(feature = "Media_Render")]
pub mod Render;
#[cfg(feature = "Media_SpeechRecognition")]
pub mod SpeechRecognition;
#[cfg(feature = "Media_SpeechSynthesis")]
pub mod SpeechSynthesis;
#[cfg(feature = "Media_Streaming")]
pub mod Streaming;
#[cfg(feature = "Media_Transcoding")]
pub mod Transcoding;
windows_core::imp::define_interface!(IAudioBuffer, IAudioBuffer_Vtbl, 0x35175827_724b_4c6a_b130_f6537f9ae0d0);
impl windows_core::RuntimeType for IAudioBuffer {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAudioBuffer_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Capacity: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Length: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetLength: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAudioFrame, IAudioFrame_Vtbl, 0xe36ac304_aab2_4277_9ed0_43cedf8e29c6);
impl windows_core::RuntimeType for IAudioFrame {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAudioFrame_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub LockBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, AudioBufferAccessMode, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAudioFrameFactory, IAudioFrameFactory_Vtbl, 0x91a90ade_2422_40a6_b9ad_30d02404317d);
impl windows_core::RuntimeType for IAudioFrameFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAudioFrameFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAutoRepeatModeChangeRequestedEventArgs, IAutoRepeatModeChangeRequestedEventArgs_Vtbl, 0xea137efa_d852_438e_882b_c990109a78f4);
impl windows_core::RuntimeType for IAutoRepeatModeChangeRequestedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAutoRepeatModeChangeRequestedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub RequestedAutoRepeatMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut MediaPlaybackAutoRepeatMode) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IImageDisplayProperties, IImageDisplayProperties_Vtbl, 0xcd0bc7ef_54e7_411f_9933_f0e98b0a96d2);
impl windows_core::RuntimeType for IImageDisplayProperties {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IImageDisplayProperties_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Title: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetTitle: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Subtitle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetSubtitle: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
#[cfg(feature = "deprecated")]
windows_core::imp::define_interface!(IMediaControl, IMediaControl_Vtbl, 0x98f1fbe1_7a8d_42cb_b6fe_8fe698264f13);
#[cfg(feature = "deprecated")]
impl windows_core::RuntimeType for IMediaControl {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct IMediaControl_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "deprecated")]
    pub SoundLevelChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SoundLevelChanged: usize,
    #[cfg(feature = "deprecated")]
    pub RemoveSoundLevelChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    RemoveSoundLevelChanged: usize,
    #[cfg(feature = "deprecated")]
    pub PlayPressed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    PlayPressed: usize,
    #[cfg(feature = "deprecated")]
    pub RemovePlayPressed: unsafe extern "system" fn(*mut core::ffi::c_void, super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    RemovePlayPressed: usize,
    #[cfg(feature = "deprecated")]
    pub PausePressed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    PausePressed: usize,
    #[cfg(feature = "deprecated")]
    pub RemovePausePressed: unsafe extern "system" fn(*mut core::ffi::c_void, super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    RemovePausePressed: usize,
    #[cfg(feature = "deprecated")]
    pub StopPressed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    StopPressed: usize,
    #[cfg(feature = "deprecated")]
    pub RemoveStopPressed: unsafe extern "system" fn(*mut core::ffi::c_void, super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    RemoveStopPressed: usize,
    #[cfg(feature = "deprecated")]
    pub PlayPauseTogglePressed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    PlayPauseTogglePressed: usize,
    #[cfg(feature = "deprecated")]
    pub RemovePlayPauseTogglePressed: unsafe extern "system" fn(*mut core::ffi::c_void, super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    RemovePlayPauseTogglePressed: usize,
    #[cfg(feature = "deprecated")]
    pub RecordPressed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    RecordPressed: usize,
    #[cfg(feature = "deprecated")]
    pub RemoveRecordPressed: unsafe extern "system" fn(*mut core::ffi::c_void, super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    RemoveRecordPressed: usize,
    #[cfg(feature = "deprecated")]
    pub NextTrackPressed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    NextTrackPressed: usize,
    #[cfg(feature = "deprecated")]
    pub RemoveNextTrackPressed: unsafe extern "system" fn(*mut core::ffi::c_void, super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    RemoveNextTrackPressed: usize,
    #[cfg(feature = "deprecated")]
    pub PreviousTrackPressed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    PreviousTrackPressed: usize,
    #[cfg(feature = "deprecated")]
    pub RemovePreviousTrackPressed: unsafe extern "system" fn(*mut core::ffi::c_void, super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    RemovePreviousTrackPressed: usize,
    #[cfg(feature = "deprecated")]
    pub FastForwardPressed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    FastForwardPressed: usize,
    #[cfg(feature = "deprecated")]
    pub RemoveFastForwardPressed: unsafe extern "system" fn(*mut core::ffi::c_void, super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    RemoveFastForwardPressed: usize,
    #[cfg(feature = "deprecated")]
    pub RewindPressed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    RewindPressed: usize,
    #[cfg(feature = "deprecated")]
    pub RemoveRewindPressed: unsafe extern "system" fn(*mut core::ffi::c_void, super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    RemoveRewindPressed: usize,
    #[cfg(feature = "deprecated")]
    pub ChannelUpPressed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    ChannelUpPressed: usize,
    #[cfg(feature = "deprecated")]
    pub RemoveChannelUpPressed: unsafe extern "system" fn(*mut core::ffi::c_void, super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    RemoveChannelUpPressed: usize,
    #[cfg(feature = "deprecated")]
    pub ChannelDownPressed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    ChannelDownPressed: usize,
    #[cfg(feature = "deprecated")]
    pub RemoveChannelDownPressed: unsafe extern "system" fn(*mut core::ffi::c_void, super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    RemoveChannelDownPressed: usize,
    #[cfg(feature = "deprecated")]
    pub SoundLevel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SoundLevel) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SoundLevel: usize,
    #[cfg(feature = "deprecated")]
    pub SetTrackName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SetTrackName: usize,
    #[cfg(feature = "deprecated")]
    pub TrackName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    TrackName: usize,
    #[cfg(feature = "deprecated")]
    pub SetArtistName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SetArtistName: usize,
    #[cfg(feature = "deprecated")]
    pub ArtistName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    ArtistName: usize,
    #[cfg(feature = "deprecated")]
    pub SetIsPlaying: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SetIsPlaying: usize,
    #[cfg(feature = "deprecated")]
    pub IsPlaying: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    IsPlaying: usize,
    #[cfg(feature = "deprecated")]
    pub SetAlbumArt: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SetAlbumArt: usize,
    #[cfg(feature = "deprecated")]
    pub AlbumArt: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    AlbumArt: usize,
}
windows_core::imp::define_interface!(IMediaExtension, IMediaExtension_Vtbl, 0x07915118_45df_442b_8a3f_f7826a6370ab);
impl core::ops::Deref for IMediaExtension {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IMediaExtension, windows_core::IUnknown, windows_core::IInspectable);
impl IMediaExtension {
    #[cfg(feature = "Foundation_Collections")]
    pub fn SetProperties<P0>(&self, configuration: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Foundation::Collections::IPropertySet>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetProperties)(windows_core::Interface::as_raw(this), configuration.param().abi()).ok() }
    }
}
impl windows_core::RuntimeType for IMediaExtension {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMediaExtension_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub SetProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SetProperties: usize,
}
windows_core::imp::define_interface!(IMediaExtensionManager, IMediaExtensionManager_Vtbl, 0x4a25eaf5_242d_4dfb_97f4_69b7c42576ff);
impl windows_core::RuntimeType for IMediaExtensionManager {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMediaExtensionManager_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub RegisterSchemeHandler: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub RegisterSchemeHandlerWithSettings: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, core::mem::MaybeUninit<windows_core::HSTRING>, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    RegisterSchemeHandlerWithSettings: usize,
    pub RegisterByteStreamHandler: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, core::mem::MaybeUninit<windows_core::HSTRING>, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub RegisterByteStreamHandlerWithSettings: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, core::mem::MaybeUninit<windows_core::HSTRING>, core::mem::MaybeUninit<windows_core::HSTRING>, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    RegisterByteStreamHandlerWithSettings: usize,
    pub RegisterAudioDecoder: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, windows_core::GUID, windows_core::GUID) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub RegisterAudioDecoderWithSettings: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, windows_core::GUID, windows_core::GUID, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    RegisterAudioDecoderWithSettings: usize,
    pub RegisterAudioEncoder: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, windows_core::GUID, windows_core::GUID) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub RegisterAudioEncoderWithSettings: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, windows_core::GUID, windows_core::GUID, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    RegisterAudioEncoderWithSettings: usize,
    pub RegisterVideoDecoder: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, windows_core::GUID, windows_core::GUID) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub RegisterVideoDecoderWithSettings: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, windows_core::GUID, windows_core::GUID, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    RegisterVideoDecoderWithSettings: usize,
    pub RegisterVideoEncoder: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, windows_core::GUID, windows_core::GUID) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub RegisterVideoEncoderWithSettings: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, windows_core::GUID, windows_core::GUID, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    RegisterVideoEncoderWithSettings: usize,
}
windows_core::imp::define_interface!(IMediaExtensionManager2, IMediaExtensionManager2_Vtbl, 0x5bcebf47_4043_4fed_acaf_54ec29dfb1f7);
impl windows_core::RuntimeType for IMediaExtensionManager2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMediaExtensionManager2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "ApplicationModel_AppService")]
    pub RegisterMediaExtensionForAppService: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_AppService"))]
    RegisterMediaExtensionForAppService: usize,
}
windows_core::imp::define_interface!(IMediaFrame, IMediaFrame_Vtbl, 0xbfb52f8c_5943_47d8_8e10_05308aa5fbd0);
impl core::ops::Deref for IMediaFrame {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IMediaFrame, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(IMediaFrame, super::Foundation::IClosable);
impl IMediaFrame {
    pub fn Type(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Type)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn IsReadOnly(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsReadOnly)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetRelativeTime<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Foundation::IReference<super::Foundation::TimeSpan>>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetRelativeTime)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn RelativeTime(&self) -> windows_core::Result<super::Foundation::IReference<super::Foundation::TimeSpan>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RelativeTime)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetSystemRelativeTime<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Foundation::IReference<super::Foundation::TimeSpan>>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetSystemRelativeTime)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn SystemRelativeTime(&self) -> windows_core::Result<super::Foundation::IReference<super::Foundation::TimeSpan>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SystemRelativeTime)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetDuration<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Foundation::IReference<super::Foundation::TimeSpan>>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetDuration)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn Duration(&self) -> windows_core::Result<super::Foundation::IReference<super::Foundation::TimeSpan>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Duration)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetIsDiscontinuous(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetIsDiscontinuous)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsDiscontinuous(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsDiscontinuous)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn ExtendedProperties(&self) -> windows_core::Result<super::Foundation::Collections::IPropertySet> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ExtendedProperties)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
}
impl windows_core::RuntimeType for IMediaFrame {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMediaFrame_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Type: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub IsReadOnly: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetRelativeTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RelativeTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetSystemRelativeTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SystemRelativeTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetDuration: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Duration: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetIsDiscontinuous: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub IsDiscontinuous: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub ExtendedProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ExtendedProperties: usize,
}
windows_core::imp::define_interface!(IMediaMarker, IMediaMarker_Vtbl, 0x1803def8_dca5_4b6f_9c20_e3d3c0643625);
impl core::ops::Deref for IMediaMarker {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IMediaMarker, windows_core::IUnknown, windows_core::IInspectable);
impl IMediaMarker {
    pub fn Time(&self) -> windows_core::Result<super::Foundation::TimeSpan> {
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
}
impl windows_core::RuntimeType for IMediaMarker {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMediaMarker_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Time: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::Foundation::TimeSpan) -> windows_core::HRESULT,
    pub MediaMarkerType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Text: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMediaMarkerTypesStatics, IMediaMarkerTypesStatics_Vtbl, 0xbb198040_482f_4743_8832_45853821ece0);
impl windows_core::RuntimeType for IMediaMarkerTypesStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMediaMarkerTypesStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Bookmark: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMediaMarkers, IMediaMarkers_Vtbl, 0xafeab189_f8dd_466e_aa10_920b52353fdf);
impl core::ops::Deref for IMediaMarkers {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IMediaMarkers, windows_core::IUnknown, windows_core::IInspectable);
impl IMediaMarkers {
    #[cfg(feature = "Foundation_Collections")]
    pub fn Markers(&self) -> windows_core::Result<super::Foundation::Collections::IVectorView<IMediaMarker>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Markers)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for IMediaMarkers {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMediaMarkers_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Markers: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Markers: usize,
}
windows_core::imp::define_interface!(IMediaProcessingTriggerDetails, IMediaProcessingTriggerDetails_Vtbl, 0xeb8564ac_a351_4f4e_b4f0_9bf2408993db);
impl windows_core::RuntimeType for IMediaProcessingTriggerDetails {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMediaProcessingTriggerDetails_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Arguments: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Arguments: usize,
}
windows_core::imp::define_interface!(IMediaTimelineController, IMediaTimelineController_Vtbl, 0x8ed361f3_0b78_4360_bf71_0c841999ea1b);
impl windows_core::RuntimeType for IMediaTimelineController {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMediaTimelineController_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Start: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Resume: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Pause: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Position: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::Foundation::TimeSpan) -> windows_core::HRESULT,
    pub SetPosition: unsafe extern "system" fn(*mut core::ffi::c_void, super::Foundation::TimeSpan) -> windows_core::HRESULT,
    pub ClockRate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub SetClockRate: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub State: unsafe extern "system" fn(*mut core::ffi::c_void, *mut MediaTimelineControllerState) -> windows_core::HRESULT,
    pub PositionChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemovePositionChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub StateChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveStateChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMediaTimelineController2, IMediaTimelineController2_Vtbl, 0xef74ea38_9e72_4df9_8355_6e90c81bbadd);
impl windows_core::RuntimeType for IMediaTimelineController2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMediaTimelineController2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Duration: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetDuration: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsLoopingEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetIsLoopingEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub Failed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveFailed: unsafe extern "system" fn(*mut core::ffi::c_void, super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub Ended: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveEnded: unsafe extern "system" fn(*mut core::ffi::c_void, super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMediaTimelineControllerFailedEventArgs, IMediaTimelineControllerFailedEventArgs_Vtbl, 0x8821f81d_3e77_43fb_be26_4fc87a044834);
impl windows_core::RuntimeType for IMediaTimelineControllerFailedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMediaTimelineControllerFailedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ExtendedError: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::HRESULT) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMusicDisplayProperties, IMusicDisplayProperties_Vtbl, 0x6bbf0c59_d0a0_4d26_92a0_f978e1d18e7b);
impl windows_core::RuntimeType for IMusicDisplayProperties {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMusicDisplayProperties_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Title: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetTitle: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub AlbumArtist: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetAlbumArtist: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Artist: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetArtist: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMusicDisplayProperties2, IMusicDisplayProperties2_Vtbl, 0x00368462_97d3_44b9_b00f_008afcefaf18);
impl windows_core::RuntimeType for IMusicDisplayProperties2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMusicDisplayProperties2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub AlbumTitle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetAlbumTitle: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub TrackNumber: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetTrackNumber: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Genres: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Genres: usize,
}
windows_core::imp::define_interface!(IMusicDisplayProperties3, IMusicDisplayProperties3_Vtbl, 0x4db51ac1_0681_4e8c_9401_b8159d9eefc7);
impl windows_core::RuntimeType for IMusicDisplayProperties3 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMusicDisplayProperties3_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub AlbumTrackCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetAlbumTrackCount: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPlaybackPositionChangeRequestedEventArgs, IPlaybackPositionChangeRequestedEventArgs_Vtbl, 0xb4493f88_eb28_4961_9c14_335e44f3e125);
impl windows_core::RuntimeType for IPlaybackPositionChangeRequestedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPlaybackPositionChangeRequestedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub RequestedPlaybackPosition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::Foundation::TimeSpan) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPlaybackRateChangeRequestedEventArgs, IPlaybackRateChangeRequestedEventArgs_Vtbl, 0x2ce2c41f_3cd6_4f77_9ba7_eb27c26a2140);
impl windows_core::RuntimeType for IPlaybackRateChangeRequestedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPlaybackRateChangeRequestedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub RequestedPlaybackRate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IShuffleEnabledChangeRequestedEventArgs, IShuffleEnabledChangeRequestedEventArgs_Vtbl, 0x49b593fe_4fd0_4666_a314_c0e01940d302);
impl windows_core::RuntimeType for IShuffleEnabledChangeRequestedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IShuffleEnabledChangeRequestedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub RequestedShuffleEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISystemMediaTransportControls, ISystemMediaTransportControls_Vtbl, 0x99fa3ff4_1742_42a6_902e_087d41f965ec);
impl windows_core::RuntimeType for ISystemMediaTransportControls {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISystemMediaTransportControls_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub PlaybackStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut MediaPlaybackStatus) -> windows_core::HRESULT,
    pub SetPlaybackStatus: unsafe extern "system" fn(*mut core::ffi::c_void, MediaPlaybackStatus) -> windows_core::HRESULT,
    pub DisplayUpdater: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SoundLevel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SoundLevel) -> windows_core::HRESULT,
    pub IsEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetIsEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub IsPlayEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetIsPlayEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub IsStopEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetIsStopEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub IsPauseEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetIsPauseEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub IsRecordEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetIsRecordEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub IsFastForwardEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetIsFastForwardEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub IsRewindEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetIsRewindEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub IsPreviousEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetIsPreviousEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub IsNextEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetIsNextEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub IsChannelUpEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetIsChannelUpEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub IsChannelDownEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetIsChannelDownEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub ButtonPressed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveButtonPressed: unsafe extern "system" fn(*mut core::ffi::c_void, super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub PropertyChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemovePropertyChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISystemMediaTransportControls2, ISystemMediaTransportControls2_Vtbl, 0xea98d2f6_7f3c_4af2_a586_72889808efb1);
impl windows_core::RuntimeType for ISystemMediaTransportControls2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISystemMediaTransportControls2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub AutoRepeatMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut MediaPlaybackAutoRepeatMode) -> windows_core::HRESULT,
    pub SetAutoRepeatMode: unsafe extern "system" fn(*mut core::ffi::c_void, MediaPlaybackAutoRepeatMode) -> windows_core::HRESULT,
    pub ShuffleEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetShuffleEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub PlaybackRate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub SetPlaybackRate: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub UpdateTimelineProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PlaybackPositionChangeRequested: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemovePlaybackPositionChangeRequested: unsafe extern "system" fn(*mut core::ffi::c_void, super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub PlaybackRateChangeRequested: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemovePlaybackRateChangeRequested: unsafe extern "system" fn(*mut core::ffi::c_void, super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub ShuffleEnabledChangeRequested: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveShuffleEnabledChangeRequested: unsafe extern "system" fn(*mut core::ffi::c_void, super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub AutoRepeatModeChangeRequested: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveAutoRepeatModeChangeRequested: unsafe extern "system" fn(*mut core::ffi::c_void, super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISystemMediaTransportControlsButtonPressedEventArgs, ISystemMediaTransportControlsButtonPressedEventArgs_Vtbl, 0xb7f47116_a56f_4dc8_9e11_92031f4a87c2);
impl windows_core::RuntimeType for ISystemMediaTransportControlsButtonPressedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISystemMediaTransportControlsButtonPressedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Button: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SystemMediaTransportControlsButton) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISystemMediaTransportControlsDisplayUpdater, ISystemMediaTransportControlsDisplayUpdater_Vtbl, 0x8abbc53e_fa55_4ecf_ad8e_c984e5dd1550);
impl windows_core::RuntimeType for ISystemMediaTransportControlsDisplayUpdater {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISystemMediaTransportControlsDisplayUpdater_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Type: unsafe extern "system" fn(*mut core::ffi::c_void, *mut MediaPlaybackType) -> windows_core::HRESULT,
    pub SetType: unsafe extern "system" fn(*mut core::ffi::c_void, MediaPlaybackType) -> windows_core::HRESULT,
    pub AppMediaId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetAppMediaId: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub Thumbnail: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Thumbnail: usize,
    #[cfg(feature = "Storage_Streams")]
    pub SetThumbnail: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetThumbnail: usize,
    pub MusicProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub VideoProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ImageProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Storage")]
    pub CopyFromFileAsync: unsafe extern "system" fn(*mut core::ffi::c_void, MediaPlaybackType, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage"))]
    CopyFromFileAsync: usize,
    pub ClearAll: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Update: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISystemMediaTransportControlsPropertyChangedEventArgs, ISystemMediaTransportControlsPropertyChangedEventArgs_Vtbl, 0xd0ca0936_339b_4cb3_8eeb_737607f56e08);
impl windows_core::RuntimeType for ISystemMediaTransportControlsPropertyChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISystemMediaTransportControlsPropertyChangedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Property: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SystemMediaTransportControlsProperty) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISystemMediaTransportControlsStatics, ISystemMediaTransportControlsStatics_Vtbl, 0x43ba380a_eca4_4832_91ab_d415fae484c6);
impl windows_core::RuntimeType for ISystemMediaTransportControlsStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISystemMediaTransportControlsStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetForCurrentView: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISystemMediaTransportControlsTimelineProperties, ISystemMediaTransportControlsTimelineProperties_Vtbl, 0x5125316a_c3a2_475b_8507_93534dc88f15);
impl windows_core::RuntimeType for ISystemMediaTransportControlsTimelineProperties {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISystemMediaTransportControlsTimelineProperties_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub StartTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::Foundation::TimeSpan) -> windows_core::HRESULT,
    pub SetStartTime: unsafe extern "system" fn(*mut core::ffi::c_void, super::Foundation::TimeSpan) -> windows_core::HRESULT,
    pub EndTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::Foundation::TimeSpan) -> windows_core::HRESULT,
    pub SetEndTime: unsafe extern "system" fn(*mut core::ffi::c_void, super::Foundation::TimeSpan) -> windows_core::HRESULT,
    pub MinSeekTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::Foundation::TimeSpan) -> windows_core::HRESULT,
    pub SetMinSeekTime: unsafe extern "system" fn(*mut core::ffi::c_void, super::Foundation::TimeSpan) -> windows_core::HRESULT,
    pub MaxSeekTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::Foundation::TimeSpan) -> windows_core::HRESULT,
    pub SetMaxSeekTime: unsafe extern "system" fn(*mut core::ffi::c_void, super::Foundation::TimeSpan) -> windows_core::HRESULT,
    pub Position: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::Foundation::TimeSpan) -> windows_core::HRESULT,
    pub SetPosition: unsafe extern "system" fn(*mut core::ffi::c_void, super::Foundation::TimeSpan) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IVideoDisplayProperties, IVideoDisplayProperties_Vtbl, 0x5609fdb1_5d2d_4872_8170_45dee5bc2f5c);
impl windows_core::RuntimeType for IVideoDisplayProperties {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IVideoDisplayProperties_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Title: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetTitle: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Subtitle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetSubtitle: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IVideoDisplayProperties2, IVideoDisplayProperties2_Vtbl, 0xb410e1ce_ab52_41ab_a486_cc10fab152f9);
impl windows_core::RuntimeType for IVideoDisplayProperties2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IVideoDisplayProperties2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Genres: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Genres: usize,
}
windows_core::imp::define_interface!(IVideoEffectsStatics, IVideoEffectsStatics_Vtbl, 0x1fcda5e8_baf1_4521_980c_3bcebb44cf38);
impl windows_core::RuntimeType for IVideoEffectsStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IVideoEffectsStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub VideoStabilization: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IVideoFrame, IVideoFrame_Vtbl, 0x0cc06625_90fc_4c92_bd95_7ded21819d1c);
impl windows_core::RuntimeType for IVideoFrame {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IVideoFrame_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Graphics_Imaging")]
    pub SoftwareBitmap: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_Imaging"))]
    SoftwareBitmap: usize,
    pub CopyToAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Graphics_DirectX_Direct3D11")]
    pub Direct3DSurface: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX_Direct3D11"))]
    Direct3DSurface: usize,
}
windows_core::imp::define_interface!(IVideoFrame2, IVideoFrame2_Vtbl, 0x3837840d_336c_4366_8d46_060798736c5d);
impl windows_core::RuntimeType for IVideoFrame2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IVideoFrame2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Graphics_Imaging")]
    pub CopyToWithBoundsAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_Imaging"))]
    CopyToWithBoundsAsync: usize,
}
windows_core::imp::define_interface!(IVideoFrameFactory, IVideoFrameFactory_Vtbl, 0x014b6d69_2228_4c92_92ff_50c380d3e776);
impl windows_core::RuntimeType for IVideoFrameFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IVideoFrameFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Graphics_Imaging")]
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, super::Graphics::Imaging::BitmapPixelFormat, i32, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_Imaging"))]
    Create: usize,
    #[cfg(feature = "Graphics_Imaging")]
    pub CreateWithAlpha: unsafe extern "system" fn(*mut core::ffi::c_void, super::Graphics::Imaging::BitmapPixelFormat, i32, i32, super::Graphics::Imaging::BitmapAlphaMode, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_Imaging"))]
    CreateWithAlpha: usize,
}
windows_core::imp::define_interface!(IVideoFrameStatics, IVideoFrameStatics_Vtbl, 0xab2a556f_6111_4b33_8ec3_2b209a02e17a);
impl windows_core::RuntimeType for IVideoFrameStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IVideoFrameStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Graphics_DirectX")]
    pub CreateAsDirect3D11SurfaceBacked: unsafe extern "system" fn(*mut core::ffi::c_void, super::Graphics::DirectX::DirectXPixelFormat, i32, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX"))]
    CreateAsDirect3D11SurfaceBacked: usize,
    #[cfg(feature = "Graphics_DirectX_Direct3D11")]
    pub CreateAsDirect3D11SurfaceBackedWithDevice: unsafe extern "system" fn(*mut core::ffi::c_void, super::Graphics::DirectX::DirectXPixelFormat, i32, i32, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX_Direct3D11"))]
    CreateAsDirect3D11SurfaceBackedWithDevice: usize,
    #[cfg(feature = "Graphics_Imaging")]
    pub CreateWithSoftwareBitmap: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_Imaging"))]
    CreateWithSoftwareBitmap: usize,
    #[cfg(feature = "Graphics_DirectX_Direct3D11")]
    pub CreateWithDirect3D11Surface: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX_Direct3D11"))]
    CreateWithDirect3D11Surface: usize,
}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct AudioBuffer(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(AudioBuffer, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(AudioBuffer, super::Foundation::IClosable, super::Foundation::IMemoryBuffer);
impl AudioBuffer {
    pub fn Capacity(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Capacity)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Length(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Length)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetLength(&self, value: u32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetLength)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn CreateReference(&self) -> windows_core::Result<super::Foundation::IMemoryBufferReference> {
        let this = &windows_core::Interface::cast::<super::Foundation::IMemoryBuffer>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateReference)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for AudioBuffer {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IAudioBuffer>();
}
unsafe impl windows_core::Interface for AudioBuffer {
    type Vtable = IAudioBuffer_Vtbl;
    const IID: windows_core::GUID = <IAudioBuffer as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for AudioBuffer {
    const NAME: &'static str = "Windows.Media.AudioBuffer";
}
unsafe impl Send for AudioBuffer {}
unsafe impl Sync for AudioBuffer {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct AudioFrame(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(AudioFrame, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(AudioFrame, super::Foundation::IClosable, IMediaFrame);
impl AudioFrame {
    pub fn LockBuffer(&self, mode: AudioBufferAccessMode) -> windows_core::Result<AudioBuffer> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LockBuffer)(windows_core::Interface::as_raw(this), mode, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Create(capacity: u32) -> windows_core::Result<AudioFrame> {
        Self::IAudioFrameFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Create)(windows_core::Interface::as_raw(this), capacity, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Type(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IMediaFrame>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Type)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn IsReadOnly(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IMediaFrame>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsReadOnly)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetRelativeTime<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Foundation::IReference<super::Foundation::TimeSpan>>,
    {
        let this = &windows_core::Interface::cast::<IMediaFrame>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetRelativeTime)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn RelativeTime(&self) -> windows_core::Result<super::Foundation::IReference<super::Foundation::TimeSpan>> {
        let this = &windows_core::Interface::cast::<IMediaFrame>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RelativeTime)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetSystemRelativeTime<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Foundation::IReference<super::Foundation::TimeSpan>>,
    {
        let this = &windows_core::Interface::cast::<IMediaFrame>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetSystemRelativeTime)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn SystemRelativeTime(&self) -> windows_core::Result<super::Foundation::IReference<super::Foundation::TimeSpan>> {
        let this = &windows_core::Interface::cast::<IMediaFrame>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SystemRelativeTime)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetDuration<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Foundation::IReference<super::Foundation::TimeSpan>>,
    {
        let this = &windows_core::Interface::cast::<IMediaFrame>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetDuration)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn Duration(&self) -> windows_core::Result<super::Foundation::IReference<super::Foundation::TimeSpan>> {
        let this = &windows_core::Interface::cast::<IMediaFrame>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Duration)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetIsDiscontinuous(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IMediaFrame>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetIsDiscontinuous)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsDiscontinuous(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IMediaFrame>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsDiscontinuous)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn ExtendedProperties(&self) -> windows_core::Result<super::Foundation::Collections::IPropertySet> {
        let this = &windows_core::Interface::cast::<IMediaFrame>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ExtendedProperties)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[doc(hidden)]
    pub fn IAudioFrameFactory<R, F: FnOnce(&IAudioFrameFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<AudioFrame, IAudioFrameFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for AudioFrame {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IAudioFrame>();
}
unsafe impl windows_core::Interface for AudioFrame {
    type Vtable = IAudioFrame_Vtbl;
    const IID: windows_core::GUID = <IAudioFrame as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for AudioFrame {
    const NAME: &'static str = "Windows.Media.AudioFrame";
}
unsafe impl Send for AudioFrame {}
unsafe impl Sync for AudioFrame {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct AutoRepeatModeChangeRequestedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(AutoRepeatModeChangeRequestedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl AutoRepeatModeChangeRequestedEventArgs {
    pub fn RequestedAutoRepeatMode(&self) -> windows_core::Result<MediaPlaybackAutoRepeatMode> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RequestedAutoRepeatMode)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for AutoRepeatModeChangeRequestedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IAutoRepeatModeChangeRequestedEventArgs>();
}
unsafe impl windows_core::Interface for AutoRepeatModeChangeRequestedEventArgs {
    type Vtable = IAutoRepeatModeChangeRequestedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IAutoRepeatModeChangeRequestedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for AutoRepeatModeChangeRequestedEventArgs {
    const NAME: &'static str = "Windows.Media.AutoRepeatModeChangeRequestedEventArgs";
}
unsafe impl Send for AutoRepeatModeChangeRequestedEventArgs {}
unsafe impl Sync for AutoRepeatModeChangeRequestedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct ImageDisplayProperties(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ImageDisplayProperties, windows_core::IUnknown, windows_core::IInspectable);
impl ImageDisplayProperties {
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
    pub fn Subtitle(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Subtitle)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetSubtitle(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetSubtitle)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
}
impl windows_core::RuntimeType for ImageDisplayProperties {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IImageDisplayProperties>();
}
unsafe impl windows_core::Interface for ImageDisplayProperties {
    type Vtable = IImageDisplayProperties_Vtbl;
    const IID: windows_core::GUID = <IImageDisplayProperties as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ImageDisplayProperties {
    const NAME: &'static str = "Windows.Media.ImageDisplayProperties";
}
unsafe impl Send for ImageDisplayProperties {}
unsafe impl Sync for ImageDisplayProperties {}
#[cfg(feature = "deprecated")]
pub struct MediaControl;
#[cfg(feature = "deprecated")]
impl MediaControl {
    #[cfg(feature = "deprecated")]
    pub fn SoundLevelChanged<P0>(handler: P0) -> windows_core::Result<super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::Foundation::EventHandler<windows_core::IInspectable>>,
    {
        Self::IMediaControl(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SoundLevelChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        })
    }
    #[cfg(feature = "deprecated")]
    pub fn RemoveSoundLevelChanged(cookie: super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        Self::IMediaControl(|this| unsafe { (windows_core::Interface::vtable(this).RemoveSoundLevelChanged)(windows_core::Interface::as_raw(this), cookie).ok() })
    }
    #[cfg(feature = "deprecated")]
    pub fn PlayPressed<P0>(handler: P0) -> windows_core::Result<super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::Foundation::EventHandler<windows_core::IInspectable>>,
    {
        Self::IMediaControl(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PlayPressed)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        })
    }
    #[cfg(feature = "deprecated")]
    pub fn RemovePlayPressed(cookie: super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        Self::IMediaControl(|this| unsafe { (windows_core::Interface::vtable(this).RemovePlayPressed)(windows_core::Interface::as_raw(this), cookie).ok() })
    }
    #[cfg(feature = "deprecated")]
    pub fn PausePressed<P0>(handler: P0) -> windows_core::Result<super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::Foundation::EventHandler<windows_core::IInspectable>>,
    {
        Self::IMediaControl(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PausePressed)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        })
    }
    #[cfg(feature = "deprecated")]
    pub fn RemovePausePressed(cookie: super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        Self::IMediaControl(|this| unsafe { (windows_core::Interface::vtable(this).RemovePausePressed)(windows_core::Interface::as_raw(this), cookie).ok() })
    }
    #[cfg(feature = "deprecated")]
    pub fn StopPressed<P0>(handler: P0) -> windows_core::Result<super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::Foundation::EventHandler<windows_core::IInspectable>>,
    {
        Self::IMediaControl(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).StopPressed)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        })
    }
    #[cfg(feature = "deprecated")]
    pub fn RemoveStopPressed(cookie: super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        Self::IMediaControl(|this| unsafe { (windows_core::Interface::vtable(this).RemoveStopPressed)(windows_core::Interface::as_raw(this), cookie).ok() })
    }
    #[cfg(feature = "deprecated")]
    pub fn PlayPauseTogglePressed<P0>(handler: P0) -> windows_core::Result<super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::Foundation::EventHandler<windows_core::IInspectable>>,
    {
        Self::IMediaControl(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PlayPauseTogglePressed)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        })
    }
    #[cfg(feature = "deprecated")]
    pub fn RemovePlayPauseTogglePressed(cookie: super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        Self::IMediaControl(|this| unsafe { (windows_core::Interface::vtable(this).RemovePlayPauseTogglePressed)(windows_core::Interface::as_raw(this), cookie).ok() })
    }
    #[cfg(feature = "deprecated")]
    pub fn RecordPressed<P0>(handler: P0) -> windows_core::Result<super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::Foundation::EventHandler<windows_core::IInspectable>>,
    {
        Self::IMediaControl(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RecordPressed)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        })
    }
    #[cfg(feature = "deprecated")]
    pub fn RemoveRecordPressed(cookie: super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        Self::IMediaControl(|this| unsafe { (windows_core::Interface::vtable(this).RemoveRecordPressed)(windows_core::Interface::as_raw(this), cookie).ok() })
    }
    #[cfg(feature = "deprecated")]
    pub fn NextTrackPressed<P0>(handler: P0) -> windows_core::Result<super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::Foundation::EventHandler<windows_core::IInspectable>>,
    {
        Self::IMediaControl(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NextTrackPressed)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        })
    }
    #[cfg(feature = "deprecated")]
    pub fn RemoveNextTrackPressed(cookie: super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        Self::IMediaControl(|this| unsafe { (windows_core::Interface::vtable(this).RemoveNextTrackPressed)(windows_core::Interface::as_raw(this), cookie).ok() })
    }
    #[cfg(feature = "deprecated")]
    pub fn PreviousTrackPressed<P0>(handler: P0) -> windows_core::Result<super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::Foundation::EventHandler<windows_core::IInspectable>>,
    {
        Self::IMediaControl(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PreviousTrackPressed)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        })
    }
    #[cfg(feature = "deprecated")]
    pub fn RemovePreviousTrackPressed(cookie: super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        Self::IMediaControl(|this| unsafe { (windows_core::Interface::vtable(this).RemovePreviousTrackPressed)(windows_core::Interface::as_raw(this), cookie).ok() })
    }
    #[cfg(feature = "deprecated")]
    pub fn FastForwardPressed<P0>(handler: P0) -> windows_core::Result<super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::Foundation::EventHandler<windows_core::IInspectable>>,
    {
        Self::IMediaControl(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FastForwardPressed)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        })
    }
    #[cfg(feature = "deprecated")]
    pub fn RemoveFastForwardPressed(cookie: super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        Self::IMediaControl(|this| unsafe { (windows_core::Interface::vtable(this).RemoveFastForwardPressed)(windows_core::Interface::as_raw(this), cookie).ok() })
    }
    #[cfg(feature = "deprecated")]
    pub fn RewindPressed<P0>(handler: P0) -> windows_core::Result<super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::Foundation::EventHandler<windows_core::IInspectable>>,
    {
        Self::IMediaControl(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RewindPressed)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        })
    }
    #[cfg(feature = "deprecated")]
    pub fn RemoveRewindPressed(cookie: super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        Self::IMediaControl(|this| unsafe { (windows_core::Interface::vtable(this).RemoveRewindPressed)(windows_core::Interface::as_raw(this), cookie).ok() })
    }
    #[cfg(feature = "deprecated")]
    pub fn ChannelUpPressed<P0>(handler: P0) -> windows_core::Result<super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::Foundation::EventHandler<windows_core::IInspectable>>,
    {
        Self::IMediaControl(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ChannelUpPressed)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        })
    }
    #[cfg(feature = "deprecated")]
    pub fn RemoveChannelUpPressed(cookie: super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        Self::IMediaControl(|this| unsafe { (windows_core::Interface::vtable(this).RemoveChannelUpPressed)(windows_core::Interface::as_raw(this), cookie).ok() })
    }
    #[cfg(feature = "deprecated")]
    pub fn ChannelDownPressed<P0>(handler: P0) -> windows_core::Result<super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::Foundation::EventHandler<windows_core::IInspectable>>,
    {
        Self::IMediaControl(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ChannelDownPressed)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        })
    }
    #[cfg(feature = "deprecated")]
    pub fn RemoveChannelDownPressed(cookie: super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        Self::IMediaControl(|this| unsafe { (windows_core::Interface::vtable(this).RemoveChannelDownPressed)(windows_core::Interface::as_raw(this), cookie).ok() })
    }
    #[cfg(feature = "deprecated")]
    pub fn SoundLevel() -> windows_core::Result<SoundLevel> {
        Self::IMediaControl(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SoundLevel)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    #[cfg(feature = "deprecated")]
    pub fn SetTrackName(value: &windows_core::HSTRING) -> windows_core::Result<()> {
        Self::IMediaControl(|this| unsafe { (windows_core::Interface::vtable(this).SetTrackName)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() })
    }
    #[cfg(feature = "deprecated")]
    pub fn TrackName() -> windows_core::Result<windows_core::HSTRING> {
        Self::IMediaControl(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TrackName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "deprecated")]
    pub fn SetArtistName(value: &windows_core::HSTRING) -> windows_core::Result<()> {
        Self::IMediaControl(|this| unsafe { (windows_core::Interface::vtable(this).SetArtistName)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() })
    }
    #[cfg(feature = "deprecated")]
    pub fn ArtistName() -> windows_core::Result<windows_core::HSTRING> {
        Self::IMediaControl(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ArtistName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "deprecated")]
    pub fn SetIsPlaying(value: bool) -> windows_core::Result<()> {
        Self::IMediaControl(|this| unsafe { (windows_core::Interface::vtable(this).SetIsPlaying)(windows_core::Interface::as_raw(this), value).ok() })
    }
    #[cfg(feature = "deprecated")]
    pub fn IsPlaying() -> windows_core::Result<bool> {
        Self::IMediaControl(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsPlaying)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    #[cfg(feature = "deprecated")]
    pub fn SetAlbumArt<P0>(value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Foundation::Uri>,
    {
        Self::IMediaControl(|this| unsafe { (windows_core::Interface::vtable(this).SetAlbumArt)(windows_core::Interface::as_raw(this), value.param().abi()).ok() })
    }
    #[cfg(feature = "deprecated")]
    pub fn AlbumArt() -> windows_core::Result<super::Foundation::Uri> {
        Self::IMediaControl(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AlbumArt)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    #[cfg(feature = "deprecated")]
    pub fn IMediaControl<R, F: FnOnce(&IMediaControl) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<MediaControl, IMediaControl> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
#[cfg(feature = "deprecated")]
impl windows_core::RuntimeName for MediaControl {
    const NAME: &'static str = "Windows.Media.MediaControl";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct MediaExtensionManager(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(MediaExtensionManager, windows_core::IUnknown, windows_core::IInspectable);
impl MediaExtensionManager {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<MediaExtensionManager, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn RegisterSchemeHandler(&self, activatableclassid: &windows_core::HSTRING, scheme: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RegisterSchemeHandler)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(activatableclassid), core::mem::transmute_copy(scheme)).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn RegisterSchemeHandlerWithSettings<P0>(&self, activatableclassid: &windows_core::HSTRING, scheme: &windows_core::HSTRING, configuration: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Foundation::Collections::IPropertySet>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RegisterSchemeHandlerWithSettings)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(activatableclassid), core::mem::transmute_copy(scheme), configuration.param().abi()).ok() }
    }
    pub fn RegisterByteStreamHandler(&self, activatableclassid: &windows_core::HSTRING, fileextension: &windows_core::HSTRING, mimetype: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RegisterByteStreamHandler)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(activatableclassid), core::mem::transmute_copy(fileextension), core::mem::transmute_copy(mimetype)).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn RegisterByteStreamHandlerWithSettings<P0>(&self, activatableclassid: &windows_core::HSTRING, fileextension: &windows_core::HSTRING, mimetype: &windows_core::HSTRING, configuration: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Foundation::Collections::IPropertySet>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RegisterByteStreamHandlerWithSettings)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(activatableclassid), core::mem::transmute_copy(fileextension), core::mem::transmute_copy(mimetype), configuration.param().abi()).ok() }
    }
    pub fn RegisterAudioDecoder(&self, activatableclassid: &windows_core::HSTRING, inputsubtype: windows_core::GUID, outputsubtype: windows_core::GUID) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RegisterAudioDecoder)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(activatableclassid), inputsubtype, outputsubtype).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn RegisterAudioDecoderWithSettings<P0>(&self, activatableclassid: &windows_core::HSTRING, inputsubtype: windows_core::GUID, outputsubtype: windows_core::GUID, configuration: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Foundation::Collections::IPropertySet>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RegisterAudioDecoderWithSettings)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(activatableclassid), inputsubtype, outputsubtype, configuration.param().abi()).ok() }
    }
    pub fn RegisterAudioEncoder(&self, activatableclassid: &windows_core::HSTRING, inputsubtype: windows_core::GUID, outputsubtype: windows_core::GUID) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RegisterAudioEncoder)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(activatableclassid), inputsubtype, outputsubtype).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn RegisterAudioEncoderWithSettings<P0>(&self, activatableclassid: &windows_core::HSTRING, inputsubtype: windows_core::GUID, outputsubtype: windows_core::GUID, configuration: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Foundation::Collections::IPropertySet>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RegisterAudioEncoderWithSettings)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(activatableclassid), inputsubtype, outputsubtype, configuration.param().abi()).ok() }
    }
    pub fn RegisterVideoDecoder(&self, activatableclassid: &windows_core::HSTRING, inputsubtype: windows_core::GUID, outputsubtype: windows_core::GUID) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RegisterVideoDecoder)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(activatableclassid), inputsubtype, outputsubtype).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn RegisterVideoDecoderWithSettings<P0>(&self, activatableclassid: &windows_core::HSTRING, inputsubtype: windows_core::GUID, outputsubtype: windows_core::GUID, configuration: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Foundation::Collections::IPropertySet>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RegisterVideoDecoderWithSettings)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(activatableclassid), inputsubtype, outputsubtype, configuration.param().abi()).ok() }
    }
    pub fn RegisterVideoEncoder(&self, activatableclassid: &windows_core::HSTRING, inputsubtype: windows_core::GUID, outputsubtype: windows_core::GUID) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RegisterVideoEncoder)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(activatableclassid), inputsubtype, outputsubtype).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn RegisterVideoEncoderWithSettings<P0>(&self, activatableclassid: &windows_core::HSTRING, inputsubtype: windows_core::GUID, outputsubtype: windows_core::GUID, configuration: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Foundation::Collections::IPropertySet>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RegisterVideoEncoderWithSettings)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(activatableclassid), inputsubtype, outputsubtype, configuration.param().abi()).ok() }
    }
    #[cfg(feature = "ApplicationModel_AppService")]
    pub fn RegisterMediaExtensionForAppService<P0, P1>(&self, extension: P0, connection: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMediaExtension>,
        P1: windows_core::Param<super::ApplicationModel::AppService::AppServiceConnection>,
    {
        let this = &windows_core::Interface::cast::<IMediaExtensionManager2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RegisterMediaExtensionForAppService)(windows_core::Interface::as_raw(this), extension.param().abi(), connection.param().abi()).ok() }
    }
}
impl windows_core::RuntimeType for MediaExtensionManager {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IMediaExtensionManager>();
}
unsafe impl windows_core::Interface for MediaExtensionManager {
    type Vtable = IMediaExtensionManager_Vtbl;
    const IID: windows_core::GUID = <IMediaExtensionManager as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for MediaExtensionManager {
    const NAME: &'static str = "Windows.Media.MediaExtensionManager";
}
unsafe impl Send for MediaExtensionManager {}
unsafe impl Sync for MediaExtensionManager {}
pub struct MediaMarkerTypes;
impl MediaMarkerTypes {
    pub fn Bookmark() -> windows_core::Result<windows_core::HSTRING> {
        Self::IMediaMarkerTypesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Bookmark)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IMediaMarkerTypesStatics<R, F: FnOnce(&IMediaMarkerTypesStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<MediaMarkerTypes, IMediaMarkerTypesStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeName for MediaMarkerTypes {
    const NAME: &'static str = "Windows.Media.MediaMarkerTypes";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct MediaProcessingTriggerDetails(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(MediaProcessingTriggerDetails, windows_core::IUnknown, windows_core::IInspectable);
impl MediaProcessingTriggerDetails {
    #[cfg(feature = "Foundation_Collections")]
    pub fn Arguments(&self) -> windows_core::Result<super::Foundation::Collections::ValueSet> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Arguments)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for MediaProcessingTriggerDetails {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IMediaProcessingTriggerDetails>();
}
unsafe impl windows_core::Interface for MediaProcessingTriggerDetails {
    type Vtable = IMediaProcessingTriggerDetails_Vtbl;
    const IID: windows_core::GUID = <IMediaProcessingTriggerDetails as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for MediaProcessingTriggerDetails {
    const NAME: &'static str = "Windows.Media.MediaProcessingTriggerDetails";
}
unsafe impl Send for MediaProcessingTriggerDetails {}
unsafe impl Sync for MediaProcessingTriggerDetails {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct MediaTimelineController(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(MediaTimelineController, windows_core::IUnknown, windows_core::IInspectable);
impl MediaTimelineController {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<MediaTimelineController, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Start(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Start)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Resume(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Resume)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Pause(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Pause)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Position(&self) -> windows_core::Result<super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Position)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetPosition(&self, value: super::Foundation::TimeSpan) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetPosition)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ClockRate(&self) -> windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ClockRate)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetClockRate(&self, value: f64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetClockRate)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn State(&self) -> windows_core::Result<MediaTimelineControllerState> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).State)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PositionChanged<P0>(&self, positionchangedeventhandler: P0) -> windows_core::Result<super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::Foundation::TypedEventHandler<MediaTimelineController, windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PositionChanged)(windows_core::Interface::as_raw(this), positionchangedeventhandler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemovePositionChanged(&self, eventcookie: super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemovePositionChanged)(windows_core::Interface::as_raw(this), eventcookie).ok() }
    }
    pub fn StateChanged<P0>(&self, statechangedeventhandler: P0) -> windows_core::Result<super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::Foundation::TypedEventHandler<MediaTimelineController, windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).StateChanged)(windows_core::Interface::as_raw(this), statechangedeventhandler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveStateChanged(&self, eventcookie: super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveStateChanged)(windows_core::Interface::as_raw(this), eventcookie).ok() }
    }
    pub fn Duration(&self) -> windows_core::Result<super::Foundation::IReference<super::Foundation::TimeSpan>> {
        let this = &windows_core::Interface::cast::<IMediaTimelineController2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Duration)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetDuration<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Foundation::IReference<super::Foundation::TimeSpan>>,
    {
        let this = &windows_core::Interface::cast::<IMediaTimelineController2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetDuration)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn IsLoopingEnabled(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IMediaTimelineController2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsLoopingEnabled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsLoopingEnabled(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IMediaTimelineController2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetIsLoopingEnabled)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Failed<P0>(&self, eventhandler: P0) -> windows_core::Result<super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::Foundation::TypedEventHandler<MediaTimelineController, MediaTimelineControllerFailedEventArgs>>,
    {
        let this = &windows_core::Interface::cast::<IMediaTimelineController2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Failed)(windows_core::Interface::as_raw(this), eventhandler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveFailed(&self, token: super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IMediaTimelineController2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemoveFailed)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn Ended<P0>(&self, eventhandler: P0) -> windows_core::Result<super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::Foundation::TypedEventHandler<MediaTimelineController, windows_core::IInspectable>>,
    {
        let this = &windows_core::Interface::cast::<IMediaTimelineController2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Ended)(windows_core::Interface::as_raw(this), eventhandler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveEnded(&self, token: super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IMediaTimelineController2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemoveEnded)(windows_core::Interface::as_raw(this), token).ok() }
    }
}
impl windows_core::RuntimeType for MediaTimelineController {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IMediaTimelineController>();
}
unsafe impl windows_core::Interface for MediaTimelineController {
    type Vtable = IMediaTimelineController_Vtbl;
    const IID: windows_core::GUID = <IMediaTimelineController as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for MediaTimelineController {
    const NAME: &'static str = "Windows.Media.MediaTimelineController";
}
unsafe impl Send for MediaTimelineController {}
unsafe impl Sync for MediaTimelineController {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct MediaTimelineControllerFailedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(MediaTimelineControllerFailedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl MediaTimelineControllerFailedEventArgs {
    pub fn ExtendedError(&self) -> windows_core::Result<windows_core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ExtendedError)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for MediaTimelineControllerFailedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IMediaTimelineControllerFailedEventArgs>();
}
unsafe impl windows_core::Interface for MediaTimelineControllerFailedEventArgs {
    type Vtable = IMediaTimelineControllerFailedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IMediaTimelineControllerFailedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for MediaTimelineControllerFailedEventArgs {
    const NAME: &'static str = "Windows.Media.MediaTimelineControllerFailedEventArgs";
}
unsafe impl Send for MediaTimelineControllerFailedEventArgs {}
unsafe impl Sync for MediaTimelineControllerFailedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct MusicDisplayProperties(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(MusicDisplayProperties, windows_core::IUnknown, windows_core::IInspectable);
impl MusicDisplayProperties {
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
    pub fn AlbumArtist(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AlbumArtist)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetAlbumArtist(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetAlbumArtist)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn Artist(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Artist)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetArtist(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetArtist)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn AlbumTitle(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IMusicDisplayProperties2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AlbumTitle)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetAlbumTitle(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IMusicDisplayProperties2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetAlbumTitle)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn TrackNumber(&self) -> windows_core::Result<u32> {
        let this = &windows_core::Interface::cast::<IMusicDisplayProperties2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TrackNumber)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetTrackNumber(&self, value: u32) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IMusicDisplayProperties2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetTrackNumber)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Genres(&self) -> windows_core::Result<super::Foundation::Collections::IVector<windows_core::HSTRING>> {
        let this = &windows_core::Interface::cast::<IMusicDisplayProperties2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Genres)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn AlbumTrackCount(&self) -> windows_core::Result<u32> {
        let this = &windows_core::Interface::cast::<IMusicDisplayProperties3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AlbumTrackCount)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetAlbumTrackCount(&self, value: u32) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IMusicDisplayProperties3>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetAlbumTrackCount)(windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl windows_core::RuntimeType for MusicDisplayProperties {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IMusicDisplayProperties>();
}
unsafe impl windows_core::Interface for MusicDisplayProperties {
    type Vtable = IMusicDisplayProperties_Vtbl;
    const IID: windows_core::GUID = <IMusicDisplayProperties as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for MusicDisplayProperties {
    const NAME: &'static str = "Windows.Media.MusicDisplayProperties";
}
unsafe impl Send for MusicDisplayProperties {}
unsafe impl Sync for MusicDisplayProperties {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct PlaybackPositionChangeRequestedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PlaybackPositionChangeRequestedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl PlaybackPositionChangeRequestedEventArgs {
    pub fn RequestedPlaybackPosition(&self) -> windows_core::Result<super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RequestedPlaybackPosition)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for PlaybackPositionChangeRequestedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPlaybackPositionChangeRequestedEventArgs>();
}
unsafe impl windows_core::Interface for PlaybackPositionChangeRequestedEventArgs {
    type Vtable = IPlaybackPositionChangeRequestedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IPlaybackPositionChangeRequestedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PlaybackPositionChangeRequestedEventArgs {
    const NAME: &'static str = "Windows.Media.PlaybackPositionChangeRequestedEventArgs";
}
unsafe impl Send for PlaybackPositionChangeRequestedEventArgs {}
unsafe impl Sync for PlaybackPositionChangeRequestedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct PlaybackRateChangeRequestedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PlaybackRateChangeRequestedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl PlaybackRateChangeRequestedEventArgs {
    pub fn RequestedPlaybackRate(&self) -> windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RequestedPlaybackRate)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for PlaybackRateChangeRequestedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPlaybackRateChangeRequestedEventArgs>();
}
unsafe impl windows_core::Interface for PlaybackRateChangeRequestedEventArgs {
    type Vtable = IPlaybackRateChangeRequestedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IPlaybackRateChangeRequestedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PlaybackRateChangeRequestedEventArgs {
    const NAME: &'static str = "Windows.Media.PlaybackRateChangeRequestedEventArgs";
}
unsafe impl Send for PlaybackRateChangeRequestedEventArgs {}
unsafe impl Sync for PlaybackRateChangeRequestedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct ShuffleEnabledChangeRequestedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ShuffleEnabledChangeRequestedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl ShuffleEnabledChangeRequestedEventArgs {
    pub fn RequestedShuffleEnabled(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RequestedShuffleEnabled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for ShuffleEnabledChangeRequestedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IShuffleEnabledChangeRequestedEventArgs>();
}
unsafe impl windows_core::Interface for ShuffleEnabledChangeRequestedEventArgs {
    type Vtable = IShuffleEnabledChangeRequestedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IShuffleEnabledChangeRequestedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ShuffleEnabledChangeRequestedEventArgs {
    const NAME: &'static str = "Windows.Media.ShuffleEnabledChangeRequestedEventArgs";
}
unsafe impl Send for ShuffleEnabledChangeRequestedEventArgs {}
unsafe impl Sync for ShuffleEnabledChangeRequestedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct SystemMediaTransportControls(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(SystemMediaTransportControls, windows_core::IUnknown, windows_core::IInspectable);
impl SystemMediaTransportControls {
    pub fn PlaybackStatus(&self) -> windows_core::Result<MediaPlaybackStatus> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PlaybackStatus)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetPlaybackStatus(&self, value: MediaPlaybackStatus) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetPlaybackStatus)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn DisplayUpdater(&self) -> windows_core::Result<SystemMediaTransportControlsDisplayUpdater> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DisplayUpdater)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SoundLevel(&self) -> windows_core::Result<SoundLevel> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SoundLevel)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
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
    pub fn IsPlayEnabled(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsPlayEnabled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsPlayEnabled(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetIsPlayEnabled)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsStopEnabled(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsStopEnabled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsStopEnabled(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetIsStopEnabled)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsPauseEnabled(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsPauseEnabled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsPauseEnabled(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetIsPauseEnabled)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsRecordEnabled(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsRecordEnabled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsRecordEnabled(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetIsRecordEnabled)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsFastForwardEnabled(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsFastForwardEnabled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsFastForwardEnabled(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetIsFastForwardEnabled)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsRewindEnabled(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsRewindEnabled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsRewindEnabled(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetIsRewindEnabled)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsPreviousEnabled(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsPreviousEnabled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsPreviousEnabled(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetIsPreviousEnabled)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsNextEnabled(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsNextEnabled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsNextEnabled(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetIsNextEnabled)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsChannelUpEnabled(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsChannelUpEnabled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsChannelUpEnabled(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetIsChannelUpEnabled)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsChannelDownEnabled(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsChannelDownEnabled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsChannelDownEnabled(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetIsChannelDownEnabled)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ButtonPressed<P0>(&self, handler: P0) -> windows_core::Result<super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::Foundation::TypedEventHandler<SystemMediaTransportControls, SystemMediaTransportControlsButtonPressedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ButtonPressed)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveButtonPressed(&self, token: super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveButtonPressed)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn PropertyChanged<P0>(&self, handler: P0) -> windows_core::Result<super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::Foundation::TypedEventHandler<SystemMediaTransportControls, SystemMediaTransportControlsPropertyChangedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PropertyChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemovePropertyChanged(&self, token: super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemovePropertyChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn AutoRepeatMode(&self) -> windows_core::Result<MediaPlaybackAutoRepeatMode> {
        let this = &windows_core::Interface::cast::<ISystemMediaTransportControls2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AutoRepeatMode)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetAutoRepeatMode(&self, value: MediaPlaybackAutoRepeatMode) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<ISystemMediaTransportControls2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetAutoRepeatMode)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ShuffleEnabled(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<ISystemMediaTransportControls2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ShuffleEnabled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetShuffleEnabled(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<ISystemMediaTransportControls2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetShuffleEnabled)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn PlaybackRate(&self) -> windows_core::Result<f64> {
        let this = &windows_core::Interface::cast::<ISystemMediaTransportControls2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PlaybackRate)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetPlaybackRate(&self, value: f64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<ISystemMediaTransportControls2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetPlaybackRate)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn UpdateTimelineProperties<P0>(&self, timelineproperties: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<SystemMediaTransportControlsTimelineProperties>,
    {
        let this = &windows_core::Interface::cast::<ISystemMediaTransportControls2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).UpdateTimelineProperties)(windows_core::Interface::as_raw(this), timelineproperties.param().abi()).ok() }
    }
    pub fn PlaybackPositionChangeRequested<P0>(&self, handler: P0) -> windows_core::Result<super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::Foundation::TypedEventHandler<SystemMediaTransportControls, PlaybackPositionChangeRequestedEventArgs>>,
    {
        let this = &windows_core::Interface::cast::<ISystemMediaTransportControls2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PlaybackPositionChangeRequested)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemovePlaybackPositionChangeRequested(&self, token: super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<ISystemMediaTransportControls2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemovePlaybackPositionChangeRequested)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn PlaybackRateChangeRequested<P0>(&self, handler: P0) -> windows_core::Result<super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::Foundation::TypedEventHandler<SystemMediaTransportControls, PlaybackRateChangeRequestedEventArgs>>,
    {
        let this = &windows_core::Interface::cast::<ISystemMediaTransportControls2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PlaybackRateChangeRequested)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemovePlaybackRateChangeRequested(&self, token: super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<ISystemMediaTransportControls2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemovePlaybackRateChangeRequested)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn ShuffleEnabledChangeRequested<P0>(&self, handler: P0) -> windows_core::Result<super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::Foundation::TypedEventHandler<SystemMediaTransportControls, ShuffleEnabledChangeRequestedEventArgs>>,
    {
        let this = &windows_core::Interface::cast::<ISystemMediaTransportControls2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ShuffleEnabledChangeRequested)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveShuffleEnabledChangeRequested(&self, token: super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<ISystemMediaTransportControls2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemoveShuffleEnabledChangeRequested)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn AutoRepeatModeChangeRequested<P0>(&self, handler: P0) -> windows_core::Result<super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::Foundation::TypedEventHandler<SystemMediaTransportControls, AutoRepeatModeChangeRequestedEventArgs>>,
    {
        let this = &windows_core::Interface::cast::<ISystemMediaTransportControls2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AutoRepeatModeChangeRequested)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveAutoRepeatModeChangeRequested(&self, token: super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<ISystemMediaTransportControls2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemoveAutoRepeatModeChangeRequested)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn GetForCurrentView() -> windows_core::Result<SystemMediaTransportControls> {
        Self::ISystemMediaTransportControlsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetForCurrentView)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn ISystemMediaTransportControlsStatics<R, F: FnOnce(&ISystemMediaTransportControlsStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<SystemMediaTransportControls, ISystemMediaTransportControlsStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for SystemMediaTransportControls {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ISystemMediaTransportControls>();
}
unsafe impl windows_core::Interface for SystemMediaTransportControls {
    type Vtable = ISystemMediaTransportControls_Vtbl;
    const IID: windows_core::GUID = <ISystemMediaTransportControls as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for SystemMediaTransportControls {
    const NAME: &'static str = "Windows.Media.SystemMediaTransportControls";
}
unsafe impl Send for SystemMediaTransportControls {}
unsafe impl Sync for SystemMediaTransportControls {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct SystemMediaTransportControlsButtonPressedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(SystemMediaTransportControlsButtonPressedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl SystemMediaTransportControlsButtonPressedEventArgs {
    pub fn Button(&self) -> windows_core::Result<SystemMediaTransportControlsButton> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Button)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for SystemMediaTransportControlsButtonPressedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ISystemMediaTransportControlsButtonPressedEventArgs>();
}
unsafe impl windows_core::Interface for SystemMediaTransportControlsButtonPressedEventArgs {
    type Vtable = ISystemMediaTransportControlsButtonPressedEventArgs_Vtbl;
    const IID: windows_core::GUID = <ISystemMediaTransportControlsButtonPressedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for SystemMediaTransportControlsButtonPressedEventArgs {
    const NAME: &'static str = "Windows.Media.SystemMediaTransportControlsButtonPressedEventArgs";
}
unsafe impl Send for SystemMediaTransportControlsButtonPressedEventArgs {}
unsafe impl Sync for SystemMediaTransportControlsButtonPressedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct SystemMediaTransportControlsDisplayUpdater(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(SystemMediaTransportControlsDisplayUpdater, windows_core::IUnknown, windows_core::IInspectable);
impl SystemMediaTransportControlsDisplayUpdater {
    pub fn Type(&self) -> windows_core::Result<MediaPlaybackType> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Type)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetType(&self, value: MediaPlaybackType) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetType)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn AppMediaId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AppMediaId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetAppMediaId(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetAppMediaId)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn Thumbnail(&self) -> windows_core::Result<super::Storage::Streams::RandomAccessStreamReference> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Thumbnail)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn SetThumbnail<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Storage::Streams::RandomAccessStreamReference>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetThumbnail)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn MusicProperties(&self) -> windows_core::Result<MusicDisplayProperties> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MusicProperties)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn VideoProperties(&self) -> windows_core::Result<VideoDisplayProperties> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).VideoProperties)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ImageProperties(&self) -> windows_core::Result<ImageDisplayProperties> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ImageProperties)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage")]
    pub fn CopyFromFileAsync<P0>(&self, r#type: MediaPlaybackType, source: P0) -> windows_core::Result<super::Foundation::IAsyncOperation<bool>>
    where
        P0: windows_core::Param<super::Storage::StorageFile>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CopyFromFileAsync)(windows_core::Interface::as_raw(this), r#type, source.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ClearAll(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).ClearAll)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Update(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Update)(windows_core::Interface::as_raw(this)).ok() }
    }
}
impl windows_core::RuntimeType for SystemMediaTransportControlsDisplayUpdater {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ISystemMediaTransportControlsDisplayUpdater>();
}
unsafe impl windows_core::Interface for SystemMediaTransportControlsDisplayUpdater {
    type Vtable = ISystemMediaTransportControlsDisplayUpdater_Vtbl;
    const IID: windows_core::GUID = <ISystemMediaTransportControlsDisplayUpdater as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for SystemMediaTransportControlsDisplayUpdater {
    const NAME: &'static str = "Windows.Media.SystemMediaTransportControlsDisplayUpdater";
}
unsafe impl Send for SystemMediaTransportControlsDisplayUpdater {}
unsafe impl Sync for SystemMediaTransportControlsDisplayUpdater {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct SystemMediaTransportControlsPropertyChangedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(SystemMediaTransportControlsPropertyChangedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl SystemMediaTransportControlsPropertyChangedEventArgs {
    pub fn Property(&self) -> windows_core::Result<SystemMediaTransportControlsProperty> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Property)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for SystemMediaTransportControlsPropertyChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ISystemMediaTransportControlsPropertyChangedEventArgs>();
}
unsafe impl windows_core::Interface for SystemMediaTransportControlsPropertyChangedEventArgs {
    type Vtable = ISystemMediaTransportControlsPropertyChangedEventArgs_Vtbl;
    const IID: windows_core::GUID = <ISystemMediaTransportControlsPropertyChangedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for SystemMediaTransportControlsPropertyChangedEventArgs {
    const NAME: &'static str = "Windows.Media.SystemMediaTransportControlsPropertyChangedEventArgs";
}
unsafe impl Send for SystemMediaTransportControlsPropertyChangedEventArgs {}
unsafe impl Sync for SystemMediaTransportControlsPropertyChangedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct SystemMediaTransportControlsTimelineProperties(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(SystemMediaTransportControlsTimelineProperties, windows_core::IUnknown, windows_core::IInspectable);
impl SystemMediaTransportControlsTimelineProperties {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<SystemMediaTransportControlsTimelineProperties, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn StartTime(&self) -> windows_core::Result<super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).StartTime)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetStartTime(&self, value: super::Foundation::TimeSpan) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetStartTime)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn EndTime(&self) -> windows_core::Result<super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).EndTime)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetEndTime(&self, value: super::Foundation::TimeSpan) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetEndTime)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn MinSeekTime(&self) -> windows_core::Result<super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MinSeekTime)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetMinSeekTime(&self, value: super::Foundation::TimeSpan) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetMinSeekTime)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn MaxSeekTime(&self) -> windows_core::Result<super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MaxSeekTime)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetMaxSeekTime(&self, value: super::Foundation::TimeSpan) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetMaxSeekTime)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Position(&self) -> windows_core::Result<super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Position)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetPosition(&self, value: super::Foundation::TimeSpan) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetPosition)(windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl windows_core::RuntimeType for SystemMediaTransportControlsTimelineProperties {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ISystemMediaTransportControlsTimelineProperties>();
}
unsafe impl windows_core::Interface for SystemMediaTransportControlsTimelineProperties {
    type Vtable = ISystemMediaTransportControlsTimelineProperties_Vtbl;
    const IID: windows_core::GUID = <ISystemMediaTransportControlsTimelineProperties as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for SystemMediaTransportControlsTimelineProperties {
    const NAME: &'static str = "Windows.Media.SystemMediaTransportControlsTimelineProperties";
}
unsafe impl Send for SystemMediaTransportControlsTimelineProperties {}
unsafe impl Sync for SystemMediaTransportControlsTimelineProperties {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct VideoDisplayProperties(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(VideoDisplayProperties, windows_core::IUnknown, windows_core::IInspectable);
impl VideoDisplayProperties {
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
    pub fn Subtitle(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Subtitle)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetSubtitle(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetSubtitle)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Genres(&self) -> windows_core::Result<super::Foundation::Collections::IVector<windows_core::HSTRING>> {
        let this = &windows_core::Interface::cast::<IVideoDisplayProperties2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Genres)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for VideoDisplayProperties {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IVideoDisplayProperties>();
}
unsafe impl windows_core::Interface for VideoDisplayProperties {
    type Vtable = IVideoDisplayProperties_Vtbl;
    const IID: windows_core::GUID = <IVideoDisplayProperties as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for VideoDisplayProperties {
    const NAME: &'static str = "Windows.Media.VideoDisplayProperties";
}
unsafe impl Send for VideoDisplayProperties {}
unsafe impl Sync for VideoDisplayProperties {}
pub struct VideoEffects;
impl VideoEffects {
    pub fn VideoStabilization() -> windows_core::Result<windows_core::HSTRING> {
        Self::IVideoEffectsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).VideoStabilization)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IVideoEffectsStatics<R, F: FnOnce(&IVideoEffectsStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<VideoEffects, IVideoEffectsStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeName for VideoEffects {
    const NAME: &'static str = "Windows.Media.VideoEffects";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct VideoFrame(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(VideoFrame, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(VideoFrame, super::Foundation::IClosable, IMediaFrame);
impl VideoFrame {
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Type(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IMediaFrame>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Type)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn IsReadOnly(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IMediaFrame>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsReadOnly)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetRelativeTime<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Foundation::IReference<super::Foundation::TimeSpan>>,
    {
        let this = &windows_core::Interface::cast::<IMediaFrame>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetRelativeTime)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn RelativeTime(&self) -> windows_core::Result<super::Foundation::IReference<super::Foundation::TimeSpan>> {
        let this = &windows_core::Interface::cast::<IMediaFrame>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RelativeTime)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetSystemRelativeTime<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Foundation::IReference<super::Foundation::TimeSpan>>,
    {
        let this = &windows_core::Interface::cast::<IMediaFrame>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetSystemRelativeTime)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn SystemRelativeTime(&self) -> windows_core::Result<super::Foundation::IReference<super::Foundation::TimeSpan>> {
        let this = &windows_core::Interface::cast::<IMediaFrame>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SystemRelativeTime)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetDuration<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Foundation::IReference<super::Foundation::TimeSpan>>,
    {
        let this = &windows_core::Interface::cast::<IMediaFrame>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetDuration)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn Duration(&self) -> windows_core::Result<super::Foundation::IReference<super::Foundation::TimeSpan>> {
        let this = &windows_core::Interface::cast::<IMediaFrame>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Duration)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetIsDiscontinuous(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IMediaFrame>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetIsDiscontinuous)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsDiscontinuous(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IMediaFrame>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsDiscontinuous)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn ExtendedProperties(&self) -> windows_core::Result<super::Foundation::Collections::IPropertySet> {
        let this = &windows_core::Interface::cast::<IMediaFrame>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ExtendedProperties)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Graphics_Imaging")]
    pub fn SoftwareBitmap(&self) -> windows_core::Result<super::Graphics::Imaging::SoftwareBitmap> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SoftwareBitmap)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CopyToAsync<P0>(&self, frame: P0) -> windows_core::Result<super::Foundation::IAsyncAction>
    where
        P0: windows_core::Param<VideoFrame>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CopyToAsync)(windows_core::Interface::as_raw(this), frame.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Graphics_DirectX_Direct3D11")]
    pub fn Direct3DSurface(&self) -> windows_core::Result<super::Graphics::DirectX::Direct3D11::IDirect3DSurface> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Direct3DSurface)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Graphics_Imaging")]
    pub fn CopyToWithBoundsAsync<P0, P1, P2>(&self, frame: P0, sourcebounds: P1, destinationbounds: P2) -> windows_core::Result<super::Foundation::IAsyncAction>
    where
        P0: windows_core::Param<VideoFrame>,
        P1: windows_core::Param<super::Foundation::IReference<super::Graphics::Imaging::BitmapBounds>>,
        P2: windows_core::Param<super::Foundation::IReference<super::Graphics::Imaging::BitmapBounds>>,
    {
        let this = &windows_core::Interface::cast::<IVideoFrame2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CopyToWithBoundsAsync)(windows_core::Interface::as_raw(this), frame.param().abi(), sourcebounds.param().abi(), destinationbounds.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Graphics_Imaging")]
    pub fn Create(format: super::Graphics::Imaging::BitmapPixelFormat, width: i32, height: i32) -> windows_core::Result<VideoFrame> {
        Self::IVideoFrameFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Create)(windows_core::Interface::as_raw(this), format, width, height, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Graphics_Imaging")]
    pub fn CreateWithAlpha(format: super::Graphics::Imaging::BitmapPixelFormat, width: i32, height: i32, alpha: super::Graphics::Imaging::BitmapAlphaMode) -> windows_core::Result<VideoFrame> {
        Self::IVideoFrameFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateWithAlpha)(windows_core::Interface::as_raw(this), format, width, height, alpha, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Graphics_DirectX")]
    pub fn CreateAsDirect3D11SurfaceBacked(format: super::Graphics::DirectX::DirectXPixelFormat, width: i32, height: i32) -> windows_core::Result<VideoFrame> {
        Self::IVideoFrameStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateAsDirect3D11SurfaceBacked)(windows_core::Interface::as_raw(this), format, width, height, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Graphics_DirectX_Direct3D11")]
    pub fn CreateAsDirect3D11SurfaceBackedWithDevice<P0>(format: super::Graphics::DirectX::DirectXPixelFormat, width: i32, height: i32, device: P0) -> windows_core::Result<VideoFrame>
    where
        P0: windows_core::Param<super::Graphics::DirectX::Direct3D11::IDirect3DDevice>,
    {
        Self::IVideoFrameStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateAsDirect3D11SurfaceBackedWithDevice)(windows_core::Interface::as_raw(this), format, width, height, device.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Graphics_Imaging")]
    pub fn CreateWithSoftwareBitmap<P0>(bitmap: P0) -> windows_core::Result<VideoFrame>
    where
        P0: windows_core::Param<super::Graphics::Imaging::SoftwareBitmap>,
    {
        Self::IVideoFrameStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateWithSoftwareBitmap)(windows_core::Interface::as_raw(this), bitmap.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Graphics_DirectX_Direct3D11")]
    pub fn CreateWithDirect3D11Surface<P0>(surface: P0) -> windows_core::Result<VideoFrame>
    where
        P0: windows_core::Param<super::Graphics::DirectX::Direct3D11::IDirect3DSurface>,
    {
        Self::IVideoFrameStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateWithDirect3D11Surface)(windows_core::Interface::as_raw(this), surface.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IVideoFrameFactory<R, F: FnOnce(&IVideoFrameFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<VideoFrame, IVideoFrameFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IVideoFrameStatics<R, F: FnOnce(&IVideoFrameStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<VideoFrame, IVideoFrameStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for VideoFrame {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IVideoFrame>();
}
unsafe impl windows_core::Interface for VideoFrame {
    type Vtable = IVideoFrame_Vtbl;
    const IID: windows_core::GUID = <IVideoFrame as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for VideoFrame {
    const NAME: &'static str = "Windows.Media.VideoFrame";
}
unsafe impl Send for VideoFrame {}
unsafe impl Sync for VideoFrame {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct AudioBufferAccessMode(pub i32);
impl AudioBufferAccessMode {
    pub const Read: Self = Self(0i32);
    pub const ReadWrite: Self = Self(1i32);
    pub const Write: Self = Self(2i32);
}
impl windows_core::TypeKind for AudioBufferAccessMode {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for AudioBufferAccessMode {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("AudioBufferAccessMode").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for AudioBufferAccessMode {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.AudioBufferAccessMode;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct AudioProcessing(pub i32);
impl AudioProcessing {
    pub const Default: Self = Self(0i32);
    pub const Raw: Self = Self(1i32);
}
impl windows_core::TypeKind for AudioProcessing {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for AudioProcessing {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("AudioProcessing").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for AudioProcessing {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.AudioProcessing;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct MediaPlaybackAutoRepeatMode(pub i32);
impl MediaPlaybackAutoRepeatMode {
    pub const None: Self = Self(0i32);
    pub const Track: Self = Self(1i32);
    pub const List: Self = Self(2i32);
}
impl windows_core::TypeKind for MediaPlaybackAutoRepeatMode {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for MediaPlaybackAutoRepeatMode {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MediaPlaybackAutoRepeatMode").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for MediaPlaybackAutoRepeatMode {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.MediaPlaybackAutoRepeatMode;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct MediaPlaybackStatus(pub i32);
impl MediaPlaybackStatus {
    pub const Closed: Self = Self(0i32);
    pub const Changing: Self = Self(1i32);
    pub const Stopped: Self = Self(2i32);
    pub const Playing: Self = Self(3i32);
    pub const Paused: Self = Self(4i32);
}
impl windows_core::TypeKind for MediaPlaybackStatus {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for MediaPlaybackStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MediaPlaybackStatus").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for MediaPlaybackStatus {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.MediaPlaybackStatus;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct MediaPlaybackType(pub i32);
impl MediaPlaybackType {
    pub const Unknown: Self = Self(0i32);
    pub const Music: Self = Self(1i32);
    pub const Video: Self = Self(2i32);
    pub const Image: Self = Self(3i32);
}
impl windows_core::TypeKind for MediaPlaybackType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for MediaPlaybackType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MediaPlaybackType").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for MediaPlaybackType {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.MediaPlaybackType;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct MediaTimelineControllerState(pub i32);
impl MediaTimelineControllerState {
    pub const Paused: Self = Self(0i32);
    pub const Running: Self = Self(1i32);
    pub const Stalled: Self = Self(2i32);
    pub const Error: Self = Self(3i32);
}
impl windows_core::TypeKind for MediaTimelineControllerState {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for MediaTimelineControllerState {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MediaTimelineControllerState").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for MediaTimelineControllerState {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.MediaTimelineControllerState;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SoundLevel(pub i32);
impl SoundLevel {
    pub const Muted: Self = Self(0i32);
    pub const Low: Self = Self(1i32);
    pub const Full: Self = Self(2i32);
}
impl windows_core::TypeKind for SoundLevel {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SoundLevel {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SoundLevel").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for SoundLevel {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.SoundLevel;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SystemMediaTransportControlsButton(pub i32);
impl SystemMediaTransportControlsButton {
    pub const Play: Self = Self(0i32);
    pub const Pause: Self = Self(1i32);
    pub const Stop: Self = Self(2i32);
    pub const Record: Self = Self(3i32);
    pub const FastForward: Self = Self(4i32);
    pub const Rewind: Self = Self(5i32);
    pub const Next: Self = Self(6i32);
    pub const Previous: Self = Self(7i32);
    pub const ChannelUp: Self = Self(8i32);
    pub const ChannelDown: Self = Self(9i32);
}
impl windows_core::TypeKind for SystemMediaTransportControlsButton {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SystemMediaTransportControlsButton {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SystemMediaTransportControlsButton").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for SystemMediaTransportControlsButton {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.SystemMediaTransportControlsButton;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SystemMediaTransportControlsProperty(pub i32);
impl SystemMediaTransportControlsProperty {
    pub const SoundLevel: Self = Self(0i32);
}
impl windows_core::TypeKind for SystemMediaTransportControlsProperty {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SystemMediaTransportControlsProperty {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SystemMediaTransportControlsProperty").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for SystemMediaTransportControlsProperty {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.SystemMediaTransportControlsProperty;i4)");
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MediaTimeRange {
    pub Start: super::Foundation::TimeSpan,
    pub End: super::Foundation::TimeSpan,
}
impl windows_core::TypeKind for MediaTimeRange {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for MediaTimeRange {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"struct(Windows.Media.MediaTimeRange;struct(Windows.Foundation.TimeSpan;i8);struct(Windows.Foundation.TimeSpan;i8))");
}
impl Default for MediaTimeRange {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "implement")]
core::include!("impl.rs");
