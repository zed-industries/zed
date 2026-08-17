windows_core::imp::define_interface!(IEnumVdsObject, IEnumVdsObject_Vtbl, 0x118610b7_8d94_4030_b5b8_500889788e4e);
impl core::ops::Deref for IEnumVdsObject {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IEnumVdsObject, windows_core::IUnknown);
impl IEnumVdsObject {
    pub unsafe fn Next(&self, ppobjectarray: &mut [Option<windows_core::IUnknown>], pcfetched: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), ppobjectarray.len().try_into().unwrap(), core::mem::transmute(ppobjectarray.as_ptr()), pcfetched).ok()
    }
    pub unsafe fn Skip(&self, celt: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), celt).ok()
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<IEnumVdsObject> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IEnumVdsObject_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IVdsAdmin, IVdsAdmin_Vtbl, 0xd188e97d_85aa_4d33_abc6_26299a10ffc1);
impl core::ops::Deref for IVdsAdmin {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IVdsAdmin, windows_core::IUnknown);
impl IVdsAdmin {
    pub unsafe fn RegisterProvider<P0, P1, P2>(&self, providerid: windows_core::GUID, providerclsid: windows_core::GUID, pwszname: P0, r#type: VDS_PROVIDER_TYPE, pwszmachinename: P1, pwszversion: P2, guidversionid: windows_core::GUID) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).RegisterProvider)(windows_core::Interface::as_raw(self), core::mem::transmute(providerid), core::mem::transmute(providerclsid), pwszname.param().abi(), r#type, pwszmachinename.param().abi(), pwszversion.param().abi(), core::mem::transmute(guidversionid)).ok()
    }
    pub unsafe fn UnregisterProvider(&self, providerid: windows_core::GUID) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).UnregisterProvider)(windows_core::Interface::as_raw(self), core::mem::transmute(providerid)).ok()
    }
}
#[repr(C)]
pub struct IVdsAdmin_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub RegisterProvider: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, windows_core::GUID, windows_core::PCWSTR, VDS_PROVIDER_TYPE, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::GUID) -> windows_core::HRESULT,
    pub UnregisterProvider: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IVdsAdvancedDisk, IVdsAdvancedDisk_Vtbl, 0x6e6f6b40_977c_4069_bddd_ac710059f8c0);
impl core::ops::Deref for IVdsAdvancedDisk {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IVdsAdvancedDisk, windows_core::IUnknown);
impl IVdsAdvancedDisk {
    pub unsafe fn GetPartitionProperties(&self, ulloffset: u64, ppartitionprop: *mut VDS_PARTITION_PROP) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetPartitionProperties)(windows_core::Interface::as_raw(self), ulloffset, ppartitionprop).ok()
    }
    pub unsafe fn QueryPartitions(&self, pppartitionproparray: *mut *mut VDS_PARTITION_PROP, plnumberofpartitions: *mut i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).QueryPartitions)(windows_core::Interface::as_raw(self), pppartitionproparray, plnumberofpartitions).ok()
    }
    pub unsafe fn CreatePartition(&self, ulloffset: u64, ullsize: u64, para: *const CREATE_PARTITION_PARAMETERS) -> windows_core::Result<IVdsAsync> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreatePartition)(windows_core::Interface::as_raw(self), ulloffset, ullsize, para, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn DeletePartition<P0, P1>(&self, ulloffset: u64, bforce: P0, bforceprotected: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
        P1: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).DeletePartition)(windows_core::Interface::as_raw(self), ulloffset, bforce.param().abi(), bforceprotected.param().abi()).ok()
    }
    pub unsafe fn ChangeAttributes(&self, ulloffset: u64, para: *const CHANGE_ATTRIBUTES_PARAMETERS) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ChangeAttributes)(windows_core::Interface::as_raw(self), ulloffset, para).ok()
    }
    pub unsafe fn AssignDriveLetter(&self, ulloffset: u64, wcletter: u16) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).AssignDriveLetter)(windows_core::Interface::as_raw(self), ulloffset, wcletter).ok()
    }
    pub unsafe fn DeleteDriveLetter(&self, ulloffset: u64, wcletter: u16) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).DeleteDriveLetter)(windows_core::Interface::as_raw(self), ulloffset, wcletter).ok()
    }
    pub unsafe fn GetDriveLetter(&self, ulloffset: u64, pwcletter: windows_core::PWSTR) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetDriveLetter)(windows_core::Interface::as_raw(self), ulloffset, core::mem::transmute(pwcletter)).ok()
    }
    pub unsafe fn FormatPartition<P0, P1, P2, P3>(&self, ulloffset: u64, r#type: VDS_FILE_SYSTEM_TYPE, pwszlabel: P0, dwunitallocationsize: u32, bforce: P1, bquickformat: P2, benablecompression: P3) -> windows_core::Result<IVdsAsync>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<super::super::Foundation::BOOL>,
        P2: windows_core::Param<super::super::Foundation::BOOL>,
        P3: windows_core::Param<super::super::Foundation::BOOL>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).FormatPartition)(windows_core::Interface::as_raw(self), ulloffset, r#type, pwszlabel.param().abi(), dwunitallocationsize, bforce.param().abi(), bquickformat.param().abi(), benablecompression.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Clean<P0, P1, P2>(&self, bforce: P0, bforceoem: P1, bfullclean: P2) -> windows_core::Result<IVdsAsync>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
        P1: windows_core::Param<super::super::Foundation::BOOL>,
        P2: windows_core::Param<super::super::Foundation::BOOL>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Clean)(windows_core::Interface::as_raw(self), bforce.param().abi(), bforceoem.param().abi(), bfullclean.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IVdsAdvancedDisk_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetPartitionProperties: unsafe extern "system" fn(*mut core::ffi::c_void, u64, *mut VDS_PARTITION_PROP) -> windows_core::HRESULT,
    pub QueryPartitions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut VDS_PARTITION_PROP, *mut i32) -> windows_core::HRESULT,
    pub CreatePartition: unsafe extern "system" fn(*mut core::ffi::c_void, u64, u64, *const CREATE_PARTITION_PARAMETERS, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DeletePartition: unsafe extern "system" fn(*mut core::ffi::c_void, u64, super::super::Foundation::BOOL, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub ChangeAttributes: unsafe extern "system" fn(*mut core::ffi::c_void, u64, *const CHANGE_ATTRIBUTES_PARAMETERS) -> windows_core::HRESULT,
    pub AssignDriveLetter: unsafe extern "system" fn(*mut core::ffi::c_void, u64, u16) -> windows_core::HRESULT,
    pub DeleteDriveLetter: unsafe extern "system" fn(*mut core::ffi::c_void, u64, u16) -> windows_core::HRESULT,
    pub GetDriveLetter: unsafe extern "system" fn(*mut core::ffi::c_void, u64, windows_core::PWSTR) -> windows_core::HRESULT,
    pub FormatPartition: unsafe extern "system" fn(*mut core::ffi::c_void, u64, VDS_FILE_SYSTEM_TYPE, windows_core::PCWSTR, u32, super::super::Foundation::BOOL, super::super::Foundation::BOOL, super::super::Foundation::BOOL, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clean: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::BOOL, super::super::Foundation::BOOL, super::super::Foundation::BOOL, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IVdsAdvancedDisk2, IVdsAdvancedDisk2_Vtbl, 0x9723f420_9355_42de_ab66_e31bb15beeac);
impl core::ops::Deref for IVdsAdvancedDisk2 {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IVdsAdvancedDisk2, windows_core::IUnknown);
impl IVdsAdvancedDisk2 {
    pub unsafe fn ChangePartitionType<P0>(&self, ulloffset: u64, bforce: P0, para: *const CHANGE_PARTITION_TYPE_PARAMETERS) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).ChangePartitionType)(windows_core::Interface::as_raw(self), ulloffset, bforce.param().abi(), para).ok()
    }
}
#[repr(C)]
pub struct IVdsAdvancedDisk2_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub ChangePartitionType: unsafe extern "system" fn(*mut core::ffi::c_void, u64, super::super::Foundation::BOOL, *const CHANGE_PARTITION_TYPE_PARAMETERS) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IVdsAdvancedDisk3, IVdsAdvancedDisk3_Vtbl, 0x3858c0d5_0f35_4bf5_9714_69874963bc36);
impl core::ops::Deref for IVdsAdvancedDisk3 {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IVdsAdvancedDisk3, windows_core::IUnknown);
impl IVdsAdvancedDisk3 {
    pub unsafe fn GetProperties(&self, padvdiskprop: *mut VDS_ADVANCEDDISK_PROP) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetProperties)(windows_core::Interface::as_raw(self), padvdiskprop).ok()
    }
    pub unsafe fn GetUniqueId(&self) -> windows_core::Result<windows_core::PWSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetUniqueId)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IVdsAdvancedDisk3_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut VDS_ADVANCEDDISK_PROP) -> windows_core::HRESULT,
    pub GetUniqueId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IVdsAdviseSink, IVdsAdviseSink_Vtbl, 0x8326cd1d_cf59_4936_b786_5efc08798e25);
impl core::ops::Deref for IVdsAdviseSink {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IVdsAdviseSink, windows_core::IUnknown);
impl IVdsAdviseSink {
    pub unsafe fn OnNotify(&self, pnotificationarray: &[VDS_NOTIFICATION]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OnNotify)(windows_core::Interface::as_raw(self), pnotificationarray.len().try_into().unwrap(), core::mem::transmute(pnotificationarray.as_ptr())).ok()
    }
}
#[repr(C)]
pub struct IVdsAdviseSink_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnNotify: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *const VDS_NOTIFICATION) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IVdsAsync, IVdsAsync_Vtbl, 0xd5d23b6d_5a55_4492_9889_397a3c2d2dbc);
impl core::ops::Deref for IVdsAsync {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IVdsAsync, windows_core::IUnknown);
impl IVdsAsync {
    pub unsafe fn Cancel(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Cancel)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Wait(&self, phrresult: *mut windows_core::HRESULT, pasyncout: *mut VDS_ASYNC_OUTPUT) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Wait)(windows_core::Interface::as_raw(self), phrresult, pasyncout).ok()
    }
    pub unsafe fn QueryStatus(&self, phrresult: *mut windows_core::HRESULT, pulpercentcompleted: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).QueryStatus)(windows_core::Interface::as_raw(self), phrresult, pulpercentcompleted).ok()
    }
}
#[repr(C)]
pub struct IVdsAsync_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Cancel: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Wait: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::HRESULT, *mut VDS_ASYNC_OUTPUT) -> windows_core::HRESULT,
    pub QueryStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::HRESULT, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IVdsController, IVdsController_Vtbl, 0xcb53d96e_dffb_474a_a078_790d1e2bc082);
impl core::ops::Deref for IVdsController {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IVdsController, windows_core::IUnknown);
impl IVdsController {
    pub unsafe fn GetProperties(&self, pcontrollerprop: *mut VDS_CONTROLLER_PROP) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetProperties)(windows_core::Interface::as_raw(self), pcontrollerprop).ok()
    }
    pub unsafe fn GetSubSystem(&self) -> windows_core::Result<IVdsSubSystem> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetSubSystem)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetPortProperties(&self, sportnumber: i16, pportprop: *mut VDS_PORT_PROP) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetPortProperties)(windows_core::Interface::as_raw(self), sportnumber, pportprop).ok()
    }
    pub unsafe fn FlushCache(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).FlushCache)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn InvalidateCache(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).InvalidateCache)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn QueryAssociatedLuns(&self) -> windows_core::Result<IEnumVdsObject> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).QueryAssociatedLuns)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetStatus(&self, status: VDS_CONTROLLER_STATUS) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetStatus)(windows_core::Interface::as_raw(self), status).ok()
    }
}
#[repr(C)]
pub struct IVdsController_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut VDS_CONTROLLER_PROP) -> windows_core::HRESULT,
    pub GetSubSystem: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetPortProperties: unsafe extern "system" fn(*mut core::ffi::c_void, i16, *mut VDS_PORT_PROP) -> windows_core::HRESULT,
    pub FlushCache: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub InvalidateCache: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub QueryAssociatedLuns: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetStatus: unsafe extern "system" fn(*mut core::ffi::c_void, VDS_CONTROLLER_STATUS) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IVdsControllerControllerPort, IVdsControllerControllerPort_Vtbl, 0xca5d735f_6bae_42c0_b30e_f2666045ce71);
impl core::ops::Deref for IVdsControllerControllerPort {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IVdsControllerControllerPort, windows_core::IUnknown);
impl IVdsControllerControllerPort {
    pub unsafe fn QueryControllerPorts(&self) -> windows_core::Result<IEnumVdsObject> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).QueryControllerPorts)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IVdsControllerControllerPort_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub QueryControllerPorts: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IVdsControllerPort, IVdsControllerPort_Vtbl, 0x18691d0d_4e7f_43e8_92e4_cf44beeed11c);
impl core::ops::Deref for IVdsControllerPort {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IVdsControllerPort, windows_core::IUnknown);
impl IVdsControllerPort {
    pub unsafe fn GetProperties(&self, pportprop: *mut VDS_PORT_PROP) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetProperties)(windows_core::Interface::as_raw(self), pportprop).ok()
    }
    pub unsafe fn GetController(&self) -> windows_core::Result<IVdsController> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetController)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn QueryAssociatedLuns(&self) -> windows_core::Result<IEnumVdsObject> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).QueryAssociatedLuns)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SetStatus(&self, status: VDS_PORT_STATUS) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetStatus)(windows_core::Interface::as_raw(self), status).ok()
    }
}
#[repr(C)]
pub struct IVdsControllerPort_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut VDS_PORT_PROP) -> windows_core::HRESULT,
    pub GetController: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub QueryAssociatedLuns: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetStatus: unsafe extern "system" fn(*mut core::ffi::c_void, VDS_PORT_STATUS) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IVdsCreatePartitionEx, IVdsCreatePartitionEx_Vtbl, 0x9882f547_cfc3_420b_9750_00dfbec50662);
impl core::ops::Deref for IVdsCreatePartitionEx {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IVdsCreatePartitionEx, windows_core::IUnknown);
impl IVdsCreatePartitionEx {
    pub unsafe fn CreatePartitionEx(&self, ulloffset: u64, ullsize: u64, ulalign: u32, para: *const CREATE_PARTITION_PARAMETERS) -> windows_core::Result<IVdsAsync> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreatePartitionEx)(windows_core::Interface::as_raw(self), ulloffset, ullsize, ulalign, para, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IVdsCreatePartitionEx_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub CreatePartitionEx: unsafe extern "system" fn(*mut core::ffi::c_void, u64, u64, u32, *const CREATE_PARTITION_PARAMETERS, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IVdsDisk, IVdsDisk_Vtbl, 0x07e5c822_f00c_47a1_8fce_b244da56fd06);
impl core::ops::Deref for IVdsDisk {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IVdsDisk, windows_core::IUnknown);
impl IVdsDisk {
    pub unsafe fn GetProperties(&self, pdiskproperties: *mut VDS_DISK_PROP) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetProperties)(windows_core::Interface::as_raw(self), pdiskproperties).ok()
    }
    pub unsafe fn GetPack(&self) -> windows_core::Result<IVdsPack> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetPack)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetIdentificationData(&self, pluninfo: *mut VDS_LUN_INFORMATION) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetIdentificationData)(windows_core::Interface::as_raw(self), pluninfo).ok()
    }
    pub unsafe fn QueryExtents(&self, ppextentarray: *mut *mut VDS_DISK_EXTENT, plnumberofextents: *mut i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).QueryExtents)(windows_core::Interface::as_raw(self), ppextentarray, plnumberofextents).ok()
    }
    pub unsafe fn ConvertStyle(&self, newstyle: VDS_PARTITION_STYLE) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ConvertStyle)(windows_core::Interface::as_raw(self), newstyle).ok()
    }
    pub unsafe fn SetFlags(&self, ulflags: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetFlags)(windows_core::Interface::as_raw(self), ulflags).ok()
    }
    pub unsafe fn ClearFlags(&self, ulflags: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ClearFlags)(windows_core::Interface::as_raw(self), ulflags).ok()
    }
}
#[repr(C)]
pub struct IVdsDisk_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut VDS_DISK_PROP) -> windows_core::HRESULT,
    pub GetPack: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetIdentificationData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut VDS_LUN_INFORMATION) -> windows_core::HRESULT,
    pub QueryExtents: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut VDS_DISK_EXTENT, *mut i32) -> windows_core::HRESULT,
    pub ConvertStyle: unsafe extern "system" fn(*mut core::ffi::c_void, VDS_PARTITION_STYLE) -> windows_core::HRESULT,
    pub SetFlags: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub ClearFlags: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IVdsDisk2, IVdsDisk2_Vtbl, 0x40f73c8b_687d_4a13_8d96_3d7f2e683936);
impl core::ops::Deref for IVdsDisk2 {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IVdsDisk2, windows_core::IUnknown);
impl IVdsDisk2 {
    pub unsafe fn SetSANMode<P0>(&self, benable: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).SetSANMode)(windows_core::Interface::as_raw(self), benable.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IVdsDisk2_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetSANMode: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::BOOL) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IVdsDisk3, IVdsDisk3_Vtbl, 0x8f4b2f5d_ec15_4357_992f_473ef10975b9);
impl core::ops::Deref for IVdsDisk3 {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IVdsDisk3, windows_core::IUnknown);
impl IVdsDisk3 {
    pub unsafe fn GetProperties2(&self, pdiskproperties: *mut VDS_DISK_PROP2) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetProperties2)(windows_core::Interface::as_raw(self), pdiskproperties).ok()
    }
    pub unsafe fn QueryFreeExtents(&self, ulalign: u32, ppfreeextentarray: *mut *mut VDS_DISK_FREE_EXTENT, plnumberoffreeextents: *mut i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).QueryFreeExtents)(windows_core::Interface::as_raw(self), ulalign, ppfreeextentarray, plnumberoffreeextents).ok()
    }
}
#[repr(C)]
pub struct IVdsDisk3_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetProperties2: unsafe extern "system" fn(*mut core::ffi::c_void, *mut VDS_DISK_PROP2) -> windows_core::HRESULT,
    pub QueryFreeExtents: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut VDS_DISK_FREE_EXTENT, *mut i32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IVdsDiskOnline, IVdsDiskOnline_Vtbl, 0x90681b1d_6a7f_48e8_9061_31b7aa125322);
impl core::ops::Deref for IVdsDiskOnline {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IVdsDiskOnline, windows_core::IUnknown);
impl IVdsDiskOnline {
    pub unsafe fn Online(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Online)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Offline(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Offline)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct IVdsDiskOnline_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Online: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Offline: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IVdsDiskPartitionMF, IVdsDiskPartitionMF_Vtbl, 0x538684e0_ba3d_4bc0_aca9_164aff85c2a9);
impl core::ops::Deref for IVdsDiskPartitionMF {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IVdsDiskPartitionMF, windows_core::IUnknown);
impl IVdsDiskPartitionMF {
    pub unsafe fn GetPartitionFileSystemProperties(&self, ulloffset: u64, pfilesystemprop: *mut VDS_FILE_SYSTEM_PROP) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetPartitionFileSystemProperties)(windows_core::Interface::as_raw(self), ulloffset, pfilesystemprop).ok()
    }
    pub unsafe fn GetPartitionFileSystemTypeName(&self, ulloffset: u64) -> windows_core::Result<windows_core::PWSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetPartitionFileSystemTypeName)(windows_core::Interface::as_raw(self), ulloffset, &mut result__).map(|| result__)
    }
    pub unsafe fn QueryPartitionFileSystemFormatSupport(&self, ulloffset: u64, ppfilesystemsupportprops: *mut *mut VDS_FILE_SYSTEM_FORMAT_SUPPORT_PROP, plnumberoffilesystems: *mut i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).QueryPartitionFileSystemFormatSupport)(windows_core::Interface::as_raw(self), ulloffset, ppfilesystemsupportprops, plnumberoffilesystems).ok()
    }
    pub unsafe fn FormatPartitionEx<P0, P1, P2, P3, P4>(&self, ulloffset: u64, pwszfilesystemtypename: P0, usfilesystemrevision: u16, uldesiredunitallocationsize: u32, pwszlabel: P1, bforce: P2, bquickformat: P3, benablecompression: P4) -> windows_core::Result<IVdsAsync>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<super::super::Foundation::BOOL>,
        P3: windows_core::Param<super::super::Foundation::BOOL>,
        P4: windows_core::Param<super::super::Foundation::BOOL>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).FormatPartitionEx)(windows_core::Interface::as_raw(self), ulloffset, pwszfilesystemtypename.param().abi(), usfilesystemrevision, uldesiredunitallocationsize, pwszlabel.param().abi(), bforce.param().abi(), bquickformat.param().abi(), benablecompression.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IVdsDiskPartitionMF_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetPartitionFileSystemProperties: unsafe extern "system" fn(*mut core::ffi::c_void, u64, *mut VDS_FILE_SYSTEM_PROP) -> windows_core::HRESULT,
    pub GetPartitionFileSystemTypeName: unsafe extern "system" fn(*mut core::ffi::c_void, u64, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub QueryPartitionFileSystemFormatSupport: unsafe extern "system" fn(*mut core::ffi::c_void, u64, *mut *mut VDS_FILE_SYSTEM_FORMAT_SUPPORT_PROP, *mut i32) -> windows_core::HRESULT,
    pub FormatPartitionEx: unsafe extern "system" fn(*mut core::ffi::c_void, u64, windows_core::PCWSTR, u16, u32, windows_core::PCWSTR, super::super::Foundation::BOOL, super::super::Foundation::BOOL, super::super::Foundation::BOOL, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IVdsDiskPartitionMF2, IVdsDiskPartitionMF2_Vtbl, 0x9cbe50ca_f2d2_4bf4_ace1_96896b729625);
impl core::ops::Deref for IVdsDiskPartitionMF2 {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IVdsDiskPartitionMF2, windows_core::IUnknown);
impl IVdsDiskPartitionMF2 {
    pub unsafe fn FormatPartitionEx2<P0, P1>(&self, ulloffset: u64, pwszfilesystemtypename: P0, usfilesystemrevision: u16, uldesiredunitallocationsize: u32, pwszlabel: P1, options: u32) -> windows_core::Result<IVdsAsync>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).FormatPartitionEx2)(windows_core::Interface::as_raw(self), ulloffset, pwszfilesystemtypename.param().abi(), usfilesystemrevision, uldesiredunitallocationsize, pwszlabel.param().abi(), options, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IVdsDiskPartitionMF2_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub FormatPartitionEx2: unsafe extern "system" fn(*mut core::ffi::c_void, u64, windows_core::PCWSTR, u16, u32, windows_core::PCWSTR, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IVdsDrive, IVdsDrive_Vtbl, 0xff24efa4_aade_4b6b_898b_eaa6a20887c7);
impl core::ops::Deref for IVdsDrive {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IVdsDrive, windows_core::IUnknown);
impl IVdsDrive {
    pub unsafe fn GetProperties(&self, pdriveprop: *mut VDS_DRIVE_PROP) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetProperties)(windows_core::Interface::as_raw(self), pdriveprop).ok()
    }
    pub unsafe fn GetSubSystem(&self) -> windows_core::Result<IVdsSubSystem> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetSubSystem)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn QueryExtents(&self, ppextentarray: *mut *mut VDS_DRIVE_EXTENT, plnumberofextents: *mut i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).QueryExtents)(windows_core::Interface::as_raw(self), ppextentarray, plnumberofextents).ok()
    }
    pub unsafe fn SetFlags(&self, ulflags: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetFlags)(windows_core::Interface::as_raw(self), ulflags).ok()
    }
    pub unsafe fn ClearFlags(&self, ulflags: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ClearFlags)(windows_core::Interface::as_raw(self), ulflags).ok()
    }
    pub unsafe fn SetStatus(&self, status: VDS_DRIVE_STATUS) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetStatus)(windows_core::Interface::as_raw(self), status).ok()
    }
}
#[repr(C)]
pub struct IVdsDrive_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut VDS_DRIVE_PROP) -> windows_core::HRESULT,
    pub GetSubSystem: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub QueryExtents: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut VDS_DRIVE_EXTENT, *mut i32) -> windows_core::HRESULT,
    pub SetFlags: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub ClearFlags: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub SetStatus: unsafe extern "system" fn(*mut core::ffi::c_void, VDS_DRIVE_STATUS) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IVdsDrive2, IVdsDrive2_Vtbl, 0x60b5a730_addf_4436_8ca7_5769e2d1ffa4);
impl core::ops::Deref for IVdsDrive2 {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IVdsDrive2, windows_core::IUnknown);
impl IVdsDrive2 {
    pub unsafe fn GetProperties2(&self, pdriveprop2: *mut VDS_DRIVE_PROP2) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetProperties2)(windows_core::Interface::as_raw(self), pdriveprop2).ok()
    }
}
#[repr(C)]
pub struct IVdsDrive2_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetProperties2: unsafe extern "system" fn(*mut core::ffi::c_void, *mut VDS_DRIVE_PROP2) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IVdsHbaPort, IVdsHbaPort_Vtbl, 0x2abd757f_2851_4997_9a13_47d2a885d6ca);
impl core::ops::Deref for IVdsHbaPort {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IVdsHbaPort, windows_core::IUnknown);
impl IVdsHbaPort {
    pub unsafe fn GetProperties(&self, phbaportprop: *mut VDS_HBAPORT_PROP) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetProperties)(windows_core::Interface::as_raw(self), phbaportprop).ok()
    }
    pub unsafe fn SetAllPathStatuses(&self, status: VDS_PATH_STATUS) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetAllPathStatuses)(windows_core::Interface::as_raw(self), status).ok()
    }
}
#[repr(C)]
pub struct IVdsHbaPort_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut VDS_HBAPORT_PROP) -> windows_core::HRESULT,
    pub SetAllPathStatuses: unsafe extern "system" fn(*mut core::ffi::c_void, VDS_PATH_STATUS) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IVdsHwProvider, IVdsHwProvider_Vtbl, 0xd99bdaae_b13a_4178_9fdb_e27f16b4603e);
impl core::ops::Deref for IVdsHwProvider {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IVdsHwProvider, windows_core::IUnknown);
impl IVdsHwProvider {
    pub unsafe fn QuerySubSystems(&self) -> windows_core::Result<IEnumVdsObject> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).QuerySubSystems)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Reenumerate(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Reenumerate)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Refresh(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Refresh)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct IVdsHwProvider_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub QuerySubSystems: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Reenumerate: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Refresh: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IVdsHwProviderPrivate, IVdsHwProviderPrivate_Vtbl, 0x98f17bf3_9f33_4f12_8714_8b4075092c2e);
impl core::ops::Deref for IVdsHwProviderPrivate {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IVdsHwProviderPrivate, windows_core::IUnknown);
impl IVdsHwProviderPrivate {
    pub unsafe fn QueryIfCreatedLun<P0>(&self, pwszdevicepath: P0, pvdsluninformation: *const VDS_LUN_INFORMATION) -> windows_core::Result<windows_core::GUID>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).QueryIfCreatedLun)(windows_core::Interface::as_raw(self), pwszdevicepath.param().abi(), pvdsluninformation, &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IVdsHwProviderPrivate_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub QueryIfCreatedLun: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *const VDS_LUN_INFORMATION, *mut windows_core::GUID) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IVdsHwProviderPrivateMpio, IVdsHwProviderPrivateMpio_Vtbl, 0x310a7715_ac2b_4c6f_9827_3d742f351676);
impl core::ops::Deref for IVdsHwProviderPrivateMpio {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IVdsHwProviderPrivateMpio, windows_core::IUnknown);
impl IVdsHwProviderPrivateMpio {
    pub unsafe fn SetAllPathStatusesFromHbaPort(&self, hbaportprop: VDS_HBAPORT_PROP, status: VDS_PATH_STATUS) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetAllPathStatusesFromHbaPort)(windows_core::Interface::as_raw(self), core::mem::transmute(hbaportprop), status).ok()
    }
}
#[repr(C)]
pub struct IVdsHwProviderPrivateMpio_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetAllPathStatusesFromHbaPort: unsafe extern "system" fn(*mut core::ffi::c_void, VDS_HBAPORT_PROP, VDS_PATH_STATUS) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IVdsHwProviderStoragePools, IVdsHwProviderStoragePools_Vtbl, 0xd5b5937a_f188_4c79_b86c_11c920ad11b8);
impl core::ops::Deref for IVdsHwProviderStoragePools {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IVdsHwProviderStoragePools, windows_core::IUnknown);
impl IVdsHwProviderStoragePools {
    pub unsafe fn QueryStoragePools(&self, ulflags: u32, ullremainingfreespace: u64, ppoolattributes: *const VDS_POOL_ATTRIBUTES) -> windows_core::Result<IEnumVdsObject> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).QueryStoragePools)(windows_core::Interface::as_raw(self), ulflags, ullremainingfreespace, ppoolattributes, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateLunInStoragePool<P0>(&self, r#type: VDS_LUN_TYPE, ullsizeinbytes: u64, storagepoolid: windows_core::GUID, pwszunmaskinglist: P0, phints2: *const VDS_HINTS2) -> windows_core::Result<IVdsAsync>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateLunInStoragePool)(windows_core::Interface::as_raw(self), r#type, ullsizeinbytes, core::mem::transmute(storagepoolid), pwszunmaskinglist.param().abi(), phints2, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn QueryMaxLunCreateSizeInStoragePool(&self, r#type: VDS_LUN_TYPE, storagepoolid: windows_core::GUID, phints2: *const VDS_HINTS2) -> windows_core::Result<u64> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).QueryMaxLunCreateSizeInStoragePool)(windows_core::Interface::as_raw(self), r#type, core::mem::transmute(storagepoolid), phints2, &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IVdsHwProviderStoragePools_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub QueryStoragePools: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u64, *const VDS_POOL_ATTRIBUTES, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateLunInStoragePool: unsafe extern "system" fn(*mut core::ffi::c_void, VDS_LUN_TYPE, u64, windows_core::GUID, windows_core::PCWSTR, *const VDS_HINTS2, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub QueryMaxLunCreateSizeInStoragePool: unsafe extern "system" fn(*mut core::ffi::c_void, VDS_LUN_TYPE, windows_core::GUID, *const VDS_HINTS2, *mut u64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IVdsHwProviderType, IVdsHwProviderType_Vtbl, 0x3e0f5166_542d_4fc6_947a_012174240b7e);
impl core::ops::Deref for IVdsHwProviderType {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IVdsHwProviderType, windows_core::IUnknown);
impl IVdsHwProviderType {
    pub unsafe fn GetProviderType(&self) -> windows_core::Result<VDS_HWPROVIDER_TYPE> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetProviderType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IVdsHwProviderType_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetProviderType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut VDS_HWPROVIDER_TYPE) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IVdsHwProviderType2, IVdsHwProviderType2_Vtbl, 0x8190236f_c4d0_4e81_8011_d69512fcc984);
impl core::ops::Deref for IVdsHwProviderType2 {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IVdsHwProviderType2, windows_core::IUnknown);
impl IVdsHwProviderType2 {
    pub unsafe fn GetProviderType2(&self) -> windows_core::Result<VDS_HWPROVIDER_TYPE> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetProviderType2)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IVdsHwProviderType2_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetProviderType2: unsafe extern "system" fn(*mut core::ffi::c_void, *mut VDS_HWPROVIDER_TYPE) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IVdsIscsiInitiatorAdapter, IVdsIscsiInitiatorAdapter_Vtbl, 0xb07fedd4_1682_4440_9189_a39b55194dc5);
impl core::ops::Deref for IVdsIscsiInitiatorAdapter {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IVdsIscsiInitiatorAdapter, windows_core::IUnknown);
impl IVdsIscsiInitiatorAdapter {
    pub unsafe fn GetProperties(&self, pinitiatoradapterprop: *mut VDS_ISCSI_INITIATOR_ADAPTER_PROP) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetProperties)(windows_core::Interface::as_raw(self), pinitiatoradapterprop).ok()
    }
    pub unsafe fn QueryInitiatorPortals(&self) -> windows_core::Result<IEnumVdsObject> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).QueryInitiatorPortals)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn LoginToTarget<P0, P1>(&self, logintype: VDS_ISCSI_LOGIN_TYPE, targetid: windows_core::GUID, targetportalid: windows_core::GUID, initiatorportalid: windows_core::GUID, ulloginflags: u32, bheaderdigest: P0, bdatadigest: P1, authtype: VDS_ISCSI_AUTH_TYPE) -> windows_core::Result<IVdsAsync>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
        P1: windows_core::Param<super::super::Foundation::BOOL>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).LoginToTarget)(windows_core::Interface::as_raw(self), logintype, core::mem::transmute(targetid), core::mem::transmute(targetportalid), core::mem::transmute(initiatorportalid), ulloginflags, bheaderdigest.param().abi(), bdatadigest.param().abi(), authtype, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn LogoutFromTarget(&self, targetid: windows_core::GUID) -> windows_core::Result<IVdsAsync> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).LogoutFromTarget)(windows_core::Interface::as_raw(self), core::mem::transmute(targetid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IVdsIscsiInitiatorAdapter_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut VDS_ISCSI_INITIATOR_ADAPTER_PROP) -> windows_core::HRESULT,
    pub QueryInitiatorPortals: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub LoginToTarget: unsafe extern "system" fn(*mut core::ffi::c_void, VDS_ISCSI_LOGIN_TYPE, windows_core::GUID, windows_core::GUID, windows_core::GUID, u32, super::super::Foundation::BOOL, super::super::Foundation::BOOL, VDS_ISCSI_AUTH_TYPE, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub LogoutFromTarget: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IVdsIscsiInitiatorPortal, IVdsIscsiInitiatorPortal_Vtbl, 0x38a0a9ab_7cc8_4693_ac07_1f28bd03c3da);
impl core::ops::Deref for IVdsIscsiInitiatorPortal {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IVdsIscsiInitiatorPortal, windows_core::IUnknown);
impl IVdsIscsiInitiatorPortal {
    pub unsafe fn GetProperties(&self, pinitiatorportalprop: *mut VDS_ISCSI_INITIATOR_PORTAL_PROP) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetProperties)(windows_core::Interface::as_raw(self), pinitiatorportalprop).ok()
    }
    pub unsafe fn GetInitiatorAdapter(&self) -> windows_core::Result<IVdsIscsiInitiatorAdapter> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetInitiatorAdapter)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetIpsecTunnelAddress(&self, ptunneladdress: *const VDS_IPADDRESS, pdestinationaddress: *const VDS_IPADDRESS) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetIpsecTunnelAddress)(windows_core::Interface::as_raw(self), ptunneladdress, pdestinationaddress).ok()
    }
    pub unsafe fn GetIpsecSecurity(&self, targetportalid: windows_core::GUID) -> windows_core::Result<u64> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetIpsecSecurity)(windows_core::Interface::as_raw(self), core::mem::transmute(targetportalid), &mut result__).map(|| result__)
    }
    pub unsafe fn SetIpsecSecurity(&self, targetportalid: windows_core::GUID, ullsecurityflags: u64, pipseckey: *const VDS_ISCSI_IPSEC_KEY) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetIpsecSecurity)(windows_core::Interface::as_raw(self), core::mem::transmute(targetportalid), ullsecurityflags, pipseckey).ok()
    }
}
#[repr(C)]
pub struct IVdsIscsiInitiatorPortal_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut VDS_ISCSI_INITIATOR_PORTAL_PROP) -> windows_core::HRESULT,
    pub GetInitiatorAdapter: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetIpsecTunnelAddress: unsafe extern "system" fn(*mut core::ffi::c_void, *const VDS_IPADDRESS, *const VDS_IPADDRESS) -> windows_core::HRESULT,
    pub GetIpsecSecurity: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, *mut u64) -> windows_core::HRESULT,
    pub SetIpsecSecurity: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, u64, *const VDS_ISCSI_IPSEC_KEY) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IVdsIscsiPortal, IVdsIscsiPortal_Vtbl, 0x7fa1499d_ec85_4a8a_a47b_ff69201fcd34);
impl core::ops::Deref for IVdsIscsiPortal {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IVdsIscsiPortal, windows_core::IUnknown);
impl IVdsIscsiPortal {
    pub unsafe fn GetProperties(&self, pportalprop: *mut VDS_ISCSI_PORTAL_PROP) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetProperties)(windows_core::Interface::as_raw(self), pportalprop).ok()
    }
    pub unsafe fn GetSubSystem(&self) -> windows_core::Result<IVdsSubSystem> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetSubSystem)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn QueryAssociatedPortalGroups(&self) -> windows_core::Result<IEnumVdsObject> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).QueryAssociatedPortalGroups)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetStatus(&self, status: VDS_ISCSI_PORTAL_STATUS) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetStatus)(windows_core::Interface::as_raw(self), status).ok()
    }
    pub unsafe fn SetIpsecTunnelAddress(&self, ptunneladdress: *const VDS_IPADDRESS, pdestinationaddress: *const VDS_IPADDRESS) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetIpsecTunnelAddress)(windows_core::Interface::as_raw(self), ptunneladdress, pdestinationaddress).ok()
    }
    pub unsafe fn GetIpsecSecurity(&self, pinitiatorportaladdress: *const VDS_IPADDRESS) -> windows_core::Result<u64> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetIpsecSecurity)(windows_core::Interface::as_raw(self), pinitiatorportaladdress, &mut result__).map(|| result__)
    }
    pub unsafe fn SetIpsecSecurity(&self, pinitiatorportaladdress: *const VDS_IPADDRESS, ullsecurityflags: u64, pipseckey: *const VDS_ISCSI_IPSEC_KEY) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetIpsecSecurity)(windows_core::Interface::as_raw(self), pinitiatorportaladdress, ullsecurityflags, pipseckey).ok()
    }
}
#[repr(C)]
pub struct IVdsIscsiPortal_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut VDS_ISCSI_PORTAL_PROP) -> windows_core::HRESULT,
    pub GetSubSystem: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub QueryAssociatedPortalGroups: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetStatus: unsafe extern "system" fn(*mut core::ffi::c_void, VDS_ISCSI_PORTAL_STATUS) -> windows_core::HRESULT,
    pub SetIpsecTunnelAddress: unsafe extern "system" fn(*mut core::ffi::c_void, *const VDS_IPADDRESS, *const VDS_IPADDRESS) -> windows_core::HRESULT,
    pub GetIpsecSecurity: unsafe extern "system" fn(*mut core::ffi::c_void, *const VDS_IPADDRESS, *mut u64) -> windows_core::HRESULT,
    pub SetIpsecSecurity: unsafe extern "system" fn(*mut core::ffi::c_void, *const VDS_IPADDRESS, u64, *const VDS_ISCSI_IPSEC_KEY) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IVdsIscsiPortalGroup, IVdsIscsiPortalGroup_Vtbl, 0xfef5f89d_a3dd_4b36_bf28_e7dde045c593);
impl core::ops::Deref for IVdsIscsiPortalGroup {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IVdsIscsiPortalGroup, windows_core::IUnknown);
impl IVdsIscsiPortalGroup {
    pub unsafe fn GetProperties(&self, pportalgroupprop: *mut VDS_ISCSI_PORTALGROUP_PROP) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetProperties)(windows_core::Interface::as_raw(self), pportalgroupprop).ok()
    }
    pub unsafe fn GetTarget(&self) -> windows_core::Result<IVdsIscsiTarget> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetTarget)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn QueryAssociatedPortals(&self) -> windows_core::Result<IEnumVdsObject> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).QueryAssociatedPortals)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn AddPortal(&self, portalid: windows_core::GUID) -> windows_core::Result<IVdsAsync> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).AddPortal)(windows_core::Interface::as_raw(self), core::mem::transmute(portalid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn RemovePortal(&self, portalid: windows_core::GUID) -> windows_core::Result<IVdsAsync> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).RemovePortal)(windows_core::Interface::as_raw(self), core::mem::transmute(portalid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Delete(&self) -> windows_core::Result<IVdsAsync> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Delete)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IVdsIscsiPortalGroup_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut VDS_ISCSI_PORTALGROUP_PROP) -> windows_core::HRESULT,
    pub GetTarget: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub QueryAssociatedPortals: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddPortal: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RemovePortal: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Delete: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IVdsIscsiPortalLocal, IVdsIscsiPortalLocal_Vtbl, 0xad837c28_52c1_421d_bf04_fae7da665396);
impl core::ops::Deref for IVdsIscsiPortalLocal {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IVdsIscsiPortalLocal, windows_core::IUnknown);
impl IVdsIscsiPortalLocal {
    pub unsafe fn SetIpsecSecurityLocal(&self, ullsecurityflags: u64, pipseckey: *const VDS_ISCSI_IPSEC_KEY) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetIpsecSecurityLocal)(windows_core::Interface::as_raw(self), ullsecurityflags, pipseckey).ok()
    }
}
#[repr(C)]
pub struct IVdsIscsiPortalLocal_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetIpsecSecurityLocal: unsafe extern "system" fn(*mut core::ffi::c_void, u64, *const VDS_ISCSI_IPSEC_KEY) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IVdsIscsiTarget, IVdsIscsiTarget_Vtbl, 0xaa8f5055_83e5_4bcc_aa73_19851a36a849);
impl core::ops::Deref for IVdsIscsiTarget {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IVdsIscsiTarget, windows_core::IUnknown);
impl IVdsIscsiTarget {
    pub unsafe fn GetProperties(&self, ptargetprop: *mut VDS_ISCSI_TARGET_PROP) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetProperties)(windows_core::Interface::as_raw(self), ptargetprop).ok()
    }
    pub unsafe fn GetSubSystem(&self) -> windows_core::Result<IVdsSubSystem> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetSubSystem)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn QueryPortalGroups(&self) -> windows_core::Result<IEnumVdsObject> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).QueryPortalGroups)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn QueryAssociatedLuns(&self) -> windows_core::Result<IEnumVdsObject> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).QueryAssociatedLuns)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreatePortalGroup(&self) -> windows_core::Result<IVdsAsync> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreatePortalGroup)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Delete(&self) -> windows_core::Result<IVdsAsync> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Delete)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetFriendlyName<P0>(&self, pwszfriendlyname: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetFriendlyName)(windows_core::Interface::as_raw(self), pwszfriendlyname.param().abi()).ok()
    }
    pub unsafe fn SetSharedSecret<P0>(&self, ptargetsharedsecret: *const VDS_ISCSI_SHARED_SECRET, pwszinitiatorname: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetSharedSecret)(windows_core::Interface::as_raw(self), ptargetsharedsecret, pwszinitiatorname.param().abi()).ok()
    }
    pub unsafe fn RememberInitiatorSharedSecret<P0>(&self, pwszinitiatorname: P0, pinitiatorsharedsecret: *const VDS_ISCSI_SHARED_SECRET) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).RememberInitiatorSharedSecret)(windows_core::Interface::as_raw(self), pwszinitiatorname.param().abi(), pinitiatorsharedsecret).ok()
    }
    pub unsafe fn GetConnectedInitiators(&self, pppwszinitiatorlist: *mut *mut windows_core::PWSTR, plnumberofinitiators: *mut i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetConnectedInitiators)(windows_core::Interface::as_raw(self), pppwszinitiatorlist, plnumberofinitiators).ok()
    }
}
#[repr(C)]
pub struct IVdsIscsiTarget_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut VDS_ISCSI_TARGET_PROP) -> windows_core::HRESULT,
    pub GetSubSystem: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub QueryPortalGroups: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub QueryAssociatedLuns: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreatePortalGroup: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Delete: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetFriendlyName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub SetSharedSecret: unsafe extern "system" fn(*mut core::ffi::c_void, *const VDS_ISCSI_SHARED_SECRET, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub RememberInitiatorSharedSecret: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *const VDS_ISCSI_SHARED_SECRET) -> windows_core::HRESULT,
    pub GetConnectedInitiators: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut windows_core::PWSTR, *mut i32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IVdsLun, IVdsLun_Vtbl, 0x3540a9c7_e60f_4111_a840_8bba6c2c83d8);
impl core::ops::Deref for IVdsLun {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IVdsLun, windows_core::IUnknown);
impl IVdsLun {
    pub unsafe fn GetProperties(&self, plunprop: *mut VDS_LUN_PROP) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetProperties)(windows_core::Interface::as_raw(self), plunprop).ok()
    }
    pub unsafe fn GetSubSystem(&self) -> windows_core::Result<IVdsSubSystem> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetSubSystem)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetIdentificationData(&self, pluninfo: *mut VDS_LUN_INFORMATION) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetIdentificationData)(windows_core::Interface::as_raw(self), pluninfo).ok()
    }
    pub unsafe fn QueryActiveControllers(&self) -> windows_core::Result<IEnumVdsObject> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).QueryActiveControllers)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Extend(&self, ullnumberofbytestoadd: u64, pdriveidarray: &[windows_core::GUID]) -> windows_core::Result<IVdsAsync> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Extend)(windows_core::Interface::as_raw(self), ullnumberofbytestoadd, core::mem::transmute(pdriveidarray.as_ptr()), pdriveidarray.len().try_into().unwrap(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Shrink(&self, ullnumberofbytestoremove: u64) -> windows_core::Result<IVdsAsync> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Shrink)(windows_core::Interface::as_raw(self), ullnumberofbytestoremove, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn QueryPlexes(&self) -> windows_core::Result<IEnumVdsObject> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).QueryPlexes)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn AddPlex(&self, lunid: windows_core::GUID) -> windows_core::Result<IVdsAsync> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).AddPlex)(windows_core::Interface::as_raw(self), core::mem::transmute(lunid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn RemovePlex(&self, plexid: windows_core::GUID) -> windows_core::Result<IVdsAsync> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).RemovePlex)(windows_core::Interface::as_raw(self), core::mem::transmute(plexid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Recover(&self) -> windows_core::Result<IVdsAsync> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Recover)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetMask<P0>(&self, pwszunmaskinglist: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetMask)(windows_core::Interface::as_raw(self), pwszunmaskinglist.param().abi()).ok()
    }
    pub unsafe fn Delete(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Delete)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn AssociateControllers(&self, pactivecontrolleridarray: &[windows_core::GUID], pinactivecontrolleridarray: &[windows_core::GUID]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).AssociateControllers)(windows_core::Interface::as_raw(self), core::mem::transmute(pactivecontrolleridarray.as_ptr()), pactivecontrolleridarray.len().try_into().unwrap(), core::mem::transmute(pinactivecontrolleridarray.as_ptr()), pinactivecontrolleridarray.len().try_into().unwrap()).ok()
    }
    pub unsafe fn QueryHints(&self, phints: *mut VDS_HINTS) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).QueryHints)(windows_core::Interface::as_raw(self), phints).ok()
    }
    pub unsafe fn ApplyHints(&self, phints: *const VDS_HINTS) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ApplyHints)(windows_core::Interface::as_raw(self), phints).ok()
    }
    pub unsafe fn SetStatus(&self, status: VDS_LUN_STATUS) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetStatus)(windows_core::Interface::as_raw(self), status).ok()
    }
    pub unsafe fn QueryMaxLunExtendSize(&self, pdriveidarray: &[windows_core::GUID]) -> windows_core::Result<u64> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).QueryMaxLunExtendSize)(windows_core::Interface::as_raw(self), core::mem::transmute(pdriveidarray.as_ptr()), pdriveidarray.len().try_into().unwrap(), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IVdsLun_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut VDS_LUN_PROP) -> windows_core::HRESULT,
    pub GetSubSystem: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetIdentificationData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut VDS_LUN_INFORMATION) -> windows_core::HRESULT,
    pub QueryActiveControllers: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Extend: unsafe extern "system" fn(*mut core::ffi::c_void, u64, *const windows_core::GUID, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Shrink: unsafe extern "system" fn(*mut core::ffi::c_void, u64, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub QueryPlexes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddPlex: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RemovePlex: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Recover: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetMask: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub Delete: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AssociateControllers: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, i32, *const windows_core::GUID, i32) -> windows_core::HRESULT,
    pub QueryHints: unsafe extern "system" fn(*mut core::ffi::c_void, *mut VDS_HINTS) -> windows_core::HRESULT,
    pub ApplyHints: unsafe extern "system" fn(*mut core::ffi::c_void, *const VDS_HINTS) -> windows_core::HRESULT,
    pub SetStatus: unsafe extern "system" fn(*mut core::ffi::c_void, VDS_LUN_STATUS) -> windows_core::HRESULT,
    pub QueryMaxLunExtendSize: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, i32, *mut u64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IVdsLun2, IVdsLun2_Vtbl, 0xe5b3a735_9efb_499a_8071_4394d9ee6fcb);
impl core::ops::Deref for IVdsLun2 {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IVdsLun2, windows_core::IUnknown);
impl IVdsLun2 {
    pub unsafe fn QueryHints2(&self, phints2: *mut VDS_HINTS2) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).QueryHints2)(windows_core::Interface::as_raw(self), phints2).ok()
    }
    pub unsafe fn ApplyHints2(&self, phints2: *const VDS_HINTS2) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ApplyHints2)(windows_core::Interface::as_raw(self), phints2).ok()
    }
}
#[repr(C)]
pub struct IVdsLun2_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub QueryHints2: unsafe extern "system" fn(*mut core::ffi::c_void, *mut VDS_HINTS2) -> windows_core::HRESULT,
    pub ApplyHints2: unsafe extern "system" fn(*mut core::ffi::c_void, *const VDS_HINTS2) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IVdsLunControllerPorts, IVdsLunControllerPorts_Vtbl, 0x451fe266_da6d_406a_bb60_82e534f85aeb);
impl core::ops::Deref for IVdsLunControllerPorts {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IVdsLunControllerPorts, windows_core::IUnknown);
impl IVdsLunControllerPorts {
    pub unsafe fn AssociateControllerPorts(&self, pactivecontrollerportidarray: &[windows_core::GUID], pinactivecontrollerportidarray: &[windows_core::GUID]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).AssociateControllerPorts)(windows_core::Interface::as_raw(self), core::mem::transmute(pactivecontrollerportidarray.as_ptr()), pactivecontrollerportidarray.len().try_into().unwrap(), core::mem::transmute(pinactivecontrollerportidarray.as_ptr()), pinactivecontrollerportidarray.len().try_into().unwrap()).ok()
    }
    pub unsafe fn QueryActiveControllerPorts(&self) -> windows_core::Result<IEnumVdsObject> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).QueryActiveControllerPorts)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IVdsLunControllerPorts_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub AssociateControllerPorts: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, i32, *const windows_core::GUID, i32) -> windows_core::HRESULT,
    pub QueryActiveControllerPorts: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IVdsLunIscsi, IVdsLunIscsi_Vtbl, 0x0d7c1e64_b59b_45ae_b86a_2c2cc6a42067);
impl core::ops::Deref for IVdsLunIscsi {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IVdsLunIscsi, windows_core::IUnknown);
impl IVdsLunIscsi {
    pub unsafe fn AssociateTargets(&self, ptargetidarray: &[windows_core::GUID]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).AssociateTargets)(windows_core::Interface::as_raw(self), core::mem::transmute(ptargetidarray.as_ptr()), ptargetidarray.len().try_into().unwrap()).ok()
    }
    pub unsafe fn QueryAssociatedTargets(&self) -> windows_core::Result<IEnumVdsObject> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).QueryAssociatedTargets)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IVdsLunIscsi_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub AssociateTargets: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, i32) -> windows_core::HRESULT,
    pub QueryAssociatedTargets: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IVdsLunMpio, IVdsLunMpio_Vtbl, 0x7c5fbae3_333a_48a1_a982_33c15788cde3);
impl core::ops::Deref for IVdsLunMpio {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IVdsLunMpio, windows_core::IUnknown);
impl IVdsLunMpio {
    pub unsafe fn GetPathInfo(&self, pppaths: *mut *mut VDS_PATH_INFO, plnumberofpaths: *mut i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetPathInfo)(windows_core::Interface::as_raw(self), pppaths, plnumberofpaths).ok()
    }
    pub unsafe fn GetLoadBalancePolicy(&self, ppolicy: *mut VDS_LOADBALANCE_POLICY_ENUM, pppaths: *mut *mut VDS_PATH_POLICY, plnumberofpaths: *mut i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetLoadBalancePolicy)(windows_core::Interface::as_raw(self), ppolicy, pppaths, plnumberofpaths).ok()
    }
    pub unsafe fn SetLoadBalancePolicy(&self, policy: VDS_LOADBALANCE_POLICY_ENUM, ppaths: &[VDS_PATH_POLICY]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetLoadBalancePolicy)(windows_core::Interface::as_raw(self), policy, core::mem::transmute(ppaths.as_ptr()), ppaths.len().try_into().unwrap()).ok()
    }
    pub unsafe fn GetSupportedLbPolicies(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetSupportedLbPolicies)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IVdsLunMpio_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetPathInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut VDS_PATH_INFO, *mut i32) -> windows_core::HRESULT,
    pub GetLoadBalancePolicy: unsafe extern "system" fn(*mut core::ffi::c_void, *mut VDS_LOADBALANCE_POLICY_ENUM, *mut *mut VDS_PATH_POLICY, *mut i32) -> windows_core::HRESULT,
    pub SetLoadBalancePolicy: unsafe extern "system" fn(*mut core::ffi::c_void, VDS_LOADBALANCE_POLICY_ENUM, *const VDS_PATH_POLICY, i32) -> windows_core::HRESULT,
    pub GetSupportedLbPolicies: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IVdsLunNaming, IVdsLunNaming_Vtbl, 0x907504cb_6b4e_4d88_a34d_17ba661fbb06);
impl core::ops::Deref for IVdsLunNaming {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IVdsLunNaming, windows_core::IUnknown);
impl IVdsLunNaming {
    pub unsafe fn SetFriendlyName<P0>(&self, pwszfriendlyname: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetFriendlyName)(windows_core::Interface::as_raw(self), pwszfriendlyname.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IVdsLunNaming_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetFriendlyName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IVdsLunNumber, IVdsLunNumber_Vtbl, 0xd3f95e46_54b3_41f9_b678_0f1871443a08);
impl core::ops::Deref for IVdsLunNumber {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IVdsLunNumber, windows_core::IUnknown);
impl IVdsLunNumber {
    pub unsafe fn GetLunNumber(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetLunNumber)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IVdsLunNumber_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetLunNumber: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IVdsLunPlex, IVdsLunPlex_Vtbl, 0x0ee1a790_5d2e_4abb_8c99_c481e8be2138);
impl core::ops::Deref for IVdsLunPlex {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IVdsLunPlex, windows_core::IUnknown);
impl IVdsLunPlex {
    pub unsafe fn GetProperties(&self, pplexprop: *mut VDS_LUN_PLEX_PROP) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetProperties)(windows_core::Interface::as_raw(self), pplexprop).ok()
    }
    pub unsafe fn GetLun(&self) -> windows_core::Result<IVdsLun> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetLun)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn QueryExtents(&self, ppextentarray: *mut *mut VDS_DRIVE_EXTENT, plnumberofextents: *mut i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).QueryExtents)(windows_core::Interface::as_raw(self), ppextentarray, plnumberofextents).ok()
    }
    pub unsafe fn QueryHints(&self, phints: *mut VDS_HINTS) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).QueryHints)(windows_core::Interface::as_raw(self), phints).ok()
    }
    pub unsafe fn ApplyHints(&self, phints: *const VDS_HINTS) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ApplyHints)(windows_core::Interface::as_raw(self), phints).ok()
    }
}
#[repr(C)]
pub struct IVdsLunPlex_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut VDS_LUN_PLEX_PROP) -> windows_core::HRESULT,
    pub GetLun: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub QueryExtents: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut VDS_DRIVE_EXTENT, *mut i32) -> windows_core::HRESULT,
    pub QueryHints: unsafe extern "system" fn(*mut core::ffi::c_void, *mut VDS_HINTS) -> windows_core::HRESULT,
    pub ApplyHints: unsafe extern "system" fn(*mut core::ffi::c_void, *const VDS_HINTS) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IVdsMaintenance, IVdsMaintenance_Vtbl, 0xdaebeef3_8523_47ed_a2b9_05cecce2a1ae);
impl core::ops::Deref for IVdsMaintenance {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IVdsMaintenance, windows_core::IUnknown);
impl IVdsMaintenance {
    pub unsafe fn StartMaintenance(&self, operation: VDS_MAINTENANCE_OPERATION) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).StartMaintenance)(windows_core::Interface::as_raw(self), operation).ok()
    }
    pub unsafe fn StopMaintenance(&self, operation: VDS_MAINTENANCE_OPERATION) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).StopMaintenance)(windows_core::Interface::as_raw(self), operation).ok()
    }
    pub unsafe fn PulseMaintenance(&self, operation: VDS_MAINTENANCE_OPERATION, ulcount: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).PulseMaintenance)(windows_core::Interface::as_raw(self), operation, ulcount).ok()
    }
}
#[repr(C)]
pub struct IVdsMaintenance_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub StartMaintenance: unsafe extern "system" fn(*mut core::ffi::c_void, VDS_MAINTENANCE_OPERATION) -> windows_core::HRESULT,
    pub StopMaintenance: unsafe extern "system" fn(*mut core::ffi::c_void, VDS_MAINTENANCE_OPERATION) -> windows_core::HRESULT,
    pub PulseMaintenance: unsafe extern "system" fn(*mut core::ffi::c_void, VDS_MAINTENANCE_OPERATION, u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IVdsOpenVDisk, IVdsOpenVDisk_Vtbl, 0x75c8f324_f715_4fe3_a28e_f9011b61a4a1);
impl core::ops::Deref for IVdsOpenVDisk {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IVdsOpenVDisk, windows_core::IUnknown);
impl IVdsOpenVDisk {
    #[cfg(feature = "Win32_Storage_Vhd")]
    pub unsafe fn Attach<P0>(&self, pstringsecuritydescriptor: P0, flags: super::Vhd::ATTACH_VIRTUAL_DISK_FLAG, providerspecificflags: u32, timeoutinms: u32) -> windows_core::Result<IVdsAsync>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Attach)(windows_core::Interface::as_raw(self), pstringsecuritydescriptor.param().abi(), flags, providerspecificflags, timeoutinms, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_Storage_Vhd")]
    pub unsafe fn Detach(&self, flags: super::Vhd::DETACH_VIRTUAL_DISK_FLAG, providerspecificflags: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Detach)(windows_core::Interface::as_raw(self), flags, providerspecificflags).ok()
    }
    #[cfg(feature = "Win32_Storage_Vhd")]
    pub unsafe fn DetachAndDelete(&self, flags: super::Vhd::DETACH_VIRTUAL_DISK_FLAG, providerspecificflags: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).DetachAndDelete)(windows_core::Interface::as_raw(self), flags, providerspecificflags).ok()
    }
    #[cfg(feature = "Win32_Storage_Vhd")]
    pub unsafe fn Compact(&self, flags: super::Vhd::COMPACT_VIRTUAL_DISK_FLAG, reserved: u32) -> windows_core::Result<IVdsAsync> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Compact)(windows_core::Interface::as_raw(self), flags, reserved, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_Storage_Vhd")]
    pub unsafe fn Merge(&self, flags: super::Vhd::MERGE_VIRTUAL_DISK_FLAG, mergedepth: u32) -> windows_core::Result<IVdsAsync> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Merge)(windows_core::Interface::as_raw(self), flags, mergedepth, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_Storage_Vhd")]
    pub unsafe fn Expand(&self, flags: super::Vhd::EXPAND_VIRTUAL_DISK_FLAG, newsize: u64) -> windows_core::Result<IVdsAsync> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Expand)(windows_core::Interface::as_raw(self), flags, newsize, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IVdsOpenVDisk_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Storage_Vhd")]
    pub Attach: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, super::Vhd::ATTACH_VIRTUAL_DISK_FLAG, u32, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Vhd"))]
    Attach: usize,
    #[cfg(feature = "Win32_Storage_Vhd")]
    pub Detach: unsafe extern "system" fn(*mut core::ffi::c_void, super::Vhd::DETACH_VIRTUAL_DISK_FLAG, u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Vhd"))]
    Detach: usize,
    #[cfg(feature = "Win32_Storage_Vhd")]
    pub DetachAndDelete: unsafe extern "system" fn(*mut core::ffi::c_void, super::Vhd::DETACH_VIRTUAL_DISK_FLAG, u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Vhd"))]
    DetachAndDelete: usize,
    #[cfg(feature = "Win32_Storage_Vhd")]
    pub Compact: unsafe extern "system" fn(*mut core::ffi::c_void, super::Vhd::COMPACT_VIRTUAL_DISK_FLAG, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Vhd"))]
    Compact: usize,
    #[cfg(feature = "Win32_Storage_Vhd")]
    pub Merge: unsafe extern "system" fn(*mut core::ffi::c_void, super::Vhd::MERGE_VIRTUAL_DISK_FLAG, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Vhd"))]
    Merge: usize,
    #[cfg(feature = "Win32_Storage_Vhd")]
    pub Expand: unsafe extern "system" fn(*mut core::ffi::c_void, super::Vhd::EXPAND_VIRTUAL_DISK_FLAG, u64, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Vhd"))]
    Expand: usize,
}
windows_core::imp::define_interface!(IVdsPack, IVdsPack_Vtbl, 0x3b69d7f5_9d94_4648_91ca_79939ba263bf);
impl core::ops::Deref for IVdsPack {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IVdsPack, windows_core::IUnknown);
impl IVdsPack {
    pub unsafe fn GetProperties(&self, ppackprop: *mut VDS_PACK_PROP) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetProperties)(windows_core::Interface::as_raw(self), ppackprop).ok()
    }
    pub unsafe fn GetProvider(&self) -> windows_core::Result<IVdsProvider> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetProvider)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn QueryVolumes(&self) -> windows_core::Result<IEnumVdsObject> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).QueryVolumes)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn QueryDisks(&self) -> windows_core::Result<IEnumVdsObject> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).QueryDisks)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateVolume(&self, r#type: VDS_VOLUME_TYPE, pinputdiskarray: &[VDS_INPUT_DISK], ulstripesize: u32) -> windows_core::Result<IVdsAsync> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateVolume)(windows_core::Interface::as_raw(self), r#type, core::mem::transmute(pinputdiskarray.as_ptr()), pinputdiskarray.len().try_into().unwrap(), ulstripesize, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn AddDisk<P0>(&self, diskid: windows_core::GUID, partitionstyle: VDS_PARTITION_STYLE, bashotspare: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).AddDisk)(windows_core::Interface::as_raw(self), core::mem::transmute(diskid), partitionstyle, bashotspare.param().abi()).ok()
    }
    pub unsafe fn MigrateDisks<P0, P1>(&self, pdiskarray: *const windows_core::GUID, lnumberofdisks: i32, targetpack: windows_core::GUID, bforce: P0, bqueryonly: P1, presults: *mut windows_core::HRESULT, pbrebootneeded: *mut super::super::Foundation::BOOL) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
        P1: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).MigrateDisks)(windows_core::Interface::as_raw(self), pdiskarray, lnumberofdisks, core::mem::transmute(targetpack), bforce.param().abi(), bqueryonly.param().abi(), presults, pbrebootneeded).ok()
    }
    pub unsafe fn ReplaceDisk(&self, olddiskid: windows_core::GUID, newdiskid: windows_core::GUID) -> windows_core::Result<IVdsAsync> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ReplaceDisk)(windows_core::Interface::as_raw(self), core::mem::transmute(olddiskid), core::mem::transmute(newdiskid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn RemoveMissingDisk(&self, diskid: windows_core::GUID) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).RemoveMissingDisk)(windows_core::Interface::as_raw(self), core::mem::transmute(diskid)).ok()
    }
    pub unsafe fn Recover(&self) -> windows_core::Result<IVdsAsync> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Recover)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IVdsPack_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut VDS_PACK_PROP) -> windows_core::HRESULT,
    pub GetProvider: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub QueryVolumes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub QueryDisks: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateVolume: unsafe extern "system" fn(*mut core::ffi::c_void, VDS_VOLUME_TYPE, *const VDS_INPUT_DISK, i32, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddDisk: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, VDS_PARTITION_STYLE, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub MigrateDisks: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, i32, windows_core::GUID, super::super::Foundation::BOOL, super::super::Foundation::BOOL, *mut windows_core::HRESULT, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub ReplaceDisk: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RemoveMissingDisk: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID) -> windows_core::HRESULT,
    pub Recover: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IVdsPack2, IVdsPack2_Vtbl, 0x13b50bff_290a_47dd_8558_b7c58db1a71a);
impl core::ops::Deref for IVdsPack2 {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IVdsPack2, windows_core::IUnknown);
impl IVdsPack2 {
    pub unsafe fn CreateVolume2(&self, r#type: VDS_VOLUME_TYPE, pinputdiskarray: &[VDS_INPUT_DISK], ulstripesize: u32, ulalign: u32) -> windows_core::Result<IVdsAsync> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateVolume2)(windows_core::Interface::as_raw(self), r#type, core::mem::transmute(pinputdiskarray.as_ptr()), pinputdiskarray.len().try_into().unwrap(), ulstripesize, ulalign, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IVdsPack2_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub CreateVolume2: unsafe extern "system" fn(*mut core::ffi::c_void, VDS_VOLUME_TYPE, *const VDS_INPUT_DISK, i32, u32, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IVdsProvider, IVdsProvider_Vtbl, 0x10c5e575_7984_4e81_a56b_431f5f92ae42);
impl core::ops::Deref for IVdsProvider {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IVdsProvider, windows_core::IUnknown);
impl IVdsProvider {
    pub unsafe fn GetProperties(&self, pproviderprop: *mut VDS_PROVIDER_PROP) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetProperties)(windows_core::Interface::as_raw(self), pproviderprop).ok()
    }
}
#[repr(C)]
pub struct IVdsProvider_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut VDS_PROVIDER_PROP) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IVdsProviderPrivate, IVdsProviderPrivate_Vtbl, 0x11f3cd41_b7e8_48ff_9472_9dff018aa292);
impl core::ops::Deref for IVdsProviderPrivate {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IVdsProviderPrivate, windows_core::IUnknown);
impl IVdsProviderPrivate {
    pub unsafe fn GetObject(&self, objectid: windows_core::GUID, r#type: VDS_OBJECT_TYPE) -> windows_core::Result<windows_core::IUnknown> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetObject)(windows_core::Interface::as_raw(self), core::mem::transmute(objectid), r#type, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn OnLoad<P0, P1>(&self, pwszmachinename: P0, pcallbackobject: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::IUnknown>,
    {
        (windows_core::Interface::vtable(self).OnLoad)(windows_core::Interface::as_raw(self), pwszmachinename.param().abi(), pcallbackobject.param().abi()).ok()
    }
    pub unsafe fn OnUnload<P0>(&self, bforceunload: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).OnUnload)(windows_core::Interface::as_raw(self), bforceunload.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IVdsProviderPrivate_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetObject: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, VDS_OBJECT_TYPE, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OnLoad: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OnUnload: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::BOOL) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IVdsProviderSupport, IVdsProviderSupport_Vtbl, 0x1732be13_e8f9_4a03_bfbc_5f616aa66ce1);
impl core::ops::Deref for IVdsProviderSupport {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IVdsProviderSupport, windows_core::IUnknown);
impl IVdsProviderSupport {
    pub unsafe fn GetVersionSupport(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetVersionSupport)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IVdsProviderSupport_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetVersionSupport: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IVdsRemovable, IVdsRemovable_Vtbl, 0x0316560b_5db4_4ed9_bbb5_213436ddc0d9);
impl core::ops::Deref for IVdsRemovable {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IVdsRemovable, windows_core::IUnknown);
impl IVdsRemovable {
    pub unsafe fn QueryMedia(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).QueryMedia)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Eject(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Eject)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct IVdsRemovable_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub QueryMedia: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Eject: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IVdsService, IVdsService_Vtbl, 0x0818a8ef_9ba9_40d8_a6f9_e22833cc771e);
impl core::ops::Deref for IVdsService {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IVdsService, windows_core::IUnknown);
impl IVdsService {
    pub unsafe fn IsServiceReady(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).IsServiceReady)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn WaitForServiceReady(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).WaitForServiceReady)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetProperties(&self) -> windows_core::Result<VDS_SERVICE_PROP> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetProperties)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn QueryProviders(&self, masks: u32) -> windows_core::Result<IEnumVdsObject> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).QueryProviders)(windows_core::Interface::as_raw(self), masks, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn QueryMaskedDisks(&self) -> windows_core::Result<IEnumVdsObject> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).QueryMaskedDisks)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn QueryUnallocatedDisks(&self) -> windows_core::Result<IEnumVdsObject> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).QueryUnallocatedDisks)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetObject(&self, objectid: windows_core::GUID, r#type: VDS_OBJECT_TYPE) -> windows_core::Result<windows_core::IUnknown> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetObject)(windows_core::Interface::as_raw(self), core::mem::transmute(objectid), r#type, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn QueryDriveLetters(&self, wcfirstletter: u16, pdriveletterproparray: &mut [VDS_DRIVE_LETTER_PROP]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).QueryDriveLetters)(windows_core::Interface::as_raw(self), wcfirstletter, pdriveletterproparray.len().try_into().unwrap(), core::mem::transmute(pdriveletterproparray.as_ptr())).ok()
    }
    pub unsafe fn QueryFileSystemTypes(&self, ppfilesystemtypeprops: *mut *mut VDS_FILE_SYSTEM_TYPE_PROP, plnumberoffilesystems: *mut i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).QueryFileSystemTypes)(windows_core::Interface::as_raw(self), ppfilesystemtypeprops, plnumberoffilesystems).ok()
    }
    pub unsafe fn Reenumerate(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Reenumerate)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Refresh(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Refresh)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn CleanupObsoleteMountPoints(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).CleanupObsoleteMountPoints)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Advise<P0>(&self, psink: P0) -> windows_core::Result<u32>
    where
        P0: windows_core::Param<IVdsAdviseSink>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Advise)(windows_core::Interface::as_raw(self), psink.param().abi(), &mut result__).map(|| result__)
    }
    pub unsafe fn Unadvise(&self, dwcookie: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Unadvise)(windows_core::Interface::as_raw(self), dwcookie).ok()
    }
    pub unsafe fn Reboot(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Reboot)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SetFlags(&self, ulflags: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetFlags)(windows_core::Interface::as_raw(self), ulflags).ok()
    }
    pub unsafe fn ClearFlags(&self, ulflags: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ClearFlags)(windows_core::Interface::as_raw(self), ulflags).ok()
    }
}
#[repr(C)]
pub struct IVdsService_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub IsServiceReady: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub WaitForServiceReady: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut VDS_SERVICE_PROP) -> windows_core::HRESULT,
    pub QueryProviders: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub QueryMaskedDisks: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub QueryUnallocatedDisks: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetObject: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, VDS_OBJECT_TYPE, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub QueryDriveLetters: unsafe extern "system" fn(*mut core::ffi::c_void, u16, u32, *mut VDS_DRIVE_LETTER_PROP) -> windows_core::HRESULT,
    pub QueryFileSystemTypes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut VDS_FILE_SYSTEM_TYPE_PROP, *mut i32) -> windows_core::HRESULT,
    pub Reenumerate: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Refresh: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CleanupObsoleteMountPoints: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Advise: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Unadvise: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Reboot: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetFlags: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub ClearFlags: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IVdsServiceHba, IVdsServiceHba_Vtbl, 0x0ac13689_3134_47c6_a17c_4669216801be);
impl core::ops::Deref for IVdsServiceHba {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IVdsServiceHba, windows_core::IUnknown);
impl IVdsServiceHba {
    pub unsafe fn QueryHbaPorts(&self) -> windows_core::Result<IEnumVdsObject> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).QueryHbaPorts)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IVdsServiceHba_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub QueryHbaPorts: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IVdsServiceInitialization, IVdsServiceInitialization_Vtbl, 0x4afc3636_db01_4052_80c3_03bbcb8d3c69);
impl core::ops::Deref for IVdsServiceInitialization {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IVdsServiceInitialization, windows_core::IUnknown);
impl IVdsServiceInitialization {
    pub unsafe fn Initialize<P0>(&self, pwszmachinename: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self), pwszmachinename.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IVdsServiceInitialization_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IVdsServiceIscsi, IVdsServiceIscsi_Vtbl, 0x14fbe036_3ed7_4e10_90e9_a5ff991aff01);
impl core::ops::Deref for IVdsServiceIscsi {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IVdsServiceIscsi, windows_core::IUnknown);
impl IVdsServiceIscsi {
    pub unsafe fn GetInitiatorName(&self) -> windows_core::Result<windows_core::PWSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetInitiatorName)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn QueryInitiatorAdapters(&self) -> windows_core::Result<IEnumVdsObject> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).QueryInitiatorAdapters)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetIpsecGroupPresharedKey(&self, pipseckey: *const VDS_ISCSI_IPSEC_KEY) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetIpsecGroupPresharedKey)(windows_core::Interface::as_raw(self), pipseckey).ok()
    }
    pub unsafe fn SetAllIpsecTunnelAddresses(&self, ptunneladdress: *const VDS_IPADDRESS, pdestinationaddress: *const VDS_IPADDRESS) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetAllIpsecTunnelAddresses)(windows_core::Interface::as_raw(self), ptunneladdress, pdestinationaddress).ok()
    }
    pub unsafe fn SetAllIpsecSecurity(&self, targetportalid: windows_core::GUID, ullsecurityflags: u64, pipseckey: *const VDS_ISCSI_IPSEC_KEY) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetAllIpsecSecurity)(windows_core::Interface::as_raw(self), core::mem::transmute(targetportalid), ullsecurityflags, pipseckey).ok()
    }
    pub unsafe fn SetInitiatorSharedSecret(&self, pinitiatorsharedsecret: *const VDS_ISCSI_SHARED_SECRET, targetid: windows_core::GUID) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetInitiatorSharedSecret)(windows_core::Interface::as_raw(self), pinitiatorsharedsecret, core::mem::transmute(targetid)).ok()
    }
    pub unsafe fn RememberTargetSharedSecret(&self, targetid: windows_core::GUID, ptargetsharedsecret: *const VDS_ISCSI_SHARED_SECRET) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).RememberTargetSharedSecret)(windows_core::Interface::as_raw(self), core::mem::transmute(targetid), ptargetsharedsecret).ok()
    }
}
#[repr(C)]
pub struct IVdsServiceIscsi_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetInitiatorName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub QueryInitiatorAdapters: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetIpsecGroupPresharedKey: unsafe extern "system" fn(*mut core::ffi::c_void, *const VDS_ISCSI_IPSEC_KEY) -> windows_core::HRESULT,
    pub SetAllIpsecTunnelAddresses: unsafe extern "system" fn(*mut core::ffi::c_void, *const VDS_IPADDRESS, *const VDS_IPADDRESS) -> windows_core::HRESULT,
    pub SetAllIpsecSecurity: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, u64, *const VDS_ISCSI_IPSEC_KEY) -> windows_core::HRESULT,
    pub SetInitiatorSharedSecret: unsafe extern "system" fn(*mut core::ffi::c_void, *const VDS_ISCSI_SHARED_SECRET, windows_core::GUID) -> windows_core::HRESULT,
    pub RememberTargetSharedSecret: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, *const VDS_ISCSI_SHARED_SECRET) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IVdsServiceLoader, IVdsServiceLoader_Vtbl, 0xe0393303_90d4_4a97_ab71_e9b671ee2729);
impl core::ops::Deref for IVdsServiceLoader {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IVdsServiceLoader, windows_core::IUnknown);
impl IVdsServiceLoader {
    pub unsafe fn LoadService<P0>(&self, pwszmachinename: P0) -> windows_core::Result<IVdsService>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).LoadService)(windows_core::Interface::as_raw(self), pwszmachinename.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IVdsServiceLoader_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub LoadService: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IVdsServiceSAN, IVdsServiceSAN_Vtbl, 0xfc5d23e8_a88b_41a5_8de0_2d2f73c5a630);
impl core::ops::Deref for IVdsServiceSAN {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IVdsServiceSAN, windows_core::IUnknown);
impl IVdsServiceSAN {
    pub unsafe fn GetSANPolicy(&self) -> windows_core::Result<VDS_SAN_POLICY> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetSANPolicy)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetSANPolicy(&self, sanpolicy: VDS_SAN_POLICY) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetSANPolicy)(windows_core::Interface::as_raw(self), sanpolicy).ok()
    }
}
#[repr(C)]
pub struct IVdsServiceSAN_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetSANPolicy: unsafe extern "system" fn(*mut core::ffi::c_void, *mut VDS_SAN_POLICY) -> windows_core::HRESULT,
    pub SetSANPolicy: unsafe extern "system" fn(*mut core::ffi::c_void, VDS_SAN_POLICY) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IVdsServiceSw, IVdsServiceSw_Vtbl, 0x15fc031c_0652_4306_b2c3_f558b8f837e2);
impl core::ops::Deref for IVdsServiceSw {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IVdsServiceSw, windows_core::IUnknown);
impl IVdsServiceSw {
    pub unsafe fn GetDiskObject<P0>(&self, pwszdeviceid: P0) -> windows_core::Result<windows_core::IUnknown>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetDiskObject)(windows_core::Interface::as_raw(self), pwszdeviceid.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IVdsServiceSw_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetDiskObject: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IVdsServiceUninstallDisk, IVdsServiceUninstallDisk_Vtbl, 0xb6b22da8_f903_4be7_b492_c09d875ac9da);
impl core::ops::Deref for IVdsServiceUninstallDisk {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IVdsServiceUninstallDisk, windows_core::IUnknown);
impl IVdsServiceUninstallDisk {
    pub unsafe fn GetDiskIdFromLunInfo(&self, pluninfo: *const VDS_LUN_INFORMATION) -> windows_core::Result<windows_core::GUID> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetDiskIdFromLunInfo)(windows_core::Interface::as_raw(self), pluninfo, &mut result__).map(|| result__)
    }
    pub unsafe fn UninstallDisks<P0>(&self, pdiskidarray: *const windows_core::GUID, ulcount: u32, bforce: P0, pbreboot: *mut u8, presults: *mut windows_core::HRESULT) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOLEAN>,
    {
        (windows_core::Interface::vtable(self).UninstallDisks)(windows_core::Interface::as_raw(self), pdiskidarray, ulcount, bforce.param().abi(), pbreboot, presults).ok()
    }
}
#[repr(C)]
pub struct IVdsServiceUninstallDisk_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetDiskIdFromLunInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *const VDS_LUN_INFORMATION, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub UninstallDisks: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, u32, super::super::Foundation::BOOLEAN, *mut u8, *mut windows_core::HRESULT) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IVdsStoragePool, IVdsStoragePool_Vtbl, 0x932ca8cf_0eb3_4ba8_9620_22665d7f8450);
impl core::ops::Deref for IVdsStoragePool {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IVdsStoragePool, windows_core::IUnknown);
impl IVdsStoragePool {
    pub unsafe fn GetProvider(&self) -> windows_core::Result<IVdsProvider> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetProvider)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetProperties(&self, pstoragepoolprop: *mut VDS_STORAGE_POOL_PROP) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetProperties)(windows_core::Interface::as_raw(self), pstoragepoolprop).ok()
    }
    pub unsafe fn GetAttributes(&self, pstoragepoolattributes: *mut VDS_POOL_ATTRIBUTES) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetAttributes)(windows_core::Interface::as_raw(self), pstoragepoolattributes).ok()
    }
    pub unsafe fn QueryDriveExtents(&self, ppextentarray: *mut *mut VDS_STORAGE_POOL_DRIVE_EXTENT, plnumberofextents: *mut i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).QueryDriveExtents)(windows_core::Interface::as_raw(self), ppextentarray, plnumberofextents).ok()
    }
    pub unsafe fn QueryAllocatedLuns(&self) -> windows_core::Result<IEnumVdsObject> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).QueryAllocatedLuns)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn QueryAllocatedStoragePools(&self) -> windows_core::Result<IEnumVdsObject> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).QueryAllocatedStoragePools)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IVdsStoragePool_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetProvider: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut VDS_STORAGE_POOL_PROP) -> windows_core::HRESULT,
    pub GetAttributes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut VDS_POOL_ATTRIBUTES) -> windows_core::HRESULT,
    pub QueryDriveExtents: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut VDS_STORAGE_POOL_DRIVE_EXTENT, *mut i32) -> windows_core::HRESULT,
    pub QueryAllocatedLuns: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub QueryAllocatedStoragePools: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IVdsSubSystem, IVdsSubSystem_Vtbl, 0x6fcee2d3_6d90_4f91_80e2_a5c7caaca9d8);
impl core::ops::Deref for IVdsSubSystem {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IVdsSubSystem, windows_core::IUnknown);
impl IVdsSubSystem {
    pub unsafe fn GetProperties(&self, psubsystemprop: *mut VDS_SUB_SYSTEM_PROP) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetProperties)(windows_core::Interface::as_raw(self), psubsystemprop).ok()
    }
    pub unsafe fn GetProvider(&self) -> windows_core::Result<IVdsProvider> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetProvider)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn QueryControllers(&self) -> windows_core::Result<IEnumVdsObject> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).QueryControllers)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn QueryLuns(&self) -> windows_core::Result<IEnumVdsObject> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).QueryLuns)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn QueryDrives(&self) -> windows_core::Result<IEnumVdsObject> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).QueryDrives)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetDrive(&self, sbusnumber: i16, sslotnumber: i16) -> windows_core::Result<IVdsDrive> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetDrive)(windows_core::Interface::as_raw(self), sbusnumber, sslotnumber, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Reenumerate(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Reenumerate)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SetControllerStatus(&self, ponlinecontrolleridarray: &[windows_core::GUID], pofflinecontrolleridarray: &[windows_core::GUID]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetControllerStatus)(windows_core::Interface::as_raw(self), core::mem::transmute(ponlinecontrolleridarray.as_ptr()), ponlinecontrolleridarray.len().try_into().unwrap(), core::mem::transmute(pofflinecontrolleridarray.as_ptr()), pofflinecontrolleridarray.len().try_into().unwrap()).ok()
    }
    pub unsafe fn CreateLun<P0>(&self, r#type: VDS_LUN_TYPE, ullsizeinbytes: u64, pdriveidarray: &[windows_core::GUID], pwszunmaskinglist: P0, phints: *const VDS_HINTS) -> windows_core::Result<IVdsAsync>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateLun)(windows_core::Interface::as_raw(self), r#type, ullsizeinbytes, core::mem::transmute(pdriveidarray.as_ptr()), pdriveidarray.len().try_into().unwrap(), pwszunmaskinglist.param().abi(), phints, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn ReplaceDrive(&self, drivetobereplaced: windows_core::GUID, replacementdrive: windows_core::GUID) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ReplaceDrive)(windows_core::Interface::as_raw(self), core::mem::transmute(drivetobereplaced), core::mem::transmute(replacementdrive)).ok()
    }
    pub unsafe fn SetStatus(&self, status: VDS_SUB_SYSTEM_STATUS) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetStatus)(windows_core::Interface::as_raw(self), status).ok()
    }
    pub unsafe fn QueryMaxLunCreateSize(&self, r#type: VDS_LUN_TYPE, pdriveidarray: &[windows_core::GUID], phints: *const VDS_HINTS) -> windows_core::Result<u64> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).QueryMaxLunCreateSize)(windows_core::Interface::as_raw(self), r#type, core::mem::transmute(pdriveidarray.as_ptr()), pdriveidarray.len().try_into().unwrap(), phints, &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IVdsSubSystem_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut VDS_SUB_SYSTEM_PROP) -> windows_core::HRESULT,
    pub GetProvider: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub QueryControllers: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub QueryLuns: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub QueryDrives: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDrive: unsafe extern "system" fn(*mut core::ffi::c_void, i16, i16, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Reenumerate: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetControllerStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, i32, *const windows_core::GUID, i32) -> windows_core::HRESULT,
    pub CreateLun: unsafe extern "system" fn(*mut core::ffi::c_void, VDS_LUN_TYPE, u64, *const windows_core::GUID, i32, windows_core::PCWSTR, *const VDS_HINTS, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ReplaceDrive: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, windows_core::GUID) -> windows_core::HRESULT,
    pub SetStatus: unsafe extern "system" fn(*mut core::ffi::c_void, VDS_SUB_SYSTEM_STATUS) -> windows_core::HRESULT,
    pub QueryMaxLunCreateSize: unsafe extern "system" fn(*mut core::ffi::c_void, VDS_LUN_TYPE, *const windows_core::GUID, i32, *const VDS_HINTS, *mut u64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IVdsSubSystem2, IVdsSubSystem2_Vtbl, 0xbe666735_7800_4a77_9d9c_40f85b87e292);
impl core::ops::Deref for IVdsSubSystem2 {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IVdsSubSystem2, windows_core::IUnknown);
impl IVdsSubSystem2 {
    pub unsafe fn GetProperties2(&self, psubsystemprop2: *mut VDS_SUB_SYSTEM_PROP2) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetProperties2)(windows_core::Interface::as_raw(self), psubsystemprop2).ok()
    }
    pub unsafe fn GetDrive2(&self, sbusnumber: i16, sslotnumber: i16, ulenclosurenumber: u32) -> windows_core::Result<IVdsDrive> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetDrive2)(windows_core::Interface::as_raw(self), sbusnumber, sslotnumber, ulenclosurenumber, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateLun2<P0>(&self, r#type: VDS_LUN_TYPE, ullsizeinbytes: u64, pdriveidarray: &[windows_core::GUID], pwszunmaskinglist: P0, phints2: *const VDS_HINTS2) -> windows_core::Result<IVdsAsync>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateLun2)(windows_core::Interface::as_raw(self), r#type, ullsizeinbytes, core::mem::transmute(pdriveidarray.as_ptr()), pdriveidarray.len().try_into().unwrap(), pwszunmaskinglist.param().abi(), phints2, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn QueryMaxLunCreateSize2(&self, r#type: VDS_LUN_TYPE, pdriveidarray: &[windows_core::GUID], phints2: *const VDS_HINTS2) -> windows_core::Result<u64> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).QueryMaxLunCreateSize2)(windows_core::Interface::as_raw(self), r#type, core::mem::transmute(pdriveidarray.as_ptr()), pdriveidarray.len().try_into().unwrap(), phints2, &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IVdsSubSystem2_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetProperties2: unsafe extern "system" fn(*mut core::ffi::c_void, *mut VDS_SUB_SYSTEM_PROP2) -> windows_core::HRESULT,
    pub GetDrive2: unsafe extern "system" fn(*mut core::ffi::c_void, i16, i16, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateLun2: unsafe extern "system" fn(*mut core::ffi::c_void, VDS_LUN_TYPE, u64, *const windows_core::GUID, i32, windows_core::PCWSTR, *const VDS_HINTS2, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub QueryMaxLunCreateSize2: unsafe extern "system" fn(*mut core::ffi::c_void, VDS_LUN_TYPE, *const windows_core::GUID, i32, *const VDS_HINTS2, *mut u64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IVdsSubSystemImportTarget, IVdsSubSystemImportTarget_Vtbl, 0x83bfb87f_43fb_4903_baa6_127f01029eec);
impl core::ops::Deref for IVdsSubSystemImportTarget {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IVdsSubSystemImportTarget, windows_core::IUnknown);
impl IVdsSubSystemImportTarget {
    pub unsafe fn GetImportTarget(&self) -> windows_core::Result<windows_core::PWSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetImportTarget)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetImportTarget<P0>(&self, pwsziscsiname: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetImportTarget)(windows_core::Interface::as_raw(self), pwsziscsiname.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IVdsSubSystemImportTarget_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetImportTarget: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub SetImportTarget: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IVdsSubSystemInterconnect, IVdsSubSystemInterconnect_Vtbl, 0x9e6fa560_c141_477b_83ba_0b6c38f7febf);
impl core::ops::Deref for IVdsSubSystemInterconnect {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IVdsSubSystemInterconnect, windows_core::IUnknown);
impl IVdsSubSystemInterconnect {
    pub unsafe fn GetSupportedInterconnects(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetSupportedInterconnects)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IVdsSubSystemInterconnect_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetSupportedInterconnects: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IVdsSubSystemIscsi, IVdsSubSystemIscsi_Vtbl, 0x0027346f_40d0_4b45_8cec_5906dc0380c8);
impl core::ops::Deref for IVdsSubSystemIscsi {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IVdsSubSystemIscsi, windows_core::IUnknown);
impl IVdsSubSystemIscsi {
    pub unsafe fn QueryTargets(&self) -> windows_core::Result<IEnumVdsObject> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).QueryTargets)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn QueryPortals(&self) -> windows_core::Result<IEnumVdsObject> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).QueryPortals)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateTarget<P0, P1>(&self, pwsziscsiname: P0, pwszfriendlyname: P1) -> windows_core::Result<IVdsAsync>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateTarget)(windows_core::Interface::as_raw(self), pwsziscsiname.param().abi(), pwszfriendlyname.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetIpsecGroupPresharedKey(&self, pipseckey: *const VDS_ISCSI_IPSEC_KEY) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetIpsecGroupPresharedKey)(windows_core::Interface::as_raw(self), pipseckey).ok()
    }
}
#[repr(C)]
pub struct IVdsSubSystemIscsi_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub QueryTargets: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub QueryPortals: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateTarget: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetIpsecGroupPresharedKey: unsafe extern "system" fn(*mut core::ffi::c_void, *const VDS_ISCSI_IPSEC_KEY) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IVdsSubSystemNaming, IVdsSubSystemNaming_Vtbl, 0x0d70faa3_9cd4_4900_aa20_6981b6aafc75);
impl core::ops::Deref for IVdsSubSystemNaming {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IVdsSubSystemNaming, windows_core::IUnknown);
impl IVdsSubSystemNaming {
    pub unsafe fn SetFriendlyName<P0>(&self, pwszfriendlyname: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetFriendlyName)(windows_core::Interface::as_raw(self), pwszfriendlyname.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IVdsSubSystemNaming_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetFriendlyName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IVdsSwProvider, IVdsSwProvider_Vtbl, 0x9aa58360_ce33_4f92_b658_ed24b14425b8);
impl core::ops::Deref for IVdsSwProvider {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IVdsSwProvider, windows_core::IUnknown);
impl IVdsSwProvider {
    pub unsafe fn QueryPacks(&self) -> windows_core::Result<IEnumVdsObject> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).QueryPacks)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreatePack(&self) -> windows_core::Result<IVdsPack> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreatePack)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IVdsSwProvider_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub QueryPacks: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreatePack: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IVdsVDisk, IVdsVDisk_Vtbl, 0x1e062b84_e5e6_4b4b_8a25_67b81e8f13e8);
impl core::ops::Deref for IVdsVDisk {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IVdsVDisk, windows_core::IUnknown);
impl IVdsVDisk {
    #[cfg(feature = "Win32_Storage_Vhd")]
    pub unsafe fn Open(&self, accessmask: super::Vhd::VIRTUAL_DISK_ACCESS_MASK, flags: super::Vhd::OPEN_VIRTUAL_DISK_FLAG, readwritedepth: u32) -> windows_core::Result<IVdsOpenVDisk> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Open)(windows_core::Interface::as_raw(self), accessmask, flags, readwritedepth, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_Storage_Vhd")]
    pub unsafe fn GetProperties(&self, pdiskproperties: *mut VDS_VDISK_PROPERTIES) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetProperties)(windows_core::Interface::as_raw(self), pdiskproperties).ok()
    }
    pub unsafe fn GetHostVolume(&self) -> windows_core::Result<IVdsVolume> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetHostVolume)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetDeviceName(&self) -> windows_core::Result<windows_core::PWSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetDeviceName)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IVdsVDisk_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Storage_Vhd")]
    pub Open: unsafe extern "system" fn(*mut core::ffi::c_void, super::Vhd::VIRTUAL_DISK_ACCESS_MASK, super::Vhd::OPEN_VIRTUAL_DISK_FLAG, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Vhd"))]
    Open: usize,
    #[cfg(feature = "Win32_Storage_Vhd")]
    pub GetProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut VDS_VDISK_PROPERTIES) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Vhd"))]
    GetProperties: usize,
    pub GetHostVolume: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDeviceName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IVdsVdProvider, IVdsVdProvider_Vtbl, 0xb481498c_8354_45f9_84a0_0bdd2832a91f);
impl core::ops::Deref for IVdsVdProvider {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IVdsVdProvider, windows_core::IUnknown);
impl IVdsVdProvider {
    pub unsafe fn QueryVDisks(&self) -> windows_core::Result<IEnumVdsObject> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).QueryVDisks)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_Storage_Vhd")]
    pub unsafe fn CreateVDisk<P0, P1>(&self, virtualdevicetype: *const super::Vhd::VIRTUAL_STORAGE_TYPE, ppath: P0, pstringsecuritydescriptor: P1, flags: super::Vhd::CREATE_VIRTUAL_DISK_FLAG, providerspecificflags: u32, reserved: u32, pcreatediskparameters: *const VDS_CREATE_VDISK_PARAMETERS, ppasync: *mut Option<IVdsAsync>) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).CreateVDisk)(windows_core::Interface::as_raw(self), virtualdevicetype, ppath.param().abi(), pstringsecuritydescriptor.param().abi(), flags, providerspecificflags, reserved, pcreatediskparameters, core::mem::transmute(ppasync)).ok()
    }
    #[cfg(feature = "Win32_Storage_Vhd")]
    pub unsafe fn AddVDisk<P0>(&self, virtualdevicetype: *const super::Vhd::VIRTUAL_STORAGE_TYPE, ppath: P0, ppvdisk: *mut Option<IVdsVDisk>) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).AddVDisk)(windows_core::Interface::as_raw(self), virtualdevicetype, ppath.param().abi(), core::mem::transmute(ppvdisk)).ok()
    }
    pub unsafe fn GetDiskFromVDisk<P0>(&self, pvdisk: P0) -> windows_core::Result<IVdsDisk>
    where
        P0: windows_core::Param<IVdsVDisk>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetDiskFromVDisk)(windows_core::Interface::as_raw(self), pvdisk.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetVDiskFromDisk<P0>(&self, pdisk: P0) -> windows_core::Result<IVdsVDisk>
    where
        P0: windows_core::Param<IVdsDisk>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetVDiskFromDisk)(windows_core::Interface::as_raw(self), pdisk.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IVdsVdProvider_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub QueryVDisks: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_Storage_Vhd")]
    pub CreateVDisk: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::Vhd::VIRTUAL_STORAGE_TYPE, windows_core::PCWSTR, windows_core::PCWSTR, super::Vhd::CREATE_VIRTUAL_DISK_FLAG, u32, u32, *const VDS_CREATE_VDISK_PARAMETERS, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Vhd"))]
    CreateVDisk: usize,
    #[cfg(feature = "Win32_Storage_Vhd")]
    pub AddVDisk: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::Vhd::VIRTUAL_STORAGE_TYPE, windows_core::PCWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Vhd"))]
    AddVDisk: usize,
    pub GetDiskFromVDisk: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetVDiskFromDisk: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IVdsVolume, IVdsVolume_Vtbl, 0x88306bb2_e71f_478c_86a2_79da200a0f11);
impl core::ops::Deref for IVdsVolume {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IVdsVolume, windows_core::IUnknown);
impl IVdsVolume {
    pub unsafe fn GetProperties(&self, pvolumeproperties: *mut VDS_VOLUME_PROP) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetProperties)(windows_core::Interface::as_raw(self), pvolumeproperties).ok()
    }
    pub unsafe fn GetPack(&self) -> windows_core::Result<IVdsPack> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetPack)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn QueryPlexes(&self) -> windows_core::Result<IEnumVdsObject> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).QueryPlexes)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Extend(&self, pinputdiskarray: &[VDS_INPUT_DISK]) -> windows_core::Result<IVdsAsync> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Extend)(windows_core::Interface::as_raw(self), core::mem::transmute(pinputdiskarray.as_ptr()), pinputdiskarray.len().try_into().unwrap(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Shrink(&self, ullnumberofbytestoremove: u64) -> windows_core::Result<IVdsAsync> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Shrink)(windows_core::Interface::as_raw(self), ullnumberofbytestoremove, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn AddPlex(&self, volumeid: windows_core::GUID) -> windows_core::Result<IVdsAsync> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).AddPlex)(windows_core::Interface::as_raw(self), core::mem::transmute(volumeid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn BreakPlex(&self, plexid: windows_core::GUID) -> windows_core::Result<IVdsAsync> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).BreakPlex)(windows_core::Interface::as_raw(self), core::mem::transmute(plexid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn RemovePlex(&self, plexid: windows_core::GUID) -> windows_core::Result<IVdsAsync> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).RemovePlex)(windows_core::Interface::as_raw(self), core::mem::transmute(plexid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Delete<P0>(&self, bforce: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).Delete)(windows_core::Interface::as_raw(self), bforce.param().abi()).ok()
    }
    pub unsafe fn SetFlags<P0>(&self, ulflags: u32, brevertonclose: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).SetFlags)(windows_core::Interface::as_raw(self), ulflags, brevertonclose.param().abi()).ok()
    }
    pub unsafe fn ClearFlags(&self, ulflags: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ClearFlags)(windows_core::Interface::as_raw(self), ulflags).ok()
    }
}
#[repr(C)]
pub struct IVdsVolume_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut VDS_VOLUME_PROP) -> windows_core::HRESULT,
    pub GetPack: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub QueryPlexes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Extend: unsafe extern "system" fn(*mut core::ffi::c_void, *const VDS_INPUT_DISK, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Shrink: unsafe extern "system" fn(*mut core::ffi::c_void, u64, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddPlex: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub BreakPlex: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RemovePlex: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Delete: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub SetFlags: unsafe extern "system" fn(*mut core::ffi::c_void, u32, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub ClearFlags: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IVdsVolume2, IVdsVolume2_Vtbl, 0x72ae6713_dcbb_4a03_b36b_371f6ac6b53d);
impl core::ops::Deref for IVdsVolume2 {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IVdsVolume2, windows_core::IUnknown);
impl IVdsVolume2 {
    pub unsafe fn GetProperties2(&self, pvolumeproperties: *mut VDS_VOLUME_PROP2) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetProperties2)(windows_core::Interface::as_raw(self), pvolumeproperties).ok()
    }
}
#[repr(C)]
pub struct IVdsVolume2_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetProperties2: unsafe extern "system" fn(*mut core::ffi::c_void, *mut VDS_VOLUME_PROP2) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IVdsVolumeMF, IVdsVolumeMF_Vtbl, 0xee2d5ded_6236_4169_931d_b9778ce03dc6);
impl core::ops::Deref for IVdsVolumeMF {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IVdsVolumeMF, windows_core::IUnknown);
impl IVdsVolumeMF {
    pub unsafe fn GetFileSystemProperties(&self, pfilesystemprop: *mut VDS_FILE_SYSTEM_PROP) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetFileSystemProperties)(windows_core::Interface::as_raw(self), pfilesystemprop).ok()
    }
    pub unsafe fn Format<P0, P1, P2, P3>(&self, r#type: VDS_FILE_SYSTEM_TYPE, pwszlabel: P0, dwunitallocationsize: u32, bforce: P1, bquickformat: P2, benablecompression: P3) -> windows_core::Result<IVdsAsync>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<super::super::Foundation::BOOL>,
        P2: windows_core::Param<super::super::Foundation::BOOL>,
        P3: windows_core::Param<super::super::Foundation::BOOL>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Format)(windows_core::Interface::as_raw(self), r#type, pwszlabel.param().abi(), dwunitallocationsize, bforce.param().abi(), bquickformat.param().abi(), benablecompression.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn AddAccessPath<P0>(&self, pwszpath: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).AddAccessPath)(windows_core::Interface::as_raw(self), pwszpath.param().abi()).ok()
    }
    pub unsafe fn QueryAccessPaths(&self, pwszpatharray: *mut *mut windows_core::PWSTR, plnumberofaccesspaths: *mut i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).QueryAccessPaths)(windows_core::Interface::as_raw(self), pwszpatharray, plnumberofaccesspaths).ok()
    }
    pub unsafe fn QueryReparsePoints(&self, ppreparsepointprops: *mut *mut VDS_REPARSE_POINT_PROP, plnumberofreparsepointprops: *mut i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).QueryReparsePoints)(windows_core::Interface::as_raw(self), ppreparsepointprops, plnumberofreparsepointprops).ok()
    }
    pub unsafe fn DeleteAccessPath<P0, P1>(&self, pwszpath: P0, bforce: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).DeleteAccessPath)(windows_core::Interface::as_raw(self), pwszpath.param().abi(), bforce.param().abi()).ok()
    }
    pub unsafe fn Mount(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Mount)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Dismount<P0, P1>(&self, bforce: P0, bpermanent: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
        P1: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).Dismount)(windows_core::Interface::as_raw(self), bforce.param().abi(), bpermanent.param().abi()).ok()
    }
    pub unsafe fn SetFileSystemFlags(&self, ulflags: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetFileSystemFlags)(windows_core::Interface::as_raw(self), ulflags).ok()
    }
    pub unsafe fn ClearFileSystemFlags(&self, ulflags: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ClearFileSystemFlags)(windows_core::Interface::as_raw(self), ulflags).ok()
    }
}
#[repr(C)]
pub struct IVdsVolumeMF_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetFileSystemProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut VDS_FILE_SYSTEM_PROP) -> windows_core::HRESULT,
    pub Format: unsafe extern "system" fn(*mut core::ffi::c_void, VDS_FILE_SYSTEM_TYPE, windows_core::PCWSTR, u32, super::super::Foundation::BOOL, super::super::Foundation::BOOL, super::super::Foundation::BOOL, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddAccessPath: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub QueryAccessPaths: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut windows_core::PWSTR, *mut i32) -> windows_core::HRESULT,
    pub QueryReparsePoints: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut VDS_REPARSE_POINT_PROP, *mut i32) -> windows_core::HRESULT,
    pub DeleteAccessPath: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub Mount: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Dismount: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::BOOL, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub SetFileSystemFlags: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub ClearFileSystemFlags: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IVdsVolumeMF2, IVdsVolumeMF2_Vtbl, 0x4dbcee9a_6343_4651_b85f_5e75d74d983c);
impl core::ops::Deref for IVdsVolumeMF2 {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IVdsVolumeMF2, windows_core::IUnknown);
impl IVdsVolumeMF2 {
    pub unsafe fn GetFileSystemTypeName(&self) -> windows_core::Result<windows_core::PWSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetFileSystemTypeName)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn QueryFileSystemFormatSupport(&self, ppfilesystemsupportprops: *mut *mut VDS_FILE_SYSTEM_FORMAT_SUPPORT_PROP, plnumberoffilesystems: *mut i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).QueryFileSystemFormatSupport)(windows_core::Interface::as_raw(self), ppfilesystemsupportprops, plnumberoffilesystems).ok()
    }
    pub unsafe fn FormatEx<P0, P1, P2, P3, P4>(&self, pwszfilesystemtypename: P0, usfilesystemrevision: u16, uldesiredunitallocationsize: u32, pwszlabel: P1, bforce: P2, bquickformat: P3, benablecompression: P4) -> windows_core::Result<IVdsAsync>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<super::super::Foundation::BOOL>,
        P3: windows_core::Param<super::super::Foundation::BOOL>,
        P4: windows_core::Param<super::super::Foundation::BOOL>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).FormatEx)(windows_core::Interface::as_raw(self), pwszfilesystemtypename.param().abi(), usfilesystemrevision, uldesiredunitallocationsize, pwszlabel.param().abi(), bforce.param().abi(), bquickformat.param().abi(), benablecompression.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IVdsVolumeMF2_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetFileSystemTypeName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub QueryFileSystemFormatSupport: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut VDS_FILE_SYSTEM_FORMAT_SUPPORT_PROP, *mut i32) -> windows_core::HRESULT,
    pub FormatEx: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u16, u32, windows_core::PCWSTR, super::super::Foundation::BOOL, super::super::Foundation::BOOL, super::super::Foundation::BOOL, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IVdsVolumeMF3, IVdsVolumeMF3_Vtbl, 0x6788faf9_214e_4b85_ba59_266953616e09);
impl core::ops::Deref for IVdsVolumeMF3 {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IVdsVolumeMF3, windows_core::IUnknown);
impl IVdsVolumeMF3 {
    pub unsafe fn QueryVolumeGuidPathnames(&self, pwszpatharray: *mut *mut windows_core::PWSTR, pulnumberofpaths: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).QueryVolumeGuidPathnames)(windows_core::Interface::as_raw(self), pwszpatharray, pulnumberofpaths).ok()
    }
    pub unsafe fn FormatEx2<P0, P1>(&self, pwszfilesystemtypename: P0, usfilesystemrevision: u16, uldesiredunitallocationsize: u32, pwszlabel: P1, options: u32) -> windows_core::Result<IVdsAsync>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).FormatEx2)(windows_core::Interface::as_raw(self), pwszfilesystemtypename.param().abi(), usfilesystemrevision, uldesiredunitallocationsize, pwszlabel.param().abi(), options, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn OfflineVolume(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OfflineVolume)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct IVdsVolumeMF3_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub QueryVolumeGuidPathnames: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut windows_core::PWSTR, *mut u32) -> windows_core::HRESULT,
    pub FormatEx2: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u16, u32, windows_core::PCWSTR, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OfflineVolume: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IVdsVolumeOnline, IVdsVolumeOnline_Vtbl, 0x1be2275a_b315_4f70_9e44_879b3a2a53f2);
impl core::ops::Deref for IVdsVolumeOnline {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IVdsVolumeOnline, windows_core::IUnknown);
impl IVdsVolumeOnline {
    pub unsafe fn Online(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Online)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct IVdsVolumeOnline_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Online: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IVdsVolumePlex, IVdsVolumePlex_Vtbl, 0x4daa0135_e1d1_40f1_aaa5_3cc1e53221c3);
impl core::ops::Deref for IVdsVolumePlex {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IVdsVolumePlex, windows_core::IUnknown);
impl IVdsVolumePlex {
    pub unsafe fn GetProperties(&self, pplexproperties: *mut VDS_VOLUME_PLEX_PROP) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetProperties)(windows_core::Interface::as_raw(self), pplexproperties).ok()
    }
    pub unsafe fn GetVolume(&self) -> windows_core::Result<IVdsVolume> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetVolume)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn QueryExtents(&self, ppextentarray: *mut *mut VDS_DISK_EXTENT, plnumberofextents: *mut i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).QueryExtents)(windows_core::Interface::as_raw(self), ppextentarray, plnumberofextents).ok()
    }
    pub unsafe fn Repair(&self, pinputdiskarray: &[VDS_INPUT_DISK]) -> windows_core::Result<IVdsAsync> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Repair)(windows_core::Interface::as_raw(self), core::mem::transmute(pinputdiskarray.as_ptr()), pinputdiskarray.len().try_into().unwrap(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IVdsVolumePlex_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut VDS_VOLUME_PLEX_PROP) -> windows_core::HRESULT,
    pub GetVolume: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub QueryExtents: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut VDS_DISK_EXTENT, *mut i32) -> windows_core::HRESULT,
    pub Repair: unsafe extern "system" fn(*mut core::ffi::c_void, *const VDS_INPUT_DISK, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IVdsVolumeShrink, IVdsVolumeShrink_Vtbl, 0xd68168c9_82a2_4f85_b6e9_74707c49a58f);
impl core::ops::Deref for IVdsVolumeShrink {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IVdsVolumeShrink, windows_core::IUnknown);
impl IVdsVolumeShrink {
    pub unsafe fn QueryMaxReclaimableBytes(&self) -> windows_core::Result<u64> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).QueryMaxReclaimableBytes)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn Shrink(&self, ulldesirednumberofreclaimablebytes: u64, ullminnumberofreclaimablebytes: u64) -> windows_core::Result<IVdsAsync> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Shrink)(windows_core::Interface::as_raw(self), ulldesirednumberofreclaimablebytes, ullminnumberofreclaimablebytes, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IVdsVolumeShrink_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub QueryMaxReclaimableBytes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u64) -> windows_core::HRESULT,
    pub Shrink: unsafe extern "system" fn(*mut core::ffi::c_void, u64, u64, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub const BeepAlarm: VDS_MAINTENANCE_OPERATION = VDS_MAINTENANCE_OPERATION(2i32);
pub const BlinkLight: VDS_MAINTENANCE_OPERATION = VDS_MAINTENANCE_OPERATION(1i32);
pub const CLSID_VdsLoader: windows_core::GUID = windows_core::GUID::from_u128(0x9c38ed61_d565_4728_aeee_c80952f0ecde);
pub const CLSID_VdsService: windows_core::GUID = windows_core::GUID::from_u128(0x7d1933cb_86f6_4a98_8628_01be94c9a575);
pub const GPT_PARTITION_NAME_LENGTH: u32 = 36u32;
pub const MAX_FS_ALLOWED_CLUSTER_SIZES_SIZE: u32 = 32u32;
pub const MAX_FS_FORMAT_SUPPORT_NAME_SIZE: u32 = 32u32;
pub const MAX_FS_NAME_SIZE: u32 = 8u32;
pub const Ping: VDS_MAINTENANCE_OPERATION = VDS_MAINTENANCE_OPERATION(5i32);
pub const SpinDown: VDS_MAINTENANCE_OPERATION = VDS_MAINTENANCE_OPERATION(3i32);
pub const SpinUp: VDS_MAINTENANCE_OPERATION = VDS_MAINTENANCE_OPERATION(4i32);
pub const VDSBusType1394: VDS_STORAGE_BUS_TYPE = VDS_STORAGE_BUS_TYPE(4i32);
pub const VDSBusTypeAta: VDS_STORAGE_BUS_TYPE = VDS_STORAGE_BUS_TYPE(3i32);
pub const VDSBusTypeAtapi: VDS_STORAGE_BUS_TYPE = VDS_STORAGE_BUS_TYPE(2i32);
pub const VDSBusTypeFibre: VDS_STORAGE_BUS_TYPE = VDS_STORAGE_BUS_TYPE(6i32);
pub const VDSBusTypeFileBackedVirtual: VDS_STORAGE_BUS_TYPE = VDS_STORAGE_BUS_TYPE(15i32);
pub const VDSBusTypeMax: VDS_STORAGE_BUS_TYPE = VDS_STORAGE_BUS_TYPE(14i32);
pub const VDSBusTypeMaxReserved: VDS_STORAGE_BUS_TYPE = VDS_STORAGE_BUS_TYPE(127i32);
pub const VDSBusTypeMmc: VDS_STORAGE_BUS_TYPE = VDS_STORAGE_BUS_TYPE(13i32);
pub const VDSBusTypeNVMe: VDS_STORAGE_BUS_TYPE = VDS_STORAGE_BUS_TYPE(17i32);
pub const VDSBusTypeRAID: VDS_STORAGE_BUS_TYPE = VDS_STORAGE_BUS_TYPE(8i32);
pub const VDSBusTypeSas: VDS_STORAGE_BUS_TYPE = VDS_STORAGE_BUS_TYPE(10i32);
pub const VDSBusTypeSata: VDS_STORAGE_BUS_TYPE = VDS_STORAGE_BUS_TYPE(11i32);
pub const VDSBusTypeScm: VDS_STORAGE_BUS_TYPE = VDS_STORAGE_BUS_TYPE(18i32);
pub const VDSBusTypeScsi: VDS_STORAGE_BUS_TYPE = VDS_STORAGE_BUS_TYPE(1i32);
pub const VDSBusTypeSd: VDS_STORAGE_BUS_TYPE = VDS_STORAGE_BUS_TYPE(12i32);
pub const VDSBusTypeSpaces: VDS_STORAGE_BUS_TYPE = VDS_STORAGE_BUS_TYPE(16i32);
pub const VDSBusTypeSsa: VDS_STORAGE_BUS_TYPE = VDS_STORAGE_BUS_TYPE(5i32);
pub const VDSBusTypeUfs: VDS_STORAGE_BUS_TYPE = VDS_STORAGE_BUS_TYPE(19i32);
pub const VDSBusTypeUnknown: VDS_STORAGE_BUS_TYPE = VDS_STORAGE_BUS_TYPE(0i32);
pub const VDSBusTypeUsb: VDS_STORAGE_BUS_TYPE = VDS_STORAGE_BUS_TYPE(7i32);
pub const VDSBusTypeVirtual: VDS_STORAGE_BUS_TYPE = VDS_STORAGE_BUS_TYPE(14i32);
pub const VDSBusTypeiScsi: VDS_STORAGE_BUS_TYPE = VDS_STORAGE_BUS_TYPE(9i32);
pub const VDSDiskOfflineReasonCollision: VDS_DISK_OFFLINE_REASON = VDS_DISK_OFFLINE_REASON(4i32);
pub const VDSDiskOfflineReasonDIScan: VDS_DISK_OFFLINE_REASON = VDS_DISK_OFFLINE_REASON(7i32);
pub const VDSDiskOfflineReasonLostDataPersistence: VDS_DISK_OFFLINE_REASON = VDS_DISK_OFFLINE_REASON(8i32);
pub const VDSDiskOfflineReasonNone: VDS_DISK_OFFLINE_REASON = VDS_DISK_OFFLINE_REASON(0i32);
pub const VDSDiskOfflineReasonPolicy: VDS_DISK_OFFLINE_REASON = VDS_DISK_OFFLINE_REASON(1i32);
pub const VDSDiskOfflineReasonRedundantPath: VDS_DISK_OFFLINE_REASON = VDS_DISK_OFFLINE_REASON(2i32);
pub const VDSDiskOfflineReasonResourceExhaustion: VDS_DISK_OFFLINE_REASON = VDS_DISK_OFFLINE_REASON(5i32);
pub const VDSDiskOfflineReasonSnapshot: VDS_DISK_OFFLINE_REASON = VDS_DISK_OFFLINE_REASON(3i32);
pub const VDSDiskOfflineReasonWriteFailure: VDS_DISK_OFFLINE_REASON = VDS_DISK_OFFLINE_REASON(6i32);
pub const VDSStorageIdCodeSetAscii: VDS_STORAGE_IDENTIFIER_CODE_SET = VDS_STORAGE_IDENTIFIER_CODE_SET(2i32);
pub const VDSStorageIdCodeSetBinary: VDS_STORAGE_IDENTIFIER_CODE_SET = VDS_STORAGE_IDENTIFIER_CODE_SET(1i32);
pub const VDSStorageIdCodeSetReserved: VDS_STORAGE_IDENTIFIER_CODE_SET = VDS_STORAGE_IDENTIFIER_CODE_SET(0i32);
pub const VDSStorageIdCodeSetUtf8: VDS_STORAGE_IDENTIFIER_CODE_SET = VDS_STORAGE_IDENTIFIER_CODE_SET(3i32);
pub const VDSStorageIdTypeEUI64: VDS_STORAGE_IDENTIFIER_TYPE = VDS_STORAGE_IDENTIFIER_TYPE(2i32);
pub const VDSStorageIdTypeFCPHName: VDS_STORAGE_IDENTIFIER_TYPE = VDS_STORAGE_IDENTIFIER_TYPE(3i32);
pub const VDSStorageIdTypeLogicalUnitGroup: VDS_STORAGE_IDENTIFIER_TYPE = VDS_STORAGE_IDENTIFIER_TYPE(6i32);
pub const VDSStorageIdTypeMD5LogicalUnitIdentifier: VDS_STORAGE_IDENTIFIER_TYPE = VDS_STORAGE_IDENTIFIER_TYPE(7i32);
pub const VDSStorageIdTypePortRelative: VDS_STORAGE_IDENTIFIER_TYPE = VDS_STORAGE_IDENTIFIER_TYPE(4i32);
pub const VDSStorageIdTypeScsiNameString: VDS_STORAGE_IDENTIFIER_TYPE = VDS_STORAGE_IDENTIFIER_TYPE(8i32);
pub const VDSStorageIdTypeTargetPortGroup: VDS_STORAGE_IDENTIFIER_TYPE = VDS_STORAGE_IDENTIFIER_TYPE(5i32);
pub const VDSStorageIdTypeVendorId: VDS_STORAGE_IDENTIFIER_TYPE = VDS_STORAGE_IDENTIFIER_TYPE(1i32);
pub const VDSStorageIdTypeVendorSpecific: VDS_STORAGE_IDENTIFIER_TYPE = VDS_STORAGE_IDENTIFIER_TYPE(0i32);
pub const VDS_ASYNCOUT_ADDLUNPLEX: VDS_ASYNC_OUTPUT_TYPE = VDS_ASYNC_OUTPUT_TYPE(52i32);
pub const VDS_ASYNCOUT_ADDPORTAL: VDS_ASYNC_OUTPUT_TYPE = VDS_ASYNC_OUTPUT_TYPE(65i32);
pub const VDS_ASYNCOUT_ADDVOLUMEPLEX: VDS_ASYNC_OUTPUT_TYPE = VDS_ASYNC_OUTPUT_TYPE(4i32);
pub const VDS_ASYNCOUT_ATTACH_VDISK: VDS_ASYNC_OUTPUT_TYPE = VDS_ASYNC_OUTPUT_TYPE(201i32);
pub const VDS_ASYNCOUT_BREAKVOLUMEPLEX: VDS_ASYNC_OUTPUT_TYPE = VDS_ASYNC_OUTPUT_TYPE(5i32);
pub const VDS_ASYNCOUT_CLEAN: VDS_ASYNC_OUTPUT_TYPE = VDS_ASYNC_OUTPUT_TYPE(11i32);
pub const VDS_ASYNCOUT_COMPACT_VDISK: VDS_ASYNC_OUTPUT_TYPE = VDS_ASYNC_OUTPUT_TYPE(202i32);
pub const VDS_ASYNCOUT_CREATELUN: VDS_ASYNC_OUTPUT_TYPE = VDS_ASYNC_OUTPUT_TYPE(50i32);
pub const VDS_ASYNCOUT_CREATEPARTITION: VDS_ASYNC_OUTPUT_TYPE = VDS_ASYNC_OUTPUT_TYPE(10i32);
pub const VDS_ASYNCOUT_CREATEPORTALGROUP: VDS_ASYNC_OUTPUT_TYPE = VDS_ASYNC_OUTPUT_TYPE(63i32);
pub const VDS_ASYNCOUT_CREATETARGET: VDS_ASYNC_OUTPUT_TYPE = VDS_ASYNC_OUTPUT_TYPE(62i32);
pub const VDS_ASYNCOUT_CREATEVOLUME: VDS_ASYNC_OUTPUT_TYPE = VDS_ASYNC_OUTPUT_TYPE(1i32);
pub const VDS_ASYNCOUT_CREATE_VDISK: VDS_ASYNC_OUTPUT_TYPE = VDS_ASYNC_OUTPUT_TYPE(200i32);
pub const VDS_ASYNCOUT_DELETEPORTALGROUP: VDS_ASYNC_OUTPUT_TYPE = VDS_ASYNC_OUTPUT_TYPE(67i32);
pub const VDS_ASYNCOUT_DELETETARGET: VDS_ASYNC_OUTPUT_TYPE = VDS_ASYNC_OUTPUT_TYPE(64i32);
pub const VDS_ASYNCOUT_EXPAND_VDISK: VDS_ASYNC_OUTPUT_TYPE = VDS_ASYNC_OUTPUT_TYPE(204i32);
pub const VDS_ASYNCOUT_EXTENDLUN: VDS_ASYNC_OUTPUT_TYPE = VDS_ASYNC_OUTPUT_TYPE(54i32);
pub const VDS_ASYNCOUT_EXTENDVOLUME: VDS_ASYNC_OUTPUT_TYPE = VDS_ASYNC_OUTPUT_TYPE(2i32);
pub const VDS_ASYNCOUT_FORMAT: VDS_ASYNC_OUTPUT_TYPE = VDS_ASYNC_OUTPUT_TYPE(101i32);
pub const VDS_ASYNCOUT_LOGINTOTARGET: VDS_ASYNC_OUTPUT_TYPE = VDS_ASYNC_OUTPUT_TYPE(60i32);
pub const VDS_ASYNCOUT_LOGOUTFROMTARGET: VDS_ASYNC_OUTPUT_TYPE = VDS_ASYNC_OUTPUT_TYPE(61i32);
pub const VDS_ASYNCOUT_MERGE_VDISK: VDS_ASYNC_OUTPUT_TYPE = VDS_ASYNC_OUTPUT_TYPE(203i32);
pub const VDS_ASYNCOUT_RECOVERLUN: VDS_ASYNC_OUTPUT_TYPE = VDS_ASYNC_OUTPUT_TYPE(56i32);
pub const VDS_ASYNCOUT_RECOVERPACK: VDS_ASYNC_OUTPUT_TYPE = VDS_ASYNC_OUTPUT_TYPE(8i32);
pub const VDS_ASYNCOUT_REMOVELUNPLEX: VDS_ASYNC_OUTPUT_TYPE = VDS_ASYNC_OUTPUT_TYPE(53i32);
pub const VDS_ASYNCOUT_REMOVEPORTAL: VDS_ASYNC_OUTPUT_TYPE = VDS_ASYNC_OUTPUT_TYPE(66i32);
pub const VDS_ASYNCOUT_REMOVEVOLUMEPLEX: VDS_ASYNC_OUTPUT_TYPE = VDS_ASYNC_OUTPUT_TYPE(6i32);
pub const VDS_ASYNCOUT_REPAIRVOLUMEPLEX: VDS_ASYNC_OUTPUT_TYPE = VDS_ASYNC_OUTPUT_TYPE(7i32);
pub const VDS_ASYNCOUT_REPLACEDISK: VDS_ASYNC_OUTPUT_TYPE = VDS_ASYNC_OUTPUT_TYPE(9i32);
pub const VDS_ASYNCOUT_SHRINKLUN: VDS_ASYNC_OUTPUT_TYPE = VDS_ASYNC_OUTPUT_TYPE(55i32);
pub const VDS_ASYNCOUT_SHRINKVOLUME: VDS_ASYNC_OUTPUT_TYPE = VDS_ASYNC_OUTPUT_TYPE(3i32);
pub const VDS_ASYNCOUT_UNKNOWN: VDS_ASYNC_OUTPUT_TYPE = VDS_ASYNC_OUTPUT_TYPE(0i32);
pub const VDS_ATTACH_VIRTUAL_DISK_FLAG_USE_FILE_ACL: u32 = 1u32;
pub const VDS_CS_FAILED: VDS_CONTROLLER_STATUS = VDS_CONTROLLER_STATUS(5i32);
pub const VDS_CS_NOT_READY: VDS_CONTROLLER_STATUS = VDS_CONTROLLER_STATUS(2i32);
pub const VDS_CS_OFFLINE: VDS_CONTROLLER_STATUS = VDS_CONTROLLER_STATUS(4i32);
pub const VDS_CS_ONLINE: VDS_CONTROLLER_STATUS = VDS_CONTROLLER_STATUS(1i32);
pub const VDS_CS_REMOVED: VDS_CONTROLLER_STATUS = VDS_CONTROLLER_STATUS(8i32);
pub const VDS_CS_UNKNOWN: VDS_CONTROLLER_STATUS = VDS_CONTROLLER_STATUS(0i32);
pub const VDS_DET_CLUSTER: VDS_DISK_EXTENT_TYPE = VDS_DISK_EXTENT_TYPE(7i32);
pub const VDS_DET_DATA: VDS_DISK_EXTENT_TYPE = VDS_DISK_EXTENT_TYPE(2i32);
pub const VDS_DET_ESP: VDS_DISK_EXTENT_TYPE = VDS_DISK_EXTENT_TYPE(4i32);
pub const VDS_DET_FREE: VDS_DISK_EXTENT_TYPE = VDS_DISK_EXTENT_TYPE(1i32);
pub const VDS_DET_LDM: VDS_DISK_EXTENT_TYPE = VDS_DISK_EXTENT_TYPE(6i32);
pub const VDS_DET_MSR: VDS_DISK_EXTENT_TYPE = VDS_DISK_EXTENT_TYPE(5i32);
pub const VDS_DET_OEM: VDS_DISK_EXTENT_TYPE = VDS_DISK_EXTENT_TYPE(3i32);
pub const VDS_DET_UNKNOWN: VDS_DISK_EXTENT_TYPE = VDS_DISK_EXTENT_TYPE(0i32);
pub const VDS_DET_UNUSABLE: VDS_DISK_EXTENT_TYPE = VDS_DISK_EXTENT_TYPE(32767i32);
pub const VDS_DF_AUDIO_CD: VDS_DISK_FLAG = VDS_DISK_FLAG(1i32);
pub const VDS_DF_BOOT_DISK: VDS_DISK_FLAG = VDS_DISK_FLAG(256i32);
pub const VDS_DF_BOOT_FROM_DISK: VDS_DISK_FLAG = VDS_DISK_FLAG(16384i32);
pub const VDS_DF_CLUSTERED: VDS_DISK_FLAG = VDS_DISK_FLAG(32i32);
pub const VDS_DF_CRASHDUMP_DISK: VDS_DISK_FLAG = VDS_DISK_FLAG(2048i32);
pub const VDS_DF_CURRENT_READ_ONLY: VDS_DISK_FLAG = VDS_DISK_FLAG(32768i32);
pub const VDS_DF_DYNAMIC: VDS_DISK_FLAG = VDS_DISK_FLAG(8192i32);
pub const VDS_DF_HAS_ARC_PATH: VDS_DISK_FLAG = VDS_DISK_FLAG(4096i32);
pub const VDS_DF_HIBERNATIONFILE_DISK: VDS_DISK_FLAG = VDS_DISK_FLAG(1024i32);
pub const VDS_DF_HOTSPARE: VDS_DISK_FLAG = VDS_DISK_FLAG(2i32);
pub const VDS_DF_MASKED: VDS_DISK_FLAG = VDS_DISK_FLAG(8i32);
pub const VDS_DF_PAGEFILE_DISK: VDS_DISK_FLAG = VDS_DISK_FLAG(512i32);
pub const VDS_DF_READ_ONLY: VDS_DISK_FLAG = VDS_DISK_FLAG(64i32);
pub const VDS_DF_REFS_NOT_SUPPORTED: VDS_DISK_FLAG = VDS_DISK_FLAG(65536i32);
pub const VDS_DF_RESERVE_CAPABLE: VDS_DISK_FLAG = VDS_DISK_FLAG(4i32);
pub const VDS_DF_STYLE_CONVERTIBLE: VDS_DISK_FLAG = VDS_DISK_FLAG(16i32);
pub const VDS_DF_SYSTEM_DISK: VDS_DISK_FLAG = VDS_DISK_FLAG(128i32);
pub const VDS_DLF_NON_PERSISTENT: VDS_DRIVE_LETTER_FLAG = VDS_DRIVE_LETTER_FLAG(1i32);
pub const VDS_DRF_ASSIGNED: VDS_DRIVE_FLAG = VDS_DRIVE_FLAG(2i32);
pub const VDS_DRF_HOTSPARE: VDS_DRIVE_FLAG = VDS_DRIVE_FLAG(1i32);
pub const VDS_DRF_HOTSPARE_IN_USE: VDS_DRIVE_FLAG = VDS_DRIVE_FLAG(8i32);
pub const VDS_DRF_HOTSPARE_STANDBY: VDS_DRIVE_FLAG = VDS_DRIVE_FLAG(16i32);
pub const VDS_DRF_UNASSIGNED: VDS_DRIVE_FLAG = VDS_DRIVE_FLAG(4i32);
pub const VDS_DRS_FAILED: VDS_DRIVE_STATUS = VDS_DRIVE_STATUS(5i32);
pub const VDS_DRS_NOT_READY: VDS_DRIVE_STATUS = VDS_DRIVE_STATUS(2i32);
pub const VDS_DRS_OFFLINE: VDS_DRIVE_STATUS = VDS_DRIVE_STATUS(4i32);
pub const VDS_DRS_ONLINE: VDS_DRIVE_STATUS = VDS_DRIVE_STATUS(1i32);
pub const VDS_DRS_REMOVED: VDS_DRIVE_STATUS = VDS_DRIVE_STATUS(8i32);
pub const VDS_DRS_UNKNOWN: VDS_DRIVE_STATUS = VDS_DRIVE_STATUS(0i32);
pub const VDS_DS_FAILED: VDS_DISK_STATUS = VDS_DISK_STATUS(5i32);
pub const VDS_DS_MISSING: VDS_DISK_STATUS = VDS_DISK_STATUS(6i32);
pub const VDS_DS_NOT_READY: VDS_DISK_STATUS = VDS_DISK_STATUS(2i32);
pub const VDS_DS_NO_MEDIA: VDS_DISK_STATUS = VDS_DISK_STATUS(3i32);
pub const VDS_DS_OFFLINE: VDS_DISK_STATUS = VDS_DISK_STATUS(4i32);
pub const VDS_DS_ONLINE: VDS_DISK_STATUS = VDS_DISK_STATUS(1i32);
pub const VDS_DS_UNKNOWN: VDS_DISK_STATUS = VDS_DISK_STATUS(0i32);
pub const VDS_E_ACCESS_DENIED: windows_core::HRESULT = windows_core::HRESULT(0x80042427_u32 as _);
pub const VDS_E_ACTIVE_PARTITION: windows_core::HRESULT = windows_core::HRESULT(0x80042438_u32 as _);
pub const VDS_E_ADDRESSES_INCOMPLETELY_SET: windows_core::HRESULT = windows_core::HRESULT(0x80042703_u32 as _);
pub const VDS_E_ALIGN_BEYOND_FIRST_CYLINDER: windows_core::HRESULT = windows_core::HRESULT(0x80042553_u32 as _);
pub const VDS_E_ALIGN_IS_ZERO: windows_core::HRESULT = windows_core::HRESULT(0x80042590_u32 as _);
pub const VDS_E_ALIGN_NOT_A_POWER_OF_TWO: windows_core::HRESULT = windows_core::HRESULT(0x8004258F_u32 as _);
pub const VDS_E_ALIGN_NOT_SECTOR_SIZE_MULTIPLE: windows_core::HRESULT = windows_core::HRESULT(0x80042554_u32 as _);
pub const VDS_E_ALIGN_NOT_ZERO: windows_core::HRESULT = windows_core::HRESULT(0x80042555_u32 as _);
pub const VDS_E_ALREADY_REGISTERED: windows_core::HRESULT = windows_core::HRESULT(0x80042403_u32 as _);
pub const VDS_E_ANOTHER_CALL_IN_PROGRESS: windows_core::HRESULT = windows_core::HRESULT(0x80042404_u32 as _);
pub const VDS_E_ASSOCIATED_LUNS_EXIST: windows_core::HRESULT = windows_core::HRESULT(0x8004270B_u32 as _);
pub const VDS_E_ASSOCIATED_PORTALS_EXIST: windows_core::HRESULT = windows_core::HRESULT(0x8004270C_u32 as _);
pub const VDS_E_ASYNC_OBJECT_FAILURE: windows_core::HRESULT = windows_core::HRESULT(0x8004244E_u32 as _);
pub const VDS_E_BAD_BOOT_DISK: windows_core::HRESULT = windows_core::HRESULT(0x80042586_u32 as _);
pub const VDS_E_BAD_COOKIE: windows_core::HRESULT = windows_core::HRESULT(0x80042411_u32 as _);
pub const VDS_E_BAD_LABEL: windows_core::HRESULT = windows_core::HRESULT(0x80042429_u32 as _);
pub const VDS_E_BAD_PNP_MESSAGE: windows_core::HRESULT = windows_core::HRESULT(0x8004250F_u32 as _);
pub const VDS_E_BAD_PROVIDER_DATA: windows_core::HRESULT = windows_core::HRESULT(0x80042441_u32 as _);
pub const VDS_E_BAD_REVISION_NUMBER: windows_core::HRESULT = windows_core::HRESULT(0x80042598_u32 as _);
pub const VDS_E_BLOCK_CLUSTERED: windows_core::HRESULT = windows_core::HRESULT(0x80042A03_u32 as _);
pub const VDS_E_BOOT_DISK: windows_core::HRESULT = windows_core::HRESULT(0x80042807_u32 as _);
pub const VDS_E_BOOT_PAGEFILE_DRIVE_LETTER: windows_core::HRESULT = windows_core::HRESULT(0x8004290E_u32 as _);
pub const VDS_E_BOOT_PARTITION_NUMBER_CHANGE: windows_core::HRESULT = windows_core::HRESULT(0x80042436_u32 as _);
pub const VDS_E_CACHE_CORRUPT: windows_core::HRESULT = windows_core::HRESULT(0x80042556_u32 as _);
pub const VDS_E_CANCEL_TOO_LATE: windows_core::HRESULT = windows_core::HRESULT(0x8004240C_u32 as _);
pub const VDS_E_CANNOT_CLEAR_VOLUME_FLAG: windows_core::HRESULT = windows_core::HRESULT(0x80042557_u32 as _);
pub const VDS_E_CANNOT_EXTEND: windows_core::HRESULT = windows_core::HRESULT(0x8004240E_u32 as _);
pub const VDS_E_CANNOT_SHRINK: windows_core::HRESULT = windows_core::HRESULT(0x8004251E_u32 as _);
pub const VDS_E_CANT_INVALIDATE_FVE: windows_core::HRESULT = windows_core::HRESULT(0x80042592_u32 as _);
pub const VDS_E_CANT_QUICK_FORMAT: windows_core::HRESULT = windows_core::HRESULT(0x8004242A_u32 as _);
pub const VDS_E_CLEAN_WITH_BOOTBACKING: windows_core::HRESULT = windows_core::HRESULT(0x80042A09_u32 as _);
pub const VDS_E_CLEAN_WITH_CRITICAL: windows_core::HRESULT = windows_core::HRESULT(0x80042912_u32 as _);
pub const VDS_E_CLEAN_WITH_DATA: windows_core::HRESULT = windows_core::HRESULT(0x80042910_u32 as _);
pub const VDS_E_CLEAN_WITH_OEM: windows_core::HRESULT = windows_core::HRESULT(0x80042911_u32 as _);
pub const VDS_E_CLUSTER_COUNT_BEYOND_32BITS: windows_core::HRESULT = windows_core::HRESULT(0x80042430_u32 as _);
pub const VDS_E_CLUSTER_SIZE_TOO_BIG: windows_core::HRESULT = windows_core::HRESULT(0x8004242F_u32 as _);
pub const VDS_E_CLUSTER_SIZE_TOO_SMALL: windows_core::HRESULT = windows_core::HRESULT(0x8004242E_u32 as _);
pub const VDS_E_COMPRESSION_NOT_SUPPORTED: windows_core::HRESULT = windows_core::HRESULT(0x80042918_u32 as _);
pub const VDS_E_CONFIG_LIMIT: windows_core::HRESULT = windows_core::HRESULT(0x80042538_u32 as _);
pub const VDS_E_CORRUPT_EXTENT_INFO: windows_core::HRESULT = windows_core::HRESULT(0x8004250B_u32 as _);
pub const VDS_E_CORRUPT_NOTIFICATION_INFO: windows_core::HRESULT = windows_core::HRESULT(0x8004252A_u32 as _);
pub const VDS_E_CORRUPT_PARTITION_INFO: windows_core::HRESULT = windows_core::HRESULT(0x80042509_u32 as _);
pub const VDS_E_CORRUPT_VOLUME_INFO: windows_core::HRESULT = windows_core::HRESULT(0x80042503_u32 as _);
pub const VDS_E_CRASHDUMP_DISK: windows_core::HRESULT = windows_core::HRESULT(0x8004280E_u32 as _);
pub const VDS_E_CRITICAL_PLEX: windows_core::HRESULT = windows_core::HRESULT(0x8004257E_u32 as _);
pub const VDS_E_DELETE_WITH_BOOTBACKING: windows_core::HRESULT = windows_core::HRESULT(0x80042A07_u32 as _);
pub const VDS_E_DELETE_WITH_CRITICAL: windows_core::HRESULT = windows_core::HRESULT(0x8004290F_u32 as _);
pub const VDS_E_DEVICE_IN_USE: windows_core::HRESULT = windows_core::HRESULT(0x80042413_u32 as _);
pub const VDS_E_DISK_BEING_CLEANED: windows_core::HRESULT = windows_core::HRESULT(0x80042558_u32 as _);
pub const VDS_E_DISK_CONFIGURATION_CORRUPTED: windows_core::HRESULT = windows_core::HRESULT(0x80042539_u32 as _);
pub const VDS_E_DISK_CONFIGURATION_NOT_IN_SYNC: windows_core::HRESULT = windows_core::HRESULT(0x8004253A_u32 as _);
pub const VDS_E_DISK_CONFIGURATION_UPDATE_FAILED: windows_core::HRESULT = windows_core::HRESULT(0x8004253B_u32 as _);
pub const VDS_E_DISK_DYNAMIC: windows_core::HRESULT = windows_core::HRESULT(0x8004253C_u32 as _);
pub const VDS_E_DISK_HAS_BANDS: windows_core::HRESULT = windows_core::HRESULT(0x80042A04_u32 as _);
pub const VDS_E_DISK_IN_USE_BY_VOLUME: windows_core::HRESULT = windows_core::HRESULT(0x8004244C_u32 as _);
pub const VDS_E_DISK_IO_FAILING: windows_core::HRESULT = windows_core::HRESULT(0x80042540_u32 as _);
pub const VDS_E_DISK_IS_OFFLINE: windows_core::HRESULT = windows_core::HRESULT(0x8004280A_u32 as _);
pub const VDS_E_DISK_IS_READ_ONLY: windows_core::HRESULT = windows_core::HRESULT(0x8004280B_u32 as _);
pub const VDS_E_DISK_LAYOUT_PARTITIONS_TOO_SMALL: windows_core::HRESULT = windows_core::HRESULT(0x8004253F_u32 as _);
pub const VDS_E_DISK_NOT_CONVERTIBLE: windows_core::HRESULT = windows_core::HRESULT(0x80042559_u32 as _);
pub const VDS_E_DISK_NOT_CONVERTIBLE_SIZE: windows_core::HRESULT = windows_core::HRESULT(0x80042925_u32 as _);
pub const VDS_E_DISK_NOT_EMPTY: windows_core::HRESULT = windows_core::HRESULT(0x80042414_u32 as _);
pub const VDS_E_DISK_NOT_FOUND_IN_PACK: windows_core::HRESULT = windows_core::HRESULT(0x8004252D_u32 as _);
pub const VDS_E_DISK_NOT_IMPORTED: windows_core::HRESULT = windows_core::HRESULT(0x80042452_u32 as _);
pub const VDS_E_DISK_NOT_INITIALIZED: windows_core::HRESULT = windows_core::HRESULT(0x80042417_u32 as _);
pub const VDS_E_DISK_NOT_LOADED_TO_CACHE: windows_core::HRESULT = windows_core::HRESULT(0x80042447_u32 as _);
pub const VDS_E_DISK_NOT_MISSING: windows_core::HRESULT = windows_core::HRESULT(0x80042501_u32 as _);
pub const VDS_E_DISK_NOT_OFFLINE: windows_core::HRESULT = windows_core::HRESULT(0x80042595_u32 as _);
pub const VDS_E_DISK_NOT_ONLINE: windows_core::HRESULT = windows_core::HRESULT(0x8004244B_u32 as _);
pub const VDS_E_DISK_PNP_REG_CORRUPT: windows_core::HRESULT = windows_core::HRESULT(0x80042455_u32 as _);
pub const VDS_E_DISK_REMOVEABLE: windows_core::HRESULT = windows_core::HRESULT(0x8004255A_u32 as _);
pub const VDS_E_DISK_REMOVEABLE_NOT_EMPTY: windows_core::HRESULT = windows_core::HRESULT(0x8004255B_u32 as _);
pub const VDS_E_DISTINCT_VOLUME: windows_core::HRESULT = windows_core::HRESULT(0x8004257B_u32 as _);
pub const VDS_E_DMADMIN_CORRUPT_NOTIFICATION: windows_core::HRESULT = windows_core::HRESULT(0x80042424_u32 as _);
pub const VDS_E_DMADMIN_METHOD_CALL_FAILED: windows_core::HRESULT = windows_core::HRESULT(0x80042420_u32 as _);
pub const VDS_E_DMADMIN_SERVICE_CONNECTION_FAILED: windows_core::HRESULT = windows_core::HRESULT(0x8004241B_u32 as _);
pub const VDS_E_DRIVER_INTERNAL_ERROR: windows_core::HRESULT = windows_core::HRESULT(0x80042505_u32 as _);
pub const VDS_E_DRIVER_INVALID_PARAM: windows_core::HRESULT = windows_core::HRESULT(0x8004251C_u32 as _);
pub const VDS_E_DRIVER_NO_PACK_NAME: windows_core::HRESULT = windows_core::HRESULT(0x8004250D_u32 as _);
pub const VDS_E_DRIVER_OBJECT_NOT_FOUND: windows_core::HRESULT = windows_core::HRESULT(0x8004253D_u32 as _);
pub const VDS_E_DRIVE_LETTER_NOT_FREE: windows_core::HRESULT = windows_core::HRESULT(0x8004255C_u32 as _);
pub const VDS_E_DUPLICATE_DISK: windows_core::HRESULT = windows_core::HRESULT(0x8004252E_u32 as _);
pub const VDS_E_DUP_EMPTY_PACK_GUID: windows_core::HRESULT = windows_core::HRESULT(0x8004250C_u32 as _);
pub const VDS_E_DYNAMIC_DISKS_NOT_SUPPORTED: windows_core::HRESULT = windows_core::HRESULT(0x80042541_u32 as _);
pub const VDS_E_EXTEND_FILE_SYSTEM_FAILED: windows_core::HRESULT = windows_core::HRESULT(0x80042466_u32 as _);
pub const VDS_E_EXTEND_MULTIPLE_DISKS_NOT_SUPPORTED: windows_core::HRESULT = windows_core::HRESULT(0x8004255D_u32 as _);
pub const VDS_E_EXTEND_TOO_MANY_CLUSTERS: windows_core::HRESULT = windows_core::HRESULT(0x80042928_u32 as _);
pub const VDS_E_EXTEND_UNKNOWN_FILESYSTEM: windows_core::HRESULT = windows_core::HRESULT(0x80042929_u32 as _);
pub const VDS_E_EXTENT_EXCEEDS_DISK_FREE_SPACE: windows_core::HRESULT = windows_core::HRESULT(0x80042515_u32 as _);
pub const VDS_E_EXTENT_SIZE_LESS_THAN_MIN: windows_core::HRESULT = windows_core::HRESULT(0x80042433_u32 as _);
pub const VDS_E_FAILED_TO_OFFLINE_DISK: windows_core::HRESULT = windows_core::HRESULT(0x80042597_u32 as _);
pub const VDS_E_FAILED_TO_ONLINE_DISK: windows_core::HRESULT = windows_core::HRESULT(0x80042596_u32 as _);
pub const VDS_E_FAT32_FORMAT_NOT_SUPPORTED: windows_core::HRESULT = windows_core::HRESULT(0x80042915_u32 as _);
pub const VDS_E_FAT_FORMAT_NOT_SUPPORTED: windows_core::HRESULT = windows_core::HRESULT(0x80042916_u32 as _);
pub const VDS_E_FAULT_TOLERANT_DISKS_NOT_SUPPORTED: windows_core::HRESULT = windows_core::HRESULT(0x80042542_u32 as _);
pub const VDS_E_FLAG_ALREADY_SET: windows_core::HRESULT = windows_core::HRESULT(0x80042579_u32 as _);
pub const VDS_E_FORMAT_CRITICAL: windows_core::HRESULT = windows_core::HRESULT(0x80042913_u32 as _);
pub const VDS_E_FORMAT_NOT_SUPPORTED: windows_core::HRESULT = windows_core::HRESULT(0x80042917_u32 as _);
pub const VDS_E_FORMAT_WITH_BOOTBACKING: windows_core::HRESULT = windows_core::HRESULT(0x80042A08_u32 as _);
pub const VDS_E_FS_NOT_DETERMINED: windows_core::HRESULT = windows_core::HRESULT(0x80042593_u32 as _);
pub const VDS_E_GET_SAN_POLICY: windows_core::HRESULT = windows_core::HRESULT(0x80042805_u32 as _);
pub const VDS_E_GPT_ATTRIBUTES_INVALID: windows_core::HRESULT = windows_core::HRESULT(0x80042543_u32 as _);
pub const VDS_E_HIBERNATION_FILE_DISK: windows_core::HRESULT = windows_core::HRESULT(0x8004280D_u32 as _);
pub const VDS_E_IA64_BOOT_MIRRORED_TO_MBR: windows_core::HRESULT = windows_core::HRESULT(0x8004245A_u32 as _);
pub const VDS_E_IMPORT_SET_INCOMPLETE: windows_core::HRESULT = windows_core::HRESULT(0x80042451_u32 as _);
pub const VDS_E_INCOMPATIBLE_FILE_SYSTEM: windows_core::HRESULT = windows_core::HRESULT(0x80042425_u32 as _);
pub const VDS_E_INCOMPATIBLE_MEDIA: windows_core::HRESULT = windows_core::HRESULT(0x80042426_u32 as _);
pub const VDS_E_INCORRECT_BOOT_VOLUME_EXTENT_INFO: windows_core::HRESULT = windows_core::HRESULT(0x80042804_u32 as _);
pub const VDS_E_INCORRECT_SYSTEM_VOLUME_EXTENT_INFO: windows_core::HRESULT = windows_core::HRESULT(0x80042810_u32 as _);
pub const VDS_E_INITIALIZED_FAILED: windows_core::HRESULT = windows_core::HRESULT(0x80042401_u32 as _);
pub const VDS_E_INITIALIZE_NOT_CALLED: windows_core::HRESULT = windows_core::HRESULT(0x80042402_u32 as _);
pub const VDS_E_INITIATOR_ADAPTER_NOT_FOUND: windows_core::HRESULT = windows_core::HRESULT(0x80042900_u32 as _);
pub const VDS_E_INITIATOR_SPECIFIC_NOT_SUPPORTED: windows_core::HRESULT = windows_core::HRESULT(0x80042707_u32 as _);
pub const VDS_E_INTERNAL_ERROR: windows_core::HRESULT = windows_core::HRESULT(0x80042448_u32 as _);
pub const VDS_E_INVALID_BLOCK_SIZE: windows_core::HRESULT = windows_core::HRESULT(0x80042532_u32 as _);
pub const VDS_E_INVALID_DISK: windows_core::HRESULT = windows_core::HRESULT(0x80042519_u32 as _);
pub const VDS_E_INVALID_DISK_COUNT: windows_core::HRESULT = windows_core::HRESULT(0x80042526_u32 as _);
pub const VDS_E_INVALID_DRIVE_LETTER: windows_core::HRESULT = windows_core::HRESULT(0x8004255E_u32 as _);
pub const VDS_E_INVALID_DRIVE_LETTER_COUNT: windows_core::HRESULT = windows_core::HRESULT(0x8004255F_u32 as _);
pub const VDS_E_INVALID_ENUMERATOR: windows_core::HRESULT = windows_core::HRESULT(0x80042504_u32 as _);
pub const VDS_E_INVALID_EXTENT_COUNT: windows_core::HRESULT = windows_core::HRESULT(0x80042527_u32 as _);
pub const VDS_E_INVALID_FS_FLAG: windows_core::HRESULT = windows_core::HRESULT(0x80042560_u32 as _);
pub const VDS_E_INVALID_FS_TYPE: windows_core::HRESULT = windows_core::HRESULT(0x80042561_u32 as _);
pub const VDS_E_INVALID_IP_ADDRESS: windows_core::HRESULT = windows_core::HRESULT(0x8004290B_u32 as _);
pub const VDS_E_INVALID_ISCSI_PATH: windows_core::HRESULT = windows_core::HRESULT(0x8004291C_u32 as _);
pub const VDS_E_INVALID_ISCSI_TARGET_NAME: windows_core::HRESULT = windows_core::HRESULT(0x80042903_u32 as _);
pub const VDS_E_INVALID_MEMBER_COUNT: windows_core::HRESULT = windows_core::HRESULT(0x80042522_u32 as _);
pub const VDS_E_INVALID_MEMBER_ORDER: windows_core::HRESULT = windows_core::HRESULT(0x80042524_u32 as _);
pub const VDS_E_INVALID_OBJECT_TYPE: windows_core::HRESULT = windows_core::HRESULT(0x80042562_u32 as _);
pub const VDS_E_INVALID_OPERATION: windows_core::HRESULT = windows_core::HRESULT(0x80042415_u32 as _);
pub const VDS_E_INVALID_PACK: windows_core::HRESULT = windows_core::HRESULT(0x8004251A_u32 as _);
pub const VDS_E_INVALID_PARTITION_LAYOUT: windows_core::HRESULT = windows_core::HRESULT(0x80042563_u32 as _);
pub const VDS_E_INVALID_PARTITION_STYLE: windows_core::HRESULT = windows_core::HRESULT(0x80042564_u32 as _);
pub const VDS_E_INVALID_PARTITION_TYPE: windows_core::HRESULT = windows_core::HRESULT(0x80042565_u32 as _);
pub const VDS_E_INVALID_PATH: windows_core::HRESULT = windows_core::HRESULT(0x8004291B_u32 as _);
pub const VDS_E_INVALID_PLEX_BLOCK_SIZE: windows_core::HRESULT = windows_core::HRESULT(0x80042536_u32 as _);
pub const VDS_E_INVALID_PLEX_COUNT: windows_core::HRESULT = windows_core::HRESULT(0x80042521_u32 as _);
pub const VDS_E_INVALID_PLEX_GUID: windows_core::HRESULT = windows_core::HRESULT(0x8004252C_u32 as _);
pub const VDS_E_INVALID_PLEX_ORDER: windows_core::HRESULT = windows_core::HRESULT(0x80042523_u32 as _);
pub const VDS_E_INVALID_PLEX_TYPE: windows_core::HRESULT = windows_core::HRESULT(0x80042535_u32 as _);
pub const VDS_E_INVALID_PORT_PATH: windows_core::HRESULT = windows_core::HRESULT(0x80042902_u32 as _);
pub const VDS_E_INVALID_PROVIDER_CLSID: windows_core::HRESULT = windows_core::HRESULT(0x80042566_u32 as _);
pub const VDS_E_INVALID_PROVIDER_ID: windows_core::HRESULT = windows_core::HRESULT(0x80042567_u32 as _);
pub const VDS_E_INVALID_PROVIDER_NAME: windows_core::HRESULT = windows_core::HRESULT(0x80042568_u32 as _);
pub const VDS_E_INVALID_PROVIDER_TYPE: windows_core::HRESULT = windows_core::HRESULT(0x80042569_u32 as _);
pub const VDS_E_INVALID_PROVIDER_VERSION_GUID: windows_core::HRESULT = windows_core::HRESULT(0x8004256A_u32 as _);
pub const VDS_E_INVALID_PROVIDER_VERSION_STRING: windows_core::HRESULT = windows_core::HRESULT(0x8004256B_u32 as _);
pub const VDS_E_INVALID_QUERY_PROVIDER_FLAG: windows_core::HRESULT = windows_core::HRESULT(0x8004256C_u32 as _);
pub const VDS_E_INVALID_SECTOR_SIZE: windows_core::HRESULT = windows_core::HRESULT(0x80042530_u32 as _);
pub const VDS_E_INVALID_SERVICE_FLAG: windows_core::HRESULT = windows_core::HRESULT(0x8004256D_u32 as _);
pub const VDS_E_INVALID_SHRINK_SIZE: windows_core::HRESULT = windows_core::HRESULT(0x80042817_u32 as _);
pub const VDS_E_INVALID_SPACE: windows_core::HRESULT = windows_core::HRESULT(0x80042406_u32 as _);
pub const VDS_E_INVALID_STATE: windows_core::HRESULT = windows_core::HRESULT(0x80042A05_u32 as _);
pub const VDS_E_INVALID_STRIPE_SIZE: windows_core::HRESULT = windows_core::HRESULT(0x80042525_u32 as _);
pub const VDS_E_INVALID_VOLUME_FLAG: windows_core::HRESULT = windows_core::HRESULT(0x8004256E_u32 as _);
pub const VDS_E_INVALID_VOLUME_LENGTH: windows_core::HRESULT = windows_core::HRESULT(0x8004254E_u32 as _);
pub const VDS_E_INVALID_VOLUME_TYPE: windows_core::HRESULT = windows_core::HRESULT(0x80042585_u32 as _);
pub const VDS_E_IO_ERROR: windows_core::HRESULT = windows_core::HRESULT(0x8004242B_u32 as _);
pub const VDS_E_ISCSI_CHAP_SECRET: windows_core::HRESULT = windows_core::HRESULT(0x8004290A_u32 as _);
pub const VDS_E_ISCSI_GET_IKE_INFO: windows_core::HRESULT = windows_core::HRESULT(0x80042905_u32 as _);
pub const VDS_E_ISCSI_GROUP_PRESHARE_KEY: windows_core::HRESULT = windows_core::HRESULT(0x80042909_u32 as _);
pub const VDS_E_ISCSI_INITIATOR_NODE_NAME: windows_core::HRESULT = windows_core::HRESULT(0x80042908_u32 as _);
pub const VDS_E_ISCSI_LOGIN_FAILED: windows_core::HRESULT = windows_core::HRESULT(0x80042708_u32 as _);
pub const VDS_E_ISCSI_LOGOUT_FAILED: windows_core::HRESULT = windows_core::HRESULT(0x80042709_u32 as _);
pub const VDS_E_ISCSI_LOGOUT_INCOMPLETE: windows_core::HRESULT = windows_core::HRESULT(0x80042710_u32 as _);
pub const VDS_E_ISCSI_SESSION_NOT_FOUND: windows_core::HRESULT = windows_core::HRESULT(0x8004270A_u32 as _);
pub const VDS_E_ISCSI_SET_IKE_INFO: windows_core::HRESULT = windows_core::HRESULT(0x80042906_u32 as _);
pub const VDS_E_LAST_VALID_DISK: windows_core::HRESULT = windows_core::HRESULT(0x8004252F_u32 as _);
pub const VDS_E_LBN_REMAP_ENABLED_FLAG: windows_core::HRESULT = windows_core::HRESULT(0x80042456_u32 as _);
pub const VDS_E_LDM_TIMEOUT: windows_core::HRESULT = windows_core::HRESULT(0x80042461_u32 as _);
pub const VDS_E_LEGACY_VOLUME_FORMAT: windows_core::HRESULT = windows_core::HRESULT(0x8004243A_u32 as _);
pub const VDS_E_LOG_UPDATE: windows_core::HRESULT = windows_core::HRESULT(0x80042587_u32 as _);
pub const VDS_E_LUN_DISK_FAILED: windows_core::HRESULT = windows_core::HRESULT(0x80042819_u32 as _);
pub const VDS_E_LUN_DISK_MISSING: windows_core::HRESULT = windows_core::HRESULT(0x80042818_u32 as _);
pub const VDS_E_LUN_DISK_NOT_READY: windows_core::HRESULT = windows_core::HRESULT(0x8004281A_u32 as _);
pub const VDS_E_LUN_DISK_NO_MEDIA: windows_core::HRESULT = windows_core::HRESULT(0x8004281B_u32 as _);
pub const VDS_E_LUN_DISK_READ_ONLY: windows_core::HRESULT = windows_core::HRESULT(0x8004291E_u32 as _);
pub const VDS_E_LUN_DYNAMIC: windows_core::HRESULT = windows_core::HRESULT(0x80042920_u32 as _);
pub const VDS_E_LUN_DYNAMIC_OFFLINE: windows_core::HRESULT = windows_core::HRESULT(0x80042921_u32 as _);
pub const VDS_E_LUN_FAILED: windows_core::HRESULT = windows_core::HRESULT(0x8004281E_u32 as _);
pub const VDS_E_LUN_NOT_READY: windows_core::HRESULT = windows_core::HRESULT(0x8004281C_u32 as _);
pub const VDS_E_LUN_OFFLINE: windows_core::HRESULT = windows_core::HRESULT(0x8004281D_u32 as _);
pub const VDS_E_LUN_SHRINK_GPT_HEADER: windows_core::HRESULT = windows_core::HRESULT(0x80042922_u32 as _);
pub const VDS_E_LUN_UPDATE_DISK: windows_core::HRESULT = windows_core::HRESULT(0x8004291F_u32 as _);
pub const VDS_E_MAX_USABLE_MBR: windows_core::HRESULT = windows_core::HRESULT(0x80042468_u32 as _);
pub const VDS_E_MEDIA_WRITE_PROTECTED: windows_core::HRESULT = windows_core::HRESULT(0x80042428_u32 as _);
pub const VDS_E_MEMBER_IS_HEALTHY: windows_core::HRESULT = windows_core::HRESULT(0x80042544_u32 as _);
pub const VDS_E_MEMBER_MISSING: windows_core::HRESULT = windows_core::HRESULT(0x8004254A_u32 as _);
pub const VDS_E_MEMBER_REGENERATING: windows_core::HRESULT = windows_core::HRESULT(0x80042545_u32 as _);
pub const VDS_E_MEMBER_SIZE_INVALID: windows_core::HRESULT = windows_core::HRESULT(0x80042516_u32 as _);
pub const VDS_E_MIGRATE_OPEN_VOLUME: windows_core::HRESULT = windows_core::HRESULT(0x8004243C_u32 as _);
pub const VDS_E_MIRROR_NOT_SUPPORTED: windows_core::HRESULT = windows_core::HRESULT(0x80042923_u32 as _);
pub const VDS_E_MISSING_DISK: windows_core::HRESULT = windows_core::HRESULT(0x80042454_u32 as _);
pub const VDS_E_MULTIPLE_DISCOVERY_DOMAINS: windows_core::HRESULT = windows_core::HRESULT(0x8004270E_u32 as _);
pub const VDS_E_MULTIPLE_PACKS: windows_core::HRESULT = windows_core::HRESULT(0x8004251F_u32 as _);
pub const VDS_E_NAME_NOT_UNIQUE: windows_core::HRESULT = windows_core::HRESULT(0x80042701_u32 as _);
pub const VDS_E_NON_CONTIGUOUS_DATA_PARTITIONS: windows_core::HRESULT = windows_core::HRESULT(0x8004243B_u32 as _);
pub const VDS_E_NOT_AN_UNALLOCATED_DISK: windows_core::HRESULT = windows_core::HRESULT(0x80042418_u32 as _);
pub const VDS_E_NOT_ENOUGH_DRIVE: windows_core::HRESULT = windows_core::HRESULT(0x80042410_u32 as _);
pub const VDS_E_NOT_ENOUGH_SPACE: windows_core::HRESULT = windows_core::HRESULT(0x8004240F_u32 as _);
pub const VDS_E_NOT_SUPPORTED: windows_core::HRESULT = windows_core::HRESULT(0x80042400_u32 as _);
pub const VDS_E_NO_DISCOVERY_DOMAIN: windows_core::HRESULT = windows_core::HRESULT(0x8004270D_u32 as _);
pub const VDS_E_NO_DISKS_FOUND: windows_core::HRESULT = windows_core::HRESULT(0x8004241E_u32 as _);
pub const VDS_E_NO_DISK_PATHNAME: windows_core::HRESULT = windows_core::HRESULT(0x8004270F_u32 as _);
pub const VDS_E_NO_DRIVELETTER_FLAG: windows_core::HRESULT = windows_core::HRESULT(0x80042457_u32 as _);
pub const VDS_E_NO_EXTENTS_FOR_PLEX: windows_core::HRESULT = windows_core::HRESULT(0x80042534_u32 as _);
pub const VDS_E_NO_EXTENTS_FOR_VOLUME: windows_core::HRESULT = windows_core::HRESULT(0x80042446_u32 as _);
pub const VDS_E_NO_FREE_SPACE: windows_core::HRESULT = windows_core::HRESULT(0x80042437_u32 as _);
pub const VDS_E_NO_HEALTHY_DISKS: windows_core::HRESULT = windows_core::HRESULT(0x80042537_u32 as _);
pub const VDS_E_NO_IMPORT_TARGET: windows_core::HRESULT = windows_core::HRESULT(0x80042713_u32 as _);
pub const VDS_E_NO_MAINTENANCE_MODE: windows_core::HRESULT = windows_core::HRESULT(0x80042A02_u32 as _);
pub const VDS_E_NO_MEDIA: windows_core::HRESULT = windows_core::HRESULT(0x80042412_u32 as _);
pub const VDS_E_NO_PNP_DISK_ARRIVE: windows_core::HRESULT = windows_core::HRESULT(0x80042510_u32 as _);
pub const VDS_E_NO_PNP_DISK_REMOVE: windows_core::HRESULT = windows_core::HRESULT(0x80042512_u32 as _);
pub const VDS_E_NO_PNP_VOLUME_ARRIVE: windows_core::HRESULT = windows_core::HRESULT(0x80042511_u32 as _);
pub const VDS_E_NO_PNP_VOLUME_REMOVE: windows_core::HRESULT = windows_core::HRESULT(0x80042513_u32 as _);
pub const VDS_E_NO_POOL: windows_core::HRESULT = windows_core::HRESULT(0x80042A00_u32 as _);
pub const VDS_E_NO_POOL_CREATED: windows_core::HRESULT = windows_core::HRESULT(0x80042A01_u32 as _);
pub const VDS_E_NO_SOFTWARE_PROVIDERS_LOADED: windows_core::HRESULT = windows_core::HRESULT(0x80042500_u32 as _);
pub const VDS_E_NO_VALID_LOG_COPIES: windows_core::HRESULT = windows_core::HRESULT(0x8004258A_u32 as _);
pub const VDS_E_NO_VOLUME_LAYOUT: windows_core::HRESULT = windows_core::HRESULT(0x80042502_u32 as _);
pub const VDS_E_NO_VOLUME_PATHNAME: windows_core::HRESULT = windows_core::HRESULT(0x80042711_u32 as _);
pub const VDS_E_NTFS_FORMAT_NOT_SUPPORTED: windows_core::HRESULT = windows_core::HRESULT(0x80042914_u32 as _);
pub const VDS_E_OBJECT_DELETED: windows_core::HRESULT = windows_core::HRESULT(0x8004240B_u32 as _);
pub const VDS_E_OBJECT_EXISTS: windows_core::HRESULT = windows_core::HRESULT(0x8004241D_u32 as _);
pub const VDS_E_OBJECT_NOT_FOUND: windows_core::HRESULT = windows_core::HRESULT(0x80042405_u32 as _);
pub const VDS_E_OBJECT_OUT_OF_SYNC: windows_core::HRESULT = windows_core::HRESULT(0x80042453_u32 as _);
pub const VDS_E_OBJECT_STATUS_FAILED: windows_core::HRESULT = windows_core::HRESULT(0x80042431_u32 as _);
pub const VDS_E_OFFLINE_NOT_SUPPORTED: windows_core::HRESULT = windows_core::HRESULT(0x80042926_u32 as _);
pub const VDS_E_ONE_EXTENT_PER_DISK: windows_core::HRESULT = windows_core::HRESULT(0x80042531_u32 as _);
pub const VDS_E_ONLINE_PACK_EXISTS: windows_core::HRESULT = windows_core::HRESULT(0x80042464_u32 as _);
pub const VDS_E_OPERATION_CANCELED: windows_core::HRESULT = windows_core::HRESULT(0x8004240D_u32 as _);
pub const VDS_E_OPERATION_DENIED: windows_core::HRESULT = windows_core::HRESULT(0x8004240A_u32 as _);
pub const VDS_E_OPERATION_PENDING: windows_core::HRESULT = windows_core::HRESULT(0x80042409_u32 as _);
pub const VDS_E_PACK_NAME_INVALID: windows_core::HRESULT = windows_core::HRESULT(0x80042546_u32 as _);
pub const VDS_E_PACK_NOT_FOUND: windows_core::HRESULT = windows_core::HRESULT(0x80042450_u32 as _);
pub const VDS_E_PACK_OFFLINE: windows_core::HRESULT = windows_core::HRESULT(0x80042444_u32 as _);
pub const VDS_E_PACK_ONLINE: windows_core::HRESULT = windows_core::HRESULT(0x80042520_u32 as _);
pub const VDS_E_PAGEFILE_DISK: windows_core::HRESULT = windows_core::HRESULT(0x8004280C_u32 as _);
pub const VDS_E_PARTITION_LDM: windows_core::HRESULT = windows_core::HRESULT(0x8004258D_u32 as _);
pub const VDS_E_PARTITION_LIMIT_REACHED: windows_core::HRESULT = windows_core::HRESULT(0x80042407_u32 as _);
pub const VDS_E_PARTITION_MSR: windows_core::HRESULT = windows_core::HRESULT(0x8004258C_u32 as _);
pub const VDS_E_PARTITION_NON_DATA: windows_core::HRESULT = windows_core::HRESULT(0x8004257D_u32 as _);
pub const VDS_E_PARTITION_NOT_CYLINDER_ALIGNED: windows_core::HRESULT = windows_core::HRESULT(0x8004253E_u32 as _);
pub const VDS_E_PARTITION_NOT_EMPTY: windows_core::HRESULT = windows_core::HRESULT(0x80042408_u32 as _);
pub const VDS_E_PARTITION_NOT_OEM: windows_core::HRESULT = windows_core::HRESULT(0x8004256F_u32 as _);
pub const VDS_E_PARTITION_OF_UNKNOWN_TYPE: windows_core::HRESULT = windows_core::HRESULT(0x80042439_u32 as _);
pub const VDS_E_PARTITION_PROTECTED: windows_core::HRESULT = windows_core::HRESULT(0x80042570_u32 as _);
pub const VDS_E_PARTITION_STYLE_MISMATCH: windows_core::HRESULT = windows_core::HRESULT(0x80042571_u32 as _);
pub const VDS_E_PATH_NOT_FOUND: windows_core::HRESULT = windows_core::HRESULT(0x80042416_u32 as _);
pub const VDS_E_PLEX_IS_HEALTHY: windows_core::HRESULT = windows_core::HRESULT(0x80042547_u32 as _);
pub const VDS_E_PLEX_LAST_ACTIVE: windows_core::HRESULT = windows_core::HRESULT(0x80042548_u32 as _);
pub const VDS_E_PLEX_MISSING: windows_core::HRESULT = windows_core::HRESULT(0x80042549_u32 as _);
pub const VDS_E_PLEX_NOT_LOADED_TO_CACHE: windows_core::HRESULT = windows_core::HRESULT(0x8004258B_u32 as _);
pub const VDS_E_PLEX_REGENERATING: windows_core::HRESULT = windows_core::HRESULT(0x8004254B_u32 as _);
pub const VDS_E_PLEX_SIZE_INVALID: windows_core::HRESULT = windows_core::HRESULT(0x80042533_u32 as _);
pub const VDS_E_PROVIDER_CACHE_CORRUPT: windows_core::HRESULT = windows_core::HRESULT(0x8004241F_u32 as _);
pub const VDS_E_PROVIDER_CACHE_OUTOFSYNC: windows_core::HRESULT = windows_core::HRESULT(0x80042712_u32 as _);
pub const VDS_E_PROVIDER_EXITING: windows_core::HRESULT = windows_core::HRESULT(0x80042514_u32 as _);
pub const VDS_E_PROVIDER_FAILURE: windows_core::HRESULT = windows_core::HRESULT(0x80042442_u32 as _);
pub const VDS_E_PROVIDER_INITIALIZATION_FAILED: windows_core::HRESULT = windows_core::HRESULT(0x8004241C_u32 as _);
pub const VDS_E_PROVIDER_INTERNAL_ERROR: windows_core::HRESULT = windows_core::HRESULT(0x80042572_u32 as _);
pub const VDS_E_PROVIDER_TYPE_NOT_SUPPORTED: windows_core::HRESULT = windows_core::HRESULT(0x8004244A_u32 as _);
pub const VDS_E_PROVIDER_VOL_DEVICE_NAME_NOT_FOUND: windows_core::HRESULT = windows_core::HRESULT(0x80042422_u32 as _);
pub const VDS_E_PROVIDER_VOL_OPEN: windows_core::HRESULT = windows_core::HRESULT(0x80042423_u32 as _);
pub const VDS_E_RAID5_NOT_SUPPORTED: windows_core::HRESULT = windows_core::HRESULT(0x80042924_u32 as _);
pub const VDS_E_READONLY: windows_core::HRESULT = windows_core::HRESULT(0x80042584_u32 as _);
pub const VDS_E_REBOOT_REQUIRED: windows_core::HRESULT = windows_core::HRESULT(0x8004290C_u32 as _);
pub const VDS_E_REFS_FORMAT_NOT_SUPPORTED: windows_core::HRESULT = windows_core::HRESULT(0x80042A06_u32 as _);
pub const VDS_E_REPAIR_VOLUMESTATE: windows_core::HRESULT = windows_core::HRESULT(0x80042460_u32 as _);
pub const VDS_E_REQUIRES_CONTIGUOUS_DISK_SPACE: windows_core::HRESULT = windows_core::HRESULT(0x80042440_u32 as _);
pub const VDS_E_RETRY: windows_core::HRESULT = windows_core::HRESULT(0x80042463_u32 as _);
pub const VDS_E_REVERT_ON_CLOSE: windows_core::HRESULT = windows_core::HRESULT(0x80042458_u32 as _);
pub const VDS_E_REVERT_ON_CLOSE_MISMATCH: windows_core::HRESULT = windows_core::HRESULT(0x80042462_u32 as _);
pub const VDS_E_REVERT_ON_CLOSE_SET: windows_core::HRESULT = windows_core::HRESULT(0x80042459_u32 as _);
pub const VDS_E_SECTOR_SIZE_ERROR: windows_core::HRESULT = windows_core::HRESULT(0x80042823_u32 as _);
pub const VDS_E_SECURITY_INCOMPLETELY_SET: windows_core::HRESULT = windows_core::HRESULT(0x80042705_u32 as _);
pub const VDS_E_SET_SAN_POLICY: windows_core::HRESULT = windows_core::HRESULT(0x80042806_u32 as _);
pub const VDS_E_SET_TUNNEL_MODE_OUTER_ADDRESS: windows_core::HRESULT = windows_core::HRESULT(0x80042904_u32 as _);
pub const VDS_E_SHRINK_DIRTY_VOLUME: windows_core::HRESULT = windows_core::HRESULT(0x8004259A_u32 as _);
pub const VDS_E_SHRINK_EXTEND_UNALIGNED: windows_core::HRESULT = windows_core::HRESULT(0x80042B00_u32 as _);
pub const VDS_E_SHRINK_IN_PROGRESS: windows_core::HRESULT = windows_core::HRESULT(0x80042591_u32 as _);
pub const VDS_E_SHRINK_LUN_NOT_UNMASKED: windows_core::HRESULT = windows_core::HRESULT(0x8004291D_u32 as _);
pub const VDS_E_SHRINK_OVER_DATA: windows_core::HRESULT = windows_core::HRESULT(0x80042816_u32 as _);
pub const VDS_E_SHRINK_SIZE_LESS_THAN_MIN: windows_core::HRESULT = windows_core::HRESULT(0x80042573_u32 as _);
pub const VDS_E_SHRINK_SIZE_TOO_BIG: windows_core::HRESULT = windows_core::HRESULT(0x80042574_u32 as _);
pub const VDS_E_SHRINK_UNKNOWN_FILESYSTEM: windows_core::HRESULT = windows_core::HRESULT(0x8004292A_u32 as _);
pub const VDS_E_SHRINK_USER_CANCELLED: windows_core::HRESULT = windows_core::HRESULT(0x80042599_u32 as _);
pub const VDS_E_SOURCE_IS_TARGET_PACK: windows_core::HRESULT = windows_core::HRESULT(0x80042528_u32 as _);
pub const VDS_E_SUBSYSTEM_ID_IS_NULL: windows_core::HRESULT = windows_core::HRESULT(0x80042907_u32 as _);
pub const VDS_E_SYSTEM_DISK: windows_core::HRESULT = windows_core::HRESULT(0x80042811_u32 as _);
pub const VDS_E_TARGET_PACK_NOT_EMPTY: windows_core::HRESULT = windows_core::HRESULT(0x8004251D_u32 as _);
pub const VDS_E_TARGET_PORTAL_NOT_FOUND: windows_core::HRESULT = windows_core::HRESULT(0x80042901_u32 as _);
pub const VDS_E_TARGET_SPECIFIC_NOT_SUPPORTED: windows_core::HRESULT = windows_core::HRESULT(0x80042706_u32 as _);
pub const VDS_E_TIMEOUT: windows_core::HRESULT = windows_core::HRESULT(0x8004245F_u32 as _);
pub const VDS_E_UNABLE_TO_FIND_BOOT_DISK: windows_core::HRESULT = windows_core::HRESULT(0x80042803_u32 as _);
pub const VDS_E_UNABLE_TO_FIND_SYSTEM_DISK: windows_core::HRESULT = windows_core::HRESULT(0x8004280F_u32 as _);
pub const VDS_E_UNEXPECTED_DISK_LAYOUT_CHANGE: windows_core::HRESULT = windows_core::HRESULT(0x8004254D_u32 as _);
pub const VDS_E_UNRECOVERABLE_ERROR: windows_core::HRESULT = windows_core::HRESULT(0x80042419_u32 as _);
pub const VDS_E_UNRECOVERABLE_PROVIDER_ERROR: windows_core::HRESULT = windows_core::HRESULT(0x80042575_u32 as _);
pub const VDS_E_VDISK_INVALID_OP_STATE: windows_core::HRESULT = windows_core::HRESULT(0x8004291A_u32 as _);
pub const VDS_E_VDISK_NOT_OPEN: windows_core::HRESULT = windows_core::HRESULT(0x80042919_u32 as _);
pub const VDS_E_VDISK_PATHNAME_INVALID: windows_core::HRESULT = windows_core::HRESULT(0x80042927_u32 as _);
pub const VDS_E_VD_ALREADY_ATTACHED: windows_core::HRESULT = windows_core::HRESULT(0x80042934_u32 as _);
pub const VDS_E_VD_ALREADY_COMPACTING: windows_core::HRESULT = windows_core::HRESULT(0x80042932_u32 as _);
pub const VDS_E_VD_ALREADY_DETACHED: windows_core::HRESULT = windows_core::HRESULT(0x80042935_u32 as _);
pub const VDS_E_VD_ALREADY_MERGING: windows_core::HRESULT = windows_core::HRESULT(0x80042933_u32 as _);
pub const VDS_E_VD_DISK_ALREADY_EXPANDING: windows_core::HRESULT = windows_core::HRESULT(0x80042931_u32 as _);
pub const VDS_E_VD_DISK_ALREADY_OPEN: windows_core::HRESULT = windows_core::HRESULT(0x80042930_u32 as _);
pub const VDS_E_VD_DISK_IS_COMPACTING: windows_core::HRESULT = windows_core::HRESULT(0x8004292D_u32 as _);
pub const VDS_E_VD_DISK_IS_EXPANDING: windows_core::HRESULT = windows_core::HRESULT(0x8004292C_u32 as _);
pub const VDS_E_VD_DISK_IS_MERGING: windows_core::HRESULT = windows_core::HRESULT(0x8004292E_u32 as _);
pub const VDS_E_VD_DISK_NOT_OPEN: windows_core::HRESULT = windows_core::HRESULT(0x8004292B_u32 as _);
pub const VDS_E_VD_IS_ATTACHED: windows_core::HRESULT = windows_core::HRESULT(0x8004292F_u32 as _);
pub const VDS_E_VD_IS_BEING_ATTACHED: windows_core::HRESULT = windows_core::HRESULT(0x80042937_u32 as _);
pub const VDS_E_VD_IS_BEING_DETACHED: windows_core::HRESULT = windows_core::HRESULT(0x80042938_u32 as _);
pub const VDS_E_VD_NOT_ATTACHED_READONLY: windows_core::HRESULT = windows_core::HRESULT(0x80042936_u32 as _);
pub const VDS_E_VOLUME_DISK_COUNT_MAX_EXCEEDED: windows_core::HRESULT = windows_core::HRESULT(0x80042529_u32 as _);
pub const VDS_E_VOLUME_EXTEND_FVE: windows_core::HRESULT = windows_core::HRESULT(0x80042822_u32 as _);
pub const VDS_E_VOLUME_EXTEND_FVE_CORRUPT: windows_core::HRESULT = windows_core::HRESULT(0x80042820_u32 as _);
pub const VDS_E_VOLUME_EXTEND_FVE_LOCKED: windows_core::HRESULT = windows_core::HRESULT(0x8004281F_u32 as _);
pub const VDS_E_VOLUME_EXTEND_FVE_RECOVERY: windows_core::HRESULT = windows_core::HRESULT(0x80042821_u32 as _);
pub const VDS_E_VOLUME_GUID_PATHNAME_NOT_ALLOWED: windows_core::HRESULT = windows_core::HRESULT(0x8004290D_u32 as _);
pub const VDS_E_VOLUME_HAS_PATH: windows_core::HRESULT = windows_core::HRESULT(0x8004245E_u32 as _);
pub const VDS_E_VOLUME_HIDDEN: windows_core::HRESULT = windows_core::HRESULT(0x80042576_u32 as _);
pub const VDS_E_VOLUME_INCOMPLETE: windows_core::HRESULT = windows_core::HRESULT(0x80042432_u32 as _);
pub const VDS_E_VOLUME_INVALID_NAME: windows_core::HRESULT = windows_core::HRESULT(0x80042507_u32 as _);
pub const VDS_E_VOLUME_LENGTH_NOT_SECTOR_SIZE_MULTIPLE: windows_core::HRESULT = windows_core::HRESULT(0x8004254F_u32 as _);
pub const VDS_E_VOLUME_MIRRORED: windows_core::HRESULT = windows_core::HRESULT(0x80042588_u32 as _);
pub const VDS_E_VOLUME_NOT_A_MIRROR: windows_core::HRESULT = windows_core::HRESULT(0x80042445_u32 as _);
pub const VDS_E_VOLUME_NOT_FOUND_IN_PACK: windows_core::HRESULT = windows_core::HRESULT(0x8004257C_u32 as _);
pub const VDS_E_VOLUME_NOT_HEALTHY: windows_core::HRESULT = windows_core::HRESULT(0x8004243E_u32 as _);
pub const VDS_E_VOLUME_NOT_MOUNTED: windows_core::HRESULT = windows_core::HRESULT(0x8004244F_u32 as _);
pub const VDS_E_VOLUME_NOT_ONLINE: windows_core::HRESULT = windows_core::HRESULT(0x8004243D_u32 as _);
pub const VDS_E_VOLUME_NOT_RETAINED: windows_core::HRESULT = windows_core::HRESULT(0x80042550_u32 as _);
pub const VDS_E_VOLUME_ON_DISK: windows_core::HRESULT = windows_core::HRESULT(0x8004251B_u32 as _);
pub const VDS_E_VOLUME_PERMANENTLY_DISMOUNTED: windows_core::HRESULT = windows_core::HRESULT(0x8004245D_u32 as _);
pub const VDS_E_VOLUME_REGENERATING: windows_core::HRESULT = windows_core::HRESULT(0x80042580_u32 as _);
pub const VDS_E_VOLUME_RETAINED: windows_core::HRESULT = windows_core::HRESULT(0x80042551_u32 as _);
pub const VDS_E_VOLUME_SHRINK_FVE: windows_core::HRESULT = windows_core::HRESULT(0x80042815_u32 as _);
pub const VDS_E_VOLUME_SHRINK_FVE_CORRUPT: windows_core::HRESULT = windows_core::HRESULT(0x80042813_u32 as _);
pub const VDS_E_VOLUME_SHRINK_FVE_LOCKED: windows_core::HRESULT = windows_core::HRESULT(0x80042812_u32 as _);
pub const VDS_E_VOLUME_SHRINK_FVE_RECOVERY: windows_core::HRESULT = windows_core::HRESULT(0x80042814_u32 as _);
pub const VDS_E_VOLUME_SIMPLE_SPANNED: windows_core::HRESULT = windows_core::HRESULT(0x80042589_u32 as _);
pub const VDS_E_VOLUME_SPANS_DISKS: windows_core::HRESULT = windows_core::HRESULT(0x8004243F_u32 as _);
pub const VDS_E_VOLUME_SYNCHRONIZING: windows_core::HRESULT = windows_core::HRESULT(0x8004257F_u32 as _);
pub const VDS_E_VOLUME_TEMPORARILY_DISMOUNTED: windows_core::HRESULT = windows_core::HRESULT(0x8004245C_u32 as _);
pub const VDS_E_VOLUME_TOO_BIG: windows_core::HRESULT = windows_core::HRESULT(0x8004242D_u32 as _);
pub const VDS_E_VOLUME_TOO_SMALL: windows_core::HRESULT = windows_core::HRESULT(0x8004242C_u32 as _);
pub const VDS_FPF_COMPRESSED: VDS_FILE_SYSTEM_PROP_FLAG = VDS_FILE_SYSTEM_PROP_FLAG(1i32);
pub const VDS_FSF_ALLOCATION_UNIT_128K: VDS_FILE_SYSTEM_FLAG = VDS_FILE_SYSTEM_FLAG(16777216i32);
pub const VDS_FSF_ALLOCATION_UNIT_16K: VDS_FILE_SYSTEM_FLAG = VDS_FILE_SYSTEM_FLAG(2097152i32);
pub const VDS_FSF_ALLOCATION_UNIT_1K: VDS_FILE_SYSTEM_FLAG = VDS_FILE_SYSTEM_FLAG(131072i32);
pub const VDS_FSF_ALLOCATION_UNIT_256K: VDS_FILE_SYSTEM_FLAG = VDS_FILE_SYSTEM_FLAG(33554432i32);
pub const VDS_FSF_ALLOCATION_UNIT_2K: VDS_FILE_SYSTEM_FLAG = VDS_FILE_SYSTEM_FLAG(262144i32);
pub const VDS_FSF_ALLOCATION_UNIT_32K: VDS_FILE_SYSTEM_FLAG = VDS_FILE_SYSTEM_FLAG(4194304i32);
pub const VDS_FSF_ALLOCATION_UNIT_4K: VDS_FILE_SYSTEM_FLAG = VDS_FILE_SYSTEM_FLAG(524288i32);
pub const VDS_FSF_ALLOCATION_UNIT_512: VDS_FILE_SYSTEM_FLAG = VDS_FILE_SYSTEM_FLAG(65536i32);
pub const VDS_FSF_ALLOCATION_UNIT_64K: VDS_FILE_SYSTEM_FLAG = VDS_FILE_SYSTEM_FLAG(8388608i32);
pub const VDS_FSF_ALLOCATION_UNIT_8K: VDS_FILE_SYSTEM_FLAG = VDS_FILE_SYSTEM_FLAG(1048576i32);
pub const VDS_FSF_SUPPORT_COMPRESS: VDS_FILE_SYSTEM_FLAG = VDS_FILE_SYSTEM_FLAG(4i32);
pub const VDS_FSF_SUPPORT_EXTEND: VDS_FILE_SYSTEM_FLAG = VDS_FILE_SYSTEM_FLAG(64i32);
pub const VDS_FSF_SUPPORT_FORMAT: VDS_FILE_SYSTEM_FLAG = VDS_FILE_SYSTEM_FLAG(1i32);
pub const VDS_FSF_SUPPORT_MOUNT_POINT: VDS_FILE_SYSTEM_FLAG = VDS_FILE_SYSTEM_FLAG(16i32);
pub const VDS_FSF_SUPPORT_QUICK_FORMAT: VDS_FILE_SYSTEM_FLAG = VDS_FILE_SYSTEM_FLAG(2i32);
pub const VDS_FSF_SUPPORT_REMOVABLE_MEDIA: VDS_FILE_SYSTEM_FLAG = VDS_FILE_SYSTEM_FLAG(32i32);
pub const VDS_FSF_SUPPORT_SPECIFY_LABEL: VDS_FILE_SYSTEM_FLAG = VDS_FILE_SYSTEM_FLAG(8i32);
pub const VDS_FSOF_COMPRESSION: VDS_FORMAT_OPTION_FLAGS = VDS_FORMAT_OPTION_FLAGS(4i32);
pub const VDS_FSOF_DUPLICATE_METADATA: VDS_FORMAT_OPTION_FLAGS = VDS_FORMAT_OPTION_FLAGS(8i32);
pub const VDS_FSOF_FORCE: VDS_FORMAT_OPTION_FLAGS = VDS_FORMAT_OPTION_FLAGS(1i32);
pub const VDS_FSOF_NONE: VDS_FORMAT_OPTION_FLAGS = VDS_FORMAT_OPTION_FLAGS(0i32);
pub const VDS_FSOF_QUICK: VDS_FORMAT_OPTION_FLAGS = VDS_FORMAT_OPTION_FLAGS(2i32);
pub const VDS_FSS_DEFAULT: VDS_FILE_SYSTEM_FORMAT_SUPPORT_FLAG = VDS_FILE_SYSTEM_FORMAT_SUPPORT_FLAG(1i32);
pub const VDS_FSS_PREVIOUS_REVISION: VDS_FILE_SYSTEM_FORMAT_SUPPORT_FLAG = VDS_FILE_SYSTEM_FORMAT_SUPPORT_FLAG(2i32);
pub const VDS_FSS_RECOMMENDED: VDS_FILE_SYSTEM_FORMAT_SUPPORT_FLAG = VDS_FILE_SYSTEM_FORMAT_SUPPORT_FLAG(4i32);
pub const VDS_FST_CDFS: VDS_FILE_SYSTEM_TYPE = VDS_FILE_SYSTEM_TYPE(5i32);
pub const VDS_FST_CSVFS: VDS_FILE_SYSTEM_TYPE = VDS_FILE_SYSTEM_TYPE(8i32);
pub const VDS_FST_EXFAT: VDS_FILE_SYSTEM_TYPE = VDS_FILE_SYSTEM_TYPE(7i32);
pub const VDS_FST_FAT: VDS_FILE_SYSTEM_TYPE = VDS_FILE_SYSTEM_TYPE(2i32);
pub const VDS_FST_FAT32: VDS_FILE_SYSTEM_TYPE = VDS_FILE_SYSTEM_TYPE(3i32);
pub const VDS_FST_NTFS: VDS_FILE_SYSTEM_TYPE = VDS_FILE_SYSTEM_TYPE(4i32);
pub const VDS_FST_RAW: VDS_FILE_SYSTEM_TYPE = VDS_FILE_SYSTEM_TYPE(1i32);
pub const VDS_FST_REFS: VDS_FILE_SYSTEM_TYPE = VDS_FILE_SYSTEM_TYPE(9i32);
pub const VDS_FST_UDF: VDS_FILE_SYSTEM_TYPE = VDS_FILE_SYSTEM_TYPE(6i32);
pub const VDS_FST_UNKNOWN: VDS_FILE_SYSTEM_TYPE = VDS_FILE_SYSTEM_TYPE(0i32);
pub const VDS_HINT_ALLOCATEHOTSPARE: i32 = 512i32;
pub const VDS_HINT_BUSTYPE: i32 = 1024i32;
pub const VDS_HINT_CONSISTENCYCHECKENABLED: i32 = 32768i32;
pub const VDS_HINT_FASTCRASHRECOVERYREQUIRED: i32 = 1i32;
pub const VDS_HINT_HARDWARECHECKSUMENABLED: i32 = 128i32;
pub const VDS_HINT_ISYANKABLE: i32 = 256i32;
pub const VDS_HINT_MEDIASCANENABLED: i32 = 16384i32;
pub const VDS_HINT_MOSTLYREADS: i32 = 2i32;
pub const VDS_HINT_OPTIMIZEFORSEQUENTIALREADS: i32 = 4i32;
pub const VDS_HINT_OPTIMIZEFORSEQUENTIALWRITES: i32 = 8i32;
pub const VDS_HINT_READBACKVERIFYENABLED: i32 = 16i32;
pub const VDS_HINT_READCACHINGENABLED: i32 = 4096i32;
pub const VDS_HINT_REMAPENABLED: i32 = 32i32;
pub const VDS_HINT_USEMIRROREDCACHE: i32 = 2048i32;
pub const VDS_HINT_WRITECACHINGENABLED: i32 = 8192i32;
pub const VDS_HINT_WRITETHROUGHCACHINGENABLED: i32 = 64i32;
pub const VDS_HPS_BYPASSED: VDS_HBAPORT_STATUS = VDS_HBAPORT_STATUS(4i32);
pub const VDS_HPS_DIAGNOSTICS: VDS_HBAPORT_STATUS = VDS_HBAPORT_STATUS(5i32);
pub const VDS_HPS_ERROR: VDS_HBAPORT_STATUS = VDS_HBAPORT_STATUS(7i32);
pub const VDS_HPS_LINKDOWN: VDS_HBAPORT_STATUS = VDS_HBAPORT_STATUS(6i32);
pub const VDS_HPS_LOOPBACK: VDS_HBAPORT_STATUS = VDS_HBAPORT_STATUS(8i32);
pub const VDS_HPS_OFFLINE: VDS_HBAPORT_STATUS = VDS_HBAPORT_STATUS(3i32);
pub const VDS_HPS_ONLINE: VDS_HBAPORT_STATUS = VDS_HBAPORT_STATUS(2i32);
pub const VDS_HPS_UNKNOWN: VDS_HBAPORT_STATUS = VDS_HBAPORT_STATUS(1i32);
pub const VDS_HPT_EPORT: VDS_HBAPORT_TYPE = VDS_HBAPORT_TYPE(9i32);
pub const VDS_HPT_FLPORT: VDS_HBAPORT_TYPE = VDS_HBAPORT_TYPE(7i32);
pub const VDS_HPT_FPORT: VDS_HBAPORT_TYPE = VDS_HBAPORT_TYPE(8i32);
pub const VDS_HPT_GPORT: VDS_HBAPORT_TYPE = VDS_HBAPORT_TYPE(10i32);
pub const VDS_HPT_LPORT: VDS_HBAPORT_TYPE = VDS_HBAPORT_TYPE(20i32);
pub const VDS_HPT_NLPORT: VDS_HBAPORT_TYPE = VDS_HBAPORT_TYPE(6i32);
pub const VDS_HPT_NOTPRESENT: VDS_HBAPORT_TYPE = VDS_HBAPORT_TYPE(3i32);
pub const VDS_HPT_NPORT: VDS_HBAPORT_TYPE = VDS_HBAPORT_TYPE(5i32);
pub const VDS_HPT_OTHER: VDS_HBAPORT_TYPE = VDS_HBAPORT_TYPE(2i32);
pub const VDS_HPT_PTP: VDS_HBAPORT_TYPE = VDS_HBAPORT_TYPE(21i32);
pub const VDS_HPT_UNKNOWN: VDS_HBAPORT_TYPE = VDS_HBAPORT_TYPE(1i32);
pub const VDS_HSF_10GBIT: VDS_HBAPORT_SPEED_FLAG = VDS_HBAPORT_SPEED_FLAG(4i32);
pub const VDS_HSF_1GBIT: VDS_HBAPORT_SPEED_FLAG = VDS_HBAPORT_SPEED_FLAG(1i32);
pub const VDS_HSF_2GBIT: VDS_HBAPORT_SPEED_FLAG = VDS_HBAPORT_SPEED_FLAG(2i32);
pub const VDS_HSF_4GBIT: VDS_HBAPORT_SPEED_FLAG = VDS_HBAPORT_SPEED_FLAG(8i32);
pub const VDS_HSF_NOT_NEGOTIATED: VDS_HBAPORT_SPEED_FLAG = VDS_HBAPORT_SPEED_FLAG(32768i32);
pub const VDS_HSF_UNKNOWN: VDS_HBAPORT_SPEED_FLAG = VDS_HBAPORT_SPEED_FLAG(0i32);
pub const VDS_HWT_FIBRE_CHANNEL: VDS_HWPROVIDER_TYPE = VDS_HWPROVIDER_TYPE(2i32);
pub const VDS_HWT_HYBRID: VDS_HWPROVIDER_TYPE = VDS_HWPROVIDER_TYPE(5i32);
pub const VDS_HWT_ISCSI: VDS_HWPROVIDER_TYPE = VDS_HWPROVIDER_TYPE(3i32);
pub const VDS_HWT_PCI_RAID: VDS_HWPROVIDER_TYPE = VDS_HWPROVIDER_TYPE(1i32);
pub const VDS_HWT_SAS: VDS_HWPROVIDER_TYPE = VDS_HWPROVIDER_TYPE(4i32);
pub const VDS_HWT_UNKNOWN: VDS_HWPROVIDER_TYPE = VDS_HWPROVIDER_TYPE(0i32);
pub const VDS_H_DEGRADED: VDS_HEALTH = VDS_HEALTH(11i32);
pub const VDS_H_FAILED: VDS_HEALTH = VDS_HEALTH(8i32);
pub const VDS_H_FAILED_REDUNDANCY: VDS_HEALTH = VDS_HEALTH(6i32);
pub const VDS_H_FAILED_REDUNDANCY_FAILING: VDS_HEALTH = VDS_HEALTH(7i32);
pub const VDS_H_FAILING: VDS_HEALTH = VDS_HEALTH(4i32);
pub const VDS_H_FAILING_REDUNDANCY: VDS_HEALTH = VDS_HEALTH(5i32);
pub const VDS_H_HEALTHY: VDS_HEALTH = VDS_HEALTH(1i32);
pub const VDS_H_PENDING_FAILURE: VDS_HEALTH = VDS_HEALTH(10i32);
pub const VDS_H_REBUILDING: VDS_HEALTH = VDS_HEALTH(2i32);
pub const VDS_H_REPLACED: VDS_HEALTH = VDS_HEALTH(9i32);
pub const VDS_H_STALE: VDS_HEALTH = VDS_HEALTH(3i32);
pub const VDS_H_UNKNOWN: VDS_HEALTH = VDS_HEALTH(0i32);
pub const VDS_IAT_CHAP: VDS_ISCSI_AUTH_TYPE = VDS_ISCSI_AUTH_TYPE(1i32);
pub const VDS_IAT_MUTUAL_CHAP: VDS_ISCSI_AUTH_TYPE = VDS_ISCSI_AUTH_TYPE(2i32);
pub const VDS_IAT_NONE: VDS_ISCSI_AUTH_TYPE = VDS_ISCSI_AUTH_TYPE(0i32);
pub const VDS_IA_FCFS: VDS_INTERCONNECT_ADDRESS_TYPE = VDS_INTERCONNECT_ADDRESS_TYPE(1i32);
pub const VDS_IA_FCPH: VDS_INTERCONNECT_ADDRESS_TYPE = VDS_INTERCONNECT_ADDRESS_TYPE(2i32);
pub const VDS_IA_FCPH3: VDS_INTERCONNECT_ADDRESS_TYPE = VDS_INTERCONNECT_ADDRESS_TYPE(3i32);
pub const VDS_IA_MAC: VDS_INTERCONNECT_ADDRESS_TYPE = VDS_INTERCONNECT_ADDRESS_TYPE(4i32);
pub const VDS_IA_SCSI: VDS_INTERCONNECT_ADDRESS_TYPE = VDS_INTERCONNECT_ADDRESS_TYPE(5i32);
pub const VDS_IA_UNKNOWN: VDS_INTERCONNECT_ADDRESS_TYPE = VDS_INTERCONNECT_ADDRESS_TYPE(0i32);
pub const VDS_IIF_AGGRESSIVE_MODE: VDS_ISCSI_IPSEC_FLAG = VDS_ISCSI_IPSEC_FLAG(8i32);
pub const VDS_IIF_IKE: VDS_ISCSI_IPSEC_FLAG = VDS_ISCSI_IPSEC_FLAG(2i32);
pub const VDS_IIF_MAIN_MODE: VDS_ISCSI_IPSEC_FLAG = VDS_ISCSI_IPSEC_FLAG(4i32);
pub const VDS_IIF_PFS_ENABLE: VDS_ISCSI_IPSEC_FLAG = VDS_ISCSI_IPSEC_FLAG(16i32);
pub const VDS_IIF_TRANSPORT_MODE_PREFERRED: VDS_ISCSI_IPSEC_FLAG = VDS_ISCSI_IPSEC_FLAG(32i32);
pub const VDS_IIF_TUNNEL_MODE_PREFERRED: VDS_ISCSI_IPSEC_FLAG = VDS_ISCSI_IPSEC_FLAG(64i32);
pub const VDS_IIF_VALID: VDS_ISCSI_IPSEC_FLAG = VDS_ISCSI_IPSEC_FLAG(1i32);
pub const VDS_ILF_MULTIPATH_ENABLED: VDS_ISCSI_LOGIN_FLAG = VDS_ISCSI_LOGIN_FLAG(2i32);
pub const VDS_ILF_REQUIRE_IPSEC: VDS_ISCSI_LOGIN_FLAG = VDS_ISCSI_LOGIN_FLAG(1i32);
pub const VDS_ILT_BOOT: VDS_ISCSI_LOGIN_TYPE = VDS_ISCSI_LOGIN_TYPE(2i32);
pub const VDS_ILT_MANUAL: VDS_ISCSI_LOGIN_TYPE = VDS_ISCSI_LOGIN_TYPE(0i32);
pub const VDS_ILT_PERSISTENT: VDS_ISCSI_LOGIN_TYPE = VDS_ISCSI_LOGIN_TYPE(1i32);
pub const VDS_IPS_FAILED: VDS_ISCSI_PORTAL_STATUS = VDS_ISCSI_PORTAL_STATUS(5i32);
pub const VDS_IPS_NOT_READY: VDS_ISCSI_PORTAL_STATUS = VDS_ISCSI_PORTAL_STATUS(2i32);
pub const VDS_IPS_OFFLINE: VDS_ISCSI_PORTAL_STATUS = VDS_ISCSI_PORTAL_STATUS(4i32);
pub const VDS_IPS_ONLINE: VDS_ISCSI_PORTAL_STATUS = VDS_ISCSI_PORTAL_STATUS(1i32);
pub const VDS_IPS_UNKNOWN: VDS_ISCSI_PORTAL_STATUS = VDS_ISCSI_PORTAL_STATUS(0i32);
pub const VDS_IPT_EMPTY: VDS_IPADDRESS_TYPE = VDS_IPADDRESS_TYPE(3i32);
pub const VDS_IPT_IPV4: VDS_IPADDRESS_TYPE = VDS_IPADDRESS_TYPE(1i32);
pub const VDS_IPT_IPV6: VDS_IPADDRESS_TYPE = VDS_IPADDRESS_TYPE(2i32);
pub const VDS_IPT_TEXT: VDS_IPADDRESS_TYPE = VDS_IPADDRESS_TYPE(0i32);
pub const VDS_ITF_FIBRE_CHANNEL: VDS_INTERCONNECT_FLAG = VDS_INTERCONNECT_FLAG(2i32);
pub const VDS_ITF_ISCSI: VDS_INTERCONNECT_FLAG = VDS_INTERCONNECT_FLAG(4i32);
pub const VDS_ITF_PCI_RAID: VDS_INTERCONNECT_FLAG = VDS_INTERCONNECT_FLAG(1i32);
pub const VDS_ITF_SAS: VDS_INTERCONNECT_FLAG = VDS_INTERCONNECT_FLAG(8i32);
pub const VDS_LBF_DYN_LEAST_QUEUE_DEPTH: VDS_PROVIDER_LBSUPPORT_FLAG = VDS_PROVIDER_LBSUPPORT_FLAG(8i32);
pub const VDS_LBF_FAILOVER: VDS_PROVIDER_LBSUPPORT_FLAG = VDS_PROVIDER_LBSUPPORT_FLAG(1i32);
pub const VDS_LBF_LEAST_BLOCKS: VDS_PROVIDER_LBSUPPORT_FLAG = VDS_PROVIDER_LBSUPPORT_FLAG(32i32);
pub const VDS_LBF_ROUND_ROBIN: VDS_PROVIDER_LBSUPPORT_FLAG = VDS_PROVIDER_LBSUPPORT_FLAG(2i32);
pub const VDS_LBF_ROUND_ROBIN_WITH_SUBSET: VDS_PROVIDER_LBSUPPORT_FLAG = VDS_PROVIDER_LBSUPPORT_FLAG(4i32);
pub const VDS_LBF_VENDOR_SPECIFIC: VDS_PROVIDER_LBSUPPORT_FLAG = VDS_PROVIDER_LBSUPPORT_FLAG(64i32);
pub const VDS_LBF_WEIGHTED_PATHS: VDS_PROVIDER_LBSUPPORT_FLAG = VDS_PROVIDER_LBSUPPORT_FLAG(16i32);
pub const VDS_LBP_DYN_LEAST_QUEUE_DEPTH: VDS_LOADBALANCE_POLICY_ENUM = VDS_LOADBALANCE_POLICY_ENUM(4i32);
pub const VDS_LBP_FAILOVER: VDS_LOADBALANCE_POLICY_ENUM = VDS_LOADBALANCE_POLICY_ENUM(1i32);
pub const VDS_LBP_LEAST_BLOCKS: VDS_LOADBALANCE_POLICY_ENUM = VDS_LOADBALANCE_POLICY_ENUM(6i32);
pub const VDS_LBP_ROUND_ROBIN: VDS_LOADBALANCE_POLICY_ENUM = VDS_LOADBALANCE_POLICY_ENUM(2i32);
pub const VDS_LBP_ROUND_ROBIN_WITH_SUBSET: VDS_LOADBALANCE_POLICY_ENUM = VDS_LOADBALANCE_POLICY_ENUM(3i32);
pub const VDS_LBP_UNKNOWN: VDS_LOADBALANCE_POLICY_ENUM = VDS_LOADBALANCE_POLICY_ENUM(0i32);
pub const VDS_LBP_VENDOR_SPECIFIC: VDS_LOADBALANCE_POLICY_ENUM = VDS_LOADBALANCE_POLICY_ENUM(7i32);
pub const VDS_LBP_WEIGHTED_PATHS: VDS_LOADBALANCE_POLICY_ENUM = VDS_LOADBALANCE_POLICY_ENUM(5i32);
pub const VDS_LF_CONSISTENCY_CHECK_ENABLED: VDS_LUN_FLAG = VDS_LUN_FLAG(128i32);
pub const VDS_LF_HARDWARE_CHECKSUM_ENABLED: VDS_LUN_FLAG = VDS_LUN_FLAG(8i32);
pub const VDS_LF_LBN_REMAP_ENABLED: VDS_LUN_FLAG = VDS_LUN_FLAG(1i32);
pub const VDS_LF_MEDIA_SCAN_ENABLED: VDS_LUN_FLAG = VDS_LUN_FLAG(64i32);
pub const VDS_LF_READ_BACK_VERIFY_ENABLED: VDS_LUN_FLAG = VDS_LUN_FLAG(2i32);
pub const VDS_LF_READ_CACHE_ENABLED: VDS_LUN_FLAG = VDS_LUN_FLAG(16i32);
pub const VDS_LF_SNAPSHOT: VDS_LUN_FLAG = VDS_LUN_FLAG(256i32);
pub const VDS_LF_WRITE_CACHE_ENABLED: VDS_LUN_FLAG = VDS_LUN_FLAG(32i32);
pub const VDS_LF_WRITE_THROUGH_CACHING_ENABLED: VDS_LUN_FLAG = VDS_LUN_FLAG(4i32);
pub const VDS_LPF_LBN_REMAP_ENABLED: VDS_LUN_PLEX_FLAG = VDS_LUN_PLEX_FLAG(1i32);
pub const VDS_LPS_FAILED: VDS_LUN_PLEX_STATUS = VDS_LUN_PLEX_STATUS(5i32);
pub const VDS_LPS_NOT_READY: VDS_LUN_PLEX_STATUS = VDS_LUN_PLEX_STATUS(2i32);
pub const VDS_LPS_OFFLINE: VDS_LUN_PLEX_STATUS = VDS_LUN_PLEX_STATUS(4i32);
pub const VDS_LPS_ONLINE: VDS_LUN_PLEX_STATUS = VDS_LUN_PLEX_STATUS(1i32);
pub const VDS_LPS_UNKNOWN: VDS_LUN_PLEX_STATUS = VDS_LUN_PLEX_STATUS(0i32);
pub const VDS_LPT_PARITY: VDS_LUN_PLEX_TYPE = VDS_LUN_PLEX_TYPE(14i32);
pub const VDS_LPT_RAID03: VDS_LUN_PLEX_TYPE = VDS_LUN_PLEX_TYPE(21i32);
pub const VDS_LPT_RAID05: VDS_LUN_PLEX_TYPE = VDS_LUN_PLEX_TYPE(22i32);
pub const VDS_LPT_RAID10: VDS_LUN_PLEX_TYPE = VDS_LUN_PLEX_TYPE(23i32);
pub const VDS_LPT_RAID15: VDS_LUN_PLEX_TYPE = VDS_LUN_PLEX_TYPE(24i32);
pub const VDS_LPT_RAID2: VDS_LUN_PLEX_TYPE = VDS_LUN_PLEX_TYPE(15i32);
pub const VDS_LPT_RAID3: VDS_LUN_PLEX_TYPE = VDS_LUN_PLEX_TYPE(16i32);
pub const VDS_LPT_RAID30: VDS_LUN_PLEX_TYPE = VDS_LUN_PLEX_TYPE(25i32);
pub const VDS_LPT_RAID4: VDS_LUN_PLEX_TYPE = VDS_LUN_PLEX_TYPE(17i32);
pub const VDS_LPT_RAID5: VDS_LUN_PLEX_TYPE = VDS_LUN_PLEX_TYPE(18i32);
pub const VDS_LPT_RAID50: VDS_LUN_PLEX_TYPE = VDS_LUN_PLEX_TYPE(26i32);
pub const VDS_LPT_RAID53: VDS_LUN_PLEX_TYPE = VDS_LUN_PLEX_TYPE(28i32);
pub const VDS_LPT_RAID6: VDS_LUN_PLEX_TYPE = VDS_LUN_PLEX_TYPE(19i32);
pub const VDS_LPT_RAID60: VDS_LUN_PLEX_TYPE = VDS_LUN_PLEX_TYPE(29i32);
pub const VDS_LPT_SIMPLE: VDS_LUN_PLEX_TYPE = VDS_LUN_PLEX_TYPE(10i32);
pub const VDS_LPT_SPAN: VDS_LUN_PLEX_TYPE = VDS_LUN_PLEX_TYPE(11i32);
pub const VDS_LPT_STRIPE: VDS_LUN_PLEX_TYPE = VDS_LUN_PLEX_TYPE(12i32);
pub const VDS_LPT_UNKNOWN: VDS_LUN_PLEX_TYPE = VDS_LUN_PLEX_TYPE(0i32);
pub const VDS_LRM_EXCLUSIVE_RO: VDS_LUN_RESERVE_MODE = VDS_LUN_RESERVE_MODE(2i32);
pub const VDS_LRM_EXCLUSIVE_RW: VDS_LUN_RESERVE_MODE = VDS_LUN_RESERVE_MODE(1i32);
pub const VDS_LRM_NONE: VDS_LUN_RESERVE_MODE = VDS_LUN_RESERVE_MODE(0i32);
pub const VDS_LRM_SHARED_RO: VDS_LUN_RESERVE_MODE = VDS_LUN_RESERVE_MODE(3i32);
pub const VDS_LRM_SHARED_RW: VDS_LUN_RESERVE_MODE = VDS_LUN_RESERVE_MODE(4i32);
pub const VDS_LS_FAILED: VDS_LUN_STATUS = VDS_LUN_STATUS(5i32);
pub const VDS_LS_NOT_READY: VDS_LUN_STATUS = VDS_LUN_STATUS(2i32);
pub const VDS_LS_OFFLINE: VDS_LUN_STATUS = VDS_LUN_STATUS(4i32);
pub const VDS_LS_ONLINE: VDS_LUN_STATUS = VDS_LUN_STATUS(1i32);
pub const VDS_LS_UNKNOWN: VDS_LUN_STATUS = VDS_LUN_STATUS(0i32);
pub const VDS_LT_DEFAULT: VDS_LUN_TYPE = VDS_LUN_TYPE(1i32);
pub const VDS_LT_FAULT_TOLERANT: VDS_LUN_TYPE = VDS_LUN_TYPE(2i32);
pub const VDS_LT_MIRROR: VDS_LUN_TYPE = VDS_LUN_TYPE(13i32);
pub const VDS_LT_NON_FAULT_TOLERANT: VDS_LUN_TYPE = VDS_LUN_TYPE(3i32);
pub const VDS_LT_PARITY: VDS_LUN_TYPE = VDS_LUN_TYPE(14i32);
pub const VDS_LT_RAID01: VDS_LUN_TYPE = VDS_LUN_TYPE(20i32);
pub const VDS_LT_RAID03: VDS_LUN_TYPE = VDS_LUN_TYPE(21i32);
pub const VDS_LT_RAID05: VDS_LUN_TYPE = VDS_LUN_TYPE(22i32);
pub const VDS_LT_RAID10: VDS_LUN_TYPE = VDS_LUN_TYPE(23i32);
pub const VDS_LT_RAID15: VDS_LUN_TYPE = VDS_LUN_TYPE(24i32);
pub const VDS_LT_RAID2: VDS_LUN_TYPE = VDS_LUN_TYPE(15i32);
pub const VDS_LT_RAID3: VDS_LUN_TYPE = VDS_LUN_TYPE(16i32);
pub const VDS_LT_RAID30: VDS_LUN_TYPE = VDS_LUN_TYPE(25i32);
pub const VDS_LT_RAID4: VDS_LUN_TYPE = VDS_LUN_TYPE(17i32);
pub const VDS_LT_RAID5: VDS_LUN_TYPE = VDS_LUN_TYPE(18i32);
pub const VDS_LT_RAID50: VDS_LUN_TYPE = VDS_LUN_TYPE(26i32);
pub const VDS_LT_RAID51: VDS_LUN_TYPE = VDS_LUN_TYPE(27i32);
pub const VDS_LT_RAID53: VDS_LUN_TYPE = VDS_LUN_TYPE(28i32);
pub const VDS_LT_RAID6: VDS_LUN_TYPE = VDS_LUN_TYPE(19i32);
pub const VDS_LT_RAID60: VDS_LUN_TYPE = VDS_LUN_TYPE(29i32);
pub const VDS_LT_RAID61: VDS_LUN_TYPE = VDS_LUN_TYPE(30i32);
pub const VDS_LT_SIMPLE: VDS_LUN_TYPE = VDS_LUN_TYPE(10i32);
pub const VDS_LT_SPAN: VDS_LUN_TYPE = VDS_LUN_TYPE(11i32);
pub const VDS_LT_STRIPE: VDS_LUN_TYPE = VDS_LUN_TYPE(12i32);
pub const VDS_LT_UNKNOWN: VDS_LUN_TYPE = VDS_LUN_TYPE(0i32);
pub const VDS_MPS_FAILED: VDS_PATH_STATUS = VDS_PATH_STATUS(5i32);
pub const VDS_MPS_ONLINE: VDS_PATH_STATUS = VDS_PATH_STATUS(1i32);
pub const VDS_MPS_STANDBY: VDS_PATH_STATUS = VDS_PATH_STATUS(7i32);
pub const VDS_MPS_UNKNOWN: VDS_PATH_STATUS = VDS_PATH_STATUS(0i32);
pub const VDS_NF_CONTROLLER_ARRIVE: VDS_NF_CONTROLLER = VDS_NF_CONTROLLER(103u32);
pub const VDS_NF_CONTROLLER_DEPART: VDS_NF_CONTROLLER = VDS_NF_CONTROLLER(104u32);
pub const VDS_NF_CONTROLLER_MODIFY: VDS_NF_CONTROLLER = VDS_NF_CONTROLLER(350u32);
pub const VDS_NF_CONTROLLER_REMOVED: VDS_NF_CONTROLLER = VDS_NF_CONTROLLER(351u32);
pub const VDS_NF_DISK_ARRIVE: VDS_NF_DISK = VDS_NF_DISK(8u32);
pub const VDS_NF_DISK_DEPART: VDS_NF_DISK = VDS_NF_DISK(9u32);
pub const VDS_NF_DISK_MODIFY: VDS_NF_DISK = VDS_NF_DISK(10u32);
pub const VDS_NF_DRIVE_ARRIVE: VDS_NF_DRIVE = VDS_NF_DRIVE(105u32);
pub const VDS_NF_DRIVE_DEPART: VDS_NF_DRIVE = VDS_NF_DRIVE(106u32);
pub const VDS_NF_DRIVE_LETTER_ASSIGN: u32 = 202u32;
pub const VDS_NF_DRIVE_LETTER_FREE: u32 = 201u32;
pub const VDS_NF_DRIVE_MODIFY: VDS_NF_DRIVE = VDS_NF_DRIVE(107u32);
pub const VDS_NF_DRIVE_REMOVED: VDS_NF_DRIVE = VDS_NF_DRIVE(354u32);
pub const VDS_NF_FILE_SYSTEM_FORMAT_PROGRESS: VDS_NF_FILE_SYSTEM = VDS_NF_FILE_SYSTEM(204u32);
pub const VDS_NF_FILE_SYSTEM_MODIFY: VDS_NF_FILE_SYSTEM = VDS_NF_FILE_SYSTEM(203u32);
pub const VDS_NF_FILE_SYSTEM_SHRINKING_PROGRESS: u32 = 206u32;
pub const VDS_NF_LUN_ARRIVE: VDS_NF_LUN = VDS_NF_LUN(108u32);
pub const VDS_NF_LUN_DEPART: VDS_NF_LUN = VDS_NF_LUN(109u32);
pub const VDS_NF_LUN_MODIFY: VDS_NF_LUN = VDS_NF_LUN(110u32);
pub const VDS_NF_MOUNT_POINTS_CHANGE: u32 = 205u32;
pub const VDS_NF_PACK_ARRIVE: VDS_NF_PACK = VDS_NF_PACK(1u32);
pub const VDS_NF_PACK_DEPART: VDS_NF_PACK = VDS_NF_PACK(2u32);
pub const VDS_NF_PACK_MODIFY: VDS_NF_PACK = VDS_NF_PACK(3u32);
pub const VDS_NF_PARTITION_ARRIVE: u32 = 11u32;
pub const VDS_NF_PARTITION_DEPART: u32 = 12u32;
pub const VDS_NF_PARTITION_MODIFY: u32 = 13u32;
pub const VDS_NF_PORTAL_ARRIVE: u32 = 123u32;
pub const VDS_NF_PORTAL_DEPART: u32 = 124u32;
pub const VDS_NF_PORTAL_GROUP_ARRIVE: u32 = 129u32;
pub const VDS_NF_PORTAL_GROUP_DEPART: u32 = 130u32;
pub const VDS_NF_PORTAL_GROUP_MODIFY: u32 = 131u32;
pub const VDS_NF_PORTAL_MODIFY: u32 = 125u32;
pub const VDS_NF_PORT_ARRIVE: VDS_NF_PORT = VDS_NF_PORT(121u32);
pub const VDS_NF_PORT_DEPART: VDS_NF_PORT = VDS_NF_PORT(122u32);
pub const VDS_NF_PORT_MODIFY: VDS_NF_PORT = VDS_NF_PORT(352u32);
pub const VDS_NF_PORT_REMOVED: VDS_NF_PORT = VDS_NF_PORT(353u32);
pub const VDS_NF_SERVICE_OUT_OF_SYNC: u32 = 301u32;
pub const VDS_NF_SUB_SYSTEM_ARRIVE: u32 = 101u32;
pub const VDS_NF_SUB_SYSTEM_DEPART: u32 = 102u32;
pub const VDS_NF_SUB_SYSTEM_MODIFY: u32 = 151u32;
pub const VDS_NF_TARGET_ARRIVE: u32 = 126u32;
pub const VDS_NF_TARGET_DEPART: u32 = 127u32;
pub const VDS_NF_TARGET_MODIFY: u32 = 128u32;
pub const VDS_NF_VOLUME_ARRIVE: u32 = 4u32;
pub const VDS_NF_VOLUME_DEPART: u32 = 5u32;
pub const VDS_NF_VOLUME_MODIFY: u32 = 6u32;
pub const VDS_NF_VOLUME_REBUILDING_PROGRESS: u32 = 7u32;
pub const VDS_NTT_CONTROLLER: VDS_NOTIFICATION_TARGET_TYPE = VDS_NOTIFICATION_TARGET_TYPE(31i32);
pub const VDS_NTT_DISK: VDS_NOTIFICATION_TARGET_TYPE = VDS_NOTIFICATION_TARGET_TYPE(13i32);
pub const VDS_NTT_DRIVE: VDS_NOTIFICATION_TARGET_TYPE = VDS_NOTIFICATION_TARGET_TYPE(32i32);
pub const VDS_NTT_DRIVE_LETTER: VDS_NOTIFICATION_TARGET_TYPE = VDS_NOTIFICATION_TARGET_TYPE(61i32);
pub const VDS_NTT_FILE_SYSTEM: VDS_NOTIFICATION_TARGET_TYPE = VDS_NOTIFICATION_TARGET_TYPE(62i32);
pub const VDS_NTT_LUN: VDS_NOTIFICATION_TARGET_TYPE = VDS_NOTIFICATION_TARGET_TYPE(33i32);
pub const VDS_NTT_MOUNT_POINT: VDS_NOTIFICATION_TARGET_TYPE = VDS_NOTIFICATION_TARGET_TYPE(63i32);
pub const VDS_NTT_PACK: VDS_NOTIFICATION_TARGET_TYPE = VDS_NOTIFICATION_TARGET_TYPE(10i32);
pub const VDS_NTT_PARTITION: VDS_NOTIFICATION_TARGET_TYPE = VDS_NOTIFICATION_TARGET_TYPE(60i32);
pub const VDS_NTT_PORT: VDS_NOTIFICATION_TARGET_TYPE = VDS_NOTIFICATION_TARGET_TYPE(35i32);
pub const VDS_NTT_PORTAL: VDS_NOTIFICATION_TARGET_TYPE = VDS_NOTIFICATION_TARGET_TYPE(36i32);
pub const VDS_NTT_PORTAL_GROUP: VDS_NOTIFICATION_TARGET_TYPE = VDS_NOTIFICATION_TARGET_TYPE(38i32);
pub const VDS_NTT_SERVICE: VDS_NOTIFICATION_TARGET_TYPE = VDS_NOTIFICATION_TARGET_TYPE(200i32);
pub const VDS_NTT_SUB_SYSTEM: VDS_NOTIFICATION_TARGET_TYPE = VDS_NOTIFICATION_TARGET_TYPE(30i32);
pub const VDS_NTT_TARGET: VDS_NOTIFICATION_TARGET_TYPE = VDS_NOTIFICATION_TARGET_TYPE(37i32);
pub const VDS_NTT_UNKNOWN: VDS_NOTIFICATION_TARGET_TYPE = VDS_NOTIFICATION_TARGET_TYPE(0i32);
pub const VDS_NTT_VOLUME: VDS_NOTIFICATION_TARGET_TYPE = VDS_NOTIFICATION_TARGET_TYPE(11i32);
pub const VDS_OT_ASYNC: VDS_OBJECT_TYPE = VDS_OBJECT_TYPE(100i32);
pub const VDS_OT_CONTROLLER: VDS_OBJECT_TYPE = VDS_OBJECT_TYPE(31i32);
pub const VDS_OT_DISK: VDS_OBJECT_TYPE = VDS_OBJECT_TYPE(13i32);
pub const VDS_OT_DRIVE: VDS_OBJECT_TYPE = VDS_OBJECT_TYPE(32i32);
pub const VDS_OT_ENUM: VDS_OBJECT_TYPE = VDS_OBJECT_TYPE(101i32);
pub const VDS_OT_HBAPORT: VDS_OBJECT_TYPE = VDS_OBJECT_TYPE(90i32);
pub const VDS_OT_INIT_ADAPTER: VDS_OBJECT_TYPE = VDS_OBJECT_TYPE(91i32);
pub const VDS_OT_INIT_PORTAL: VDS_OBJECT_TYPE = VDS_OBJECT_TYPE(92i32);
pub const VDS_OT_LUN: VDS_OBJECT_TYPE = VDS_OBJECT_TYPE(33i32);
pub const VDS_OT_LUN_PLEX: VDS_OBJECT_TYPE = VDS_OBJECT_TYPE(34i32);
pub const VDS_OT_OPEN_VDISK: VDS_OBJECT_TYPE = VDS_OBJECT_TYPE(201i32);
pub const VDS_OT_PACK: VDS_OBJECT_TYPE = VDS_OBJECT_TYPE(10i32);
pub const VDS_OT_PORT: VDS_OBJECT_TYPE = VDS_OBJECT_TYPE(35i32);
pub const VDS_OT_PORTAL: VDS_OBJECT_TYPE = VDS_OBJECT_TYPE(36i32);
pub const VDS_OT_PORTAL_GROUP: VDS_OBJECT_TYPE = VDS_OBJECT_TYPE(38i32);
pub const VDS_OT_PROVIDER: VDS_OBJECT_TYPE = VDS_OBJECT_TYPE(1i32);
pub const VDS_OT_STORAGE_POOL: VDS_OBJECT_TYPE = VDS_OBJECT_TYPE(39i32);
pub const VDS_OT_SUB_SYSTEM: VDS_OBJECT_TYPE = VDS_OBJECT_TYPE(30i32);
pub const VDS_OT_TARGET: VDS_OBJECT_TYPE = VDS_OBJECT_TYPE(37i32);
pub const VDS_OT_UNKNOWN: VDS_OBJECT_TYPE = VDS_OBJECT_TYPE(0i32);
pub const VDS_OT_VDISK: VDS_OBJECT_TYPE = VDS_OBJECT_TYPE(200i32);
pub const VDS_OT_VOLUME: VDS_OBJECT_TYPE = VDS_OBJECT_TYPE(11i32);
pub const VDS_OT_VOLUME_PLEX: VDS_OBJECT_TYPE = VDS_OBJECT_TYPE(12i32);
pub const VDS_PARTITION_STYLE_GPT: __VDS_PARTITION_STYLE = __VDS_PARTITION_STYLE(1i32);
pub const VDS_PARTITION_STYLE_MBR: __VDS_PARTITION_STYLE = __VDS_PARTITION_STYLE(0i32);
pub const VDS_PARTITION_STYLE_RAW: __VDS_PARTITION_STYLE = __VDS_PARTITION_STYLE(2i32);
pub const VDS_PF_DYNAMIC: VDS_PROVIDER_FLAG = VDS_PROVIDER_FLAG(1i32);
pub const VDS_PF_INTERNAL_HARDWARE_PROVIDER: VDS_PROVIDER_FLAG = VDS_PROVIDER_FLAG(2i32);
pub const VDS_PF_ONE_DISK_ONLY_PER_PACK: VDS_PROVIDER_FLAG = VDS_PROVIDER_FLAG(4i32);
pub const VDS_PF_ONE_PACK_ONLINE_ONLY: VDS_PROVIDER_FLAG = VDS_PROVIDER_FLAG(8i32);
pub const VDS_PF_SUPPORT_DYNAMIC: VDS_PROVIDER_FLAG = VDS_PROVIDER_FLAG(-2147483648i32);
pub const VDS_PF_SUPPORT_DYNAMIC_1394: VDS_PROVIDER_FLAG = VDS_PROVIDER_FLAG(536870912i32);
pub const VDS_PF_SUPPORT_FAULT_TOLERANT: VDS_PROVIDER_FLAG = VDS_PROVIDER_FLAG(1073741824i32);
pub const VDS_PF_SUPPORT_MIRROR: VDS_PROVIDER_FLAG = VDS_PROVIDER_FLAG(32i32);
pub const VDS_PF_SUPPORT_RAID5: VDS_PROVIDER_FLAG = VDS_PROVIDER_FLAG(64i32);
pub const VDS_PF_VOLUME_SPACE_MUST_BE_CONTIGUOUS: VDS_PROVIDER_FLAG = VDS_PROVIDER_FLAG(16i32);
pub const VDS_PKF_CORRUPTED: VDS_PACK_FLAG = VDS_PACK_FLAG(8i32);
pub const VDS_PKF_FOREIGN: VDS_PACK_FLAG = VDS_PACK_FLAG(1i32);
pub const VDS_PKF_NOQUORUM: VDS_PACK_FLAG = VDS_PACK_FLAG(2i32);
pub const VDS_PKF_ONLINE_ERROR: VDS_PACK_FLAG = VDS_PACK_FLAG(16i32);
pub const VDS_PKF_POLICY: VDS_PACK_FLAG = VDS_PACK_FLAG(4i32);
pub const VDS_POOL_ATTRIB_ACCS_BDW_WT_HINT: i32 = 16777216i32;
pub const VDS_POOL_ATTRIB_ACCS_DIR_HINT: i32 = 2097152i32;
pub const VDS_POOL_ATTRIB_ACCS_LTNCY_HINT: i32 = 8388608i32;
pub const VDS_POOL_ATTRIB_ACCS_RNDM_HINT: i32 = 1048576i32;
pub const VDS_POOL_ATTRIB_ACCS_SIZE_HINT: i32 = 4194304i32;
pub const VDS_POOL_ATTRIB_ALLOW_SPINDOWN: i32 = 4i32;
pub const VDS_POOL_ATTRIB_BUSTYPE: i32 = 2i32;
pub const VDS_POOL_ATTRIB_CUSTOM_ATTRIB: i32 = 134217728i32;
pub const VDS_POOL_ATTRIB_DATA_AVL_HINT: i32 = 524288i32;
pub const VDS_POOL_ATTRIB_DATA_RDNCY_DEF: i32 = 128i32;
pub const VDS_POOL_ATTRIB_DATA_RDNCY_MAX: i32 = 32i32;
pub const VDS_POOL_ATTRIB_DATA_RDNCY_MIN: i32 = 64i32;
pub const VDS_POOL_ATTRIB_NO_SINGLE_POF: i32 = 16i32;
pub const VDS_POOL_ATTRIB_NUM_CLMNS: i32 = 32768i32;
pub const VDS_POOL_ATTRIB_NUM_CLMNS_DEF: i32 = 262144i32;
pub const VDS_POOL_ATTRIB_NUM_CLMNS_MAX: i32 = 65536i32;
pub const VDS_POOL_ATTRIB_NUM_CLMNS_MIN: i32 = 131072i32;
pub const VDS_POOL_ATTRIB_PKG_RDNCY_DEF: i32 = 1024i32;
pub const VDS_POOL_ATTRIB_PKG_RDNCY_MAX: i32 = 256i32;
pub const VDS_POOL_ATTRIB_PKG_RDNCY_MIN: i32 = 512i32;
pub const VDS_POOL_ATTRIB_RAIDTYPE: i32 = 1i32;
pub const VDS_POOL_ATTRIB_STOR_COST_HINT: i32 = 33554432i32;
pub const VDS_POOL_ATTRIB_STOR_EFFCY_HINT: i32 = 67108864i32;
pub const VDS_POOL_ATTRIB_STRIPE_SIZE: i32 = 2048i32;
pub const VDS_POOL_ATTRIB_STRIPE_SIZE_DEF: i32 = 16384i32;
pub const VDS_POOL_ATTRIB_STRIPE_SIZE_MAX: i32 = 4096i32;
pub const VDS_POOL_ATTRIB_STRIPE_SIZE_MIN: i32 = 8192i32;
pub const VDS_POOL_ATTRIB_THIN_PROVISION: i32 = 8i32;
pub const VDS_PRS_FAILED: VDS_PORT_STATUS = VDS_PORT_STATUS(5i32);
pub const VDS_PRS_NOT_READY: VDS_PORT_STATUS = VDS_PORT_STATUS(2i32);
pub const VDS_PRS_OFFLINE: VDS_PORT_STATUS = VDS_PORT_STATUS(4i32);
pub const VDS_PRS_ONLINE: VDS_PORT_STATUS = VDS_PORT_STATUS(1i32);
pub const VDS_PRS_REMOVED: VDS_PORT_STATUS = VDS_PORT_STATUS(8i32);
pub const VDS_PRS_UNKNOWN: VDS_PORT_STATUS = VDS_PORT_STATUS(0i32);
pub const VDS_PST_GPT: VDS_PARTITION_STYLE = VDS_PARTITION_STYLE(2i32);
pub const VDS_PST_MBR: VDS_PARTITION_STYLE = VDS_PARTITION_STYLE(1i32);
pub const VDS_PST_UNKNOWN: VDS_PARTITION_STYLE = VDS_PARTITION_STYLE(0i32);
pub const VDS_PS_OFFLINE: VDS_PACK_STATUS = VDS_PACK_STATUS(4i32);
pub const VDS_PS_ONLINE: VDS_PACK_STATUS = VDS_PACK_STATUS(1i32);
pub const VDS_PS_UNKNOWN: VDS_PACK_STATUS = VDS_PACK_STATUS(0i32);
pub const VDS_PTF_SYSTEM: VDS_PARTITION_FLAG = VDS_PARTITION_FLAG(1i32);
pub const VDS_PT_HARDWARE: VDS_PROVIDER_TYPE = VDS_PROVIDER_TYPE(2i32);
pub const VDS_PT_MAX: VDS_PROVIDER_TYPE = VDS_PROVIDER_TYPE(4i32);
pub const VDS_PT_SOFTWARE: VDS_PROVIDER_TYPE = VDS_PROVIDER_TYPE(1i32);
pub const VDS_PT_UNKNOWN: VDS_PROVIDER_TYPE = VDS_PROVIDER_TYPE(0i32);
pub const VDS_PT_VIRTUALDISK: VDS_PROVIDER_TYPE = VDS_PROVIDER_TYPE(3i32);
pub const VDS_QUERY_HARDWARE_PROVIDERS: VDS_QUERY_PROVIDER_FLAG = VDS_QUERY_PROVIDER_FLAG(2i32);
pub const VDS_QUERY_SOFTWARE_PROVIDERS: VDS_QUERY_PROVIDER_FLAG = VDS_QUERY_PROVIDER_FLAG(1i32);
pub const VDS_QUERY_VIRTUALDISK_PROVIDERS: VDS_QUERY_PROVIDER_FLAG = VDS_QUERY_PROVIDER_FLAG(4i32);
pub const VDS_RA_REFRESH: VDS_RECOVER_ACTION = VDS_RECOVER_ACTION(1i32);
pub const VDS_RA_RESTART: VDS_RECOVER_ACTION = VDS_RECOVER_ACTION(2i32);
pub const VDS_RA_UNKNOWN: VDS_RECOVER_ACTION = VDS_RECOVER_ACTION(0i32);
pub const VDS_REBUILD_PRIORITY_MAX: u32 = 16u32;
pub const VDS_REBUILD_PRIORITY_MIN: u32 = 0u32;
pub const VDS_RT_RAID0: VDS_RAID_TYPE = VDS_RAID_TYPE(10i32);
pub const VDS_RT_RAID01: VDS_RAID_TYPE = VDS_RAID_TYPE(17i32);
pub const VDS_RT_RAID03: VDS_RAID_TYPE = VDS_RAID_TYPE(18i32);
pub const VDS_RT_RAID05: VDS_RAID_TYPE = VDS_RAID_TYPE(19i32);
pub const VDS_RT_RAID1: VDS_RAID_TYPE = VDS_RAID_TYPE(11i32);
pub const VDS_RT_RAID10: VDS_RAID_TYPE = VDS_RAID_TYPE(20i32);
pub const VDS_RT_RAID15: VDS_RAID_TYPE = VDS_RAID_TYPE(21i32);
pub const VDS_RT_RAID2: VDS_RAID_TYPE = VDS_RAID_TYPE(12i32);
pub const VDS_RT_RAID3: VDS_RAID_TYPE = VDS_RAID_TYPE(13i32);
pub const VDS_RT_RAID30: VDS_RAID_TYPE = VDS_RAID_TYPE(22i32);
pub const VDS_RT_RAID4: VDS_RAID_TYPE = VDS_RAID_TYPE(14i32);
pub const VDS_RT_RAID5: VDS_RAID_TYPE = VDS_RAID_TYPE(15i32);
pub const VDS_RT_RAID50: VDS_RAID_TYPE = VDS_RAID_TYPE(23i32);
pub const VDS_RT_RAID51: VDS_RAID_TYPE = VDS_RAID_TYPE(24i32);
pub const VDS_RT_RAID53: VDS_RAID_TYPE = VDS_RAID_TYPE(25i32);
pub const VDS_RT_RAID6: VDS_RAID_TYPE = VDS_RAID_TYPE(16i32);
pub const VDS_RT_RAID60: VDS_RAID_TYPE = VDS_RAID_TYPE(26i32);
pub const VDS_RT_RAID61: VDS_RAID_TYPE = VDS_RAID_TYPE(27i32);
pub const VDS_RT_UNKNOWN: VDS_RAID_TYPE = VDS_RAID_TYPE(0i32);
pub const VDS_SF_CONSISTENCY_CHECK_CAPABLE: VDS_SUB_SYSTEM_FLAG = VDS_SUB_SYSTEM_FLAG(16777216i32);
pub const VDS_SF_DRIVE_EXTENT_CAPABLE: VDS_SUB_SYSTEM_FLAG = VDS_SUB_SYSTEM_FLAG(8i32);
pub const VDS_SF_HARDWARE_CHECKSUM_CAPABLE: VDS_SUB_SYSTEM_FLAG = VDS_SUB_SYSTEM_FLAG(16i32);
pub const VDS_SF_LUN_MASKING_CAPABLE: VDS_SUB_SYSTEM_FLAG = VDS_SUB_SYSTEM_FLAG(1i32);
pub const VDS_SF_LUN_PLEXING_CAPABLE: VDS_SUB_SYSTEM_FLAG = VDS_SUB_SYSTEM_FLAG(2i32);
pub const VDS_SF_LUN_REMAPPING_CAPABLE: VDS_SUB_SYSTEM_FLAG = VDS_SUB_SYSTEM_FLAG(4i32);
pub const VDS_SF_MEDIA_SCAN_CAPABLE: VDS_SUB_SYSTEM_FLAG = VDS_SUB_SYSTEM_FLAG(8388608i32);
pub const VDS_SF_RADIUS_CAPABLE: VDS_SUB_SYSTEM_FLAG = VDS_SUB_SYSTEM_FLAG(32i32);
pub const VDS_SF_READ_BACK_VERIFY_CAPABLE: VDS_SUB_SYSTEM_FLAG = VDS_SUB_SYSTEM_FLAG(64i32);
pub const VDS_SF_READ_CACHING_CAPABLE: VDS_SUB_SYSTEM_FLAG = VDS_SUB_SYSTEM_FLAG(2097152i32);
pub const VDS_SF_SUPPORTS_AUTH_CHAP: VDS_SUB_SYSTEM_FLAG = VDS_SUB_SYSTEM_FLAG(65536i32);
pub const VDS_SF_SUPPORTS_AUTH_MUTUAL_CHAP: VDS_SUB_SYSTEM_FLAG = VDS_SUB_SYSTEM_FLAG(131072i32);
pub const VDS_SF_SUPPORTS_FAULT_TOLERANT_LUNS: VDS_SUB_SYSTEM_FLAG = VDS_SUB_SYSTEM_FLAG(512i32);
pub const VDS_SF_SUPPORTS_LUN_NUMBER: VDS_SUB_SYSTEM_FLAG = VDS_SUB_SYSTEM_FLAG(524288i32);
pub const VDS_SF_SUPPORTS_MIRRORED_CACHE: VDS_SUB_SYSTEM_FLAG = VDS_SUB_SYSTEM_FLAG(1048576i32);
pub const VDS_SF_SUPPORTS_MIRROR_LUNS: VDS_SUB_SYSTEM_FLAG = VDS_SUB_SYSTEM_FLAG(16384i32);
pub const VDS_SF_SUPPORTS_NON_FAULT_TOLERANT_LUNS: VDS_SUB_SYSTEM_FLAG = VDS_SUB_SYSTEM_FLAG(1024i32);
pub const VDS_SF_SUPPORTS_PARITY_LUNS: VDS_SUB_SYSTEM_FLAG = VDS_SUB_SYSTEM_FLAG(32768i32);
pub const VDS_SF_SUPPORTS_RAID01_LUNS: VDS_SUB_SYSTEM_SUPPORTED_RAID_TYPE_FLAG = VDS_SUB_SYSTEM_SUPPORTED_RAID_TYPE_FLAG(32i32);
pub const VDS_SF_SUPPORTS_RAID03_LUNS: VDS_SUB_SYSTEM_SUPPORTED_RAID_TYPE_FLAG = VDS_SUB_SYSTEM_SUPPORTED_RAID_TYPE_FLAG(64i32);
pub const VDS_SF_SUPPORTS_RAID05_LUNS: VDS_SUB_SYSTEM_SUPPORTED_RAID_TYPE_FLAG = VDS_SUB_SYSTEM_SUPPORTED_RAID_TYPE_FLAG(128i32);
pub const VDS_SF_SUPPORTS_RAID10_LUNS: VDS_SUB_SYSTEM_SUPPORTED_RAID_TYPE_FLAG = VDS_SUB_SYSTEM_SUPPORTED_RAID_TYPE_FLAG(256i32);
pub const VDS_SF_SUPPORTS_RAID15_LUNS: VDS_SUB_SYSTEM_SUPPORTED_RAID_TYPE_FLAG = VDS_SUB_SYSTEM_SUPPORTED_RAID_TYPE_FLAG(512i32);
pub const VDS_SF_SUPPORTS_RAID2_LUNS: VDS_SUB_SYSTEM_SUPPORTED_RAID_TYPE_FLAG = VDS_SUB_SYSTEM_SUPPORTED_RAID_TYPE_FLAG(1i32);
pub const VDS_SF_SUPPORTS_RAID30_LUNS: VDS_SUB_SYSTEM_SUPPORTED_RAID_TYPE_FLAG = VDS_SUB_SYSTEM_SUPPORTED_RAID_TYPE_FLAG(1024i32);
pub const VDS_SF_SUPPORTS_RAID3_LUNS: VDS_SUB_SYSTEM_SUPPORTED_RAID_TYPE_FLAG = VDS_SUB_SYSTEM_SUPPORTED_RAID_TYPE_FLAG(2i32);
pub const VDS_SF_SUPPORTS_RAID4_LUNS: VDS_SUB_SYSTEM_SUPPORTED_RAID_TYPE_FLAG = VDS_SUB_SYSTEM_SUPPORTED_RAID_TYPE_FLAG(4i32);
pub const VDS_SF_SUPPORTS_RAID50_LUNS: VDS_SUB_SYSTEM_SUPPORTED_RAID_TYPE_FLAG = VDS_SUB_SYSTEM_SUPPORTED_RAID_TYPE_FLAG(2048i32);
pub const VDS_SF_SUPPORTS_RAID51_LUNS: VDS_SUB_SYSTEM_SUPPORTED_RAID_TYPE_FLAG = VDS_SUB_SYSTEM_SUPPORTED_RAID_TYPE_FLAG(4096i32);
pub const VDS_SF_SUPPORTS_RAID53_LUNS: VDS_SUB_SYSTEM_SUPPORTED_RAID_TYPE_FLAG = VDS_SUB_SYSTEM_SUPPORTED_RAID_TYPE_FLAG(8192i32);
pub const VDS_SF_SUPPORTS_RAID5_LUNS: VDS_SUB_SYSTEM_SUPPORTED_RAID_TYPE_FLAG = VDS_SUB_SYSTEM_SUPPORTED_RAID_TYPE_FLAG(8i32);
pub const VDS_SF_SUPPORTS_RAID60_LUNS: VDS_SUB_SYSTEM_SUPPORTED_RAID_TYPE_FLAG = VDS_SUB_SYSTEM_SUPPORTED_RAID_TYPE_FLAG(16384i32);
pub const VDS_SF_SUPPORTS_RAID61_LUNS: VDS_SUB_SYSTEM_SUPPORTED_RAID_TYPE_FLAG = VDS_SUB_SYSTEM_SUPPORTED_RAID_TYPE_FLAG(32768i32);
pub const VDS_SF_SUPPORTS_RAID6_LUNS: VDS_SUB_SYSTEM_SUPPORTED_RAID_TYPE_FLAG = VDS_SUB_SYSTEM_SUPPORTED_RAID_TYPE_FLAG(16i32);
pub const VDS_SF_SUPPORTS_SIMPLE_LUNS: VDS_SUB_SYSTEM_FLAG = VDS_SUB_SYSTEM_FLAG(2048i32);
pub const VDS_SF_SUPPORTS_SIMPLE_TARGET_CONFIG: VDS_SUB_SYSTEM_FLAG = VDS_SUB_SYSTEM_FLAG(262144i32);
pub const VDS_SF_SUPPORTS_SPAN_LUNS: VDS_SUB_SYSTEM_FLAG = VDS_SUB_SYSTEM_FLAG(4096i32);
pub const VDS_SF_SUPPORTS_STRIPE_LUNS: VDS_SUB_SYSTEM_FLAG = VDS_SUB_SYSTEM_FLAG(8192i32);
pub const VDS_SF_WRITE_CACHING_CAPABLE: VDS_SUB_SYSTEM_FLAG = VDS_SUB_SYSTEM_FLAG(4194304i32);
pub const VDS_SF_WRITE_THROUGH_CACHING_CAPABLE: VDS_SUB_SYSTEM_FLAG = VDS_SUB_SYSTEM_FLAG(128i32);
pub const VDS_SPS_NOT_READY: VDS_STORAGE_POOL_STATUS = VDS_STORAGE_POOL_STATUS(2i32);
pub const VDS_SPS_OFFLINE: VDS_STORAGE_POOL_STATUS = VDS_STORAGE_POOL_STATUS(4i32);
pub const VDS_SPS_ONLINE: VDS_STORAGE_POOL_STATUS = VDS_STORAGE_POOL_STATUS(1i32);
pub const VDS_SPS_UNKNOWN: VDS_STORAGE_POOL_STATUS = VDS_STORAGE_POOL_STATUS(0i32);
pub const VDS_SPT_CONCRETE: VDS_STORAGE_POOL_TYPE = VDS_STORAGE_POOL_TYPE(2i32);
pub const VDS_SPT_PRIMORDIAL: VDS_STORAGE_POOL_TYPE = VDS_STORAGE_POOL_TYPE(1i32);
pub const VDS_SPT_UNKNOWN: VDS_STORAGE_POOL_TYPE = VDS_STORAGE_POOL_TYPE(0i32);
pub const VDS_SP_MAX: VDS_SAN_POLICY = VDS_SAN_POLICY(5i32);
pub const VDS_SP_OFFLINE: VDS_SAN_POLICY = VDS_SAN_POLICY(3i32);
pub const VDS_SP_OFFLINE_INTERNAL: VDS_SAN_POLICY = VDS_SAN_POLICY(4i32);
pub const VDS_SP_OFFLINE_SHARED: VDS_SAN_POLICY = VDS_SAN_POLICY(2i32);
pub const VDS_SP_ONLINE: VDS_SAN_POLICY = VDS_SAN_POLICY(1i32);
pub const VDS_SP_UNKNOWN: VDS_SAN_POLICY = VDS_SAN_POLICY(0i32);
pub const VDS_SSS_FAILED: VDS_SUB_SYSTEM_STATUS = VDS_SUB_SYSTEM_STATUS(5i32);
pub const VDS_SSS_NOT_READY: VDS_SUB_SYSTEM_STATUS = VDS_SUB_SYSTEM_STATUS(2i32);
pub const VDS_SSS_OFFLINE: VDS_SUB_SYSTEM_STATUS = VDS_SUB_SYSTEM_STATUS(4i32);
pub const VDS_SSS_ONLINE: VDS_SUB_SYSTEM_STATUS = VDS_SUB_SYSTEM_STATUS(1i32);
pub const VDS_SSS_PARTIALLY_MANAGED: VDS_SUB_SYSTEM_STATUS = VDS_SUB_SYSTEM_STATUS(9i32);
pub const VDS_SSS_UNKNOWN: VDS_SUB_SYSTEM_STATUS = VDS_SUB_SYSTEM_STATUS(0i32);
pub const VDS_SVF_AUTO_MOUNT_OFF: VDS_SERVICE_FLAG = VDS_SERVICE_FLAG(32i32);
pub const VDS_SVF_CLUSTER_SERVICE_CONFIGURED: VDS_SERVICE_FLAG = VDS_SERVICE_FLAG(16i32);
pub const VDS_SVF_EFI: VDS_SERVICE_FLAG = VDS_SERVICE_FLAG(128i32);
pub const VDS_SVF_OS_UNINSTALL_VALID: VDS_SERVICE_FLAG = VDS_SERVICE_FLAG(64i32);
pub const VDS_SVF_SUPPORT_DYNAMIC: VDS_SERVICE_FLAG = VDS_SERVICE_FLAG(1i32);
pub const VDS_SVF_SUPPORT_DYNAMIC_1394: VDS_SERVICE_FLAG = VDS_SERVICE_FLAG(8i32);
pub const VDS_SVF_SUPPORT_FAULT_TOLERANT: VDS_SERVICE_FLAG = VDS_SERVICE_FLAG(2i32);
pub const VDS_SVF_SUPPORT_GPT: VDS_SERVICE_FLAG = VDS_SERVICE_FLAG(4i32);
pub const VDS_SVF_SUPPORT_MIRROR: VDS_SERVICE_FLAG = VDS_SERVICE_FLAG(256i32);
pub const VDS_SVF_SUPPORT_RAID5: VDS_SERVICE_FLAG = VDS_SERVICE_FLAG(512i32);
pub const VDS_SVF_SUPPORT_REFS: VDS_SERVICE_FLAG = VDS_SERVICE_FLAG(1024i32);
pub const VDS_S_ACCESS_PATH_NOT_DELETED: windows_core::HRESULT = windows_core::HRESULT(0x44244_u32 as _);
pub const VDS_S_ALREADY_EXISTS: windows_core::HRESULT = windows_core::HRESULT(0x42714_u32 as _);
pub const VDS_S_BOOT_PARTITION_NUMBER_CHANGE: windows_core::HRESULT = windows_core::HRESULT(0x42436_u32 as _);
pub const VDS_S_DEFAULT_PLEX_MEMBER_IDS: windows_core::HRESULT = windows_core::HRESULT(0x42518_u32 as _);
pub const VDS_S_DISK_DISMOUNT_FAILED: windows_core::HRESULT = windows_core::HRESULT(0x42809_u32 as _);
pub const VDS_S_DISK_IS_MISSING: windows_core::HRESULT = windows_core::HRESULT(0x42508_u32 as _);
pub const VDS_S_DISK_MOUNT_FAILED: windows_core::HRESULT = windows_core::HRESULT(0x42808_u32 as _);
pub const VDS_S_DISK_PARTIALLY_CLEANED: windows_core::HRESULT = windows_core::HRESULT(0x4241A_u32 as _);
pub const VDS_S_DISMOUNT_FAILED: windows_core::HRESULT = windows_core::HRESULT(0x42577_u32 as _);
pub const VDS_S_EXTEND_FILE_SYSTEM_FAILED: windows_core::HRESULT = windows_core::HRESULT(0x42465_u32 as _);
pub const VDS_S_FS_LOCK: windows_core::HRESULT = windows_core::HRESULT(0x42583_u32 as _);
pub const VDS_S_GPT_BOOT_MIRRORED_TO_MBR: windows_core::HRESULT = windows_core::HRESULT(0x80042469_u32 as _);
pub const VDS_S_IA64_BOOT_MIRRORED_TO_MBR: windows_core::HRESULT = windows_core::HRESULT(0x4245A_u32 as _);
pub const VDS_S_IN_PROGRESS: windows_core::HRESULT = windows_core::HRESULT(0x4244D_u32 as _);
pub const VDS_S_ISCSI_LOGIN_ALREAD_EXISTS: windows_core::HRESULT = windows_core::HRESULT(0x42802_u32 as _);
pub const VDS_S_ISCSI_PERSISTENT_LOGIN_MAY_NOT_BE_REMOVED: windows_core::HRESULT = windows_core::HRESULT(0x42801_u32 as _);
pub const VDS_S_ISCSI_SESSION_NOT_FOUND_PERSISTENT_LOGIN_REMOVED: windows_core::HRESULT = windows_core::HRESULT(0x42800_u32 as _);
pub const VDS_S_MBR_BOOT_MIRRORED_TO_GPT: windows_core::HRESULT = windows_core::HRESULT(0x42467_u32 as _);
pub const VDS_S_NAME_TRUNCATED: windows_core::HRESULT = windows_core::HRESULT(0x42700_u32 as _);
pub const VDS_S_NONCONFORMANT_PARTITION_INFO: windows_core::HRESULT = windows_core::HRESULT(0x4250A_u32 as _);
pub const VDS_S_NO_NOTIFICATION: windows_core::HRESULT = windows_core::HRESULT(0x42517_u32 as _);
pub const VDS_S_PLEX_NOT_LOADED_TO_CACHE: windows_core::HRESULT = windows_core::HRESULT(0x4258B_u32 as _);
pub const VDS_S_PROPERTIES_INCOMPLETE: windows_core::HRESULT = windows_core::HRESULT(0x42715_u32 as _);
pub const VDS_S_PROVIDER_ERROR_LOADING_CACHE: windows_core::HRESULT = windows_core::HRESULT(0x42421_u32 as _);
pub const VDS_S_REMOUNT_FAILED: windows_core::HRESULT = windows_core::HRESULT(0x42578_u32 as _);
pub const VDS_S_RESYNC_NOTIFICATION_TASK_FAILED: windows_core::HRESULT = windows_core::HRESULT(0x4257A_u32 as _);
pub const VDS_S_STATUSES_INCOMPLETELY_SET: windows_core::HRESULT = windows_core::HRESULT(0x42702_u32 as _);
pub const VDS_S_SYSTEM_PARTITION: windows_core::HRESULT = windows_core::HRESULT(0x4250E_u32 as _);
pub const VDS_S_UNABLE_TO_GET_GPT_ATTRIBUTES: windows_core::HRESULT = windows_core::HRESULT(0x4245B_u32 as _);
pub const VDS_S_UPDATE_BOOTFILE_FAILED: windows_core::HRESULT = windows_core::HRESULT(0x42434_u32 as _);
pub const VDS_S_VOLUME_COMPRESS_FAILED: windows_core::HRESULT = windows_core::HRESULT(0x42443_u32 as _);
pub const VDS_S_VSS_FLUSH_AND_HOLD_WRITES: windows_core::HRESULT = windows_core::HRESULT(0x42581_u32 as _);
pub const VDS_S_VSS_RELEASE_WRITES: windows_core::HRESULT = windows_core::HRESULT(0x42582_u32 as _);
pub const VDS_S_WINPE_BOOTENTRY: windows_core::HRESULT = windows_core::HRESULT(0x4258E_u32 as _);
pub const VDS_TS_EXTENDING: VDS_TRANSITION_STATE = VDS_TRANSITION_STATE(2i32);
pub const VDS_TS_RECONFIGING: VDS_TRANSITION_STATE = VDS_TRANSITION_STATE(4i32);
pub const VDS_TS_RESTRIPING: VDS_TRANSITION_STATE = VDS_TRANSITION_STATE(5i32);
pub const VDS_TS_SHRINKING: VDS_TRANSITION_STATE = VDS_TRANSITION_STATE(3i32);
pub const VDS_TS_STABLE: VDS_TRANSITION_STATE = VDS_TRANSITION_STATE(1i32);
pub const VDS_TS_UNKNOWN: VDS_TRANSITION_STATE = VDS_TRANSITION_STATE(0i32);
pub const VDS_VF_ACTIVE: VDS_VOLUME_FLAG = VDS_VOLUME_FLAG(4i32);
pub const VDS_VF_BACKED_BY_WIM_IMAGE: VDS_VOLUME_FLAG = VDS_VOLUME_FLAG(33554432i32);
pub const VDS_VF_BACKS_BOOT_VOLUME: VDS_VOLUME_FLAG = VDS_VOLUME_FLAG(16777216i32);
pub const VDS_VF_BOOT_VOLUME: VDS_VOLUME_FLAG = VDS_VOLUME_FLAG(2i32);
pub const VDS_VF_CAN_EXTEND: VDS_VOLUME_FLAG = VDS_VOLUME_FLAG(32i32);
pub const VDS_VF_CAN_SHRINK: VDS_VOLUME_FLAG = VDS_VOLUME_FLAG(64i32);
pub const VDS_VF_CRASHDUMP: VDS_VOLUME_FLAG = VDS_VOLUME_FLAG(512i32);
pub const VDS_VF_DIRTY: VDS_VOLUME_FLAG = VDS_VOLUME_FLAG(4194304i32);
pub const VDS_VF_FAT32_NOT_SUPPORTED: VDS_VOLUME_FLAG = VDS_VOLUME_FLAG(32768i32);
pub const VDS_VF_FAT_NOT_SUPPORTED: VDS_VOLUME_FLAG = VDS_VOLUME_FLAG(65536i32);
pub const VDS_VF_FORMATTING: VDS_VOLUME_FLAG = VDS_VOLUME_FLAG(4096i32);
pub const VDS_VF_FVE_ENABLED: VDS_VOLUME_FLAG = VDS_VOLUME_FLAG(2097152i32);
pub const VDS_VF_HIBERNATION: VDS_VOLUME_FLAG = VDS_VOLUME_FLAG(256i32);
pub const VDS_VF_HIDDEN: VDS_VOLUME_FLAG = VDS_VOLUME_FLAG(16i32);
pub const VDS_VF_INSTALLABLE: VDS_VOLUME_FLAG = VDS_VOLUME_FLAG(1024i32);
pub const VDS_VF_LBN_REMAP_ENABLED: VDS_VOLUME_FLAG = VDS_VOLUME_FLAG(2048i32);
pub const VDS_VF_NOT_FORMATTABLE: VDS_VOLUME_FLAG = VDS_VOLUME_FLAG(8192i32);
pub const VDS_VF_NO_DEFAULT_DRIVE_LETTER: VDS_VOLUME_FLAG = VDS_VOLUME_FLAG(131072i32);
pub const VDS_VF_NTFS_NOT_SUPPORTED: VDS_VOLUME_FLAG = VDS_VOLUME_FLAG(16384i32);
pub const VDS_VF_PAGEFILE: VDS_VOLUME_FLAG = VDS_VOLUME_FLAG(128i32);
pub const VDS_VF_PERMANENTLY_DISMOUNTED: VDS_VOLUME_FLAG = VDS_VOLUME_FLAG(262144i32);
pub const VDS_VF_PERMANENT_DISMOUNT_SUPPORTED: VDS_VOLUME_FLAG = VDS_VOLUME_FLAG(524288i32);
pub const VDS_VF_READONLY: VDS_VOLUME_FLAG = VDS_VOLUME_FLAG(8i32);
pub const VDS_VF_REFS_NOT_SUPPORTED: VDS_VOLUME_FLAG = VDS_VOLUME_FLAG(8388608i32);
pub const VDS_VF_SHADOW_COPY: VDS_VOLUME_FLAG = VDS_VOLUME_FLAG(1048576i32);
pub const VDS_VF_SYSTEM_VOLUME: VDS_VOLUME_FLAG = VDS_VOLUME_FLAG(1i32);
pub const VDS_VPS_FAILED: VDS_VOLUME_PLEX_STATUS = VDS_VOLUME_PLEX_STATUS(5i32);
pub const VDS_VPS_NO_MEDIA: VDS_VOLUME_PLEX_STATUS = VDS_VOLUME_PLEX_STATUS(3i32);
pub const VDS_VPS_ONLINE: VDS_VOLUME_PLEX_STATUS = VDS_VOLUME_PLEX_STATUS(1i32);
pub const VDS_VPS_UNKNOWN: VDS_VOLUME_PLEX_STATUS = VDS_VOLUME_PLEX_STATUS(0i32);
pub const VDS_VPT_PARITY: VDS_VOLUME_PLEX_TYPE = VDS_VOLUME_PLEX_TYPE(14i32);
pub const VDS_VPT_SIMPLE: VDS_VOLUME_PLEX_TYPE = VDS_VOLUME_PLEX_TYPE(10i32);
pub const VDS_VPT_SPAN: VDS_VOLUME_PLEX_TYPE = VDS_VOLUME_PLEX_TYPE(11i32);
pub const VDS_VPT_STRIPE: VDS_VOLUME_PLEX_TYPE = VDS_VOLUME_PLEX_TYPE(12i32);
pub const VDS_VPT_UNKNOWN: VDS_VOLUME_PLEX_TYPE = VDS_VOLUME_PLEX_TYPE(0i32);
pub const VDS_VSF_1_0: VDS_VERSION_SUPPORT_FLAG = VDS_VERSION_SUPPORT_FLAG(1i32);
pub const VDS_VSF_1_1: VDS_VERSION_SUPPORT_FLAG = VDS_VERSION_SUPPORT_FLAG(2i32);
pub const VDS_VSF_2_0: VDS_VERSION_SUPPORT_FLAG = VDS_VERSION_SUPPORT_FLAG(4i32);
pub const VDS_VSF_2_1: VDS_VERSION_SUPPORT_FLAG = VDS_VERSION_SUPPORT_FLAG(8i32);
pub const VDS_VSF_3_0: VDS_VERSION_SUPPORT_FLAG = VDS_VERSION_SUPPORT_FLAG(16i32);
pub const VDS_VST_ADDED: VDS_VDISK_STATE = VDS_VDISK_STATE(1i32);
pub const VDS_VST_ATTACHED: VDS_VDISK_STATE = VDS_VDISK_STATE(5i32);
pub const VDS_VST_ATTACHED_NOT_OPEN: VDS_VDISK_STATE = VDS_VDISK_STATE(4i32);
pub const VDS_VST_ATTACH_PENDING: VDS_VDISK_STATE = VDS_VDISK_STATE(3i32);
pub const VDS_VST_COMPACTING: VDS_VDISK_STATE = VDS_VDISK_STATE(7i32);
pub const VDS_VST_DELETED: VDS_VDISK_STATE = VDS_VDISK_STATE(10i32);
pub const VDS_VST_DETACH_PENDING: VDS_VDISK_STATE = VDS_VDISK_STATE(6i32);
pub const VDS_VST_EXPANDING: VDS_VDISK_STATE = VDS_VDISK_STATE(9i32);
pub const VDS_VST_MAX: VDS_VDISK_STATE = VDS_VDISK_STATE(11i32);
pub const VDS_VST_MERGING: VDS_VDISK_STATE = VDS_VDISK_STATE(8i32);
pub const VDS_VST_OPEN: VDS_VDISK_STATE = VDS_VDISK_STATE(2i32);
pub const VDS_VST_UNKNOWN: VDS_VDISK_STATE = VDS_VDISK_STATE(0i32);
pub const VDS_VS_FAILED: VDS_VOLUME_STATUS = VDS_VOLUME_STATUS(5i32);
pub const VDS_VS_NO_MEDIA: VDS_VOLUME_STATUS = VDS_VOLUME_STATUS(3i32);
pub const VDS_VS_OFFLINE: VDS_VOLUME_STATUS = VDS_VOLUME_STATUS(4i32);
pub const VDS_VS_ONLINE: VDS_VOLUME_STATUS = VDS_VOLUME_STATUS(1i32);
pub const VDS_VS_UNKNOWN: VDS_VOLUME_STATUS = VDS_VOLUME_STATUS(0i32);
pub const VDS_VT_MIRROR: VDS_VOLUME_TYPE = VDS_VOLUME_TYPE(13i32);
pub const VDS_VT_PARITY: VDS_VOLUME_TYPE = VDS_VOLUME_TYPE(14i32);
pub const VDS_VT_SIMPLE: VDS_VOLUME_TYPE = VDS_VOLUME_TYPE(10i32);
pub const VDS_VT_SPAN: VDS_VOLUME_TYPE = VDS_VOLUME_TYPE(11i32);
pub const VDS_VT_STRIPE: VDS_VOLUME_TYPE = VDS_VOLUME_TYPE(12i32);
pub const VDS_VT_UNKNOWN: VDS_VOLUME_TYPE = VDS_VOLUME_TYPE(0i32);
pub const VER_VDS_LUN_INFORMATION: u32 = 1u32;
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct VDS_ASYNC_OUTPUT_TYPE(pub i32);
impl windows_core::TypeKind for VDS_ASYNC_OUTPUT_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for VDS_ASYNC_OUTPUT_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("VDS_ASYNC_OUTPUT_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct VDS_CONTROLLER_STATUS(pub i32);
impl windows_core::TypeKind for VDS_CONTROLLER_STATUS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for VDS_CONTROLLER_STATUS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("VDS_CONTROLLER_STATUS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct VDS_DISK_EXTENT_TYPE(pub i32);
impl windows_core::TypeKind for VDS_DISK_EXTENT_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for VDS_DISK_EXTENT_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("VDS_DISK_EXTENT_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct VDS_DISK_FLAG(pub i32);
impl windows_core::TypeKind for VDS_DISK_FLAG {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for VDS_DISK_FLAG {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("VDS_DISK_FLAG").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct VDS_DISK_OFFLINE_REASON(pub i32);
impl windows_core::TypeKind for VDS_DISK_OFFLINE_REASON {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for VDS_DISK_OFFLINE_REASON {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("VDS_DISK_OFFLINE_REASON").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct VDS_DISK_STATUS(pub i32);
impl windows_core::TypeKind for VDS_DISK_STATUS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for VDS_DISK_STATUS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("VDS_DISK_STATUS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct VDS_DRIVE_FLAG(pub i32);
impl windows_core::TypeKind for VDS_DRIVE_FLAG {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for VDS_DRIVE_FLAG {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("VDS_DRIVE_FLAG").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct VDS_DRIVE_LETTER_FLAG(pub i32);
impl windows_core::TypeKind for VDS_DRIVE_LETTER_FLAG {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for VDS_DRIVE_LETTER_FLAG {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("VDS_DRIVE_LETTER_FLAG").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct VDS_DRIVE_STATUS(pub i32);
impl windows_core::TypeKind for VDS_DRIVE_STATUS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for VDS_DRIVE_STATUS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("VDS_DRIVE_STATUS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct VDS_FILE_SYSTEM_FLAG(pub i32);
impl windows_core::TypeKind for VDS_FILE_SYSTEM_FLAG {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for VDS_FILE_SYSTEM_FLAG {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("VDS_FILE_SYSTEM_FLAG").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct VDS_FILE_SYSTEM_FORMAT_SUPPORT_FLAG(pub i32);
impl windows_core::TypeKind for VDS_FILE_SYSTEM_FORMAT_SUPPORT_FLAG {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for VDS_FILE_SYSTEM_FORMAT_SUPPORT_FLAG {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("VDS_FILE_SYSTEM_FORMAT_SUPPORT_FLAG").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct VDS_FILE_SYSTEM_PROP_FLAG(pub i32);
impl windows_core::TypeKind for VDS_FILE_SYSTEM_PROP_FLAG {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for VDS_FILE_SYSTEM_PROP_FLAG {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("VDS_FILE_SYSTEM_PROP_FLAG").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct VDS_FILE_SYSTEM_TYPE(pub i32);
impl windows_core::TypeKind for VDS_FILE_SYSTEM_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for VDS_FILE_SYSTEM_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("VDS_FILE_SYSTEM_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct VDS_FORMAT_OPTION_FLAGS(pub i32);
impl windows_core::TypeKind for VDS_FORMAT_OPTION_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for VDS_FORMAT_OPTION_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("VDS_FORMAT_OPTION_FLAGS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct VDS_HBAPORT_SPEED_FLAG(pub i32);
impl windows_core::TypeKind for VDS_HBAPORT_SPEED_FLAG {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for VDS_HBAPORT_SPEED_FLAG {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("VDS_HBAPORT_SPEED_FLAG").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct VDS_HBAPORT_STATUS(pub i32);
impl windows_core::TypeKind for VDS_HBAPORT_STATUS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for VDS_HBAPORT_STATUS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("VDS_HBAPORT_STATUS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct VDS_HBAPORT_TYPE(pub i32);
impl windows_core::TypeKind for VDS_HBAPORT_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for VDS_HBAPORT_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("VDS_HBAPORT_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct VDS_HEALTH(pub i32);
impl windows_core::TypeKind for VDS_HEALTH {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for VDS_HEALTH {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("VDS_HEALTH").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct VDS_HWPROVIDER_TYPE(pub i32);
impl windows_core::TypeKind for VDS_HWPROVIDER_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for VDS_HWPROVIDER_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("VDS_HWPROVIDER_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct VDS_INTERCONNECT_ADDRESS_TYPE(pub i32);
impl windows_core::TypeKind for VDS_INTERCONNECT_ADDRESS_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for VDS_INTERCONNECT_ADDRESS_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("VDS_INTERCONNECT_ADDRESS_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct VDS_INTERCONNECT_FLAG(pub i32);
impl windows_core::TypeKind for VDS_INTERCONNECT_FLAG {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for VDS_INTERCONNECT_FLAG {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("VDS_INTERCONNECT_FLAG").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct VDS_IPADDRESS_TYPE(pub i32);
impl windows_core::TypeKind for VDS_IPADDRESS_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for VDS_IPADDRESS_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("VDS_IPADDRESS_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct VDS_ISCSI_AUTH_TYPE(pub i32);
impl windows_core::TypeKind for VDS_ISCSI_AUTH_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for VDS_ISCSI_AUTH_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("VDS_ISCSI_AUTH_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct VDS_ISCSI_IPSEC_FLAG(pub i32);
impl windows_core::TypeKind for VDS_ISCSI_IPSEC_FLAG {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for VDS_ISCSI_IPSEC_FLAG {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("VDS_ISCSI_IPSEC_FLAG").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct VDS_ISCSI_LOGIN_FLAG(pub i32);
impl windows_core::TypeKind for VDS_ISCSI_LOGIN_FLAG {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for VDS_ISCSI_LOGIN_FLAG {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("VDS_ISCSI_LOGIN_FLAG").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct VDS_ISCSI_LOGIN_TYPE(pub i32);
impl windows_core::TypeKind for VDS_ISCSI_LOGIN_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for VDS_ISCSI_LOGIN_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("VDS_ISCSI_LOGIN_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct VDS_ISCSI_PORTAL_STATUS(pub i32);
impl windows_core::TypeKind for VDS_ISCSI_PORTAL_STATUS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for VDS_ISCSI_PORTAL_STATUS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("VDS_ISCSI_PORTAL_STATUS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct VDS_LOADBALANCE_POLICY_ENUM(pub i32);
impl windows_core::TypeKind for VDS_LOADBALANCE_POLICY_ENUM {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for VDS_LOADBALANCE_POLICY_ENUM {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("VDS_LOADBALANCE_POLICY_ENUM").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct VDS_LUN_FLAG(pub i32);
impl windows_core::TypeKind for VDS_LUN_FLAG {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for VDS_LUN_FLAG {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("VDS_LUN_FLAG").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct VDS_LUN_PLEX_FLAG(pub i32);
impl windows_core::TypeKind for VDS_LUN_PLEX_FLAG {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for VDS_LUN_PLEX_FLAG {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("VDS_LUN_PLEX_FLAG").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct VDS_LUN_PLEX_STATUS(pub i32);
impl windows_core::TypeKind for VDS_LUN_PLEX_STATUS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for VDS_LUN_PLEX_STATUS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("VDS_LUN_PLEX_STATUS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct VDS_LUN_PLEX_TYPE(pub i32);
impl windows_core::TypeKind for VDS_LUN_PLEX_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for VDS_LUN_PLEX_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("VDS_LUN_PLEX_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct VDS_LUN_RESERVE_MODE(pub i32);
impl windows_core::TypeKind for VDS_LUN_RESERVE_MODE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for VDS_LUN_RESERVE_MODE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("VDS_LUN_RESERVE_MODE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct VDS_LUN_STATUS(pub i32);
impl windows_core::TypeKind for VDS_LUN_STATUS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for VDS_LUN_STATUS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("VDS_LUN_STATUS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct VDS_LUN_TYPE(pub i32);
impl windows_core::TypeKind for VDS_LUN_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for VDS_LUN_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("VDS_LUN_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct VDS_MAINTENANCE_OPERATION(pub i32);
impl windows_core::TypeKind for VDS_MAINTENANCE_OPERATION {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for VDS_MAINTENANCE_OPERATION {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("VDS_MAINTENANCE_OPERATION").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct VDS_NF_CONTROLLER(pub u32);
impl windows_core::TypeKind for VDS_NF_CONTROLLER {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for VDS_NF_CONTROLLER {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("VDS_NF_CONTROLLER").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct VDS_NF_DISK(pub u32);
impl windows_core::TypeKind for VDS_NF_DISK {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for VDS_NF_DISK {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("VDS_NF_DISK").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct VDS_NF_DRIVE(pub u32);
impl windows_core::TypeKind for VDS_NF_DRIVE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for VDS_NF_DRIVE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("VDS_NF_DRIVE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct VDS_NF_FILE_SYSTEM(pub u32);
impl windows_core::TypeKind for VDS_NF_FILE_SYSTEM {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for VDS_NF_FILE_SYSTEM {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("VDS_NF_FILE_SYSTEM").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct VDS_NF_LUN(pub u32);
impl windows_core::TypeKind for VDS_NF_LUN {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for VDS_NF_LUN {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("VDS_NF_LUN").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct VDS_NF_PACK(pub u32);
impl windows_core::TypeKind for VDS_NF_PACK {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for VDS_NF_PACK {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("VDS_NF_PACK").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct VDS_NF_PORT(pub u32);
impl windows_core::TypeKind for VDS_NF_PORT {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for VDS_NF_PORT {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("VDS_NF_PORT").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct VDS_NOTIFICATION_TARGET_TYPE(pub i32);
impl windows_core::TypeKind for VDS_NOTIFICATION_TARGET_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for VDS_NOTIFICATION_TARGET_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("VDS_NOTIFICATION_TARGET_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct VDS_OBJECT_TYPE(pub i32);
impl windows_core::TypeKind for VDS_OBJECT_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for VDS_OBJECT_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("VDS_OBJECT_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct VDS_PACK_FLAG(pub i32);
impl windows_core::TypeKind for VDS_PACK_FLAG {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for VDS_PACK_FLAG {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("VDS_PACK_FLAG").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct VDS_PACK_STATUS(pub i32);
impl windows_core::TypeKind for VDS_PACK_STATUS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for VDS_PACK_STATUS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("VDS_PACK_STATUS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct VDS_PARTITION_FLAG(pub i32);
impl windows_core::TypeKind for VDS_PARTITION_FLAG {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for VDS_PARTITION_FLAG {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("VDS_PARTITION_FLAG").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct VDS_PARTITION_STYLE(pub i32);
impl windows_core::TypeKind for VDS_PARTITION_STYLE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for VDS_PARTITION_STYLE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("VDS_PARTITION_STYLE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct VDS_PATH_STATUS(pub i32);
impl windows_core::TypeKind for VDS_PATH_STATUS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for VDS_PATH_STATUS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("VDS_PATH_STATUS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct VDS_PORT_STATUS(pub i32);
impl windows_core::TypeKind for VDS_PORT_STATUS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for VDS_PORT_STATUS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("VDS_PORT_STATUS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct VDS_PROVIDER_FLAG(pub i32);
impl windows_core::TypeKind for VDS_PROVIDER_FLAG {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for VDS_PROVIDER_FLAG {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("VDS_PROVIDER_FLAG").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct VDS_PROVIDER_LBSUPPORT_FLAG(pub i32);
impl windows_core::TypeKind for VDS_PROVIDER_LBSUPPORT_FLAG {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for VDS_PROVIDER_LBSUPPORT_FLAG {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("VDS_PROVIDER_LBSUPPORT_FLAG").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct VDS_PROVIDER_TYPE(pub i32);
impl windows_core::TypeKind for VDS_PROVIDER_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for VDS_PROVIDER_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("VDS_PROVIDER_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct VDS_QUERY_PROVIDER_FLAG(pub i32);
impl windows_core::TypeKind for VDS_QUERY_PROVIDER_FLAG {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for VDS_QUERY_PROVIDER_FLAG {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("VDS_QUERY_PROVIDER_FLAG").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct VDS_RAID_TYPE(pub i32);
impl windows_core::TypeKind for VDS_RAID_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for VDS_RAID_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("VDS_RAID_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct VDS_RECOVER_ACTION(pub i32);
impl windows_core::TypeKind for VDS_RECOVER_ACTION {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for VDS_RECOVER_ACTION {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("VDS_RECOVER_ACTION").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct VDS_SAN_POLICY(pub i32);
impl windows_core::TypeKind for VDS_SAN_POLICY {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for VDS_SAN_POLICY {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("VDS_SAN_POLICY").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct VDS_SERVICE_FLAG(pub i32);
impl windows_core::TypeKind for VDS_SERVICE_FLAG {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for VDS_SERVICE_FLAG {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("VDS_SERVICE_FLAG").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct VDS_STORAGE_BUS_TYPE(pub i32);
impl windows_core::TypeKind for VDS_STORAGE_BUS_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for VDS_STORAGE_BUS_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("VDS_STORAGE_BUS_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct VDS_STORAGE_IDENTIFIER_CODE_SET(pub i32);
impl windows_core::TypeKind for VDS_STORAGE_IDENTIFIER_CODE_SET {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for VDS_STORAGE_IDENTIFIER_CODE_SET {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("VDS_STORAGE_IDENTIFIER_CODE_SET").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct VDS_STORAGE_IDENTIFIER_TYPE(pub i32);
impl windows_core::TypeKind for VDS_STORAGE_IDENTIFIER_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for VDS_STORAGE_IDENTIFIER_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("VDS_STORAGE_IDENTIFIER_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct VDS_STORAGE_POOL_STATUS(pub i32);
impl windows_core::TypeKind for VDS_STORAGE_POOL_STATUS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for VDS_STORAGE_POOL_STATUS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("VDS_STORAGE_POOL_STATUS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct VDS_STORAGE_POOL_TYPE(pub i32);
impl windows_core::TypeKind for VDS_STORAGE_POOL_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for VDS_STORAGE_POOL_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("VDS_STORAGE_POOL_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct VDS_SUB_SYSTEM_FLAG(pub i32);
impl windows_core::TypeKind for VDS_SUB_SYSTEM_FLAG {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for VDS_SUB_SYSTEM_FLAG {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("VDS_SUB_SYSTEM_FLAG").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct VDS_SUB_SYSTEM_STATUS(pub i32);
impl windows_core::TypeKind for VDS_SUB_SYSTEM_STATUS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for VDS_SUB_SYSTEM_STATUS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("VDS_SUB_SYSTEM_STATUS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct VDS_SUB_SYSTEM_SUPPORTED_RAID_TYPE_FLAG(pub i32);
impl windows_core::TypeKind for VDS_SUB_SYSTEM_SUPPORTED_RAID_TYPE_FLAG {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for VDS_SUB_SYSTEM_SUPPORTED_RAID_TYPE_FLAG {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("VDS_SUB_SYSTEM_SUPPORTED_RAID_TYPE_FLAG").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct VDS_TRANSITION_STATE(pub i32);
impl windows_core::TypeKind for VDS_TRANSITION_STATE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for VDS_TRANSITION_STATE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("VDS_TRANSITION_STATE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct VDS_VDISK_STATE(pub i32);
impl windows_core::TypeKind for VDS_VDISK_STATE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for VDS_VDISK_STATE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("VDS_VDISK_STATE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct VDS_VERSION_SUPPORT_FLAG(pub i32);
impl windows_core::TypeKind for VDS_VERSION_SUPPORT_FLAG {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for VDS_VERSION_SUPPORT_FLAG {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("VDS_VERSION_SUPPORT_FLAG").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct VDS_VOLUME_FLAG(pub i32);
impl windows_core::TypeKind for VDS_VOLUME_FLAG {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for VDS_VOLUME_FLAG {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("VDS_VOLUME_FLAG").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct VDS_VOLUME_PLEX_STATUS(pub i32);
impl windows_core::TypeKind for VDS_VOLUME_PLEX_STATUS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for VDS_VOLUME_PLEX_STATUS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("VDS_VOLUME_PLEX_STATUS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct VDS_VOLUME_PLEX_TYPE(pub i32);
impl windows_core::TypeKind for VDS_VOLUME_PLEX_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for VDS_VOLUME_PLEX_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("VDS_VOLUME_PLEX_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct VDS_VOLUME_STATUS(pub i32);
impl windows_core::TypeKind for VDS_VOLUME_STATUS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for VDS_VOLUME_STATUS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("VDS_VOLUME_STATUS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct VDS_VOLUME_TYPE(pub i32);
impl windows_core::TypeKind for VDS_VOLUME_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for VDS_VOLUME_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("VDS_VOLUME_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct __VDS_PARTITION_STYLE(pub i32);
impl windows_core::TypeKind for __VDS_PARTITION_STYLE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for __VDS_PARTITION_STYLE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("__VDS_PARTITION_STYLE").field(&self.0).finish()
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CHANGE_ATTRIBUTES_PARAMETERS {
    pub style: VDS_PARTITION_STYLE,
    pub Anonymous: CHANGE_ATTRIBUTES_PARAMETERS_0,
}
impl windows_core::TypeKind for CHANGE_ATTRIBUTES_PARAMETERS {
    type TypeKind = windows_core::CopyType;
}
impl Default for CHANGE_ATTRIBUTES_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union CHANGE_ATTRIBUTES_PARAMETERS_0 {
    pub MbrPartInfo: CHANGE_ATTRIBUTES_PARAMETERS_0_1,
    pub GptPartInfo: CHANGE_ATTRIBUTES_PARAMETERS_0_0,
}
impl windows_core::TypeKind for CHANGE_ATTRIBUTES_PARAMETERS_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for CHANGE_ATTRIBUTES_PARAMETERS_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CHANGE_ATTRIBUTES_PARAMETERS_0_0 {
    pub attributes: u64,
}
impl windows_core::TypeKind for CHANGE_ATTRIBUTES_PARAMETERS_0_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for CHANGE_ATTRIBUTES_PARAMETERS_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CHANGE_ATTRIBUTES_PARAMETERS_0_1 {
    pub bootIndicator: super::super::Foundation::BOOLEAN,
}
impl windows_core::TypeKind for CHANGE_ATTRIBUTES_PARAMETERS_0_1 {
    type TypeKind = windows_core::CopyType;
}
impl Default for CHANGE_ATTRIBUTES_PARAMETERS_0_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CHANGE_PARTITION_TYPE_PARAMETERS {
    pub style: VDS_PARTITION_STYLE,
    pub Anonymous: CHANGE_PARTITION_TYPE_PARAMETERS_0,
}
impl windows_core::TypeKind for CHANGE_PARTITION_TYPE_PARAMETERS {
    type TypeKind = windows_core::CopyType;
}
impl Default for CHANGE_PARTITION_TYPE_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union CHANGE_PARTITION_TYPE_PARAMETERS_0 {
    pub MbrPartInfo: CHANGE_PARTITION_TYPE_PARAMETERS_0_1,
    pub GptPartInfo: CHANGE_PARTITION_TYPE_PARAMETERS_0_0,
}
impl windows_core::TypeKind for CHANGE_PARTITION_TYPE_PARAMETERS_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for CHANGE_PARTITION_TYPE_PARAMETERS_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CHANGE_PARTITION_TYPE_PARAMETERS_0_0 {
    pub partitionType: windows_core::GUID,
}
impl windows_core::TypeKind for CHANGE_PARTITION_TYPE_PARAMETERS_0_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for CHANGE_PARTITION_TYPE_PARAMETERS_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CHANGE_PARTITION_TYPE_PARAMETERS_0_1 {
    pub partitionType: u8,
}
impl windows_core::TypeKind for CHANGE_PARTITION_TYPE_PARAMETERS_0_1 {
    type TypeKind = windows_core::CopyType;
}
impl Default for CHANGE_PARTITION_TYPE_PARAMETERS_0_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CREATE_PARTITION_PARAMETERS {
    pub style: VDS_PARTITION_STYLE,
    pub Anonymous: CREATE_PARTITION_PARAMETERS_0,
}
impl windows_core::TypeKind for CREATE_PARTITION_PARAMETERS {
    type TypeKind = windows_core::CopyType;
}
impl Default for CREATE_PARTITION_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union CREATE_PARTITION_PARAMETERS_0 {
    pub MbrPartInfo: CREATE_PARTITION_PARAMETERS_0_1,
    pub GptPartInfo: CREATE_PARTITION_PARAMETERS_0_0,
}
impl windows_core::TypeKind for CREATE_PARTITION_PARAMETERS_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for CREATE_PARTITION_PARAMETERS_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CREATE_PARTITION_PARAMETERS_0_0 {
    pub partitionType: windows_core::GUID,
    pub partitionId: windows_core::GUID,
    pub attributes: u64,
    pub name: [u16; 36],
}
impl windows_core::TypeKind for CREATE_PARTITION_PARAMETERS_0_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for CREATE_PARTITION_PARAMETERS_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CREATE_PARTITION_PARAMETERS_0_1 {
    pub partitionType: u8,
    pub bootIndicator: super::super::Foundation::BOOLEAN,
}
impl windows_core::TypeKind for CREATE_PARTITION_PARAMETERS_0_1 {
    type TypeKind = windows_core::CopyType;
}
impl Default for CREATE_PARTITION_PARAMETERS_0_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct VDS_ADVANCEDDISK_PROP {
    pub pwszId: windows_core::PWSTR,
    pub pwszPathname: windows_core::PWSTR,
    pub pwszLocation: windows_core::PWSTR,
    pub pwszFriendlyName: windows_core::PWSTR,
    pub pswzIdentifier: windows_core::PWSTR,
    pub usIdentifierFormat: u16,
    pub ulNumber: u32,
    pub pwszSerialNumber: windows_core::PWSTR,
    pub pwszFirmwareVersion: windows_core::PWSTR,
    pub pwszManufacturer: windows_core::PWSTR,
    pub pwszModel: windows_core::PWSTR,
    pub ullTotalSize: u64,
    pub ullAllocatedSize: u64,
    pub ulLogicalSectorSize: u32,
    pub ulPhysicalSectorSize: u32,
    pub ulPartitionCount: u32,
    pub status: VDS_DISK_STATUS,
    pub health: VDS_HEALTH,
    pub BusType: VDS_STORAGE_BUS_TYPE,
    pub PartitionStyle: VDS_PARTITION_STYLE,
    pub Anonymous: VDS_ADVANCEDDISK_PROP_0,
    pub ulFlags: u32,
    pub dwDeviceType: u32,
}
impl windows_core::TypeKind for VDS_ADVANCEDDISK_PROP {
    type TypeKind = windows_core::CopyType;
}
impl Default for VDS_ADVANCEDDISK_PROP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union VDS_ADVANCEDDISK_PROP_0 {
    pub dwSignature: u32,
    pub DiskGuid: windows_core::GUID,
}
impl windows_core::TypeKind for VDS_ADVANCEDDISK_PROP_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for VDS_ADVANCEDDISK_PROP_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct VDS_ASYNC_OUTPUT {
    pub r#type: VDS_ASYNC_OUTPUT_TYPE,
    pub Anonymous: VDS_ASYNC_OUTPUT_0,
}
impl Clone for VDS_ASYNC_OUTPUT {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
impl windows_core::TypeKind for VDS_ASYNC_OUTPUT {
    type TypeKind = windows_core::CopyType;
}
impl Default for VDS_ASYNC_OUTPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub union VDS_ASYNC_OUTPUT_0 {
    pub cp: VDS_ASYNC_OUTPUT_0_2,
    pub cv: core::mem::ManuallyDrop<VDS_ASYNC_OUTPUT_0_5>,
    pub bvp: core::mem::ManuallyDrop<VDS_ASYNC_OUTPUT_0_0>,
    pub sv: VDS_ASYNC_OUTPUT_0_7,
    pub cl: core::mem::ManuallyDrop<VDS_ASYNC_OUTPUT_0_1>,
    pub ct: core::mem::ManuallyDrop<VDS_ASYNC_OUTPUT_0_4>,
    pub cpg: core::mem::ManuallyDrop<VDS_ASYNC_OUTPUT_0_3>,
    pub cvd: core::mem::ManuallyDrop<VDS_ASYNC_OUTPUT_0_6>,
}
impl Clone for VDS_ASYNC_OUTPUT_0 {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
impl windows_core::TypeKind for VDS_ASYNC_OUTPUT_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for VDS_ASYNC_OUTPUT_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Eq, PartialEq)]
pub struct VDS_ASYNC_OUTPUT_0_0 {
    pub pVolumeUnk: core::mem::ManuallyDrop<Option<windows_core::IUnknown>>,
}
impl Clone for VDS_ASYNC_OUTPUT_0_0 {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
impl windows_core::TypeKind for VDS_ASYNC_OUTPUT_0_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for VDS_ASYNC_OUTPUT_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Eq, PartialEq)]
pub struct VDS_ASYNC_OUTPUT_0_1 {
    pub pLunUnk: core::mem::ManuallyDrop<Option<windows_core::IUnknown>>,
}
impl Clone for VDS_ASYNC_OUTPUT_0_1 {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
impl windows_core::TypeKind for VDS_ASYNC_OUTPUT_0_1 {
    type TypeKind = windows_core::CopyType;
}
impl Default for VDS_ASYNC_OUTPUT_0_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VDS_ASYNC_OUTPUT_0_2 {
    pub ullOffset: u64,
    pub volumeId: windows_core::GUID,
}
impl windows_core::TypeKind for VDS_ASYNC_OUTPUT_0_2 {
    type TypeKind = windows_core::CopyType;
}
impl Default for VDS_ASYNC_OUTPUT_0_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Eq, PartialEq)]
pub struct VDS_ASYNC_OUTPUT_0_3 {
    pub pPortalGroupUnk: core::mem::ManuallyDrop<Option<windows_core::IUnknown>>,
}
impl Clone for VDS_ASYNC_OUTPUT_0_3 {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
impl windows_core::TypeKind for VDS_ASYNC_OUTPUT_0_3 {
    type TypeKind = windows_core::CopyType;
}
impl Default for VDS_ASYNC_OUTPUT_0_3 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Eq, PartialEq)]
pub struct VDS_ASYNC_OUTPUT_0_4 {
    pub pTargetUnk: core::mem::ManuallyDrop<Option<windows_core::IUnknown>>,
}
impl Clone for VDS_ASYNC_OUTPUT_0_4 {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
impl windows_core::TypeKind for VDS_ASYNC_OUTPUT_0_4 {
    type TypeKind = windows_core::CopyType;
}
impl Default for VDS_ASYNC_OUTPUT_0_4 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Eq, PartialEq)]
pub struct VDS_ASYNC_OUTPUT_0_5 {
    pub pVolumeUnk: core::mem::ManuallyDrop<Option<windows_core::IUnknown>>,
}
impl Clone for VDS_ASYNC_OUTPUT_0_5 {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
impl windows_core::TypeKind for VDS_ASYNC_OUTPUT_0_5 {
    type TypeKind = windows_core::CopyType;
}
impl Default for VDS_ASYNC_OUTPUT_0_5 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Eq, PartialEq)]
pub struct VDS_ASYNC_OUTPUT_0_6 {
    pub pVDiskUnk: core::mem::ManuallyDrop<Option<windows_core::IUnknown>>,
}
impl Clone for VDS_ASYNC_OUTPUT_0_6 {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
impl windows_core::TypeKind for VDS_ASYNC_OUTPUT_0_6 {
    type TypeKind = windows_core::CopyType;
}
impl Default for VDS_ASYNC_OUTPUT_0_6 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VDS_ASYNC_OUTPUT_0_7 {
    pub ullReclaimedBytes: u64,
}
impl windows_core::TypeKind for VDS_ASYNC_OUTPUT_0_7 {
    type TypeKind = windows_core::CopyType;
}
impl Default for VDS_ASYNC_OUTPUT_0_7 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VDS_CONTROLLER_NOTIFICATION {
    pub ulEvent: VDS_NF_CONTROLLER,
    pub controllerId: windows_core::GUID,
}
impl windows_core::TypeKind for VDS_CONTROLLER_NOTIFICATION {
    type TypeKind = windows_core::CopyType;
}
impl Default for VDS_CONTROLLER_NOTIFICATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VDS_CONTROLLER_PROP {
    pub id: windows_core::GUID,
    pub pwszFriendlyName: windows_core::PWSTR,
    pub pwszIdentification: windows_core::PWSTR,
    pub status: VDS_CONTROLLER_STATUS,
    pub health: VDS_HEALTH,
    pub sNumberOfPorts: i16,
}
impl windows_core::TypeKind for VDS_CONTROLLER_PROP {
    type TypeKind = windows_core::CopyType;
}
impl Default for VDS_CONTROLLER_PROP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VDS_CREATE_VDISK_PARAMETERS {
    pub UniqueId: windows_core::GUID,
    pub MaximumSize: u64,
    pub BlockSizeInBytes: u32,
    pub SectorSizeInBytes: u32,
    pub pParentPath: windows_core::PWSTR,
    pub pSourcePath: windows_core::PWSTR,
}
impl windows_core::TypeKind for VDS_CREATE_VDISK_PARAMETERS {
    type TypeKind = windows_core::CopyType;
}
impl Default for VDS_CREATE_VDISK_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VDS_DISK_EXTENT {
    pub diskId: windows_core::GUID,
    pub r#type: VDS_DISK_EXTENT_TYPE,
    pub ullOffset: u64,
    pub ullSize: u64,
    pub volumeId: windows_core::GUID,
    pub plexId: windows_core::GUID,
    pub memberIdx: u32,
}
impl windows_core::TypeKind for VDS_DISK_EXTENT {
    type TypeKind = windows_core::CopyType;
}
impl Default for VDS_DISK_EXTENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VDS_DISK_FREE_EXTENT {
    pub diskId: windows_core::GUID,
    pub ullOffset: u64,
    pub ullSize: u64,
}
impl windows_core::TypeKind for VDS_DISK_FREE_EXTENT {
    type TypeKind = windows_core::CopyType;
}
impl Default for VDS_DISK_FREE_EXTENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VDS_DISK_NOTIFICATION {
    pub ulEvent: VDS_NF_DISK,
    pub diskId: windows_core::GUID,
}
impl windows_core::TypeKind for VDS_DISK_NOTIFICATION {
    type TypeKind = windows_core::CopyType;
}
impl Default for VDS_DISK_NOTIFICATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct VDS_DISK_PROP {
    pub id: windows_core::GUID,
    pub status: VDS_DISK_STATUS,
    pub ReserveMode: VDS_LUN_RESERVE_MODE,
    pub health: VDS_HEALTH,
    pub dwDeviceType: u32,
    pub dwMediaType: u32,
    pub ullSize: u64,
    pub ulBytesPerSector: u32,
    pub ulSectorsPerTrack: u32,
    pub ulTracksPerCylinder: u32,
    pub ulFlags: u32,
    pub BusType: VDS_STORAGE_BUS_TYPE,
    pub PartitionStyle: VDS_PARTITION_STYLE,
    pub Anonymous: VDS_DISK_PROP_0,
    pub pwszDiskAddress: windows_core::PWSTR,
    pub pwszName: windows_core::PWSTR,
    pub pwszFriendlyName: windows_core::PWSTR,
    pub pwszAdaptorName: windows_core::PWSTR,
    pub pwszDevicePath: windows_core::PWSTR,
}
impl windows_core::TypeKind for VDS_DISK_PROP {
    type TypeKind = windows_core::CopyType;
}
impl Default for VDS_DISK_PROP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union VDS_DISK_PROP_0 {
    pub dwSignature: u32,
    pub DiskGuid: windows_core::GUID,
}
impl windows_core::TypeKind for VDS_DISK_PROP_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for VDS_DISK_PROP_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct VDS_DISK_PROP2 {
    pub id: windows_core::GUID,
    pub status: VDS_DISK_STATUS,
    pub OfflineReason: VDS_DISK_OFFLINE_REASON,
    pub ReserveMode: VDS_LUN_RESERVE_MODE,
    pub health: VDS_HEALTH,
    pub dwDeviceType: u32,
    pub dwMediaType: u32,
    pub ullSize: u64,
    pub ulBytesPerSector: u32,
    pub ulSectorsPerTrack: u32,
    pub ulTracksPerCylinder: u32,
    pub ulFlags: u32,
    pub BusType: VDS_STORAGE_BUS_TYPE,
    pub PartitionStyle: VDS_PARTITION_STYLE,
    pub Anonymous: VDS_DISK_PROP2_0,
    pub pwszDiskAddress: windows_core::PWSTR,
    pub pwszName: windows_core::PWSTR,
    pub pwszFriendlyName: windows_core::PWSTR,
    pub pwszAdaptorName: windows_core::PWSTR,
    pub pwszDevicePath: windows_core::PWSTR,
    pub pwszLocationPath: windows_core::PWSTR,
}
impl windows_core::TypeKind for VDS_DISK_PROP2 {
    type TypeKind = windows_core::CopyType;
}
impl Default for VDS_DISK_PROP2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union VDS_DISK_PROP2_0 {
    pub dwSignature: u32,
    pub DiskGuid: windows_core::GUID,
}
impl windows_core::TypeKind for VDS_DISK_PROP2_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for VDS_DISK_PROP2_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VDS_DRIVE_EXTENT {
    pub id: windows_core::GUID,
    pub LunId: windows_core::GUID,
    pub ullSize: u64,
    pub bUsed: super::super::Foundation::BOOL,
}
impl windows_core::TypeKind for VDS_DRIVE_EXTENT {
    type TypeKind = windows_core::CopyType;
}
impl Default for VDS_DRIVE_EXTENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VDS_DRIVE_LETTER_NOTIFICATION {
    pub ulEvent: u32,
    pub wcLetter: u16,
    pub volumeId: windows_core::GUID,
}
impl windows_core::TypeKind for VDS_DRIVE_LETTER_NOTIFICATION {
    type TypeKind = windows_core::CopyType;
}
impl Default for VDS_DRIVE_LETTER_NOTIFICATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VDS_DRIVE_LETTER_PROP {
    pub wcLetter: u16,
    pub volumeId: windows_core::GUID,
    pub ulFlags: u32,
    pub bUsed: super::super::Foundation::BOOL,
}
impl windows_core::TypeKind for VDS_DRIVE_LETTER_PROP {
    type TypeKind = windows_core::CopyType;
}
impl Default for VDS_DRIVE_LETTER_PROP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VDS_DRIVE_NOTIFICATION {
    pub ulEvent: VDS_NF_DRIVE,
    pub driveId: windows_core::GUID,
}
impl windows_core::TypeKind for VDS_DRIVE_NOTIFICATION {
    type TypeKind = windows_core::CopyType;
}
impl Default for VDS_DRIVE_NOTIFICATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VDS_DRIVE_PROP {
    pub id: windows_core::GUID,
    pub ullSize: u64,
    pub pwszFriendlyName: windows_core::PWSTR,
    pub pwszIdentification: windows_core::PWSTR,
    pub ulFlags: u32,
    pub status: VDS_DRIVE_STATUS,
    pub health: VDS_HEALTH,
    pub sInternalBusNumber: i16,
    pub sSlotNumber: i16,
}
impl windows_core::TypeKind for VDS_DRIVE_PROP {
    type TypeKind = windows_core::CopyType;
}
impl Default for VDS_DRIVE_PROP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VDS_DRIVE_PROP2 {
    pub id: windows_core::GUID,
    pub ullSize: u64,
    pub pwszFriendlyName: windows_core::PWSTR,
    pub pwszIdentification: windows_core::PWSTR,
    pub ulFlags: u32,
    pub status: VDS_DRIVE_STATUS,
    pub health: VDS_HEALTH,
    pub sInternalBusNumber: i16,
    pub sSlotNumber: i16,
    pub ulEnclosureNumber: u32,
    pub busType: VDS_STORAGE_BUS_TYPE,
    pub ulSpindleSpeed: u32,
}
impl windows_core::TypeKind for VDS_DRIVE_PROP2 {
    type TypeKind = windows_core::CopyType;
}
impl Default for VDS_DRIVE_PROP2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VDS_FILE_SYSTEM_FORMAT_SUPPORT_PROP {
    pub ulFlags: u32,
    pub usRevision: u16,
    pub ulDefaultUnitAllocationSize: u32,
    pub rgulAllowedUnitAllocationSizes: [u32; 32],
    pub wszName: [u16; 32],
}
impl windows_core::TypeKind for VDS_FILE_SYSTEM_FORMAT_SUPPORT_PROP {
    type TypeKind = windows_core::CopyType;
}
impl Default for VDS_FILE_SYSTEM_FORMAT_SUPPORT_PROP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VDS_FILE_SYSTEM_NOTIFICATION {
    pub ulEvent: VDS_NF_FILE_SYSTEM,
    pub volumeId: windows_core::GUID,
    pub dwPercentCompleted: u32,
}
impl windows_core::TypeKind for VDS_FILE_SYSTEM_NOTIFICATION {
    type TypeKind = windows_core::CopyType;
}
impl Default for VDS_FILE_SYSTEM_NOTIFICATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VDS_FILE_SYSTEM_PROP {
    pub r#type: VDS_FILE_SYSTEM_TYPE,
    pub volumeId: windows_core::GUID,
    pub ulFlags: u32,
    pub ullTotalAllocationUnits: u64,
    pub ullAvailableAllocationUnits: u64,
    pub ulAllocationUnitSize: u32,
    pub pwszLabel: windows_core::PWSTR,
}
impl windows_core::TypeKind for VDS_FILE_SYSTEM_PROP {
    type TypeKind = windows_core::CopyType;
}
impl Default for VDS_FILE_SYSTEM_PROP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VDS_FILE_SYSTEM_TYPE_PROP {
    pub r#type: VDS_FILE_SYSTEM_TYPE,
    pub wszName: [u16; 8],
    pub ulFlags: u32,
    pub ulCompressionFlags: u32,
    pub ulMaxLableLength: u32,
    pub pwszIllegalLabelCharSet: windows_core::PWSTR,
}
impl windows_core::TypeKind for VDS_FILE_SYSTEM_TYPE_PROP {
    type TypeKind = windows_core::CopyType;
}
impl Default for VDS_FILE_SYSTEM_TYPE_PROP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VDS_HBAPORT_PROP {
    pub id: windows_core::GUID,
    pub wwnNode: VDS_WWN,
    pub wwnPort: VDS_WWN,
    pub r#type: VDS_HBAPORT_TYPE,
    pub status: VDS_HBAPORT_STATUS,
    pub ulPortSpeed: u32,
    pub ulSupportedPortSpeed: u32,
}
impl windows_core::TypeKind for VDS_HBAPORT_PROP {
    type TypeKind = windows_core::CopyType;
}
impl Default for VDS_HBAPORT_PROP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VDS_HINTS {
    pub ullHintMask: u64,
    pub ullExpectedMaximumSize: u64,
    pub ulOptimalReadSize: u32,
    pub ulOptimalReadAlignment: u32,
    pub ulOptimalWriteSize: u32,
    pub ulOptimalWriteAlignment: u32,
    pub ulMaximumDriveCount: u32,
    pub ulStripeSize: u32,
    pub bFastCrashRecoveryRequired: super::super::Foundation::BOOL,
    pub bMostlyReads: super::super::Foundation::BOOL,
    pub bOptimizeForSequentialReads: super::super::Foundation::BOOL,
    pub bOptimizeForSequentialWrites: super::super::Foundation::BOOL,
    pub bRemapEnabled: super::super::Foundation::BOOL,
    pub bReadBackVerifyEnabled: super::super::Foundation::BOOL,
    pub bWriteThroughCachingEnabled: super::super::Foundation::BOOL,
    pub bHardwareChecksumEnabled: super::super::Foundation::BOOL,
    pub bIsYankable: super::super::Foundation::BOOL,
    pub sRebuildPriority: i16,
}
impl windows_core::TypeKind for VDS_HINTS {
    type TypeKind = windows_core::CopyType;
}
impl Default for VDS_HINTS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VDS_HINTS2 {
    pub ullHintMask: u64,
    pub ullExpectedMaximumSize: u64,
    pub ulOptimalReadSize: u32,
    pub ulOptimalReadAlignment: u32,
    pub ulOptimalWriteSize: u32,
    pub ulOptimalWriteAlignment: u32,
    pub ulMaximumDriveCount: u32,
    pub ulStripeSize: u32,
    pub ulReserved1: u32,
    pub ulReserved2: u32,
    pub ulReserved3: u32,
    pub bFastCrashRecoveryRequired: super::super::Foundation::BOOL,
    pub bMostlyReads: super::super::Foundation::BOOL,
    pub bOptimizeForSequentialReads: super::super::Foundation::BOOL,
    pub bOptimizeForSequentialWrites: super::super::Foundation::BOOL,
    pub bRemapEnabled: super::super::Foundation::BOOL,
    pub bReadBackVerifyEnabled: super::super::Foundation::BOOL,
    pub bWriteThroughCachingEnabled: super::super::Foundation::BOOL,
    pub bHardwareChecksumEnabled: super::super::Foundation::BOOL,
    pub bIsYankable: super::super::Foundation::BOOL,
    pub bAllocateHotSpare: super::super::Foundation::BOOL,
    pub bUseMirroredCache: super::super::Foundation::BOOL,
    pub bReadCachingEnabled: super::super::Foundation::BOOL,
    pub bWriteCachingEnabled: super::super::Foundation::BOOL,
    pub bMediaScanEnabled: super::super::Foundation::BOOL,
    pub bConsistencyCheckEnabled: super::super::Foundation::BOOL,
    pub BusType: VDS_STORAGE_BUS_TYPE,
    pub bReserved1: super::super::Foundation::BOOL,
    pub bReserved2: super::super::Foundation::BOOL,
    pub bReserved3: super::super::Foundation::BOOL,
    pub sRebuildPriority: i16,
}
impl windows_core::TypeKind for VDS_HINTS2 {
    type TypeKind = windows_core::CopyType;
}
impl Default for VDS_HINTS2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VDS_INPUT_DISK {
    pub diskId: windows_core::GUID,
    pub ullSize: u64,
    pub plexId: windows_core::GUID,
    pub memberIdx: u32,
}
impl windows_core::TypeKind for VDS_INPUT_DISK {
    type TypeKind = windows_core::CopyType;
}
impl Default for VDS_INPUT_DISK {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VDS_INTERCONNECT {
    pub m_addressType: VDS_INTERCONNECT_ADDRESS_TYPE,
    pub m_cbPort: u32,
    pub m_pbPort: *mut u8,
    pub m_cbAddress: u32,
    pub m_pbAddress: *mut u8,
}
impl windows_core::TypeKind for VDS_INTERCONNECT {
    type TypeKind = windows_core::CopyType;
}
impl Default for VDS_INTERCONNECT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VDS_IPADDRESS {
    pub r#type: VDS_IPADDRESS_TYPE,
    pub ipv4Address: u32,
    pub ipv6Address: [u8; 16],
    pub ulIpv6FlowInfo: u32,
    pub ulIpv6ScopeId: u32,
    pub wszTextAddress: [u16; 257],
    pub ulPort: u32,
}
impl windows_core::TypeKind for VDS_IPADDRESS {
    type TypeKind = windows_core::CopyType;
}
impl Default for VDS_IPADDRESS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VDS_ISCSI_INITIATOR_ADAPTER_PROP {
    pub id: windows_core::GUID,
    pub pwszName: windows_core::PWSTR,
}
impl windows_core::TypeKind for VDS_ISCSI_INITIATOR_ADAPTER_PROP {
    type TypeKind = windows_core::CopyType;
}
impl Default for VDS_ISCSI_INITIATOR_ADAPTER_PROP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VDS_ISCSI_INITIATOR_PORTAL_PROP {
    pub id: windows_core::GUID,
    pub address: VDS_IPADDRESS,
    pub ulPortIndex: u32,
}
impl windows_core::TypeKind for VDS_ISCSI_INITIATOR_PORTAL_PROP {
    type TypeKind = windows_core::CopyType;
}
impl Default for VDS_ISCSI_INITIATOR_PORTAL_PROP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VDS_ISCSI_IPSEC_KEY {
    pub pKey: *mut u8,
    pub ulKeySize: u32,
}
impl windows_core::TypeKind for VDS_ISCSI_IPSEC_KEY {
    type TypeKind = windows_core::CopyType;
}
impl Default for VDS_ISCSI_IPSEC_KEY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VDS_ISCSI_PORTALGROUP_PROP {
    pub id: windows_core::GUID,
    pub tag: u16,
}
impl windows_core::TypeKind for VDS_ISCSI_PORTALGROUP_PROP {
    type TypeKind = windows_core::CopyType;
}
impl Default for VDS_ISCSI_PORTALGROUP_PROP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VDS_ISCSI_PORTAL_PROP {
    pub id: windows_core::GUID,
    pub address: VDS_IPADDRESS,
    pub status: VDS_ISCSI_PORTAL_STATUS,
}
impl windows_core::TypeKind for VDS_ISCSI_PORTAL_PROP {
    type TypeKind = windows_core::CopyType;
}
impl Default for VDS_ISCSI_PORTAL_PROP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VDS_ISCSI_SHARED_SECRET {
    pub pSharedSecret: *mut u8,
    pub ulSharedSecretSize: u32,
}
impl windows_core::TypeKind for VDS_ISCSI_SHARED_SECRET {
    type TypeKind = windows_core::CopyType;
}
impl Default for VDS_ISCSI_SHARED_SECRET {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VDS_ISCSI_TARGET_PROP {
    pub id: windows_core::GUID,
    pub pwszIscsiName: windows_core::PWSTR,
    pub pwszFriendlyName: windows_core::PWSTR,
    pub bChapEnabled: super::super::Foundation::BOOL,
}
impl windows_core::TypeKind for VDS_ISCSI_TARGET_PROP {
    type TypeKind = windows_core::CopyType;
}
impl Default for VDS_ISCSI_TARGET_PROP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VDS_LUN_INFORMATION {
    pub m_version: u32,
    pub m_DeviceType: u8,
    pub m_DeviceTypeModifier: u8,
    pub m_bCommandQueueing: super::super::Foundation::BOOL,
    pub m_BusType: VDS_STORAGE_BUS_TYPE,
    pub m_szVendorId: *mut u8,
    pub m_szProductId: *mut u8,
    pub m_szProductRevision: *mut u8,
    pub m_szSerialNumber: *mut u8,
    pub m_diskSignature: windows_core::GUID,
    pub m_deviceIdDescriptor: VDS_STORAGE_DEVICE_ID_DESCRIPTOR,
    pub m_cInterconnects: u32,
    pub m_rgInterconnects: *mut VDS_INTERCONNECT,
}
impl windows_core::TypeKind for VDS_LUN_INFORMATION {
    type TypeKind = windows_core::CopyType;
}
impl Default for VDS_LUN_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VDS_LUN_NOTIFICATION {
    pub ulEvent: VDS_NF_LUN,
    pub LunId: windows_core::GUID,
}
impl windows_core::TypeKind for VDS_LUN_NOTIFICATION {
    type TypeKind = windows_core::CopyType;
}
impl Default for VDS_LUN_NOTIFICATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VDS_LUN_PLEX_PROP {
    pub id: windows_core::GUID,
    pub ullSize: u64,
    pub r#type: VDS_LUN_PLEX_TYPE,
    pub status: VDS_LUN_PLEX_STATUS,
    pub health: VDS_HEALTH,
    pub TransitionState: VDS_TRANSITION_STATE,
    pub ulFlags: u32,
    pub ulStripeSize: u32,
    pub sRebuildPriority: i16,
}
impl windows_core::TypeKind for VDS_LUN_PLEX_PROP {
    type TypeKind = windows_core::CopyType;
}
impl Default for VDS_LUN_PLEX_PROP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VDS_LUN_PROP {
    pub id: windows_core::GUID,
    pub ullSize: u64,
    pub pwszFriendlyName: windows_core::PWSTR,
    pub pwszIdentification: windows_core::PWSTR,
    pub pwszUnmaskingList: windows_core::PWSTR,
    pub ulFlags: u32,
    pub r#type: VDS_LUN_TYPE,
    pub status: VDS_LUN_STATUS,
    pub health: VDS_HEALTH,
    pub TransitionState: VDS_TRANSITION_STATE,
    pub sRebuildPriority: i16,
}
impl windows_core::TypeKind for VDS_LUN_PROP {
    type TypeKind = windows_core::CopyType;
}
impl Default for VDS_LUN_PROP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VDS_MOUNT_POINT_NOTIFICATION {
    pub ulEvent: u32,
    pub volumeId: windows_core::GUID,
}
impl windows_core::TypeKind for VDS_MOUNT_POINT_NOTIFICATION {
    type TypeKind = windows_core::CopyType;
}
impl Default for VDS_MOUNT_POINT_NOTIFICATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct VDS_NOTIFICATION {
    pub objectType: VDS_NOTIFICATION_TARGET_TYPE,
    pub Anonymous: VDS_NOTIFICATION_0,
}
impl windows_core::TypeKind for VDS_NOTIFICATION {
    type TypeKind = windows_core::CopyType;
}
impl Default for VDS_NOTIFICATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union VDS_NOTIFICATION_0 {
    pub Pack: VDS_PACK_NOTIFICATION,
    pub Disk: VDS_DISK_NOTIFICATION,
    pub Volume: VDS_VOLUME_NOTIFICATION,
    pub Partition: VDS_PARTITION_NOTIFICATION,
    pub Letter: VDS_DRIVE_LETTER_NOTIFICATION,
    pub FileSystem: VDS_FILE_SYSTEM_NOTIFICATION,
    pub MountPoint: VDS_MOUNT_POINT_NOTIFICATION,
    pub SubSystem: VDS_SUB_SYSTEM_NOTIFICATION,
    pub Controller: VDS_CONTROLLER_NOTIFICATION,
    pub Drive: VDS_DRIVE_NOTIFICATION,
    pub Lun: VDS_LUN_NOTIFICATION,
    pub Port: VDS_PORT_NOTIFICATION,
    pub Portal: VDS_PORTAL_NOTIFICATION,
    pub Target: VDS_TARGET_NOTIFICATION,
    pub PortalGroup: VDS_PORTAL_GROUP_NOTIFICATION,
    pub Service: VDS_SERVICE_NOTIFICATION,
}
impl windows_core::TypeKind for VDS_NOTIFICATION_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for VDS_NOTIFICATION_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VDS_PACK_NOTIFICATION {
    pub ulEvent: VDS_NF_PACK,
    pub packId: windows_core::GUID,
}
impl windows_core::TypeKind for VDS_PACK_NOTIFICATION {
    type TypeKind = windows_core::CopyType;
}
impl Default for VDS_PACK_NOTIFICATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VDS_PACK_PROP {
    pub id: windows_core::GUID,
    pub pwszName: windows_core::PWSTR,
    pub status: VDS_PACK_STATUS,
    pub ulFlags: u32,
}
impl windows_core::TypeKind for VDS_PACK_PROP {
    type TypeKind = windows_core::CopyType;
}
impl Default for VDS_PACK_PROP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct VDS_PARTITION_INFORMATION_EX {
    pub dwPartitionStyle: __VDS_PARTITION_STYLE,
    pub ullStartingOffset: u64,
    pub ullPartitionLength: u64,
    pub dwPartitionNumber: u32,
    pub bRewritePartition: super::super::Foundation::BOOLEAN,
    pub Anonymous: VDS_PARTITION_INFORMATION_EX_0,
}
impl windows_core::TypeKind for VDS_PARTITION_INFORMATION_EX {
    type TypeKind = windows_core::CopyType;
}
impl Default for VDS_PARTITION_INFORMATION_EX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union VDS_PARTITION_INFORMATION_EX_0 {
    pub Mbr: VDS_PARTITION_INFO_MBR,
    pub Gpt: VDS_PARTITION_INFO_GPT,
}
impl windows_core::TypeKind for VDS_PARTITION_INFORMATION_EX_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for VDS_PARTITION_INFORMATION_EX_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VDS_PARTITION_INFO_GPT {
    pub partitionType: windows_core::GUID,
    pub partitionId: windows_core::GUID,
    pub attributes: u64,
    pub name: [u16; 36],
}
impl windows_core::TypeKind for VDS_PARTITION_INFO_GPT {
    type TypeKind = windows_core::CopyType;
}
impl Default for VDS_PARTITION_INFO_GPT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VDS_PARTITION_INFO_MBR {
    pub partitionType: u8,
    pub bootIndicator: super::super::Foundation::BOOLEAN,
    pub recognizedPartition: super::super::Foundation::BOOLEAN,
    pub hiddenSectors: u32,
}
impl windows_core::TypeKind for VDS_PARTITION_INFO_MBR {
    type TypeKind = windows_core::CopyType;
}
impl Default for VDS_PARTITION_INFO_MBR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VDS_PARTITION_NOTIFICATION {
    pub ulEvent: u32,
    pub diskId: windows_core::GUID,
    pub ullOffset: u64,
}
impl windows_core::TypeKind for VDS_PARTITION_NOTIFICATION {
    type TypeKind = windows_core::CopyType;
}
impl Default for VDS_PARTITION_NOTIFICATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct VDS_PARTITION_PROP {
    pub PartitionStyle: VDS_PARTITION_STYLE,
    pub ulFlags: u32,
    pub ulPartitionNumber: u32,
    pub ullOffset: u64,
    pub ullSize: u64,
    pub Anonymous: VDS_PARTITION_PROP_0,
}
impl windows_core::TypeKind for VDS_PARTITION_PROP {
    type TypeKind = windows_core::CopyType;
}
impl Default for VDS_PARTITION_PROP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union VDS_PARTITION_PROP_0 {
    pub Mbr: VDS_PARTITION_INFO_MBR,
    pub Gpt: VDS_PARTITION_INFO_GPT,
}
impl windows_core::TypeKind for VDS_PARTITION_PROP_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for VDS_PARTITION_PROP_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VDS_PATH_ID {
    pub ullSourceId: u64,
    pub ullPathId: u64,
}
impl windows_core::TypeKind for VDS_PATH_ID {
    type TypeKind = windows_core::CopyType;
}
impl Default for VDS_PATH_ID {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct VDS_PATH_INFO {
    pub pathId: VDS_PATH_ID,
    pub r#type: VDS_HWPROVIDER_TYPE,
    pub status: VDS_PATH_STATUS,
    pub Anonymous1: VDS_PATH_INFO_0,
    pub Anonymous2: VDS_PATH_INFO_1,
    pub Anonymous3: VDS_PATH_INFO_2,
}
impl windows_core::TypeKind for VDS_PATH_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for VDS_PATH_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union VDS_PATH_INFO_0 {
    pub controllerPortId: windows_core::GUID,
    pub targetPortalId: windows_core::GUID,
}
impl windows_core::TypeKind for VDS_PATH_INFO_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for VDS_PATH_INFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union VDS_PATH_INFO_1 {
    pub hbaPortId: windows_core::GUID,
    pub initiatorAdapterId: windows_core::GUID,
}
impl windows_core::TypeKind for VDS_PATH_INFO_1 {
    type TypeKind = windows_core::CopyType;
}
impl Default for VDS_PATH_INFO_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union VDS_PATH_INFO_2 {
    pub pHbaPortProp: *mut VDS_HBAPORT_PROP,
    pub pInitiatorPortalIpAddr: *mut VDS_IPADDRESS,
}
impl windows_core::TypeKind for VDS_PATH_INFO_2 {
    type TypeKind = windows_core::CopyType;
}
impl Default for VDS_PATH_INFO_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VDS_PATH_POLICY {
    pub pathId: VDS_PATH_ID,
    pub bPrimaryPath: super::super::Foundation::BOOL,
    pub ulWeight: u32,
}
impl windows_core::TypeKind for VDS_PATH_POLICY {
    type TypeKind = windows_core::CopyType;
}
impl Default for VDS_PATH_POLICY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VDS_POOL_ATTRIBUTES {
    pub ullAttributeMask: u64,
    pub raidType: VDS_RAID_TYPE,
    pub busType: VDS_STORAGE_BUS_TYPE,
    pub pwszIntendedUsage: windows_core::PWSTR,
    pub bSpinDown: super::super::Foundation::BOOL,
    pub bIsThinProvisioned: super::super::Foundation::BOOL,
    pub ullProvisionedSpace: u64,
    pub bNoSinglePointOfFailure: super::super::Foundation::BOOL,
    pub ulDataRedundancyMax: u32,
    pub ulDataRedundancyMin: u32,
    pub ulDataRedundancyDefault: u32,
    pub ulPackageRedundancyMax: u32,
    pub ulPackageRedundancyMin: u32,
    pub ulPackageRedundancyDefault: u32,
    pub ulStripeSize: u32,
    pub ulStripeSizeMax: u32,
    pub ulStripeSizeMin: u32,
    pub ulDefaultStripeSize: u32,
    pub ulNumberOfColumns: u32,
    pub ulNumberOfColumnsMax: u32,
    pub ulNumberOfColumnsMin: u32,
    pub ulDefaultNumberofColumns: u32,
    pub ulDataAvailabilityHint: u32,
    pub ulAccessRandomnessHint: u32,
    pub ulAccessDirectionHint: u32,
    pub ulAccessSizeHint: u32,
    pub ulAccessLatencyHint: u32,
    pub ulAccessBandwidthWeightHint: u32,
    pub ulStorageCostHint: u32,
    pub ulStorageEfficiencyHint: u32,
    pub ulNumOfCustomAttributes: u32,
    pub pPoolCustomAttributes: *mut VDS_POOL_CUSTOM_ATTRIBUTES,
    pub bReserved1: super::super::Foundation::BOOL,
    pub bReserved2: super::super::Foundation::BOOL,
    pub ulReserved1: u32,
    pub ulReserved2: u32,
    pub ullReserved1: u64,
    pub ullReserved2: u64,
}
impl windows_core::TypeKind for VDS_POOL_ATTRIBUTES {
    type TypeKind = windows_core::CopyType;
}
impl Default for VDS_POOL_ATTRIBUTES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VDS_POOL_CUSTOM_ATTRIBUTES {
    pub pwszName: windows_core::PWSTR,
    pub pwszValue: windows_core::PWSTR,
}
impl windows_core::TypeKind for VDS_POOL_CUSTOM_ATTRIBUTES {
    type TypeKind = windows_core::CopyType;
}
impl Default for VDS_POOL_CUSTOM_ATTRIBUTES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VDS_PORTAL_GROUP_NOTIFICATION {
    pub ulEvent: u32,
    pub portalGroupId: windows_core::GUID,
}
impl windows_core::TypeKind for VDS_PORTAL_GROUP_NOTIFICATION {
    type TypeKind = windows_core::CopyType;
}
impl Default for VDS_PORTAL_GROUP_NOTIFICATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VDS_PORTAL_NOTIFICATION {
    pub ulEvent: u32,
    pub portalId: windows_core::GUID,
}
impl windows_core::TypeKind for VDS_PORTAL_NOTIFICATION {
    type TypeKind = windows_core::CopyType;
}
impl Default for VDS_PORTAL_NOTIFICATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VDS_PORT_NOTIFICATION {
    pub ulEvent: VDS_NF_PORT,
    pub portId: windows_core::GUID,
}
impl windows_core::TypeKind for VDS_PORT_NOTIFICATION {
    type TypeKind = windows_core::CopyType;
}
impl Default for VDS_PORT_NOTIFICATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VDS_PORT_PROP {
    pub id: windows_core::GUID,
    pub pwszFriendlyName: windows_core::PWSTR,
    pub pwszIdentification: windows_core::PWSTR,
    pub status: VDS_PORT_STATUS,
}
impl windows_core::TypeKind for VDS_PORT_PROP {
    type TypeKind = windows_core::CopyType;
}
impl Default for VDS_PORT_PROP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VDS_PROVIDER_PROP {
    pub id: windows_core::GUID,
    pub pwszName: windows_core::PWSTR,
    pub guidVersionId: windows_core::GUID,
    pub pwszVersion: windows_core::PWSTR,
    pub r#type: VDS_PROVIDER_TYPE,
    pub ulFlags: u32,
    pub ulStripeSizeFlags: u32,
    pub sRebuildPriority: i16,
}
impl windows_core::TypeKind for VDS_PROVIDER_PROP {
    type TypeKind = windows_core::CopyType;
}
impl Default for VDS_PROVIDER_PROP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VDS_REPARSE_POINT_PROP {
    pub SourceVolumeId: windows_core::GUID,
    pub pwszPath: windows_core::PWSTR,
}
impl windows_core::TypeKind for VDS_REPARSE_POINT_PROP {
    type TypeKind = windows_core::CopyType;
}
impl Default for VDS_REPARSE_POINT_PROP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VDS_SERVICE_NOTIFICATION {
    pub ulEvent: u32,
    pub action: VDS_RECOVER_ACTION,
}
impl windows_core::TypeKind for VDS_SERVICE_NOTIFICATION {
    type TypeKind = windows_core::CopyType;
}
impl Default for VDS_SERVICE_NOTIFICATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VDS_SERVICE_PROP {
    pub pwszVersion: windows_core::PWSTR,
    pub ulFlags: u32,
}
impl windows_core::TypeKind for VDS_SERVICE_PROP {
    type TypeKind = windows_core::CopyType;
}
impl Default for VDS_SERVICE_PROP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VDS_STORAGE_DEVICE_ID_DESCRIPTOR {
    pub m_version: u32,
    pub m_cIdentifiers: u32,
    pub m_rgIdentifiers: *mut VDS_STORAGE_IDENTIFIER,
}
impl windows_core::TypeKind for VDS_STORAGE_DEVICE_ID_DESCRIPTOR {
    type TypeKind = windows_core::CopyType;
}
impl Default for VDS_STORAGE_DEVICE_ID_DESCRIPTOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VDS_STORAGE_IDENTIFIER {
    pub m_CodeSet: VDS_STORAGE_IDENTIFIER_CODE_SET,
    pub m_Type: VDS_STORAGE_IDENTIFIER_TYPE,
    pub m_cbIdentifier: u32,
    pub m_rgbIdentifier: *mut u8,
}
impl windows_core::TypeKind for VDS_STORAGE_IDENTIFIER {
    type TypeKind = windows_core::CopyType;
}
impl Default for VDS_STORAGE_IDENTIFIER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VDS_STORAGE_POOL_DRIVE_EXTENT {
    pub id: windows_core::GUID,
    pub ullSize: u64,
    pub bUsed: super::super::Foundation::BOOL,
}
impl windows_core::TypeKind for VDS_STORAGE_POOL_DRIVE_EXTENT {
    type TypeKind = windows_core::CopyType;
}
impl Default for VDS_STORAGE_POOL_DRIVE_EXTENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VDS_STORAGE_POOL_PROP {
    pub id: windows_core::GUID,
    pub status: VDS_STORAGE_POOL_STATUS,
    pub health: VDS_HEALTH,
    pub r#type: VDS_STORAGE_POOL_TYPE,
    pub pwszName: windows_core::PWSTR,
    pub pwszDescription: windows_core::PWSTR,
    pub ullTotalConsumedSpace: u64,
    pub ullTotalManagedSpace: u64,
    pub ullRemainingFreeSpace: u64,
}
impl windows_core::TypeKind for VDS_STORAGE_POOL_PROP {
    type TypeKind = windows_core::CopyType;
}
impl Default for VDS_STORAGE_POOL_PROP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VDS_SUB_SYSTEM_NOTIFICATION {
    pub ulEvent: u32,
    pub subSystemId: windows_core::GUID,
}
impl windows_core::TypeKind for VDS_SUB_SYSTEM_NOTIFICATION {
    type TypeKind = windows_core::CopyType;
}
impl Default for VDS_SUB_SYSTEM_NOTIFICATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VDS_SUB_SYSTEM_PROP {
    pub id: windows_core::GUID,
    pub pwszFriendlyName: windows_core::PWSTR,
    pub pwszIdentification: windows_core::PWSTR,
    pub ulFlags: u32,
    pub ulStripeSizeFlags: u32,
    pub status: VDS_SUB_SYSTEM_STATUS,
    pub health: VDS_HEALTH,
    pub sNumberOfInternalBuses: i16,
    pub sMaxNumberOfSlotsEachBus: i16,
    pub sMaxNumberOfControllers: i16,
    pub sRebuildPriority: i16,
}
impl windows_core::TypeKind for VDS_SUB_SYSTEM_PROP {
    type TypeKind = windows_core::CopyType;
}
impl Default for VDS_SUB_SYSTEM_PROP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VDS_SUB_SYSTEM_PROP2 {
    pub id: windows_core::GUID,
    pub pwszFriendlyName: windows_core::PWSTR,
    pub pwszIdentification: windows_core::PWSTR,
    pub ulFlags: u32,
    pub ulStripeSizeFlags: u32,
    pub ulSupportedRaidTypeFlags: u32,
    pub status: VDS_SUB_SYSTEM_STATUS,
    pub health: VDS_HEALTH,
    pub sNumberOfInternalBuses: i16,
    pub sMaxNumberOfSlotsEachBus: i16,
    pub sMaxNumberOfControllers: i16,
    pub sRebuildPriority: i16,
    pub ulNumberOfEnclosures: u32,
}
impl windows_core::TypeKind for VDS_SUB_SYSTEM_PROP2 {
    type TypeKind = windows_core::CopyType;
}
impl Default for VDS_SUB_SYSTEM_PROP2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VDS_TARGET_NOTIFICATION {
    pub ulEvent: u32,
    pub targetId: windows_core::GUID,
}
impl windows_core::TypeKind for VDS_TARGET_NOTIFICATION {
    type TypeKind = windows_core::CopyType;
}
impl Default for VDS_TARGET_NOTIFICATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Storage_Vhd")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VDS_VDISK_PROPERTIES {
    pub Id: windows_core::GUID,
    pub State: VDS_VDISK_STATE,
    pub VirtualDeviceType: super::Vhd::VIRTUAL_STORAGE_TYPE,
    pub VirtualSize: u64,
    pub PhysicalSize: u64,
    pub pPath: windows_core::PWSTR,
    pub pDeviceName: windows_core::PWSTR,
    pub DiskFlag: super::Vhd::DEPENDENT_DISK_FLAG,
    pub bIsChild: super::super::Foundation::BOOL,
    pub pParentPath: windows_core::PWSTR,
}
#[cfg(feature = "Win32_Storage_Vhd")]
impl windows_core::TypeKind for VDS_VDISK_PROPERTIES {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Storage_Vhd")]
impl Default for VDS_VDISK_PROPERTIES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VDS_VOLUME_NOTIFICATION {
    pub ulEvent: u32,
    pub volumeId: windows_core::GUID,
    pub plexId: windows_core::GUID,
    pub ulPercentCompleted: u32,
}
impl windows_core::TypeKind for VDS_VOLUME_NOTIFICATION {
    type TypeKind = windows_core::CopyType;
}
impl Default for VDS_VOLUME_NOTIFICATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VDS_VOLUME_PLEX_PROP {
    pub id: windows_core::GUID,
    pub r#type: VDS_VOLUME_PLEX_TYPE,
    pub status: VDS_VOLUME_PLEX_STATUS,
    pub health: VDS_HEALTH,
    pub TransitionState: VDS_TRANSITION_STATE,
    pub ullSize: u64,
    pub ulStripeSize: u32,
    pub ulNumberOfMembers: u32,
}
impl windows_core::TypeKind for VDS_VOLUME_PLEX_PROP {
    type TypeKind = windows_core::CopyType;
}
impl Default for VDS_VOLUME_PLEX_PROP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VDS_VOLUME_PROP {
    pub id: windows_core::GUID,
    pub r#type: VDS_VOLUME_TYPE,
    pub status: VDS_VOLUME_STATUS,
    pub health: VDS_HEALTH,
    pub TransitionState: VDS_TRANSITION_STATE,
    pub ullSize: u64,
    pub ulFlags: u32,
    pub RecommendedFileSystemType: VDS_FILE_SYSTEM_TYPE,
    pub pwszName: windows_core::PWSTR,
}
impl windows_core::TypeKind for VDS_VOLUME_PROP {
    type TypeKind = windows_core::CopyType;
}
impl Default for VDS_VOLUME_PROP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VDS_VOLUME_PROP2 {
    pub id: windows_core::GUID,
    pub r#type: VDS_VOLUME_TYPE,
    pub status: VDS_VOLUME_STATUS,
    pub health: VDS_HEALTH,
    pub TransitionState: VDS_TRANSITION_STATE,
    pub ullSize: u64,
    pub ulFlags: u32,
    pub RecommendedFileSystemType: VDS_FILE_SYSTEM_TYPE,
    pub cbUniqueId: u32,
    pub pwszName: windows_core::PWSTR,
    pub pUniqueId: *mut u8,
}
impl windows_core::TypeKind for VDS_VOLUME_PROP2 {
    type TypeKind = windows_core::CopyType;
}
impl Default for VDS_VOLUME_PROP2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VDS_WWN {
    pub rguchWwn: [u8; 8],
}
impl windows_core::TypeKind for VDS_WWN {
    type TypeKind = windows_core::CopyType;
}
impl Default for VDS_WWN {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "implement")]
core::include!("impl.rs");
