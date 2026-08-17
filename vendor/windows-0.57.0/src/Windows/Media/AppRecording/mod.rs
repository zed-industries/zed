windows_core::imp::define_interface!(IAppRecordingManager, IAppRecordingManager_Vtbl, 0xe7e26076_a044_48e2_a512_3094d574c7cc);
impl windows_core::RuntimeType for IAppRecordingManager {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAppRecordingManager_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Storage")]
    pub StartRecordingToFileAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage"))]
    StartRecordingToFileAsync: usize,
    #[cfg(feature = "Storage")]
    pub RecordTimeSpanToFileAsync: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::DateTime, super::super::Foundation::TimeSpan, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage"))]
    RecordTimeSpanToFileAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub SupportedScreenshotMediaEncodingSubtypes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SupportedScreenshotMediaEncodingSubtypes: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage"))]
    pub SaveScreenshotToFilesAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, AppRecordingSaveScreenshotOption, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Storage")))]
    SaveScreenshotToFilesAsync: usize,
}
windows_core::imp::define_interface!(IAppRecordingManagerStatics, IAppRecordingManagerStatics_Vtbl, 0x50e709f7_38ce_4bd3_9db2_e72bbe9de11d);
impl windows_core::RuntimeType for IAppRecordingManagerStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAppRecordingManagerStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetDefault: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAppRecordingResult, IAppRecordingResult_Vtbl, 0x3a900864_c66d_46f9_b2d9_5bc2dad070d7);
impl windows_core::RuntimeType for IAppRecordingResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAppRecordingResult_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Succeeded: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub ExtendedError: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::HRESULT) -> windows_core::HRESULT,
    pub Duration: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::TimeSpan) -> windows_core::HRESULT,
    pub IsFileTruncated: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAppRecordingSaveScreenshotResult, IAppRecordingSaveScreenshotResult_Vtbl, 0x9c5b8d0a_0abb_4457_aaee_24f9c12ec778);
impl windows_core::RuntimeType for IAppRecordingSaveScreenshotResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAppRecordingSaveScreenshotResult_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Succeeded: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub ExtendedError: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::HRESULT) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub SavedScreenshotInfos: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SavedScreenshotInfos: usize,
}
windows_core::imp::define_interface!(IAppRecordingSavedScreenshotInfo, IAppRecordingSavedScreenshotInfo_Vtbl, 0x9b642d0a_189a_4d00_bf25_e1bb1249d594);
impl windows_core::RuntimeType for IAppRecordingSavedScreenshotInfo {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAppRecordingSavedScreenshotInfo_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Storage")]
    pub File: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage"))]
    File: usize,
    pub MediaEncodingSubtype: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAppRecordingStatus, IAppRecordingStatus_Vtbl, 0x1d0cc82c_bc18_4b8a_a6ef_127efab3b5d9);
impl windows_core::RuntimeType for IAppRecordingStatus {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAppRecordingStatus_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CanRecord: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub CanRecordTimeSpan: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub HistoricalBufferDuration: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::TimeSpan) -> windows_core::HRESULT,
    pub Details: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAppRecordingStatusDetails, IAppRecordingStatusDetails_Vtbl, 0xb538a9b0_14ed_4412_ac45_6d672c9c9949);
