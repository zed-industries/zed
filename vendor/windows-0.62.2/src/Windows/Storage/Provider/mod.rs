#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CachedFileOptions(pub u32);
impl CachedFileOptions {
    pub const None: Self = Self(0u32);
    pub const RequireUpdateOnAccess: Self = Self(1u32);
    pub const UseCachedFileWhenOffline: Self = Self(2u32);
    pub const DenyAccessWhenOffline: Self = Self(4u32);
}
impl windows_core::TypeKind for CachedFileOptions {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for CachedFileOptions {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Storage.Provider.CachedFileOptions;u4)");
}
impl CachedFileOptions {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for CachedFileOptions {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for CachedFileOptions {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for CachedFileOptions {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for CachedFileOptions {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for CachedFileOptions {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CachedFileTarget(pub i32);
impl CachedFileTarget {
    pub const Local: Self = Self(0i32);
    pub const Remote: Self = Self(1i32);
}
impl windows_core::TypeKind for CachedFileTarget {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for CachedFileTarget {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Storage.Provider.CachedFileTarget;i4)");
}
pub struct CachedFileUpdater;
impl CachedFileUpdater {
    #[cfg(feature = "Storage_Streams")]
    pub fn SetUpdateInformation<P0>(file: P0, contentid: &windows_core::HSTRING, readmode: ReadActivationMode, writemode: WriteActivationMode, options: CachedFileOptions) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::IStorageFile>,
    {
        Self::ICachedFileUpdaterStatics(|this| unsafe { (windows_core::Interface::vtable(this).SetUpdateInformation)(windows_core::Interface::as_raw(this), file.param().abi(), core::mem::transmute_copy(contentid), readmode, writemode, options).ok() })
    }
    fn ICachedFileUpdaterStatics<R, F: FnOnce(&ICachedFileUpdaterStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<CachedFileUpdater, ICachedFileUpdaterStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeName for CachedFileUpdater {
    const NAME: &'static str = "Windows.Storage.Provider.CachedFileUpdater";
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CachedFileUpdaterUI(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(CachedFileUpdaterUI, windows_core::IUnknown, windows_core::IInspectable);
impl CachedFileUpdaterUI {
    pub fn Title(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Title)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetTitle(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetTitle)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn UpdateTarget(&self) -> windows_core::Result<CachedFileTarget> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UpdateTarget)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn FileUpdateRequested<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<CachedFileUpdaterUI, FileUpdateRequestedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FileUpdateRequested)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveFileUpdateRequested(&self, token: i64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveFileUpdateRequested)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn UIRequested<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<CachedFileUpdaterUI, windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UIRequested)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveUIRequested(&self, token: i64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveUIRequested)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn UIStatus(&self) -> windows_core::Result<UIStatus> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UIStatus)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn UpdateRequest(&self) -> windows_core::Result<FileUpdateRequest> {
        let this = &windows_core::Interface::cast::<ICachedFileUpdaterUI2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UpdateRequest)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetDeferral(&self) -> windows_core::Result<FileUpdateRequestDeferral> {
        let this = &windows_core::Interface::cast::<ICachedFileUpdaterUI2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDeferral)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for CachedFileUpdaterUI {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ICachedFileUpdaterUI>();
}
unsafe impl windows_core::Interface for CachedFileUpdaterUI {
    type Vtable = <ICachedFileUpdaterUI as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ICachedFileUpdaterUI as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for CachedFileUpdaterUI {
    const NAME: &'static str = "Windows.Storage.Provider.CachedFileUpdaterUI";
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FileUpdateRequest(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(FileUpdateRequest, windows_core::IUnknown, windows_core::IInspectable);
impl FileUpdateRequest {
    pub fn ContentId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ContentId)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn File(&self) -> windows_core::Result<super::StorageFile> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).File)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Status(&self) -> windows_core::Result<FileUpdateStatus> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Status)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetStatus(&self, value: FileUpdateStatus) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetStatus)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn GetDeferral(&self) -> windows_core::Result<FileUpdateRequestDeferral> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDeferral)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn UpdateLocalFile<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::IStorageFile>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).UpdateLocalFile)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn UserInputNeededMessage(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IFileUpdateRequest2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UserInputNeededMessage)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetUserInputNeededMessage(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IFileUpdateRequest2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetUserInputNeededMessage)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
}
impl windows_core::RuntimeType for FileUpdateRequest {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IFileUpdateRequest>();
}
unsafe impl windows_core::Interface for FileUpdateRequest {
    type Vtable = <IFileUpdateRequest as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IFileUpdateRequest as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for FileUpdateRequest {
    const NAME: &'static str = "Windows.Storage.Provider.FileUpdateRequest";
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FileUpdateRequestDeferral(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(FileUpdateRequestDeferral, windows_core::IUnknown, windows_core::IInspectable);
impl FileUpdateRequestDeferral {
    pub fn Complete(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Complete)(windows_core::Interface::as_raw(this)).ok() }
    }
}
impl windows_core::RuntimeType for FileUpdateRequestDeferral {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IFileUpdateRequestDeferral>();
}
unsafe impl windows_core::Interface for FileUpdateRequestDeferral {
    type Vtable = <IFileUpdateRequestDeferral as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IFileUpdateRequestDeferral as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for FileUpdateRequestDeferral {
    const NAME: &'static str = "Windows.Storage.Provider.FileUpdateRequestDeferral";
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FileUpdateRequestedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(FileUpdateRequestedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl FileUpdateRequestedEventArgs {
    pub fn Request(&self) -> windows_core::Result<FileUpdateRequest> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Request)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for FileUpdateRequestedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IFileUpdateRequestedEventArgs>();
}
unsafe impl windows_core::Interface for FileUpdateRequestedEventArgs {
    type Vtable = <IFileUpdateRequestedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IFileUpdateRequestedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for FileUpdateRequestedEventArgs {
    const NAME: &'static str = "Windows.Storage.Provider.FileUpdateRequestedEventArgs";
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FileUpdateStatus(pub i32);
impl FileUpdateStatus {
    pub const Incomplete: Self = Self(0i32);
    pub const Complete: Self = Self(1i32);
    pub const UserInputNeeded: Self = Self(2i32);
    pub const CurrentlyUnavailable: Self = Self(3i32);
    pub const Failed: Self = Self(4i32);
    pub const CompleteAndRenamed: Self = Self(5i32);
}
impl windows_core::TypeKind for FileUpdateStatus {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for FileUpdateStatus {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Storage.Provider.FileUpdateStatus;i4)");
}
windows_core::imp::define_interface!(ICachedFileUpdaterStatics, ICachedFileUpdaterStatics_Vtbl, 0x9fc90920_7bcf_4888_a81e_102d7034d7ce);
impl windows_core::RuntimeType for ICachedFileUpdaterStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ICachedFileUpdaterStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Storage_Streams")]
    pub SetUpdateInformation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, ReadActivationMode, WriteActivationMode, CachedFileOptions) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetUpdateInformation: usize,
}
windows_core::imp::define_interface!(ICachedFileUpdaterUI, ICachedFileUpdaterUI_Vtbl, 0x9e6f41e6_baf2_4a97_b600_9333f5df80fd);
impl windows_core::RuntimeType for ICachedFileUpdaterUI {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ICachedFileUpdaterUI_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Title: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetTitle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub UpdateTarget: unsafe extern "system" fn(*mut core::ffi::c_void, *mut CachedFileTarget) -> windows_core::HRESULT,
    pub FileUpdateRequested: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveFileUpdateRequested: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub UIRequested: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveUIRequested: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub UIStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut UIStatus) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICachedFileUpdaterUI2, ICachedFileUpdaterUI2_Vtbl, 0x8856a21c_8699_4340_9f49_f7cad7fe8991);
impl windows_core::RuntimeType for ICachedFileUpdaterUI2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ICachedFileUpdaterUI2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub UpdateRequest: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDeferral: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IFileUpdateRequest, IFileUpdateRequest_Vtbl, 0x40c82536_c1fe_4d93_a792_1e736bc70837);
impl windows_core::RuntimeType for IFileUpdateRequest {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IFileUpdateRequest_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ContentId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub File: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    File: usize,
    pub Status: unsafe extern "system" fn(*mut core::ffi::c_void, *mut FileUpdateStatus) -> windows_core::HRESULT,
    pub SetStatus: unsafe extern "system" fn(*mut core::ffi::c_void, FileUpdateStatus) -> windows_core::HRESULT,
    pub GetDeferral: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub UpdateLocalFile: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    UpdateLocalFile: usize,
}
windows_core::imp::define_interface!(IFileUpdateRequest2, IFileUpdateRequest2_Vtbl, 0x82484648_bdbe_447b_a2ee_7afe6a032a94);
impl windows_core::RuntimeType for IFileUpdateRequest2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IFileUpdateRequest2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub UserInputNeededMessage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetUserInputNeededMessage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IFileUpdateRequestDeferral, IFileUpdateRequestDeferral_Vtbl, 0xffcedb2b_8ade_44a5_bb00_164c4e72f13a);
impl windows_core::RuntimeType for IFileUpdateRequestDeferral {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IFileUpdateRequestDeferral_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Complete: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IFileUpdateRequestedEventArgs, IFileUpdateRequestedEventArgs_Vtbl, 0x7b0a9342_3905_438d_aaef_78ae265f8dd2);
impl windows_core::RuntimeType for IFileUpdateRequestedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IFileUpdateRequestedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Request: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IStorageProviderFileTypeInfo, IStorageProviderFileTypeInfo_Vtbl, 0x1955b9c1_0184_5a88_87df_4544f464365d);
impl windows_core::RuntimeType for IStorageProviderFileTypeInfo {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageProviderFileTypeInfo_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub FileExtension: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IconResource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IStorageProviderFileTypeInfoFactory, IStorageProviderFileTypeInfoFactory_Vtbl, 0x3fa12c6f_cce6_5d5d_80b1_389e7cf92dbf);
impl windows_core::RuntimeType for IStorageProviderFileTypeInfoFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageProviderFileTypeInfoFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IStorageProviderGetContentInfoForPathResult, IStorageProviderGetContentInfoForPathResult_Vtbl, 0x2564711d_aa89_4d12_82e3_f72a92e33966);
impl windows_core::RuntimeType for IStorageProviderGetContentInfoForPathResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageProviderGetContentInfoForPathResult_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Status: unsafe extern "system" fn(*mut core::ffi::c_void, *mut StorageProviderUriSourceStatus) -> windows_core::HRESULT,
    pub SetStatus: unsafe extern "system" fn(*mut core::ffi::c_void, StorageProviderUriSourceStatus) -> windows_core::HRESULT,
    pub ContentUri: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetContentUri: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ContentId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetContentId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IStorageProviderGetPathForContentUriResult, IStorageProviderGetPathForContentUriResult_Vtbl, 0x63711a9d_4118_45a6_acb6_22c49d019f40);
impl windows_core::RuntimeType for IStorageProviderGetPathForContentUriResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageProviderGetPathForContentUriResult_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Status: unsafe extern "system" fn(*mut core::ffi::c_void, *mut StorageProviderUriSourceStatus) -> windows_core::HRESULT,
    pub SetStatus: unsafe extern "system" fn(*mut core::ffi::c_void, StorageProviderUriSourceStatus) -> windows_core::HRESULT,
    pub Path: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetPath: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IStorageProviderItemPropertiesStatics, IStorageProviderItemPropertiesStatics_Vtbl, 0x2d2c1c97_2704_4729_8fa9_7e6b8e158c2f);
