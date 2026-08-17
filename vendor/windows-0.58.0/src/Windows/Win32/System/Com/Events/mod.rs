windows_core::imp::define_interface!(IDontSupportEventSubscription, IDontSupportEventSubscription_Vtbl, 0x784121f1_62a6_4b89_855f_d65f296de83a);
impl core::ops::Deref for IDontSupportEventSubscription {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDontSupportEventSubscription, windows_core::IUnknown);
impl IDontSupportEventSubscription {}
#[repr(C)]
pub struct IDontSupportEventSubscription_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
}
windows_core::imp::define_interface!(IEnumEventObject, IEnumEventObject_Vtbl, 0xf4a07d63_2e25_11d1_9964_00c04fbbb345);
impl core::ops::Deref for IEnumEventObject {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IEnumEventObject, windows_core::IUnknown);
impl IEnumEventObject {
    pub unsafe fn Clone(&self) -> windows_core::Result<IEnumEventObject> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Next(&self, ppinterface: &mut [Option<windows_core::IUnknown>], cretelem: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), ppinterface.len().try_into().unwrap(), core::mem::transmute(ppinterface.as_ptr()), cretelem).ok()
    }
    pub unsafe fn Reset(&self) -> windows_core::HRESULT {
        (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn Skip(&self, cskipelem: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), cskipelem).ok()
    }
}
#[repr(C)]
pub struct IEnumEventObject_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IEventClass, IEventClass_Vtbl, 0xfb2b72a0_7a68_11d1_88f9_0080c7d771bf);
impl core::ops::Deref for IEventClass {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IEventClass, windows_core::IUnknown, super::IDispatch);
impl IEventClass {
    pub unsafe fn EventClassID(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EventClassID)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetEventClassID<P0>(&self, bstreventclassid: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetEventClassID)(windows_core::Interface::as_raw(self), bstreventclassid.param().abi()).ok()
    }
    pub unsafe fn EventClassName(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EventClassName)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetEventClassName<P0>(&self, bstreventclassname: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetEventClassName)(windows_core::Interface::as_raw(self), bstreventclassname.param().abi()).ok()
    }
    pub unsafe fn OwnerSID(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).OwnerSID)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetOwnerSID<P0>(&self, bstrownersid: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetOwnerSID)(windows_core::Interface::as_raw(self), bstrownersid.param().abi()).ok()
    }
    pub unsafe fn FiringInterfaceID(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).FiringInterfaceID)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetFiringInterfaceID<P0>(&self, bstrfiringinterfaceid: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetFiringInterfaceID)(windows_core::Interface::as_raw(self), bstrfiringinterfaceid.param().abi()).ok()
    }
    pub unsafe fn Description(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Description)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetDescription<P0>(&self, bstrdescription: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetDescription)(windows_core::Interface::as_raw(self), bstrdescription.param().abi()).ok()
    }
    pub unsafe fn CustomConfigCLSID(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CustomConfigCLSID)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetCustomConfigCLSID<P0>(&self, bstrcustomconfigclsid: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetCustomConfigCLSID)(windows_core::Interface::as_raw(self), bstrcustomconfigclsid.param().abi()).ok()
    }
    pub unsafe fn TypeLib(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).TypeLib)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetTypeLib<P0>(&self, bstrtypelib: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetTypeLib)(windows_core::Interface::as_raw(self), bstrtypelib.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IEventClass_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub EventClassID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetEventClassID: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub EventClassName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetEventClassName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub OwnerSID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetOwnerSID: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub FiringInterfaceID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetFiringInterfaceID: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Description: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub CustomConfigCLSID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetCustomConfigCLSID: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub TypeLib: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetTypeLib: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IEventClass2, IEventClass2_Vtbl, 0xfb2b72a1_7a68_11d1_88f9_0080c7d771bf);
impl core::ops::Deref for IEventClass2 {
    type Target = IEventClass;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IEventClass2, windows_core::IUnknown, super::IDispatch, IEventClass);
