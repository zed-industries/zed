windows_core::imp::define_interface!(IPnpObject, IPnpObject_Vtbl, 0x95c66258_733b_4a8f_93a3_db078ac870c1);
impl windows_core::RuntimeType for IPnpObject {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IPnpObject_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Type: unsafe extern "system" fn(*mut core::ffi::c_void, *mut PnpObjectType) -> windows_core::HRESULT,
    pub Id: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Properties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Update: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPnpObjectStatics, IPnpObjectStatics_Vtbl, 0xb3c32a3d_d168_4660_bbf3_a733b14b6e01);
impl windows_core::RuntimeType for IPnpObjectStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IPnpObjectStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateFromIdAsync: unsafe extern "system" fn(*mut core::ffi::c_void, PnpObjectType, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FindAllAsync: unsafe extern "system" fn(*mut core::ffi::c_void, PnpObjectType, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FindAllAsyncAqsFilter: unsafe extern "system" fn(*mut core::ffi::c_void, PnpObjectType, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateWatcher: unsafe extern "system" fn(*mut core::ffi::c_void, PnpObjectType, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateWatcherAqsFilter: unsafe extern "system" fn(*mut core::ffi::c_void, PnpObjectType, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPnpObjectUpdate, IPnpObjectUpdate_Vtbl, 0x6f59e812_001e_4844_bcc6_432886856a17);
impl windows_core::RuntimeType for IPnpObjectUpdate {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IPnpObjectUpdate_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Type: unsafe extern "system" fn(*mut core::ffi::c_void, *mut PnpObjectType) -> windows_core::HRESULT,
    pub Id: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Properties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPnpObjectWatcher, IPnpObjectWatcher_Vtbl, 0x83c95ca8_4772_4a7a_aca8_e48c42a89c44);
impl windows_core::RuntimeType for IPnpObjectWatcher {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IPnpObjectWatcher_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Added: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveAdded: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub Updated: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveUpdated: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub Removed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveRemoved: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub EnumerationCompleted: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveEnumerationCompleted: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub Stopped: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveStopped: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub Status: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::DeviceWatcherStatus) -> windows_core::HRESULT,
    pub Start: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Stop: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PnpObject(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PnpObject, windows_core::IUnknown, windows_core::IInspectable);
impl PnpObject {
    pub fn Type(&self) -> windows_core::Result<PnpObjectType> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Type)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Id(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Id)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn Properties(&self) -> windows_core::Result<windows_collections::IMapView<windows_core::HSTRING, windows_core::IInspectable>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Properties)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Update<P0>(&self, updateinfo: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<PnpObjectUpdate>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Update)(windows_core::Interface::as_raw(this), updateinfo.param().abi()).ok() }
    }
    pub fn CreateFromIdAsync<P2>(r#type: PnpObjectType, id: &windows_core::HSTRING, requestedproperties: P2) -> windows_core::Result<windows_future::IAsyncOperation<PnpObject>>
    where
        P2: windows_core::Param<windows_collections::IIterable<windows_core::HSTRING>>,
    {
        Self::IPnpObjectStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFromIdAsync)(windows_core::Interface::as_raw(this), r#type, core::mem::transmute_copy(id), requestedproperties.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn FindAllAsync<P1>(r#type: PnpObjectType, requestedproperties: P1) -> windows_core::Result<windows_future::IAsyncOperation<PnpObjectCollection>>
    where
        P1: windows_core::Param<windows_collections::IIterable<windows_core::HSTRING>>,
    {
        Self::IPnpObjectStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FindAllAsync)(windows_core::Interface::as_raw(this), r#type, requestedproperties.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn FindAllAsyncAqsFilter<P1>(r#type: PnpObjectType, requestedproperties: P1, aqsfilter: &windows_core::HSTRING) -> windows_core::Result<windows_future::IAsyncOperation<PnpObjectCollection>>
    where
        P1: windows_core::Param<windows_collections::IIterable<windows_core::HSTRING>>,
    {
        Self::IPnpObjectStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FindAllAsyncAqsFilter)(windows_core::Interface::as_raw(this), r#type, requestedproperties.param().abi(), core::mem::transmute_copy(aqsfilter), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateWatcher<P1>(r#type: PnpObjectType, requestedproperties: P1) -> windows_core::Result<PnpObjectWatcher>
    where
        P1: windows_core::Param<windows_collections::IIterable<windows_core::HSTRING>>,
    {
        Self::IPnpObjectStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateWatcher)(windows_core::Interface::as_raw(this), r#type, requestedproperties.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateWatcherAqsFilter<P1>(r#type: PnpObjectType, requestedproperties: P1, aqsfilter: &windows_core::HSTRING) -> windows_core::Result<PnpObjectWatcher>
    where
        P1: windows_core::Param<windows_collections::IIterable<windows_core::HSTRING>>,
    {
        Self::IPnpObjectStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateWatcherAqsFilter)(windows_core::Interface::as_raw(this), r#type, requestedproperties.param().abi(), core::mem::transmute_copy(aqsfilter), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    fn IPnpObjectStatics<R, F: FnOnce(&IPnpObjectStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<PnpObject, IPnpObjectStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for PnpObject {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPnpObject>();
}
unsafe impl windows_core::Interface for PnpObject {
    type Vtable = <IPnpObject as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IPnpObject as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PnpObject {
    const NAME: &'static str = "Windows.Devices.Enumeration.Pnp.PnpObject";
}
unsafe impl Send for PnpObject {}
unsafe impl Sync for PnpObject {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PnpObjectCollection(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PnpObjectCollection, windows_core::IUnknown, windows_core::IInspectable, windows_collections::IVectorView<PnpObject>);
windows_core::imp::required_hierarchy!(PnpObjectCollection, windows_collections::IIterable<PnpObject>);
impl PnpObjectCollection {
    pub fn First(&self) -> windows_core::Result<windows_collections::IIterator<PnpObject>> {
        let this = &windows_core::Interface::cast::<windows_collections::IIterable<PnpObject>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).First)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetAt(&self, index: u32) -> windows_core::Result<PnpObject> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetAt)(windows_core::Interface::as_raw(this), index, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Size(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Size)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IndexOf<P0>(&self, value: P0, index: &mut u32) -> windows_core::Result<bool>
    where
        P0: windows_core::Param<PnpObject>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IndexOf)(windows_core::Interface::as_raw(this), value.param().abi(), index, &mut result__).map(|| result__)
        }
    }
    pub fn GetMany(&self, startindex: u32, items: &mut [Option<PnpObject>]) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetMany)(windows_core::Interface::as_raw(this), startindex, items.len().try_into().unwrap(), core::mem::transmute_copy(&items), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for PnpObjectCollection {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, windows_collections::IVectorView<PnpObject>>();
}
unsafe impl windows_core::Interface for PnpObjectCollection {
    type Vtable = <windows_collections::IVectorView<PnpObject> as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <windows_collections::IVectorView<PnpObject> as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PnpObjectCollection {
    const NAME: &'static str = "Windows.Devices.Enumeration.Pnp.PnpObjectCollection";
}
unsafe impl Send for PnpObjectCollection {}
unsafe impl Sync for PnpObjectCollection {}
impl IntoIterator for PnpObjectCollection {
    type Item = PnpObject;
    type IntoIter = windows_collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        IntoIterator::into_iter(&self)
    }
}
impl IntoIterator for &PnpObjectCollection {
    type Item = PnpObject;
    type IntoIter = windows_collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.First().unwrap()
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PnpObjectType(pub i32);
impl PnpObjectType {
    pub const Unknown: Self = Self(0i32);
    pub const DeviceInterface: Self = Self(1i32);
    pub const DeviceContainer: Self = Self(2i32);
    pub const Device: Self = Self(3i32);
    pub const DeviceInterfaceClass: Self = Self(4i32);
    pub const AssociationEndpoint: Self = Self(5i32);
    pub const AssociationEndpointContainer: Self = Self(6i32);
    pub const AssociationEndpointService: Self = Self(7i32);
    pub const DevicePanel: Self = Self(8i32);
    pub const AssociationEndpointProtocol: Self = Self(9i32);
}
impl windows_core::TypeKind for PnpObjectType {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for PnpObjectType {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Enumeration.Pnp.PnpObjectType;i4)");
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PnpObjectUpdate(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PnpObjectUpdate, windows_core::IUnknown, windows_core::IInspectable);
impl PnpObjectUpdate {
    pub fn Type(&self) -> windows_core::Result<PnpObjectType> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Type)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Id(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Id)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn Properties(&self) -> windows_core::Result<windows_collections::IMapView<windows_core::HSTRING, windows_core::IInspectable>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Properties)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for PnpObjectUpdate {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPnpObjectUpdate>();
}
unsafe impl windows_core::Interface for PnpObjectUpdate {
    type Vtable = <IPnpObjectUpdate as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IPnpObjectUpdate as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PnpObjectUpdate {
    const NAME: &'static str = "Windows.Devices.Enumeration.Pnp.PnpObjectUpdate";
}
unsafe impl Send for PnpObjectUpdate {}
unsafe impl Sync for PnpObjectUpdate {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PnpObjectWatcher(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PnpObjectWatcher, windows_core::IUnknown, windows_core::IInspectable);
impl PnpObjectWatcher {
    pub fn Added<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<super::super::super::Foundation::TypedEventHandler<PnpObjectWatcher, PnpObject>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Added)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveAdded(&self, token: i64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveAdded)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn Updated<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<super::super::super::Foundation::TypedEventHandler<PnpObjectWatcher, PnpObjectUpdate>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Updated)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveUpdated(&self, token: i64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveUpdated)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn Removed<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<super::super::super::Foundation::TypedEventHandler<PnpObjectWatcher, PnpObjectUpdate>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Removed)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveRemoved(&self, token: i64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveRemoved)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn EnumerationCompleted<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<super::super::super::Foundation::TypedEventHandler<PnpObjectWatcher, windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).EnumerationCompleted)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveEnumerationCompleted(&self, token: i64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveEnumerationCompleted)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn Stopped<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<super::super::super::Foundation::TypedEventHandler<PnpObjectWatcher, windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Stopped)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveStopped(&self, token: i64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveStopped)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn Status(&self) -> windows_core::Result<super::DeviceWatcherStatus> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Status)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Start(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Start)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Stop(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Stop)(windows_core::Interface::as_raw(this)).ok() }
    }
}
impl windows_core::RuntimeType for PnpObjectWatcher {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPnpObjectWatcher>();
}
unsafe impl windows_core::Interface for PnpObjectWatcher {
    type Vtable = <IPnpObjectWatcher as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IPnpObjectWatcher as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PnpObjectWatcher {
    const NAME: &'static str = "Windows.Devices.Enumeration.Pnp.PnpObjectWatcher";
}
unsafe impl Send for PnpObjectWatcher {}
unsafe impl Sync for PnpObjectWatcher {}
