windows_core::imp::define_interface!(IPdfDocument, IPdfDocument_Vtbl, 0xac7ebedd_80fa_4089_846e_81b77ff5a86c);
impl windows_core::RuntimeType for IPdfDocument {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPdfDocument_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetPage: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PageCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub IsPasswordProtected: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPdfDocumentStatics, IPdfDocumentStatics_Vtbl, 0x433a0b5f_c007_4788_90f2_08143d922599);
impl windows_core::RuntimeType for IPdfDocumentStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPdfDocumentStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Storage")]
    pub LoadFromFileAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage"))]
    LoadFromFileAsync: usize,
    #[cfg(feature = "Storage")]
    pub LoadFromFileWithPasswordAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage"))]
    LoadFromFileWithPasswordAsync: usize,
    #[cfg(feature = "Storage_Streams")]
    pub LoadFromStreamAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    LoadFromStreamAsync: usize,
    #[cfg(feature = "Storage_Streams")]
    pub LoadFromStreamWithPasswordAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    LoadFromStreamWithPasswordAsync: usize,
}
windows_core::imp::define_interface!(IPdfPage, IPdfPage_Vtbl, 0x9db4b0c8_5320_4cfc_ad76_493fdad0e594);
impl windows_core::RuntimeType for IPdfPage {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPdfPage_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Storage_Streams")]
    pub RenderToStreamAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    RenderToStreamAsync: usize,
    #[cfg(feature = "Storage_Streams")]
    pub RenderWithOptionsToStreamAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    RenderWithOptionsToStreamAsync: usize,
    pub PreparePageAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Index: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Size: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::Size) -> windows_core::HRESULT,
    pub Dimensions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Rotation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut PdfPageRotation) -> windows_core::HRESULT,
    pub PreferredZoom: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPdfPageDimensions, IPdfPageDimensions_Vtbl, 0x22170471_313e_44e8_835d_63a3e7624a10);
impl windows_core::RuntimeType for IPdfPageDimensions {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPdfPageDimensions_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub MediaBox: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::Rect) -> windows_core::HRESULT,
    pub CropBox: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::Rect) -> windows_core::HRESULT,
    pub BleedBox: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::Rect) -> windows_core::HRESULT,
    pub TrimBox: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::Rect) -> windows_core::HRESULT,
    pub ArtBox: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::Rect) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPdfPageRenderOptions, IPdfPageRenderOptions_Vtbl, 0x3c98056f_b7cf_4c29_9a04_52d90267f425);
impl windows_core::RuntimeType for IPdfPageRenderOptions {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPdfPageRenderOptions_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SourceRect: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::Rect) -> windows_core::HRESULT,
    pub SetSourceRect: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::Rect) -> windows_core::HRESULT,
    pub DestinationWidth: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetDestinationWidth: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub DestinationHeight: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetDestinationHeight: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    #[cfg(feature = "UI")]
    pub BackgroundColor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::UI::Color) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI"))]
    BackgroundColor: usize,
    #[cfg(feature = "UI")]
    pub SetBackgroundColor: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::UI::Color) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI"))]
    SetBackgroundColor: usize,
    pub IsIgnoringHighContrast: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetIsIgnoringHighContrast: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub BitmapEncoderId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub SetBitmapEncoderId: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID) -> windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct PdfDocument(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PdfDocument, windows_core::IUnknown, windows_core::IInspectable);
