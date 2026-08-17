windows_core::imp::define_interface!(IMediaTranscoder, IMediaTranscoder_Vtbl, 0x190c99d2_a0aa_4d34_86bc_eed1b12c2f5b);
impl windows_core::RuntimeType for IMediaTranscoder {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMediaTranscoder_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SetTrimStartTime: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::TimeSpan) -> windows_core::HRESULT,
    pub TrimStartTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::TimeSpan) -> windows_core::HRESULT,
    pub SetTrimStopTime: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::TimeSpan) -> windows_core::HRESULT,
    pub TrimStopTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::TimeSpan) -> windows_core::HRESULT,
    pub SetAlwaysReencode: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub AlwaysReencode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetHardwareAccelerationEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub HardwareAccelerationEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub AddAudioEffect: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub AddAudioEffectWithSettings: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, bool, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    AddAudioEffectWithSettings: usize,
    pub AddVideoEffect: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub AddVideoEffectWithSettings: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, bool, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    AddVideoEffectWithSettings: usize,
    pub ClearEffects: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Media_MediaProperties", feature = "Storage"))]
    pub PrepareFileTranscodeAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Media_MediaProperties", feature = "Storage")))]
    PrepareFileTranscodeAsync: usize,
    #[cfg(all(feature = "Media_MediaProperties", feature = "Storage_Streams"))]
    pub PrepareStreamTranscodeAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Media_MediaProperties", feature = "Storage_Streams")))]
    PrepareStreamTranscodeAsync: usize,
}
windows_core::imp::define_interface!(IMediaTranscoder2, IMediaTranscoder2_Vtbl, 0x40531d74_35e0_4f04_8574_ca8bc4e5a082);
impl windows_core::RuntimeType for IMediaTranscoder2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMediaTranscoder2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(all(feature = "Media_Core", feature = "Media_MediaProperties", feature = "Storage_Streams"))]
    pub PrepareMediaStreamSourceTranscodeAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Media_Core", feature = "Media_MediaProperties", feature = "Storage_Streams")))]
    PrepareMediaStreamSourceTranscodeAsync: usize,
    pub SetVideoProcessingAlgorithm: unsafe extern "system" fn(*mut core::ffi::c_void, MediaVideoProcessingAlgorithm) -> windows_core::HRESULT,
    pub VideoProcessingAlgorithm: unsafe extern "system" fn(*mut core::ffi::c_void, *mut MediaVideoProcessingAlgorithm) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPrepareTranscodeResult, IPrepareTranscodeResult_Vtbl, 0x05f25dce_994f_4a34_9d68_97ccce1730d6);
