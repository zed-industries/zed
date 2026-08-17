windows_core::imp::define_interface!(IOfflineMapPackage, IOfflineMapPackage_Vtbl, 0xa797673b_a5b5_4144_b525_e68c8862664b);
impl windows_core::RuntimeType for IOfflineMapPackage {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IOfflineMapPackage_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Status: unsafe extern "system" fn(*mut core::ffi::c_void, *mut OfflineMapPackageStatus) -> windows_core::HRESULT,
    pub DisplayName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub EnclosingRegionName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub EstimatedSizeInBytes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u64) -> windows_core::HRESULT,
    pub RemoveStatusChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub StatusChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RequestStartDownloadAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IOfflineMapPackageQueryResult, IOfflineMapPackageQueryResult_Vtbl, 0x55585411_39e1_4e41_a4e1_5f4872bee199);
impl windows_core::RuntimeType for IOfflineMapPackageQueryResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IOfflineMapPackageQueryResult_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Status: unsafe extern "system" fn(*mut core::ffi::c_void, *mut OfflineMapPackageQueryStatus) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Packages: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Packages: usize,
}
windows_core::imp::define_interface!(IOfflineMapPackageStartDownloadResult, IOfflineMapPackageStartDownloadResult_Vtbl, 0xd965b918_d4d6_4afe_9378_3ec71ef11c3d);
impl windows_core::RuntimeType for IOfflineMapPackageStartDownloadResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IOfflineMapPackageStartDownloadResult_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Status: unsafe extern "system" fn(*mut core::ffi::c_void, *mut OfflineMapPackageStartDownloadStatus) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IOfflineMapPackageStatics, IOfflineMapPackageStatics_Vtbl, 0x185e7922_a831_4ab0_941f_6998fa929285);
impl windows_core::RuntimeType for IOfflineMapPackageStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IOfflineMapPackageStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Devices_Geolocation")]
    pub FindPackagesAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Devices_Geolocation"))]
    FindPackagesAsync: usize,
    #[cfg(feature = "Devices_Geolocation")]
    pub FindPackagesInBoundingBoxAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Devices_Geolocation"))]
    FindPackagesInBoundingBoxAsync: usize,
    #[cfg(feature = "Devices_Geolocation")]
    pub FindPackagesInGeocircleAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Devices_Geolocation"))]
    FindPackagesInGeocircleAsync: usize,
}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct OfflineMapPackage(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(OfflineMapPackage, windows_core::IUnknown, windows_core::IInspectable);
impl OfflineMapPackage {
    pub fn Status(&self) -> windows_core::Result<OfflineMapPackageStatus> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Status)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn DisplayName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DisplayName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn EnclosingRegionName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).EnclosingRegionName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn EstimatedSizeInBytes(&self) -> windows_core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).EstimatedSizeInBytes)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveStatusChanged(&self, token: super::super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveStatusChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn StatusChanged<P0>(&self, value: P0) -> windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::super::Foundation::TypedEventHandler<OfflineMapPackage, windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).StatusChanged)(windows_core::Interface::as_raw(this), value.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RequestStartDownloadAsync(&self) -> windows_core::Result<super::super::super::Foundation::IAsyncOperation<OfflineMapPackageStartDownloadResult>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RequestStartDownloadAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Devices_Geolocation")]
    pub fn FindPackagesAsync<P0>(querypoint: P0) -> windows_core::Result<super::super::super::Foundation::IAsyncOperation<OfflineMapPackageQueryResult>>
    where
        P0: windows_core::Param<super::super::super::Devices::Geolocation::Geopoint>,
    {
        Self::IOfflineMapPackageStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FindPackagesAsync)(windows_core::Interface::as_raw(this), querypoint.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Devices_Geolocation")]
    pub fn FindPackagesInBoundingBoxAsync<P0>(queryboundingbox: P0) -> windows_core::Result<super::super::super::Foundation::IAsyncOperation<OfflineMapPackageQueryResult>>
    where
        P0: windows_core::Param<super::super::super::Devices::Geolocation::GeoboundingBox>,
    {
        Self::IOfflineMapPackageStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FindPackagesInBoundingBoxAsync)(windows_core::Interface::as_raw(this), queryboundingbox.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Devices_Geolocation")]
    pub fn FindPackagesInGeocircleAsync<P0>(querycircle: P0) -> windows_core::Result<super::super::super::Foundation::IAsyncOperation<OfflineMapPackageQueryResult>>
    where
        P0: windows_core::Param<super::super::super::Devices::Geolocation::Geocircle>,
    {
        Self::IOfflineMapPackageStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FindPackagesInGeocircleAsync)(windows_core::Interface::as_raw(this), querycircle.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IOfflineMapPackageStatics<R, F: FnOnce(&IOfflineMapPackageStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<OfflineMapPackage, IOfflineMapPackageStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for OfflineMapPackage {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IOfflineMapPackage>();
}
unsafe impl windows_core::Interface for OfflineMapPackage {
    type Vtable = IOfflineMapPackage_Vtbl;
    const IID: windows_core::GUID = <IOfflineMapPackage as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for OfflineMapPackage {
    const NAME: &'static str = "Windows.Services.Maps.OfflineMaps.OfflineMapPackage";
}
unsafe impl Send for OfflineMapPackage {}
unsafe impl Sync for OfflineMapPackage {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct OfflineMapPackageQueryResult(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(OfflineMapPackageQueryResult, windows_core::IUnknown, windows_core::IInspectable);
impl OfflineMapPackageQueryResult {
    pub fn Status(&self) -> windows_core::Result<OfflineMapPackageQueryStatus> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Status)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Packages(&self) -> windows_core::Result<super::super::super::Foundation::Collections::IVectorView<OfflineMapPackage>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Packages)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for OfflineMapPackageQueryResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IOfflineMapPackageQueryResult>();
}
unsafe impl windows_core::Interface for OfflineMapPackageQueryResult {
    type Vtable = IOfflineMapPackageQueryResult_Vtbl;
    const IID: windows_core::GUID = <IOfflineMapPackageQueryResult as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for OfflineMapPackageQueryResult {
    const NAME: &'static str = "Windows.Services.Maps.OfflineMaps.OfflineMapPackageQueryResult";
}
unsafe impl Send for OfflineMapPackageQueryResult {}
unsafe impl Sync for OfflineMapPackageQueryResult {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct OfflineMapPackageStartDownloadResult(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(OfflineMapPackageStartDownloadResult, windows_core::IUnknown, windows_core::IInspectable);
impl OfflineMapPackageStartDownloadResult {
    pub fn Status(&self) -> windows_core::Result<OfflineMapPackageStartDownloadStatus> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Status)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for OfflineMapPackageStartDownloadResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IOfflineMapPackageStartDownloadResult>();
}
unsafe impl windows_core::Interface for OfflineMapPackageStartDownloadResult {
    type Vtable = IOfflineMapPackageStartDownloadResult_Vtbl;
    const IID: windows_core::GUID = <IOfflineMapPackageStartDownloadResult as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for OfflineMapPackageStartDownloadResult {
    const NAME: &'static str = "Windows.Services.Maps.OfflineMaps.OfflineMapPackageStartDownloadResult";
}
unsafe impl Send for OfflineMapPackageStartDownloadResult {}
unsafe impl Sync for OfflineMapPackageStartDownloadResult {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct OfflineMapPackageQueryStatus(pub i32);
impl OfflineMapPackageQueryStatus {
    pub const Success: Self = Self(0i32);
    pub const UnknownError: Self = Self(1i32);
    pub const InvalidCredentials: Self = Self(2i32);
    pub const NetworkFailure: Self = Self(3i32);
}
impl windows_core::TypeKind for OfflineMapPackageQueryStatus {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for OfflineMapPackageQueryStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("OfflineMapPackageQueryStatus").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for OfflineMapPackageQueryStatus {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Services.Maps.OfflineMaps.OfflineMapPackageQueryStatus;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct OfflineMapPackageStartDownloadStatus(pub i32);
impl OfflineMapPackageStartDownloadStatus {
    pub const Success: Self = Self(0i32);
    pub const UnknownError: Self = Self(1i32);
    pub const InvalidCredentials: Self = Self(2i32);
    pub const DeniedWithoutCapability: Self = Self(3i32);
}
impl windows_core::TypeKind for OfflineMapPackageStartDownloadStatus {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for OfflineMapPackageStartDownloadStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("OfflineMapPackageStartDownloadStatus").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for OfflineMapPackageStartDownloadStatus {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Services.Maps.OfflineMaps.OfflineMapPackageStartDownloadStatus;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct OfflineMapPackageStatus(pub i32);
impl OfflineMapPackageStatus {
    pub const NotDownloaded: Self = Self(0i32);
    pub const Downloading: Self = Self(1i32);
    pub const Downloaded: Self = Self(2i32);
    pub const Deleting: Self = Self(3i32);
}
impl windows_core::TypeKind for OfflineMapPackageStatus {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for OfflineMapPackageStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("OfflineMapPackageStatus").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for OfflineMapPackageStatus {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Services.Maps.OfflineMaps.OfflineMapPackageStatus;i4)");
}