impl windows_core::RuntimeType for IStorageProviderItemPropertiesStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageProviderItemPropertiesStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SetAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IStorageProviderItemProperty, IStorageProviderItemProperty_Vtbl, 0x476cb558_730b_4188_b7b5_63b716ed476d);
impl windows_core::RuntimeType for IStorageProviderItemProperty {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageProviderItemProperty_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SetId: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub Id: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetValue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Value: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetIconResource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IconResource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IStorageProviderItemPropertyDefinition, IStorageProviderItemPropertyDefinition_Vtbl, 0xc5b383bb_ff1f_4298_831e_ff1c08089690);
impl windows_core::RuntimeType for IStorageProviderItemPropertyDefinition {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageProviderItemPropertyDefinition_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Id: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetId: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub DisplayNameResource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetDisplayNameResource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IStorageProviderItemPropertySource, IStorageProviderItemPropertySource_Vtbl, 0x8f6f9c3e_f632_4a9b_8d99_d2d7a11df56a);
impl windows_core::RuntimeType for IStorageProviderItemPropertySource {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(IStorageProviderItemPropertySource, windows_core::IUnknown, windows_core::IInspectable);
impl IStorageProviderItemPropertySource {
    pub fn GetItemProperties(&self, itempath: &windows_core::HSTRING) -> windows_core::Result<windows_collections::IIterable<StorageProviderItemProperty>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetItemProperties)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(itempath), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeName for IStorageProviderItemPropertySource {
    const NAME: &'static str = "Windows.Storage.Provider.IStorageProviderItemPropertySource";
}
pub trait IStorageProviderItemPropertySource_Impl: windows_core::IUnknownImpl {
    fn GetItemProperties(&self, itemPath: &windows_core::HSTRING) -> windows_core::Result<windows_collections::IIterable<StorageProviderItemProperty>>;
}
impl IStorageProviderItemPropertySource_Vtbl {
    pub const fn new<Identity: IStorageProviderItemPropertySource_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetItemProperties<Identity: IStorageProviderItemPropertySource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, itempath: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IStorageProviderItemPropertySource_Impl::GetItemProperties(this, core::mem::transmute(&itempath)) {
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
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IStorageProviderItemPropertySource, OFFSET>(),
            GetItemProperties: GetItemProperties::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IStorageProviderItemPropertySource as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageProviderItemPropertySource_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetItemProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IStorageProviderKnownFolderEntry, IStorageProviderKnownFolderEntry_Vtbl, 0xeffa7db0_1d44_596b_8464_928800c5e2d8);
impl windows_core::RuntimeType for IStorageProviderKnownFolderEntry {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageProviderKnownFolderEntry_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub KnownFolderId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub SetKnownFolderId: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID) -> windows_core::HRESULT,
    pub Status: unsafe extern "system" fn(*mut core::ffi::c_void, *mut StorageProviderKnownFolderSyncStatus) -> windows_core::HRESULT,
    pub SetStatus: unsafe extern "system" fn(*mut core::ffi::c_void, StorageProviderKnownFolderSyncStatus) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IStorageProviderKnownFolderSyncInfo, IStorageProviderKnownFolderSyncInfo_Vtbl, 0x98b017ce_ffc1_5b11_ae77_cc17afec1049);
impl windows_core::RuntimeType for IStorageProviderKnownFolderSyncInfo {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageProviderKnownFolderSyncInfo_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ProviderDisplayName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetProviderDisplayName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub KnownFolderEntries: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SyncRequested: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetSyncRequested: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IStorageProviderKnownFolderSyncInfoSource, IStorageProviderKnownFolderSyncInfoSource_Vtbl, 0x51359342_f7c0_53d0_bbb6_1cdc098ebda9);
impl windows_core::RuntimeType for IStorageProviderKnownFolderSyncInfoSource {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(IStorageProviderKnownFolderSyncInfoSource, windows_core::IUnknown, windows_core::IInspectable);
impl IStorageProviderKnownFolderSyncInfoSource {
    pub fn GetKnownFolderSyncInfo(&self) -> windows_core::Result<StorageProviderKnownFolderSyncInfo> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetKnownFolderSyncInfo)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn KnownFolderSyncInfoChanged<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<IStorageProviderKnownFolderSyncInfoSource, windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).KnownFolderSyncInfoChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveKnownFolderSyncInfoChanged(&self, token: i64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveKnownFolderSyncInfoChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
}
impl windows_core::RuntimeName for IStorageProviderKnownFolderSyncInfoSource {
    const NAME: &'static str = "Windows.Storage.Provider.IStorageProviderKnownFolderSyncInfoSource";
}
pub trait IStorageProviderKnownFolderSyncInfoSource_Impl: windows_core::IUnknownImpl {
    fn GetKnownFolderSyncInfo(&self) -> windows_core::Result<StorageProviderKnownFolderSyncInfo>;
    fn KnownFolderSyncInfoChanged(&self, handler: windows_core::Ref<super::super::Foundation::TypedEventHandler<IStorageProviderKnownFolderSyncInfoSource, windows_core::IInspectable>>) -> windows_core::Result<i64>;
    fn RemoveKnownFolderSyncInfoChanged(&self, token: i64) -> windows_core::Result<()>;
}
impl IStorageProviderKnownFolderSyncInfoSource_Vtbl {
    pub const fn new<Identity: IStorageProviderKnownFolderSyncInfoSource_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetKnownFolderSyncInfo<Identity: IStorageProviderKnownFolderSyncInfoSource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IStorageProviderKnownFolderSyncInfoSource_Impl::GetKnownFolderSyncInfo(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn KnownFolderSyncInfoChanged<Identity: IStorageProviderKnownFolderSyncInfoSource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, handler: *mut core::ffi::c_void, result__: *mut i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IStorageProviderKnownFolderSyncInfoSource_Impl::KnownFolderSyncInfoChanged(this, core::mem::transmute_copy(&handler)) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RemoveKnownFolderSyncInfoChanged<Identity: IStorageProviderKnownFolderSyncInfoSource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, token: i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IStorageProviderKnownFolderSyncInfoSource_Impl::RemoveKnownFolderSyncInfoChanged(this, token).into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IStorageProviderKnownFolderSyncInfoSource, OFFSET>(),
            GetKnownFolderSyncInfo: GetKnownFolderSyncInfo::<Identity, OFFSET>,
            KnownFolderSyncInfoChanged: KnownFolderSyncInfoChanged::<Identity, OFFSET>,
            RemoveKnownFolderSyncInfoChanged: RemoveKnownFolderSyncInfoChanged::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IStorageProviderKnownFolderSyncInfoSource as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageProviderKnownFolderSyncInfoSource_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetKnownFolderSyncInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub KnownFolderSyncInfoChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveKnownFolderSyncInfoChanged: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IStorageProviderKnownFolderSyncInfoSourceFactory, IStorageProviderKnownFolderSyncInfoSourceFactory_Vtbl, 0xaaee03a7_a7f6_50be_a9b0_8e82d0c81082);
impl windows_core::RuntimeType for IStorageProviderKnownFolderSyncInfoSourceFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(IStorageProviderKnownFolderSyncInfoSourceFactory, windows_core::IUnknown, windows_core::IInspectable);
impl IStorageProviderKnownFolderSyncInfoSourceFactory {
    pub fn GetKnownFolderSyncInfoSource(&self) -> windows_core::Result<IStorageProviderKnownFolderSyncInfoSource> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetKnownFolderSyncInfoSource)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeName for IStorageProviderKnownFolderSyncInfoSourceFactory {
    const NAME: &'static str = "Windows.Storage.Provider.IStorageProviderKnownFolderSyncInfoSourceFactory";
}
pub trait IStorageProviderKnownFolderSyncInfoSourceFactory_Impl: windows_core::IUnknownImpl {
    fn GetKnownFolderSyncInfoSource(&self) -> windows_core::Result<IStorageProviderKnownFolderSyncInfoSource>;
}
impl IStorageProviderKnownFolderSyncInfoSourceFactory_Vtbl {
    pub const fn new<Identity: IStorageProviderKnownFolderSyncInfoSourceFactory_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetKnownFolderSyncInfoSource<Identity: IStorageProviderKnownFolderSyncInfoSourceFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IStorageProviderKnownFolderSyncInfoSourceFactory_Impl::GetKnownFolderSyncInfoSource(this) {
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
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IStorageProviderKnownFolderSyncInfoSourceFactory, OFFSET>(),
            GetKnownFolderSyncInfoSource: GetKnownFolderSyncInfoSource::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IStorageProviderKnownFolderSyncInfoSourceFactory as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageProviderKnownFolderSyncInfoSourceFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetKnownFolderSyncInfoSource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IStorageProviderKnownFolderSyncRequestArgs, IStorageProviderKnownFolderSyncRequestArgs_Vtbl, 0xeda6d569_b4e8_542f_ab8d_f3613f250a4a);
impl windows_core::RuntimeType for IStorageProviderKnownFolderSyncRequestArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageProviderKnownFolderSyncRequestArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub KnownFolders: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Storage_Search")]
    pub Source: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Search"))]
    Source: usize,
}
windows_core::imp::define_interface!(IStorageProviderMoreInfoUI, IStorageProviderMoreInfoUI_Vtbl, 0xef38e591_a7cb_5e7d_9b5e_22749842697c);
impl windows_core::RuntimeType for IStorageProviderMoreInfoUI {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageProviderMoreInfoUI_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Message: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetMessage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Command: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetCommand: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IStorageProviderPropertyCapabilities, IStorageProviderPropertyCapabilities_Vtbl, 0x658d2f0e_63b7_4567_acf9_51abe301dda5);
impl windows_core::RuntimeType for IStorageProviderPropertyCapabilities {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(IStorageProviderPropertyCapabilities, windows_core::IUnknown, windows_core::IInspectable);
impl IStorageProviderPropertyCapabilities {
    pub fn IsPropertySupported(&self, propertycanonicalname: &windows_core::HSTRING) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsPropertySupported)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(propertycanonicalname), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeName for IStorageProviderPropertyCapabilities {
    const NAME: &'static str = "Windows.Storage.Provider.IStorageProviderPropertyCapabilities";
}
pub trait IStorageProviderPropertyCapabilities_Impl: windows_core::IUnknownImpl {
    fn IsPropertySupported(&self, propertyCanonicalName: &windows_core::HSTRING) -> windows_core::Result<bool>;
}
impl IStorageProviderPropertyCapabilities_Vtbl {
    pub const fn new<Identity: IStorageProviderPropertyCapabilities_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn IsPropertySupported<Identity: IStorageProviderPropertyCapabilities_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, propertycanonicalname: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IStorageProviderPropertyCapabilities_Impl::IsPropertySupported(this, core::mem::transmute(&propertycanonicalname)) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IStorageProviderPropertyCapabilities, OFFSET>(),
            IsPropertySupported: IsPropertySupported::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IStorageProviderPropertyCapabilities as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageProviderPropertyCapabilities_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IsPropertySupported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IStorageProviderQueryResult, IStorageProviderQueryResult_Vtbl, 0xf1cd00ae_b4a9_5d20_a598_3eb4dd8ff8f4);
impl windows_core::RuntimeType for IStorageProviderQueryResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(IStorageProviderQueryResult, windows_core::IUnknown, windows_core::IInspectable);
impl IStorageProviderQueryResult {
    pub fn Kind(&self) -> windows_core::Result<StorageProviderResultKind> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetKind(&self, value: StorageProviderResultKind) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetKind)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ResultId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ResultId)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetResultId(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetResultId)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn RemoteFileId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RemoteFileId)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetRemoteFileId(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetRemoteFileId)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn FilePath(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FilePath)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetFilePath(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetFilePath)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn RequestedProperties(&self) -> windows_core::Result<super::super::Foundation::Collections::PropertySet> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RequestedProperties)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Foundation_Collections")]
impl windows_core::RuntimeName for IStorageProviderQueryResult {
    const NAME: &'static str = "Windows.Storage.Provider.IStorageProviderQueryResult";
}
#[cfg(feature = "Foundation_Collections")]
pub trait IStorageProviderQueryResult_Impl: windows_core::IUnknownImpl {
    fn Kind(&self) -> windows_core::Result<StorageProviderResultKind>;
    fn SetKind(&self, value: StorageProviderResultKind) -> windows_core::Result<()>;
    fn ResultId(&self) -> windows_core::Result<windows_core::HSTRING>;
    fn SetResultId(&self, value: &windows_core::HSTRING) -> windows_core::Result<()>;
    fn RemoteFileId(&self) -> windows_core::Result<windows_core::HSTRING>;
    fn SetRemoteFileId(&self, value: &windows_core::HSTRING) -> windows_core::Result<()>;
    fn FilePath(&self) -> windows_core::Result<windows_core::HSTRING>;
    fn SetFilePath(&self, value: &windows_core::HSTRING) -> windows_core::Result<()>;
    fn RequestedProperties(&self) -> windows_core::Result<super::super::Foundation::Collections::PropertySet>;
}
#[cfg(feature = "Foundation_Collections")]
impl IStorageProviderQueryResult_Vtbl {
    pub const fn new<Identity: IStorageProviderQueryResult_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Kind<Identity: IStorageProviderQueryResult_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut StorageProviderResultKind) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IStorageProviderQueryResult_Impl::Kind(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetKind<Identity: IStorageProviderQueryResult_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: StorageProviderResultKind) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IStorageProviderQueryResult_Impl::SetKind(this, value).into()
            }
        }
        unsafe extern "system" fn ResultId<Identity: IStorageProviderQueryResult_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IStorageProviderQueryResult_Impl::ResultId(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetResultId<Identity: IStorageProviderQueryResult_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IStorageProviderQueryResult_Impl::SetResultId(this, core::mem::transmute(&value)).into()
            }
        }
        unsafe extern "system" fn RemoteFileId<Identity: IStorageProviderQueryResult_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IStorageProviderQueryResult_Impl::RemoteFileId(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetRemoteFileId<Identity: IStorageProviderQueryResult_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IStorageProviderQueryResult_Impl::SetRemoteFileId(this, core::mem::transmute(&value)).into()
            }
        }
        unsafe extern "system" fn FilePath<Identity: IStorageProviderQueryResult_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IStorageProviderQueryResult_Impl::FilePath(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetFilePath<Identity: IStorageProviderQueryResult_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IStorageProviderQueryResult_Impl::SetFilePath(this, core::mem::transmute(&value)).into()
            }
        }
        unsafe extern "system" fn RequestedProperties<Identity: IStorageProviderQueryResult_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IStorageProviderQueryResult_Impl::RequestedProperties(this) {
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
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IStorageProviderQueryResult, OFFSET>(),
            Kind: Kind::<Identity, OFFSET>,
            SetKind: SetKind::<Identity, OFFSET>,
            ResultId: ResultId::<Identity, OFFSET>,
            SetResultId: SetResultId::<Identity, OFFSET>,
            RemoteFileId: RemoteFileId::<Identity, OFFSET>,
            SetRemoteFileId: SetRemoteFileId::<Identity, OFFSET>,
            FilePath: FilePath::<Identity, OFFSET>,
            SetFilePath: SetFilePath::<Identity, OFFSET>,
            RequestedProperties: RequestedProperties::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IStorageProviderQueryResult as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageProviderQueryResult_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Kind: unsafe extern "system" fn(*mut core::ffi::c_void, *mut StorageProviderResultKind) -> windows_core::HRESULT,
    pub SetKind: unsafe extern "system" fn(*mut core::ffi::c_void, StorageProviderResultKind) -> windows_core::HRESULT,
    pub ResultId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetResultId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RemoteFileId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetRemoteFileId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FilePath: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetFilePath: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub RequestedProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    RequestedProperties: usize,
}
windows_core::imp::define_interface!(IStorageProviderQueryResultSet, IStorageProviderQueryResultSet_Vtbl, 0x57c28407_7d21_5f98_ac52_0926a97f3259);
impl windows_core::RuntimeType for IStorageProviderQueryResultSet {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageProviderQueryResultSet_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetResults: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub QueryResultId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetQueryResultId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Status: unsafe extern "system" fn(*mut core::ffi::c_void, *mut StorageProviderSearchQueryStatus) -> windows_core::HRESULT,
    pub SetStatus: unsafe extern "system" fn(*mut core::ffi::c_void, StorageProviderSearchQueryStatus) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IStorageProviderQueryResultSetFactory, IStorageProviderQueryResultSetFactory_Vtbl, 0x301974c2_9b0a_51d1_84b5_32578ee3083d);
