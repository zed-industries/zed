windows_core::imp::define_interface!(IPlaylist, IPlaylist_Vtbl, 0x803736f5_cf44_4d97_83b3_7a089e9ab663);
impl windows_core::RuntimeType for IPlaylist {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPlaylist_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage"))]
    pub Files: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Storage")))]
    Files: usize,
    pub SaveAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Storage")]
    pub SaveAsAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, super::super::Storage::NameCollisionOption, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage"))]
    SaveAsAsync: usize,
    #[cfg(feature = "Storage")]
    pub SaveAsWithFormatAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, super::super::Storage::NameCollisionOption, PlaylistFormat, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage"))]
    SaveAsWithFormatAsync: usize,
}
windows_core::imp::define_interface!(IPlaylistStatics, IPlaylistStatics_Vtbl, 0xc5c331cd_81f9_4ff3_95b9_70b6ff046b68);
impl windows_core::RuntimeType for IPlaylistStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPlaylistStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Storage")]
    pub LoadAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage"))]
    LoadAsync: usize,
}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct Playlist(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(Playlist, windows_core::IUnknown, windows_core::IInspectable);
impl Playlist {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<Playlist, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage"))]
    pub fn Files(&self) -> windows_core::Result<super::super::Foundation::Collections::IVector<super::super::Storage::StorageFile>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Files)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SaveAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SaveAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage")]
    pub fn SaveAsAsync<P0>(&self, savelocation: P0, desiredname: &windows_core::HSTRING, option: super::super::Storage::NameCollisionOption) -> windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Storage::StorageFile>>
    where
        P0: windows_core::Param<super::super::Storage::IStorageFolder>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SaveAsAsync)(windows_core::Interface::as_raw(this), savelocation.param().abi(), core::mem::transmute_copy(desiredname), option, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage")]
    pub fn SaveAsWithFormatAsync<P0>(&self, savelocation: P0, desiredname: &windows_core::HSTRING, option: super::super::Storage::NameCollisionOption, playlistformat: PlaylistFormat) -> windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Storage::StorageFile>>
    where
        P0: windows_core::Param<super::super::Storage::IStorageFolder>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SaveAsWithFormatAsync)(windows_core::Interface::as_raw(this), savelocation.param().abi(), core::mem::transmute_copy(desiredname), option, playlistformat, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage")]
    pub fn LoadAsync<P0>(file: P0) -> windows_core::Result<super::super::Foundation::IAsyncOperation<Playlist>>
    where
        P0: windows_core::Param<super::super::Storage::IStorageFile>,
    {
        Self::IPlaylistStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LoadAsync)(windows_core::Interface::as_raw(this), file.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IPlaylistStatics<R, F: FnOnce(&IPlaylistStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<Playlist, IPlaylistStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for Playlist {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPlaylist>();
}
unsafe impl windows_core::Interface for Playlist {
    type Vtable = IPlaylist_Vtbl;
    const IID: windows_core::GUID = <IPlaylist as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for Playlist {
    const NAME: &'static str = "Windows.Media.Playlists.Playlist";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PlaylistFormat(pub i32);
impl PlaylistFormat {
    pub const WindowsMedia: Self = Self(0i32);
    pub const Zune: Self = Self(1i32);
    pub const M3u: Self = Self(2i32);
}
impl windows_core::TypeKind for PlaylistFormat {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PlaylistFormat {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PlaylistFormat").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for PlaylistFormat {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Playlists.PlaylistFormat;i4)");
}
