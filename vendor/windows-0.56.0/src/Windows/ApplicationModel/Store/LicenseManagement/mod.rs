windows_core::imp::define_interface!(ILicenseManagerStatics, ILicenseManagerStatics_Vtbl, 0xb5ac3ae0_da47_4f20_9a23_09182c9476ff);
impl windows_core::RuntimeType for ILicenseManagerStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ILicenseManagerStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Storage_Streams")]
    pub AddLicenseAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    AddLicenseAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetSatisfactionInfosAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetSatisfactionInfosAsync: usize,
}
windows_core::imp::define_interface!(ILicenseManagerStatics2, ILicenseManagerStatics2_Vtbl, 0xab2ec47b_1f79_4480_b87e_2c499e601ba3);
impl windows_core::RuntimeType for ILicenseManagerStatics2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ILicenseManagerStatics2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub RefreshLicensesAsync: unsafe extern "system" fn(*mut core::ffi::c_void, LicenseRefreshOption, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ILicenseSatisfactionInfo, ILicenseSatisfactionInfo_Vtbl, 0x3ccbb08f_db31_48d5_8384_fa17c81474e2);
impl windows_core::RuntimeType for ILicenseSatisfactionInfo {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ILicenseSatisfactionInfo_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SatisfiedByDevice: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SatisfiedByOpenLicense: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SatisfiedByTrial: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SatisfiedByPass: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SatisfiedByInstallMedia: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SatisfiedBySignedInUser: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsSatisfied: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ILicenseSatisfactionResult, ILicenseSatisfactionResult_Vtbl, 0x3c674f73_3c87_4ee1_8201_f428359bd3af);
impl windows_core::RuntimeType for ILicenseSatisfactionResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ILicenseSatisfactionResult_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub LicenseSatisfactionInfos: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    LicenseSatisfactionInfos: usize,
    pub ExtendedError: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::HRESULT) -> windows_core::HRESULT,
}
pub struct LicenseManager;
impl LicenseManager {
    #[cfg(feature = "Storage_Streams")]
    pub fn AddLicenseAsync<P0>(license: P0) -> windows_core::Result<super::super::super::Foundation::IAsyncAction>
    where
        P0: windows_core::Param<super::super::super::Storage::Streams::IBuffer>,
    {
        Self::ILicenseManagerStatics(|this| unsafe {
            let mut result__ = std::mem::zeroed();
            (windows_core::Interface::vtable(this).AddLicenseAsync)(windows_core::Interface::as_raw(this), license.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetSatisfactionInfosAsync<P0, P1>(contentids: P0, keyids: P1) -> windows_core::Result<super::super::super::Foundation::IAsyncOperation<LicenseSatisfactionResult>>
    where
        P0: windows_core::Param<super::super::super::Foundation::Collections::IIterable<windows_core::HSTRING>>,
        P1: windows_core::Param<super::super::super::Foundation::Collections::IIterable<windows_core::HSTRING>>,
    {
        Self::ILicenseManagerStatics(|this| unsafe {
            let mut result__ = std::mem::zeroed();
            (windows_core::Interface::vtable(this).GetSatisfactionInfosAsync)(windows_core::Interface::as_raw(this), contentids.param().abi(), keyids.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn RefreshLicensesAsync(refreshoption: LicenseRefreshOption) -> windows_core::Result<super::super::super::Foundation::IAsyncAction> {
        Self::ILicenseManagerStatics2(|this| unsafe {
            let mut result__ = std::mem::zeroed();
            (windows_core::Interface::vtable(this).RefreshLicensesAsync)(windows_core::Interface::as_raw(this), refreshoption, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn ILicenseManagerStatics<R, F: FnOnce(&ILicenseManagerStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<LicenseManager, ILicenseManagerStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn ILicenseManagerStatics2<R, F: FnOnce(&ILicenseManagerStatics2) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<LicenseManager, ILicenseManagerStatics2> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeName for LicenseManager {
    const NAME: &'static str = "Windows.ApplicationModel.Store.LicenseManagement.LicenseManager";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct LicenseSatisfactionInfo(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(LicenseSatisfactionInfo, windows_core::IUnknown, windows_core::IInspectable);
impl LicenseSatisfactionInfo {
    pub fn SatisfiedByDevice(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = std::mem::zeroed();
            (windows_core::Interface::vtable(this).SatisfiedByDevice)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SatisfiedByOpenLicense(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = std::mem::zeroed();
            (windows_core::Interface::vtable(this).SatisfiedByOpenLicense)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SatisfiedByTrial(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = std::mem::zeroed();
            (windows_core::Interface::vtable(this).SatisfiedByTrial)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SatisfiedByPass(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = std::mem::zeroed();
            (windows_core::Interface::vtable(this).SatisfiedByPass)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SatisfiedByInstallMedia(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = std::mem::zeroed();
            (windows_core::Interface::vtable(this).SatisfiedByInstallMedia)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SatisfiedBySignedInUser(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = std::mem::zeroed();
            (windows_core::Interface::vtable(this).SatisfiedBySignedInUser)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsSatisfied(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = std::mem::zeroed();
            (windows_core::Interface::vtable(this).IsSatisfied)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for LicenseSatisfactionInfo {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ILicenseSatisfactionInfo>();
}
unsafe impl windows_core::Interface for LicenseSatisfactionInfo {
    type Vtable = ILicenseSatisfactionInfo_Vtbl;
    const IID: windows_core::GUID = <ILicenseSatisfactionInfo as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for LicenseSatisfactionInfo {
    const NAME: &'static str = "Windows.ApplicationModel.Store.LicenseManagement.LicenseSatisfactionInfo";
}
unsafe impl Send for LicenseSatisfactionInfo {}
unsafe impl Sync for LicenseSatisfactionInfo {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct LicenseSatisfactionResult(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(LicenseSatisfactionResult, windows_core::IUnknown, windows_core::IInspectable);
impl LicenseSatisfactionResult {
    #[cfg(feature = "Foundation_Collections")]
    pub fn LicenseSatisfactionInfos(&self) -> windows_core::Result<super::super::super::Foundation::Collections::IMapView<windows_core::HSTRING, LicenseSatisfactionInfo>> {
        let this = self;
        unsafe {
            let mut result__ = std::mem::zeroed();
            (windows_core::Interface::vtable(this).LicenseSatisfactionInfos)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ExtendedError(&self) -> windows_core::Result<windows_core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = std::mem::zeroed();
            (windows_core::Interface::vtable(this).ExtendedError)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for LicenseSatisfactionResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ILicenseSatisfactionResult>();
}
unsafe impl windows_core::Interface for LicenseSatisfactionResult {
    type Vtable = ILicenseSatisfactionResult_Vtbl;
    const IID: windows_core::GUID = <ILicenseSatisfactionResult as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for LicenseSatisfactionResult {
    const NAME: &'static str = "Windows.ApplicationModel.Store.LicenseManagement.LicenseSatisfactionResult";
}
unsafe impl Send for LicenseSatisfactionResult {}
unsafe impl Sync for LicenseSatisfactionResult {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct LicenseRefreshOption(pub i32);
impl LicenseRefreshOption {
    pub const RunningLicenses: Self = Self(0i32);
    pub const AllLicenses: Self = Self(1i32);
}
impl windows_core::TypeKind for LicenseRefreshOption {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for LicenseRefreshOption {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("LicenseRefreshOption").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for LicenseRefreshOption {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Store.LicenseManagement.LicenseRefreshOption;i4)");
}
