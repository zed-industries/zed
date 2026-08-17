windows_core::imp::define_interface!(IGeofence, IGeofence_Vtbl, 0x9c090823_edb8_47e0_8245_5bf61d321f2d);
impl windows_core::RuntimeType for IGeofence {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IGeofence_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub StartTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::DateTime) -> windows_core::HRESULT,
    pub Duration: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::TimeSpan) -> windows_core::HRESULT,
    pub DwellTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::TimeSpan) -> windows_core::HRESULT,
    pub Id: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub MonitoredStates: unsafe extern "system" fn(*mut core::ffi::c_void, *mut MonitoredGeofenceStates) -> windows_core::HRESULT,
    pub Geoshape: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SingleUse: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IGeofenceFactory, IGeofenceFactory_Vtbl, 0x841f624b_325f_4b90_bca7_2b8022a93796);
impl windows_core::RuntimeType for IGeofenceFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IGeofenceFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateWithMonitorStates: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut core::ffi::c_void, MonitoredGeofenceStates, bool, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateWithMonitorStatesAndDwellTime: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut core::ffi::c_void, MonitoredGeofenceStates, bool, super::super::super::Foundation::TimeSpan, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateWithMonitorStatesDwellTimeStartTimeAndDuration: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut core::ffi::c_void, MonitoredGeofenceStates, bool, super::super::super::Foundation::TimeSpan, super::super::super::Foundation::DateTime, super::super::super::Foundation::TimeSpan, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IGeofenceMonitor, IGeofenceMonitor_Vtbl, 0x4c0f5f78_1c1f_4621_bbbd_833b92247226);
impl windows_core::RuntimeType for IGeofenceMonitor {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IGeofenceMonitor_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Status: unsafe extern "system" fn(*mut core::ffi::c_void, *mut GeofenceMonitorStatus) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Geofences: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Geofences: usize,
    pub LastKnownGeoposition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GeofenceStateChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveGeofenceStateChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub ReadReports: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ReadReports: usize,
    pub StatusChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveStatusChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IGeofenceMonitorStatics, IGeofenceMonitorStatics_Vtbl, 0x2dd32fcf_7e75_4899_ace3_2bd0a65cce06);
