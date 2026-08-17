#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CompressAlgorithm(pub i32);
impl CompressAlgorithm {
    pub const InvalidAlgorithm: Self = Self(0i32);
    pub const NullAlgorithm: Self = Self(1i32);
    pub const Mszip: Self = Self(2i32);
    pub const Xpress: Self = Self(3i32);
    pub const XpressHuff: Self = Self(4i32);
    pub const Lzms: Self = Self(5i32);
}
impl windows_core::TypeKind for CompressAlgorithm {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for CompressAlgorithm {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Storage.Compression.CompressAlgorithm;i4)");
}
#[cfg(feature = "Storage_Streams")]
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Compressor(windows_core::IUnknown);
#[cfg(feature = "Storage_Streams")]
windows_core::imp::interface_hierarchy!(Compressor, windows_core::IUnknown, windows_core::IInspectable);
#[cfg(feature = "Storage_Streams")]
windows_core::imp::required_hierarchy!(Compressor, super::super::Foundation::IClosable, super::Streams::IOutputStream);
#[cfg(feature = "Storage_Streams")]
impl Compressor {
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn FinishAsync(&self) -> windows_core::Result<windows_future::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FinishAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn DetachStream(&self) -> windows_core::Result<super::Streams::IOutputStream> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DetachStream)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CreateCompressor<P0>(underlyingstream: P0) -> windows_core::Result<Compressor>
    where
        P0: windows_core::Param<super::Streams::IOutputStream>,
    {
        Self::ICompressorFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateCompressor)(windows_core::Interface::as_raw(this), underlyingstream.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateCompressorEx<P0>(underlyingstream: P0, algorithm: CompressAlgorithm, blocksize: u32) -> windows_core::Result<Compressor>
    where
        P0: windows_core::Param<super::Streams::IOutputStream>,
    {
        Self::ICompressorFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateCompressorEx)(windows_core::Interface::as_raw(this), underlyingstream.param().abi(), algorithm, blocksize, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn WriteAsync<P0>(&self, buffer: P0) -> windows_core::Result<windows_future::IAsyncOperationWithProgress<u32, u32>>
    where
        P0: windows_core::Param<super::Streams::IBuffer>,
    {
        let this = &windows_core::Interface::cast::<super::Streams::IOutputStream>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).WriteAsync)(windows_core::Interface::as_raw(this), buffer.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn FlushAsync(&self) -> windows_core::Result<windows_future::IAsyncOperation<bool>> {
        let this = &windows_core::Interface::cast::<super::Streams::IOutputStream>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FlushAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    fn ICompressorFactory<R, F: FnOnce(&ICompressorFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<Compressor, ICompressorFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
#[cfg(feature = "Storage_Streams")]
impl windows_core::RuntimeType for Compressor {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ICompressor>();
}
#[cfg(feature = "Storage_Streams")]
unsafe impl windows_core::Interface for Compressor {
    type Vtable = <ICompressor as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ICompressor as windows_core::Interface>::IID;
}
#[cfg(feature = "Storage_Streams")]
impl windows_core::RuntimeName for Compressor {
    const NAME: &'static str = "Windows.Storage.Compression.Compressor";
}
#[cfg(feature = "Storage_Streams")]
unsafe impl Send for Compressor {}
#[cfg(feature = "Storage_Streams")]
unsafe impl Sync for Compressor {}
#[cfg(feature = "Storage_Streams")]
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Decompressor(windows_core::IUnknown);
#[cfg(feature = "Storage_Streams")]
windows_core::imp::interface_hierarchy!(Decompressor, windows_core::IUnknown, windows_core::IInspectable);
#[cfg(feature = "Storage_Streams")]
windows_core::imp::required_hierarchy!(Decompressor, super::super::Foundation::IClosable, super::Streams::IInputStream);
#[cfg(feature = "Storage_Streams")]
impl Decompressor {
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn DetachStream(&self) -> windows_core::Result<super::Streams::IInputStream> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DetachStream)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CreateDecompressor<P0>(underlyingstream: P0) -> windows_core::Result<Decompressor>
    where
        P0: windows_core::Param<super::Streams::IInputStream>,
    {
        Self::IDecompressorFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateDecompressor)(windows_core::Interface::as_raw(this), underlyingstream.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn ReadAsync<P0>(&self, buffer: P0, count: u32, options: super::Streams::InputStreamOptions) -> windows_core::Result<windows_future::IAsyncOperationWithProgress<super::Streams::IBuffer, u32>>
    where
        P0: windows_core::Param<super::Streams::IBuffer>,
    {
        let this = &windows_core::Interface::cast::<super::Streams::IInputStream>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReadAsync)(windows_core::Interface::as_raw(this), buffer.param().abi(), count, options, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    fn IDecompressorFactory<R, F: FnOnce(&IDecompressorFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<Decompressor, IDecompressorFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
#[cfg(feature = "Storage_Streams")]
impl windows_core::RuntimeType for Decompressor {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IDecompressor>();
}
#[cfg(feature = "Storage_Streams")]
unsafe impl windows_core::Interface for Decompressor {
    type Vtable = <IDecompressor as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IDecompressor as windows_core::Interface>::IID;
}
#[cfg(feature = "Storage_Streams")]
impl windows_core::RuntimeName for Decompressor {
    const NAME: &'static str = "Windows.Storage.Compression.Decompressor";
}
#[cfg(feature = "Storage_Streams")]
unsafe impl Send for Decompressor {}
#[cfg(feature = "Storage_Streams")]
unsafe impl Sync for Decompressor {}
#[cfg(feature = "Storage_Streams")]
windows_core::imp::define_interface!(ICompressor, ICompressor_Vtbl, 0x0ac3645a_57ac_4ee1_b702_84d39d5424e0);
#[cfg(feature = "Storage_Streams")]
impl windows_core::RuntimeType for ICompressor {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[cfg(feature = "Storage_Streams")]
#[repr(C)]
#[doc(hidden)]
pub struct ICompressor_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub FinishAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DetachStream: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICompressorFactory, ICompressorFactory_Vtbl, 0x5f3d96a4_2cfb_442c_a8ba_d7d11b039da0);
impl windows_core::RuntimeType for ICompressorFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ICompressorFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Storage_Streams")]
    pub CreateCompressor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CreateCompressor: usize,
    #[cfg(feature = "Storage_Streams")]
    pub CreateCompressorEx: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, CompressAlgorithm, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CreateCompressorEx: usize,
}
#[cfg(feature = "Storage_Streams")]
windows_core::imp::define_interface!(IDecompressor, IDecompressor_Vtbl, 0xb883fe46_d68a_4c8b_ada0_4ee813fc5283);
#[cfg(feature = "Storage_Streams")]
impl windows_core::RuntimeType for IDecompressor {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[cfg(feature = "Storage_Streams")]
#[repr(C)]
#[doc(hidden)]
pub struct IDecompressor_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub DetachStream: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDecompressorFactory, IDecompressorFactory_Vtbl, 0x5337e252_1da2_42e1_8834_0379d28d742f);
impl windows_core::RuntimeType for IDecompressorFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IDecompressorFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Storage_Streams")]
    pub CreateDecompressor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CreateDecompressor: usize,
}