impl windows_core::RuntimeType for IStorageProviderQueryResultSetFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageProviderQueryResultSetFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const IStorageProviderQueryResult, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IStorageProviderQuotaUI, IStorageProviderQuotaUI_Vtbl, 0xba6295c3_312e_544f_9fd5_1f81b21f3649);
impl windows_core::RuntimeType for IStorageProviderQuotaUI {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageProviderQuotaUI_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub QuotaTotalInBytes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u64) -> windows_core::HRESULT,
    pub SetQuotaTotalInBytes: unsafe extern "system" fn(*mut core::ffi::c_void, u64) -> windows_core::HRESULT,
    pub QuotaUsedInBytes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u64) -> windows_core::HRESULT,
    pub SetQuotaUsedInBytes: unsafe extern "system" fn(*mut core::ffi::c_void, u64) -> windows_core::HRESULT,
    pub QuotaUsedLabel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetQuotaUsedLabel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "UI")]
    pub QuotaUsedColor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI"))]
    QuotaUsedColor: usize,
    #[cfg(feature = "UI")]
    pub SetQuotaUsedColor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI"))]
    SetQuotaUsedColor: usize,
}
windows_core::imp::define_interface!(IStorageProviderSearchHandler, IStorageProviderSearchHandler_Vtbl, 0x69cc977d_adad_59c9_8fd1_f30b6fae0fd9);
impl windows_core::RuntimeType for IStorageProviderSearchHandler {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(IStorageProviderSearchHandler, windows_core::IUnknown, windows_core::IInspectable);
impl IStorageProviderSearchHandler {
    pub fn Find<P0>(&self, options: P0) -> windows_core::Result<StorageProviderQueryResultSet>
    where
        P0: windows_core::Param<StorageProviderSearchQueryOptions>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Find)(windows_core::Interface::as_raw(this), options.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ReportUsage(&self, resultusagekind: StorageProviderResultUsageKind, remotefileid: &windows_core::HSTRING, resultid: &windows_core::HSTRING, latency: super::super::Foundation::TimeSpan) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).ReportUsage)(windows_core::Interface::as_raw(this), resultusagekind, core::mem::transmute_copy(remotefileid), core::mem::transmute_copy(resultid), latency).ok() }
    }
}
impl windows_core::RuntimeName for IStorageProviderSearchHandler {
    const NAME: &'static str = "Windows.Storage.Provider.IStorageProviderSearchHandler";
}
pub trait IStorageProviderSearchHandler_Impl: windows_core::IUnknownImpl {
    fn Find(&self, options: windows_core::Ref<StorageProviderSearchQueryOptions>) -> windows_core::Result<StorageProviderQueryResultSet>;
    fn ReportUsage(&self, resultUsageKind: StorageProviderResultUsageKind, remoteFileId: &windows_core::HSTRING, resultId: &windows_core::HSTRING, latency: &super::super::Foundation::TimeSpan) -> windows_core::Result<()>;
}
impl IStorageProviderSearchHandler_Vtbl {
    pub const fn new<Identity: IStorageProviderSearchHandler_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Find<Identity: IStorageProviderSearchHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, options: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IStorageProviderSearchHandler_Impl::Find(this, core::mem::transmute_copy(&options)) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ReportUsage<Identity: IStorageProviderSearchHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, resultusagekind: StorageProviderResultUsageKind, remotefileid: *mut core::ffi::c_void, resultid: *mut core::ffi::c_void, latency: super::super::Foundation::TimeSpan) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IStorageProviderSearchHandler_Impl::ReportUsage(this, resultusagekind, core::mem::transmute(&remotefileid), core::mem::transmute(&resultid), core::mem::transmute(&latency)).into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IStorageProviderSearchHandler, OFFSET>(),
            Find: Find::<Identity, OFFSET>,
            ReportUsage: ReportUsage::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IStorageProviderSearchHandler as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageProviderSearchHandler_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Find: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ReportUsage: unsafe extern "system" fn(*mut core::ffi::c_void, StorageProviderResultUsageKind, *mut core::ffi::c_void, *mut core::ffi::c_void, super::super::Foundation::TimeSpan) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IStorageProviderSearchHandlerFactory, IStorageProviderSearchHandlerFactory_Vtbl, 0xb0dcad80_f3f5_516b_8ace_4e77022c9598);
