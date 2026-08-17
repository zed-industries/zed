windows_core::imp::define_interface!(ISystemUpdateItem, ISystemUpdateItem_Vtbl, 0x779740eb_5624_519e_a8e2_09e9173b3fb7);
impl windows_core::RuntimeType for ISystemUpdateItem {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISystemUpdateItem_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub State: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SystemUpdateItemState) -> windows_core::HRESULT,
    pub Title: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Description: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Id: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Revision: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub DownloadProgress: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub InstallProgress: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub ExtendedError: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::HRESULT) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISystemUpdateLastErrorInfo, ISystemUpdateLastErrorInfo_Vtbl, 0x7ee887f7_8a44_5b6e_bd07_7aece4116ea9);
impl windows_core::RuntimeType for ISystemUpdateLastErrorInfo {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISystemUpdateLastErrorInfo_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub State: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SystemUpdateManagerState) -> windows_core::HRESULT,
    pub ExtendedError: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::HRESULT) -> windows_core::HRESULT,
    pub IsInteractive: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISystemUpdateManagerStatics, ISystemUpdateManagerStatics_Vtbl, 0xb2d3fcef_2971_51be_b41a_8bd703bb701a);
impl windows_core::RuntimeType for ISystemUpdateManagerStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISystemUpdateManagerStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IsSupported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub State: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SystemUpdateManagerState) -> windows_core::HRESULT,
    pub StateChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveStateChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub DownloadProgress: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub InstallProgress: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub UserActiveHoursStart: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::TimeSpan) -> windows_core::HRESULT,
    pub UserActiveHoursEnd: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::TimeSpan) -> windows_core::HRESULT,
    pub UserActiveHoursMax: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub TrySetUserActiveHours: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::TimeSpan, super::super::Foundation::TimeSpan, *mut bool) -> windows_core::HRESULT,
    pub LastUpdateCheckTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::DateTime) -> windows_core::HRESULT,
    pub LastUpdateInstallTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::DateTime) -> windows_core::HRESULT,
    pub LastErrorInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetAutomaticRebootBlockIds: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetAutomaticRebootBlockIds: usize,
    pub BlockAutomaticRebootAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub UnblockAutomaticRebootAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ExtendedError: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::HRESULT) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetUpdateItems: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetUpdateItems: usize,
    pub AttentionRequiredReason: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SystemUpdateAttentionRequiredReason) -> windows_core::HRESULT,
    pub SetFlightRing: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut bool) -> windows_core::HRESULT,
    pub GetFlightRing: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub StartInstall: unsafe extern "system" fn(*mut core::ffi::c_void, SystemUpdateStartInstallAction) -> windows_core::HRESULT,
    pub RebootToCompleteInstall: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub StartCancelUpdates: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct SystemUpdateItem(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(SystemUpdateItem, windows_core::IUnknown, windows_core::IInspectable);
impl SystemUpdateItem {
    pub fn State(&self) -> windows_core::Result<SystemUpdateItemState> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).State)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Title(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Title)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Description(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Description)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Id(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Id)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Revision(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Revision)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn DownloadProgress(&self) -> windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DownloadProgress)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn InstallProgress(&self) -> windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).InstallProgress)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ExtendedError(&self) -> windows_core::Result<windows_core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ExtendedError)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for SystemUpdateItem {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ISystemUpdateItem>();
}
unsafe impl windows_core::Interface for SystemUpdateItem {
    type Vtable = ISystemUpdateItem_Vtbl;
    const IID: windows_core::GUID = <ISystemUpdateItem as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for SystemUpdateItem {
    const NAME: &'static str = "Windows.System.Update.SystemUpdateItem";
}
unsafe impl Send for SystemUpdateItem {}
unsafe impl Sync for SystemUpdateItem {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct SystemUpdateLastErrorInfo(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(SystemUpdateLastErrorInfo, windows_core::IUnknown, windows_core::IInspectable);
impl SystemUpdateLastErrorInfo {
    pub fn State(&self) -> windows_core::Result<SystemUpdateManagerState> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).State)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ExtendedError(&self) -> windows_core::Result<windows_core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ExtendedError)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsInteractive(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsInteractive)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for SystemUpdateLastErrorInfo {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ISystemUpdateLastErrorInfo>();
}
unsafe impl windows_core::Interface for SystemUpdateLastErrorInfo {
    type Vtable = ISystemUpdateLastErrorInfo_Vtbl;
    const IID: windows_core::GUID = <ISystemUpdateLastErrorInfo as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for SystemUpdateLastErrorInfo {
    const NAME: &'static str = "Windows.System.Update.SystemUpdateLastErrorInfo";
}
unsafe impl Send for SystemUpdateLastErrorInfo {}
unsafe impl Sync for SystemUpdateLastErrorInfo {}
pub struct SystemUpdateManager;
impl SystemUpdateManager {
    pub fn IsSupported() -> windows_core::Result<bool> {
        Self::ISystemUpdateManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsSupported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn State() -> windows_core::Result<SystemUpdateManagerState> {
        Self::ISystemUpdateManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).State)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn StateChanged<P0>(handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::EventHandler<windows_core::IInspectable>>,
    {
        Self::ISystemUpdateManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).StateChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        })
    }
    pub fn RemoveStateChanged(token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        Self::ISystemUpdateManagerStatics(|this| unsafe { (windows_core::Interface::vtable(this).RemoveStateChanged)(windows_core::Interface::as_raw(this), token).ok() })
    }
    pub fn DownloadProgress() -> windows_core::Result<f64> {
        Self::ISystemUpdateManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DownloadProgress)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn InstallProgress() -> windows_core::Result<f64> {
        Self::ISystemUpdateManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).InstallProgress)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn UserActiveHoursStart() -> windows_core::Result<super::super::Foundation::TimeSpan> {
        Self::ISystemUpdateManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UserActiveHoursStart)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn UserActiveHoursEnd() -> windows_core::Result<super::super::Foundation::TimeSpan> {
        Self::ISystemUpdateManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UserActiveHoursEnd)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn UserActiveHoursMax() -> windows_core::Result<i32> {
        Self::ISystemUpdateManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UserActiveHoursMax)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn TrySetUserActiveHours(start: super::super::Foundation::TimeSpan, end: super::super::Foundation::TimeSpan) -> windows_core::Result<bool> {
        Self::ISystemUpdateManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TrySetUserActiveHours)(windows_core::Interface::as_raw(this), start, end, &mut result__).map(|| result__)
        })
    }
    pub fn LastUpdateCheckTime() -> windows_core::Result<super::super::Foundation::DateTime> {
        Self::ISystemUpdateManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LastUpdateCheckTime)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn LastUpdateInstallTime() -> windows_core::Result<super::super::Foundation::DateTime> {
        Self::ISystemUpdateManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LastUpdateInstallTime)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn LastErrorInfo() -> windows_core::Result<SystemUpdateLastErrorInfo> {
        Self::ISystemUpdateManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LastErrorInfo)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAutomaticRebootBlockIds() -> windows_core::Result<super::super::Foundation::Collections::IVectorView<windows_core::HSTRING>> {
        Self::ISystemUpdateManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetAutomaticRebootBlockIds)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn BlockAutomaticRebootAsync(lockid: &windows_core::HSTRING) -> windows_core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        Self::ISystemUpdateManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BlockAutomaticRebootAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(lockid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn UnblockAutomaticRebootAsync(lockid: &windows_core::HSTRING) -> windows_core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        Self::ISystemUpdateManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UnblockAutomaticRebootAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(lockid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn ExtendedError() -> windows_core::Result<windows_core::HRESULT> {
        Self::ISystemUpdateManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ExtendedError)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetUpdateItems() -> windows_core::Result<super::super::Foundation::Collections::IVectorView<SystemUpdateItem>> {
        Self::ISystemUpdateManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetUpdateItems)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn AttentionRequiredReason() -> windows_core::Result<SystemUpdateAttentionRequiredReason> {
        Self::ISystemUpdateManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AttentionRequiredReason)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn SetFlightRing(flightring: &windows_core::HSTRING) -> windows_core::Result<bool> {
        Self::ISystemUpdateManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SetFlightRing)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(flightring), &mut result__).map(|| result__)
        })
    }
    pub fn GetFlightRing() -> windows_core::Result<windows_core::HSTRING> {
        Self::ISystemUpdateManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetFlightRing)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn StartInstall(action: SystemUpdateStartInstallAction) -> windows_core::Result<()> {
        Self::ISystemUpdateManagerStatics(|this| unsafe { (windows_core::Interface::vtable(this).StartInstall)(windows_core::Interface::as_raw(this), action).ok() })
    }
    pub fn RebootToCompleteInstall() -> windows_core::Result<()> {
        Self::ISystemUpdateManagerStatics(|this| unsafe { (windows_core::Interface::vtable(this).RebootToCompleteInstall)(windows_core::Interface::as_raw(this)).ok() })
    }
    pub fn StartCancelUpdates() -> windows_core::Result<()> {
        Self::ISystemUpdateManagerStatics(|this| unsafe { (windows_core::Interface::vtable(this).StartCancelUpdates)(windows_core::Interface::as_raw(this)).ok() })
    }
    #[doc(hidden)]
    pub fn ISystemUpdateManagerStatics<R, F: FnOnce(&ISystemUpdateManagerStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<SystemUpdateManager, ISystemUpdateManagerStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeName for SystemUpdateManager {
    const NAME: &'static str = "Windows.System.Update.SystemUpdateManager";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SystemUpdateAttentionRequiredReason(pub i32);
impl SystemUpdateAttentionRequiredReason {
    pub const None: Self = Self(0i32);
    pub const NetworkRequired: Self = Self(1i32);
    pub const InsufficientDiskSpace: Self = Self(2i32);
    pub const InsufficientBattery: Self = Self(3i32);
    pub const UpdateBlocked: Self = Self(4i32);
}
impl windows_core::TypeKind for SystemUpdateAttentionRequiredReason {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SystemUpdateAttentionRequiredReason {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SystemUpdateAttentionRequiredReason").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for SystemUpdateAttentionRequiredReason {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.System.Update.SystemUpdateAttentionRequiredReason;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SystemUpdateItemState(pub i32);
impl SystemUpdateItemState {
    pub const NotStarted: Self = Self(0i32);
    pub const Initializing: Self = Self(1i32);
    pub const Preparing: Self = Self(2i32);
    pub const Calculating: Self = Self(3i32);
    pub const Downloading: Self = Self(4i32);
    pub const Installing: Self = Self(5i32);
    pub const Completed: Self = Self(6i32);
    pub const RebootRequired: Self = Self(7i32);
    pub const Error: Self = Self(8i32);
}
impl windows_core::TypeKind for SystemUpdateItemState {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SystemUpdateItemState {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SystemUpdateItemState").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for SystemUpdateItemState {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.System.Update.SystemUpdateItemState;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SystemUpdateManagerState(pub i32);
impl SystemUpdateManagerState {
    pub const Idle: Self = Self(0i32);
    pub const Detecting: Self = Self(1i32);
    pub const ReadyToDownload: Self = Self(2i32);
    pub const Downloading: Self = Self(3i32);
    pub const ReadyToInstall: Self = Self(4i32);
    pub const Installing: Self = Self(5i32);
    pub const RebootRequired: Self = Self(6i32);
    pub const ReadyToFinalize: Self = Self(7i32);
    pub const Finalizing: Self = Self(8i32);
    pub const Completed: Self = Self(9i32);
    pub const AttentionRequired: Self = Self(10i32);
    pub const Error: Self = Self(11i32);
}
impl windows_core::TypeKind for SystemUpdateManagerState {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SystemUpdateManagerState {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SystemUpdateManagerState").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for SystemUpdateManagerState {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.System.Update.SystemUpdateManagerState;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SystemUpdateStartInstallAction(pub i32);
impl SystemUpdateStartInstallAction {
    pub const UpToReboot: Self = Self(0i32);
    pub const AllowReboot: Self = Self(1i32);
}
impl windows_core::TypeKind for SystemUpdateStartInstallAction {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SystemUpdateStartInstallAction {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SystemUpdateStartInstallAction").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for SystemUpdateStartInstallAction {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.System.Update.SystemUpdateStartInstallAction;i4)");
}
