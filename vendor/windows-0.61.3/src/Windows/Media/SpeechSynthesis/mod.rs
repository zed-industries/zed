windows_core::imp::define_interface!(IInstalledVoicesStatic, IInstalledVoicesStatic_Vtbl, 0x7d526ecc_7533_4c3f_85be_888c2baeebdc);
impl windows_core::RuntimeType for IInstalledVoicesStatic {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IInstalledVoicesStatic_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub AllVoices: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DefaultVoice: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IInstalledVoicesStatic2, IInstalledVoicesStatic2_Vtbl, 0x64255f2e_358d_4058_be9a_fd3fcb423530);
impl windows_core::RuntimeType for IInstalledVoicesStatic2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IInstalledVoicesStatic2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub TrySetDefaultVoiceAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "Storage_Streams")]
windows_core::imp::define_interface!(ISpeechSynthesisStream, ISpeechSynthesisStream_Vtbl, 0x83e46e93_244c_4622_ba0b_6229c4d0d65d);
#[cfg(feature = "Storage_Streams")]
impl windows_core::RuntimeType for ISpeechSynthesisStream {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[cfg(feature = "Storage_Streams")]
#[repr(C)]
#[doc(hidden)]
pub struct ISpeechSynthesisStream_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Markers: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISpeechSynthesizer, ISpeechSynthesizer_Vtbl, 0xce9f7c76_97f4_4ced_ad68_d51c458e45c6);
impl windows_core::RuntimeType for ISpeechSynthesizer {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpeechSynthesizer_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(all(feature = "Media_Core", feature = "Storage_Streams"))]
    pub SynthesizeTextToStreamAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Media_Core", feature = "Storage_Streams")))]
    SynthesizeTextToStreamAsync: usize,
    #[cfg(all(feature = "Media_Core", feature = "Storage_Streams"))]
    pub SynthesizeSsmlToStreamAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Media_Core", feature = "Storage_Streams")))]
    SynthesizeSsmlToStreamAsync: usize,
    pub SetVoice: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Voice: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISpeechSynthesizer2, ISpeechSynthesizer2_Vtbl, 0xa7c5ecb2_4339_4d6a_bbf8_c7a4f1544c2e);
impl windows_core::RuntimeType for ISpeechSynthesizer2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpeechSynthesizer2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Options: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISpeechSynthesizerOptions, ISpeechSynthesizerOptions_Vtbl, 0xa0e23871_cc3d_43c9_91b1_ee185324d83d);
impl windows_core::RuntimeType for ISpeechSynthesizerOptions {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpeechSynthesizerOptions_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IncludeWordBoundaryMetadata: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetIncludeWordBoundaryMetadata: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub IncludeSentenceBoundaryMetadata: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetIncludeSentenceBoundaryMetadata: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISpeechSynthesizerOptions2, ISpeechSynthesizerOptions2_Vtbl, 0x1cbef60e_119c_4bed_b118_d250c3a25793);
impl windows_core::RuntimeType for ISpeechSynthesizerOptions2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpeechSynthesizerOptions2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub AudioVolume: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub SetAudioVolume: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub SpeakingRate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub SetSpeakingRate: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub AudioPitch: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub SetAudioPitch: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISpeechSynthesizerOptions3, ISpeechSynthesizerOptions3_Vtbl, 0x401ed877_902c_4814_a582_a5d0c0769fa8);
impl windows_core::RuntimeType for ISpeechSynthesizerOptions3 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpeechSynthesizerOptions3_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub AppendedSilence: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SpeechAppendedSilence) -> windows_core::HRESULT,
    pub SetAppendedSilence: unsafe extern "system" fn(*mut core::ffi::c_void, SpeechAppendedSilence) -> windows_core::HRESULT,
    pub PunctuationSilence: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SpeechPunctuationSilence) -> windows_core::HRESULT,
    pub SetPunctuationSilence: unsafe extern "system" fn(*mut core::ffi::c_void, SpeechPunctuationSilence) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IVoiceInformation, IVoiceInformation_Vtbl, 0xb127d6a4_1291_4604_aa9c_83134083352c);