impl windows_core::RuntimeType for IPrepareTranscodeResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPrepareTranscodeResult_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CanTranscode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub FailureReason: unsafe extern "system" fn(*mut core::ffi::c_void, *mut TranscodeFailureReason) -> windows_core::HRESULT,
    pub TranscodeAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct MediaTranscoder(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(MediaTranscoder, windows_core::IUnknown, windows_core::IInspectable);
impl MediaTranscoder {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<MediaTranscoder, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn SetTrimStartTime(&self, value: super::super::Foundation::TimeSpan) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetTrimStartTime)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn TrimStartTime(&self) -> windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TrimStartTime)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetTrimStopTime(&self, value: super::super::Foundation::TimeSpan) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetTrimStopTime)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn TrimStopTime(&self) -> windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TrimStopTime)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetAlwaysReencode(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetAlwaysReencode)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn AlwaysReencode(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AlwaysReencode)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetHardwareAccelerationEnabled(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetHardwareAccelerationEnabled)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn HardwareAccelerationEnabled(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HardwareAccelerationEnabled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn AddAudioEffect(&self, activatableclassid: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).AddAudioEffect)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(activatableclassid)).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn AddAudioEffectWithSettings<P0>(&self, activatableclassid: &windows_core::HSTRING, effectrequired: bool, configuration: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::Collections::IPropertySet>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).AddAudioEffectWithSettings)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(activatableclassid), effectrequired, configuration.param().abi()).ok() }
    }
    pub fn AddVideoEffect(&self, activatableclassid: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).AddVideoEffect)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(activatableclassid)).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn AddVideoEffectWithSettings<P0>(&self, activatableclassid: &windows_core::HSTRING, effectrequired: bool, configuration: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::Collections::IPropertySet>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).AddVideoEffectWithSettings)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(activatableclassid), effectrequired, configuration.param().abi()).ok() }
    }
    pub fn ClearEffects(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).ClearEffects)(windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(all(feature = "Media_MediaProperties", feature = "Storage"))]
    pub fn PrepareFileTranscodeAsync<P0, P1, P2>(&self, source: P0, destination: P1, profile: P2) -> windows_core::Result<super::super::Foundation::IAsyncOperation<PrepareTranscodeResult>>
    where
        P0: windows_core::Param<super::super::Storage::IStorageFile>,
        P1: windows_core::Param<super::super::Storage::IStorageFile>,
        P2: windows_core::Param<super::MediaProperties::MediaEncodingProfile>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PrepareFileTranscodeAsync)(windows_core::Interface::as_raw(this), source.param().abi(), destination.param().abi(), profile.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Media_MediaProperties", feature = "Storage_Streams"))]
    pub fn PrepareStreamTranscodeAsync<P0, P1, P2>(&self, source: P0, destination: P1, profile: P2) -> windows_core::Result<super::super::Foundation::IAsyncOperation<PrepareTranscodeResult>>
    where
        P0: windows_core::Param<super::super::Storage::Streams::IRandomAccessStream>,
        P1: windows_core::Param<super::super::Storage::Streams::IRandomAccessStream>,
        P2: windows_core::Param<super::MediaProperties::MediaEncodingProfile>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PrepareStreamTranscodeAsync)(windows_core::Interface::as_raw(this), source.param().abi(), destination.param().abi(), profile.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Media_Core", feature = "Media_MediaProperties", feature = "Storage_Streams"))]
    pub fn PrepareMediaStreamSourceTranscodeAsync<P0, P1, P2>(&self, source: P0, destination: P1, profile: P2) -> windows_core::Result<super::super::Foundation::IAsyncOperation<PrepareTranscodeResult>>
    where
        P0: windows_core::Param<super::Core::IMediaSource>,
        P1: windows_core::Param<super::super::Storage::Streams::IRandomAccessStream>,
        P2: windows_core::Param<super::MediaProperties::MediaEncodingProfile>,
    {
        let this = &windows_core::Interface::cast::<IMediaTranscoder2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PrepareMediaStreamSourceTranscodeAsync)(windows_core::Interface::as_raw(this), source.param().abi(), destination.param().abi(), profile.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetVideoProcessingAlgorithm(&self, value: MediaVideoProcessingAlgorithm) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IMediaTranscoder2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetVideoProcessingAlgorithm)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn VideoProcessingAlgorithm(&self) -> windows_core::Result<MediaVideoProcessingAlgorithm> {
        let this = &windows_core::Interface::cast::<IMediaTranscoder2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).VideoProcessingAlgorithm)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for MediaTranscoder {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IMediaTranscoder>();
}
unsafe impl windows_core::Interface for MediaTranscoder {
    type Vtable = IMediaTranscoder_Vtbl;
    const IID: windows_core::GUID = <IMediaTranscoder as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for MediaTranscoder {
    const NAME: &'static str = "Windows.Media.Transcoding.MediaTranscoder";
}
unsafe impl Send for MediaTranscoder {}
unsafe impl Sync for MediaTranscoder {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct PrepareTranscodeResult(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PrepareTranscodeResult, windows_core::IUnknown, windows_core::IInspectable);
impl PrepareTranscodeResult {
    pub fn CanTranscode(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CanTranscode)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn FailureReason(&self) -> windows_core::Result<TranscodeFailureReason> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FailureReason)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn TranscodeAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncActionWithProgress<f64>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TranscodeAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for PrepareTranscodeResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPrepareTranscodeResult>();
}
unsafe impl windows_core::Interface for PrepareTranscodeResult {
    type Vtable = IPrepareTranscodeResult_Vtbl;
    const IID: windows_core::GUID = <IPrepareTranscodeResult as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PrepareTranscodeResult {
    const NAME: &'static str = "Windows.Media.Transcoding.PrepareTranscodeResult";
}
unsafe impl Send for PrepareTranscodeResult {}
unsafe impl Sync for PrepareTranscodeResult {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct MediaVideoProcessingAlgorithm(pub i32);
impl MediaVideoProcessingAlgorithm {
    pub const Default: Self = Self(0i32);
    pub const MrfCrf444: Self = Self(1i32);
}
impl windows_core::TypeKind for MediaVideoProcessingAlgorithm {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for MediaVideoProcessingAlgorithm {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MediaVideoProcessingAlgorithm").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for MediaVideoProcessingAlgorithm {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Transcoding.MediaVideoProcessingAlgorithm;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct TranscodeFailureReason(pub i32);
impl TranscodeFailureReason {
    pub const None: Self = Self(0i32);
    pub const Unknown: Self = Self(1i32);
    pub const InvalidProfile: Self = Self(2i32);
    pub const CodecNotFound: Self = Self(3i32);
}
impl windows_core::TypeKind for TranscodeFailureReason {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for TranscodeFailureReason {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("TranscodeFailureReason").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for TranscodeFailureReason {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Transcoding.TranscodeFailureReason;i4)");
}