impl IEventClass2 {
    pub unsafe fn PublisherID(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).PublisherID)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetPublisherID<P0>(&self, bstrpublisherid: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetPublisherID)(windows_core::Interface::as_raw(self), bstrpublisherid.param().abi()).ok()
    }
    pub unsafe fn MultiInterfacePublisherFilterCLSID(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).MultiInterfacePublisherFilterCLSID)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetMultiInterfacePublisherFilterCLSID<P0>(&self, bstrpubfilclsid: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetMultiInterfacePublisherFilterCLSID)(windows_core::Interface::as_raw(self), bstrpubfilclsid.param().abi()).ok()
    }
    pub unsafe fn AllowInprocActivation(&self) -> windows_core::Result<super::super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).AllowInprocActivation)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetAllowInprocActivation<P0>(&self, fallowinprocactivation: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).SetAllowInprocActivation)(windows_core::Interface::as_raw(self), fallowinprocactivation.param().abi()).ok()
    }
    pub unsafe fn FireInParallel(&self) -> windows_core::Result<super::super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).FireInParallel)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetFireInParallel<P0>(&self, ffireinparallel: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).SetFireInParallel)(windows_core::Interface::as_raw(self), ffireinparallel.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IEventClass2_Vtbl {
    pub base__: IEventClass_Vtbl,
    pub PublisherID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetPublisherID: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub MultiInterfacePublisherFilterCLSID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetMultiInterfacePublisherFilterCLSID: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub AllowInprocActivation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub SetAllowInprocActivation: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub FireInParallel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub SetFireInParallel: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::BOOL) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IEventControl, IEventControl_Vtbl, 0x0343e2f4_86f6_11d1_b760_00c04fb926af);
impl core::ops::Deref for IEventControl {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IEventControl, windows_core::IUnknown, super::IDispatch);
impl IEventControl {
    pub unsafe fn SetPublisherFilter<P0, P1>(&self, methodname: P0, ppublisherfilter: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<IPublisherFilter>,
    {
        (windows_core::Interface::vtable(self).SetPublisherFilter)(windows_core::Interface::as_raw(self), methodname.param().abi(), ppublisherfilter.param().abi()).ok()
    }
    pub unsafe fn AllowInprocActivation(&self) -> windows_core::Result<super::super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).AllowInprocActivation)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetAllowInprocActivation<P0>(&self, fallowinprocactivation: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).SetAllowInprocActivation)(windows_core::Interface::as_raw(self), fallowinprocactivation.param().abi()).ok()
    }
    pub unsafe fn GetSubscriptions<P0, P1>(&self, methodname: P0, optionalcriteria: P1, optionalerrorindex: *const i32) -> windows_core::Result<IEventObjectCollection>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetSubscriptions)(windows_core::Interface::as_raw(self), methodname.param().abi(), optionalcriteria.param().abi(), optionalerrorindex, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetDefaultQuery<P0, P1>(&self, methodname: P0, criteria: P1) -> windows_core::Result<i32>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).SetDefaultQuery)(windows_core::Interface::as_raw(self), methodname.param().abi(), criteria.param().abi(), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IEventControl_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub SetPublisherFilter: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AllowInprocActivation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub SetAllowInprocActivation: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub GetSubscriptions: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, *const i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetDefaultQuery: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, *mut i32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IEventObjectChange, IEventObjectChange_Vtbl, 0xf4a07d70_2e25_11d1_9964_00c04fbbb345);