impl PdfDocument {
    pub fn GetPage(&self, pageindex: u32) -> windows_core::Result<PdfPage> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetPage)(windows_core::Interface::as_raw(this), pageindex, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn PageCount(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PageCount)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsPasswordProtected(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsPasswordProtected)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Storage")]
    pub fn LoadFromFileAsync<P0>(file: P0) -> windows_core::Result<super::super::Foundation::IAsyncOperation<PdfDocument>>
    where
        P0: windows_core::Param<super::super::Storage::IStorageFile>,
    {
        Self::IPdfDocumentStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LoadFromFileAsync)(windows_core::Interface::as_raw(this), file.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Storage")]
    pub fn LoadFromFileWithPasswordAsync<P0>(file: P0, password: &windows_core::HSTRING) -> windows_core::Result<super::super::Foundation::IAsyncOperation<PdfDocument>>
    where
        P0: windows_core::Param<super::super::Storage::IStorageFile>,
    {
        Self::IPdfDocumentStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LoadFromFileWithPasswordAsync)(windows_core::Interface::as_raw(this), file.param().abi(), core::mem::transmute_copy(password), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn LoadFromStreamAsync<P0>(inputstream: P0) -> windows_core::Result<super::super::Foundation::IAsyncOperation<PdfDocument>>
    where
        P0: windows_core::Param<super::super::Storage::Streams::IRandomAccessStream>,
    {
        Self::IPdfDocumentStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LoadFromStreamAsync)(windows_core::Interface::as_raw(this), inputstream.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn LoadFromStreamWithPasswordAsync<P0>(inputstream: P0, password: &windows_core::HSTRING) -> windows_core::Result<super::super::Foundation::IAsyncOperation<PdfDocument>>
    where
        P0: windows_core::Param<super::super::Storage::Streams::IRandomAccessStream>,
    {
        Self::IPdfDocumentStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LoadFromStreamWithPasswordAsync)(windows_core::Interface::as_raw(this), inputstream.param().abi(), core::mem::transmute_copy(password), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IPdfDocumentStatics<R, F: FnOnce(&IPdfDocumentStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<PdfDocument, IPdfDocumentStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for PdfDocument {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPdfDocument>();
}
unsafe impl windows_core::Interface for PdfDocument {
    type Vtable = IPdfDocument_Vtbl;
    const IID: windows_core::GUID = <IPdfDocument as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PdfDocument {
    const NAME: &'static str = "Windows.Data.Pdf.PdfDocument";
}
unsafe impl Send for PdfDocument {}
unsafe impl Sync for PdfDocument {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct PdfPage(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PdfPage, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(PdfPage, super::super::Foundation::IClosable);
impl PdfPage {
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn RenderToStreamAsync<P0>(&self, outputstream: P0) -> windows_core::Result<super::super::Foundation::IAsyncAction>
    where
        P0: windows_core::Param<super::super::Storage::Streams::IRandomAccessStream>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RenderToStreamAsync)(windows_core::Interface::as_raw(this), outputstream.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn RenderWithOptionsToStreamAsync<P0, P1>(&self, outputstream: P0, options: P1) -> windows_core::Result<super::super::Foundation::IAsyncAction>
    where
        P0: windows_core::Param<super::super::Storage::Streams::IRandomAccessStream>,
        P1: windows_core::Param<PdfPageRenderOptions>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RenderWithOptionsToStreamAsync)(windows_core::Interface::as_raw(this), outputstream.param().abi(), options.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn PreparePageAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PreparePageAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Index(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Index)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Size(&self) -> windows_core::Result<super::super::Foundation::Size> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Size)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Dimensions(&self) -> windows_core::Result<PdfPageDimensions> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Dimensions)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Rotation(&self) -> windows_core::Result<PdfPageRotation> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Rotation)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PreferredZoom(&self) -> windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PreferredZoom)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for PdfPage {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPdfPage>();
}
unsafe impl windows_core::Interface for PdfPage {
    type Vtable = IPdfPage_Vtbl;
    const IID: windows_core::GUID = <IPdfPage as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PdfPage {
    const NAME: &'static str = "Windows.Data.Pdf.PdfPage";
}
unsafe impl Send for PdfPage {}
unsafe impl Sync for PdfPage {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct PdfPageDimensions(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PdfPageDimensions, windows_core::IUnknown, windows_core::IInspectable);
impl PdfPageDimensions {
    pub fn MediaBox(&self) -> windows_core::Result<super::super::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MediaBox)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn CropBox(&self) -> windows_core::Result<super::super::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CropBox)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn BleedBox(&self) -> windows_core::Result<super::super::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BleedBox)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn TrimBox(&self) -> windows_core::Result<super::super::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TrimBox)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ArtBox(&self) -> windows_core::Result<super::super::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ArtBox)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for PdfPageDimensions {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPdfPageDimensions>();
}
unsafe impl windows_core::Interface for PdfPageDimensions {
    type Vtable = IPdfPageDimensions_Vtbl;
    const IID: windows_core::GUID = <IPdfPageDimensions as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PdfPageDimensions {
    const NAME: &'static str = "Windows.Data.Pdf.PdfPageDimensions";
}
unsafe impl Send for PdfPageDimensions {}
unsafe impl Sync for PdfPageDimensions {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct PdfPageRenderOptions(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PdfPageRenderOptions, windows_core::IUnknown, windows_core::IInspectable);
impl PdfPageRenderOptions {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<PdfPageRenderOptions, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn SourceRect(&self) -> windows_core::Result<super::super::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SourceRect)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetSourceRect(&self, value: super::super::Foundation::Rect) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetSourceRect)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn DestinationWidth(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DestinationWidth)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetDestinationWidth(&self, value: u32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetDestinationWidth)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn DestinationHeight(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DestinationHeight)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetDestinationHeight(&self, value: u32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetDestinationHeight)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "UI")]
    pub fn BackgroundColor(&self) -> windows_core::Result<super::super::UI::Color> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BackgroundColor)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "UI")]
    pub fn SetBackgroundColor(&self, value: super::super::UI::Color) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetBackgroundColor)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsIgnoringHighContrast(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsIgnoringHighContrast)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsIgnoringHighContrast(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetIsIgnoringHighContrast)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn BitmapEncoderId(&self) -> windows_core::Result<windows_core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BitmapEncoderId)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetBitmapEncoderId(&self, value: windows_core::GUID) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetBitmapEncoderId)(windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl windows_core::RuntimeType for PdfPageRenderOptions {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPdfPageRenderOptions>();
}
unsafe impl windows_core::Interface for PdfPageRenderOptions {
    type Vtable = IPdfPageRenderOptions_Vtbl;
    const IID: windows_core::GUID = <IPdfPageRenderOptions as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PdfPageRenderOptions {
    const NAME: &'static str = "Windows.Data.Pdf.PdfPageRenderOptions";
}
unsafe impl Send for PdfPageRenderOptions {}
unsafe impl Sync for PdfPageRenderOptions {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PdfPageRotation(pub i32);
impl PdfPageRotation {
    pub const Normal: Self = Self(0i32);
    pub const Rotate90: Self = Self(1i32);
    pub const Rotate180: Self = Self(2i32);
    pub const Rotate270: Self = Self(3i32);
}
impl windows_core::TypeKind for PdfPageRotation {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PdfPageRotation {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PdfPageRotation").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for PdfPageRotation {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Data.Pdf.PdfPageRotation;i4)");
}