impl windows_core::RuntimeType for IAppRecordingStatusDetails {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAppRecordingStatusDetails_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IsAnyAppBroadcasting: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsCaptureResourceUnavailable: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsGameStreamInProgress: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsTimeSpanRecordingDisabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsGpuConstrained: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsAppInactive: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsBlockedForApp: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsDisabledByUser: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsDisabledBySystem: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct AppRecordingManager(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(AppRecordingManager, windows_core::IUnknown, windows_core::IInspectable);
impl AppRecordingManager {
    pub fn GetStatus(&self) -> windows_core::Result<AppRecordingStatus> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetStatus)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage")]
    pub fn StartRecordingToFileAsync<P0>(&self, file: P0) -> windows_core::Result<super::super::Foundation::IAsyncOperation<AppRecordingResult>>
    where
        P0: windows_core::Param<super::super::Storage::StorageFile>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).StartRecordingToFileAsync)(windows_core::Interface::as_raw(this), file.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage")]
    pub fn RecordTimeSpanToFileAsync<P0>(&self, starttime: super::super::Foundation::DateTime, duration: super::super::Foundation::TimeSpan, file: P0) -> windows_core::Result<super::super::Foundation::IAsyncOperation<AppRecordingResult>>
    where
        P0: windows_core::Param<super::super::Storage::StorageFile>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RecordTimeSpanToFileAsync)(windows_core::Interface::as_raw(this), starttime, duration, file.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn SupportedScreenshotMediaEncodingSubtypes(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SupportedScreenshotMediaEncodingSubtypes)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage"))]
    pub fn SaveScreenshotToFilesAsync<P0, P1>(&self, folder: P0, filenameprefix: &windows_core::HSTRING, option: AppRecordingSaveScreenshotOption, requestedformats: P1) -> windows_core::Result<super::super::Foundation::IAsyncOperation<AppRecordingSaveScreenshotResult>>
    where
        P0: windows_core::Param<super::super::Storage::StorageFolder>,
        P1: windows_core::Param<super::super::Foundation::Collections::IIterable<windows_core::HSTRING>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SaveScreenshotToFilesAsync)(windows_core::Interface::as_raw(this), folder.param().abi(), core::mem::transmute_copy(filenameprefix), option, requestedformats.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetDefault() -> windows_core::Result<AppRecordingManager> {
        Self::IAppRecordingManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDefault)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IAppRecordingManagerStatics<R, F: FnOnce(&IAppRecordingManagerStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<AppRecordingManager, IAppRecordingManagerStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for AppRecordingManager {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IAppRecordingManager>();
}
unsafe impl windows_core::Interface for AppRecordingManager {
    type Vtable = IAppRecordingManager_Vtbl;
    const IID: windows_core::GUID = <IAppRecordingManager as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for AppRecordingManager {
    const NAME: &'static str = "Windows.Media.AppRecording.AppRecordingManager";
}
unsafe impl Send for AppRecordingManager {}
unsafe impl Sync for AppRecordingManager {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct AppRecordingResult(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(AppRecordingResult, windows_core::IUnknown, windows_core::IInspectable);
impl AppRecordingResult {
    pub fn Succeeded(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Succeeded)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ExtendedError(&self) -> windows_core::Result<windows_core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ExtendedError)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Duration(&self) -> windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Duration)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsFileTruncated(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsFileTruncated)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for AppRecordingResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IAppRecordingResult>();
}
unsafe impl windows_core::Interface for AppRecordingResult {
    type Vtable = IAppRecordingResult_Vtbl;
    const IID: windows_core::GUID = <IAppRecordingResult as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for AppRecordingResult {
    const NAME: &'static str = "Windows.Media.AppRecording.AppRecordingResult";
}
unsafe impl Send for AppRecordingResult {}
unsafe impl Sync for AppRecordingResult {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct AppRecordingSaveScreenshotResult(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(AppRecordingSaveScreenshotResult, windows_core::IUnknown, windows_core::IInspectable);
impl AppRecordingSaveScreenshotResult {
    pub fn Succeeded(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Succeeded)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ExtendedError(&self) -> windows_core::Result<windows_core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ExtendedError)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn SavedScreenshotInfos(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<AppRecordingSavedScreenshotInfo>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SavedScreenshotInfos)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for AppRecordingSaveScreenshotResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IAppRecordingSaveScreenshotResult>();
}
unsafe impl windows_core::Interface for AppRecordingSaveScreenshotResult {
    type Vtable = IAppRecordingSaveScreenshotResult_Vtbl;
    const IID: windows_core::GUID = <IAppRecordingSaveScreenshotResult as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for AppRecordingSaveScreenshotResult {
    const NAME: &'static str = "Windows.Media.AppRecording.AppRecordingSaveScreenshotResult";
}
unsafe impl Send for AppRecordingSaveScreenshotResult {}
unsafe impl Sync for AppRecordingSaveScreenshotResult {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct AppRecordingSavedScreenshotInfo(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(AppRecordingSavedScreenshotInfo, windows_core::IUnknown, windows_core::IInspectable);
impl AppRecordingSavedScreenshotInfo {
    #[cfg(feature = "Storage")]
    pub fn File(&self) -> windows_core::Result<super::super::Storage::StorageFile> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).File)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn MediaEncodingSubtype(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MediaEncodingSubtype)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for AppRecordingSavedScreenshotInfo {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IAppRecordingSavedScreenshotInfo>();
}
unsafe impl windows_core::Interface for AppRecordingSavedScreenshotInfo {
    type Vtable = IAppRecordingSavedScreenshotInfo_Vtbl;
    const IID: windows_core::GUID = <IAppRecordingSavedScreenshotInfo as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for AppRecordingSavedScreenshotInfo {
    const NAME: &'static str = "Windows.Media.AppRecording.AppRecordingSavedScreenshotInfo";
}
unsafe impl Send for AppRecordingSavedScreenshotInfo {}
unsafe impl Sync for AppRecordingSavedScreenshotInfo {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct AppRecordingStatus(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(AppRecordingStatus, windows_core::IUnknown, windows_core::IInspectable);
impl AppRecordingStatus {
    pub fn CanRecord(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CanRecord)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn CanRecordTimeSpan(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CanRecordTimeSpan)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn HistoricalBufferDuration(&self) -> windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HistoricalBufferDuration)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Details(&self) -> windows_core::Result<AppRecordingStatusDetails> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Details)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for AppRecordingStatus {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IAppRecordingStatus>();
}
unsafe impl windows_core::Interface for AppRecordingStatus {
    type Vtable = IAppRecordingStatus_Vtbl;
    const IID: windows_core::GUID = <IAppRecordingStatus as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for AppRecordingStatus {
    const NAME: &'static str = "Windows.Media.AppRecording.AppRecordingStatus";
}
unsafe impl Send for AppRecordingStatus {}
unsafe impl Sync for AppRecordingStatus {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct AppRecordingStatusDetails(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(AppRecordingStatusDetails, windows_core::IUnknown, windows_core::IInspectable);
impl AppRecordingStatusDetails {
    pub fn IsAnyAppBroadcasting(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsAnyAppBroadcasting)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsCaptureResourceUnavailable(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsCaptureResourceUnavailable)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsGameStreamInProgress(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsGameStreamInProgress)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsTimeSpanRecordingDisabled(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsTimeSpanRecordingDisabled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsGpuConstrained(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsGpuConstrained)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsAppInactive(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsAppInactive)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsBlockedForApp(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsBlockedForApp)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsDisabledByUser(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsDisabledByUser)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsDisabledBySystem(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsDisabledBySystem)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for AppRecordingStatusDetails {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IAppRecordingStatusDetails>();
}
unsafe impl windows_core::Interface for AppRecordingStatusDetails {
    type Vtable = IAppRecordingStatusDetails_Vtbl;
    const IID: windows_core::GUID = <IAppRecordingStatusDetails as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for AppRecordingStatusDetails {
    const NAME: &'static str = "Windows.Media.AppRecording.AppRecordingStatusDetails";
}
unsafe impl Send for AppRecordingStatusDetails {}
unsafe impl Sync for AppRecordingStatusDetails {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct AppRecordingSaveScreenshotOption(pub i32);
impl AppRecordingSaveScreenshotOption {
    pub const None: Self = Self(0i32);
    pub const HdrContentVisible: Self = Self(1i32);
}
impl windows_core::TypeKind for AppRecordingSaveScreenshotOption {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for AppRecordingSaveScreenshotOption {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("AppRecordingSaveScreenshotOption").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for AppRecordingSaveScreenshotOption {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.AppRecording.AppRecordingSaveScreenshotOption;i4)");
}