impl core::ops::Deref for IEventObjectChange {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IEventObjectChange, windows_core::IUnknown);
impl IEventObjectChange {
    pub unsafe fn ChangedSubscription<P0>(&self, changetype: EOC_ChangeType, bstrsubscriptionid: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).ChangedSubscription)(windows_core::Interface::as_raw(self), changetype, bstrsubscriptionid.param().abi()).ok()
    }
    pub unsafe fn ChangedEventClass<P0>(&self, changetype: EOC_ChangeType, bstreventclassid: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).ChangedEventClass)(windows_core::Interface::as_raw(self), changetype, bstreventclassid.param().abi()).ok()
    }
    pub unsafe fn ChangedPublisher<P0>(&self, changetype: EOC_ChangeType, bstrpublisherid: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).ChangedPublisher)(windows_core::Interface::as_raw(self), changetype, bstrpublisherid.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IEventObjectChange_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub ChangedSubscription: unsafe extern "system" fn(*mut core::ffi::c_void, EOC_ChangeType, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub ChangedEventClass: unsafe extern "system" fn(*mut core::ffi::c_void, EOC_ChangeType, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub ChangedPublisher: unsafe extern "system" fn(*mut core::ffi::c_void, EOC_ChangeType, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IEventObjectChange2, IEventObjectChange2_Vtbl, 0x7701a9c3_bd68_438f_83e0_67bf4f53a422);
impl core::ops::Deref for IEventObjectChange2 {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IEventObjectChange2, windows_core::IUnknown);
impl IEventObjectChange2 {
    pub unsafe fn ChangedSubscription(&self, pinfo: *const COMEVENTSYSCHANGEINFO) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ChangedSubscription)(windows_core::Interface::as_raw(self), pinfo).ok()
    }
    pub unsafe fn ChangedEventClass(&self, pinfo: *const COMEVENTSYSCHANGEINFO) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ChangedEventClass)(windows_core::Interface::as_raw(self), pinfo).ok()
    }
}
#[repr(C)]
pub struct IEventObjectChange2_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub ChangedSubscription: unsafe extern "system" fn(*mut core::ffi::c_void, *const COMEVENTSYSCHANGEINFO) -> windows_core::HRESULT,
    pub ChangedEventClass: unsafe extern "system" fn(*mut core::ffi::c_void, *const COMEVENTSYSCHANGEINFO) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IEventObjectCollection, IEventObjectCollection_Vtbl, 0xf89ac270_d4eb_11d1_b682_00805fc79216);
impl core::ops::Deref for IEventObjectCollection {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IEventObjectCollection, windows_core::IUnknown, super::IDispatch);
impl IEventObjectCollection {
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn get_Item<P0>(&self, objectid: P0) -> windows_core::Result<windows_core::VARIANT>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).get_Item)(windows_core::Interface::as_raw(self), objectid.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn NewEnum(&self) -> windows_core::Result<IEnumEventObject> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn Add<P0>(&self, item: *const windows_core::VARIANT, objectid: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).Add)(windows_core::Interface::as_raw(self), core::mem::transmute(item), objectid.param().abi()).ok()
    }
    pub unsafe fn Remove<P0>(&self, objectid: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).Remove)(windows_core::Interface::as_raw(self), objectid.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IEventObjectCollection_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_Item: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub Add: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::mem::MaybeUninit<windows_core::VARIANT>, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Remove: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IEventProperty, IEventProperty_Vtbl, 0xda538ee2_f4de_11d1_b6bb_00805fc79216);
