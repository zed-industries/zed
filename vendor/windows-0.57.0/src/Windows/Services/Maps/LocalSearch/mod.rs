windows_core::imp::define_interface!(ILocalCategoriesStatics, ILocalCategoriesStatics_Vtbl, 0xf49399f5_8261_4321_9974_ef92d49a8dca);
impl windows_core::RuntimeType for ILocalCategoriesStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ILocalCategoriesStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub BankAndCreditUnions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub EatDrink: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Hospitals: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub HotelsAndMotels: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub All: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Parking: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SeeDo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Shop: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ILocalLocation, ILocalLocation_Vtbl, 0xbb0fe9ab_4502_4f2c_94a9_0d60de0e2163);
impl windows_core::RuntimeType for ILocalLocation {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ILocalLocation_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Address: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Identifier: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Description: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub DisplayName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    #[cfg(feature = "Devices_Geolocation")]
    pub Point: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Devices_Geolocation"))]
    Point: usize,
    pub PhoneNumber: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub DataAttribution: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ILocalLocation2, ILocalLocation2_Vtbl, 0x6e9e307c_ecb5_4ffc_bb8c_ba50ba8c2dc6);
impl windows_core::RuntimeType for ILocalLocation2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ILocalLocation2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Category: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub RatingInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub HoursOfOperation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    HoursOfOperation: usize,
}
windows_core::imp::define_interface!(ILocalLocationFinderResult, ILocalLocationFinderResult_Vtbl, 0xd09b6cc6_f338_4191_9fd8_5440b9a68f52);
impl windows_core::RuntimeType for ILocalLocationFinderResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ILocalLocationFinderResult_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub LocalLocations: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    LocalLocations: usize,
    pub Status: unsafe extern "system" fn(*mut core::ffi::c_void, *mut LocalLocationFinderStatus) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ILocalLocationFinderStatics, ILocalLocationFinderStatics_Vtbl, 0xd2ef7344_a0de_48ca_81a8_07c7dcfd37ab);
impl windows_core::RuntimeType for ILocalLocationFinderStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ILocalLocationFinderStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Devices_Geolocation")]
    pub FindLocalLocationsAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Devices_Geolocation"))]
    FindLocalLocationsAsync: usize,
}
windows_core::imp::define_interface!(ILocalLocationHoursOfOperationItem, ILocalLocationHoursOfOperationItem_Vtbl, 0x23548c72_a1c7_43f1_a4f0_1091c39ec640);
impl windows_core::RuntimeType for ILocalLocationHoursOfOperationItem {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ILocalLocationHoursOfOperationItem_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Globalization")]
    pub Day: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Globalization::DayOfWeek) -> windows_core::HRESULT,
    #[cfg(not(feature = "Globalization"))]
    Day: usize,
    pub Start: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::TimeSpan) -> windows_core::HRESULT,
    pub Span: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::TimeSpan) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ILocalLocationRatingInfo, ILocalLocationRatingInfo_Vtbl, 0xcb1dab56_3354_4311_8bc0_a2d4d5eb806e);