impl windows_core::RuntimeType for IGeofenceMonitorStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IGeofenceMonitorStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Current: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IGeofenceStateChangeReport, IGeofenceStateChangeReport_Vtbl, 0x9a243c18_2464_4c89_be05_b3ffff5babc5);
impl windows_core::RuntimeType for IGeofenceStateChangeReport {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IGeofenceStateChangeReport_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub NewState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut GeofenceState) -> windows_core::HRESULT,
    pub Geofence: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Geoposition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RemovalReason: unsafe extern "system" fn(*mut core::ffi::c_void, *mut GeofenceRemovalReason) -> windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct Geofence(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(Geofence, windows_core::IUnknown, windows_core::IInspectable);
impl Geofence {
    pub fn StartTime(&self) -> windows_core::Result<super::super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).StartTime)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Duration(&self) -> windows_core::Result<super::super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Duration)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn DwellTime(&self) -> windows_core::Result<super::super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DwellTime)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Id(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Id)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn MonitoredStates(&self) -> windows_core::Result<MonitoredGeofenceStates> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MonitoredStates)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Geoshape(&self) -> windows_core::Result<super::IGeoshape> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Geoshape)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SingleUse(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SingleUse)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Create<P0>(id: &windows_core::HSTRING, geoshape: P0) -> windows_core::Result<Geofence>
    where
        P0: windows_core::Param<super::IGeoshape>,
    {
        Self::IGeofenceFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Create)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(id), geoshape.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateWithMonitorStates<P0>(id: &windows_core::HSTRING, geoshape: P0, monitoredstates: MonitoredGeofenceStates, singleuse: bool) -> windows_core::Result<Geofence>
    where
        P0: windows_core::Param<super::IGeoshape>,
    {
        Self::IGeofenceFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateWithMonitorStates)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(id), geoshape.param().abi(), monitoredstates, singleuse, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateWithMonitorStatesAndDwellTime<P0>(id: &windows_core::HSTRING, geoshape: P0, monitoredstates: MonitoredGeofenceStates, singleuse: bool, dwelltime: super::super::super::Foundation::TimeSpan) -> windows_core::Result<Geofence>
    where
        P0: windows_core::Param<super::IGeoshape>,
    {
        Self::IGeofenceFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateWithMonitorStatesAndDwellTime)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(id), geoshape.param().abi(), monitoredstates, singleuse, dwelltime, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateWithMonitorStatesDwellTimeStartTimeAndDuration<P0>(id: &windows_core::HSTRING, geoshape: P0, monitoredstates: MonitoredGeofenceStates, singleuse: bool, dwelltime: super::super::super::Foundation::TimeSpan, starttime: super::super::super::Foundation::DateTime, duration: super::super::super::Foundation::TimeSpan) -> windows_core::Result<Geofence>
    where
        P0: windows_core::Param<super::IGeoshape>,
    {
        Self::IGeofenceFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateWithMonitorStatesDwellTimeStartTimeAndDuration)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(id), geoshape.param().abi(), monitoredstates, singleuse, dwelltime, starttime, duration, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IGeofenceFactory<R, F: FnOnce(&IGeofenceFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<Geofence, IGeofenceFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for Geofence {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IGeofence>();
}
unsafe impl windows_core::Interface for Geofence {
    type Vtable = IGeofence_Vtbl;
    const IID: windows_core::GUID = <IGeofence as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for Geofence {
    const NAME: &'static str = "Windows.Devices.Geolocation.Geofencing.Geofence";
}
unsafe impl Send for Geofence {}
unsafe impl Sync for Geofence {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct GeofenceMonitor(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(GeofenceMonitor, windows_core::IUnknown, windows_core::IInspectable);
impl GeofenceMonitor {
    pub fn Status(&self) -> windows_core::Result<GeofenceMonitorStatus> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Status)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Geofences(&self) -> windows_core::Result<super::super::super::Foundation::Collections::IVector<Geofence>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Geofences)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn LastKnownGeoposition(&self) -> windows_core::Result<super::Geoposition> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LastKnownGeoposition)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GeofenceStateChanged<P0>(&self, eventhandler: P0) -> windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::super::Foundation::TypedEventHandler<GeofenceMonitor, windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GeofenceStateChanged)(windows_core::Interface::as_raw(this), eventhandler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveGeofenceStateChanged(&self, token: super::super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveGeofenceStateChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn ReadReports(&self) -> windows_core::Result<super::super::super::Foundation::Collections::IVectorView<GeofenceStateChangeReport>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReadReports)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn StatusChanged<P0>(&self, eventhandler: P0) -> windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::super::Foundation::TypedEventHandler<GeofenceMonitor, windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).StatusChanged)(windows_core::Interface::as_raw(this), eventhandler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveStatusChanged(&self, token: super::super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveStatusChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn Current() -> windows_core::Result<GeofenceMonitor> {
        Self::IGeofenceMonitorStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Current)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IGeofenceMonitorStatics<R, F: FnOnce(&IGeofenceMonitorStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<GeofenceMonitor, IGeofenceMonitorStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for GeofenceMonitor {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IGeofenceMonitor>();
}
unsafe impl windows_core::Interface for GeofenceMonitor {
    type Vtable = IGeofenceMonitor_Vtbl;
    const IID: windows_core::GUID = <IGeofenceMonitor as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for GeofenceMonitor {
    const NAME: &'static str = "Windows.Devices.Geolocation.Geofencing.GeofenceMonitor";
}
unsafe impl Send for GeofenceMonitor {}
unsafe impl Sync for GeofenceMonitor {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct GeofenceStateChangeReport(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(GeofenceStateChangeReport, windows_core::IUnknown, windows_core::IInspectable);
impl GeofenceStateChangeReport {
    pub fn NewState(&self) -> windows_core::Result<GeofenceState> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NewState)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Geofence(&self) -> windows_core::Result<Geofence> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Geofence)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Geoposition(&self) -> windows_core::Result<super::Geoposition> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Geoposition)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn RemovalReason(&self) -> windows_core::Result<GeofenceRemovalReason> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RemovalReason)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for GeofenceStateChangeReport {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IGeofenceStateChangeReport>();
}
unsafe impl windows_core::Interface for GeofenceStateChangeReport {
    type Vtable = IGeofenceStateChangeReport_Vtbl;
    const IID: windows_core::GUID = <IGeofenceStateChangeReport as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for GeofenceStateChangeReport {
    const NAME: &'static str = "Windows.Devices.Geolocation.Geofencing.GeofenceStateChangeReport";
}
unsafe impl Send for GeofenceStateChangeReport {}
unsafe impl Sync for GeofenceStateChangeReport {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct GeofenceMonitorStatus(pub i32);
impl GeofenceMonitorStatus {
    pub const Ready: Self = Self(0i32);
    pub const Initializing: Self = Self(1i32);
    pub const NoData: Self = Self(2i32);
    pub const Disabled: Self = Self(3i32);
    pub const NotInitialized: Self = Self(4i32);
    pub const NotAvailable: Self = Self(5i32);
}
impl windows_core::TypeKind for GeofenceMonitorStatus {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for GeofenceMonitorStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("GeofenceMonitorStatus").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for GeofenceMonitorStatus {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Geolocation.Geofencing.GeofenceMonitorStatus;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct GeofenceRemovalReason(pub i32);
impl GeofenceRemovalReason {
    pub const Used: Self = Self(0i32);
    pub const Expired: Self = Self(1i32);
}
impl windows_core::TypeKind for GeofenceRemovalReason {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for GeofenceRemovalReason {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("GeofenceRemovalReason").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for GeofenceRemovalReason {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Geolocation.Geofencing.GeofenceRemovalReason;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct GeofenceState(pub u32);
impl GeofenceState {
    pub const None: Self = Self(0u32);
    pub const Entered: Self = Self(1u32);
    pub const Exited: Self = Self(2u32);
    pub const Removed: Self = Self(4u32);
}
impl windows_core::TypeKind for GeofenceState {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for GeofenceState {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("GeofenceState").field(&self.0).finish()
    }
}
impl GeofenceState {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for GeofenceState {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for GeofenceState {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for GeofenceState {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for GeofenceState {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for GeofenceState {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl windows_core::RuntimeType for GeofenceState {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Geolocation.Geofencing.GeofenceState;u4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct MonitoredGeofenceStates(pub u32);
impl MonitoredGeofenceStates {
    pub const None: Self = Self(0u32);
    pub const Entered: Self = Self(1u32);
    pub const Exited: Self = Self(2u32);
    pub const Removed: Self = Self(4u32);
}
impl windows_core::TypeKind for MonitoredGeofenceStates {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for MonitoredGeofenceStates {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MonitoredGeofenceStates").field(&self.0).finish()
    }
}
impl MonitoredGeofenceStates {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for MonitoredGeofenceStates {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for MonitoredGeofenceStates {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for MonitoredGeofenceStates {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for MonitoredGeofenceStates {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for MonitoredGeofenceStates {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl windows_core::RuntimeType for MonitoredGeofenceStates {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Geolocation.Geofencing.MonitoredGeofenceStates;u4)");
}