impl core::ops::Deref for IEventProperty {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IEventProperty, windows_core::IUnknown, super::IDispatch);
impl IEventProperty {
    pub unsafe fn Name(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Name)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetName<P0>(&self, propertyname: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetName)(windows_core::Interface::as_raw(self), propertyname.param().abi()).ok()
    }
    pub unsafe fn Value(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Value)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetValue(&self, propertyvalue: *const windows_core::VARIANT) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetValue)(windows_core::Interface::as_raw(self), core::mem::transmute(propertyvalue)).ok()
    }
}
#[repr(C)]
pub struct IEventProperty_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Value: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub SetValue: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IEventPublisher, IEventPublisher_Vtbl, 0xe341516b_2e32_11d1_9964_00c04fbbb345);
impl core::ops::Deref for IEventPublisher {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IEventPublisher, windows_core::IUnknown, super::IDispatch);
impl IEventPublisher {
    pub unsafe fn PublisherID(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).PublisherID)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetPublisherID<P0>(&self, bstrpublisherid: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetPublisherID)(windows_core::Interface::as_raw(self), bstrpublisherid.param().abi()).ok()
    }
    pub unsafe fn PublisherName(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).PublisherName)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetPublisherName<P0>(&self, bstrpublishername: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetPublisherName)(windows_core::Interface::as_raw(self), bstrpublishername.param().abi()).ok()
    }
    pub unsafe fn PublisherType(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).PublisherType)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetPublisherType<P0>(&self, bstrpublishertype: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetPublisherType)(windows_core::Interface::as_raw(self), bstrpublishertype.param().abi()).ok()
    }
    pub unsafe fn OwnerSID(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).OwnerSID)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetOwnerSID<P0>(&self, bstrownersid: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetOwnerSID)(windows_core::Interface::as_raw(self), bstrownersid.param().abi()).ok()
    }
    pub unsafe fn Description(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Description)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetDescription<P0>(&self, bstrdescription: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetDescription)(windows_core::Interface::as_raw(self), bstrdescription.param().abi()).ok()
    }
    pub unsafe fn GetDefaultProperty<P0>(&self, bstrpropertyname: P0) -> windows_core::Result<windows_core::VARIANT>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetDefaultProperty)(windows_core::Interface::as_raw(self), bstrpropertyname.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn PutDefaultProperty<P0>(&self, bstrpropertyname: P0, propertyvalue: *const windows_core::VARIANT) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).PutDefaultProperty)(windows_core::Interface::as_raw(self), bstrpropertyname.param().abi(), core::mem::transmute(propertyvalue)).ok()
    }
    pub unsafe fn RemoveDefaultProperty<P0>(&self, bstrpropertyname: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).RemoveDefaultProperty)(windows_core::Interface::as_raw(self), bstrpropertyname.param().abi()).ok()
    }
    pub unsafe fn GetDefaultPropertyCollection(&self) -> windows_core::Result<IEventObjectCollection> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetDefaultPropertyCollection)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IEventPublisher_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub PublisherID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetPublisherID: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub PublisherName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetPublisherName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub PublisherType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetPublisherType: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub OwnerSID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetOwnerSID: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Description: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub GetDefaultProperty: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub PutDefaultProperty: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *const core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub RemoveDefaultProperty: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub GetDefaultPropertyCollection: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IEventSubscription, IEventSubscription_Vtbl, 0x4a6b0e15_2e38_11d1_9965_00c04fbbb345);