impl windows_core::RuntimeType for IStorageProviderSearchHandlerFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(IStorageProviderSearchHandlerFactory, windows_core::IUnknown, windows_core::IInspectable);
impl IStorageProviderSearchHandlerFactory {
    pub fn CreateSearchHandler(&self, cloudproviderid: &windows_core::HSTRING) -> windows_core::Result<IStorageProviderSearchHandler> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateSearchHandler)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(cloudproviderid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeName for IStorageProviderSearchHandlerFactory {
    const NAME: &'static str = "Windows.Storage.Provider.IStorageProviderSearchHandlerFactory";
}
pub trait IStorageProviderSearchHandlerFactory_Impl: windows_core::IUnknownImpl {
    fn CreateSearchHandler(&self, cloudProviderId: &windows_core::HSTRING) -> windows_core::Result<IStorageProviderSearchHandler>;
}
impl IStorageProviderSearchHandlerFactory_Vtbl {
    pub const fn new<Identity: IStorageProviderSearchHandlerFactory_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateSearchHandler<Identity: IStorageProviderSearchHandlerFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cloudproviderid: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IStorageProviderSearchHandlerFactory_Impl::CreateSearchHandler(this, core::mem::transmute(&cloudproviderid)) {
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
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IStorageProviderSearchHandlerFactory, OFFSET>(),
            CreateSearchHandler: CreateSearchHandler::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IStorageProviderSearchHandlerFactory as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageProviderSearchHandlerFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateSearchHandler: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IStorageProviderSearchQueryOptions, IStorageProviderSearchQueryOptions_Vtbl, 0x93d854eb_1007_563c_b213_cc44bd88fef2);
impl windows_core::RuntimeType for IStorageProviderSearchQueryOptions {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageProviderSearchQueryOptions_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub UserQuery: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Language: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Storage_Search")]
    pub SortOrder: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Search"))]
    SortOrder: usize,
    pub ProgrammaticQuery: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub MaxResults: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub FolderScope: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub QueryId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PropertiesToFetch: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IStorageProviderSearchResult, IStorageProviderSearchResult_Vtbl, 0xfc161049_0995_535f_99b7_fe292cbabaf5);
impl windows_core::RuntimeType for IStorageProviderSearchResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageProviderSearchResult_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub MatchScore: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub SetMatchScore: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub MatchKind: unsafe extern "system" fn(*mut core::ffi::c_void, *mut StorageProviderSearchMatchKind) -> windows_core::HRESULT,
    pub SetMatchKind: unsafe extern "system" fn(*mut core::ffi::c_void, StorageProviderSearchMatchKind) -> windows_core::HRESULT,
    pub MatchedPropertyName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetMatchedPropertyName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IStorageProviderShareLinkSource, IStorageProviderShareLinkSource_Vtbl, 0x4c6055e2_029c_5539_8e51_a1afc838b5cb);
impl windows_core::RuntimeType for IStorageProviderShareLinkSource {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(IStorageProviderShareLinkSource, windows_core::IUnknown, windows_core::IInspectable);
impl IStorageProviderShareLinkSource {
    pub fn CreateLinkAsync<P0>(&self, storageitemlist: P0) -> windows_core::Result<windows_future::IAsyncOperation<super::super::Foundation::Uri>>
    where
        P0: windows_core::Param<windows_collections::IVectorView<super::IStorageItem>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateLinkAsync)(windows_core::Interface::as_raw(this), storageitemlist.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetDefaultAccessControlStringAsync<P0>(&self, storageitemlist: P0) -> windows_core::Result<windows_future::IAsyncOperation<windows_core::HSTRING>>
    where
        P0: windows_core::Param<windows_collections::IVectorView<super::IStorageItem>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDefaultAccessControlStringAsync)(windows_core::Interface::as_raw(this), storageitemlist.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetState<P0>(&self, storageitemlist: P0) -> windows_core::Result<windows_future::IAsyncOperation<StorageProviderShareLinkState>>
    where
        P0: windows_core::Param<windows_collections::IVectorView<super::IStorageItem>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetState)(windows_core::Interface::as_raw(this), storageitemlist.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeName for IStorageProviderShareLinkSource {
    const NAME: &'static str = "Windows.Storage.Provider.IStorageProviderShareLinkSource";
}
pub trait IStorageProviderShareLinkSource_Impl: windows_core::IUnknownImpl {
    fn CreateLinkAsync(&self, storageItemList: windows_core::Ref<windows_collections::IVectorView<super::IStorageItem>>) -> windows_core::Result<windows_future::IAsyncOperation<super::super::Foundation::Uri>>;
    fn GetDefaultAccessControlStringAsync(&self, storageItemList: windows_core::Ref<windows_collections::IVectorView<super::IStorageItem>>) -> windows_core::Result<windows_future::IAsyncOperation<windows_core::HSTRING>>;
    fn GetState(&self, storageItemList: windows_core::Ref<windows_collections::IVectorView<super::IStorageItem>>) -> windows_core::Result<windows_future::IAsyncOperation<StorageProviderShareLinkState>>;
}
impl IStorageProviderShareLinkSource_Vtbl {
    pub const fn new<Identity: IStorageProviderShareLinkSource_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateLinkAsync<Identity: IStorageProviderShareLinkSource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, storageitemlist: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IStorageProviderShareLinkSource_Impl::CreateLinkAsync(this, core::mem::transmute_copy(&storageitemlist)) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetDefaultAccessControlStringAsync<Identity: IStorageProviderShareLinkSource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, storageitemlist: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IStorageProviderShareLinkSource_Impl::GetDefaultAccessControlStringAsync(this, core::mem::transmute_copy(&storageitemlist)) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetState<Identity: IStorageProviderShareLinkSource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, storageitemlist: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IStorageProviderShareLinkSource_Impl::GetState(this, core::mem::transmute_copy(&storageitemlist)) {
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
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IStorageProviderShareLinkSource, OFFSET>(),
            CreateLinkAsync: CreateLinkAsync::<Identity, OFFSET>,
            GetDefaultAccessControlStringAsync: GetDefaultAccessControlStringAsync::<Identity, OFFSET>,
            GetState: GetState::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IStorageProviderShareLinkSource as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageProviderShareLinkSource_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateLinkAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDefaultAccessControlStringAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IStorageProviderStatusUI, IStorageProviderStatusUI_Vtbl, 0xd6b6a758_198d_5b80_977f_5ff73da33118);
impl windows_core::RuntimeType for IStorageProviderStatusUI {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageProviderStatusUI_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ProviderState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut StorageProviderState) -> windows_core::HRESULT,
    pub SetProviderState: unsafe extern "system" fn(*mut core::ffi::c_void, StorageProviderState) -> windows_core::HRESULT,
    pub ProviderStateLabel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetProviderStateLabel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ProviderStateIcon: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetProviderStateIcon: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SyncStatusCommand: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetSyncStatusCommand: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub QuotaUI: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetQuotaUI: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub MoreInfoUI: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetMoreInfoUI: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ProviderPrimaryCommand: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetProviderPrimaryCommand: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ProviderSecondaryCommands: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetProviderSecondaryCommands: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IStorageProviderStatusUISource, IStorageProviderStatusUISource_Vtbl, 0xa306c249_3d66_5e70_9007_e43df96051ff);
impl windows_core::RuntimeType for IStorageProviderStatusUISource {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(IStorageProviderStatusUISource, windows_core::IUnknown, windows_core::IInspectable);
impl IStorageProviderStatusUISource {
    pub fn GetStatusUI(&self) -> windows_core::Result<StorageProviderStatusUI> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetStatusUI)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn StatusUIChanged<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<IStorageProviderStatusUISource, windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).StatusUIChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveStatusUIChanged(&self, token: i64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveStatusUIChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
}
impl windows_core::RuntimeName for IStorageProviderStatusUISource {
    const NAME: &'static str = "Windows.Storage.Provider.IStorageProviderStatusUISource";
}
pub trait IStorageProviderStatusUISource_Impl: windows_core::IUnknownImpl {
    fn GetStatusUI(&self) -> windows_core::Result<StorageProviderStatusUI>;
    fn StatusUIChanged(&self, handler: windows_core::Ref<super::super::Foundation::TypedEventHandler<IStorageProviderStatusUISource, windows_core::IInspectable>>) -> windows_core::Result<i64>;
    fn RemoveStatusUIChanged(&self, token: i64) -> windows_core::Result<()>;
}
impl IStorageProviderStatusUISource_Vtbl {
    pub const fn new<Identity: IStorageProviderStatusUISource_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetStatusUI<Identity: IStorageProviderStatusUISource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IStorageProviderStatusUISource_Impl::GetStatusUI(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn StatusUIChanged<Identity: IStorageProviderStatusUISource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, handler: *mut core::ffi::c_void, result__: *mut i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IStorageProviderStatusUISource_Impl::StatusUIChanged(this, core::mem::transmute_copy(&handler)) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RemoveStatusUIChanged<Identity: IStorageProviderStatusUISource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, token: i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IStorageProviderStatusUISource_Impl::RemoveStatusUIChanged(this, token).into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IStorageProviderStatusUISource, OFFSET>(),
            GetStatusUI: GetStatusUI::<Identity, OFFSET>,
            StatusUIChanged: StatusUIChanged::<Identity, OFFSET>,
            RemoveStatusUIChanged: RemoveStatusUIChanged::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IStorageProviderStatusUISource as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageProviderStatusUISource_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetStatusUI: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub StatusUIChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveStatusUIChanged: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IStorageProviderStatusUISourceFactory, IStorageProviderStatusUISourceFactory_Vtbl, 0x12e46b74_4e5a_58d1_a62f_0376e8ee7dd8);
impl windows_core::RuntimeType for IStorageProviderStatusUISourceFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(IStorageProviderStatusUISourceFactory, windows_core::IUnknown, windows_core::IInspectable);
impl IStorageProviderStatusUISourceFactory {
    pub fn GetStatusUISource(&self, syncrootid: &windows_core::HSTRING) -> windows_core::Result<IStorageProviderStatusUISource> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetStatusUISource)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(syncrootid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeName for IStorageProviderStatusUISourceFactory {
    const NAME: &'static str = "Windows.Storage.Provider.IStorageProviderStatusUISourceFactory";
}
pub trait IStorageProviderStatusUISourceFactory_Impl: windows_core::IUnknownImpl {
    fn GetStatusUISource(&self, syncRootId: &windows_core::HSTRING) -> windows_core::Result<IStorageProviderStatusUISource>;
}
impl IStorageProviderStatusUISourceFactory_Vtbl {
    pub const fn new<Identity: IStorageProviderStatusUISourceFactory_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetStatusUISource<Identity: IStorageProviderStatusUISourceFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, syncrootid: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IStorageProviderStatusUISourceFactory_Impl::GetStatusUISource(this, core::mem::transmute(&syncrootid)) {
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
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IStorageProviderStatusUISourceFactory, OFFSET>(),
            GetStatusUISource: GetStatusUISource::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IStorageProviderStatusUISourceFactory as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageProviderStatusUISourceFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetStatusUISource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IStorageProviderSuggestionsHandler, IStorageProviderSuggestionsHandler_Vtbl, 0xaff493f6_e1fd_5d03_b480_f1849c83ef4a);
impl windows_core::RuntimeType for IStorageProviderSuggestionsHandler {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(IStorageProviderSuggestionsHandler, windows_core::IUnknown, windows_core::IInspectable);
impl IStorageProviderSuggestionsHandler {
    pub fn GetSuggestions<P0>(&self, options: P0) -> windows_core::Result<StorageProviderQueryResultSet>
    where
        P0: windows_core::Param<StorageProviderSuggestionsQueryOptions>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetSuggestions)(windows_core::Interface::as_raw(this), options.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Add(&self, kind: StorageProviderResultKind, remotefileid: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Add)(windows_core::Interface::as_raw(this), kind, core::mem::transmute_copy(remotefileid)).ok() }
    }
    pub fn Remove(&self, kind: StorageProviderResultKind, remotefileid: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Remove)(windows_core::Interface::as_raw(this), kind, core::mem::transmute_copy(remotefileid)).ok() }
    }
    pub fn GetDetails(&self, remotefileid: &windows_core::HSTRING, propertiestofetch: &[windows_core::HSTRING], queryid: &windows_core::HSTRING) -> windows_core::Result<StorageProviderSuggestionResult> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDetails)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(remotefileid), propertiestofetch.len().try_into().unwrap(), core::mem::transmute(propertiestofetch.as_ptr()), core::mem::transmute_copy(queryid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ReportUsage(&self, resultusagekind: StorageProviderResultUsageKind, remotefileid: &windows_core::HSTRING, resultid: &windows_core::HSTRING, latency: super::super::Foundation::TimeSpan) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).ReportUsage)(windows_core::Interface::as_raw(this), resultusagekind, core::mem::transmute_copy(remotefileid), core::mem::transmute_copy(resultid), latency).ok() }
    }
}
impl windows_core::RuntimeName for IStorageProviderSuggestionsHandler {
    const NAME: &'static str = "Windows.Storage.Provider.IStorageProviderSuggestionsHandler";
}
pub trait IStorageProviderSuggestionsHandler_Impl: windows_core::IUnknownImpl {
    fn GetSuggestions(&self, options: windows_core::Ref<StorageProviderSuggestionsQueryOptions>) -> windows_core::Result<StorageProviderQueryResultSet>;
    fn Add(&self, kind: StorageProviderResultKind, remoteFileId: &windows_core::HSTRING) -> windows_core::Result<()>;
    fn Remove(&self, kind: StorageProviderResultKind, remoteFileId: &windows_core::HSTRING) -> windows_core::Result<()>;
    fn GetDetails(&self, remoteFileId: &windows_core::HSTRING, propertiesToFetch: &[windows_core::HSTRING], queryId: &windows_core::HSTRING) -> windows_core::Result<StorageProviderSuggestionResult>;
    fn ReportUsage(&self, resultUsageKind: StorageProviderResultUsageKind, remoteFileId: &windows_core::HSTRING, resultId: &windows_core::HSTRING, latency: &super::super::Foundation::TimeSpan) -> windows_core::Result<()>;
}
impl IStorageProviderSuggestionsHandler_Vtbl {
    pub const fn new<Identity: IStorageProviderSuggestionsHandler_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetSuggestions<Identity: IStorageProviderSuggestionsHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, options: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IStorageProviderSuggestionsHandler_Impl::GetSuggestions(this, core::mem::transmute_copy(&options)) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Add<Identity: IStorageProviderSuggestionsHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, kind: StorageProviderResultKind, remotefileid: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IStorageProviderSuggestionsHandler_Impl::Add(this, kind, core::mem::transmute(&remotefileid)).into()
            }
        }
        unsafe extern "system" fn Remove<Identity: IStorageProviderSuggestionsHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, kind: StorageProviderResultKind, remotefileid: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IStorageProviderSuggestionsHandler_Impl::Remove(this, kind, core::mem::transmute(&remotefileid)).into()
            }
        }
        unsafe extern "system" fn GetDetails<Identity: IStorageProviderSuggestionsHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, remotefileid: *mut core::ffi::c_void, propertiestofetch_array_size: u32, propertiestofetch: *const windows_core::HSTRING, queryid: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IStorageProviderSuggestionsHandler_Impl::GetDetails(this, core::mem::transmute(&remotefileid), core::slice::from_raw_parts(core::mem::transmute_copy(&propertiestofetch), propertiestofetch_array_size as usize), core::mem::transmute(&queryid)) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ReportUsage<Identity: IStorageProviderSuggestionsHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, resultusagekind: StorageProviderResultUsageKind, remotefileid: *mut core::ffi::c_void, resultid: *mut core::ffi::c_void, latency: super::super::Foundation::TimeSpan) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IStorageProviderSuggestionsHandler_Impl::ReportUsage(this, resultusagekind, core::mem::transmute(&remotefileid), core::mem::transmute(&resultid), core::mem::transmute(&latency)).into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IStorageProviderSuggestionsHandler, OFFSET>(),
            GetSuggestions: GetSuggestions::<Identity, OFFSET>,
            Add: Add::<Identity, OFFSET>,
            Remove: Remove::<Identity, OFFSET>,
            GetDetails: GetDetails::<Identity, OFFSET>,
            ReportUsage: ReportUsage::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IStorageProviderSuggestionsHandler as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageProviderSuggestionsHandler_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetSuggestions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Add: unsafe extern "system" fn(*mut core::ffi::c_void, StorageProviderResultKind, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Remove: unsafe extern "system" fn(*mut core::ffi::c_void, StorageProviderResultKind, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDetails: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, *const windows_core::HSTRING, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ReportUsage: unsafe extern "system" fn(*mut core::ffi::c_void, StorageProviderResultUsageKind, *mut core::ffi::c_void, *mut core::ffi::c_void, super::super::Foundation::TimeSpan) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IStorageProviderSuggestionsHandlerFactory, IStorageProviderSuggestionsHandlerFactory_Vtbl, 0xdc7b35d8_a25b_58a3_ace7_b3543106a2aa);
impl windows_core::RuntimeType for IStorageProviderSuggestionsHandlerFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(IStorageProviderSuggestionsHandlerFactory, windows_core::IUnknown, windows_core::IInspectable);
impl IStorageProviderSuggestionsHandlerFactory {
    pub fn CreateSuggestionsHandler(&self, cloudproviderid: &windows_core::HSTRING) -> windows_core::Result<IStorageProviderSuggestionsHandler> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateSuggestionsHandler)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(cloudproviderid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeName for IStorageProviderSuggestionsHandlerFactory {
    const NAME: &'static str = "Windows.Storage.Provider.IStorageProviderSuggestionsHandlerFactory";
}
pub trait IStorageProviderSuggestionsHandlerFactory_Impl: windows_core::IUnknownImpl {
    fn CreateSuggestionsHandler(&self, cloudProviderId: &windows_core::HSTRING) -> windows_core::Result<IStorageProviderSuggestionsHandler>;
}
impl IStorageProviderSuggestionsHandlerFactory_Vtbl {
    pub const fn new<Identity: IStorageProviderSuggestionsHandlerFactory_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateSuggestionsHandler<Identity: IStorageProviderSuggestionsHandlerFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cloudproviderid: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IStorageProviderSuggestionsHandlerFactory_Impl::CreateSuggestionsHandler(this, core::mem::transmute(&cloudproviderid)) {
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
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IStorageProviderSuggestionsHandlerFactory, OFFSET>(),
            CreateSuggestionsHandler: CreateSuggestionsHandler::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IStorageProviderSuggestionsHandlerFactory as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageProviderSuggestionsHandlerFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateSuggestionsHandler: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IStorageProviderSuggestionsQueryOptions, IStorageProviderSuggestionsQueryOptions_Vtbl, 0xefb8b74d_0d84_579c_b137_ea730635d9bb);
impl windows_core::RuntimeType for IStorageProviderSuggestionsQueryOptions {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageProviderSuggestionsQueryOptions_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SuggestionsKind: unsafe extern "system" fn(*mut core::ffi::c_void, *mut StorageProviderResultKind) -> windows_core::HRESULT,
    pub RemoteFileId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub MaxResults: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub QueryId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PropertiesToFetch: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IStorageProviderSyncRootInfo, IStorageProviderSyncRootInfo_Vtbl, 0x7c1305c4_99f9_41ac_8904_ab055d654926);
impl windows_core::RuntimeType for IStorageProviderSyncRootInfo {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageProviderSyncRootInfo_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Id: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub Context: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Context: usize,
    #[cfg(feature = "Storage_Streams")]
    pub SetContext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetContext: usize,
    pub Path: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetPath: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DisplayNameResource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetDisplayNameResource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IconResource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetIconResource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub HydrationPolicy: unsafe extern "system" fn(*mut core::ffi::c_void, *mut StorageProviderHydrationPolicy) -> windows_core::HRESULT,
    pub SetHydrationPolicy: unsafe extern "system" fn(*mut core::ffi::c_void, StorageProviderHydrationPolicy) -> windows_core::HRESULT,
    pub HydrationPolicyModifier: unsafe extern "system" fn(*mut core::ffi::c_void, *mut StorageProviderHydrationPolicyModifier) -> windows_core::HRESULT,
    pub SetHydrationPolicyModifier: unsafe extern "system" fn(*mut core::ffi::c_void, StorageProviderHydrationPolicyModifier) -> windows_core::HRESULT,
    pub PopulationPolicy: unsafe extern "system" fn(*mut core::ffi::c_void, *mut StorageProviderPopulationPolicy) -> windows_core::HRESULT,
    pub SetPopulationPolicy: unsafe extern "system" fn(*mut core::ffi::c_void, StorageProviderPopulationPolicy) -> windows_core::HRESULT,
    pub InSyncPolicy: unsafe extern "system" fn(*mut core::ffi::c_void, *mut StorageProviderInSyncPolicy) -> windows_core::HRESULT,
    pub SetInSyncPolicy: unsafe extern "system" fn(*mut core::ffi::c_void, StorageProviderInSyncPolicy) -> windows_core::HRESULT,
    pub HardlinkPolicy: unsafe extern "system" fn(*mut core::ffi::c_void, *mut StorageProviderHardlinkPolicy) -> windows_core::HRESULT,
    pub SetHardlinkPolicy: unsafe extern "system" fn(*mut core::ffi::c_void, StorageProviderHardlinkPolicy) -> windows_core::HRESULT,
    pub ShowSiblingsAsGroup: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetShowSiblingsAsGroup: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub Version: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetVersion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ProtectionMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut StorageProviderProtectionMode) -> windows_core::HRESULT,
    pub SetProtectionMode: unsafe extern "system" fn(*mut core::ffi::c_void, StorageProviderProtectionMode) -> windows_core::HRESULT,
    pub AllowPinning: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetAllowPinning: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub StorageProviderItemPropertyDefinitions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RecycleBinUri: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetRecycleBinUri: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IStorageProviderSyncRootInfo2, IStorageProviderSyncRootInfo2_Vtbl, 0xcf51b023_7cf1_5166_bdba_efd95f529e31);
impl windows_core::RuntimeType for IStorageProviderSyncRootInfo2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageProviderSyncRootInfo2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ProviderId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub SetProviderId: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IStorageProviderSyncRootInfo3, IStorageProviderSyncRootInfo3_Vtbl, 0x507a6617_bef6_56fd_855e_75ace2e45cf5);
impl windows_core::RuntimeType for IStorageProviderSyncRootInfo3 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageProviderSyncRootInfo3_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub FallbackFileTypeInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IStorageProviderSyncRootManagerStatics, IStorageProviderSyncRootManagerStatics_Vtbl, 0x3e99fbbf_8fe3_4b40_abc7_f6fc3d74c98e);
impl windows_core::RuntimeType for IStorageProviderSyncRootManagerStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageProviderSyncRootManagerStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Register: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Unregister: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetSyncRootInformationForFolder: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetSyncRootInformationForId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetCurrentSyncRoots: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IStorageProviderSyncRootManagerStatics2, IStorageProviderSyncRootManagerStatics2_Vtbl, 0xefb6cfee_1374_544e_9df1_5598d2e9cfdd);
impl windows_core::RuntimeType for IStorageProviderSyncRootManagerStatics2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageProviderSyncRootManagerStatics2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IsSupported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IStorageProviderUICommand, IStorageProviderUICommand_Vtbl, 0x0c3e0760_d846_568f_9484_105cc57b502b);
impl windows_core::RuntimeType for IStorageProviderUICommand {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(IStorageProviderUICommand, windows_core::IUnknown, windows_core::IInspectable);
impl IStorageProviderUICommand {
    pub fn Label(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Label)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn Description(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Description)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn Icon(&self) -> windows_core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Icon)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn State(&self) -> windows_core::Result<StorageProviderUICommandState> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).State)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Invoke(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Invoke)(windows_core::Interface::as_raw(this)).ok() }
    }
}
impl windows_core::RuntimeName for IStorageProviderUICommand {
    const NAME: &'static str = "Windows.Storage.Provider.IStorageProviderUICommand";
}
pub trait IStorageProviderUICommand_Impl: windows_core::IUnknownImpl {
    fn Label(&self) -> windows_core::Result<windows_core::HSTRING>;
    fn Description(&self) -> windows_core::Result<windows_core::HSTRING>;
    fn Icon(&self) -> windows_core::Result<super::super::Foundation::Uri>;
    fn State(&self) -> windows_core::Result<StorageProviderUICommandState>;
    fn Invoke(&self) -> windows_core::Result<()>;
}
impl IStorageProviderUICommand_Vtbl {
    pub const fn new<Identity: IStorageProviderUICommand_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Label<Identity: IStorageProviderUICommand_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IStorageProviderUICommand_Impl::Label(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Description<Identity: IStorageProviderUICommand_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IStorageProviderUICommand_Impl::Description(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Icon<Identity: IStorageProviderUICommand_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IStorageProviderUICommand_Impl::Icon(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn State<Identity: IStorageProviderUICommand_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut StorageProviderUICommandState) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IStorageProviderUICommand_Impl::State(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Invoke<Identity: IStorageProviderUICommand_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IStorageProviderUICommand_Impl::Invoke(this).into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IStorageProviderUICommand, OFFSET>(),
            Label: Label::<Identity, OFFSET>,
            Description: Description::<Identity, OFFSET>,
            Icon: Icon::<Identity, OFFSET>,
            State: State::<Identity, OFFSET>,
            Invoke: Invoke::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IStorageProviderUICommand as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageProviderUICommand_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Label: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Description: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Icon: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub State: unsafe extern "system" fn(*mut core::ffi::c_void, *mut StorageProviderUICommandState) -> windows_core::HRESULT,
    pub Invoke: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IStorageProviderUriSource, IStorageProviderUriSource_Vtbl, 0xb29806d1_8be0_4962_8bb6_0d4c2e14d47a);
impl windows_core::RuntimeType for IStorageProviderUriSource {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(IStorageProviderUriSource, windows_core::IUnknown, windows_core::IInspectable);
impl IStorageProviderUriSource {
    pub fn GetPathForContentUri<P1>(&self, contenturi: &windows_core::HSTRING, result: P1) -> windows_core::Result<()>
    where
        P1: windows_core::Param<StorageProviderGetPathForContentUriResult>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).GetPathForContentUri)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(contenturi), result.param().abi()).ok() }
    }
    pub fn GetContentInfoForPath<P1>(&self, path: &windows_core::HSTRING, result: P1) -> windows_core::Result<()>
    where
        P1: windows_core::Param<StorageProviderGetContentInfoForPathResult>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).GetContentInfoForPath)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(path), result.param().abi()).ok() }
    }
}
impl windows_core::RuntimeName for IStorageProviderUriSource {
    const NAME: &'static str = "Windows.Storage.Provider.IStorageProviderUriSource";
}
pub trait IStorageProviderUriSource_Impl: windows_core::IUnknownImpl {
    fn GetPathForContentUri(&self, contentUri: &windows_core::HSTRING, result: windows_core::Ref<StorageProviderGetPathForContentUriResult>) -> windows_core::Result<()>;
    fn GetContentInfoForPath(&self, path: &windows_core::HSTRING, result: windows_core::Ref<StorageProviderGetContentInfoForPathResult>) -> windows_core::Result<()>;
}
impl IStorageProviderUriSource_Vtbl {
    pub const fn new<Identity: IStorageProviderUriSource_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetPathForContentUri<Identity: IStorageProviderUriSource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, contenturi: *mut core::ffi::c_void, result: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IStorageProviderUriSource_Impl::GetPathForContentUri(this, core::mem::transmute(&contenturi), core::mem::transmute_copy(&result)).into()
            }
        }
        unsafe extern "system" fn GetContentInfoForPath<Identity: IStorageProviderUriSource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, path: *mut core::ffi::c_void, result: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IStorageProviderUriSource_Impl::GetContentInfoForPath(this, core::mem::transmute(&path), core::mem::transmute_copy(&result)).into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IStorageProviderUriSource, OFFSET>(),
            GetPathForContentUri: GetPathForContentUri::<Identity, OFFSET>,
            GetContentInfoForPath: GetContentInfoForPath::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IStorageProviderUriSource as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageProviderUriSource_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetPathForContentUri: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetContentInfoForPath: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ReadActivationMode(pub i32);