impl windows_core::RuntimeType for ILocalLocationRatingInfo {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ILocalLocationRatingInfo_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub AggregateRating: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RatingCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ProviderIdentifier: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPlaceInfoHelperStatics, IPlaceInfoHelperStatics_Vtbl, 0xdd1ca9a7_a9c6_491b_bc09_e80fcea48ee6);
impl windows_core::RuntimeType for IPlaceInfoHelperStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPlaceInfoHelperStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateFromLocalLocation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub struct LocalCategories;
impl LocalCategories {
    pub fn BankAndCreditUnions() -> windows_core::Result<windows_core::HSTRING> {
        Self::ILocalCategoriesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BankAndCreditUnions)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn EatDrink() -> windows_core::Result<windows_core::HSTRING> {
        Self::ILocalCategoriesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).EatDrink)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn Hospitals() -> windows_core::Result<windows_core::HSTRING> {
        Self::ILocalCategoriesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Hospitals)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn HotelsAndMotels() -> windows_core::Result<windows_core::HSTRING> {
        Self::ILocalCategoriesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HotelsAndMotels)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn All() -> windows_core::Result<windows_core::HSTRING> {
        Self::ILocalCategoriesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).All)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn Parking() -> windows_core::Result<windows_core::HSTRING> {
        Self::ILocalCategoriesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Parking)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn SeeDo() -> windows_core::Result<windows_core::HSTRING> {
        Self::ILocalCategoriesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SeeDo)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn Shop() -> windows_core::Result<windows_core::HSTRING> {
        Self::ILocalCategoriesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Shop)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn ILocalCategoriesStatics<R, F: FnOnce(&ILocalCategoriesStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<LocalCategories, ILocalCategoriesStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeName for LocalCategories {
    const NAME: &'static str = "Windows.Services.Maps.LocalSearch.LocalCategories";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct LocalLocation(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(LocalLocation, windows_core::IUnknown, windows_core::IInspectable);
impl LocalLocation {
    pub fn Address(&self) -> windows_core::Result<super::MapAddress> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Address)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Identifier(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Identifier)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Description(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Description)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn DisplayName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DisplayName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Devices_Geolocation")]
    pub fn Point(&self) -> windows_core::Result<super::super::super::Devices::Geolocation::Geopoint> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Point)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn PhoneNumber(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PhoneNumber)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn DataAttribution(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DataAttribution)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Category(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<ILocalLocation2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Category)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn RatingInfo(&self) -> windows_core::Result<LocalLocationRatingInfo> {
        let this = &windows_core::Interface::cast::<ILocalLocation2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RatingInfo)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn HoursOfOperation(&self) -> windows_core::Result<super::super::super::Foundation::Collections::IVectorView<LocalLocationHoursOfOperationItem>> {
        let this = &windows_core::Interface::cast::<ILocalLocation2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HoursOfOperation)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for LocalLocation {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ILocalLocation>();
}
unsafe impl windows_core::Interface for LocalLocation {
    type Vtable = ILocalLocation_Vtbl;
    const IID: windows_core::GUID = <ILocalLocation as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for LocalLocation {
    const NAME: &'static str = "Windows.Services.Maps.LocalSearch.LocalLocation";
}
unsafe impl Send for LocalLocation {}
unsafe impl Sync for LocalLocation {}
pub struct LocalLocationFinder;
impl LocalLocationFinder {
    #[cfg(feature = "Devices_Geolocation")]
    pub fn FindLocalLocationsAsync<P0>(searchterm: &windows_core::HSTRING, searcharea: P0, localcategory: &windows_core::HSTRING, maxresults: u32) -> windows_core::Result<super::super::super::Foundation::IAsyncOperation<LocalLocationFinderResult>>
    where
        P0: windows_core::Param<super::super::super::Devices::Geolocation::Geocircle>,
    {
        Self::ILocalLocationFinderStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FindLocalLocationsAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(searchterm), searcharea.param().abi(), core::mem::transmute_copy(localcategory), maxresults, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn ILocalLocationFinderStatics<R, F: FnOnce(&ILocalLocationFinderStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<LocalLocationFinder, ILocalLocationFinderStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeName for LocalLocationFinder {
    const NAME: &'static str = "Windows.Services.Maps.LocalSearch.LocalLocationFinder";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct LocalLocationFinderResult(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(LocalLocationFinderResult, windows_core::IUnknown, windows_core::IInspectable);
impl LocalLocationFinderResult {
    #[cfg(feature = "Foundation_Collections")]
    pub fn LocalLocations(&self) -> windows_core::Result<super::super::super::Foundation::Collections::IVectorView<LocalLocation>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LocalLocations)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Status(&self) -> windows_core::Result<LocalLocationFinderStatus> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Status)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for LocalLocationFinderResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ILocalLocationFinderResult>();
}
unsafe impl windows_core::Interface for LocalLocationFinderResult {
    type Vtable = ILocalLocationFinderResult_Vtbl;
    const IID: windows_core::GUID = <ILocalLocationFinderResult as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for LocalLocationFinderResult {
    const NAME: &'static str = "Windows.Services.Maps.LocalSearch.LocalLocationFinderResult";
}
unsafe impl Send for LocalLocationFinderResult {}
unsafe impl Sync for LocalLocationFinderResult {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct LocalLocationHoursOfOperationItem(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(LocalLocationHoursOfOperationItem, windows_core::IUnknown, windows_core::IInspectable);
impl LocalLocationHoursOfOperationItem {
    #[cfg(feature = "Globalization")]
    pub fn Day(&self) -> windows_core::Result<super::super::super::Globalization::DayOfWeek> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Day)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Start(&self) -> windows_core::Result<super::super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Start)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Span(&self) -> windows_core::Result<super::super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Span)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for LocalLocationHoursOfOperationItem {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ILocalLocationHoursOfOperationItem>();
}
unsafe impl windows_core::Interface for LocalLocationHoursOfOperationItem {
    type Vtable = ILocalLocationHoursOfOperationItem_Vtbl;
    const IID: windows_core::GUID = <ILocalLocationHoursOfOperationItem as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for LocalLocationHoursOfOperationItem {
    const NAME: &'static str = "Windows.Services.Maps.LocalSearch.LocalLocationHoursOfOperationItem";
}
unsafe impl Send for LocalLocationHoursOfOperationItem {}
unsafe impl Sync for LocalLocationHoursOfOperationItem {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct LocalLocationRatingInfo(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(LocalLocationRatingInfo, windows_core::IUnknown, windows_core::IInspectable);
impl LocalLocationRatingInfo {
    pub fn AggregateRating(&self) -> windows_core::Result<super::super::super::Foundation::IReference<f64>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AggregateRating)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn RatingCount(&self) -> windows_core::Result<super::super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RatingCount)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ProviderIdentifier(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ProviderIdentifier)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for LocalLocationRatingInfo {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ILocalLocationRatingInfo>();
}
unsafe impl windows_core::Interface for LocalLocationRatingInfo {
    type Vtable = ILocalLocationRatingInfo_Vtbl;
    const IID: windows_core::GUID = <ILocalLocationRatingInfo as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for LocalLocationRatingInfo {
    const NAME: &'static str = "Windows.Services.Maps.LocalSearch.LocalLocationRatingInfo";
}
unsafe impl Send for LocalLocationRatingInfo {}
unsafe impl Sync for LocalLocationRatingInfo {}
pub struct PlaceInfoHelper;
impl PlaceInfoHelper {
    pub fn CreateFromLocalLocation<P0>(location: P0) -> windows_core::Result<super::PlaceInfo>
    where
        P0: windows_core::Param<LocalLocation>,
    {
        Self::IPlaceInfoHelperStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFromLocalLocation)(windows_core::Interface::as_raw(this), location.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IPlaceInfoHelperStatics<R, F: FnOnce(&IPlaceInfoHelperStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<PlaceInfoHelper, IPlaceInfoHelperStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeName for PlaceInfoHelper {
    const NAME: &'static str = "Windows.Services.Maps.LocalSearch.PlaceInfoHelper";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct LocalLocationFinderStatus(pub i32);
impl LocalLocationFinderStatus {
    pub const Success: Self = Self(0i32);
    pub const UnknownError: Self = Self(1i32);
    pub const InvalidCredentials: Self = Self(2i32);
    pub const InvalidCategory: Self = Self(3i32);
    pub const InvalidSearchTerm: Self = Self(4i32);
    pub const InvalidSearchArea: Self = Self(5i32);
    pub const NetworkFailure: Self = Self(6i32);
    pub const NotSupported: Self = Self(7i32);
}
impl windows_core::TypeKind for LocalLocationFinderStatus {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for LocalLocationFinderStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("LocalLocationFinderStatus").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for LocalLocationFinderStatus {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Services.Maps.LocalSearch.LocalLocationFinderStatus;i4)");
}