impl core::ops::Deref for IEventSubscription {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IEventSubscription, windows_core::IUnknown, super::IDispatch);
impl IEventSubscription {
    pub unsafe fn SubscriptionID(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).SubscriptionID)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetSubscriptionID<P0>(&self, bstrsubscriptionid: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetSubscriptionID)(windows_core::Interface::as_raw(self), bstrsubscriptionid.param().abi()).ok()
    }
    pub unsafe fn SubscriptionName(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).SubscriptionName)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetSubscriptionName<P0>(&self, bstrsubscriptionname: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetSubscriptionName)(windows_core::Interface::as_raw(self), bstrsubscriptionname.param().abi()).ok()
    }
    pub unsafe fn PublisherID(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).PublisherID)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetPublisherID<P0>(&self, bstrpublisherid: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetPublisherID)(windows_core::Interface::as_raw(self), bstrpublisherid.param().abi()).ok()
    }
    pub unsafe fn EventClassID(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EventClassID)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetEventClassID<P0>(&self, bstreventclassid: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetEventClassID)(windows_core::Interface::as_raw(self), bstreventclassid.param().abi()).ok()
    }
    pub unsafe fn MethodName(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).MethodName)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetMethodName<P0>(&self, bstrmethodname: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetMethodName)(windows_core::Interface::as_raw(self), bstrmethodname.param().abi()).ok()
    }
    pub unsafe fn SubscriberCLSID(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).SubscriberCLSID)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetSubscriberCLSID<P0>(&self, bstrsubscriberclsid: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetSubscriberCLSID)(windows_core::Interface::as_raw(self), bstrsubscriberclsid.param().abi()).ok()
    }
    pub unsafe fn SubscriberInterface(&self) -> windows_core::Result<windows_core::IUnknown> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).SubscriberInterface)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetSubscriberInterface<P0>(&self, psubscriberinterface: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        (windows_core::Interface::vtable(self).SetSubscriberInterface)(windows_core::Interface::as_raw(self), psubscriberinterface.param().abi()).ok()
    }
    pub unsafe fn PerUser(&self) -> windows_core::Result<super::super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).PerUser)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetPerUser<P0>(&self, fperuser: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).SetPerUser)(windows_core::Interface::as_raw(self), fperuser.param().abi()).ok()
    }
    pub unsafe fn OwnerSID(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).OwnerSID)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetOwnerSID<P0>(&self, bstrownersid: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetOwnerSID)(windows_core::Interface::as_raw(self), bstrownersid.param().abi()).ok()
    }
    pub unsafe fn Enabled(&self) -> windows_core::Result<super::super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Enabled)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetEnabled<P0>(&self, fenabled: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).SetEnabled)(windows_core::Interface::as_raw(self), fenabled.param().abi()).ok()
    }
    pub unsafe fn Description(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Description)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetDescription<P0>(&self, bstrdescription: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetDescription)(windows_core::Interface::as_raw(self), bstrdescription.param().abi()).ok()
    }
    pub unsafe fn MachineName(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).MachineName)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetMachineName<P0>(&self, bstrmachinename: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetMachineName)(windows_core::Interface::as_raw(self), bstrmachinename.param().abi()).ok()
    }
    pub unsafe fn GetPublisherProperty<P0>(&self, bstrpropertyname: P0) -> windows_core::Result<windows_core::VARIANT>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetPublisherProperty)(windows_core::Interface::as_raw(self), bstrpropertyname.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn PutPublisherProperty<P0>(&self, bstrpropertyname: P0, propertyvalue: *const windows_core::VARIANT) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).PutPublisherProperty)(windows_core::Interface::as_raw(self), bstrpropertyname.param().abi(), core::mem::transmute(propertyvalue)).ok()
    }
    pub unsafe fn RemovePublisherProperty<P0>(&self, bstrpropertyname: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).RemovePublisherProperty)(windows_core::Interface::as_raw(self), bstrpropertyname.param().abi()).ok()
    }
    pub unsafe fn GetPublisherPropertyCollection(&self) -> windows_core::Result<IEventObjectCollection> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetPublisherPropertyCollection)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetSubscriberProperty<P0>(&self, bstrpropertyname: P0) -> windows_core::Result<windows_core::VARIANT>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetSubscriberProperty)(windows_core::Interface::as_raw(self), bstrpropertyname.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn PutSubscriberProperty<P0>(&self, bstrpropertyname: P0, propertyvalue: *const windows_core::VARIANT) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).PutSubscriberProperty)(windows_core::Interface::as_raw(self), bstrpropertyname.param().abi(), core::mem::transmute(propertyvalue)).ok()
    }
    pub unsafe fn RemoveSubscriberProperty<P0>(&self, bstrpropertyname: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).RemoveSubscriberProperty)(windows_core::Interface::as_raw(self), bstrpropertyname.param().abi()).ok()
    }
    pub unsafe fn GetSubscriberPropertyCollection(&self) -> windows_core::Result<IEventObjectCollection> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetSubscriberPropertyCollection)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn InterfaceID(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).InterfaceID)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetInterfaceID<P0>(&self, bstrinterfaceid: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetInterfaceID)(windows_core::Interface::as_raw(self), bstrinterfaceid.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IEventSubscription_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub SubscriptionID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetSubscriptionID: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SubscriptionName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetSubscriptionName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub PublisherID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetPublisherID: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub EventClassID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetEventClassID: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub MethodName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetMethodName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SubscriberCLSID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetSubscriberCLSID: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SubscriberInterface: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetSubscriberInterface: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PerUser: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub SetPerUser: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub OwnerSID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetOwnerSID: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Enabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub SetEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub Description: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub MachineName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetMachineName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub GetPublisherProperty: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub PutPublisherProperty: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *const core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub RemovePublisherProperty: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub GetPublisherPropertyCollection: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetSubscriberProperty: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub PutSubscriberProperty: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *const core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub RemoveSubscriberProperty: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub GetSubscriberPropertyCollection: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub InterfaceID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetInterfaceID: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IEventSystem, IEventSystem_Vtbl, 0x4e14fb9f_2e22_11d1_9964_00c04fbbb345);