impl ReadActivationMode {
    pub const NotNeeded: Self = Self(0i32);
    pub const BeforeAccess: Self = Self(1i32);
}
impl windows_core::TypeKind for ReadActivationMode {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for ReadActivationMode {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Storage.Provider.ReadActivationMode;i4)");
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StorageProviderFileTypeInfo(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(StorageProviderFileTypeInfo, windows_core::IUnknown, windows_core::IInspectable);
impl StorageProviderFileTypeInfo {
    pub fn FileExtension(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FileExtension)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn IconResource(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IconResource)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn CreateInstance(fileextension: &windows_core::HSTRING, iconresource: &windows_core::HSTRING) -> windows_core::Result<StorageProviderFileTypeInfo> {
        Self::IStorageProviderFileTypeInfoFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(fileextension), core::mem::transmute_copy(iconresource), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    fn IStorageProviderFileTypeInfoFactory<R, F: FnOnce(&IStorageProviderFileTypeInfoFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<StorageProviderFileTypeInfo, IStorageProviderFileTypeInfoFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for StorageProviderFileTypeInfo {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IStorageProviderFileTypeInfo>();
}
unsafe impl windows_core::Interface for StorageProviderFileTypeInfo {
    type Vtable = <IStorageProviderFileTypeInfo as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IStorageProviderFileTypeInfo as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for StorageProviderFileTypeInfo {
    const NAME: &'static str = "Windows.Storage.Provider.StorageProviderFileTypeInfo";
}
unsafe impl Send for StorageProviderFileTypeInfo {}
unsafe impl Sync for StorageProviderFileTypeInfo {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StorageProviderGetContentInfoForPathResult(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(StorageProviderGetContentInfoForPathResult, windows_core::IUnknown, windows_core::IInspectable);
impl StorageProviderGetContentInfoForPathResult {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<StorageProviderGetContentInfoForPathResult, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Status(&self) -> windows_core::Result<StorageProviderUriSourceStatus> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Status)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetStatus(&self, value: StorageProviderUriSourceStatus) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetStatus)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ContentUri(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ContentUri)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetContentUri(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetContentUri)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn ContentId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ContentId)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetContentId(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetContentId)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
}
impl windows_core::RuntimeType for StorageProviderGetContentInfoForPathResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IStorageProviderGetContentInfoForPathResult>();
}
unsafe impl windows_core::Interface for StorageProviderGetContentInfoForPathResult {
    type Vtable = <IStorageProviderGetContentInfoForPathResult as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IStorageProviderGetContentInfoForPathResult as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for StorageProviderGetContentInfoForPathResult {
    const NAME: &'static str = "Windows.Storage.Provider.StorageProviderGetContentInfoForPathResult";
}
unsafe impl Send for StorageProviderGetContentInfoForPathResult {}
unsafe impl Sync for StorageProviderGetContentInfoForPathResult {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StorageProviderGetPathForContentUriResult(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(StorageProviderGetPathForContentUriResult, windows_core::IUnknown, windows_core::IInspectable);
impl StorageProviderGetPathForContentUriResult {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<StorageProviderGetPathForContentUriResult, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Status(&self) -> windows_core::Result<StorageProviderUriSourceStatus> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Status)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetStatus(&self, value: StorageProviderUriSourceStatus) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetStatus)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Path(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Path)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetPath(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetPath)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
}
impl windows_core::RuntimeType for StorageProviderGetPathForContentUriResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IStorageProviderGetPathForContentUriResult>();
}
unsafe impl windows_core::Interface for StorageProviderGetPathForContentUriResult {
    type Vtable = <IStorageProviderGetPathForContentUriResult as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IStorageProviderGetPathForContentUriResult as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for StorageProviderGetPathForContentUriResult {
    const NAME: &'static str = "Windows.Storage.Provider.StorageProviderGetPathForContentUriResult";
}
unsafe impl Send for StorageProviderGetPathForContentUriResult {}
unsafe impl Sync for StorageProviderGetPathForContentUriResult {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct StorageProviderHardlinkPolicy(pub u32);
impl StorageProviderHardlinkPolicy {
    pub const None: Self = Self(0u32);
    pub const Allowed: Self = Self(1u32);
}
impl windows_core::TypeKind for StorageProviderHardlinkPolicy {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for StorageProviderHardlinkPolicy {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Storage.Provider.StorageProviderHardlinkPolicy;u4)");
}
impl StorageProviderHardlinkPolicy {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for StorageProviderHardlinkPolicy {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for StorageProviderHardlinkPolicy {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for StorageProviderHardlinkPolicy {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for StorageProviderHardlinkPolicy {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for StorageProviderHardlinkPolicy {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct StorageProviderHydrationPolicy(pub i32);
impl StorageProviderHydrationPolicy {
    pub const Partial: Self = Self(0i32);
    pub const Progressive: Self = Self(1i32);
    pub const Full: Self = Self(2i32);
    pub const AlwaysFull: Self = Self(3i32);
}
impl windows_core::TypeKind for StorageProviderHydrationPolicy {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for StorageProviderHydrationPolicy {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Storage.Provider.StorageProviderHydrationPolicy;i4)");
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct StorageProviderHydrationPolicyModifier(pub u32);
impl StorageProviderHydrationPolicyModifier {
    pub const None: Self = Self(0u32);
    pub const ValidationRequired: Self = Self(1u32);
    pub const StreamingAllowed: Self = Self(2u32);
    pub const AutoDehydrationAllowed: Self = Self(4u32);
    pub const AllowFullRestartHydration: Self = Self(8u32);
}
impl windows_core::TypeKind for StorageProviderHydrationPolicyModifier {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for StorageProviderHydrationPolicyModifier {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Storage.Provider.StorageProviderHydrationPolicyModifier;u4)");
}
impl StorageProviderHydrationPolicyModifier {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for StorageProviderHydrationPolicyModifier {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for StorageProviderHydrationPolicyModifier {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for StorageProviderHydrationPolicyModifier {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for StorageProviderHydrationPolicyModifier {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for StorageProviderHydrationPolicyModifier {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct StorageProviderInSyncPolicy(pub u32);
impl StorageProviderInSyncPolicy {
    pub const Default: Self = Self(0u32);
    pub const FileCreationTime: Self = Self(1u32);
    pub const FileReadOnlyAttribute: Self = Self(2u32);
    pub const FileHiddenAttribute: Self = Self(4u32);
    pub const FileSystemAttribute: Self = Self(8u32);
    pub const DirectoryCreationTime: Self = Self(16u32);
    pub const DirectoryReadOnlyAttribute: Self = Self(32u32);
    pub const DirectoryHiddenAttribute: Self = Self(64u32);
    pub const DirectorySystemAttribute: Self = Self(128u32);
    pub const FileLastWriteTime: Self = Self(256u32);
    pub const DirectoryLastWriteTime: Self = Self(512u32);
    pub const PreserveInsyncForSyncEngine: Self = Self(2147483648u32);
}
impl windows_core::TypeKind for StorageProviderInSyncPolicy {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for StorageProviderInSyncPolicy {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Storage.Provider.StorageProviderInSyncPolicy;u4)");
}
impl StorageProviderInSyncPolicy {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for StorageProviderInSyncPolicy {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for StorageProviderInSyncPolicy {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for StorageProviderInSyncPolicy {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for StorageProviderInSyncPolicy {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for StorageProviderInSyncPolicy {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub struct StorageProviderItemProperties;
impl StorageProviderItemProperties {
    pub fn SetAsync<P0, P1>(item: P0, itemproperties: P1) -> windows_core::Result<windows_future::IAsyncAction>
    where
        P0: windows_core::Param<super::IStorageItem>,
        P1: windows_core::Param<windows_collections::IIterable<StorageProviderItemProperty>>,
    {
        Self::IStorageProviderItemPropertiesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SetAsync)(windows_core::Interface::as_raw(this), item.param().abi(), itemproperties.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    fn IStorageProviderItemPropertiesStatics<R, F: FnOnce(&IStorageProviderItemPropertiesStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<StorageProviderItemProperties, IStorageProviderItemPropertiesStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeName for StorageProviderItemProperties {
    const NAME: &'static str = "Windows.Storage.Provider.StorageProviderItemProperties";
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StorageProviderItemProperty(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(StorageProviderItemProperty, windows_core::IUnknown, windows_core::IInspectable);
impl StorageProviderItemProperty {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<StorageProviderItemProperty, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn SetId(&self, value: i32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetId)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Id(&self) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Id)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetValue(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetValue)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn Value(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Value)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetIconResource(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetIconResource)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn IconResource(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IconResource)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
impl windows_core::RuntimeType for StorageProviderItemProperty {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IStorageProviderItemProperty>();
}
unsafe impl windows_core::Interface for StorageProviderItemProperty {
    type Vtable = <IStorageProviderItemProperty as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IStorageProviderItemProperty as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for StorageProviderItemProperty {
    const NAME: &'static str = "Windows.Storage.Provider.StorageProviderItemProperty";
}
unsafe impl Send for StorageProviderItemProperty {}
unsafe impl Sync for StorageProviderItemProperty {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StorageProviderItemPropertyDefinition(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(StorageProviderItemPropertyDefinition, windows_core::IUnknown, windows_core::IInspectable);
impl StorageProviderItemPropertyDefinition {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<StorageProviderItemPropertyDefinition, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Id(&self) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Id)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetId(&self, value: i32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetId)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn DisplayNameResource(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DisplayNameResource)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetDisplayNameResource(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetDisplayNameResource)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
}
impl windows_core::RuntimeType for StorageProviderItemPropertyDefinition {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IStorageProviderItemPropertyDefinition>();
}
unsafe impl windows_core::Interface for StorageProviderItemPropertyDefinition {
    type Vtable = <IStorageProviderItemPropertyDefinition as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IStorageProviderItemPropertyDefinition as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for StorageProviderItemPropertyDefinition {
    const NAME: &'static str = "Windows.Storage.Provider.StorageProviderItemPropertyDefinition";
}
unsafe impl Send for StorageProviderItemPropertyDefinition {}
unsafe impl Sync for StorageProviderItemPropertyDefinition {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StorageProviderKnownFolderEntry(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(StorageProviderKnownFolderEntry, windows_core::IUnknown, windows_core::IInspectable);
impl StorageProviderKnownFolderEntry {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<StorageProviderKnownFolderEntry, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn KnownFolderId(&self) -> windows_core::Result<windows_core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).KnownFolderId)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetKnownFolderId(&self, value: windows_core::GUID) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetKnownFolderId)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Status(&self) -> windows_core::Result<StorageProviderKnownFolderSyncStatus> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Status)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetStatus(&self, value: StorageProviderKnownFolderSyncStatus) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetStatus)(windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl windows_core::RuntimeType for StorageProviderKnownFolderEntry {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IStorageProviderKnownFolderEntry>();
}
unsafe impl windows_core::Interface for StorageProviderKnownFolderEntry {
    type Vtable = <IStorageProviderKnownFolderEntry as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IStorageProviderKnownFolderEntry as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for StorageProviderKnownFolderEntry {
    const NAME: &'static str = "Windows.Storage.Provider.StorageProviderKnownFolderEntry";
}
unsafe impl Send for StorageProviderKnownFolderEntry {}
unsafe impl Sync for StorageProviderKnownFolderEntry {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StorageProviderKnownFolderSyncInfo(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(StorageProviderKnownFolderSyncInfo, windows_core::IUnknown, windows_core::IInspectable);
impl StorageProviderKnownFolderSyncInfo {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<StorageProviderKnownFolderSyncInfo, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn ProviderDisplayName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ProviderDisplayName)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetProviderDisplayName(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetProviderDisplayName)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn KnownFolderEntries(&self) -> windows_core::Result<windows_collections::IVector<StorageProviderKnownFolderEntry>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).KnownFolderEntries)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SyncRequested(&self) -> windows_core::Result<StorageProviderKnownFolderSyncRequestedHandler> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SyncRequested)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetSyncRequested<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<StorageProviderKnownFolderSyncRequestedHandler>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetSyncRequested)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
}
impl windows_core::RuntimeType for StorageProviderKnownFolderSyncInfo {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IStorageProviderKnownFolderSyncInfo>();
}
unsafe impl windows_core::Interface for StorageProviderKnownFolderSyncInfo {
    type Vtable = <IStorageProviderKnownFolderSyncInfo as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IStorageProviderKnownFolderSyncInfo as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for StorageProviderKnownFolderSyncInfo {
    const NAME: &'static str = "Windows.Storage.Provider.StorageProviderKnownFolderSyncInfo";
}
unsafe impl Send for StorageProviderKnownFolderSyncInfo {}
unsafe impl Sync for StorageProviderKnownFolderSyncInfo {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StorageProviderKnownFolderSyncRequestArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(StorageProviderKnownFolderSyncRequestArgs, windows_core::IUnknown, windows_core::IInspectable);
impl StorageProviderKnownFolderSyncRequestArgs {
    pub fn KnownFolders(&self) -> windows_core::Result<windows_collections::IVectorView<windows_core::GUID>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).KnownFolders)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage_Search")]
    pub fn Source(&self) -> windows_core::Result<super::StorageFolder> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Source)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for StorageProviderKnownFolderSyncRequestArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IStorageProviderKnownFolderSyncRequestArgs>();
}
unsafe impl windows_core::Interface for StorageProviderKnownFolderSyncRequestArgs {
    type Vtable = <IStorageProviderKnownFolderSyncRequestArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IStorageProviderKnownFolderSyncRequestArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for StorageProviderKnownFolderSyncRequestArgs {
    const NAME: &'static str = "Windows.Storage.Provider.StorageProviderKnownFolderSyncRequestArgs";
}
unsafe impl Send for StorageProviderKnownFolderSyncRequestArgs {}
unsafe impl Sync for StorageProviderKnownFolderSyncRequestArgs {}
windows_core::imp::define_interface!(StorageProviderKnownFolderSyncRequestedHandler, StorageProviderKnownFolderSyncRequestedHandler_Vtbl, 0xc4cbb4f5_13dd_5c8e_8b96_336fc30c629b);
impl windows_core::RuntimeType for StorageProviderKnownFolderSyncRequestedHandler {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl StorageProviderKnownFolderSyncRequestedHandler {
    pub fn new<F: Fn(windows_core::Ref<StorageProviderKnownFolderSyncRequestArgs>) -> windows_core::Result<()> + Send + 'static>(invoke: F) -> Self {
        let com = StorageProviderKnownFolderSyncRequestedHandlerBox { vtable: &StorageProviderKnownFolderSyncRequestedHandlerBox::<F>::VTABLE, count: windows_core::imp::RefCount::new(1), invoke };
        unsafe { core::mem::transmute(windows_core::imp::Box::new(com)) }
    }
    pub fn Invoke<P0>(&self, args: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<StorageProviderKnownFolderSyncRequestArgs>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Invoke)(windows_core::Interface::as_raw(this), args.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct StorageProviderKnownFolderSyncRequestedHandler_Vtbl {
    base__: windows_core::IUnknown_Vtbl,
    Invoke: unsafe extern "system" fn(this: *mut core::ffi::c_void, args: *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[repr(C)]
struct StorageProviderKnownFolderSyncRequestedHandlerBox<F: Fn(windows_core::Ref<StorageProviderKnownFolderSyncRequestArgs>) -> windows_core::Result<()> + Send + 'static> {
    vtable: *const StorageProviderKnownFolderSyncRequestedHandler_Vtbl,
    invoke: F,
    count: windows_core::imp::RefCount,
}
impl<F: Fn(windows_core::Ref<StorageProviderKnownFolderSyncRequestArgs>) -> windows_core::Result<()> + Send + 'static> StorageProviderKnownFolderSyncRequestedHandlerBox<F> {
    const VTABLE: StorageProviderKnownFolderSyncRequestedHandler_Vtbl = StorageProviderKnownFolderSyncRequestedHandler_Vtbl { base__: windows_core::IUnknown_Vtbl { QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release }, Invoke: Self::Invoke };
    unsafe extern "system" fn QueryInterface(this: *mut core::ffi::c_void, iid: *const windows_core::GUID, interface: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
        unsafe {
            let this = this as *mut *mut core::ffi::c_void as *mut Self;
            if iid.is_null() || interface.is_null() {
                return windows_core::HRESULT(-2147467261);
            }
            *interface = if *iid == <StorageProviderKnownFolderSyncRequestedHandler as windows_core::Interface>::IID || *iid == <windows_core::IUnknown as windows_core::Interface>::IID || *iid == <windows_core::imp::IAgileObject as windows_core::Interface>::IID {
                &mut (*this).vtable as *mut _ as _
            } else if *iid == <windows_core::imp::IMarshal as windows_core::Interface>::IID {
                (*this).count.add_ref();
                return windows_core::imp::marshaler(core::mem::transmute(&mut (*this).vtable as *mut _ as *mut core::ffi::c_void), interface);
            } else {
                core::ptr::null_mut()
            };
            if (*interface).is_null() {
                windows_core::HRESULT(-2147467262)
            } else {
                (*this).count.add_ref();
                windows_core::HRESULT(0)
            }
        }
    }
    unsafe extern "system" fn AddRef(this: *mut core::ffi::c_void) -> u32 {
        unsafe {
            let this = this as *mut *mut core::ffi::c_void as *mut Self;
            (*this).count.add_ref()
        }
    }
    unsafe extern "system" fn Release(this: *mut core::ffi::c_void) -> u32 {
        unsafe {
            let this = this as *mut *mut core::ffi::c_void as *mut Self;
            let remaining = (*this).count.release();
            if remaining == 0 {
                let _ = windows_core::imp::Box::from_raw(this);
            }
            remaining
        }
    }
    unsafe extern "system" fn Invoke(this: *mut core::ffi::c_void, args: *mut core::ffi::c_void) -> windows_core::HRESULT {
        unsafe {
            let this = &mut *(this as *mut *mut core::ffi::c_void as *mut Self);
            (this.invoke)(core::mem::transmute_copy(&args)).into()
        }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct StorageProviderKnownFolderSyncStatus(pub i32);
impl StorageProviderKnownFolderSyncStatus {
    pub const Available: Self = Self(0i32);
    pub const Enrolling: Self = Self(1i32);
    pub const Enrolled: Self = Self(2i32);
}
impl windows_core::TypeKind for StorageProviderKnownFolderSyncStatus {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for StorageProviderKnownFolderSyncStatus {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Storage.Provider.StorageProviderKnownFolderSyncStatus;i4)");
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StorageProviderMoreInfoUI(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(StorageProviderMoreInfoUI, windows_core::IUnknown, windows_core::IInspectable);
impl StorageProviderMoreInfoUI {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<StorageProviderMoreInfoUI, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Message(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Message)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetMessage(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetMessage)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn Command(&self) -> windows_core::Result<IStorageProviderUICommand> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Command)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetCommand<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IStorageProviderUICommand>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetCommand)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
}
impl windows_core::RuntimeType for StorageProviderMoreInfoUI {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IStorageProviderMoreInfoUI>();
}
unsafe impl windows_core::Interface for StorageProviderMoreInfoUI {
    type Vtable = <IStorageProviderMoreInfoUI as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IStorageProviderMoreInfoUI as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for StorageProviderMoreInfoUI {
    const NAME: &'static str = "Windows.Storage.Provider.StorageProviderMoreInfoUI";
}
unsafe impl Send for StorageProviderMoreInfoUI {}
unsafe impl Sync for StorageProviderMoreInfoUI {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct StorageProviderPopulationPolicy(pub i32);
impl StorageProviderPopulationPolicy {
    pub const Full: Self = Self(1i32);
    pub const AlwaysFull: Self = Self(2i32);
}
impl windows_core::TypeKind for StorageProviderPopulationPolicy {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for StorageProviderPopulationPolicy {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Storage.Provider.StorageProviderPopulationPolicy;i4)");
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct StorageProviderProtectionMode(pub i32);
impl StorageProviderProtectionMode {
    pub const Unknown: Self = Self(0i32);
    pub const Personal: Self = Self(1i32);
}
impl windows_core::TypeKind for StorageProviderProtectionMode {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for StorageProviderProtectionMode {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Storage.Provider.StorageProviderProtectionMode;i4)");
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StorageProviderQueryResultSet(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(StorageProviderQueryResultSet, windows_core::IUnknown, windows_core::IInspectable);
impl StorageProviderQueryResultSet {
    pub fn GetResults(&self) -> windows_core::Result<windows_core::Array<IStorageProviderQueryResult>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::MaybeUninit::zeroed();
            (windows_core::Interface::vtable(this).GetResults)(windows_core::Interface::as_raw(this), windows_core::Array::<IStorageProviderQueryResult>::set_abi_len(core::mem::transmute(&mut result__)), result__.as_mut_ptr() as *mut _ as _).map(|| result__.assume_init())
        }
    }
    pub fn QueryResultId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).QueryResultId)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetQueryResultId(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetQueryResultId)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn Status(&self) -> windows_core::Result<StorageProviderSearchQueryStatus> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Status)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetStatus(&self, value: StorageProviderSearchQueryStatus) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetStatus)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn CreateInstance(results: &[Option<IStorageProviderQueryResult>]) -> windows_core::Result<StorageProviderQueryResultSet> {
        Self::IStorageProviderQueryResultSetFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(windows_core::Interface::as_raw(this), results.len().try_into().unwrap(), core::mem::transmute(results.as_ptr()), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    fn IStorageProviderQueryResultSetFactory<R, F: FnOnce(&IStorageProviderQueryResultSetFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<StorageProviderQueryResultSet, IStorageProviderQueryResultSetFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for StorageProviderQueryResultSet {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IStorageProviderQueryResultSet>();
}
unsafe impl windows_core::Interface for StorageProviderQueryResultSet {
    type Vtable = <IStorageProviderQueryResultSet as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IStorageProviderQueryResultSet as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for StorageProviderQueryResultSet {
    const NAME: &'static str = "Windows.Storage.Provider.StorageProviderQueryResultSet";
}
unsafe impl Send for StorageProviderQueryResultSet {}
unsafe impl Sync for StorageProviderQueryResultSet {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StorageProviderQuotaUI(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(StorageProviderQuotaUI, windows_core::IUnknown, windows_core::IInspectable);
impl StorageProviderQuotaUI {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<StorageProviderQuotaUI, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn QuotaTotalInBytes(&self) -> windows_core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).QuotaTotalInBytes)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetQuotaTotalInBytes(&self, value: u64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetQuotaTotalInBytes)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn QuotaUsedInBytes(&self) -> windows_core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).QuotaUsedInBytes)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetQuotaUsedInBytes(&self, value: u64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetQuotaUsedInBytes)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn QuotaUsedLabel(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).QuotaUsedLabel)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetQuotaUsedLabel(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetQuotaUsedLabel)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    #[cfg(feature = "UI")]
    pub fn QuotaUsedColor(&self) -> windows_core::Result<super::super::Foundation::IReference<super::super::UI::Color>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).QuotaUsedColor)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "UI")]
    pub fn SetQuotaUsedColor<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::IReference<super::super::UI::Color>>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetQuotaUsedColor)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
}
impl windows_core::RuntimeType for StorageProviderQuotaUI {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IStorageProviderQuotaUI>();
}
unsafe impl windows_core::Interface for StorageProviderQuotaUI {
    type Vtable = <IStorageProviderQuotaUI as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IStorageProviderQuotaUI as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for StorageProviderQuotaUI {
    const NAME: &'static str = "Windows.Storage.Provider.StorageProviderQuotaUI";
}
unsafe impl Send for StorageProviderQuotaUI {}
unsafe impl Sync for StorageProviderQuotaUI {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct StorageProviderResultKind(pub i32);
impl StorageProviderResultKind {
    pub const Search: Self = Self(0i32);
    pub const Recommended: Self = Self(1i32);
    pub const Favorites: Self = Self(2i32);
    pub const Recent: Self = Self(3i32);
    pub const Shared: Self = Self(4i32);
    pub const RelatedFiles: Self = Self(5i32);
    pub const RelatedConversations: Self = Self(6i32);
}
impl windows_core::TypeKind for StorageProviderResultKind {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for StorageProviderResultKind {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Storage.Provider.StorageProviderResultKind;i4)");
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct StorageProviderResultUsageKind(pub i32);
impl StorageProviderResultUsageKind {
    pub const Rendered: Self = Self(0i32);
    pub const Opened: Self = Self(1i32);
    pub const SuggestionResponseReceived: Self = Self(2i32);
}
impl windows_core::TypeKind for StorageProviderResultUsageKind {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for StorageProviderResultUsageKind {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Storage.Provider.StorageProviderResultUsageKind;i4)");
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct StorageProviderSearchMatchKind(pub i32);
impl StorageProviderSearchMatchKind {
    pub const Lexical: Self = Self(0i32);
    pub const Semantic: Self = Self(1i32);
}
impl windows_core::TypeKind for StorageProviderSearchMatchKind {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for StorageProviderSearchMatchKind {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Storage.Provider.StorageProviderSearchMatchKind;i4)");
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StorageProviderSearchQueryOptions(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(StorageProviderSearchQueryOptions, windows_core::IUnknown, windows_core::IInspectable);
impl StorageProviderSearchQueryOptions {
    pub fn UserQuery(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UserQuery)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn Language(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Language)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(feature = "Storage_Search")]
    pub fn SortOrder(&self) -> windows_core::Result<windows_collections::IVectorView<super::Search::SortEntry>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SortOrder)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ProgrammaticQuery(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ProgrammaticQuery)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn MaxResults(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MaxResults)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn FolderScope(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FolderScope)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn QueryId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).QueryId)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn PropertiesToFetch(&self) -> windows_core::Result<windows_collections::IVectorView<windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PropertiesToFetch)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for StorageProviderSearchQueryOptions {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IStorageProviderSearchQueryOptions>();
}
unsafe impl windows_core::Interface for StorageProviderSearchQueryOptions {
    type Vtable = <IStorageProviderSearchQueryOptions as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IStorageProviderSearchQueryOptions as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for StorageProviderSearchQueryOptions {
    const NAME: &'static str = "Windows.Storage.Provider.StorageProviderSearchQueryOptions";
}
unsafe impl Send for StorageProviderSearchQueryOptions {}
unsafe impl Sync for StorageProviderSearchQueryOptions {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct StorageProviderSearchQueryStatus(pub i32);
impl StorageProviderSearchQueryStatus {
    pub const Success: Self = Self(0i32);
    pub const Error: Self = Self(1i32);
    pub const Timeout: Self = Self(2i32);
    pub const NoNetwork: Self = Self(3i32);
    pub const NetworkError: Self = Self(4i32);
    pub const NotSignedIn: Self = Self(5i32);
    pub const QueryNotSupported: Self = Self(6i32);
    pub const SortOrderNotSupported: Self = Self(7i32);
}
impl windows_core::TypeKind for StorageProviderSearchQueryStatus {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for StorageProviderSearchQueryStatus {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Storage.Provider.StorageProviderSearchQueryStatus;i4)");
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StorageProviderSearchResult(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(StorageProviderSearchResult, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(StorageProviderSearchResult, IStorageProviderQueryResult);
impl StorageProviderSearchResult {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<StorageProviderSearchResult, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Kind(&self) -> windows_core::Result<StorageProviderResultKind> {
        let this = &windows_core::Interface::cast::<IStorageProviderQueryResult>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetKind(&self, value: StorageProviderResultKind) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IStorageProviderQueryResult>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetKind)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ResultId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IStorageProviderQueryResult>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ResultId)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetResultId(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IStorageProviderQueryResult>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetResultId)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn RemoteFileId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IStorageProviderQueryResult>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RemoteFileId)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetRemoteFileId(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IStorageProviderQueryResult>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetRemoteFileId)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn FilePath(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IStorageProviderQueryResult>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FilePath)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetFilePath(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IStorageProviderQueryResult>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetFilePath)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn RequestedProperties(&self) -> windows_core::Result<super::super::Foundation::Collections::PropertySet> {
        let this = &windows_core::Interface::cast::<IStorageProviderQueryResult>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RequestedProperties)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn MatchScore(&self) -> windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MatchScore)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetMatchScore(&self, value: f64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetMatchScore)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn MatchKind(&self) -> windows_core::Result<StorageProviderSearchMatchKind> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MatchKind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetMatchKind(&self, value: StorageProviderSearchMatchKind) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetMatchKind)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn MatchedPropertyName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MatchedPropertyName)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetMatchedPropertyName(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetMatchedPropertyName)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
}
impl windows_core::RuntimeType for StorageProviderSearchResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IStorageProviderSearchResult>();
}
unsafe impl windows_core::Interface for StorageProviderSearchResult {
    type Vtable = <IStorageProviderSearchResult as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IStorageProviderSearchResult as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for StorageProviderSearchResult {
    const NAME: &'static str = "Windows.Storage.Provider.StorageProviderSearchResult";
}
unsafe impl Send for StorageProviderSearchResult {}
unsafe impl Sync for StorageProviderSearchResult {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct StorageProviderShareLinkState(pub i32);
impl StorageProviderShareLinkState {
    pub const Enabled: Self = Self(0i32);
    pub const Disabled: Self = Self(1i32);
}
impl windows_core::TypeKind for StorageProviderShareLinkState {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for StorageProviderShareLinkState {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Storage.Provider.StorageProviderShareLinkState;i4)");
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct StorageProviderState(pub i32);
impl StorageProviderState {
    pub const InSync: Self = Self(0i32);
    pub const Syncing: Self = Self(1i32);
    pub const Paused: Self = Self(2i32);
    pub const Error: Self = Self(3i32);
    pub const Warning: Self = Self(4i32);
    pub const Offline: Self = Self(5i32);
}
impl windows_core::TypeKind for StorageProviderState {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for StorageProviderState {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Storage.Provider.StorageProviderState;i4)");
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StorageProviderStatusUI(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(StorageProviderStatusUI, windows_core::IUnknown, windows_core::IInspectable);
impl StorageProviderStatusUI {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<StorageProviderStatusUI, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn ProviderState(&self) -> windows_core::Result<StorageProviderState> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ProviderState)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetProviderState(&self, value: StorageProviderState) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetProviderState)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ProviderStateLabel(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ProviderStateLabel)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetProviderStateLabel(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetProviderStateLabel)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn ProviderStateIcon(&self) -> windows_core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ProviderStateIcon)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetProviderStateIcon<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::Uri>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetProviderStateIcon)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn SyncStatusCommand(&self) -> windows_core::Result<IStorageProviderUICommand> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SyncStatusCommand)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetSyncStatusCommand<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IStorageProviderUICommand>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetSyncStatusCommand)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn QuotaUI(&self) -> windows_core::Result<StorageProviderQuotaUI> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).QuotaUI)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetQuotaUI<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<StorageProviderQuotaUI>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetQuotaUI)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn MoreInfoUI(&self) -> windows_core::Result<StorageProviderMoreInfoUI> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MoreInfoUI)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetMoreInfoUI<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<StorageProviderMoreInfoUI>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetMoreInfoUI)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn ProviderPrimaryCommand(&self) -> windows_core::Result<IStorageProviderUICommand> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ProviderPrimaryCommand)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetProviderPrimaryCommand<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IStorageProviderUICommand>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetProviderPrimaryCommand)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn ProviderSecondaryCommands(&self) -> windows_core::Result<windows_collections::IVector<IStorageProviderUICommand>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ProviderSecondaryCommands)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetProviderSecondaryCommands<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_collections::IVector<IStorageProviderUICommand>>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetProviderSecondaryCommands)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
}
impl windows_core::RuntimeType for StorageProviderStatusUI {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IStorageProviderStatusUI>();
}
unsafe impl windows_core::Interface for StorageProviderStatusUI {
    type Vtable = <IStorageProviderStatusUI as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IStorageProviderStatusUI as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for StorageProviderStatusUI {
    const NAME: &'static str = "Windows.Storage.Provider.StorageProviderStatusUI";
}
unsafe impl Send for StorageProviderStatusUI {}
unsafe impl Sync for StorageProviderStatusUI {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StorageProviderSuggestionResult(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(StorageProviderSuggestionResult, windows_core::IUnknown, windows_core::IInspectable, IStorageProviderQueryResult);
impl StorageProviderSuggestionResult {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<StorageProviderSuggestionResult, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Kind(&self) -> windows_core::Result<StorageProviderResultKind> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetKind(&self, value: StorageProviderResultKind) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetKind)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ResultId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ResultId)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetResultId(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetResultId)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn RemoteFileId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RemoteFileId)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetRemoteFileId(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetRemoteFileId)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn FilePath(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FilePath)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetFilePath(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetFilePath)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn RequestedProperties(&self) -> windows_core::Result<super::super::Foundation::Collections::PropertySet> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RequestedProperties)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for StorageProviderSuggestionResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IStorageProviderQueryResult>();
}
unsafe impl windows_core::Interface for StorageProviderSuggestionResult {
    type Vtable = <IStorageProviderQueryResult as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IStorageProviderQueryResult as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for StorageProviderSuggestionResult {
    const NAME: &'static str = "Windows.Storage.Provider.StorageProviderSuggestionResult";
}
unsafe impl Send for StorageProviderSuggestionResult {}
unsafe impl Sync for StorageProviderSuggestionResult {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StorageProviderSuggestionsQueryOptions(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(StorageProviderSuggestionsQueryOptions, windows_core::IUnknown, windows_core::IInspectable);
impl StorageProviderSuggestionsQueryOptions {
    pub fn SuggestionsKind(&self) -> windows_core::Result<StorageProviderResultKind> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SuggestionsKind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn RemoteFileId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RemoteFileId)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn MaxResults(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MaxResults)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn QueryId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).QueryId)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn PropertiesToFetch(&self) -> windows_core::Result<windows_collections::IVectorView<windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PropertiesToFetch)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for StorageProviderSuggestionsQueryOptions {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IStorageProviderSuggestionsQueryOptions>();
}
unsafe impl windows_core::Interface for StorageProviderSuggestionsQueryOptions {
    type Vtable = <IStorageProviderSuggestionsQueryOptions as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IStorageProviderSuggestionsQueryOptions as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for StorageProviderSuggestionsQueryOptions {
    const NAME: &'static str = "Windows.Storage.Provider.StorageProviderSuggestionsQueryOptions";
}
unsafe impl Send for StorageProviderSuggestionsQueryOptions {}
unsafe impl Sync for StorageProviderSuggestionsQueryOptions {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StorageProviderSyncRootInfo(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(StorageProviderSyncRootInfo, windows_core::IUnknown, windows_core::IInspectable);
impl StorageProviderSyncRootInfo {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<StorageProviderSyncRootInfo, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Id(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Id)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetId(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetId)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn Context(&self) -> windows_core::Result<super::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Context)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn SetContext<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Streams::IBuffer>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetContext)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn Path(&self) -> windows_core::Result<super::IStorageFolder> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Path)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetPath<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::IStorageFolder>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetPath)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn DisplayNameResource(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DisplayNameResource)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetDisplayNameResource(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetDisplayNameResource)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn IconResource(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IconResource)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetIconResource(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetIconResource)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn HydrationPolicy(&self) -> windows_core::Result<StorageProviderHydrationPolicy> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HydrationPolicy)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetHydrationPolicy(&self, value: StorageProviderHydrationPolicy) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetHydrationPolicy)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn HydrationPolicyModifier(&self) -> windows_core::Result<StorageProviderHydrationPolicyModifier> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HydrationPolicyModifier)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetHydrationPolicyModifier(&self, value: StorageProviderHydrationPolicyModifier) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetHydrationPolicyModifier)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn PopulationPolicy(&self) -> windows_core::Result<StorageProviderPopulationPolicy> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PopulationPolicy)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetPopulationPolicy(&self, value: StorageProviderPopulationPolicy) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetPopulationPolicy)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn InSyncPolicy(&self) -> windows_core::Result<StorageProviderInSyncPolicy> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).InSyncPolicy)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetInSyncPolicy(&self, value: StorageProviderInSyncPolicy) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetInSyncPolicy)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn HardlinkPolicy(&self) -> windows_core::Result<StorageProviderHardlinkPolicy> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HardlinkPolicy)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetHardlinkPolicy(&self, value: StorageProviderHardlinkPolicy) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetHardlinkPolicy)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ShowSiblingsAsGroup(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ShowSiblingsAsGroup)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetShowSiblingsAsGroup(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetShowSiblingsAsGroup)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Version(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Version)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetVersion(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetVersion)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn ProtectionMode(&self) -> windows_core::Result<StorageProviderProtectionMode> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ProtectionMode)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetProtectionMode(&self, value: StorageProviderProtectionMode) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetProtectionMode)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn AllowPinning(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AllowPinning)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetAllowPinning(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetAllowPinning)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn StorageProviderItemPropertyDefinitions(&self) -> windows_core::Result<windows_collections::IVector<StorageProviderItemPropertyDefinition>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).StorageProviderItemPropertyDefinitions)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn RecycleBinUri(&self) -> windows_core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RecycleBinUri)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetRecycleBinUri<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::Uri>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetRecycleBinUri)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn ProviderId(&self) -> windows_core::Result<windows_core::GUID> {
        let this = &windows_core::Interface::cast::<IStorageProviderSyncRootInfo2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ProviderId)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetProviderId(&self, value: windows_core::GUID) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IStorageProviderSyncRootInfo2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetProviderId)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn FallbackFileTypeInfo(&self) -> windows_core::Result<windows_collections::IVector<StorageProviderFileTypeInfo>> {
        let this = &windows_core::Interface::cast::<IStorageProviderSyncRootInfo3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FallbackFileTypeInfo)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for StorageProviderSyncRootInfo {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IStorageProviderSyncRootInfo>();
}
unsafe impl windows_core::Interface for StorageProviderSyncRootInfo {
    type Vtable = <IStorageProviderSyncRootInfo as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IStorageProviderSyncRootInfo as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for StorageProviderSyncRootInfo {
    const NAME: &'static str = "Windows.Storage.Provider.StorageProviderSyncRootInfo";
}
unsafe impl Send for StorageProviderSyncRootInfo {}
unsafe impl Sync for StorageProviderSyncRootInfo {}
pub struct StorageProviderSyncRootManager;
impl StorageProviderSyncRootManager {
    pub fn Register<P0>(syncrootinformation: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<StorageProviderSyncRootInfo>,
    {
        Self::IStorageProviderSyncRootManagerStatics(|this| unsafe { (windows_core::Interface::vtable(this).Register)(windows_core::Interface::as_raw(this), syncrootinformation.param().abi()).ok() })
    }
    pub fn Unregister(id: &windows_core::HSTRING) -> windows_core::Result<()> {
        Self::IStorageProviderSyncRootManagerStatics(|this| unsafe { (windows_core::Interface::vtable(this).Unregister)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(id)).ok() })
    }
    pub fn GetSyncRootInformationForFolder<P0>(folder: P0) -> windows_core::Result<StorageProviderSyncRootInfo>
    where
        P0: windows_core::Param<super::IStorageFolder>,
    {
        Self::IStorageProviderSyncRootManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetSyncRootInformationForFolder)(windows_core::Interface::as_raw(this), folder.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn GetSyncRootInformationForId(id: &windows_core::HSTRING) -> windows_core::Result<StorageProviderSyncRootInfo> {
        Self::IStorageProviderSyncRootManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetSyncRootInformationForId)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(id), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn GetCurrentSyncRoots() -> windows_core::Result<windows_collections::IVectorView<StorageProviderSyncRootInfo>> {
        Self::IStorageProviderSyncRootManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetCurrentSyncRoots)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn IsSupported() -> windows_core::Result<bool> {
        Self::IStorageProviderSyncRootManagerStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsSupported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    fn IStorageProviderSyncRootManagerStatics<R, F: FnOnce(&IStorageProviderSyncRootManagerStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<StorageProviderSyncRootManager, IStorageProviderSyncRootManagerStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    fn IStorageProviderSyncRootManagerStatics2<R, F: FnOnce(&IStorageProviderSyncRootManagerStatics2) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<StorageProviderSyncRootManager, IStorageProviderSyncRootManagerStatics2> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeName for StorageProviderSyncRootManager {
    const NAME: &'static str = "Windows.Storage.Provider.StorageProviderSyncRootManager";
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct StorageProviderUICommandState(pub i32);
impl StorageProviderUICommandState {
    pub const Enabled: Self = Self(0i32);
    pub const Disabled: Self = Self(1i32);
    pub const Hidden: Self = Self(2i32);
}
impl windows_core::TypeKind for StorageProviderUICommandState {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for StorageProviderUICommandState {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Storage.Provider.StorageProviderUICommandState;i4)");
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct StorageProviderUriSourceStatus(pub i32);
impl StorageProviderUriSourceStatus {
    pub const Success: Self = Self(0i32);
    pub const NoSyncRoot: Self = Self(1i32);
    pub const FileNotFound: Self = Self(2i32);
}
impl windows_core::TypeKind for StorageProviderUriSourceStatus {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for StorageProviderUriSourceStatus {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Storage.Provider.StorageProviderUriSourceStatus;i4)");
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct UIStatus(pub i32);
impl UIStatus {
    pub const Unavailable: Self = Self(0i32);
    pub const Hidden: Self = Self(1i32);
    pub const Visible: Self = Self(2i32);
    pub const Complete: Self = Self(3i32);
}
impl windows_core::TypeKind for UIStatus {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for UIStatus {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Storage.Provider.UIStatus;i4)");
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WriteActivationMode(pub i32);
impl WriteActivationMode {
    pub const ReadOnly: Self = Self(0i32);
    pub const NotNeeded: Self = Self(1i32);
    pub const AfterWrite: Self = Self(2i32);
}
impl windows_core::TypeKind for WriteActivationMode {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for WriteActivationMode {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Storage.Provider.WriteActivationMode;i4)");
}
