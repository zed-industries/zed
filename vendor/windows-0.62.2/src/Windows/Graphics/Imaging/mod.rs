#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct BitmapAlphaMode(pub i32);
impl BitmapAlphaMode {
    pub const Premultiplied: Self = Self(0i32);
    pub const Straight: Self = Self(1i32);
    pub const Ignore: Self = Self(2i32);
}
impl windows_core::TypeKind for BitmapAlphaMode {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for BitmapAlphaMode {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Graphics.Imaging.BitmapAlphaMode;i4)");
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct BitmapBounds {
    pub X: u32,
    pub Y: u32,
    pub Width: u32,
    pub Height: u32,
}
impl windows_core::TypeKind for BitmapBounds {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for BitmapBounds {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"struct(Windows.Graphics.Imaging.BitmapBounds;u4;u4;u4;u4)");
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BitmapBuffer(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(BitmapBuffer, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(BitmapBuffer, super::super::Foundation::IClosable, super::super::Foundation::IMemoryBuffer);
impl BitmapBuffer {
    pub fn GetPlaneCount(&self) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetPlaneCount)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn GetPlaneDescription(&self, index: i32) -> windows_core::Result<BitmapPlaneDescription> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetPlaneDescription)(windows_core::Interface::as_raw(this), index, &mut result__).map(|| result__)
        }
    }
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn CreateReference(&self) -> windows_core::Result<super::super::Foundation::IMemoryBufferReference> {
        let this = &windows_core::Interface::cast::<super::super::Foundation::IMemoryBuffer>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateReference)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for BitmapBuffer {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IBitmapBuffer>();
}
unsafe impl windows_core::Interface for BitmapBuffer {
    type Vtable = <IBitmapBuffer as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IBitmapBuffer as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for BitmapBuffer {
    const NAME: &'static str = "Windows.Graphics.Imaging.BitmapBuffer";
}
unsafe impl Send for BitmapBuffer {}
unsafe impl Sync for BitmapBuffer {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct BitmapBufferAccessMode(pub i32);
impl BitmapBufferAccessMode {
    pub const Read: Self = Self(0i32);
    pub const ReadWrite: Self = Self(1i32);
    pub const Write: Self = Self(2i32);
}
impl windows_core::TypeKind for BitmapBufferAccessMode {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for BitmapBufferAccessMode {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Graphics.Imaging.BitmapBufferAccessMode;i4)");
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BitmapCodecInformation(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(BitmapCodecInformation, windows_core::IUnknown, windows_core::IInspectable);
impl BitmapCodecInformation {
    pub fn CodecId(&self) -> windows_core::Result<windows_core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CodecId)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn FileExtensions(&self) -> windows_core::Result<windows_collections::IVectorView<windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FileExtensions)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn FriendlyName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FriendlyName)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn MimeTypes(&self) -> windows_core::Result<windows_collections::IVectorView<windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MimeTypes)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for BitmapCodecInformation {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IBitmapCodecInformation>();
}
unsafe impl windows_core::Interface for BitmapCodecInformation {
    type Vtable = <IBitmapCodecInformation as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IBitmapCodecInformation as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for BitmapCodecInformation {
    const NAME: &'static str = "Windows.Graphics.Imaging.BitmapCodecInformation";
}
unsafe impl Send for BitmapCodecInformation {}
unsafe impl Sync for BitmapCodecInformation {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BitmapDecoder(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(BitmapDecoder, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(BitmapDecoder, IBitmapFrame, IBitmapFrameWithSoftwareBitmap);
impl BitmapDecoder {
    pub fn BitmapContainerProperties(&self) -> windows_core::Result<BitmapPropertiesView> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BitmapContainerProperties)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn DecoderInformation(&self) -> windows_core::Result<BitmapCodecInformation> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DecoderInformation)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn FrameCount(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FrameCount)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn GetPreviewAsync(&self) -> windows_core::Result<windows_future::IAsyncOperation<ImageStream>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetPreviewAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetFrameAsync(&self, frameindex: u32) -> windows_core::Result<windows_future::IAsyncOperation<BitmapFrame>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetFrameAsync)(windows_core::Interface::as_raw(this), frameindex, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn BmpDecoderId() -> windows_core::Result<windows_core::GUID> {
        Self::IBitmapDecoderStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BmpDecoderId)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn JpegDecoderId() -> windows_core::Result<windows_core::GUID> {
        Self::IBitmapDecoderStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).JpegDecoderId)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn PngDecoderId() -> windows_core::Result<windows_core::GUID> {
        Self::IBitmapDecoderStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PngDecoderId)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn TiffDecoderId() -> windows_core::Result<windows_core::GUID> {
        Self::IBitmapDecoderStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TiffDecoderId)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn GifDecoderId() -> windows_core::Result<windows_core::GUID> {
        Self::IBitmapDecoderStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GifDecoderId)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn JpegXRDecoderId() -> windows_core::Result<windows_core::GUID> {
        Self::IBitmapDecoderStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).JpegXRDecoderId)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn IcoDecoderId() -> windows_core::Result<windows_core::GUID> {
        Self::IBitmapDecoderStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IcoDecoderId)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn GetDecoderInformationEnumerator() -> windows_core::Result<windows_collections::IVectorView<BitmapCodecInformation>> {
        Self::IBitmapDecoderStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDecoderInformationEnumerator)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn CreateAsync<P0>(stream: P0) -> windows_core::Result<windows_future::IAsyncOperation<BitmapDecoder>>
    where
        P0: windows_core::Param<super::super::Storage::Streams::IRandomAccessStream>,
    {
        Self::IBitmapDecoderStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateAsync)(windows_core::Interface::as_raw(this), stream.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn CreateWithIdAsync<P1>(decoderid: windows_core::GUID, stream: P1) -> windows_core::Result<windows_future::IAsyncOperation<BitmapDecoder>>
    where
        P1: windows_core::Param<super::super::Storage::Streams::IRandomAccessStream>,
    {
        Self::IBitmapDecoderStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateWithIdAsync)(windows_core::Interface::as_raw(this), decoderid, stream.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn HeifDecoderId() -> windows_core::Result<windows_core::GUID> {
        Self::IBitmapDecoderStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HeifDecoderId)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn WebpDecoderId() -> windows_core::Result<windows_core::GUID> {
        Self::IBitmapDecoderStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).WebpDecoderId)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn GetThumbnailAsync(&self) -> windows_core::Result<windows_future::IAsyncOperation<ImageStream>> {
        let this = &windows_core::Interface::cast::<IBitmapFrame>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetThumbnailAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn BitmapProperties(&self) -> windows_core::Result<BitmapPropertiesView> {
        let this = &windows_core::Interface::cast::<IBitmapFrame>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BitmapProperties)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn BitmapPixelFormat(&self) -> windows_core::Result<BitmapPixelFormat> {
        let this = &windows_core::Interface::cast::<IBitmapFrame>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BitmapPixelFormat)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn BitmapAlphaMode(&self) -> windows_core::Result<BitmapAlphaMode> {
        let this = &windows_core::Interface::cast::<IBitmapFrame>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BitmapAlphaMode)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn DpiX(&self) -> windows_core::Result<f64> {
        let this = &windows_core::Interface::cast::<IBitmapFrame>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DpiX)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn DpiY(&self) -> windows_core::Result<f64> {
        let this = &windows_core::Interface::cast::<IBitmapFrame>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DpiY)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PixelWidth(&self) -> windows_core::Result<u32> {
        let this = &windows_core::Interface::cast::<IBitmapFrame>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PixelWidth)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PixelHeight(&self) -> windows_core::Result<u32> {
        let this = &windows_core::Interface::cast::<IBitmapFrame>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PixelHeight)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn OrientedPixelWidth(&self) -> windows_core::Result<u32> {
        let this = &windows_core::Interface::cast::<IBitmapFrame>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OrientedPixelWidth)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn OrientedPixelHeight(&self) -> windows_core::Result<u32> {
        let this = &windows_core::Interface::cast::<IBitmapFrame>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OrientedPixelHeight)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn GetPixelDataAsync(&self) -> windows_core::Result<windows_future::IAsyncOperation<PixelDataProvider>> {
        let this = &windows_core::Interface::cast::<IBitmapFrame>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetPixelDataAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetPixelDataTransformedAsync<P2>(&self, pixelformat: BitmapPixelFormat, alphamode: BitmapAlphaMode, transform: P2, exiforientationmode: ExifOrientationMode, colormanagementmode: ColorManagementMode) -> windows_core::Result<windows_future::IAsyncOperation<PixelDataProvider>>
    where
        P2: windows_core::Param<BitmapTransform>,
    {
        let this = &windows_core::Interface::cast::<IBitmapFrame>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetPixelDataTransformedAsync)(windows_core::Interface::as_raw(this), pixelformat, alphamode, transform.param().abi(), exiforientationmode, colormanagementmode, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetSoftwareBitmapAsync(&self) -> windows_core::Result<windows_future::IAsyncOperation<SoftwareBitmap>> {
        let this = &windows_core::Interface::cast::<IBitmapFrameWithSoftwareBitmap>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetSoftwareBitmapAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetSoftwareBitmapConvertedAsync(&self, pixelformat: BitmapPixelFormat, alphamode: BitmapAlphaMode) -> windows_core::Result<windows_future::IAsyncOperation<SoftwareBitmap>> {
        let this = &windows_core::Interface::cast::<IBitmapFrameWithSoftwareBitmap>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetSoftwareBitmapConvertedAsync)(windows_core::Interface::as_raw(this), pixelformat, alphamode, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetSoftwareBitmapTransformedAsync<P2>(&self, pixelformat: BitmapPixelFormat, alphamode: BitmapAlphaMode, transform: P2, exiforientationmode: ExifOrientationMode, colormanagementmode: ColorManagementMode) -> windows_core::Result<windows_future::IAsyncOperation<SoftwareBitmap>>
    where
        P2: windows_core::Param<BitmapTransform>,
    {
        let this = &windows_core::Interface::cast::<IBitmapFrameWithSoftwareBitmap>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetSoftwareBitmapTransformedAsync)(windows_core::Interface::as_raw(this), pixelformat, alphamode, transform.param().abi(), exiforientationmode, colormanagementmode, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    fn IBitmapDecoderStatics<R, F: FnOnce(&IBitmapDecoderStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<BitmapDecoder, IBitmapDecoderStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    fn IBitmapDecoderStatics2<R, F: FnOnce(&IBitmapDecoderStatics2) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<BitmapDecoder, IBitmapDecoderStatics2> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for BitmapDecoder {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IBitmapDecoder>();
}
unsafe impl windows_core::Interface for BitmapDecoder {
    type Vtable = <IBitmapDecoder as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IBitmapDecoder as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for BitmapDecoder {
    const NAME: &'static str = "Windows.Graphics.Imaging.BitmapDecoder";
}
unsafe impl Send for BitmapDecoder {}
unsafe impl Sync for BitmapDecoder {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BitmapEncoder(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(BitmapEncoder, windows_core::IUnknown, windows_core::IInspectable);
impl BitmapEncoder {
    pub fn EncoderInformation(&self) -> windows_core::Result<BitmapCodecInformation> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).EncoderInformation)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn BitmapProperties(&self) -> windows_core::Result<BitmapProperties> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BitmapProperties)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn BitmapContainerProperties(&self) -> windows_core::Result<BitmapProperties> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BitmapContainerProperties)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn IsThumbnailGenerated(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsThumbnailGenerated)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsThumbnailGenerated(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetIsThumbnailGenerated)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn GeneratedThumbnailWidth(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GeneratedThumbnailWidth)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetGeneratedThumbnailWidth(&self, value: u32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetGeneratedThumbnailWidth)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn GeneratedThumbnailHeight(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GeneratedThumbnailHeight)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetGeneratedThumbnailHeight(&self, value: u32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetGeneratedThumbnailHeight)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn BitmapTransform(&self) -> windows_core::Result<BitmapTransform> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BitmapTransform)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetPixelData(&self, pixelformat: BitmapPixelFormat, alphamode: BitmapAlphaMode, width: u32, height: u32, dpix: f64, dpiy: f64, pixels: &[u8]) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetPixelData)(windows_core::Interface::as_raw(this), pixelformat, alphamode, width, height, dpix, dpiy, pixels.len().try_into().unwrap(), pixels.as_ptr()).ok() }
    }
    pub fn GoToNextFrameAsync(&self) -> windows_core::Result<windows_future::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GoToNextFrameAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GoToNextFrameWithEncodingOptionsAsync<P0>(&self, encodingoptions: P0) -> windows_core::Result<windows_future::IAsyncAction>
    where
        P0: windows_core::Param<windows_collections::IIterable<windows_collections::IKeyValuePair<windows_core::HSTRING, BitmapTypedValue>>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GoToNextFrameWithEncodingOptionsAsync)(windows_core::Interface::as_raw(this), encodingoptions.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn FlushAsync(&self) -> windows_core::Result<windows_future::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FlushAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn BmpEncoderId() -> windows_core::Result<windows_core::GUID> {
        Self::IBitmapEncoderStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BmpEncoderId)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn JpegEncoderId() -> windows_core::Result<windows_core::GUID> {
        Self::IBitmapEncoderStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).JpegEncoderId)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn PngEncoderId() -> windows_core::Result<windows_core::GUID> {
        Self::IBitmapEncoderStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PngEncoderId)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn TiffEncoderId() -> windows_core::Result<windows_core::GUID> {
        Self::IBitmapEncoderStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TiffEncoderId)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn GifEncoderId() -> windows_core::Result<windows_core::GUID> {
        Self::IBitmapEncoderStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GifEncoderId)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn JpegXREncoderId() -> windows_core::Result<windows_core::GUID> {
        Self::IBitmapEncoderStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).JpegXREncoderId)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn GetEncoderInformationEnumerator() -> windows_core::Result<windows_collections::IVectorView<BitmapCodecInformation>> {
        Self::IBitmapEncoderStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetEncoderInformationEnumerator)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn CreateAsync<P1>(encoderid: windows_core::GUID, stream: P1) -> windows_core::Result<windows_future::IAsyncOperation<BitmapEncoder>>
    where
        P1: windows_core::Param<super::super::Storage::Streams::IRandomAccessStream>,
    {
        Self::IBitmapEncoderStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateAsync)(windows_core::Interface::as_raw(this), encoderid, stream.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn CreateWithEncodingOptionsAsync<P1, P2>(encoderid: windows_core::GUID, stream: P1, encodingoptions: P2) -> windows_core::Result<windows_future::IAsyncOperation<BitmapEncoder>>
    where
        P1: windows_core::Param<super::super::Storage::Streams::IRandomAccessStream>,
        P2: windows_core::Param<windows_collections::IIterable<windows_collections::IKeyValuePair<windows_core::HSTRING, BitmapTypedValue>>>,
    {
        Self::IBitmapEncoderStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateWithEncodingOptionsAsync)(windows_core::Interface::as_raw(this), encoderid, stream.param().abi(), encodingoptions.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn CreateForTranscodingAsync<P0, P1>(stream: P0, bitmapdecoder: P1) -> windows_core::Result<windows_future::IAsyncOperation<BitmapEncoder>>
    where
        P0: windows_core::Param<super::super::Storage::Streams::IRandomAccessStream>,
        P1: windows_core::Param<BitmapDecoder>,
    {
        Self::IBitmapEncoderStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateForTranscodingAsync)(windows_core::Interface::as_raw(this), stream.param().abi(), bitmapdecoder.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateForInPlacePropertyEncodingAsync<P0>(bitmapdecoder: P0) -> windows_core::Result<windows_future::IAsyncOperation<BitmapEncoder>>
    where
        P0: windows_core::Param<BitmapDecoder>,
    {
        Self::IBitmapEncoderStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateForInPlacePropertyEncodingAsync)(windows_core::Interface::as_raw(this), bitmapdecoder.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn HeifEncoderId() -> windows_core::Result<windows_core::GUID> {
        Self::IBitmapEncoderStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HeifEncoderId)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn SetSoftwareBitmap<P0>(&self, bitmap: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<SoftwareBitmap>,
    {
        let this = &windows_core::Interface::cast::<IBitmapEncoderWithSoftwareBitmap>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetSoftwareBitmap)(windows_core::Interface::as_raw(this), bitmap.param().abi()).ok() }
    }
    fn IBitmapEncoderStatics<R, F: FnOnce(&IBitmapEncoderStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<BitmapEncoder, IBitmapEncoderStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    fn IBitmapEncoderStatics2<R, F: FnOnce(&IBitmapEncoderStatics2) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<BitmapEncoder, IBitmapEncoderStatics2> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for BitmapEncoder {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IBitmapEncoder>();
}
unsafe impl windows_core::Interface for BitmapEncoder {
    type Vtable = <IBitmapEncoder as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IBitmapEncoder as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for BitmapEncoder {
    const NAME: &'static str = "Windows.Graphics.Imaging.BitmapEncoder";
}
unsafe impl Send for BitmapEncoder {}
unsafe impl Sync for BitmapEncoder {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct BitmapFlip(pub i32);
impl BitmapFlip {
    pub const None: Self = Self(0i32);
    pub const Horizontal: Self = Self(1i32);
    pub const Vertical: Self = Self(2i32);
}
impl windows_core::TypeKind for BitmapFlip {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for BitmapFlip {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Graphics.Imaging.BitmapFlip;i4)");
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BitmapFrame(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(BitmapFrame, windows_core::IUnknown, windows_core::IInspectable, IBitmapFrame);
windows_core::imp::required_hierarchy!(BitmapFrame, IBitmapFrameWithSoftwareBitmap);
impl BitmapFrame {
    #[cfg(feature = "Storage_Streams")]
    pub fn GetThumbnailAsync(&self) -> windows_core::Result<windows_future::IAsyncOperation<ImageStream>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetThumbnailAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn BitmapProperties(&self) -> windows_core::Result<BitmapPropertiesView> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BitmapProperties)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn BitmapPixelFormat(&self) -> windows_core::Result<BitmapPixelFormat> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BitmapPixelFormat)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn BitmapAlphaMode(&self) -> windows_core::Result<BitmapAlphaMode> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BitmapAlphaMode)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn DpiX(&self) -> windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DpiX)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn DpiY(&self) -> windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DpiY)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PixelWidth(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PixelWidth)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PixelHeight(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PixelHeight)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn OrientedPixelWidth(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OrientedPixelWidth)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn OrientedPixelHeight(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OrientedPixelHeight)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn GetPixelDataAsync(&self) -> windows_core::Result<windows_future::IAsyncOperation<PixelDataProvider>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetPixelDataAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetPixelDataTransformedAsync<P2>(&self, pixelformat: BitmapPixelFormat, alphamode: BitmapAlphaMode, transform: P2, exiforientationmode: ExifOrientationMode, colormanagementmode: ColorManagementMode) -> windows_core::Result<windows_future::IAsyncOperation<PixelDataProvider>>
    where
        P2: windows_core::Param<BitmapTransform>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetPixelDataTransformedAsync)(windows_core::Interface::as_raw(this), pixelformat, alphamode, transform.param().abi(), exiforientationmode, colormanagementmode, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetSoftwareBitmapAsync(&self) -> windows_core::Result<windows_future::IAsyncOperation<SoftwareBitmap>> {
        let this = &windows_core::Interface::cast::<IBitmapFrameWithSoftwareBitmap>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetSoftwareBitmapAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetSoftwareBitmapConvertedAsync(&self, pixelformat: BitmapPixelFormat, alphamode: BitmapAlphaMode) -> windows_core::Result<windows_future::IAsyncOperation<SoftwareBitmap>> {
        let this = &windows_core::Interface::cast::<IBitmapFrameWithSoftwareBitmap>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetSoftwareBitmapConvertedAsync)(windows_core::Interface::as_raw(this), pixelformat, alphamode, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetSoftwareBitmapTransformedAsync<P2>(&self, pixelformat: BitmapPixelFormat, alphamode: BitmapAlphaMode, transform: P2, exiforientationmode: ExifOrientationMode, colormanagementmode: ColorManagementMode) -> windows_core::Result<windows_future::IAsyncOperation<SoftwareBitmap>>
    where
        P2: windows_core::Param<BitmapTransform>,
    {
        let this = &windows_core::Interface::cast::<IBitmapFrameWithSoftwareBitmap>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetSoftwareBitmapTransformedAsync)(windows_core::Interface::as_raw(this), pixelformat, alphamode, transform.param().abi(), exiforientationmode, colormanagementmode, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for BitmapFrame {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IBitmapFrame>();
}
unsafe impl windows_core::Interface for BitmapFrame {
    type Vtable = <IBitmapFrame as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IBitmapFrame as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for BitmapFrame {
    const NAME: &'static str = "Windows.Graphics.Imaging.BitmapFrame";
}
unsafe impl Send for BitmapFrame {}
unsafe impl Sync for BitmapFrame {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct BitmapInterpolationMode(pub i32);
impl BitmapInterpolationMode {
    pub const NearestNeighbor: Self = Self(0i32);
    pub const Linear: Self = Self(1i32);
    pub const Cubic: Self = Self(2i32);
    pub const Fant: Self = Self(3i32);
}
impl windows_core::TypeKind for BitmapInterpolationMode {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for BitmapInterpolationMode {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Graphics.Imaging.BitmapInterpolationMode;i4)");
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct BitmapPixelFormat(pub i32);
impl BitmapPixelFormat {
    pub const Unknown: Self = Self(0i32);
    pub const Rgba16: Self = Self(12i32);
    pub const Rgba8: Self = Self(30i32);
    pub const Gray16: Self = Self(57i32);
    pub const Gray8: Self = Self(62i32);
    pub const Bgra8: Self = Self(87i32);
    pub const Nv12: Self = Self(103i32);
    pub const P010: Self = Self(104i32);
    pub const Yuy2: Self = Self(107i32);
}
impl windows_core::TypeKind for BitmapPixelFormat {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for BitmapPixelFormat {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Graphics.Imaging.BitmapPixelFormat;i4)");
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct BitmapPlaneDescription {
    pub StartIndex: i32,
    pub Width: i32,
    pub Height: i32,
    pub Stride: i32,
}
impl windows_core::TypeKind for BitmapPlaneDescription {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for BitmapPlaneDescription {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"struct(Windows.Graphics.Imaging.BitmapPlaneDescription;i4;i4;i4;i4)");
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BitmapProperties(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(BitmapProperties, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(BitmapProperties, IBitmapPropertiesView);
impl BitmapProperties {
    pub fn SetPropertiesAsync<P0>(&self, propertiestoset: P0) -> windows_core::Result<windows_future::IAsyncAction>
    where
        P0: windows_core::Param<windows_collections::IIterable<windows_collections::IKeyValuePair<windows_core::HSTRING, BitmapTypedValue>>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SetPropertiesAsync)(windows_core::Interface::as_raw(this), propertiestoset.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetPropertiesAsync<P0>(&self, propertiestoretrieve: P0) -> windows_core::Result<windows_future::IAsyncOperation<BitmapPropertySet>>
    where
        P0: windows_core::Param<windows_collections::IIterable<windows_core::HSTRING>>,
    {
        let this = &windows_core::Interface::cast::<IBitmapPropertiesView>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetPropertiesAsync)(windows_core::Interface::as_raw(this), propertiestoretrieve.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for BitmapProperties {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IBitmapProperties>();
}
unsafe impl windows_core::Interface for BitmapProperties {
    type Vtable = <IBitmapProperties as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IBitmapProperties as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for BitmapProperties {
    const NAME: &'static str = "Windows.Graphics.Imaging.BitmapProperties";
}
unsafe impl Send for BitmapProperties {}
unsafe impl Sync for BitmapProperties {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BitmapPropertiesView(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(BitmapPropertiesView, windows_core::IUnknown, windows_core::IInspectable, IBitmapPropertiesView);
impl BitmapPropertiesView {
    pub fn GetPropertiesAsync<P0>(&self, propertiestoretrieve: P0) -> windows_core::Result<windows_future::IAsyncOperation<BitmapPropertySet>>
    where
        P0: windows_core::Param<windows_collections::IIterable<windows_core::HSTRING>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetPropertiesAsync)(windows_core::Interface::as_raw(this), propertiestoretrieve.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for BitmapPropertiesView {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IBitmapPropertiesView>();
}
unsafe impl windows_core::Interface for BitmapPropertiesView {
    type Vtable = <IBitmapPropertiesView as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IBitmapPropertiesView as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for BitmapPropertiesView {
    const NAME: &'static str = "Windows.Graphics.Imaging.BitmapPropertiesView";
}
unsafe impl Send for BitmapPropertiesView {}
unsafe impl Sync for BitmapPropertiesView {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BitmapPropertySet(windows_core::IUnknown);
windows_core::imp::interface_hierarchy ! ( BitmapPropertySet , windows_core::IUnknown , windows_core::IInspectable , windows_collections:: IMap < windows_core::HSTRING , BitmapTypedValue > );
windows_core::imp::required_hierarchy!(BitmapPropertySet, windows_collections::IIterable<windows_collections::IKeyValuePair<windows_core::HSTRING, BitmapTypedValue>>);
impl BitmapPropertySet {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<BitmapPropertySet, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn First(&self) -> windows_core::Result<windows_collections::IIterator<windows_collections::IKeyValuePair<windows_core::HSTRING, BitmapTypedValue>>> {
        let this = &windows_core::Interface::cast::<windows_collections::IIterable<windows_collections::IKeyValuePair<windows_core::HSTRING, BitmapTypedValue>>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).First)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Lookup(&self, key: &windows_core::HSTRING) -> windows_core::Result<BitmapTypedValue> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Lookup)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(key), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Size(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Size)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn HasKey(&self, key: &windows_core::HSTRING) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HasKey)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(key), &mut result__).map(|| result__)
        }
    }
    pub fn GetView(&self) -> windows_core::Result<windows_collections::IMapView<windows_core::HSTRING, BitmapTypedValue>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetView)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Insert<P1>(&self, key: &windows_core::HSTRING, value: P1) -> windows_core::Result<bool>
    where
        P1: windows_core::Param<BitmapTypedValue>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Insert)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(key), value.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn Remove(&self, key: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Remove)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(key)).ok() }
    }
    pub fn Clear(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Clear)(windows_core::Interface::as_raw(this)).ok() }
    }
}
impl windows_core::RuntimeType for BitmapPropertySet {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, windows_collections::IMap<windows_core::HSTRING, BitmapTypedValue>>();
}
unsafe impl windows_core::Interface for BitmapPropertySet {
    type Vtable = <windows_collections::IMap<windows_core::HSTRING, BitmapTypedValue> as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <windows_collections::IMap<windows_core::HSTRING, BitmapTypedValue> as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for BitmapPropertySet {
    const NAME: &'static str = "Windows.Graphics.Imaging.BitmapPropertySet";
}
unsafe impl Send for BitmapPropertySet {}
unsafe impl Sync for BitmapPropertySet {}
impl IntoIterator for BitmapPropertySet {
    type Item = windows_collections::IKeyValuePair<windows_core::HSTRING, BitmapTypedValue>;
    type IntoIter = windows_collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        IntoIterator::into_iter(&self)
    }
}
impl IntoIterator for &BitmapPropertySet {
    type Item = windows_collections::IKeyValuePair<windows_core::HSTRING, BitmapTypedValue>;
    type IntoIter = windows_collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.First().unwrap()
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct BitmapRotation(pub i32);
impl BitmapRotation {
    pub const None: Self = Self(0i32);
    pub const Clockwise90Degrees: Self = Self(1i32);
    pub const Clockwise180Degrees: Self = Self(2i32);
    pub const Clockwise270Degrees: Self = Self(3i32);
}
impl windows_core::TypeKind for BitmapRotation {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for BitmapRotation {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Graphics.Imaging.BitmapRotation;i4)");
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct BitmapSize {
    pub Width: u32,
    pub Height: u32,
}
impl windows_core::TypeKind for BitmapSize {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for BitmapSize {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"struct(Windows.Graphics.Imaging.BitmapSize;u4;u4)");
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BitmapTransform(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(BitmapTransform, windows_core::IUnknown, windows_core::IInspectable);
impl BitmapTransform {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<BitmapTransform, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn ScaledWidth(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ScaledWidth)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetScaledWidth(&self, value: u32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetScaledWidth)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ScaledHeight(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ScaledHeight)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetScaledHeight(&self, value: u32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetScaledHeight)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn InterpolationMode(&self) -> windows_core::Result<BitmapInterpolationMode> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).InterpolationMode)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetInterpolationMode(&self, value: BitmapInterpolationMode) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetInterpolationMode)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Flip(&self) -> windows_core::Result<BitmapFlip> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Flip)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetFlip(&self, value: BitmapFlip) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetFlip)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Rotation(&self) -> windows_core::Result<BitmapRotation> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Rotation)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetRotation(&self, value: BitmapRotation) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetRotation)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Bounds(&self) -> windows_core::Result<BitmapBounds> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Bounds)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetBounds(&self, value: BitmapBounds) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetBounds)(windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl windows_core::RuntimeType for BitmapTransform {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IBitmapTransform>();
}
unsafe impl windows_core::Interface for BitmapTransform {
    type Vtable = <IBitmapTransform as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IBitmapTransform as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for BitmapTransform {
    const NAME: &'static str = "Windows.Graphics.Imaging.BitmapTransform";
}
unsafe impl Send for BitmapTransform {}
unsafe impl Sync for BitmapTransform {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BitmapTypedValue(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(BitmapTypedValue, windows_core::IUnknown, windows_core::IInspectable);
impl BitmapTypedValue {
    pub fn Value(&self) -> windows_core::Result<windows_core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Value)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Type(&self) -> windows_core::Result<super::super::Foundation::PropertyType> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Type)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Create<P0>(value: P0, r#type: super::super::Foundation::PropertyType) -> windows_core::Result<BitmapTypedValue>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        Self::IBitmapTypedValueFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Create)(windows_core::Interface::as_raw(this), value.param().abi(), r#type, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    fn IBitmapTypedValueFactory<R, F: FnOnce(&IBitmapTypedValueFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<BitmapTypedValue, IBitmapTypedValueFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for BitmapTypedValue {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IBitmapTypedValue>();
}
unsafe impl windows_core::Interface for BitmapTypedValue {
    type Vtable = <IBitmapTypedValue as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IBitmapTypedValue as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for BitmapTypedValue {
    const NAME: &'static str = "Windows.Graphics.Imaging.BitmapTypedValue";
}
unsafe impl Send for BitmapTypedValue {}
unsafe impl Sync for BitmapTypedValue {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ColorManagementMode(pub i32);
impl ColorManagementMode {
    pub const DoNotColorManage: Self = Self(0i32);
    pub const ColorManageToSRgb: Self = Self(1i32);
}
impl windows_core::TypeKind for ColorManagementMode {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for ColorManagementMode {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Graphics.Imaging.ColorManagementMode;i4)");
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ExifOrientationMode(pub i32);
impl ExifOrientationMode {
    pub const IgnoreExifOrientation: Self = Self(0i32);
    pub const RespectExifOrientation: Self = Self(1i32);
}
impl windows_core::TypeKind for ExifOrientationMode {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for ExifOrientationMode {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Graphics.Imaging.ExifOrientationMode;i4)");
}
windows_core::imp::define_interface!(IBitmapBuffer, IBitmapBuffer_Vtbl, 0xa53e04c4_399c_438c_b28f_a63a6b83d1a1);
impl windows_core::RuntimeType for IBitmapBuffer {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IBitmapBuffer_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetPlaneCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub GetPlaneDescription: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut BitmapPlaneDescription) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IBitmapCodecInformation, IBitmapCodecInformation_Vtbl, 0x400caaf2_c4b0_4392_a3b0_6f6f9ba95cb4);
impl windows_core::RuntimeType for IBitmapCodecInformation {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IBitmapCodecInformation_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CodecId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub FileExtensions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FriendlyName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub MimeTypes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IBitmapDecoder, IBitmapDecoder_Vtbl, 0xacef22ba_1d74_4c91_9dfc_9620745233e6);
impl windows_core::RuntimeType for IBitmapDecoder {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IBitmapDecoder_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub BitmapContainerProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DecoderInformation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FrameCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub GetPreviewAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    GetPreviewAsync: usize,
    pub GetFrameAsync: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IBitmapDecoderStatics, IBitmapDecoderStatics_Vtbl, 0x438ccb26_bcef_4e95_bad6_23a822e58d01);
impl windows_core::RuntimeType for IBitmapDecoderStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IBitmapDecoderStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub BmpDecoderId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub JpegDecoderId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub PngDecoderId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub TiffDecoderId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub GifDecoderId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub JpegXRDecoderId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub IcoDecoderId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub GetDecoderInformationEnumerator: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub CreateAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CreateAsync: usize,
    #[cfg(feature = "Storage_Streams")]
    pub CreateWithIdAsync: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CreateWithIdAsync: usize,
}
windows_core::imp::define_interface!(IBitmapDecoderStatics2, IBitmapDecoderStatics2_Vtbl, 0x50ba68ea_99a1_40c4_80d9_aef0dafa6c3f);
impl windows_core::RuntimeType for IBitmapDecoderStatics2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IBitmapDecoderStatics2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub HeifDecoderId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub WebpDecoderId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IBitmapEncoder, IBitmapEncoder_Vtbl, 0x2bc468e3_e1f8_4b54_95e8_32919551ce62);
impl windows_core::RuntimeType for IBitmapEncoder {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IBitmapEncoder_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub EncoderInformation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub BitmapProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub BitmapContainerProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsThumbnailGenerated: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetIsThumbnailGenerated: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub GeneratedThumbnailWidth: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetGeneratedThumbnailWidth: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GeneratedThumbnailHeight: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetGeneratedThumbnailHeight: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub BitmapTransform: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetPixelData: unsafe extern "system" fn(*mut core::ffi::c_void, BitmapPixelFormat, BitmapAlphaMode, u32, u32, f64, f64, u32, *const u8) -> windows_core::HRESULT,
    pub GoToNextFrameAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GoToNextFrameWithEncodingOptionsAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FlushAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IBitmapEncoderStatics, IBitmapEncoderStatics_Vtbl, 0xa74356a7_a4e4_4eb9_8e40_564de7e1ccb2);
impl windows_core::RuntimeType for IBitmapEncoderStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IBitmapEncoderStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub BmpEncoderId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub JpegEncoderId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub PngEncoderId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub TiffEncoderId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub GifEncoderId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub JpegXREncoderId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub GetEncoderInformationEnumerator: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub CreateAsync: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CreateAsync: usize,
    #[cfg(feature = "Storage_Streams")]
    pub CreateWithEncodingOptionsAsync: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CreateWithEncodingOptionsAsync: usize,
    #[cfg(feature = "Storage_Streams")]
    pub CreateForTranscodingAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CreateForTranscodingAsync: usize,
    pub CreateForInPlacePropertyEncodingAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IBitmapEncoderStatics2, IBitmapEncoderStatics2_Vtbl, 0x33cbc259_fe31_41b1_b812_086d21e87e16);
impl windows_core::RuntimeType for IBitmapEncoderStatics2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IBitmapEncoderStatics2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub HeifEncoderId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IBitmapEncoderWithSoftwareBitmap, IBitmapEncoderWithSoftwareBitmap_Vtbl, 0x686cd241_4330_4c77_ace4_0334968b1768);
impl windows_core::RuntimeType for IBitmapEncoderWithSoftwareBitmap {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IBitmapEncoderWithSoftwareBitmap_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SetSoftwareBitmap: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IBitmapFrame, IBitmapFrame_Vtbl, 0x72a49a1c_8081_438d_91bc_94ecfc8185c6);
impl windows_core::RuntimeType for IBitmapFrame {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(IBitmapFrame, windows_core::IUnknown, windows_core::IInspectable);
impl IBitmapFrame {
    #[cfg(feature = "Storage_Streams")]
    pub fn GetThumbnailAsync(&self) -> windows_core::Result<windows_future::IAsyncOperation<ImageStream>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetThumbnailAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn BitmapProperties(&self) -> windows_core::Result<BitmapPropertiesView> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BitmapProperties)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn BitmapPixelFormat(&self) -> windows_core::Result<BitmapPixelFormat> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BitmapPixelFormat)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn BitmapAlphaMode(&self) -> windows_core::Result<BitmapAlphaMode> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BitmapAlphaMode)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn DpiX(&self) -> windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DpiX)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn DpiY(&self) -> windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DpiY)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PixelWidth(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PixelWidth)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PixelHeight(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PixelHeight)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn OrientedPixelWidth(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OrientedPixelWidth)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn OrientedPixelHeight(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OrientedPixelHeight)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn GetPixelDataAsync(&self) -> windows_core::Result<windows_future::IAsyncOperation<PixelDataProvider>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetPixelDataAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetPixelDataTransformedAsync<P2>(&self, pixelformat: BitmapPixelFormat, alphamode: BitmapAlphaMode, transform: P2, exiforientationmode: ExifOrientationMode, colormanagementmode: ColorManagementMode) -> windows_core::Result<windows_future::IAsyncOperation<PixelDataProvider>>
    where
        P2: windows_core::Param<BitmapTransform>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetPixelDataTransformedAsync)(windows_core::Interface::as_raw(this), pixelformat, alphamode, transform.param().abi(), exiforientationmode, colormanagementmode, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Storage_Streams")]
impl windows_core::RuntimeName for IBitmapFrame {
    const NAME: &'static str = "Windows.Graphics.Imaging.IBitmapFrame";
}
#[cfg(feature = "Storage_Streams")]
pub trait IBitmapFrame_Impl: windows_core::IUnknownImpl {
    fn GetThumbnailAsync(&self) -> windows_core::Result<windows_future::IAsyncOperation<ImageStream>>;
    fn BitmapProperties(&self) -> windows_core::Result<BitmapPropertiesView>;
    fn BitmapPixelFormat(&self) -> windows_core::Result<BitmapPixelFormat>;
    fn BitmapAlphaMode(&self) -> windows_core::Result<BitmapAlphaMode>;
    fn DpiX(&self) -> windows_core::Result<f64>;
    fn DpiY(&self) -> windows_core::Result<f64>;
    fn PixelWidth(&self) -> windows_core::Result<u32>;
    fn PixelHeight(&self) -> windows_core::Result<u32>;
    fn OrientedPixelWidth(&self) -> windows_core::Result<u32>;
    fn OrientedPixelHeight(&self) -> windows_core::Result<u32>;
    fn GetPixelDataAsync(&self) -> windows_core::Result<windows_future::IAsyncOperation<PixelDataProvider>>;
    fn GetPixelDataTransformedAsync(&self, pixelFormat: BitmapPixelFormat, alphaMode: BitmapAlphaMode, transform: windows_core::Ref<BitmapTransform>, exifOrientationMode: ExifOrientationMode, colorManagementMode: ColorManagementMode) -> windows_core::Result<windows_future::IAsyncOperation<PixelDataProvider>>;
}
#[cfg(feature = "Storage_Streams")]
impl IBitmapFrame_Vtbl {
    pub const fn new<Identity: IBitmapFrame_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetThumbnailAsync<Identity: IBitmapFrame_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IBitmapFrame_Impl::GetThumbnailAsync(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn BitmapProperties<Identity: IBitmapFrame_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IBitmapFrame_Impl::BitmapProperties(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn BitmapPixelFormat<Identity: IBitmapFrame_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut BitmapPixelFormat) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IBitmapFrame_Impl::BitmapPixelFormat(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn BitmapAlphaMode<Identity: IBitmapFrame_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut BitmapAlphaMode) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IBitmapFrame_Impl::BitmapAlphaMode(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DpiX<Identity: IBitmapFrame_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IBitmapFrame_Impl::DpiX(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DpiY<Identity: IBitmapFrame_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IBitmapFrame_Impl::DpiY(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn PixelWidth<Identity: IBitmapFrame_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IBitmapFrame_Impl::PixelWidth(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn PixelHeight<Identity: IBitmapFrame_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IBitmapFrame_Impl::PixelHeight(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn OrientedPixelWidth<Identity: IBitmapFrame_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IBitmapFrame_Impl::OrientedPixelWidth(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn OrientedPixelHeight<Identity: IBitmapFrame_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IBitmapFrame_Impl::OrientedPixelHeight(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetPixelDataAsync<Identity: IBitmapFrame_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IBitmapFrame_Impl::GetPixelDataAsync(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetPixelDataTransformedAsync<Identity: IBitmapFrame_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pixelformat: BitmapPixelFormat, alphamode: BitmapAlphaMode, transform: *mut core::ffi::c_void, exiforientationmode: ExifOrientationMode, colormanagementmode: ColorManagementMode, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IBitmapFrame_Impl::GetPixelDataTransformedAsync(this, pixelformat, alphamode, core::mem::transmute_copy(&transform), exiforientationmode, colormanagementmode) {
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
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IBitmapFrame, OFFSET>(),
            GetThumbnailAsync: GetThumbnailAsync::<Identity, OFFSET>,
            BitmapProperties: BitmapProperties::<Identity, OFFSET>,
            BitmapPixelFormat: BitmapPixelFormat::<Identity, OFFSET>,
            BitmapAlphaMode: BitmapAlphaMode::<Identity, OFFSET>,
            DpiX: DpiX::<Identity, OFFSET>,
            DpiY: DpiY::<Identity, OFFSET>,
            PixelWidth: PixelWidth::<Identity, OFFSET>,
            PixelHeight: PixelHeight::<Identity, OFFSET>,
            OrientedPixelWidth: OrientedPixelWidth::<Identity, OFFSET>,
            OrientedPixelHeight: OrientedPixelHeight::<Identity, OFFSET>,
            GetPixelDataAsync: GetPixelDataAsync::<Identity, OFFSET>,
            GetPixelDataTransformedAsync: GetPixelDataTransformedAsync::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IBitmapFrame as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IBitmapFrame_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Storage_Streams")]
    pub GetThumbnailAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    GetThumbnailAsync: usize,
    pub BitmapProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub BitmapPixelFormat: unsafe extern "system" fn(*mut core::ffi::c_void, *mut BitmapPixelFormat) -> windows_core::HRESULT,
    pub BitmapAlphaMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut BitmapAlphaMode) -> windows_core::HRESULT,
    pub DpiX: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub DpiY: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub PixelWidth: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub PixelHeight: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub OrientedPixelWidth: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub OrientedPixelHeight: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetPixelDataAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetPixelDataTransformedAsync: unsafe extern "system" fn(*mut core::ffi::c_void, BitmapPixelFormat, BitmapAlphaMode, *mut core::ffi::c_void, ExifOrientationMode, ColorManagementMode, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IBitmapFrameWithSoftwareBitmap, IBitmapFrameWithSoftwareBitmap_Vtbl, 0xfe287c9a_420c_4963_87ad_691436e08383);
impl windows_core::RuntimeType for IBitmapFrameWithSoftwareBitmap {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(IBitmapFrameWithSoftwareBitmap, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(IBitmapFrameWithSoftwareBitmap, IBitmapFrame);
impl IBitmapFrameWithSoftwareBitmap {
    pub fn GetSoftwareBitmapAsync(&self) -> windows_core::Result<windows_future::IAsyncOperation<SoftwareBitmap>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetSoftwareBitmapAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetSoftwareBitmapConvertedAsync(&self, pixelformat: BitmapPixelFormat, alphamode: BitmapAlphaMode) -> windows_core::Result<windows_future::IAsyncOperation<SoftwareBitmap>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetSoftwareBitmapConvertedAsync)(windows_core::Interface::as_raw(this), pixelformat, alphamode, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetSoftwareBitmapTransformedAsync<P2>(&self, pixelformat: BitmapPixelFormat, alphamode: BitmapAlphaMode, transform: P2, exiforientationmode: ExifOrientationMode, colormanagementmode: ColorManagementMode) -> windows_core::Result<windows_future::IAsyncOperation<SoftwareBitmap>>
    where
        P2: windows_core::Param<BitmapTransform>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetSoftwareBitmapTransformedAsync)(windows_core::Interface::as_raw(this), pixelformat, alphamode, transform.param().abi(), exiforientationmode, colormanagementmode, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn GetThumbnailAsync(&self) -> windows_core::Result<windows_future::IAsyncOperation<ImageStream>> {
        let this = &windows_core::Interface::cast::<IBitmapFrame>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetThumbnailAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn BitmapProperties(&self) -> windows_core::Result<BitmapPropertiesView> {
        let this = &windows_core::Interface::cast::<IBitmapFrame>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BitmapProperties)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn BitmapPixelFormat(&self) -> windows_core::Result<BitmapPixelFormat> {
        let this = &windows_core::Interface::cast::<IBitmapFrame>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BitmapPixelFormat)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn BitmapAlphaMode(&self) -> windows_core::Result<BitmapAlphaMode> {
        let this = &windows_core::Interface::cast::<IBitmapFrame>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BitmapAlphaMode)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn DpiX(&self) -> windows_core::Result<f64> {
        let this = &windows_core::Interface::cast::<IBitmapFrame>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DpiX)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn DpiY(&self) -> windows_core::Result<f64> {
        let this = &windows_core::Interface::cast::<IBitmapFrame>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DpiY)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PixelWidth(&self) -> windows_core::Result<u32> {
        let this = &windows_core::Interface::cast::<IBitmapFrame>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PixelWidth)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PixelHeight(&self) -> windows_core::Result<u32> {
        let this = &windows_core::Interface::cast::<IBitmapFrame>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PixelHeight)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn OrientedPixelWidth(&self) -> windows_core::Result<u32> {
        let this = &windows_core::Interface::cast::<IBitmapFrame>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OrientedPixelWidth)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn OrientedPixelHeight(&self) -> windows_core::Result<u32> {
        let this = &windows_core::Interface::cast::<IBitmapFrame>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OrientedPixelHeight)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn GetPixelDataAsync(&self) -> windows_core::Result<windows_future::IAsyncOperation<PixelDataProvider>> {
        let this = &windows_core::Interface::cast::<IBitmapFrame>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetPixelDataAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetPixelDataTransformedAsync<P2>(&self, pixelformat: BitmapPixelFormat, alphamode: BitmapAlphaMode, transform: P2, exiforientationmode: ExifOrientationMode, colormanagementmode: ColorManagementMode) -> windows_core::Result<windows_future::IAsyncOperation<PixelDataProvider>>
    where
        P2: windows_core::Param<BitmapTransform>,
    {
        let this = &windows_core::Interface::cast::<IBitmapFrame>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetPixelDataTransformedAsync)(windows_core::Interface::as_raw(this), pixelformat, alphamode, transform.param().abi(), exiforientationmode, colormanagementmode, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Storage_Streams")]
impl windows_core::RuntimeName for IBitmapFrameWithSoftwareBitmap {
    const NAME: &'static str = "Windows.Graphics.Imaging.IBitmapFrameWithSoftwareBitmap";
}
#[cfg(feature = "Storage_Streams")]
pub trait IBitmapFrameWithSoftwareBitmap_Impl: IBitmapFrame_Impl {
    fn GetSoftwareBitmapAsync(&self) -> windows_core::Result<windows_future::IAsyncOperation<SoftwareBitmap>>;
    fn GetSoftwareBitmapConvertedAsync(&self, pixelFormat: BitmapPixelFormat, alphaMode: BitmapAlphaMode) -> windows_core::Result<windows_future::IAsyncOperation<SoftwareBitmap>>;
    fn GetSoftwareBitmapTransformedAsync(&self, pixelFormat: BitmapPixelFormat, alphaMode: BitmapAlphaMode, transform: windows_core::Ref<BitmapTransform>, exifOrientationMode: ExifOrientationMode, colorManagementMode: ColorManagementMode) -> windows_core::Result<windows_future::IAsyncOperation<SoftwareBitmap>>;
}
#[cfg(feature = "Storage_Streams")]
impl IBitmapFrameWithSoftwareBitmap_Vtbl {
    pub const fn new<Identity: IBitmapFrameWithSoftwareBitmap_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetSoftwareBitmapAsync<Identity: IBitmapFrameWithSoftwareBitmap_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IBitmapFrameWithSoftwareBitmap_Impl::GetSoftwareBitmapAsync(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetSoftwareBitmapConvertedAsync<Identity: IBitmapFrameWithSoftwareBitmap_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pixelformat: BitmapPixelFormat, alphamode: BitmapAlphaMode, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IBitmapFrameWithSoftwareBitmap_Impl::GetSoftwareBitmapConvertedAsync(this, pixelformat, alphamode) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetSoftwareBitmapTransformedAsync<Identity: IBitmapFrameWithSoftwareBitmap_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pixelformat: BitmapPixelFormat, alphamode: BitmapAlphaMode, transform: *mut core::ffi::c_void, exiforientationmode: ExifOrientationMode, colormanagementmode: ColorManagementMode, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IBitmapFrameWithSoftwareBitmap_Impl::GetSoftwareBitmapTransformedAsync(this, pixelformat, alphamode, core::mem::transmute_copy(&transform), exiforientationmode, colormanagementmode) {
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
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IBitmapFrameWithSoftwareBitmap, OFFSET>(),
            GetSoftwareBitmapAsync: GetSoftwareBitmapAsync::<Identity, OFFSET>,
            GetSoftwareBitmapConvertedAsync: GetSoftwareBitmapConvertedAsync::<Identity, OFFSET>,
            GetSoftwareBitmapTransformedAsync: GetSoftwareBitmapTransformedAsync::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IBitmapFrameWithSoftwareBitmap as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IBitmapFrameWithSoftwareBitmap_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetSoftwareBitmapAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetSoftwareBitmapConvertedAsync: unsafe extern "system" fn(*mut core::ffi::c_void, BitmapPixelFormat, BitmapAlphaMode, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetSoftwareBitmapTransformedAsync: unsafe extern "system" fn(*mut core::ffi::c_void, BitmapPixelFormat, BitmapAlphaMode, *mut core::ffi::c_void, ExifOrientationMode, ColorManagementMode, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IBitmapProperties, IBitmapProperties_Vtbl, 0xea9f4f1b_b505_4450_a4d1_e8ca94529d8d);
impl windows_core::RuntimeType for IBitmapProperties {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IBitmapProperties_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SetPropertiesAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IBitmapPropertiesView, IBitmapPropertiesView_Vtbl, 0x7e0fe87a_3a70_48f8_9c55_196cf5a545f5);
impl windows_core::RuntimeType for IBitmapPropertiesView {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(IBitmapPropertiesView, windows_core::IUnknown, windows_core::IInspectable);
impl IBitmapPropertiesView {
    pub fn GetPropertiesAsync<P0>(&self, propertiestoretrieve: P0) -> windows_core::Result<windows_future::IAsyncOperation<BitmapPropertySet>>
    where
        P0: windows_core::Param<windows_collections::IIterable<windows_core::HSTRING>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetPropertiesAsync)(windows_core::Interface::as_raw(this), propertiestoretrieve.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeName for IBitmapPropertiesView {
    const NAME: &'static str = "Windows.Graphics.Imaging.IBitmapPropertiesView";
}
pub trait IBitmapPropertiesView_Impl: windows_core::IUnknownImpl {
    fn GetPropertiesAsync(&self, propertiesToRetrieve: windows_core::Ref<windows_collections::IIterable<windows_core::HSTRING>>) -> windows_core::Result<windows_future::IAsyncOperation<BitmapPropertySet>>;
}
impl IBitmapPropertiesView_Vtbl {
    pub const fn new<Identity: IBitmapPropertiesView_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetPropertiesAsync<Identity: IBitmapPropertiesView_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, propertiestoretrieve: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IBitmapPropertiesView_Impl::GetPropertiesAsync(this, core::mem::transmute_copy(&propertiestoretrieve)) {
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
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IBitmapPropertiesView, OFFSET>(),
            GetPropertiesAsync: GetPropertiesAsync::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IBitmapPropertiesView as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IBitmapPropertiesView_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetPropertiesAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IBitmapTransform, IBitmapTransform_Vtbl, 0xae755344_e268_4d35_adcf_e995d31a8d34);
impl windows_core::RuntimeType for IBitmapTransform {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IBitmapTransform_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ScaledWidth: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetScaledWidth: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub ScaledHeight: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetScaledHeight: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub InterpolationMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut BitmapInterpolationMode) -> windows_core::HRESULT,
    pub SetInterpolationMode: unsafe extern "system" fn(*mut core::ffi::c_void, BitmapInterpolationMode) -> windows_core::HRESULT,
    pub Flip: unsafe extern "system" fn(*mut core::ffi::c_void, *mut BitmapFlip) -> windows_core::HRESULT,
    pub SetFlip: unsafe extern "system" fn(*mut core::ffi::c_void, BitmapFlip) -> windows_core::HRESULT,
    pub Rotation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut BitmapRotation) -> windows_core::HRESULT,
    pub SetRotation: unsafe extern "system" fn(*mut core::ffi::c_void, BitmapRotation) -> windows_core::HRESULT,
    pub Bounds: unsafe extern "system" fn(*mut core::ffi::c_void, *mut BitmapBounds) -> windows_core::HRESULT,
    pub SetBounds: unsafe extern "system" fn(*mut core::ffi::c_void, BitmapBounds) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IBitmapTypedValue, IBitmapTypedValue_Vtbl, 0xcd8044a9_2443_4000_b0cd_79316c56f589);
impl windows_core::RuntimeType for IBitmapTypedValue {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IBitmapTypedValue_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Value: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Type: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::PropertyType) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IBitmapTypedValueFactory, IBitmapTypedValueFactory_Vtbl, 0x92dbb599_ce13_46bb_9545_cb3a3f63eb8b);
impl windows_core::RuntimeType for IBitmapTypedValueFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IBitmapTypedValueFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::Foundation::PropertyType, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPixelDataProvider, IPixelDataProvider_Vtbl, 0xdd831f25_185c_4595_9fb9_ccbe6ec18a6f);
impl windows_core::RuntimeType for IPixelDataProvider {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IPixelDataProvider_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub DetachPixelData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut *mut u8) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISoftwareBitmap, ISoftwareBitmap_Vtbl, 0x689e0708_7eef_483f_963f_da938818e073);
impl windows_core::RuntimeType for ISoftwareBitmap {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ISoftwareBitmap_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub BitmapPixelFormat: unsafe extern "system" fn(*mut core::ffi::c_void, *mut BitmapPixelFormat) -> windows_core::HRESULT,
    pub BitmapAlphaMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut BitmapAlphaMode) -> windows_core::HRESULT,
    pub PixelWidth: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub PixelHeight: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub IsReadOnly: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetDpiX: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub DpiX: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub SetDpiY: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub DpiY: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub LockBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, BitmapBufferAccessMode, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CopyTo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub CopyFromBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CopyFromBuffer: usize,
    #[cfg(feature = "Storage_Streams")]
    pub CopyToBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CopyToBuffer: usize,
    pub GetReadOnlyView: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISoftwareBitmapFactory, ISoftwareBitmapFactory_Vtbl, 0xc99feb69_2d62_4d47_a6b3_4fdb6a07fdf8);
impl windows_core::RuntimeType for ISoftwareBitmapFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ISoftwareBitmapFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, BitmapPixelFormat, i32, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateWithAlpha: unsafe extern "system" fn(*mut core::ffi::c_void, BitmapPixelFormat, i32, i32, BitmapAlphaMode, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISoftwareBitmapStatics, ISoftwareBitmapStatics_Vtbl, 0xdf0385db_672f_4a9d_806e_c2442f343e86);
impl windows_core::RuntimeType for ISoftwareBitmapStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ISoftwareBitmapStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Copy: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Convert: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, BitmapPixelFormat, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ConvertWithAlpha: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, BitmapPixelFormat, BitmapAlphaMode, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub CreateCopyFromBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, BitmapPixelFormat, i32, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CreateCopyFromBuffer: usize,
    #[cfg(feature = "Storage_Streams")]
    pub CreateCopyWithAlphaFromBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, BitmapPixelFormat, i32, i32, BitmapAlphaMode, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CreateCopyWithAlphaFromBuffer: usize,
    #[cfg(feature = "Graphics_DirectX_Direct3D11")]
    pub CreateCopyFromSurfaceAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX_Direct3D11"))]
    CreateCopyFromSurfaceAsync: usize,
    #[cfg(feature = "Graphics_DirectX_Direct3D11")]
    pub CreateCopyWithAlphaFromSurfaceAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, BitmapAlphaMode, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX_Direct3D11"))]
    CreateCopyWithAlphaFromSurfaceAsync: usize,
}
#[cfg(feature = "Storage_Streams")]
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ImageStream(windows_core::IUnknown);
#[cfg(feature = "Storage_Streams")]
windows_core::imp::interface_hierarchy!(ImageStream, windows_core::IUnknown, windows_core::IInspectable, super::super::Storage::Streams::IRandomAccessStreamWithContentType);
#[cfg(feature = "Storage_Streams")]
windows_core::imp::required_hierarchy!(ImageStream, super::super::Foundation::IClosable, super::super::Storage::Streams::IContentTypeProvider, super::super::Storage::Streams::IInputStream, super::super::Storage::Streams::IOutputStream, super::super::Storage::Streams::IRandomAccessStream);
#[cfg(feature = "Storage_Streams")]
impl ImageStream {
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
}
#[cfg(feature = "Storage_Streams")]
impl windows_core::RuntimeType for ImageStream {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, super::super::Storage::Streams::IRandomAccessStreamWithContentType>();
}
#[cfg(feature = "Storage_Streams")]
unsafe impl windows_core::Interface for ImageStream {
    type Vtable = <super::super::Storage::Streams::IRandomAccessStreamWithContentType as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <super::super::Storage::Streams::IRandomAccessStreamWithContentType as windows_core::Interface>::IID;
}
#[cfg(feature = "Storage_Streams")]
impl windows_core::RuntimeName for ImageStream {
    const NAME: &'static str = "Windows.Graphics.Imaging.ImageStream";
}
#[cfg(feature = "Storage_Streams")]
unsafe impl Send for ImageStream {}
#[cfg(feature = "Storage_Streams")]
unsafe impl Sync for ImageStream {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct JpegSubsamplingMode(pub i32);
impl JpegSubsamplingMode {
    pub const Default: Self = Self(0i32);
    pub const Y4Cb2Cr0: Self = Self(1i32);
    pub const Y4Cb2Cr2: Self = Self(2i32);
    pub const Y4Cb4Cr4: Self = Self(3i32);
}
impl windows_core::TypeKind for JpegSubsamplingMode {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for JpegSubsamplingMode {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Graphics.Imaging.JpegSubsamplingMode;i4)");
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PixelDataProvider(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PixelDataProvider, windows_core::IUnknown, windows_core::IInspectable);
impl PixelDataProvider {
    pub fn DetachPixelData(&self) -> windows_core::Result<windows_core::Array<u8>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::MaybeUninit::zeroed();
            (windows_core::Interface::vtable(this).DetachPixelData)(windows_core::Interface::as_raw(this), windows_core::Array::<u8>::set_abi_len(core::mem::transmute(&mut result__)), result__.as_mut_ptr() as *mut _ as _).map(|| result__.assume_init())
        }
    }
}
impl windows_core::RuntimeType for PixelDataProvider {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPixelDataProvider>();
}
unsafe impl windows_core::Interface for PixelDataProvider {
    type Vtable = <IPixelDataProvider as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IPixelDataProvider as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PixelDataProvider {
    const NAME: &'static str = "Windows.Graphics.Imaging.PixelDataProvider";
}
unsafe impl Send for PixelDataProvider {}
unsafe impl Sync for PixelDataProvider {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PngFilterMode(pub i32);
impl PngFilterMode {
    pub const Automatic: Self = Self(0i32);
    pub const None: Self = Self(1i32);
    pub const Sub: Self = Self(2i32);
    pub const Up: Self = Self(3i32);
    pub const Average: Self = Self(4i32);
    pub const Paeth: Self = Self(5i32);
    pub const Adaptive: Self = Self(6i32);
}
impl windows_core::TypeKind for PngFilterMode {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for PngFilterMode {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Graphics.Imaging.PngFilterMode;i4)");
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SoftwareBitmap(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(SoftwareBitmap, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(SoftwareBitmap, super::super::Foundation::IClosable);
impl SoftwareBitmap {
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn BitmapPixelFormat(&self) -> windows_core::Result<BitmapPixelFormat> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BitmapPixelFormat)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn BitmapAlphaMode(&self) -> windows_core::Result<BitmapAlphaMode> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BitmapAlphaMode)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PixelWidth(&self) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PixelWidth)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PixelHeight(&self) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PixelHeight)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsReadOnly(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsReadOnly)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetDpiX(&self, value: f64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetDpiX)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn DpiX(&self) -> windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DpiX)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetDpiY(&self, value: f64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetDpiY)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn DpiY(&self) -> windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DpiY)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn LockBuffer(&self, mode: BitmapBufferAccessMode) -> windows_core::Result<BitmapBuffer> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LockBuffer)(windows_core::Interface::as_raw(this), mode, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CopyTo<P0>(&self, bitmap: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<SoftwareBitmap>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).CopyTo)(windows_core::Interface::as_raw(this), bitmap.param().abi()).ok() }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn CopyFromBuffer<P0>(&self, buffer: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Storage::Streams::IBuffer>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).CopyFromBuffer)(windows_core::Interface::as_raw(this), buffer.param().abi()).ok() }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn CopyToBuffer<P0>(&self, buffer: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Storage::Streams::IBuffer>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).CopyToBuffer)(windows_core::Interface::as_raw(this), buffer.param().abi()).ok() }
    }
    pub fn GetReadOnlyView(&self) -> windows_core::Result<SoftwareBitmap> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetReadOnlyView)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Create(format: BitmapPixelFormat, width: i32, height: i32) -> windows_core::Result<SoftwareBitmap> {
        Self::ISoftwareBitmapFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Create)(windows_core::Interface::as_raw(this), format, width, height, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateWithAlpha(format: BitmapPixelFormat, width: i32, height: i32, alpha: BitmapAlphaMode) -> windows_core::Result<SoftwareBitmap> {
        Self::ISoftwareBitmapFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateWithAlpha)(windows_core::Interface::as_raw(this), format, width, height, alpha, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn Copy<P0>(source: P0) -> windows_core::Result<SoftwareBitmap>
    where
        P0: windows_core::Param<SoftwareBitmap>,
    {
        Self::ISoftwareBitmapStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Copy)(windows_core::Interface::as_raw(this), source.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn Convert<P0>(source: P0, format: BitmapPixelFormat) -> windows_core::Result<SoftwareBitmap>
    where
        P0: windows_core::Param<SoftwareBitmap>,
    {
        Self::ISoftwareBitmapStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Convert)(windows_core::Interface::as_raw(this), source.param().abi(), format, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn ConvertWithAlpha<P0>(source: P0, format: BitmapPixelFormat, alpha: BitmapAlphaMode) -> windows_core::Result<SoftwareBitmap>
    where
        P0: windows_core::Param<SoftwareBitmap>,
    {
        Self::ISoftwareBitmapStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ConvertWithAlpha)(windows_core::Interface::as_raw(this), source.param().abi(), format, alpha, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn CreateCopyFromBuffer<P0>(source: P0, format: BitmapPixelFormat, width: i32, height: i32) -> windows_core::Result<SoftwareBitmap>
    where
        P0: windows_core::Param<super::super::Storage::Streams::IBuffer>,
    {
        Self::ISoftwareBitmapStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateCopyFromBuffer)(windows_core::Interface::as_raw(this), source.param().abi(), format, width, height, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn CreateCopyWithAlphaFromBuffer<P0>(source: P0, format: BitmapPixelFormat, width: i32, height: i32, alpha: BitmapAlphaMode) -> windows_core::Result<SoftwareBitmap>
    where
        P0: windows_core::Param<super::super::Storage::Streams::IBuffer>,
    {
        Self::ISoftwareBitmapStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateCopyWithAlphaFromBuffer)(windows_core::Interface::as_raw(this), source.param().abi(), format, width, height, alpha, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Graphics_DirectX_Direct3D11")]
    pub fn CreateCopyFromSurfaceAsync<P0>(surface: P0) -> windows_core::Result<windows_future::IAsyncOperation<SoftwareBitmap>>
    where
        P0: windows_core::Param<super::DirectX::Direct3D11::IDirect3DSurface>,
    {
        Self::ISoftwareBitmapStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateCopyFromSurfaceAsync)(windows_core::Interface::as_raw(this), surface.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Graphics_DirectX_Direct3D11")]
    pub fn CreateCopyWithAlphaFromSurfaceAsync<P0>(surface: P0, alpha: BitmapAlphaMode) -> windows_core::Result<windows_future::IAsyncOperation<SoftwareBitmap>>
    where
        P0: windows_core::Param<super::DirectX::Direct3D11::IDirect3DSurface>,
    {
        Self::ISoftwareBitmapStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateCopyWithAlphaFromSurfaceAsync)(windows_core::Interface::as_raw(this), surface.param().abi(), alpha, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    fn ISoftwareBitmapFactory<R, F: FnOnce(&ISoftwareBitmapFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<SoftwareBitmap, ISoftwareBitmapFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    fn ISoftwareBitmapStatics<R, F: FnOnce(&ISoftwareBitmapStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<SoftwareBitmap, ISoftwareBitmapStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for SoftwareBitmap {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ISoftwareBitmap>();
}
unsafe impl windows_core::Interface for SoftwareBitmap {
    type Vtable = <ISoftwareBitmap as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ISoftwareBitmap as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for SoftwareBitmap {
    const NAME: &'static str = "Windows.Graphics.Imaging.SoftwareBitmap";
}
unsafe impl Send for SoftwareBitmap {}
unsafe impl Sync for SoftwareBitmap {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TiffCompressionMode(pub i32);
impl TiffCompressionMode {
    pub const Automatic: Self = Self(0i32);
    pub const None: Self = Self(1i32);
    pub const Ccitt3: Self = Self(2i32);
    pub const Ccitt4: Self = Self(3i32);
    pub const Lzw: Self = Self(4i32);
    pub const Rle: Self = Self(5i32);
    pub const Zip: Self = Self(6i32);
    pub const LzwhDifferencing: Self = Self(7i32);
}
impl windows_core::TypeKind for TiffCompressionMode {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for TiffCompressionMode {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Graphics.Imaging.TiffCompressionMode;i4)");
}