impl core::ops::Deref for IEventSystem {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IEventSystem, windows_core::IUnknown, super::IDispatch);
impl IEventSystem {
    pub unsafe fn Query<P0, P1>(&self, progid: P0, querycriteria: P1, errorindex: *mut i32) -> windows_core::Result<windows_core::IUnknown>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Query)(windows_core::Interface::as_raw(self), progid.param().abi(), querycriteria.param().abi(), errorindex, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Store<P0, P1>(&self, progid: P0, pinterface: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::IUnknown>,
    {
        (windows_core::Interface::vtable(self).Store)(windows_core::Interface::as_raw(self), progid.param().abi(), pinterface.param().abi()).ok()
    }
    pub unsafe fn Remove<P0, P1>(&self, progid: P0, querycriteria: P1) -> windows_core::Result<i32>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Remove)(windows_core::Interface::as_raw(self), progid.param().abi(), querycriteria.param().abi(), &mut result__).map(|| result__)
    }
    pub unsafe fn EventObjectChangeEventClassID(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EventObjectChangeEventClassID)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn QueryS<P0, P1>(&self, progid: P0, querycriteria: P1) -> windows_core::Result<windows_core::IUnknown>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).QueryS)(windows_core::Interface::as_raw(self), progid.param().abi(), querycriteria.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn RemoveS<P0, P1>(&self, progid: P0, querycriteria: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).RemoveS)(windows_core::Interface::as_raw(self), progid.param().abi(), querycriteria.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IEventSystem_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub Query: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, *mut i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Store: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Remove: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, *mut i32) -> windows_core::HRESULT,
    pub EventObjectChangeEventClassID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub QueryS: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RemoveS: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IFiringControl, IFiringControl_Vtbl, 0xe0498c93_4efe_11d1_9971_00c04fbbb345);
impl core::ops::Deref for IFiringControl {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IFiringControl, windows_core::IUnknown, super::IDispatch);
impl IFiringControl {
    pub unsafe fn FireSubscription<P0>(&self, subscription: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IEventSubscription>,
    {
        (windows_core::Interface::vtable(self).FireSubscription)(windows_core::Interface::as_raw(self), subscription.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IFiringControl_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub FireSubscription: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMultiInterfaceEventControl, IMultiInterfaceEventControl_Vtbl, 0x0343e2f5_86f6_11d1_b760_00c04fb926af);
impl core::ops::Deref for IMultiInterfaceEventControl {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IMultiInterfaceEventControl, windows_core::IUnknown);
impl IMultiInterfaceEventControl {
    pub unsafe fn SetMultiInterfacePublisherFilter<P0>(&self, classfilter: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMultiInterfacePublisherFilter>,
    {
        (windows_core::Interface::vtable(self).SetMultiInterfacePublisherFilter)(windows_core::Interface::as_raw(self), classfilter.param().abi()).ok()
    }
    pub unsafe fn GetSubscriptions<P0, P1>(&self, eventiid: *const windows_core::GUID, bstrmethodname: P0, optionalcriteria: P1, optionalerrorindex: *const i32) -> windows_core::Result<IEventObjectCollection>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetSubscriptions)(windows_core::Interface::as_raw(self), eventiid, bstrmethodname.param().abi(), optionalcriteria.param().abi(), optionalerrorindex, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetDefaultQuery<P0, P1>(&self, eventiid: *const windows_core::GUID, bstrmethodname: P0, bstrcriteria: P1) -> windows_core::Result<i32>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).SetDefaultQuery)(windows_core::Interface::as_raw(self), eventiid, bstrmethodname.param().abi(), bstrcriteria.param().abi(), &mut result__).map(|| result__)
    }
    pub unsafe fn AllowInprocActivation(&self) -> windows_core::Result<super::super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).AllowInprocActivation)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetAllowInprocActivation<P0>(&self, fallowinprocactivation: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).SetAllowInprocActivation)(windows_core::Interface::as_raw(self), fallowinprocactivation.param().abi()).ok()
    }
    pub unsafe fn FireInParallel(&self) -> windows_core::Result<super::super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).FireInParallel)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetFireInParallel<P0>(&self, ffireinparallel: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).SetFireInParallel)(windows_core::Interface::as_raw(self), ffireinparallel.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IMultiInterfaceEventControl_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetMultiInterfacePublisherFilter: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetSubscriptions: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, *const i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetDefaultQuery: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, *mut i32) -> windows_core::HRESULT,
    pub AllowInprocActivation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub SetAllowInprocActivation: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub FireInParallel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub SetFireInParallel: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::BOOL) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMultiInterfacePublisherFilter, IMultiInterfacePublisherFilter_Vtbl, 0x465e5cc1_7b26_11d1_88fb_0080c7d771bf);
