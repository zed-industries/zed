#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub trait ICivicAddressReport_Impl: Sized + ILocationReport_Impl {
    fn GetAddressLine1(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetAddressLine2(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetCity(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetStateProvince(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetPostalCode(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetCountryRegion(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetDetailLevel(&self) -> windows_core::Result<u32>;
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl windows_core::RuntimeName for ICivicAddressReport {}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ICivicAddressReport_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICivicAddressReport_Impl, const OFFSET: isize>() -> ICivicAddressReport_Vtbl {
        unsafe extern "system" fn GetAddressLine1<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICivicAddressReport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstraddress1: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ICivicAddressReport_Impl::GetAddressLine1(this) {
                Ok(ok__) => {
                    core::ptr::write(pbstraddress1, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAddressLine2<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICivicAddressReport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstraddress2: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ICivicAddressReport_Impl::GetAddressLine2(this) {
                Ok(ok__) => {
                    core::ptr::write(pbstraddress2, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCity<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICivicAddressReport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrcity: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ICivicAddressReport_Impl::GetCity(this) {
                Ok(ok__) => {
                    core::ptr::write(pbstrcity, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStateProvince<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICivicAddressReport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrstateprovince: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ICivicAddressReport_Impl::GetStateProvince(this) {
                Ok(ok__) => {
                    core::ptr::write(pbstrstateprovince, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPostalCode<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICivicAddressReport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrpostalcode: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ICivicAddressReport_Impl::GetPostalCode(this) {
                Ok(ok__) => {
                    core::ptr::write(pbstrpostalcode, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCountryRegion<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICivicAddressReport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrcountryregion: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ICivicAddressReport_Impl::GetCountryRegion(this) {
                Ok(ok__) => {
                    core::ptr::write(pbstrcountryregion, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDetailLevel<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICivicAddressReport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdetaillevel: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ICivicAddressReport_Impl::GetDetailLevel(this) {
                Ok(ok__) => {
                    core::ptr::write(pdetaillevel, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: ILocationReport_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetAddressLine1: GetAddressLine1::<Identity, Impl, OFFSET>,
            GetAddressLine2: GetAddressLine2::<Identity, Impl, OFFSET>,
            GetCity: GetCity::<Identity, Impl, OFFSET>,
            GetStateProvince: GetStateProvince::<Identity, Impl, OFFSET>,
            GetPostalCode: GetPostalCode::<Identity, Impl, OFFSET>,
            GetCountryRegion: GetCountryRegion::<Identity, Impl, OFFSET>,
            GetDetailLevel: GetDetailLevel::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICivicAddressReport as windows_core::Interface>::IID || iid == &<ILocationReport as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ICivicAddressReportFactory_Impl: Sized + ILocationReportFactory_Impl {
    fn CivicAddressReport(&self) -> windows_core::Result<IDispCivicAddressReport>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ICivicAddressReportFactory {}
#[cfg(feature = "Win32_System_Com")]
impl ICivicAddressReportFactory_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICivicAddressReportFactory_Impl, const OFFSET: isize>() -> ICivicAddressReportFactory_Vtbl {
        unsafe extern "system" fn CivicAddressReport<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICivicAddressReportFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ICivicAddressReportFactory_Impl::CivicAddressReport(this) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: ILocationReportFactory_Vtbl::new::<Identity, Impl, OFFSET>(), CivicAddressReport: CivicAddressReport::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICivicAddressReportFactory as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<ILocationReportFactory as windows_core::Interface>::IID
    }
}
pub trait IDefaultLocation_Impl: Sized {
    fn SetReport(&self, reporttype: *const windows_core::GUID, plocationreport: Option<&ILocationReport>) -> windows_core::Result<()>;
    fn GetReport(&self, reporttype: *const windows_core::GUID) -> windows_core::Result<ILocationReport>;
}
impl windows_core::RuntimeName for IDefaultLocation {}
impl IDefaultLocation_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDefaultLocation_Impl, const OFFSET: isize>() -> IDefaultLocation_Vtbl {
        unsafe extern "system" fn SetReport<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDefaultLocation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, reporttype: *const windows_core::GUID, plocationreport: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDefaultLocation_Impl::SetReport(this, core::mem::transmute_copy(&reporttype), windows_core::from_raw_borrowed(&plocationreport)).into()
        }
        unsafe extern "system" fn GetReport<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDefaultLocation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, reporttype: *const windows_core::GUID, pplocationreport: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IDefaultLocation_Impl::GetReport(this, core::mem::transmute_copy(&reporttype)) {
                Ok(ok__) => {
                    core::ptr::write(pplocationreport, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetReport: SetReport::<Identity, Impl, OFFSET>,
            GetReport: GetReport::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDefaultLocation as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IDispCivicAddressReport_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn AddressLine1(&self) -> windows_core::Result<windows_core::BSTR>;
    fn AddressLine2(&self) -> windows_core::Result<windows_core::BSTR>;
    fn City(&self) -> windows_core::Result<windows_core::BSTR>;
    fn StateProvince(&self) -> windows_core::Result<windows_core::BSTR>;
    fn PostalCode(&self) -> windows_core::Result<windows_core::BSTR>;
    fn CountryRegion(&self) -> windows_core::Result<windows_core::BSTR>;
    fn DetailLevel(&self) -> windows_core::Result<u32>;
    fn Timestamp(&self) -> windows_core::Result<f64>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IDispCivicAddressReport {}
#[cfg(feature = "Win32_System_Com")]
impl IDispCivicAddressReport_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDispCivicAddressReport_Impl, const OFFSET: isize>() -> IDispCivicAddressReport_Vtbl {
        unsafe extern "system" fn AddressLine1<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDispCivicAddressReport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, paddress1: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IDispCivicAddressReport_Impl::AddressLine1(this) {
                Ok(ok__) => {
                    core::ptr::write(paddress1, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddressLine2<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDispCivicAddressReport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, paddress2: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IDispCivicAddressReport_Impl::AddressLine2(this) {
                Ok(ok__) => {
                    core::ptr::write(paddress2, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn City<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDispCivicAddressReport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcity: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IDispCivicAddressReport_Impl::City(this) {
                Ok(ok__) => {
                    core::ptr::write(pcity, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StateProvince<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDispCivicAddressReport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstateprovince: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IDispCivicAddressReport_Impl::StateProvince(this) {
                Ok(ok__) => {
                    core::ptr::write(pstateprovince, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PostalCode<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDispCivicAddressReport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppostalcode: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IDispCivicAddressReport_Impl::PostalCode(this) {
                Ok(ok__) => {
                    core::ptr::write(ppostalcode, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CountryRegion<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDispCivicAddressReport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcountryregion: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IDispCivicAddressReport_Impl::CountryRegion(this) {
                Ok(ok__) => {
                    core::ptr::write(pcountryregion, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DetailLevel<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDispCivicAddressReport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdetaillevel: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IDispCivicAddressReport_Impl::DetailLevel(this) {
                Ok(ok__) => {
                    core::ptr::write(pdetaillevel, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Timestamp<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDispCivicAddressReport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut f64) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IDispCivicAddressReport_Impl::Timestamp(this) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            AddressLine1: AddressLine1::<Identity, Impl, OFFSET>,
            AddressLine2: AddressLine2::<Identity, Impl, OFFSET>,
            City: City::<Identity, Impl, OFFSET>,
            StateProvince: StateProvince::<Identity, Impl, OFFSET>,
            PostalCode: PostalCode::<Identity, Impl, OFFSET>,
            CountryRegion: CountryRegion::<Identity, Impl, OFFSET>,
            DetailLevel: DetailLevel::<Identity, Impl, OFFSET>,
            Timestamp: Timestamp::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDispCivicAddressReport as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IDispLatLongReport_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Latitude(&self) -> windows_core::Result<f64>;
    fn Longitude(&self) -> windows_core::Result<f64>;
    fn ErrorRadius(&self) -> windows_core::Result<f64>;
    fn Altitude(&self) -> windows_core::Result<f64>;
    fn AltitudeError(&self) -> windows_core::Result<f64>;
    fn Timestamp(&self) -> windows_core::Result<f64>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IDispLatLongReport {}
#[cfg(feature = "Win32_System_Com")]
impl IDispLatLongReport_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDispLatLongReport_Impl, const OFFSET: isize>() -> IDispLatLongReport_Vtbl {
        unsafe extern "system" fn Latitude<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDispLatLongReport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut f64) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IDispLatLongReport_Impl::Latitude(this) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Longitude<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDispLatLongReport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut f64) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IDispLatLongReport_Impl::Longitude(this) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ErrorRadius<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDispLatLongReport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut f64) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IDispLatLongReport_Impl::ErrorRadius(this) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Altitude<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDispLatLongReport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut f64) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IDispLatLongReport_Impl::Altitude(this) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AltitudeError<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDispLatLongReport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut f64) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IDispLatLongReport_Impl::AltitudeError(this) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Timestamp<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDispLatLongReport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut f64) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IDispLatLongReport_Impl::Timestamp(this) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Latitude: Latitude::<Identity, Impl, OFFSET>,
            Longitude: Longitude::<Identity, Impl, OFFSET>,
            ErrorRadius: ErrorRadius::<Identity, Impl, OFFSET>,
            Altitude: Altitude::<Identity, Impl, OFFSET>,
            AltitudeError: AltitudeError::<Identity, Impl, OFFSET>,
            Timestamp: Timestamp::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDispLatLongReport as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub trait ILatLongReport_Impl: Sized + ILocationReport_Impl {
    fn GetLatitude(&self) -> windows_core::Result<f64>;
    fn GetLongitude(&self) -> windows_core::Result<f64>;
    fn GetErrorRadius(&self) -> windows_core::Result<f64>;
    fn GetAltitude(&self) -> windows_core::Result<f64>;
    fn GetAltitudeError(&self) -> windows_core::Result<f64>;
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl windows_core::RuntimeName for ILatLongReport {}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ILatLongReport_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ILatLongReport_Impl, const OFFSET: isize>() -> ILatLongReport_Vtbl {
        unsafe extern "system" fn GetLatitude<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ILatLongReport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, platitude: *mut f64) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ILatLongReport_Impl::GetLatitude(this) {
                Ok(ok__) => {
                    core::ptr::write(platitude, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLongitude<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ILatLongReport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plongitude: *mut f64) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ILatLongReport_Impl::GetLongitude(this) {
                Ok(ok__) => {
                    core::ptr::write(plongitude, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetErrorRadius<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ILatLongReport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, perrorradius: *mut f64) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ILatLongReport_Impl::GetErrorRadius(this) {
                Ok(ok__) => {
                    core::ptr::write(perrorradius, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAltitude<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ILatLongReport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, paltitude: *mut f64) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ILatLongReport_Impl::GetAltitude(this) {
                Ok(ok__) => {
                    core::ptr::write(paltitude, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAltitudeError<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ILatLongReport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, paltitudeerror: *mut f64) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ILatLongReport_Impl::GetAltitudeError(this) {
                Ok(ok__) => {
                    core::ptr::write(paltitudeerror, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: ILocationReport_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetLatitude: GetLatitude::<Identity, Impl, OFFSET>,
            GetLongitude: GetLongitude::<Identity, Impl, OFFSET>,
            GetErrorRadius: GetErrorRadius::<Identity, Impl, OFFSET>,
            GetAltitude: GetAltitude::<Identity, Impl, OFFSET>,
            GetAltitudeError: GetAltitudeError::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ILatLongReport as windows_core::Interface>::IID || iid == &<ILocationReport as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ILatLongReportFactory_Impl: Sized + ILocationReportFactory_Impl {
    fn LatLongReport(&self) -> windows_core::Result<IDispLatLongReport>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ILatLongReportFactory {}
#[cfg(feature = "Win32_System_Com")]
impl ILatLongReportFactory_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ILatLongReportFactory_Impl, const OFFSET: isize>() -> ILatLongReportFactory_Vtbl {
        unsafe extern "system" fn LatLongReport<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ILatLongReportFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ILatLongReportFactory_Impl::LatLongReport(this) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: ILocationReportFactory_Vtbl::new::<Identity, Impl, OFFSET>(), LatLongReport: LatLongReport::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ILatLongReportFactory as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<ILocationReportFactory as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Devices_Sensors")]
pub trait ILocation_Impl: Sized {
    fn RegisterForReport(&self, pevents: Option<&ILocationEvents>, reporttype: *const windows_core::GUID, dwrequestedreportinterval: u32) -> windows_core::Result<()>;
    fn UnregisterForReport(&self, reporttype: *const windows_core::GUID) -> windows_core::Result<()>;
    fn GetReport(&self, reporttype: *const windows_core::GUID) -> windows_core::Result<ILocationReport>;
    fn GetReportStatus(&self, reporttype: *const windows_core::GUID) -> windows_core::Result<LOCATION_REPORT_STATUS>;
    fn GetReportInterval(&self, reporttype: *const windows_core::GUID) -> windows_core::Result<u32>;
    fn SetReportInterval(&self, reporttype: *const windows_core::GUID, millisecondsrequested: u32) -> windows_core::Result<()>;
    fn GetDesiredAccuracy(&self, reporttype: *const windows_core::GUID) -> windows_core::Result<super::Sensors::LOCATION_DESIRED_ACCURACY>;
    fn SetDesiredAccuracy(&self, reporttype: *const windows_core::GUID, desiredaccuracy: super::Sensors::LOCATION_DESIRED_ACCURACY) -> windows_core::Result<()>;
    fn RequestPermissions(&self, hparent: super::super::Foundation::HWND, preporttypes: *const windows_core::GUID, count: u32, fmodal: super::super::Foundation::BOOL) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_Devices_Sensors")]
impl windows_core::RuntimeName for ILocation {}
#[cfg(feature = "Win32_Devices_Sensors")]
impl ILocation_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ILocation_Impl, const OFFSET: isize>() -> ILocation_Vtbl {
        unsafe extern "system" fn RegisterForReport<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ILocation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pevents: *mut core::ffi::c_void, reporttype: *const windows_core::GUID, dwrequestedreportinterval: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ILocation_Impl::RegisterForReport(this, windows_core::from_raw_borrowed(&pevents), core::mem::transmute_copy(&reporttype), core::mem::transmute_copy(&dwrequestedreportinterval)).into()
        }
        unsafe extern "system" fn UnregisterForReport<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ILocation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, reporttype: *const windows_core::GUID) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ILocation_Impl::UnregisterForReport(this, core::mem::transmute_copy(&reporttype)).into()
        }
        unsafe extern "system" fn GetReport<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ILocation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, reporttype: *const windows_core::GUID, pplocationreport: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ILocation_Impl::GetReport(this, core::mem::transmute_copy(&reporttype)) {
                Ok(ok__) => {
                    core::ptr::write(pplocationreport, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetReportStatus<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ILocation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, reporttype: *const windows_core::GUID, pstatus: *mut LOCATION_REPORT_STATUS) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ILocation_Impl::GetReportStatus(this, core::mem::transmute_copy(&reporttype)) {
                Ok(ok__) => {
                    core::ptr::write(pstatus, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetReportInterval<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ILocation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, reporttype: *const windows_core::GUID, pmilliseconds: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ILocation_Impl::GetReportInterval(this, core::mem::transmute_copy(&reporttype)) {
                Ok(ok__) => {
                    core::ptr::write(pmilliseconds, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetReportInterval<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ILocation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, reporttype: *const windows_core::GUID, millisecondsrequested: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ILocation_Impl::SetReportInterval(this, core::mem::transmute_copy(&reporttype), core::mem::transmute_copy(&millisecondsrequested)).into()
        }
        unsafe extern "system" fn GetDesiredAccuracy<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ILocation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, reporttype: *const windows_core::GUID, pdesiredaccuracy: *mut super::Sensors::LOCATION_DESIRED_ACCURACY) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ILocation_Impl::GetDesiredAccuracy(this, core::mem::transmute_copy(&reporttype)) {
                Ok(ok__) => {
                    core::ptr::write(pdesiredaccuracy, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDesiredAccuracy<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ILocation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, reporttype: *const windows_core::GUID, desiredaccuracy: super::Sensors::LOCATION_DESIRED_ACCURACY) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ILocation_Impl::SetDesiredAccuracy(this, core::mem::transmute_copy(&reporttype), core::mem::transmute_copy(&desiredaccuracy)).into()
        }
        unsafe extern "system" fn RequestPermissions<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ILocation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hparent: super::super::Foundation::HWND, preporttypes: *const windows_core::GUID, count: u32, fmodal: super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ILocation_Impl::RequestPermissions(this, core::mem::transmute_copy(&hparent), core::mem::transmute_copy(&preporttypes), core::mem::transmute_copy(&count), core::mem::transmute_copy(&fmodal)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            RegisterForReport: RegisterForReport::<Identity, Impl, OFFSET>,
            UnregisterForReport: UnregisterForReport::<Identity, Impl, OFFSET>,
            GetReport: GetReport::<Identity, Impl, OFFSET>,
            GetReportStatus: GetReportStatus::<Identity, Impl, OFFSET>,
            GetReportInterval: GetReportInterval::<Identity, Impl, OFFSET>,
            SetReportInterval: SetReportInterval::<Identity, Impl, OFFSET>,
            GetDesiredAccuracy: GetDesiredAccuracy::<Identity, Impl, OFFSET>,
            SetDesiredAccuracy: SetDesiredAccuracy::<Identity, Impl, OFFSET>,
            RequestPermissions: RequestPermissions::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ILocation as windows_core::Interface>::IID
    }
}
pub trait ILocationEvents_Impl: Sized {
    fn OnLocationChanged(&self, reporttype: *const windows_core::GUID, plocationreport: Option<&ILocationReport>) -> windows_core::Result<()>;
    fn OnStatusChanged(&self, reporttype: *const windows_core::GUID, newstatus: LOCATION_REPORT_STATUS) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ILocationEvents {}
impl ILocationEvents_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ILocationEvents_Impl, const OFFSET: isize>() -> ILocationEvents_Vtbl {
        unsafe extern "system" fn OnLocationChanged<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ILocationEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, reporttype: *const windows_core::GUID, plocationreport: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ILocationEvents_Impl::OnLocationChanged(this, core::mem::transmute_copy(&reporttype), windows_core::from_raw_borrowed(&plocationreport)).into()
        }
        unsafe extern "system" fn OnStatusChanged<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ILocationEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, reporttype: *const windows_core::GUID, newstatus: LOCATION_REPORT_STATUS) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ILocationEvents_Impl::OnStatusChanged(this, core::mem::transmute_copy(&reporttype), core::mem::transmute_copy(&newstatus)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnLocationChanged: OnLocationChanged::<Identity, Impl, OFFSET>,
            OnStatusChanged: OnStatusChanged::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ILocationEvents as windows_core::Interface>::IID
    }
}
pub trait ILocationPower_Impl: Sized {
    fn Connect(&self) -> windows_core::Result<()>;
    fn Disconnect(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ILocationPower {}
impl ILocationPower_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ILocationPower_Impl, const OFFSET: isize>() -> ILocationPower_Vtbl {
        unsafe extern "system" fn Connect<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ILocationPower_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ILocationPower_Impl::Connect(this).into()
        }
        unsafe extern "system" fn Disconnect<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ILocationPower_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ILocationPower_Impl::Disconnect(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Connect: Connect::<Identity, Impl, OFFSET>,
            Disconnect: Disconnect::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ILocationPower as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub trait ILocationReport_Impl: Sized {
    fn GetSensorID(&self) -> windows_core::Result<windows_core::GUID>;
    fn GetTimestamp(&self) -> windows_core::Result<super::super::Foundation::SYSTEMTIME>;
    fn GetValue(&self, pkey: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> windows_core::Result<windows_core::PROPVARIANT>;
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl windows_core::RuntimeName for ILocationReport {}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ILocationReport_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ILocationReport_Impl, const OFFSET: isize>() -> ILocationReport_Vtbl {
        unsafe extern "system" fn GetSensorID<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ILocationReport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psensorid: *mut windows_core::GUID) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ILocationReport_Impl::GetSensorID(this) {
                Ok(ok__) => {
                    core::ptr::write(psensorid, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTimestamp<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ILocationReport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcreationtime: *mut super::super::Foundation::SYSTEMTIME) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ILocationReport_Impl::GetTimestamp(this) {
                Ok(ok__) => {
                    core::ptr::write(pcreationtime, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetValue<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ILocationReport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pkey: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: *mut core::mem::MaybeUninit<windows_core::PROPVARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ILocationReport_Impl::GetValue(this, core::mem::transmute_copy(&pkey)) {
                Ok(ok__) => {
                    core::ptr::write(pvalue, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetSensorID: GetSensorID::<Identity, Impl, OFFSET>,
            GetTimestamp: GetTimestamp::<Identity, Impl, OFFSET>,
            GetValue: GetValue::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ILocationReport as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ILocationReportFactory_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn ListenForReports(&self, requestedreportinterval: u32) -> windows_core::Result<()>;
    fn StopListeningForReports(&self) -> windows_core::Result<()>;
    fn Status(&self) -> windows_core::Result<u32>;
    fn ReportInterval(&self) -> windows_core::Result<u32>;
    fn SetReportInterval(&self, millisecondsrequested: u32) -> windows_core::Result<()>;
    fn DesiredAccuracy(&self) -> windows_core::Result<u32>;
    fn SetDesiredAccuracy(&self, desiredaccuracy: u32) -> windows_core::Result<()>;
    fn RequestPermissions(&self, hwnd: *const u32) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ILocationReportFactory {}
#[cfg(feature = "Win32_System_Com")]
impl ILocationReportFactory_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ILocationReportFactory_Impl, const OFFSET: isize>() -> ILocationReportFactory_Vtbl {
        unsafe extern "system" fn ListenForReports<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ILocationReportFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, requestedreportinterval: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ILocationReportFactory_Impl::ListenForReports(this, core::mem::transmute_copy(&requestedreportinterval)).into()
        }
        unsafe extern "system" fn StopListeningForReports<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ILocationReportFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ILocationReportFactory_Impl::StopListeningForReports(this).into()
        }
        unsafe extern "system" fn Status<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ILocationReportFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ILocationReportFactory_Impl::Status(this) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReportInterval<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ILocationReportFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmilliseconds: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ILocationReportFactory_Impl::ReportInterval(this) {
                Ok(ok__) => {
                    core::ptr::write(pmilliseconds, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetReportInterval<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ILocationReportFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, millisecondsrequested: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ILocationReportFactory_Impl::SetReportInterval(this, core::mem::transmute_copy(&millisecondsrequested)).into()
        }
        unsafe extern "system" fn DesiredAccuracy<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ILocationReportFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesiredaccuracy: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ILocationReportFactory_Impl::DesiredAccuracy(this) {
                Ok(ok__) => {
                    core::ptr::write(pdesiredaccuracy, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDesiredAccuracy<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ILocationReportFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, desiredaccuracy: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ILocationReportFactory_Impl::SetDesiredAccuracy(this, core::mem::transmute_copy(&desiredaccuracy)).into()
        }
        unsafe extern "system" fn RequestPermissions<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ILocationReportFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwnd: *const u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ILocationReportFactory_Impl::RequestPermissions(this, core::mem::transmute_copy(&hwnd)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            ListenForReports: ListenForReports::<Identity, Impl, OFFSET>,
            StopListeningForReports: StopListeningForReports::<Identity, Impl, OFFSET>,
            Status: Status::<Identity, Impl, OFFSET>,
            ReportInterval: ReportInterval::<Identity, Impl, OFFSET>,
            SetReportInterval: SetReportInterval::<Identity, Impl, OFFSET>,
            DesiredAccuracy: DesiredAccuracy::<Identity, Impl, OFFSET>,
            SetDesiredAccuracy: SetDesiredAccuracy::<Identity, Impl, OFFSET>,
            RequestPermissions: RequestPermissions::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ILocationReportFactory as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait _ICivicAddressReportFactoryEvents_Impl: Sized + super::super::System::Com::IDispatch_Impl {}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for _ICivicAddressReportFactoryEvents {}
#[cfg(feature = "Win32_System_Com")]
impl _ICivicAddressReportFactoryEvents_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: _ICivicAddressReportFactoryEvents_Impl, const OFFSET: isize>() -> _ICivicAddressReportFactoryEvents_Vtbl {
        Self { base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<_ICivicAddressReportFactoryEvents as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait _ILatLongReportFactoryEvents_Impl: Sized + super::super::System::Com::IDispatch_Impl {}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for _ILatLongReportFactoryEvents {}
#[cfg(feature = "Win32_System_Com")]
impl _ILatLongReportFactoryEvents_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: _ILatLongReportFactoryEvents_Impl, const OFFSET: isize>() -> _ILatLongReportFactoryEvents_Vtbl {
        Self { base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<_ILatLongReportFactoryEvents as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