impl windows_core::RuntimeType for IVoiceInformation {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IVoiceInformation_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub DisplayName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Id: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Language: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Description: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Gender: unsafe extern "system" fn(*mut core::ffi::c_void, *mut VoiceGender) -> windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SpeechAppendedSilence(pub i32);
impl SpeechAppendedSilence {
    pub const Default: Self = Self(0i32);
    pub const Min: Self = Self(1i32);
}
impl windows_core::TypeKind for SpeechAppendedSilence {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for SpeechAppendedSilence {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.SpeechSynthesis.SpeechAppendedSilence;i4)");
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SpeechPunctuationSilence(pub i32);
impl SpeechPunctuationSilence {
    pub const Default: Self = Self(0i32);
    pub const Min: Self = Self(1i32);
}
impl windows_core::TypeKind for SpeechPunctuationSilence {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for SpeechPunctuationSilence {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.SpeechSynthesis.SpeechPunctuationSilence;i4)");
}
#[cfg(all(feature = "Media_Core", feature = "Storage_Streams"))]
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SpeechSynthesisStream(windows_core::IUnknown);
#[cfg(all(feature = "Media_Core", feature = "Storage_Streams"))]
windows_core::imp::interface_hierarchy!(SpeechSynthesisStream, windows_core::IUnknown, windows_core::IInspectable);
#[cfg(all(feature = "Media_Core", feature = "Storage_Streams"))]
windows_core::imp::required_hierarchy!(SpeechSynthesisStream, super::super::Foundation::IClosable, super::super::Storage::Streams::IContentTypeProvider, super::super::Storage::Streams::IInputStream, super::super::Storage::Streams::IOutputStream, super::super::Storage::Streams::IRandomAccessStream, super::super::Storage::Streams::IRandomAccessStreamWithContentType, super::Core::ITimedMetadataTrackProvider);
#[cfg(all(feature = "Media_Core", feature = "Storage_Streams"))]
impl SpeechSynthesisStream {
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn ContentType(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<super::super::Storage::Streams::IContentTypeProvider>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ContentType)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn ReadAsync<P0>(&self, buffer: P0, count: u32, options: super::super::Storage::Streams::InputStreamOptions) -> windows_core::Result<windows_future::IAsyncOperationWithProgress<super::super::Storage::Streams::IBuffer, u32>>
    where
        P0: windows_core::Param<super::super::Storage::Streams::IBuffer>,
    {
        let this = &windows_core::Interface::cast::<super::super::Storage::Streams::IInputStream>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReadAsync)(windows_core::Interface::as_raw(this), buffer.param().abi(), count, options, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn WriteAsync<P0>(&self, buffer: P0) -> windows_core::Result<windows_future::IAsyncOperationWithProgress<u32, u32>>
    where
        P0: windows_core::Param<super::super::Storage::Streams::IBuffer>,
    {
        let this = &windows_core::Interface::cast::<super::super::Storage::Streams::IOutputStream>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).WriteAsync)(windows_core::Interface::as_raw(this), buffer.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn FlushAsync(&self) -> windows_core::Result<windows_future::IAsyncOperation<bool>> {
        let this = &windows_core::Interface::cast::<super::super::Storage::Streams::IOutputStream>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FlushAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Size(&self) -> windows_core::Result<u64> {
        let this = &windows_core::Interface::cast::<super::super::Storage::Streams::IRandomAccessStream>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Size)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetSize(&self, value: u64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::Storage::Streams::IRandomAccessStream>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetSize)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn GetInputStreamAt(&self, position: u64) -> windows_core::Result<super::super::Storage::Streams::IInputStream> {
        let this = &windows_core::Interface::cast::<super::super::Storage::Streams::IRandomAccessStream>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetInputStreamAt)(windows_core::Interface::as_raw(this), position, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetOutputStreamAt(&self, position: u64) -> windows_core::Result<super::super::Storage::Streams::IOutputStream> {
        let this = &windows_core::Interface::cast::<super::super::Storage::Streams::IRandomAccessStream>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetOutputStreamAt)(windows_core::Interface::as_raw(this), position, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Position(&self) -> windows_core::Result<u64> {
        let this = &windows_core::Interface::cast::<super::super::Storage::Streams::IRandomAccessStream>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Position)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Seek(&self, position: u64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::Storage::Streams::IRandomAccessStream>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Seek)(windows_core::Interface::as_raw(this), position).ok() }
    }
    pub fn CloneStream(&self) -> windows_core::Result<super::super::Storage::Streams::IRandomAccessStream> {
        let this = &windows_core::Interface::cast::<super::super::Storage::Streams::IRandomAccessStream>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CloneStream)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CanRead(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<super::super::Storage::Streams::IRandomAccessStream>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CanRead)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn CanWrite(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<super::super::Storage::Streams::IRandomAccessStream>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CanWrite)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Markers(&self) -> windows_core::Result<windows_collections::IVectorView<super::IMediaMarker>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Markers)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn TimedMetadataTracks(&self) -> windows_core::Result<windows_collections::IVectorView<super::Core::TimedMetadataTrack>> {
        let this = &windows_core::Interface::cast::<super::Core::ITimedMetadataTrackProvider>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TimedMetadataTracks)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(all(feature = "Media_Core", feature = "Storage_Streams"))]
impl windows_core::RuntimeType for SpeechSynthesisStream {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ISpeechSynthesisStream>();
}
#[cfg(all(feature = "Media_Core", feature = "Storage_Streams"))]
unsafe impl windows_core::Interface for SpeechSynthesisStream {
    type Vtable = <ISpeechSynthesisStream as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ISpeechSynthesisStream as windows_core::Interface>::IID;
}
#[cfg(all(feature = "Media_Core", feature = "Storage_Streams"))]
impl windows_core::RuntimeName for SpeechSynthesisStream {
    const NAME: &'static str = "Windows.Media.SpeechSynthesis.SpeechSynthesisStream";
}
#[cfg(all(feature = "Media_Core", feature = "Storage_Streams"))]
unsafe impl Send for SpeechSynthesisStream {}
#[cfg(all(feature = "Media_Core", feature = "Storage_Streams"))]
unsafe impl Sync for SpeechSynthesisStream {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SpeechSynthesizer(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(SpeechSynthesizer, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(SpeechSynthesizer, super::super::Foundation::IClosable);
impl SpeechSynthesizer {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<SpeechSynthesizer, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn AllVoices() -> windows_core::Result<windows_collections::IVectorView<VoiceInformation>> {
        Self::IInstalledVoicesStatic(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AllVoices)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn DefaultVoice() -> windows_core::Result<VoiceInformation> {
        Self::IInstalledVoicesStatic(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DefaultVoice)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn TrySetDefaultVoiceAsync<P0>(voice: P0) -> windows_core::Result<windows_future::IAsyncOperation<bool>>
    where
        P0: windows_core::Param<VoiceInformation>,
    {
        Self::IInstalledVoicesStatic2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TrySetDefaultVoiceAsync)(windows_core::Interface::as_raw(this), voice.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(all(feature = "Media_Core", feature = "Storage_Streams"))]
    pub fn SynthesizeTextToStreamAsync(&self, text: &windows_core::HSTRING) -> windows_core::Result<windows_future::IAsyncOperation<SpeechSynthesisStream>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SynthesizeTextToStreamAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(text), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Media_Core", feature = "Storage_Streams"))]
    pub fn SynthesizeSsmlToStreamAsync(&self, ssml: &windows_core::HSTRING) -> windows_core::Result<windows_future::IAsyncOperation<SpeechSynthesisStream>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SynthesizeSsmlToStreamAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(ssml), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetVoice<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<VoiceInformation>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetVoice)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn Voice(&self) -> windows_core::Result<VoiceInformation> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Voice)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Options(&self) -> windows_core::Result<SpeechSynthesizerOptions> {
        let this = &windows_core::Interface::cast::<ISpeechSynthesizer2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Options)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    fn IInstalledVoicesStatic<R, F: FnOnce(&IInstalledVoicesStatic) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<SpeechSynthesizer, IInstalledVoicesStatic> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    fn IInstalledVoicesStatic2<R, F: FnOnce(&IInstalledVoicesStatic2) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<SpeechSynthesizer, IInstalledVoicesStatic2> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for SpeechSynthesizer {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ISpeechSynthesizer>();
}
unsafe impl windows_core::Interface for SpeechSynthesizer {
    type Vtable = <ISpeechSynthesizer as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ISpeechSynthesizer as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for SpeechSynthesizer {
    const NAME: &'static str = "Windows.Media.SpeechSynthesis.SpeechSynthesizer";
}
unsafe impl Send for SpeechSynthesizer {}
unsafe impl Sync for SpeechSynthesizer {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SpeechSynthesizerOptions(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(SpeechSynthesizerOptions, windows_core::IUnknown, windows_core::IInspectable);
impl SpeechSynthesizerOptions {
    pub fn IncludeWordBoundaryMetadata(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IncludeWordBoundaryMetadata)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIncludeWordBoundaryMetadata(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetIncludeWordBoundaryMetadata)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IncludeSentenceBoundaryMetadata(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IncludeSentenceBoundaryMetadata)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIncludeSentenceBoundaryMetadata(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetIncludeSentenceBoundaryMetadata)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn AudioVolume(&self) -> windows_core::Result<f64> {
        let this = &windows_core::Interface::cast::<ISpeechSynthesizerOptions2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AudioVolume)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetAudioVolume(&self, value: f64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<ISpeechSynthesizerOptions2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetAudioVolume)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn SpeakingRate(&self) -> windows_core::Result<f64> {
        let this = &windows_core::Interface::cast::<ISpeechSynthesizerOptions2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SpeakingRate)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetSpeakingRate(&self, value: f64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<ISpeechSynthesizerOptions2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetSpeakingRate)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn AudioPitch(&self) -> windows_core::Result<f64> {
        let this = &windows_core::Interface::cast::<ISpeechSynthesizerOptions2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AudioPitch)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetAudioPitch(&self, value: f64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<ISpeechSynthesizerOptions2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetAudioPitch)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn AppendedSilence(&self) -> windows_core::Result<SpeechAppendedSilence> {
        let this = &windows_core::Interface::cast::<ISpeechSynthesizerOptions3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AppendedSilence)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetAppendedSilence(&self, value: SpeechAppendedSilence) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<ISpeechSynthesizerOptions3>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetAppendedSilence)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn PunctuationSilence(&self) -> windows_core::Result<SpeechPunctuationSilence> {
        let this = &windows_core::Interface::cast::<ISpeechSynthesizerOptions3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PunctuationSilence)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetPunctuationSilence(&self, value: SpeechPunctuationSilence) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<ISpeechSynthesizerOptions3>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetPunctuationSilence)(windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl windows_core::RuntimeType for SpeechSynthesizerOptions {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ISpeechSynthesizerOptions>();
}
unsafe impl windows_core::Interface for SpeechSynthesizerOptions {
    type Vtable = <ISpeechSynthesizerOptions as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ISpeechSynthesizerOptions as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for SpeechSynthesizerOptions {
    const NAME: &'static str = "Windows.Media.SpeechSynthesis.SpeechSynthesizerOptions";
}
unsafe impl Send for SpeechSynthesizerOptions {}
unsafe impl Sync for SpeechSynthesizerOptions {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct VoiceGender(pub i32);
impl VoiceGender {
    pub const Male: Self = Self(0i32);
    pub const Female: Self = Self(1i32);
}
impl windows_core::TypeKind for VoiceGender {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for VoiceGender {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.SpeechSynthesis.VoiceGender;i4)");
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VoiceInformation(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(VoiceInformation, windows_core::IUnknown, windows_core::IInspectable);
impl VoiceInformation {
    pub fn DisplayName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DisplayName)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn Id(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Id)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn Language(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Language)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn Description(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Description)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn Gender(&self) -> windows_core::Result<VoiceGender> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Gender)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for VoiceInformation {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IVoiceInformation>();
}
unsafe impl windows_core::Interface for VoiceInformation {
    type Vtable = <IVoiceInformation as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IVoiceInformation as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for VoiceInformation {
    const NAME: &'static str = "Windows.Media.SpeechSynthesis.VoiceInformation";
}
unsafe impl Send for VoiceInformation {}
unsafe impl Sync for VoiceInformation {}