impl core::ops::Deref for IMultiInterfacePublisherFilter {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IMultiInterfacePublisherFilter, windows_core::IUnknown);
impl IMultiInterfacePublisherFilter {
    pub unsafe fn Initialize<P0>(&self, peic: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMultiInterfaceEventControl>,
    {
        (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self), peic.param().abi()).ok()
    }
    pub unsafe fn PrepareToFire<P0, P1>(&self, iid: *const windows_core::GUID, methodname: P0, firingcontrol: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<IFiringControl>,
    {
        (windows_core::Interface::vtable(self).PrepareToFire)(windows_core::Interface::as_raw(self), iid, methodname.param().abi(), firingcontrol.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IMultiInterfacePublisherFilter_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PrepareToFire: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, core::mem::MaybeUninit<windows_core::BSTR>, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPublisherFilter, IPublisherFilter_Vtbl, 0x465e5cc0_7b26_11d1_88fb_0080c7d771bf);
impl core::ops::Deref for IPublisherFilter {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IPublisherFilter, windows_core::IUnknown);
impl IPublisherFilter {
    pub unsafe fn Initialize<P0, P1>(&self, methodname: P0, dispuserdefined: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<super::IDispatch>,
    {
        (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self), methodname.param().abi(), dispuserdefined.param().abi()).ok()
    }
    pub unsafe fn PrepareToFire<P0, P1>(&self, methodname: P0, firingcontrol: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<IFiringControl>,
    {
        (windows_core::Interface::vtable(self).PrepareToFire)(windows_core::Interface::as_raw(self), methodname.param().abi(), firingcontrol.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IPublisherFilter_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PrepareToFire: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub const EOC_DeletedObject: EOC_ChangeType = EOC_ChangeType(2i32);
pub const EOC_ModifiedObject: EOC_ChangeType = EOC_ChangeType(1i32);
pub const EOC_NewObject: EOC_ChangeType = EOC_ChangeType(0i32);
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct EOC_ChangeType(pub i32);
impl windows_core::TypeKind for EOC_ChangeType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for EOC_ChangeType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("EOC_ChangeType").field(&self.0).finish()
    }
}
pub const CEventClass: windows_core::GUID = windows_core::GUID::from_u128(0xcdbec9c0_7a68_11d1_88f9_0080c7d771bf);
pub const CEventPublisher: windows_core::GUID = windows_core::GUID::from_u128(0xab944620_79c6_11d1_88f9_0080c7d771bf);
pub const CEventSubscription: windows_core::GUID = windows_core::GUID::from_u128(0x7542e960_79c7_11d1_88f9_0080c7d771bf);
pub const CEventSystem: windows_core::GUID = windows_core::GUID::from_u128(0x4e14fba2_2e22_11d1_9964_00c04fbbb345);
#[repr(C)]
#[derive(Debug, Eq, PartialEq)]
pub struct COMEVENTSYSCHANGEINFO {
    pub cbSize: u32,
    pub changeType: EOC_ChangeType,
    pub objectId: core::mem::ManuallyDrop<windows_core::BSTR>,
    pub partitionId: core::mem::ManuallyDrop<windows_core::BSTR>,
    pub applicationId: core::mem::ManuallyDrop<windows_core::BSTR>,
    pub reserved: [windows_core::GUID; 10],
}
impl Clone for COMEVENTSYSCHANGEINFO {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
impl windows_core::TypeKind for COMEVENTSYSCHANGEINFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for COMEVENTSYSCHANGEINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const EventObjectChange: windows_core::GUID = windows_core::GUID::from_u128(0xd0565000_9df4_11d1_a281_00c04fca0aa7);
pub const EventObjectChange2: windows_core::GUID = windows_core::GUID::from_u128(0xbb07bacd_cd56_4e63_a8ff_cbf0355fb9f4);
#[cfg(feature = "implement")]
core::include!("impl.rs");
