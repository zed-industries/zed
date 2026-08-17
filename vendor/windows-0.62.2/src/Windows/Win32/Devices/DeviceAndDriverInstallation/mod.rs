#[inline]
pub unsafe fn CMP_WaitNoPendingInstallEvents(dwtimeout: u32) -> u32 {
    windows_core::link!("cfgmgr32.dll" "system" fn CMP_WaitNoPendingInstallEvents(dwtimeout : u32) -> u32);
    unsafe { CMP_WaitNoPendingInstallEvents(dwtimeout) }
}
#[cfg(feature = "Win32_Data_HtmlHelp")]
#[inline]
pub unsafe fn CM_Add_Empty_Log_Conf(plclogconf: *mut usize, dndevinst: u32, priority: super::super::Data::HtmlHelp::PRIORITY, ulflags: u32) -> CONFIGRET {
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Add_Empty_Log_Conf(plclogconf : *mut usize, dndevinst : u32, priority : super::super::Data::HtmlHelp:: PRIORITY, ulflags : u32) -> CONFIGRET);
    unsafe { CM_Add_Empty_Log_Conf(plclogconf as _, dndevinst, priority, ulflags) }
}
#[cfg(feature = "Win32_Data_HtmlHelp")]
#[inline]
pub unsafe fn CM_Add_Empty_Log_Conf_Ex(plclogconf: *mut usize, dndevinst: u32, priority: super::super::Data::HtmlHelp::PRIORITY, ulflags: u32, hmachine: Option<isize>) -> CONFIGRET {
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Add_Empty_Log_Conf_Ex(plclogconf : *mut usize, dndevinst : u32, priority : super::super::Data::HtmlHelp:: PRIORITY, ulflags : u32, hmachine : isize) -> CONFIGRET);
    unsafe { CM_Add_Empty_Log_Conf_Ex(plclogconf as _, dndevinst, priority, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CM_Add_IDA<P1>(dndevinst: u32, pszid: P1, ulflags: u32) -> CONFIGRET
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Add_IDA(dndevinst : u32, pszid : windows_core::PCSTR, ulflags : u32) -> CONFIGRET);
    unsafe { CM_Add_IDA(dndevinst, pszid.param().abi(), ulflags) }
}
#[inline]
pub unsafe fn CM_Add_IDW<P1>(dndevinst: u32, pszid: P1, ulflags: u32) -> CONFIGRET
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Add_IDW(dndevinst : u32, pszid : windows_core::PCWSTR, ulflags : u32) -> CONFIGRET);
    unsafe { CM_Add_IDW(dndevinst, pszid.param().abi(), ulflags) }
}
#[inline]
pub unsafe fn CM_Add_ID_ExA<P1>(dndevinst: u32, pszid: P1, ulflags: u32, hmachine: Option<isize>) -> CONFIGRET
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Add_ID_ExA(dndevinst : u32, pszid : windows_core::PCSTR, ulflags : u32, hmachine : isize) -> CONFIGRET);
    unsafe { CM_Add_ID_ExA(dndevinst, pszid.param().abi(), ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CM_Add_ID_ExW<P1>(dndevinst: u32, pszid: P1, ulflags: u32, hmachine: Option<isize>) -> CONFIGRET
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Add_ID_ExW(dndevinst : u32, pszid : windows_core::PCWSTR, ulflags : u32, hmachine : isize) -> CONFIGRET);
    unsafe { CM_Add_ID_ExW(dndevinst, pszid.param().abi(), ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CM_Add_Range(ullstartvalue: u64, ullendvalue: u64, rlh: usize, ulflags: u32) -> CONFIGRET {
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Add_Range(ullstartvalue : u64, ullendvalue : u64, rlh : usize, ulflags : u32) -> CONFIGRET);
    unsafe { CM_Add_Range(ullstartvalue, ullendvalue, rlh, ulflags) }
}
#[inline]
pub unsafe fn CM_Add_Res_Des(prdresdes: Option<*mut usize>, lclogconf: usize, resourceid: u32, resourcedata: *const core::ffi::c_void, resourcelen: u32, ulflags: u32) -> CONFIGRET {
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Add_Res_Des(prdresdes : *mut usize, lclogconf : usize, resourceid : u32, resourcedata : *const core::ffi::c_void, resourcelen : u32, ulflags : u32) -> CONFIGRET);
    unsafe { CM_Add_Res_Des(prdresdes.unwrap_or(core::mem::zeroed()) as _, lclogconf, resourceid, resourcedata, resourcelen, ulflags) }
}
#[inline]
pub unsafe fn CM_Add_Res_Des_Ex(prdresdes: Option<*mut usize>, lclogconf: usize, resourceid: u32, resourcedata: *const core::ffi::c_void, resourcelen: u32, ulflags: u32, hmachine: Option<isize>) -> CONFIGRET {
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Add_Res_Des_Ex(prdresdes : *mut usize, lclogconf : usize, resourceid : u32, resourcedata : *const core::ffi::c_void, resourcelen : u32, ulflags : u32, hmachine : isize) -> CONFIGRET);
    unsafe { CM_Add_Res_Des_Ex(prdresdes.unwrap_or(core::mem::zeroed()) as _, lclogconf, resourceid, resourcedata, resourcelen, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CM_Connect_MachineA<P0>(uncservername: P0, phmachine: *mut isize) -> CONFIGRET
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Connect_MachineA(uncservername : windows_core::PCSTR, phmachine : *mut isize) -> CONFIGRET);
    unsafe { CM_Connect_MachineA(uncservername.param().abi(), phmachine as _) }
}
#[inline]
pub unsafe fn CM_Connect_MachineW<P0>(uncservername: P0, phmachine: *mut isize) -> CONFIGRET
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Connect_MachineW(uncservername : windows_core::PCWSTR, phmachine : *mut isize) -> CONFIGRET);
    unsafe { CM_Connect_MachineW(uncservername.param().abi(), phmachine as _) }
}
#[inline]
pub unsafe fn CM_Create_DevNodeA<P1>(pdndevinst: *mut u32, pdeviceid: P1, dnparent: u32, ulflags: u32) -> CONFIGRET
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Create_DevNodeA(pdndevinst : *mut u32, pdeviceid : windows_core::PCSTR, dnparent : u32, ulflags : u32) -> CONFIGRET);
    unsafe { CM_Create_DevNodeA(pdndevinst as _, pdeviceid.param().abi(), dnparent, ulflags) }
}
#[inline]
pub unsafe fn CM_Create_DevNodeW<P1>(pdndevinst: *mut u32, pdeviceid: P1, dnparent: u32, ulflags: u32) -> CONFIGRET
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Create_DevNodeW(pdndevinst : *mut u32, pdeviceid : windows_core::PCWSTR, dnparent : u32, ulflags : u32) -> CONFIGRET);
    unsafe { CM_Create_DevNodeW(pdndevinst as _, pdeviceid.param().abi(), dnparent, ulflags) }
}
#[inline]
pub unsafe fn CM_Create_DevNode_ExA<P1>(pdndevinst: *mut u32, pdeviceid: P1, dnparent: u32, ulflags: u32, hmachine: Option<isize>) -> CONFIGRET
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Create_DevNode_ExA(pdndevinst : *mut u32, pdeviceid : windows_core::PCSTR, dnparent : u32, ulflags : u32, hmachine : isize) -> CONFIGRET);
    unsafe { CM_Create_DevNode_ExA(pdndevinst as _, pdeviceid.param().abi(), dnparent, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CM_Create_DevNode_ExW<P1>(pdndevinst: *mut u32, pdeviceid: P1, dnparent: u32, ulflags: u32, hmachine: Option<isize>) -> CONFIGRET
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Create_DevNode_ExW(pdndevinst : *mut u32, pdeviceid : windows_core::PCWSTR, dnparent : u32, ulflags : u32, hmachine : isize) -> CONFIGRET);
    unsafe { CM_Create_DevNode_ExW(pdndevinst as _, pdeviceid.param().abi(), dnparent, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CM_Create_Range_List(prlh: *mut usize, ulflags: u32) -> CONFIGRET {
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Create_Range_List(prlh : *mut usize, ulflags : u32) -> CONFIGRET);
    unsafe { CM_Create_Range_List(prlh as _, ulflags) }
}
#[inline]
pub unsafe fn CM_Delete_Class_Key(classguid: *const windows_core::GUID, ulflags: u32) -> CONFIGRET {
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Delete_Class_Key(classguid : *const windows_core::GUID, ulflags : u32) -> CONFIGRET);
    unsafe { CM_Delete_Class_Key(classguid, ulflags) }
}
#[inline]
pub unsafe fn CM_Delete_Class_Key_Ex(classguid: *const windows_core::GUID, ulflags: u32, hmachine: Option<isize>) -> CONFIGRET {
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Delete_Class_Key_Ex(classguid : *const windows_core::GUID, ulflags : u32, hmachine : isize) -> CONFIGRET);
    unsafe { CM_Delete_Class_Key_Ex(classguid, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CM_Delete_DevNode_Key(dndevnode: u32, ulhardwareprofile: u32, ulflags: u32) -> CONFIGRET {
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Delete_DevNode_Key(dndevnode : u32, ulhardwareprofile : u32, ulflags : u32) -> CONFIGRET);
    unsafe { CM_Delete_DevNode_Key(dndevnode, ulhardwareprofile, ulflags) }
}
#[inline]
pub unsafe fn CM_Delete_DevNode_Key_Ex(dndevnode: u32, ulhardwareprofile: u32, ulflags: u32, hmachine: Option<isize>) -> CONFIGRET {
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Delete_DevNode_Key_Ex(dndevnode : u32, ulhardwareprofile : u32, ulflags : u32, hmachine : isize) -> CONFIGRET);
    unsafe { CM_Delete_DevNode_Key_Ex(dndevnode, ulhardwareprofile, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CM_Delete_Device_Interface_KeyA<P0>(pszdeviceinterface: P0, ulflags: u32) -> CONFIGRET
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Delete_Device_Interface_KeyA(pszdeviceinterface : windows_core::PCSTR, ulflags : u32) -> CONFIGRET);
    unsafe { CM_Delete_Device_Interface_KeyA(pszdeviceinterface.param().abi(), ulflags) }
}
#[inline]
pub unsafe fn CM_Delete_Device_Interface_KeyW<P0>(pszdeviceinterface: P0, ulflags: u32) -> CONFIGRET
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Delete_Device_Interface_KeyW(pszdeviceinterface : windows_core::PCWSTR, ulflags : u32) -> CONFIGRET);
    unsafe { CM_Delete_Device_Interface_KeyW(pszdeviceinterface.param().abi(), ulflags) }
}
#[inline]
pub unsafe fn CM_Delete_Device_Interface_Key_ExA<P0>(pszdeviceinterface: P0, ulflags: u32, hmachine: Option<isize>) -> CONFIGRET
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Delete_Device_Interface_Key_ExA(pszdeviceinterface : windows_core::PCSTR, ulflags : u32, hmachine : isize) -> CONFIGRET);
    unsafe { CM_Delete_Device_Interface_Key_ExA(pszdeviceinterface.param().abi(), ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CM_Delete_Device_Interface_Key_ExW<P0>(pszdeviceinterface: P0, ulflags: u32, hmachine: Option<isize>) -> CONFIGRET
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Delete_Device_Interface_Key_ExW(pszdeviceinterface : windows_core::PCWSTR, ulflags : u32, hmachine : isize) -> CONFIGRET);
    unsafe { CM_Delete_Device_Interface_Key_ExW(pszdeviceinterface.param().abi(), ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CM_Delete_Range(ullstartvalue: u64, ullendvalue: u64, rlh: usize, ulflags: u32) -> CONFIGRET {
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Delete_Range(ullstartvalue : u64, ullendvalue : u64, rlh : usize, ulflags : u32) -> CONFIGRET);
    unsafe { CM_Delete_Range(ullstartvalue, ullendvalue, rlh, ulflags) }
}
#[inline]
pub unsafe fn CM_Detect_Resource_Conflict(dndevinst: u32, resourceid: u32, resourcedata: *const core::ffi::c_void, resourcelen: u32, pbconflictdetected: *mut windows_core::BOOL, ulflags: u32) -> CONFIGRET {
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Detect_Resource_Conflict(dndevinst : u32, resourceid : u32, resourcedata : *const core::ffi::c_void, resourcelen : u32, pbconflictdetected : *mut windows_core::BOOL, ulflags : u32) -> CONFIGRET);
    unsafe { CM_Detect_Resource_Conflict(dndevinst, resourceid, resourcedata, resourcelen, pbconflictdetected as _, ulflags) }
}
#[inline]
pub unsafe fn CM_Detect_Resource_Conflict_Ex(dndevinst: u32, resourceid: u32, resourcedata: *const core::ffi::c_void, resourcelen: u32, pbconflictdetected: *mut windows_core::BOOL, ulflags: u32, hmachine: Option<isize>) -> CONFIGRET {
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Detect_Resource_Conflict_Ex(dndevinst : u32, resourceid : u32, resourcedata : *const core::ffi::c_void, resourcelen : u32, pbconflictdetected : *mut windows_core::BOOL, ulflags : u32, hmachine : isize) -> CONFIGRET);
    unsafe { CM_Detect_Resource_Conflict_Ex(dndevinst, resourceid, resourcedata, resourcelen, pbconflictdetected as _, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CM_Disable_DevNode(dndevinst: u32, ulflags: u32) -> CONFIGRET {
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Disable_DevNode(dndevinst : u32, ulflags : u32) -> CONFIGRET);
    unsafe { CM_Disable_DevNode(dndevinst, ulflags) }
}
#[inline]
pub unsafe fn CM_Disable_DevNode_Ex(dndevinst: u32, ulflags: u32, hmachine: Option<isize>) -> CONFIGRET {
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Disable_DevNode_Ex(dndevinst : u32, ulflags : u32, hmachine : isize) -> CONFIGRET);
    unsafe { CM_Disable_DevNode_Ex(dndevinst, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CM_Disconnect_Machine(hmachine: Option<isize>) -> CONFIGRET {
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Disconnect_Machine(hmachine : isize) -> CONFIGRET);
    unsafe { CM_Disconnect_Machine(hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CM_Dup_Range_List(rlhold: usize, rlhnew: usize, ulflags: u32) -> CONFIGRET {
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Dup_Range_List(rlhold : usize, rlhnew : usize, ulflags : u32) -> CONFIGRET);
    unsafe { CM_Dup_Range_List(rlhold, rlhnew, ulflags) }
}
#[inline]
pub unsafe fn CM_Enable_DevNode(dndevinst: u32, ulflags: u32) -> CONFIGRET {
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Enable_DevNode(dndevinst : u32, ulflags : u32) -> CONFIGRET);
    unsafe { CM_Enable_DevNode(dndevinst, ulflags) }
}
#[inline]
pub unsafe fn CM_Enable_DevNode_Ex(dndevinst: u32, ulflags: u32, hmachine: Option<isize>) -> CONFIGRET {
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Enable_DevNode_Ex(dndevinst : u32, ulflags : u32, hmachine : isize) -> CONFIGRET);
    unsafe { CM_Enable_DevNode_Ex(dndevinst, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CM_Enumerate_Classes(ulclassindex: u32, classguid: *mut windows_core::GUID, ulflags: CM_ENUMERATE_FLAGS) -> CONFIGRET {
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Enumerate_Classes(ulclassindex : u32, classguid : *mut windows_core::GUID, ulflags : CM_ENUMERATE_FLAGS) -> CONFIGRET);
    unsafe { CM_Enumerate_Classes(ulclassindex, classguid as _, ulflags) }
}
#[inline]
pub unsafe fn CM_Enumerate_Classes_Ex(ulclassindex: u32, classguid: *mut windows_core::GUID, ulflags: CM_ENUMERATE_FLAGS, hmachine: Option<isize>) -> CONFIGRET {
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Enumerate_Classes_Ex(ulclassindex : u32, classguid : *mut windows_core::GUID, ulflags : CM_ENUMERATE_FLAGS, hmachine : isize) -> CONFIGRET);
    unsafe { CM_Enumerate_Classes_Ex(ulclassindex, classguid as _, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CM_Enumerate_EnumeratorsA(ulenumindex: u32, buffer: windows_core::PSTR, pullength: *mut u32, ulflags: u32) -> CONFIGRET {
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Enumerate_EnumeratorsA(ulenumindex : u32, buffer : windows_core::PSTR, pullength : *mut u32, ulflags : u32) -> CONFIGRET);
    unsafe { CM_Enumerate_EnumeratorsA(ulenumindex, core::mem::transmute(buffer), pullength as _, ulflags) }
}
#[inline]
pub unsafe fn CM_Enumerate_EnumeratorsW(ulenumindex: u32, buffer: windows_core::PWSTR, pullength: *mut u32, ulflags: u32) -> CONFIGRET {
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Enumerate_EnumeratorsW(ulenumindex : u32, buffer : windows_core::PWSTR, pullength : *mut u32, ulflags : u32) -> CONFIGRET);
    unsafe { CM_Enumerate_EnumeratorsW(ulenumindex, core::mem::transmute(buffer), pullength as _, ulflags) }
}
#[inline]
pub unsafe fn CM_Enumerate_Enumerators_ExA(ulenumindex: u32, buffer: windows_core::PSTR, pullength: *mut u32, ulflags: u32, hmachine: Option<isize>) -> CONFIGRET {
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Enumerate_Enumerators_ExA(ulenumindex : u32, buffer : windows_core::PSTR, pullength : *mut u32, ulflags : u32, hmachine : isize) -> CONFIGRET);
    unsafe { CM_Enumerate_Enumerators_ExA(ulenumindex, core::mem::transmute(buffer), pullength as _, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CM_Enumerate_Enumerators_ExW(ulenumindex: u32, buffer: windows_core::PWSTR, pullength: *mut u32, ulflags: u32, hmachine: Option<isize>) -> CONFIGRET {
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Enumerate_Enumerators_ExW(ulenumindex : u32, buffer : windows_core::PWSTR, pullength : *mut u32, ulflags : u32, hmachine : isize) -> CONFIGRET);
    unsafe { CM_Enumerate_Enumerators_ExW(ulenumindex, core::mem::transmute(buffer), pullength as _, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CM_Find_Range(pullstart: *mut u64, ullstart: u64, ullength: u32, ullalignment: u64, ullend: u64, rlh: usize, ulflags: u32) -> CONFIGRET {
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Find_Range(pullstart : *mut u64, ullstart : u64, ullength : u32, ullalignment : u64, ullend : u64, rlh : usize, ulflags : u32) -> CONFIGRET);
    unsafe { CM_Find_Range(pullstart as _, ullstart, ullength, ullalignment, ullend, rlh, ulflags) }
}
#[inline]
pub unsafe fn CM_First_Range(rlh: usize, pullstart: *mut u64, pullend: *mut u64, preelement: *mut usize, ulflags: u32) -> CONFIGRET {
    windows_core::link!("cfgmgr32.dll" "system" fn CM_First_Range(rlh : usize, pullstart : *mut u64, pullend : *mut u64, preelement : *mut usize, ulflags : u32) -> CONFIGRET);
    unsafe { CM_First_Range(rlh, pullstart as _, pullend as _, preelement as _, ulflags) }
}
#[inline]
pub unsafe fn CM_Free_Log_Conf(lclogconftobefreed: usize, ulflags: u32) -> CONFIGRET {
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Free_Log_Conf(lclogconftobefreed : usize, ulflags : u32) -> CONFIGRET);
    unsafe { CM_Free_Log_Conf(lclogconftobefreed, ulflags) }
}
#[inline]
pub unsafe fn CM_Free_Log_Conf_Ex(lclogconftobefreed: usize, ulflags: u32, hmachine: Option<isize>) -> CONFIGRET {
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Free_Log_Conf_Ex(lclogconftobefreed : usize, ulflags : u32, hmachine : isize) -> CONFIGRET);
    unsafe { CM_Free_Log_Conf_Ex(lclogconftobefreed, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CM_Free_Log_Conf_Handle(lclogconf: usize) -> CONFIGRET {
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Free_Log_Conf_Handle(lclogconf : usize) -> CONFIGRET);
    unsafe { CM_Free_Log_Conf_Handle(lclogconf) }
}
#[inline]
pub unsafe fn CM_Free_Range_List(rlh: usize, ulflags: u32) -> CONFIGRET {
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Free_Range_List(rlh : usize, ulflags : u32) -> CONFIGRET);
    unsafe { CM_Free_Range_List(rlh, ulflags) }
}
#[inline]
pub unsafe fn CM_Free_Res_Des(prdresdes: Option<*mut usize>, rdresdes: usize, ulflags: u32) -> CONFIGRET {
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Free_Res_Des(prdresdes : *mut usize, rdresdes : usize, ulflags : u32) -> CONFIGRET);
    unsafe { CM_Free_Res_Des(prdresdes.unwrap_or(core::mem::zeroed()) as _, rdresdes, ulflags) }
}
#[inline]
pub unsafe fn CM_Free_Res_Des_Ex(prdresdes: Option<*mut usize>, rdresdes: usize, ulflags: u32, hmachine: Option<isize>) -> CONFIGRET {
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Free_Res_Des_Ex(prdresdes : *mut usize, rdresdes : usize, ulflags : u32, hmachine : isize) -> CONFIGRET);
    unsafe { CM_Free_Res_Des_Ex(prdresdes.unwrap_or(core::mem::zeroed()) as _, rdresdes, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CM_Free_Res_Des_Handle(rdresdes: usize) -> CONFIGRET {
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Free_Res_Des_Handle(rdresdes : usize) -> CONFIGRET);
    unsafe { CM_Free_Res_Des_Handle(rdresdes) }
}
#[inline]
pub unsafe fn CM_Free_Resource_Conflict_Handle(clconflictlist: usize) -> CONFIGRET {
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Free_Resource_Conflict_Handle(clconflictlist : usize) -> CONFIGRET);
    unsafe { CM_Free_Resource_Conflict_Handle(clconflictlist) }
}
#[inline]
pub unsafe fn CM_Get_Child(pdndevinst: *mut u32, dndevinst: u32, ulflags: u32) -> CONFIGRET {
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Get_Child(pdndevinst : *mut u32, dndevinst : u32, ulflags : u32) -> CONFIGRET);
    unsafe { CM_Get_Child(pdndevinst as _, dndevinst, ulflags) }
}
#[inline]
pub unsafe fn CM_Get_Child_Ex(pdndevinst: *mut u32, dndevinst: u32, ulflags: u32, hmachine: Option<isize>) -> CONFIGRET {
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Get_Child_Ex(pdndevinst : *mut u32, dndevinst : u32, ulflags : u32, hmachine : isize) -> CONFIGRET);
    unsafe { CM_Get_Child_Ex(pdndevinst as _, dndevinst, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CM_Get_Class_Key_NameA(classguid: *const windows_core::GUID, pszkeyname: Option<windows_core::PSTR>, pullength: *mut u32, ulflags: u32) -> CONFIGRET {
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Get_Class_Key_NameA(classguid : *const windows_core::GUID, pszkeyname : windows_core::PSTR, pullength : *mut u32, ulflags : u32) -> CONFIGRET);
    unsafe { CM_Get_Class_Key_NameA(classguid, pszkeyname.unwrap_or(core::mem::zeroed()) as _, pullength as _, ulflags) }
}
#[inline]
pub unsafe fn CM_Get_Class_Key_NameW(classguid: *const windows_core::GUID, pszkeyname: Option<windows_core::PWSTR>, pullength: *mut u32, ulflags: u32) -> CONFIGRET {
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Get_Class_Key_NameW(classguid : *const windows_core::GUID, pszkeyname : windows_core::PWSTR, pullength : *mut u32, ulflags : u32) -> CONFIGRET);
    unsafe { CM_Get_Class_Key_NameW(classguid, pszkeyname.unwrap_or(core::mem::zeroed()) as _, pullength as _, ulflags) }
}
#[inline]
pub unsafe fn CM_Get_Class_Key_Name_ExA(classguid: *const windows_core::GUID, pszkeyname: Option<windows_core::PSTR>, pullength: *mut u32, ulflags: u32, hmachine: Option<isize>) -> CONFIGRET {
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Get_Class_Key_Name_ExA(classguid : *const windows_core::GUID, pszkeyname : windows_core::PSTR, pullength : *mut u32, ulflags : u32, hmachine : isize) -> CONFIGRET);
    unsafe { CM_Get_Class_Key_Name_ExA(classguid, pszkeyname.unwrap_or(core::mem::zeroed()) as _, pullength as _, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CM_Get_Class_Key_Name_ExW(classguid: *const windows_core::GUID, pszkeyname: Option<windows_core::PWSTR>, pullength: *mut u32, ulflags: u32, hmachine: Option<isize>) -> CONFIGRET {
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Get_Class_Key_Name_ExW(classguid : *const windows_core::GUID, pszkeyname : windows_core::PWSTR, pullength : *mut u32, ulflags : u32, hmachine : isize) -> CONFIGRET);
    unsafe { CM_Get_Class_Key_Name_ExW(classguid, pszkeyname.unwrap_or(core::mem::zeroed()) as _, pullength as _, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CM_Get_Class_NameA(classguid: *const windows_core::GUID, buffer: Option<windows_core::PSTR>, pullength: *mut u32, ulflags: u32) -> CONFIGRET {
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Get_Class_NameA(classguid : *const windows_core::GUID, buffer : windows_core::PSTR, pullength : *mut u32, ulflags : u32) -> CONFIGRET);
    unsafe { CM_Get_Class_NameA(classguid, buffer.unwrap_or(core::mem::zeroed()) as _, pullength as _, ulflags) }
}
#[inline]
pub unsafe fn CM_Get_Class_NameW(classguid: *const windows_core::GUID, buffer: Option<windows_core::PWSTR>, pullength: *mut u32, ulflags: u32) -> CONFIGRET {
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Get_Class_NameW(classguid : *const windows_core::GUID, buffer : windows_core::PWSTR, pullength : *mut u32, ulflags : u32) -> CONFIGRET);
    unsafe { CM_Get_Class_NameW(classguid, buffer.unwrap_or(core::mem::zeroed()) as _, pullength as _, ulflags) }
}
#[inline]
pub unsafe fn CM_Get_Class_Name_ExA(classguid: *const windows_core::GUID, buffer: Option<windows_core::PSTR>, pullength: *mut u32, ulflags: u32, hmachine: Option<isize>) -> CONFIGRET {
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Get_Class_Name_ExA(classguid : *const windows_core::GUID, buffer : windows_core::PSTR, pullength : *mut u32, ulflags : u32, hmachine : isize) -> CONFIGRET);
    unsafe { CM_Get_Class_Name_ExA(classguid, buffer.unwrap_or(core::mem::zeroed()) as _, pullength as _, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CM_Get_Class_Name_ExW(classguid: *const windows_core::GUID, buffer: Option<windows_core::PWSTR>, pullength: *mut u32, ulflags: u32, hmachine: Option<isize>) -> CONFIGRET {
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Get_Class_Name_ExW(classguid : *const windows_core::GUID, buffer : windows_core::PWSTR, pullength : *mut u32, ulflags : u32, hmachine : isize) -> CONFIGRET);
    unsafe { CM_Get_Class_Name_ExW(classguid, buffer.unwrap_or(core::mem::zeroed()) as _, pullength as _, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "Win32_Devices_Properties")]
#[inline]
pub unsafe fn CM_Get_Class_PropertyW(classguid: *const windows_core::GUID, propertykey: *const super::super::Foundation::DEVPROPKEY, propertytype: *mut super::Properties::DEVPROPTYPE, propertybuffer: Option<*mut u8>, propertybuffersize: *mut u32, ulflags: u32) -> CONFIGRET {
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Get_Class_PropertyW(classguid : *const windows_core::GUID, propertykey : *const super::super::Foundation:: DEVPROPKEY, propertytype : *mut super::Properties:: DEVPROPTYPE, propertybuffer : *mut u8, propertybuffersize : *mut u32, ulflags : u32) -> CONFIGRET);
    unsafe { CM_Get_Class_PropertyW(classguid, propertykey, propertytype as _, propertybuffer.unwrap_or(core::mem::zeroed()) as _, propertybuffersize as _, ulflags) }
}
#[cfg(feature = "Win32_Devices_Properties")]
#[inline]
pub unsafe fn CM_Get_Class_Property_ExW(classguid: *const windows_core::GUID, propertykey: *const super::super::Foundation::DEVPROPKEY, propertytype: *mut super::Properties::DEVPROPTYPE, propertybuffer: Option<*mut u8>, propertybuffersize: *mut u32, ulflags: u32, hmachine: Option<isize>) -> CONFIGRET {
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Get_Class_Property_ExW(classguid : *const windows_core::GUID, propertykey : *const super::super::Foundation:: DEVPROPKEY, propertytype : *mut super::Properties:: DEVPROPTYPE, propertybuffer : *mut u8, propertybuffersize : *mut u32, ulflags : u32, hmachine : isize) -> CONFIGRET);
    unsafe { CM_Get_Class_Property_ExW(classguid, propertykey, propertytype as _, propertybuffer.unwrap_or(core::mem::zeroed()) as _, propertybuffersize as _, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CM_Get_Class_Property_Keys(classguid: *const windows_core::GUID, propertykeyarray: Option<*mut super::super::Foundation::DEVPROPKEY>, propertykeycount: *mut u32, ulflags: u32) -> CONFIGRET {
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Get_Class_Property_Keys(classguid : *const windows_core::GUID, propertykeyarray : *mut super::super::Foundation:: DEVPROPKEY, propertykeycount : *mut u32, ulflags : u32) -> CONFIGRET);
    unsafe { CM_Get_Class_Property_Keys(classguid, propertykeyarray.unwrap_or(core::mem::zeroed()) as _, propertykeycount as _, ulflags) }
}
#[inline]
pub unsafe fn CM_Get_Class_Property_Keys_Ex(classguid: *const windows_core::GUID, propertykeyarray: Option<*mut super::super::Foundation::DEVPROPKEY>, propertykeycount: *mut u32, ulflags: u32, hmachine: Option<isize>) -> CONFIGRET {
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Get_Class_Property_Keys_Ex(classguid : *const windows_core::GUID, propertykeyarray : *mut super::super::Foundation:: DEVPROPKEY, propertykeycount : *mut u32, ulflags : u32, hmachine : isize) -> CONFIGRET);
    unsafe { CM_Get_Class_Property_Keys_Ex(classguid, propertykeyarray.unwrap_or(core::mem::zeroed()) as _, propertykeycount as _, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CM_Get_Class_Registry_PropertyA(classguid: *const windows_core::GUID, ulproperty: u32, pulregdatatype: Option<*mut u32>, buffer: Option<*mut core::ffi::c_void>, pullength: *mut u32, ulflags: u32, hmachine: Option<isize>) -> CONFIGRET {
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Get_Class_Registry_PropertyA(classguid : *const windows_core::GUID, ulproperty : u32, pulregdatatype : *mut u32, buffer : *mut core::ffi::c_void, pullength : *mut u32, ulflags : u32, hmachine : isize) -> CONFIGRET);
    unsafe { CM_Get_Class_Registry_PropertyA(classguid, ulproperty, pulregdatatype.unwrap_or(core::mem::zeroed()) as _, buffer.unwrap_or(core::mem::zeroed()) as _, pullength as _, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CM_Get_Class_Registry_PropertyW(classguid: *const windows_core::GUID, ulproperty: u32, pulregdatatype: Option<*mut u32>, buffer: Option<*mut core::ffi::c_void>, pullength: *mut u32, ulflags: u32, hmachine: Option<isize>) -> CONFIGRET {
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Get_Class_Registry_PropertyW(classguid : *const windows_core::GUID, ulproperty : u32, pulregdatatype : *mut u32, buffer : *mut core::ffi::c_void, pullength : *mut u32, ulflags : u32, hmachine : isize) -> CONFIGRET);
    unsafe { CM_Get_Class_Registry_PropertyW(classguid, ulproperty, pulregdatatype.unwrap_or(core::mem::zeroed()) as _, buffer.unwrap_or(core::mem::zeroed()) as _, pullength as _, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CM_Get_Depth(puldepth: *mut u32, dndevinst: u32, ulflags: u32) -> CONFIGRET {
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Get_Depth(puldepth : *mut u32, dndevinst : u32, ulflags : u32) -> CONFIGRET);
    unsafe { CM_Get_Depth(puldepth as _, dndevinst, ulflags) }
}
#[inline]
pub unsafe fn CM_Get_Depth_Ex(puldepth: *mut u32, dndevinst: u32, ulflags: u32, hmachine: Option<isize>) -> CONFIGRET {
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Get_Depth_Ex(puldepth : *mut u32, dndevinst : u32, ulflags : u32, hmachine : isize) -> CONFIGRET);
    unsafe { CM_Get_Depth_Ex(puldepth as _, dndevinst, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CM_Get_DevNode_Custom_PropertyA<P1>(dndevinst: u32, pszcustompropertyname: P1, pulregdatatype: Option<*mut u32>, buffer: Option<*mut core::ffi::c_void>, pullength: *mut u32, ulflags: u32) -> CONFIGRET
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Get_DevNode_Custom_PropertyA(dndevinst : u32, pszcustompropertyname : windows_core::PCSTR, pulregdatatype : *mut u32, buffer : *mut core::ffi::c_void, pullength : *mut u32, ulflags : u32) -> CONFIGRET);
    unsafe { CM_Get_DevNode_Custom_PropertyA(dndevinst, pszcustompropertyname.param().abi(), pulregdatatype.unwrap_or(core::mem::zeroed()) as _, buffer.unwrap_or(core::mem::zeroed()) as _, pullength as _, ulflags) }
}
#[inline]
pub unsafe fn CM_Get_DevNode_Custom_PropertyW<P1>(dndevinst: u32, pszcustompropertyname: P1, pulregdatatype: Option<*mut u32>, buffer: Option<*mut core::ffi::c_void>, pullength: *mut u32, ulflags: u32) -> CONFIGRET
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Get_DevNode_Custom_PropertyW(dndevinst : u32, pszcustompropertyname : windows_core::PCWSTR, pulregdatatype : *mut u32, buffer : *mut core::ffi::c_void, pullength : *mut u32, ulflags : u32) -> CONFIGRET);
    unsafe { CM_Get_DevNode_Custom_PropertyW(dndevinst, pszcustompropertyname.param().abi(), pulregdatatype.unwrap_or(core::mem::zeroed()) as _, buffer.unwrap_or(core::mem::zeroed()) as _, pullength as _, ulflags) }
}
#[inline]
pub unsafe fn CM_Get_DevNode_Custom_Property_ExA<P1>(dndevinst: u32, pszcustompropertyname: P1, pulregdatatype: Option<*mut u32>, buffer: Option<*mut core::ffi::c_void>, pullength: *mut u32, ulflags: u32, hmachine: Option<isize>) -> CONFIGRET
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Get_DevNode_Custom_Property_ExA(dndevinst : u32, pszcustompropertyname : windows_core::PCSTR, pulregdatatype : *mut u32, buffer : *mut core::ffi::c_void, pullength : *mut u32, ulflags : u32, hmachine : isize) -> CONFIGRET);
    unsafe { CM_Get_DevNode_Custom_Property_ExA(dndevinst, pszcustompropertyname.param().abi(), pulregdatatype.unwrap_or(core::mem::zeroed()) as _, buffer.unwrap_or(core::mem::zeroed()) as _, pullength as _, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CM_Get_DevNode_Custom_Property_ExW<P1>(dndevinst: u32, pszcustompropertyname: P1, pulregdatatype: Option<*mut u32>, buffer: Option<*mut core::ffi::c_void>, pullength: *mut u32, ulflags: u32, hmachine: Option<isize>) -> CONFIGRET
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Get_DevNode_Custom_Property_ExW(dndevinst : u32, pszcustompropertyname : windows_core::PCWSTR, pulregdatatype : *mut u32, buffer : *mut core::ffi::c_void, pullength : *mut u32, ulflags : u32, hmachine : isize) -> CONFIGRET);
    unsafe { CM_Get_DevNode_Custom_Property_ExW(dndevinst, pszcustompropertyname.param().abi(), pulregdatatype.unwrap_or(core::mem::zeroed()) as _, buffer.unwrap_or(core::mem::zeroed()) as _, pullength as _, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "Win32_Devices_Properties")]
#[inline]
pub unsafe fn CM_Get_DevNode_PropertyW(dndevinst: u32, propertykey: *const super::super::Foundation::DEVPROPKEY, propertytype: *mut super::Properties::DEVPROPTYPE, propertybuffer: Option<*mut u8>, propertybuffersize: *mut u32, ulflags: u32) -> CONFIGRET {
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Get_DevNode_PropertyW(dndevinst : u32, propertykey : *const super::super::Foundation:: DEVPROPKEY, propertytype : *mut super::Properties:: DEVPROPTYPE, propertybuffer : *mut u8, propertybuffersize : *mut u32, ulflags : u32) -> CONFIGRET);
    unsafe { CM_Get_DevNode_PropertyW(dndevinst, propertykey, propertytype as _, propertybuffer.unwrap_or(core::mem::zeroed()) as _, propertybuffersize as _, ulflags) }
}
#[cfg(feature = "Win32_Devices_Properties")]
#[inline]
pub unsafe fn CM_Get_DevNode_Property_ExW(dndevinst: u32, propertykey: *const super::super::Foundation::DEVPROPKEY, propertytype: *mut super::Properties::DEVPROPTYPE, propertybuffer: Option<*mut u8>, propertybuffersize: *mut u32, ulflags: u32, hmachine: Option<isize>) -> CONFIGRET {
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Get_DevNode_Property_ExW(dndevinst : u32, propertykey : *const super::super::Foundation:: DEVPROPKEY, propertytype : *mut super::Properties:: DEVPROPTYPE, propertybuffer : *mut u8, propertybuffersize : *mut u32, ulflags : u32, hmachine : isize) -> CONFIGRET);
    unsafe { CM_Get_DevNode_Property_ExW(dndevinst, propertykey, propertytype as _, propertybuffer.unwrap_or(core::mem::zeroed()) as _, propertybuffersize as _, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CM_Get_DevNode_Property_Keys(dndevinst: u32, propertykeyarray: Option<*mut super::super::Foundation::DEVPROPKEY>, propertykeycount: *mut u32, ulflags: u32) -> CONFIGRET {
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Get_DevNode_Property_Keys(dndevinst : u32, propertykeyarray : *mut super::super::Foundation:: DEVPROPKEY, propertykeycount : *mut u32, ulflags : u32) -> CONFIGRET);
    unsafe { CM_Get_DevNode_Property_Keys(dndevinst, propertykeyarray.unwrap_or(core::mem::zeroed()) as _, propertykeycount as _, ulflags) }
}
#[inline]
pub unsafe fn CM_Get_DevNode_Property_Keys_Ex(dndevinst: u32, propertykeyarray: Option<*mut super::super::Foundation::DEVPROPKEY>, propertykeycount: *mut u32, ulflags: u32, hmachine: Option<isize>) -> CONFIGRET {
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Get_DevNode_Property_Keys_Ex(dndevinst : u32, propertykeyarray : *mut super::super::Foundation:: DEVPROPKEY, propertykeycount : *mut u32, ulflags : u32, hmachine : isize) -> CONFIGRET);
    unsafe { CM_Get_DevNode_Property_Keys_Ex(dndevinst, propertykeyarray.unwrap_or(core::mem::zeroed()) as _, propertykeycount as _, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CM_Get_DevNode_Registry_PropertyA(dndevinst: u32, ulproperty: u32, pulregdatatype: Option<*mut u32>, buffer: Option<*mut core::ffi::c_void>, pullength: *mut u32, ulflags: u32) -> CONFIGRET {
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Get_DevNode_Registry_PropertyA(dndevinst : u32, ulproperty : u32, pulregdatatype : *mut u32, buffer : *mut core::ffi::c_void, pullength : *mut u32, ulflags : u32) -> CONFIGRET);
    unsafe { CM_Get_DevNode_Registry_PropertyA(dndevinst, ulproperty, pulregdatatype.unwrap_or(core::mem::zeroed()) as _, buffer.unwrap_or(core::mem::zeroed()) as _, pullength as _, ulflags) }
}
#[inline]
pub unsafe fn CM_Get_DevNode_Registry_PropertyW(dndevinst: u32, ulproperty: u32, pulregdatatype: Option<*mut u32>, buffer: Option<*mut core::ffi::c_void>, pullength: *mut u32, ulflags: u32) -> CONFIGRET {
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Get_DevNode_Registry_PropertyW(dndevinst : u32, ulproperty : u32, pulregdatatype : *mut u32, buffer : *mut core::ffi::c_void, pullength : *mut u32, ulflags : u32) -> CONFIGRET);
    unsafe { CM_Get_DevNode_Registry_PropertyW(dndevinst, ulproperty, pulregdatatype.unwrap_or(core::mem::zeroed()) as _, buffer.unwrap_or(core::mem::zeroed()) as _, pullength as _, ulflags) }
}
#[inline]
pub unsafe fn CM_Get_DevNode_Registry_Property_ExA(dndevinst: u32, ulproperty: u32, pulregdatatype: Option<*mut u32>, buffer: Option<*mut core::ffi::c_void>, pullength: *mut u32, ulflags: u32, hmachine: Option<isize>) -> CONFIGRET {
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Get_DevNode_Registry_Property_ExA(dndevinst : u32, ulproperty : u32, pulregdatatype : *mut u32, buffer : *mut core::ffi::c_void, pullength : *mut u32, ulflags : u32, hmachine : isize) -> CONFIGRET);
    unsafe { CM_Get_DevNode_Registry_Property_ExA(dndevinst, ulproperty, pulregdatatype.unwrap_or(core::mem::zeroed()) as _, buffer.unwrap_or(core::mem::zeroed()) as _, pullength as _, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CM_Get_DevNode_Registry_Property_ExW(dndevinst: u32, ulproperty: u32, pulregdatatype: Option<*mut u32>, buffer: Option<*mut core::ffi::c_void>, pullength: *mut u32, ulflags: u32, hmachine: Option<isize>) -> CONFIGRET {
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Get_DevNode_Registry_Property_ExW(dndevinst : u32, ulproperty : u32, pulregdatatype : *mut u32, buffer : *mut core::ffi::c_void, pullength : *mut u32, ulflags : u32, hmachine : isize) -> CONFIGRET);
    unsafe { CM_Get_DevNode_Registry_Property_ExW(dndevinst, ulproperty, pulregdatatype.unwrap_or(core::mem::zeroed()) as _, buffer.unwrap_or(core::mem::zeroed()) as _, pullength as _, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CM_Get_DevNode_Status(pulstatus: *mut CM_DEVNODE_STATUS_FLAGS, pulproblemnumber: *mut CM_PROB, dndevinst: u32, ulflags: u32) -> CONFIGRET {
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Get_DevNode_Status(pulstatus : *mut CM_DEVNODE_STATUS_FLAGS, pulproblemnumber : *mut CM_PROB, dndevinst : u32, ulflags : u32) -> CONFIGRET);
    unsafe { CM_Get_DevNode_Status(pulstatus as _, pulproblemnumber as _, dndevinst, ulflags) }
}
#[inline]
pub unsafe fn CM_Get_DevNode_Status_Ex(pulstatus: *mut CM_DEVNODE_STATUS_FLAGS, pulproblemnumber: *mut CM_PROB, dndevinst: u32, ulflags: u32, hmachine: Option<isize>) -> CONFIGRET {
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Get_DevNode_Status_Ex(pulstatus : *mut CM_DEVNODE_STATUS_FLAGS, pulproblemnumber : *mut CM_PROB, dndevinst : u32, ulflags : u32, hmachine : isize) -> CONFIGRET);
    unsafe { CM_Get_DevNode_Status_Ex(pulstatus as _, pulproblemnumber as _, dndevinst, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CM_Get_Device_IDA(dndevinst: u32, buffer: &mut [u8], ulflags: u32) -> CONFIGRET {
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Get_Device_IDA(dndevinst : u32, buffer : windows_core::PSTR, bufferlen : u32, ulflags : u32) -> CONFIGRET);
    unsafe { CM_Get_Device_IDA(dndevinst, core::mem::transmute(buffer.as_ptr()), buffer.len().try_into().unwrap(), ulflags) }
}
#[inline]
pub unsafe fn CM_Get_Device_IDW(dndevinst: u32, buffer: &mut [u16], ulflags: u32) -> CONFIGRET {
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Get_Device_IDW(dndevinst : u32, buffer : windows_core::PWSTR, bufferlen : u32, ulflags : u32) -> CONFIGRET);
    unsafe { CM_Get_Device_IDW(dndevinst, core::mem::transmute(buffer.as_ptr()), buffer.len().try_into().unwrap(), ulflags) }
}
#[inline]
pub unsafe fn CM_Get_Device_ID_ExA(dndevinst: u32, buffer: &mut [u8], ulflags: u32, hmachine: Option<isize>) -> CONFIGRET {
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Get_Device_ID_ExA(dndevinst : u32, buffer : windows_core::PSTR, bufferlen : u32, ulflags : u32, hmachine : isize) -> CONFIGRET);
    unsafe { CM_Get_Device_ID_ExA(dndevinst, core::mem::transmute(buffer.as_ptr()), buffer.len().try_into().unwrap(), ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CM_Get_Device_ID_ExW(dndevinst: u32, buffer: &mut [u16], ulflags: u32, hmachine: Option<isize>) -> CONFIGRET {
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Get_Device_ID_ExW(dndevinst : u32, buffer : windows_core::PWSTR, bufferlen : u32, ulflags : u32, hmachine : isize) -> CONFIGRET);
    unsafe { CM_Get_Device_ID_ExW(dndevinst, core::mem::transmute(buffer.as_ptr()), buffer.len().try_into().unwrap(), ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CM_Get_Device_ID_ListA<P0>(pszfilter: P0, buffer: &mut [u8], ulflags: u32) -> CONFIGRET
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Get_Device_ID_ListA(pszfilter : windows_core::PCSTR, buffer : windows_core::PSTR, bufferlen : u32, ulflags : u32) -> CONFIGRET);
    unsafe { CM_Get_Device_ID_ListA(pszfilter.param().abi(), core::mem::transmute(buffer.as_ptr()), buffer.len().try_into().unwrap(), ulflags) }
}
#[inline]
pub unsafe fn CM_Get_Device_ID_ListW<P0>(pszfilter: P0, buffer: &mut [u16], ulflags: u32) -> CONFIGRET
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Get_Device_ID_ListW(pszfilter : windows_core::PCWSTR, buffer : windows_core::PWSTR, bufferlen : u32, ulflags : u32) -> CONFIGRET);
    unsafe { CM_Get_Device_ID_ListW(pszfilter.param().abi(), core::mem::transmute(buffer.as_ptr()), buffer.len().try_into().unwrap(), ulflags) }
}
#[inline]
pub unsafe fn CM_Get_Device_ID_List_ExA<P0>(pszfilter: P0, buffer: &mut [u8], ulflags: u32, hmachine: Option<isize>) -> CONFIGRET
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Get_Device_ID_List_ExA(pszfilter : windows_core::PCSTR, buffer : windows_core::PSTR, bufferlen : u32, ulflags : u32, hmachine : isize) -> CONFIGRET);
    unsafe { CM_Get_Device_ID_List_ExA(pszfilter.param().abi(), core::mem::transmute(buffer.as_ptr()), buffer.len().try_into().unwrap(), ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CM_Get_Device_ID_List_ExW<P0>(pszfilter: P0, buffer: &mut [u16], ulflags: u32, hmachine: Option<isize>) -> CONFIGRET
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Get_Device_ID_List_ExW(pszfilter : windows_core::PCWSTR, buffer : windows_core::PWSTR, bufferlen : u32, ulflags : u32, hmachine : isize) -> CONFIGRET);
    unsafe { CM_Get_Device_ID_List_ExW(pszfilter.param().abi(), core::mem::transmute(buffer.as_ptr()), buffer.len().try_into().unwrap(), ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CM_Get_Device_ID_List_SizeA<P1>(pullen: *mut u32, pszfilter: P1, ulflags: u32) -> CONFIGRET
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Get_Device_ID_List_SizeA(pullen : *mut u32, pszfilter : windows_core::PCSTR, ulflags : u32) -> CONFIGRET);
    unsafe { CM_Get_Device_ID_List_SizeA(pullen as _, pszfilter.param().abi(), ulflags) }
}
#[inline]
pub unsafe fn CM_Get_Device_ID_List_SizeW<P1>(pullen: *mut u32, pszfilter: P1, ulflags: u32) -> CONFIGRET
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Get_Device_ID_List_SizeW(pullen : *mut u32, pszfilter : windows_core::PCWSTR, ulflags : u32) -> CONFIGRET);
    unsafe { CM_Get_Device_ID_List_SizeW(pullen as _, pszfilter.param().abi(), ulflags) }
}
#[inline]
pub unsafe fn CM_Get_Device_ID_List_Size_ExA<P1>(pullen: *mut u32, pszfilter: P1, ulflags: u32, hmachine: Option<isize>) -> CONFIGRET
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Get_Device_ID_List_Size_ExA(pullen : *mut u32, pszfilter : windows_core::PCSTR, ulflags : u32, hmachine : isize) -> CONFIGRET);
    unsafe { CM_Get_Device_ID_List_Size_ExA(pullen as _, pszfilter.param().abi(), ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CM_Get_Device_ID_List_Size_ExW<P1>(pullen: *mut u32, pszfilter: P1, ulflags: u32, hmachine: Option<isize>) -> CONFIGRET
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Get_Device_ID_List_Size_ExW(pullen : *mut u32, pszfilter : windows_core::PCWSTR, ulflags : u32, hmachine : isize) -> CONFIGRET);
    unsafe { CM_Get_Device_ID_List_Size_ExW(pullen as _, pszfilter.param().abi(), ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CM_Get_Device_ID_Size(pullen: *mut u32, dndevinst: u32, ulflags: u32) -> CONFIGRET {
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Get_Device_ID_Size(pullen : *mut u32, dndevinst : u32, ulflags : u32) -> CONFIGRET);
    unsafe { CM_Get_Device_ID_Size(pullen as _, dndevinst, ulflags) }
}
#[inline]
pub unsafe fn CM_Get_Device_ID_Size_Ex(pullen: *mut u32, dndevinst: u32, ulflags: u32, hmachine: Option<isize>) -> CONFIGRET {
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Get_Device_ID_Size_Ex(pullen : *mut u32, dndevinst : u32, ulflags : u32, hmachine : isize) -> CONFIGRET);
    unsafe { CM_Get_Device_ID_Size_Ex(pullen as _, dndevinst, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CM_Get_Device_Interface_AliasA<P0>(pszdeviceinterface: P0, aliasinterfaceguid: *const windows_core::GUID, pszaliasdeviceinterface: windows_core::PSTR, pullength: *mut u32, ulflags: u32) -> CONFIGRET
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Get_Device_Interface_AliasA(pszdeviceinterface : windows_core::PCSTR, aliasinterfaceguid : *const windows_core::GUID, pszaliasdeviceinterface : windows_core::PSTR, pullength : *mut u32, ulflags : u32) -> CONFIGRET);
    unsafe { CM_Get_Device_Interface_AliasA(pszdeviceinterface.param().abi(), aliasinterfaceguid, core::mem::transmute(pszaliasdeviceinterface), pullength as _, ulflags) }
}
#[inline]
pub unsafe fn CM_Get_Device_Interface_AliasW<P0>(pszdeviceinterface: P0, aliasinterfaceguid: *const windows_core::GUID, pszaliasdeviceinterface: windows_core::PWSTR, pullength: *mut u32, ulflags: u32) -> CONFIGRET
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Get_Device_Interface_AliasW(pszdeviceinterface : windows_core::PCWSTR, aliasinterfaceguid : *const windows_core::GUID, pszaliasdeviceinterface : windows_core::PWSTR, pullength : *mut u32, ulflags : u32) -> CONFIGRET);
    unsafe { CM_Get_Device_Interface_AliasW(pszdeviceinterface.param().abi(), aliasinterfaceguid, core::mem::transmute(pszaliasdeviceinterface), pullength as _, ulflags) }
}
#[inline]
pub unsafe fn CM_Get_Device_Interface_Alias_ExA<P0>(pszdeviceinterface: P0, aliasinterfaceguid: *const windows_core::GUID, pszaliasdeviceinterface: windows_core::PSTR, pullength: *mut u32, ulflags: u32, hmachine: Option<isize>) -> CONFIGRET
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Get_Device_Interface_Alias_ExA(pszdeviceinterface : windows_core::PCSTR, aliasinterfaceguid : *const windows_core::GUID, pszaliasdeviceinterface : windows_core::PSTR, pullength : *mut u32, ulflags : u32, hmachine : isize) -> CONFIGRET);
    unsafe { CM_Get_Device_Interface_Alias_ExA(pszdeviceinterface.param().abi(), aliasinterfaceguid, core::mem::transmute(pszaliasdeviceinterface), pullength as _, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CM_Get_Device_Interface_Alias_ExW<P0>(pszdeviceinterface: P0, aliasinterfaceguid: *const windows_core::GUID, pszaliasdeviceinterface: windows_core::PWSTR, pullength: *mut u32, ulflags: u32, hmachine: Option<isize>) -> CONFIGRET
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Get_Device_Interface_Alias_ExW(pszdeviceinterface : windows_core::PCWSTR, aliasinterfaceguid : *const windows_core::GUID, pszaliasdeviceinterface : windows_core::PWSTR, pullength : *mut u32, ulflags : u32, hmachine : isize) -> CONFIGRET);
    unsafe { CM_Get_Device_Interface_Alias_ExW(pszdeviceinterface.param().abi(), aliasinterfaceguid, core::mem::transmute(pszaliasdeviceinterface), pullength as _, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CM_Get_Device_Interface_ListA<P1>(interfaceclassguid: *const windows_core::GUID, pdeviceid: P1, buffer: &mut [u8], ulflags: CM_GET_DEVICE_INTERFACE_LIST_FLAGS) -> CONFIGRET
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Get_Device_Interface_ListA(interfaceclassguid : *const windows_core::GUID, pdeviceid : windows_core::PCSTR, buffer : windows_core::PSTR, bufferlen : u32, ulflags : CM_GET_DEVICE_INTERFACE_LIST_FLAGS) -> CONFIGRET);
    unsafe { CM_Get_Device_Interface_ListA(interfaceclassguid, pdeviceid.param().abi(), core::mem::transmute(buffer.as_ptr()), buffer.len().try_into().unwrap(), ulflags) }
}
#[inline]
pub unsafe fn CM_Get_Device_Interface_ListW<P1>(interfaceclassguid: *const windows_core::GUID, pdeviceid: P1, buffer: &mut [u16], ulflags: CM_GET_DEVICE_INTERFACE_LIST_FLAGS) -> CONFIGRET
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Get_Device_Interface_ListW(interfaceclassguid : *const windows_core::GUID, pdeviceid : windows_core::PCWSTR, buffer : windows_core::PWSTR, bufferlen : u32, ulflags : CM_GET_DEVICE_INTERFACE_LIST_FLAGS) -> CONFIGRET);
    unsafe { CM_Get_Device_Interface_ListW(interfaceclassguid, pdeviceid.param().abi(), core::mem::transmute(buffer.as_ptr()), buffer.len().try_into().unwrap(), ulflags) }
}
#[inline]
pub unsafe fn CM_Get_Device_Interface_List_ExA<P1>(interfaceclassguid: *const windows_core::GUID, pdeviceid: P1, buffer: &mut [u8], ulflags: CM_GET_DEVICE_INTERFACE_LIST_FLAGS, hmachine: Option<isize>) -> CONFIGRET
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Get_Device_Interface_List_ExA(interfaceclassguid : *const windows_core::GUID, pdeviceid : windows_core::PCSTR, buffer : windows_core::PSTR, bufferlen : u32, ulflags : CM_GET_DEVICE_INTERFACE_LIST_FLAGS, hmachine : isize) -> CONFIGRET);
    unsafe { CM_Get_Device_Interface_List_ExA(interfaceclassguid, pdeviceid.param().abi(), core::mem::transmute(buffer.as_ptr()), buffer.len().try_into().unwrap(), ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CM_Get_Device_Interface_List_ExW<P1>(interfaceclassguid: *const windows_core::GUID, pdeviceid: P1, buffer: &mut [u16], ulflags: CM_GET_DEVICE_INTERFACE_LIST_FLAGS, hmachine: Option<isize>) -> CONFIGRET
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Get_Device_Interface_List_ExW(interfaceclassguid : *const windows_core::GUID, pdeviceid : windows_core::PCWSTR, buffer : windows_core::PWSTR, bufferlen : u32, ulflags : CM_GET_DEVICE_INTERFACE_LIST_FLAGS, hmachine : isize) -> CONFIGRET);
    unsafe { CM_Get_Device_Interface_List_ExW(interfaceclassguid, pdeviceid.param().abi(), core::mem::transmute(buffer.as_ptr()), buffer.len().try_into().unwrap(), ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CM_Get_Device_Interface_List_SizeA<P2>(pullen: *mut u32, interfaceclassguid: *const windows_core::GUID, pdeviceid: P2, ulflags: CM_GET_DEVICE_INTERFACE_LIST_FLAGS) -> CONFIGRET
where
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Get_Device_Interface_List_SizeA(pullen : *mut u32, interfaceclassguid : *const windows_core::GUID, pdeviceid : windows_core::PCSTR, ulflags : CM_GET_DEVICE_INTERFACE_LIST_FLAGS) -> CONFIGRET);
    unsafe { CM_Get_Device_Interface_List_SizeA(pullen as _, interfaceclassguid, pdeviceid.param().abi(), ulflags) }
}
#[inline]
pub unsafe fn CM_Get_Device_Interface_List_SizeW<P2>(pullen: *mut u32, interfaceclassguid: *const windows_core::GUID, pdeviceid: P2, ulflags: CM_GET_DEVICE_INTERFACE_LIST_FLAGS) -> CONFIGRET
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Get_Device_Interface_List_SizeW(pullen : *mut u32, interfaceclassguid : *const windows_core::GUID, pdeviceid : windows_core::PCWSTR, ulflags : CM_GET_DEVICE_INTERFACE_LIST_FLAGS) -> CONFIGRET);
    unsafe { CM_Get_Device_Interface_List_SizeW(pullen as _, interfaceclassguid, pdeviceid.param().abi(), ulflags) }
}
#[inline]
pub unsafe fn CM_Get_Device_Interface_List_Size_ExA<P2>(pullen: *mut u32, interfaceclassguid: *const windows_core::GUID, pdeviceid: P2, ulflags: CM_GET_DEVICE_INTERFACE_LIST_FLAGS, hmachine: Option<isize>) -> CONFIGRET
where
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Get_Device_Interface_List_Size_ExA(pullen : *mut u32, interfaceclassguid : *const windows_core::GUID, pdeviceid : windows_core::PCSTR, ulflags : CM_GET_DEVICE_INTERFACE_LIST_FLAGS, hmachine : isize) -> CONFIGRET);
    unsafe { CM_Get_Device_Interface_List_Size_ExA(pullen as _, interfaceclassguid, pdeviceid.param().abi(), ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CM_Get_Device_Interface_List_Size_ExW<P2>(pullen: *mut u32, interfaceclassguid: *const windows_core::GUID, pdeviceid: P2, ulflags: CM_GET_DEVICE_INTERFACE_LIST_FLAGS, hmachine: Option<isize>) -> CONFIGRET
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Get_Device_Interface_List_Size_ExW(pullen : *mut u32, interfaceclassguid : *const windows_core::GUID, pdeviceid : windows_core::PCWSTR, ulflags : CM_GET_DEVICE_INTERFACE_LIST_FLAGS, hmachine : isize) -> CONFIGRET);
    unsafe { CM_Get_Device_Interface_List_Size_ExW(pullen as _, interfaceclassguid, pdeviceid.param().abi(), ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "Win32_Devices_Properties")]
#[inline]
pub unsafe fn CM_Get_Device_Interface_PropertyW<P0>(pszdeviceinterface: P0, propertykey: *const super::super::Foundation::DEVPROPKEY, propertytype: *mut super::Properties::DEVPROPTYPE, propertybuffer: Option<*mut u8>, propertybuffersize: *mut u32, ulflags: u32) -> CONFIGRET
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Get_Device_Interface_PropertyW(pszdeviceinterface : windows_core::PCWSTR, propertykey : *const super::super::Foundation:: DEVPROPKEY, propertytype : *mut super::Properties:: DEVPROPTYPE, propertybuffer : *mut u8, propertybuffersize : *mut u32, ulflags : u32) -> CONFIGRET);
    unsafe { CM_Get_Device_Interface_PropertyW(pszdeviceinterface.param().abi(), propertykey, propertytype as _, propertybuffer.unwrap_or(core::mem::zeroed()) as _, propertybuffersize as _, ulflags) }
}
#[cfg(feature = "Win32_Devices_Properties")]
#[inline]
pub unsafe fn CM_Get_Device_Interface_Property_ExW<P0>(pszdeviceinterface: P0, propertykey: *const super::super::Foundation::DEVPROPKEY, propertytype: *mut super::Properties::DEVPROPTYPE, propertybuffer: Option<*mut u8>, propertybuffersize: *mut u32, ulflags: u32, hmachine: Option<isize>) -> CONFIGRET
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Get_Device_Interface_Property_ExW(pszdeviceinterface : windows_core::PCWSTR, propertykey : *const super::super::Foundation:: DEVPROPKEY, propertytype : *mut super::Properties:: DEVPROPTYPE, propertybuffer : *mut u8, propertybuffersize : *mut u32, ulflags : u32, hmachine : isize) -> CONFIGRET);
    unsafe { CM_Get_Device_Interface_Property_ExW(pszdeviceinterface.param().abi(), propertykey, propertytype as _, propertybuffer.unwrap_or(core::mem::zeroed()) as _, propertybuffersize as _, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CM_Get_Device_Interface_Property_KeysW<P0>(pszdeviceinterface: P0, propertykeyarray: Option<*mut super::super::Foundation::DEVPROPKEY>, propertykeycount: *mut u32, ulflags: u32) -> CONFIGRET
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Get_Device_Interface_Property_KeysW(pszdeviceinterface : windows_core::PCWSTR, propertykeyarray : *mut super::super::Foundation:: DEVPROPKEY, propertykeycount : *mut u32, ulflags : u32) -> CONFIGRET);
    unsafe { CM_Get_Device_Interface_Property_KeysW(pszdeviceinterface.param().abi(), propertykeyarray.unwrap_or(core::mem::zeroed()) as _, propertykeycount as _, ulflags) }
}
#[inline]
pub unsafe fn CM_Get_Device_Interface_Property_Keys_ExW<P0>(pszdeviceinterface: P0, propertykeyarray: Option<*mut super::super::Foundation::DEVPROPKEY>, propertykeycount: *mut u32, ulflags: u32, hmachine: Option<isize>) -> CONFIGRET
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Get_Device_Interface_Property_Keys_ExW(pszdeviceinterface : windows_core::PCWSTR, propertykeyarray : *mut super::super::Foundation:: DEVPROPKEY, propertykeycount : *mut u32, ulflags : u32, hmachine : isize) -> CONFIGRET);
    unsafe { CM_Get_Device_Interface_Property_Keys_ExW(pszdeviceinterface.param().abi(), propertykeyarray.unwrap_or(core::mem::zeroed()) as _, propertykeycount as _, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CM_Get_First_Log_Conf(plclogconf: Option<*mut usize>, dndevinst: u32, ulflags: CM_LOG_CONF) -> CONFIGRET {
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Get_First_Log_Conf(plclogconf : *mut usize, dndevinst : u32, ulflags : CM_LOG_CONF) -> CONFIGRET);
    unsafe { CM_Get_First_Log_Conf(plclogconf.unwrap_or(core::mem::zeroed()) as _, dndevinst, ulflags) }
}
#[inline]
pub unsafe fn CM_Get_First_Log_Conf_Ex(plclogconf: Option<*mut usize>, dndevinst: u32, ulflags: CM_LOG_CONF, hmachine: Option<isize>) -> CONFIGRET {
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Get_First_Log_Conf_Ex(plclogconf : *mut usize, dndevinst : u32, ulflags : CM_LOG_CONF, hmachine : isize) -> CONFIGRET);
    unsafe { CM_Get_First_Log_Conf_Ex(plclogconf.unwrap_or(core::mem::zeroed()) as _, dndevinst, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CM_Get_Global_State(pulstate: *mut u32, ulflags: u32) -> CONFIGRET {
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Get_Global_State(pulstate : *mut u32, ulflags : u32) -> CONFIGRET);
    unsafe { CM_Get_Global_State(pulstate as _, ulflags) }
}
#[inline]
pub unsafe fn CM_Get_Global_State_Ex(pulstate: *mut u32, ulflags: u32, hmachine: Option<isize>) -> CONFIGRET {
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Get_Global_State_Ex(pulstate : *mut u32, ulflags : u32, hmachine : isize) -> CONFIGRET);
    unsafe { CM_Get_Global_State_Ex(pulstate as _, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CM_Get_HW_Prof_FlagsA<P0>(pdeviceid: P0, ulhardwareprofile: u32, pulvalue: *mut u32, ulflags: u32) -> CONFIGRET
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Get_HW_Prof_FlagsA(pdeviceid : windows_core::PCSTR, ulhardwareprofile : u32, pulvalue : *mut u32, ulflags : u32) -> CONFIGRET);
    unsafe { CM_Get_HW_Prof_FlagsA(pdeviceid.param().abi(), ulhardwareprofile, pulvalue as _, ulflags) }
}
#[inline]
pub unsafe fn CM_Get_HW_Prof_FlagsW<P0>(pdeviceid: P0, ulhardwareprofile: u32, pulvalue: *mut u32, ulflags: u32) -> CONFIGRET
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Get_HW_Prof_FlagsW(pdeviceid : windows_core::PCWSTR, ulhardwareprofile : u32, pulvalue : *mut u32, ulflags : u32) -> CONFIGRET);
    unsafe { CM_Get_HW_Prof_FlagsW(pdeviceid.param().abi(), ulhardwareprofile, pulvalue as _, ulflags) }
}
#[inline]
pub unsafe fn CM_Get_HW_Prof_Flags_ExA<P0>(pdeviceid: P0, ulhardwareprofile: u32, pulvalue: *mut u32, ulflags: u32, hmachine: Option<isize>) -> CONFIGRET
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Get_HW_Prof_Flags_ExA(pdeviceid : windows_core::PCSTR, ulhardwareprofile : u32, pulvalue : *mut u32, ulflags : u32, hmachine : isize) -> CONFIGRET);
    unsafe { CM_Get_HW_Prof_Flags_ExA(pdeviceid.param().abi(), ulhardwareprofile, pulvalue as _, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CM_Get_HW_Prof_Flags_ExW<P0>(pdeviceid: P0, ulhardwareprofile: u32, pulvalue: *mut u32, ulflags: u32, hmachine: Option<isize>) -> CONFIGRET
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Get_HW_Prof_Flags_ExW(pdeviceid : windows_core::PCWSTR, ulhardwareprofile : u32, pulvalue : *mut u32, ulflags : u32, hmachine : isize) -> CONFIGRET);
    unsafe { CM_Get_HW_Prof_Flags_ExW(pdeviceid.param().abi(), ulhardwareprofile, pulvalue as _, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CM_Get_Hardware_Profile_InfoA(ulindex: u32, phwprofileinfo: *mut HWPROFILEINFO_A, ulflags: u32) -> CONFIGRET {
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Get_Hardware_Profile_InfoA(ulindex : u32, phwprofileinfo : *mut HWPROFILEINFO_A, ulflags : u32) -> CONFIGRET);
    unsafe { CM_Get_Hardware_Profile_InfoA(ulindex, phwprofileinfo as _, ulflags) }
}
#[inline]
pub unsafe fn CM_Get_Hardware_Profile_InfoW(ulindex: u32, phwprofileinfo: *mut HWPROFILEINFO_W, ulflags: u32) -> CONFIGRET {
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Get_Hardware_Profile_InfoW(ulindex : u32, phwprofileinfo : *mut HWPROFILEINFO_W, ulflags : u32) -> CONFIGRET);
    unsafe { CM_Get_Hardware_Profile_InfoW(ulindex, phwprofileinfo as _, ulflags) }
}
#[inline]
pub unsafe fn CM_Get_Hardware_Profile_Info_ExA(ulindex: u32, phwprofileinfo: *mut HWPROFILEINFO_A, ulflags: u32, hmachine: Option<isize>) -> CONFIGRET {
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Get_Hardware_Profile_Info_ExA(ulindex : u32, phwprofileinfo : *mut HWPROFILEINFO_A, ulflags : u32, hmachine : isize) -> CONFIGRET);
    unsafe { CM_Get_Hardware_Profile_Info_ExA(ulindex, phwprofileinfo as _, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CM_Get_Hardware_Profile_Info_ExW(ulindex: u32, phwprofileinfo: *mut HWPROFILEINFO_W, ulflags: u32, hmachine: Option<isize>) -> CONFIGRET {
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Get_Hardware_Profile_Info_ExW(ulindex : u32, phwprofileinfo : *mut HWPROFILEINFO_W, ulflags : u32, hmachine : isize) -> CONFIGRET);
    unsafe { CM_Get_Hardware_Profile_Info_ExW(ulindex, phwprofileinfo as _, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CM_Get_Log_Conf_Priority(lclogconf: usize, ppriority: *mut u32, ulflags: u32) -> CONFIGRET {
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Get_Log_Conf_Priority(lclogconf : usize, ppriority : *mut u32, ulflags : u32) -> CONFIGRET);
    unsafe { CM_Get_Log_Conf_Priority(lclogconf, ppriority as _, ulflags) }
}
#[inline]
pub unsafe fn CM_Get_Log_Conf_Priority_Ex(lclogconf: usize, ppriority: *mut u32, ulflags: u32, hmachine: Option<isize>) -> CONFIGRET {
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Get_Log_Conf_Priority_Ex(lclogconf : usize, ppriority : *mut u32, ulflags : u32, hmachine : isize) -> CONFIGRET);
    unsafe { CM_Get_Log_Conf_Priority_Ex(lclogconf, ppriority as _, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CM_Get_Next_Log_Conf(plclogconf: Option<*mut usize>, lclogconf: usize, ulflags: Option<u32>) -> CONFIGRET {
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Get_Next_Log_Conf(plclogconf : *mut usize, lclogconf : usize, ulflags : u32) -> CONFIGRET);
    unsafe { CM_Get_Next_Log_Conf(plclogconf.unwrap_or(core::mem::zeroed()) as _, lclogconf, ulflags.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CM_Get_Next_Log_Conf_Ex(plclogconf: Option<*mut usize>, lclogconf: usize, ulflags: Option<u32>, hmachine: Option<isize>) -> CONFIGRET {
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Get_Next_Log_Conf_Ex(plclogconf : *mut usize, lclogconf : usize, ulflags : u32, hmachine : isize) -> CONFIGRET);
    unsafe { CM_Get_Next_Log_Conf_Ex(plclogconf.unwrap_or(core::mem::zeroed()) as _, lclogconf, ulflags.unwrap_or(core::mem::zeroed()) as _, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CM_Get_Next_Res_Des(prdresdes: *mut usize, rdresdes: usize, forresource: CM_RESTYPE, presourceid: Option<*mut CM_RESTYPE>, ulflags: u32) -> CONFIGRET {
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Get_Next_Res_Des(prdresdes : *mut usize, rdresdes : usize, forresource : CM_RESTYPE, presourceid : *mut CM_RESTYPE, ulflags : u32) -> CONFIGRET);
    unsafe { CM_Get_Next_Res_Des(prdresdes as _, rdresdes, forresource, presourceid.unwrap_or(core::mem::zeroed()) as _, ulflags) }
}
#[inline]
pub unsafe fn CM_Get_Next_Res_Des_Ex(prdresdes: *mut usize, rdresdes: usize, forresource: CM_RESTYPE, presourceid: Option<*mut CM_RESTYPE>, ulflags: u32, hmachine: Option<isize>) -> CONFIGRET {
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Get_Next_Res_Des_Ex(prdresdes : *mut usize, rdresdes : usize, forresource : CM_RESTYPE, presourceid : *mut CM_RESTYPE, ulflags : u32, hmachine : isize) -> CONFIGRET);
    unsafe { CM_Get_Next_Res_Des_Ex(prdresdes as _, rdresdes, forresource, presourceid.unwrap_or(core::mem::zeroed()) as _, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CM_Get_Parent(pdndevinst: *mut u32, dndevinst: u32, ulflags: u32) -> CONFIGRET {
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Get_Parent(pdndevinst : *mut u32, dndevinst : u32, ulflags : u32) -> CONFIGRET);
    unsafe { CM_Get_Parent(pdndevinst as _, dndevinst, ulflags) }
}
#[inline]
pub unsafe fn CM_Get_Parent_Ex(pdndevinst: *mut u32, dndevinst: u32, ulflags: u32, hmachine: Option<isize>) -> CONFIGRET {
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Get_Parent_Ex(pdndevinst : *mut u32, dndevinst : u32, ulflags : u32, hmachine : isize) -> CONFIGRET);
    unsafe { CM_Get_Parent_Ex(pdndevinst as _, dndevinst, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CM_Get_Res_Des_Data(rdresdes: usize, buffer: *mut core::ffi::c_void, bufferlen: u32, ulflags: u32) -> CONFIGRET {
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Get_Res_Des_Data(rdresdes : usize, buffer : *mut core::ffi::c_void, bufferlen : u32, ulflags : u32) -> CONFIGRET);
    unsafe { CM_Get_Res_Des_Data(rdresdes, buffer as _, bufferlen, ulflags) }
}
#[inline]
pub unsafe fn CM_Get_Res_Des_Data_Ex(rdresdes: usize, buffer: *mut core::ffi::c_void, bufferlen: u32, ulflags: u32, hmachine: Option<isize>) -> CONFIGRET {
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Get_Res_Des_Data_Ex(rdresdes : usize, buffer : *mut core::ffi::c_void, bufferlen : u32, ulflags : u32, hmachine : isize) -> CONFIGRET);
    unsafe { CM_Get_Res_Des_Data_Ex(rdresdes, buffer as _, bufferlen, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CM_Get_Res_Des_Data_Size(pulsize: *mut u32, rdresdes: usize, ulflags: u32) -> CONFIGRET {
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Get_Res_Des_Data_Size(pulsize : *mut u32, rdresdes : usize, ulflags : u32) -> CONFIGRET);
    unsafe { CM_Get_Res_Des_Data_Size(pulsize as _, rdresdes, ulflags) }
}
#[inline]
pub unsafe fn CM_Get_Res_Des_Data_Size_Ex(pulsize: *mut u32, rdresdes: usize, ulflags: u32, hmachine: Option<isize>) -> CONFIGRET {
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Get_Res_Des_Data_Size_Ex(pulsize : *mut u32, rdresdes : usize, ulflags : u32, hmachine : isize) -> CONFIGRET);
    unsafe { CM_Get_Res_Des_Data_Size_Ex(pulsize as _, rdresdes, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CM_Get_Resource_Conflict_Count(clconflictlist: usize, pulcount: *mut u32) -> CONFIGRET {
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Get_Resource_Conflict_Count(clconflictlist : usize, pulcount : *mut u32) -> CONFIGRET);
    unsafe { CM_Get_Resource_Conflict_Count(clconflictlist, pulcount as _) }
}
#[inline]
pub unsafe fn CM_Get_Resource_Conflict_DetailsA(clconflictlist: usize, ulindex: u32, pconflictdetails: *mut CONFLICT_DETAILS_A) -> CONFIGRET {
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Get_Resource_Conflict_DetailsA(clconflictlist : usize, ulindex : u32, pconflictdetails : *mut CONFLICT_DETAILS_A) -> CONFIGRET);
    unsafe { CM_Get_Resource_Conflict_DetailsA(clconflictlist, ulindex, pconflictdetails as _) }
}
#[inline]
pub unsafe fn CM_Get_Resource_Conflict_DetailsW(clconflictlist: usize, ulindex: u32, pconflictdetails: *mut CONFLICT_DETAILS_W) -> CONFIGRET {
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Get_Resource_Conflict_DetailsW(clconflictlist : usize, ulindex : u32, pconflictdetails : *mut CONFLICT_DETAILS_W) -> CONFIGRET);
    unsafe { CM_Get_Resource_Conflict_DetailsW(clconflictlist, ulindex, pconflictdetails as _) }
}
#[inline]
pub unsafe fn CM_Get_Sibling(pdndevinst: *mut u32, dndevinst: u32, ulflags: u32) -> CONFIGRET {
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Get_Sibling(pdndevinst : *mut u32, dndevinst : u32, ulflags : u32) -> CONFIGRET);
    unsafe { CM_Get_Sibling(pdndevinst as _, dndevinst, ulflags) }
}
#[inline]
pub unsafe fn CM_Get_Sibling_Ex(pdndevinst: *mut u32, dndevinst: u32, ulflags: u32, hmachine: Option<isize>) -> CONFIGRET {
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Get_Sibling_Ex(pdndevinst : *mut u32, dndevinst : u32, ulflags : u32, hmachine : isize) -> CONFIGRET);
    unsafe { CM_Get_Sibling_Ex(pdndevinst as _, dndevinst, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CM_Get_Version() -> u16 {
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Get_Version() -> u16);
    unsafe { CM_Get_Version() }
}
#[inline]
pub unsafe fn CM_Get_Version_Ex(hmachine: Option<isize>) -> u16 {
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Get_Version_Ex(hmachine : isize) -> u16);
    unsafe { CM_Get_Version_Ex(hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CM_Intersect_Range_List(rlhold1: usize, rlhold2: usize, rlhnew: usize, ulflags: u32) -> CONFIGRET {
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Intersect_Range_List(rlhold1 : usize, rlhold2 : usize, rlhnew : usize, ulflags : u32) -> CONFIGRET);
    unsafe { CM_Intersect_Range_List(rlhold1, rlhold2, rlhnew, ulflags) }
}
#[inline]
pub unsafe fn CM_Invert_Range_List(rlhold: usize, rlhnew: usize, ullmaxvalue: u64, ulflags: u32) -> CONFIGRET {
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Invert_Range_List(rlhold : usize, rlhnew : usize, ullmaxvalue : u64, ulflags : u32) -> CONFIGRET);
    unsafe { CM_Invert_Range_List(rlhold, rlhnew, ullmaxvalue, ulflags) }
}
#[inline]
pub unsafe fn CM_Is_Dock_Station_Present(pbpresent: *mut windows_core::BOOL) -> CONFIGRET {
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Is_Dock_Station_Present(pbpresent : *mut windows_core::BOOL) -> CONFIGRET);
    unsafe { CM_Is_Dock_Station_Present(pbpresent as _) }
}
#[inline]
pub unsafe fn CM_Is_Dock_Station_Present_Ex(pbpresent: *mut windows_core::BOOL, hmachine: Option<isize>) -> CONFIGRET {
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Is_Dock_Station_Present_Ex(pbpresent : *mut windows_core::BOOL, hmachine : isize) -> CONFIGRET);
    unsafe { CM_Is_Dock_Station_Present_Ex(pbpresent as _, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CM_Is_Version_Available(wversion: u16) -> windows_core::BOOL {
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Is_Version_Available(wversion : u16) -> windows_core::BOOL);
    unsafe { CM_Is_Version_Available(wversion) }
}
#[inline]
pub unsafe fn CM_Is_Version_Available_Ex(wversion: u16, hmachine: Option<isize>) -> windows_core::BOOL {
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Is_Version_Available_Ex(wversion : u16, hmachine : isize) -> windows_core::BOOL);
    unsafe { CM_Is_Version_Available_Ex(wversion, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CM_Locate_DevNodeA<P1>(pdndevinst: *mut u32, pdeviceid: P1, ulflags: CM_LOCATE_DEVNODE_FLAGS) -> CONFIGRET
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Locate_DevNodeA(pdndevinst : *mut u32, pdeviceid : windows_core::PCSTR, ulflags : CM_LOCATE_DEVNODE_FLAGS) -> CONFIGRET);
    unsafe { CM_Locate_DevNodeA(pdndevinst as _, pdeviceid.param().abi(), ulflags) }
}
#[inline]
pub unsafe fn CM_Locate_DevNodeW<P1>(pdndevinst: *mut u32, pdeviceid: P1, ulflags: CM_LOCATE_DEVNODE_FLAGS) -> CONFIGRET
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Locate_DevNodeW(pdndevinst : *mut u32, pdeviceid : windows_core::PCWSTR, ulflags : CM_LOCATE_DEVNODE_FLAGS) -> CONFIGRET);
    unsafe { CM_Locate_DevNodeW(pdndevinst as _, pdeviceid.param().abi(), ulflags) }
}
#[inline]
pub unsafe fn CM_Locate_DevNode_ExA<P1>(pdndevinst: *mut u32, pdeviceid: P1, ulflags: u32, hmachine: Option<isize>) -> CONFIGRET
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Locate_DevNode_ExA(pdndevinst : *mut u32, pdeviceid : windows_core::PCSTR, ulflags : u32, hmachine : isize) -> CONFIGRET);
    unsafe { CM_Locate_DevNode_ExA(pdndevinst as _, pdeviceid.param().abi(), ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CM_Locate_DevNode_ExW<P1>(pdndevinst: *mut u32, pdeviceid: P1, ulflags: u32, hmachine: Option<isize>) -> CONFIGRET
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Locate_DevNode_ExW(pdndevinst : *mut u32, pdeviceid : windows_core::PCWSTR, ulflags : u32, hmachine : isize) -> CONFIGRET);
    unsafe { CM_Locate_DevNode_ExW(pdndevinst as _, pdeviceid.param().abi(), ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CM_MapCrToWin32Err(cmreturncode: CONFIGRET, defaulterr: u32) -> u32 {
    windows_core::link!("cfgmgr32.dll" "system" fn CM_MapCrToWin32Err(cmreturncode : CONFIGRET, defaulterr : u32) -> u32);
    unsafe { CM_MapCrToWin32Err(cmreturncode, defaulterr) }
}
#[inline]
pub unsafe fn CM_Merge_Range_List(rlhold1: usize, rlhold2: usize, rlhnew: usize, ulflags: u32) -> CONFIGRET {
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Merge_Range_List(rlhold1 : usize, rlhold2 : usize, rlhnew : usize, ulflags : u32) -> CONFIGRET);
    unsafe { CM_Merge_Range_List(rlhold1, rlhold2, rlhnew, ulflags) }
}
#[inline]
pub unsafe fn CM_Modify_Res_Des(prdresdes: *mut usize, rdresdes: usize, resourceid: u32, resourcedata: *const core::ffi::c_void, resourcelen: u32, ulflags: u32) -> CONFIGRET {
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Modify_Res_Des(prdresdes : *mut usize, rdresdes : usize, resourceid : u32, resourcedata : *const core::ffi::c_void, resourcelen : u32, ulflags : u32) -> CONFIGRET);
    unsafe { CM_Modify_Res_Des(prdresdes as _, rdresdes, resourceid, resourcedata, resourcelen, ulflags) }
}
#[inline]
pub unsafe fn CM_Modify_Res_Des_Ex(prdresdes: *mut usize, rdresdes: usize, resourceid: u32, resourcedata: *const core::ffi::c_void, resourcelen: u32, ulflags: u32, hmachine: Option<isize>) -> CONFIGRET {
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Modify_Res_Des_Ex(prdresdes : *mut usize, rdresdes : usize, resourceid : u32, resourcedata : *const core::ffi::c_void, resourcelen : u32, ulflags : u32, hmachine : isize) -> CONFIGRET);
    unsafe { CM_Modify_Res_Des_Ex(prdresdes as _, rdresdes, resourceid, resourcedata, resourcelen, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CM_Move_DevNode(dnfromdevinst: u32, dntodevinst: u32, ulflags: u32) -> CONFIGRET {
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Move_DevNode(dnfromdevinst : u32, dntodevinst : u32, ulflags : u32) -> CONFIGRET);
    unsafe { CM_Move_DevNode(dnfromdevinst, dntodevinst, ulflags) }
}
#[inline]
pub unsafe fn CM_Move_DevNode_Ex(dnfromdevinst: u32, dntodevinst: u32, ulflags: u32, hmachine: Option<isize>) -> CONFIGRET {
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Move_DevNode_Ex(dnfromdevinst : u32, dntodevinst : u32, ulflags : u32, hmachine : isize) -> CONFIGRET);
    unsafe { CM_Move_DevNode_Ex(dnfromdevinst, dntodevinst, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CM_Next_Range(preelement: *mut usize, pullstart: *mut u64, pullend: *mut u64, ulflags: u32) -> CONFIGRET {
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Next_Range(preelement : *mut usize, pullstart : *mut u64, pullend : *mut u64, ulflags : u32) -> CONFIGRET);
    unsafe { CM_Next_Range(preelement as _, pullstart as _, pullend as _, ulflags) }
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn CM_Open_Class_KeyA<P1>(classguid: Option<*const windows_core::GUID>, pszclassname: P1, samdesired: u32, disposition: u32, phkclass: *mut super::super::System::Registry::HKEY, ulflags: u32) -> CONFIGRET
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Open_Class_KeyA(classguid : *const windows_core::GUID, pszclassname : windows_core::PCSTR, samdesired : u32, disposition : u32, phkclass : *mut super::super::System::Registry:: HKEY, ulflags : u32) -> CONFIGRET);
    unsafe { CM_Open_Class_KeyA(classguid.unwrap_or(core::mem::zeroed()) as _, pszclassname.param().abi(), samdesired, disposition, phkclass as _, ulflags) }
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn CM_Open_Class_KeyW<P1>(classguid: Option<*const windows_core::GUID>, pszclassname: P1, samdesired: u32, disposition: u32, phkclass: *mut super::super::System::Registry::HKEY, ulflags: u32) -> CONFIGRET
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Open_Class_KeyW(classguid : *const windows_core::GUID, pszclassname : windows_core::PCWSTR, samdesired : u32, disposition : u32, phkclass : *mut super::super::System::Registry:: HKEY, ulflags : u32) -> CONFIGRET);
    unsafe { CM_Open_Class_KeyW(classguid.unwrap_or(core::mem::zeroed()) as _, pszclassname.param().abi(), samdesired, disposition, phkclass as _, ulflags) }
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn CM_Open_Class_Key_ExA<P1>(classguid: Option<*const windows_core::GUID>, pszclassname: P1, samdesired: u32, disposition: u32, phkclass: *mut super::super::System::Registry::HKEY, ulflags: u32, hmachine: Option<isize>) -> CONFIGRET
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Open_Class_Key_ExA(classguid : *const windows_core::GUID, pszclassname : windows_core::PCSTR, samdesired : u32, disposition : u32, phkclass : *mut super::super::System::Registry:: HKEY, ulflags : u32, hmachine : isize) -> CONFIGRET);
    unsafe { CM_Open_Class_Key_ExA(classguid.unwrap_or(core::mem::zeroed()) as _, pszclassname.param().abi(), samdesired, disposition, phkclass as _, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn CM_Open_Class_Key_ExW<P1>(classguid: Option<*const windows_core::GUID>, pszclassname: P1, samdesired: u32, disposition: u32, phkclass: *mut super::super::System::Registry::HKEY, ulflags: u32, hmachine: Option<isize>) -> CONFIGRET
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Open_Class_Key_ExW(classguid : *const windows_core::GUID, pszclassname : windows_core::PCWSTR, samdesired : u32, disposition : u32, phkclass : *mut super::super::System::Registry:: HKEY, ulflags : u32, hmachine : isize) -> CONFIGRET);
    unsafe { CM_Open_Class_Key_ExW(classguid.unwrap_or(core::mem::zeroed()) as _, pszclassname.param().abi(), samdesired, disposition, phkclass as _, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn CM_Open_DevNode_Key(dndevnode: u32, samdesired: u32, ulhardwareprofile: u32, disposition: u32, phkdevice: *mut super::super::System::Registry::HKEY, ulflags: u32) -> CONFIGRET {
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Open_DevNode_Key(dndevnode : u32, samdesired : u32, ulhardwareprofile : u32, disposition : u32, phkdevice : *mut super::super::System::Registry:: HKEY, ulflags : u32) -> CONFIGRET);
    unsafe { CM_Open_DevNode_Key(dndevnode, samdesired, ulhardwareprofile, disposition, phkdevice as _, ulflags) }
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn CM_Open_DevNode_Key_Ex(dndevnode: u32, samdesired: u32, ulhardwareprofile: u32, disposition: u32, phkdevice: *mut super::super::System::Registry::HKEY, ulflags: u32, hmachine: Option<isize>) -> CONFIGRET {
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Open_DevNode_Key_Ex(dndevnode : u32, samdesired : u32, ulhardwareprofile : u32, disposition : u32, phkdevice : *mut super::super::System::Registry:: HKEY, ulflags : u32, hmachine : isize) -> CONFIGRET);
    unsafe { CM_Open_DevNode_Key_Ex(dndevnode, samdesired, ulhardwareprofile, disposition, phkdevice as _, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn CM_Open_Device_Interface_KeyA<P0>(pszdeviceinterface: P0, samdesired: u32, disposition: u32, phkdeviceinterface: *mut super::super::System::Registry::HKEY, ulflags: u32) -> CONFIGRET
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Open_Device_Interface_KeyA(pszdeviceinterface : windows_core::PCSTR, samdesired : u32, disposition : u32, phkdeviceinterface : *mut super::super::System::Registry:: HKEY, ulflags : u32) -> CONFIGRET);
    unsafe { CM_Open_Device_Interface_KeyA(pszdeviceinterface.param().abi(), samdesired, disposition, phkdeviceinterface as _, ulflags) }
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn CM_Open_Device_Interface_KeyW<P0>(pszdeviceinterface: P0, samdesired: u32, disposition: u32, phkdeviceinterface: *mut super::super::System::Registry::HKEY, ulflags: u32) -> CONFIGRET
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Open_Device_Interface_KeyW(pszdeviceinterface : windows_core::PCWSTR, samdesired : u32, disposition : u32, phkdeviceinterface : *mut super::super::System::Registry:: HKEY, ulflags : u32) -> CONFIGRET);
    unsafe { CM_Open_Device_Interface_KeyW(pszdeviceinterface.param().abi(), samdesired, disposition, phkdeviceinterface as _, ulflags) }
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn CM_Open_Device_Interface_Key_ExA<P0>(pszdeviceinterface: P0, samdesired: u32, disposition: u32, phkdeviceinterface: *mut super::super::System::Registry::HKEY, ulflags: u32, hmachine: Option<isize>) -> CONFIGRET
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Open_Device_Interface_Key_ExA(pszdeviceinterface : windows_core::PCSTR, samdesired : u32, disposition : u32, phkdeviceinterface : *mut super::super::System::Registry:: HKEY, ulflags : u32, hmachine : isize) -> CONFIGRET);
    unsafe { CM_Open_Device_Interface_Key_ExA(pszdeviceinterface.param().abi(), samdesired, disposition, phkdeviceinterface as _, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn CM_Open_Device_Interface_Key_ExW<P0>(pszdeviceinterface: P0, samdesired: u32, disposition: u32, phkdeviceinterface: *mut super::super::System::Registry::HKEY, ulflags: u32, hmachine: Option<isize>) -> CONFIGRET
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Open_Device_Interface_Key_ExW(pszdeviceinterface : windows_core::PCWSTR, samdesired : u32, disposition : u32, phkdeviceinterface : *mut super::super::System::Registry:: HKEY, ulflags : u32, hmachine : isize) -> CONFIGRET);
    unsafe { CM_Open_Device_Interface_Key_ExW(pszdeviceinterface.param().abi(), samdesired, disposition, phkdeviceinterface as _, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CM_Query_And_Remove_SubTreeA(dnancestor: u32, pvetotype: Option<*mut PNP_VETO_TYPE>, pszvetoname: Option<&mut [u8]>, ulflags: u32) -> CONFIGRET {
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Query_And_Remove_SubTreeA(dnancestor : u32, pvetotype : *mut PNP_VETO_TYPE, pszvetoname : windows_core::PSTR, ulnamelength : u32, ulflags : u32) -> CONFIGRET);
    unsafe { CM_Query_And_Remove_SubTreeA(dnancestor, pvetotype.unwrap_or(core::mem::zeroed()) as _, core::mem::transmute(pszvetoname.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pszvetoname.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), ulflags) }
}
#[inline]
pub unsafe fn CM_Query_And_Remove_SubTreeW(dnancestor: u32, pvetotype: Option<*mut PNP_VETO_TYPE>, pszvetoname: Option<&mut [u16]>, ulflags: u32) -> CONFIGRET {
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Query_And_Remove_SubTreeW(dnancestor : u32, pvetotype : *mut PNP_VETO_TYPE, pszvetoname : windows_core::PWSTR, ulnamelength : u32, ulflags : u32) -> CONFIGRET);
    unsafe { CM_Query_And_Remove_SubTreeW(dnancestor, pvetotype.unwrap_or(core::mem::zeroed()) as _, core::mem::transmute(pszvetoname.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pszvetoname.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), ulflags) }
}
#[inline]
pub unsafe fn CM_Query_And_Remove_SubTree_ExA(dnancestor: u32, pvetotype: Option<*mut PNP_VETO_TYPE>, pszvetoname: Option<&mut [u8]>, ulflags: u32, hmachine: Option<isize>) -> CONFIGRET {
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Query_And_Remove_SubTree_ExA(dnancestor : u32, pvetotype : *mut PNP_VETO_TYPE, pszvetoname : windows_core::PSTR, ulnamelength : u32, ulflags : u32, hmachine : isize) -> CONFIGRET);
    unsafe { CM_Query_And_Remove_SubTree_ExA(dnancestor, pvetotype.unwrap_or(core::mem::zeroed()) as _, core::mem::transmute(pszvetoname.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pszvetoname.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CM_Query_And_Remove_SubTree_ExW(dnancestor: u32, pvetotype: Option<*mut PNP_VETO_TYPE>, pszvetoname: Option<&mut [u16]>, ulflags: u32, hmachine: Option<isize>) -> CONFIGRET {
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Query_And_Remove_SubTree_ExW(dnancestor : u32, pvetotype : *mut PNP_VETO_TYPE, pszvetoname : windows_core::PWSTR, ulnamelength : u32, ulflags : u32, hmachine : isize) -> CONFIGRET);
    unsafe { CM_Query_And_Remove_SubTree_ExW(dnancestor, pvetotype.unwrap_or(core::mem::zeroed()) as _, core::mem::transmute(pszvetoname.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pszvetoname.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CM_Query_Arbitrator_Free_Data(pdata: *mut core::ffi::c_void, datalen: u32, dndevinst: u32, resourceid: u32, ulflags: u32) -> CONFIGRET {
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Query_Arbitrator_Free_Data(pdata : *mut core::ffi::c_void, datalen : u32, dndevinst : u32, resourceid : u32, ulflags : u32) -> CONFIGRET);
    unsafe { CM_Query_Arbitrator_Free_Data(pdata as _, datalen, dndevinst, resourceid, ulflags) }
}
#[inline]
pub unsafe fn CM_Query_Arbitrator_Free_Data_Ex(pdata: *mut core::ffi::c_void, datalen: u32, dndevinst: u32, resourceid: u32, ulflags: u32, hmachine: Option<isize>) -> CONFIGRET {
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Query_Arbitrator_Free_Data_Ex(pdata : *mut core::ffi::c_void, datalen : u32, dndevinst : u32, resourceid : u32, ulflags : u32, hmachine : isize) -> CONFIGRET);
    unsafe { CM_Query_Arbitrator_Free_Data_Ex(pdata as _, datalen, dndevinst, resourceid, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CM_Query_Arbitrator_Free_Size(pulsize: *mut u32, dndevinst: u32, resourceid: u32, ulflags: u32) -> CONFIGRET {
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Query_Arbitrator_Free_Size(pulsize : *mut u32, dndevinst : u32, resourceid : u32, ulflags : u32) -> CONFIGRET);
    unsafe { CM_Query_Arbitrator_Free_Size(pulsize as _, dndevinst, resourceid, ulflags) }
}
#[inline]
pub unsafe fn CM_Query_Arbitrator_Free_Size_Ex(pulsize: *mut u32, dndevinst: u32, resourceid: u32, ulflags: u32, hmachine: Option<isize>) -> CONFIGRET {
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Query_Arbitrator_Free_Size_Ex(pulsize : *mut u32, dndevinst : u32, resourceid : u32, ulflags : u32, hmachine : isize) -> CONFIGRET);
    unsafe { CM_Query_Arbitrator_Free_Size_Ex(pulsize as _, dndevinst, resourceid, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CM_Query_Remove_SubTree(dnancestor: u32, ulflags: u32) -> CONFIGRET {
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Query_Remove_SubTree(dnancestor : u32, ulflags : u32) -> CONFIGRET);
    unsafe { CM_Query_Remove_SubTree(dnancestor, ulflags) }
}
#[inline]
pub unsafe fn CM_Query_Remove_SubTree_Ex(dnancestor: u32, ulflags: u32, hmachine: Option<isize>) -> CONFIGRET {
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Query_Remove_SubTree_Ex(dnancestor : u32, ulflags : u32, hmachine : isize) -> CONFIGRET);
    unsafe { CM_Query_Remove_SubTree_Ex(dnancestor, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CM_Query_Resource_Conflict_List(pclconflictlist: *mut usize, dndevinst: u32, resourceid: CM_RESTYPE, resourcedata: *const core::ffi::c_void, resourcelen: u32, ulflags: u32, hmachine: Option<isize>) -> CONFIGRET {
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Query_Resource_Conflict_List(pclconflictlist : *mut usize, dndevinst : u32, resourceid : CM_RESTYPE, resourcedata : *const core::ffi::c_void, resourcelen : u32, ulflags : u32, hmachine : isize) -> CONFIGRET);
    unsafe { CM_Query_Resource_Conflict_List(pclconflictlist as _, dndevinst, resourceid, resourcedata, resourcelen, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CM_Reenumerate_DevNode(dndevinst: u32, ulflags: CM_REENUMERATE_FLAGS) -> CONFIGRET {
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Reenumerate_DevNode(dndevinst : u32, ulflags : CM_REENUMERATE_FLAGS) -> CONFIGRET);
    unsafe { CM_Reenumerate_DevNode(dndevinst, ulflags) }
}
#[inline]
pub unsafe fn CM_Reenumerate_DevNode_Ex(dndevinst: u32, ulflags: u32, hmachine: Option<isize>) -> CONFIGRET {
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Reenumerate_DevNode_Ex(dndevinst : u32, ulflags : u32, hmachine : isize) -> CONFIGRET);
    unsafe { CM_Reenumerate_DevNode_Ex(dndevinst, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CM_Register_Device_Driver(dndevinst: u32, ulflags: u32) -> CONFIGRET {
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Register_Device_Driver(dndevinst : u32, ulflags : u32) -> CONFIGRET);
    unsafe { CM_Register_Device_Driver(dndevinst, ulflags) }
}
#[inline]
pub unsafe fn CM_Register_Device_Driver_Ex(dndevinst: u32, ulflags: u32, hmachine: Option<isize>) -> CONFIGRET {
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Register_Device_Driver_Ex(dndevinst : u32, ulflags : u32, hmachine : isize) -> CONFIGRET);
    unsafe { CM_Register_Device_Driver_Ex(dndevinst, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CM_Register_Device_InterfaceA<P2>(dndevinst: u32, interfaceclassguid: *const windows_core::GUID, pszreference: P2, pszdeviceinterface: windows_core::PSTR, pullength: *mut u32, ulflags: u32) -> CONFIGRET
where
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Register_Device_InterfaceA(dndevinst : u32, interfaceclassguid : *const windows_core::GUID, pszreference : windows_core::PCSTR, pszdeviceinterface : windows_core::PSTR, pullength : *mut u32, ulflags : u32) -> CONFIGRET);
    unsafe { CM_Register_Device_InterfaceA(dndevinst, interfaceclassguid, pszreference.param().abi(), core::mem::transmute(pszdeviceinterface), pullength as _, ulflags) }
}
#[inline]
pub unsafe fn CM_Register_Device_InterfaceW<P2>(dndevinst: u32, interfaceclassguid: *const windows_core::GUID, pszreference: P2, pszdeviceinterface: windows_core::PWSTR, pullength: *mut u32, ulflags: u32) -> CONFIGRET
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Register_Device_InterfaceW(dndevinst : u32, interfaceclassguid : *const windows_core::GUID, pszreference : windows_core::PCWSTR, pszdeviceinterface : windows_core::PWSTR, pullength : *mut u32, ulflags : u32) -> CONFIGRET);
    unsafe { CM_Register_Device_InterfaceW(dndevinst, interfaceclassguid, pszreference.param().abi(), core::mem::transmute(pszdeviceinterface), pullength as _, ulflags) }
}
#[inline]
pub unsafe fn CM_Register_Device_Interface_ExA<P2>(dndevinst: u32, interfaceclassguid: *const windows_core::GUID, pszreference: P2, pszdeviceinterface: windows_core::PSTR, pullength: *mut u32, ulflags: u32, hmachine: Option<isize>) -> CONFIGRET
where
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Register_Device_Interface_ExA(dndevinst : u32, interfaceclassguid : *const windows_core::GUID, pszreference : windows_core::PCSTR, pszdeviceinterface : windows_core::PSTR, pullength : *mut u32, ulflags : u32, hmachine : isize) -> CONFIGRET);
    unsafe { CM_Register_Device_Interface_ExA(dndevinst, interfaceclassguid, pszreference.param().abi(), core::mem::transmute(pszdeviceinterface), pullength as _, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CM_Register_Device_Interface_ExW<P2>(dndevinst: u32, interfaceclassguid: *const windows_core::GUID, pszreference: P2, pszdeviceinterface: windows_core::PWSTR, pullength: *mut u32, ulflags: u32, hmachine: Option<isize>) -> CONFIGRET
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Register_Device_Interface_ExW(dndevinst : u32, interfaceclassguid : *const windows_core::GUID, pszreference : windows_core::PCWSTR, pszdeviceinterface : windows_core::PWSTR, pullength : *mut u32, ulflags : u32, hmachine : isize) -> CONFIGRET);
    unsafe { CM_Register_Device_Interface_ExW(dndevinst, interfaceclassguid, pszreference.param().abi(), core::mem::transmute(pszdeviceinterface), pullength as _, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CM_Register_Notification(pfilter: *const CM_NOTIFY_FILTER, pcontext: Option<*const core::ffi::c_void>, pcallback: PCM_NOTIFY_CALLBACK, pnotifycontext: *mut HCMNOTIFICATION) -> CONFIGRET {
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Register_Notification(pfilter : *const CM_NOTIFY_FILTER, pcontext : *const core::ffi::c_void, pcallback : PCM_NOTIFY_CALLBACK, pnotifycontext : *mut HCMNOTIFICATION) -> CONFIGRET);
    unsafe { CM_Register_Notification(pfilter, pcontext.unwrap_or(core::mem::zeroed()) as _, pcallback, pnotifycontext as _) }
}
#[inline]
pub unsafe fn CM_Remove_SubTree(dnancestor: u32, ulflags: u32) -> CONFIGRET {
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Remove_SubTree(dnancestor : u32, ulflags : u32) -> CONFIGRET);
    unsafe { CM_Remove_SubTree(dnancestor, ulflags) }
}
#[inline]
pub unsafe fn CM_Remove_SubTree_Ex(dnancestor: u32, ulflags: u32, hmachine: Option<isize>) -> CONFIGRET {
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Remove_SubTree_Ex(dnancestor : u32, ulflags : u32, hmachine : isize) -> CONFIGRET);
    unsafe { CM_Remove_SubTree_Ex(dnancestor, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CM_Request_Device_EjectA(dndevinst: u32, pvetotype: Option<*mut PNP_VETO_TYPE>, pszvetoname: Option<&mut [u8]>, ulflags: u32) -> CONFIGRET {
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Request_Device_EjectA(dndevinst : u32, pvetotype : *mut PNP_VETO_TYPE, pszvetoname : windows_core::PSTR, ulnamelength : u32, ulflags : u32) -> CONFIGRET);
    unsafe { CM_Request_Device_EjectA(dndevinst, pvetotype.unwrap_or(core::mem::zeroed()) as _, core::mem::transmute(pszvetoname.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pszvetoname.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), ulflags) }
}
#[inline]
pub unsafe fn CM_Request_Device_EjectW(dndevinst: u32, pvetotype: Option<*mut PNP_VETO_TYPE>, pszvetoname: Option<&mut [u16]>, ulflags: u32) -> CONFIGRET {
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Request_Device_EjectW(dndevinst : u32, pvetotype : *mut PNP_VETO_TYPE, pszvetoname : windows_core::PWSTR, ulnamelength : u32, ulflags : u32) -> CONFIGRET);
    unsafe { CM_Request_Device_EjectW(dndevinst, pvetotype.unwrap_or(core::mem::zeroed()) as _, core::mem::transmute(pszvetoname.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pszvetoname.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), ulflags) }
}
#[inline]
pub unsafe fn CM_Request_Device_Eject_ExA(dndevinst: u32, pvetotype: Option<*mut PNP_VETO_TYPE>, pszvetoname: Option<&mut [u8]>, ulflags: u32, hmachine: Option<isize>) -> CONFIGRET {
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Request_Device_Eject_ExA(dndevinst : u32, pvetotype : *mut PNP_VETO_TYPE, pszvetoname : windows_core::PSTR, ulnamelength : u32, ulflags : u32, hmachine : isize) -> CONFIGRET);
    unsafe { CM_Request_Device_Eject_ExA(dndevinst, pvetotype.unwrap_or(core::mem::zeroed()) as _, core::mem::transmute(pszvetoname.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pszvetoname.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CM_Request_Device_Eject_ExW(dndevinst: u32, pvetotype: Option<*mut PNP_VETO_TYPE>, pszvetoname: Option<&mut [u16]>, ulflags: u32, hmachine: Option<isize>) -> CONFIGRET {
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Request_Device_Eject_ExW(dndevinst : u32, pvetotype : *mut PNP_VETO_TYPE, pszvetoname : windows_core::PWSTR, ulnamelength : u32, ulflags : u32, hmachine : isize) -> CONFIGRET);
    unsafe { CM_Request_Device_Eject_ExW(dndevinst, pvetotype.unwrap_or(core::mem::zeroed()) as _, core::mem::transmute(pszvetoname.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pszvetoname.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CM_Request_Eject_PC() -> CONFIGRET {
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Request_Eject_PC() -> CONFIGRET);
    unsafe { CM_Request_Eject_PC() }
}
#[inline]
pub unsafe fn CM_Request_Eject_PC_Ex(hmachine: Option<isize>) -> CONFIGRET {
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Request_Eject_PC_Ex(hmachine : isize) -> CONFIGRET);
    unsafe { CM_Request_Eject_PC_Ex(hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CM_Run_Detection(ulflags: u32) -> CONFIGRET {
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Run_Detection(ulflags : u32) -> CONFIGRET);
    unsafe { CM_Run_Detection(ulflags) }
}
#[inline]
pub unsafe fn CM_Run_Detection_Ex(ulflags: u32, hmachine: Option<isize>) -> CONFIGRET {
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Run_Detection_Ex(ulflags : u32, hmachine : isize) -> CONFIGRET);
    unsafe { CM_Run_Detection_Ex(ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "Win32_Devices_Properties")]
#[inline]
pub unsafe fn CM_Set_Class_PropertyW(classguid: *const windows_core::GUID, propertykey: *const super::super::Foundation::DEVPROPKEY, propertytype: super::Properties::DEVPROPTYPE, propertybuffer: Option<&[u8]>, ulflags: u32) -> CONFIGRET {
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Set_Class_PropertyW(classguid : *const windows_core::GUID, propertykey : *const super::super::Foundation:: DEVPROPKEY, propertytype : super::Properties:: DEVPROPTYPE, propertybuffer : *const u8, propertybuffersize : u32, ulflags : u32) -> CONFIGRET);
    unsafe { CM_Set_Class_PropertyW(classguid, propertykey, propertytype, core::mem::transmute(propertybuffer.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), propertybuffer.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), ulflags) }
}
#[cfg(feature = "Win32_Devices_Properties")]
#[inline]
pub unsafe fn CM_Set_Class_Property_ExW(classguid: *const windows_core::GUID, propertykey: *const super::super::Foundation::DEVPROPKEY, propertytype: super::Properties::DEVPROPTYPE, propertybuffer: Option<&[u8]>, ulflags: u32, hmachine: Option<isize>) -> CONFIGRET {
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Set_Class_Property_ExW(classguid : *const windows_core::GUID, propertykey : *const super::super::Foundation:: DEVPROPKEY, propertytype : super::Properties:: DEVPROPTYPE, propertybuffer : *const u8, propertybuffersize : u32, ulflags : u32, hmachine : isize) -> CONFIGRET);
    unsafe { CM_Set_Class_Property_ExW(classguid, propertykey, propertytype, core::mem::transmute(propertybuffer.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), propertybuffer.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CM_Set_Class_Registry_PropertyA(classguid: *const windows_core::GUID, ulproperty: u32, buffer: Option<*const core::ffi::c_void>, ullength: u32, ulflags: u32, hmachine: Option<isize>) -> CONFIGRET {
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Set_Class_Registry_PropertyA(classguid : *const windows_core::GUID, ulproperty : u32, buffer : *const core::ffi::c_void, ullength : u32, ulflags : u32, hmachine : isize) -> CONFIGRET);
    unsafe { CM_Set_Class_Registry_PropertyA(classguid, ulproperty, buffer.unwrap_or(core::mem::zeroed()) as _, ullength, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CM_Set_Class_Registry_PropertyW(classguid: *const windows_core::GUID, ulproperty: u32, buffer: Option<*const core::ffi::c_void>, ullength: u32, ulflags: u32, hmachine: Option<isize>) -> CONFIGRET {
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Set_Class_Registry_PropertyW(classguid : *const windows_core::GUID, ulproperty : u32, buffer : *const core::ffi::c_void, ullength : u32, ulflags : u32, hmachine : isize) -> CONFIGRET);
    unsafe { CM_Set_Class_Registry_PropertyW(classguid, ulproperty, buffer.unwrap_or(core::mem::zeroed()) as _, ullength, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CM_Set_DevNode_Problem(dndevinst: u32, ulproblem: u32, ulflags: u32) -> CONFIGRET {
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Set_DevNode_Problem(dndevinst : u32, ulproblem : u32, ulflags : u32) -> CONFIGRET);
    unsafe { CM_Set_DevNode_Problem(dndevinst, ulproblem, ulflags) }
}
#[inline]
pub unsafe fn CM_Set_DevNode_Problem_Ex(dndevinst: u32, ulproblem: u32, ulflags: u32, hmachine: Option<isize>) -> CONFIGRET {
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Set_DevNode_Problem_Ex(dndevinst : u32, ulproblem : u32, ulflags : u32, hmachine : isize) -> CONFIGRET);
    unsafe { CM_Set_DevNode_Problem_Ex(dndevinst, ulproblem, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "Win32_Devices_Properties")]
#[inline]
pub unsafe fn CM_Set_DevNode_PropertyW(dndevinst: u32, propertykey: *const super::super::Foundation::DEVPROPKEY, propertytype: super::Properties::DEVPROPTYPE, propertybuffer: Option<&[u8]>, ulflags: u32) -> CONFIGRET {
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Set_DevNode_PropertyW(dndevinst : u32, propertykey : *const super::super::Foundation:: DEVPROPKEY, propertytype : super::Properties:: DEVPROPTYPE, propertybuffer : *const u8, propertybuffersize : u32, ulflags : u32) -> CONFIGRET);
    unsafe { CM_Set_DevNode_PropertyW(dndevinst, propertykey, propertytype, core::mem::transmute(propertybuffer.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), propertybuffer.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), ulflags) }
}
#[cfg(feature = "Win32_Devices_Properties")]
#[inline]
pub unsafe fn CM_Set_DevNode_Property_ExW(dndevinst: u32, propertykey: *const super::super::Foundation::DEVPROPKEY, propertytype: super::Properties::DEVPROPTYPE, propertybuffer: Option<&[u8]>, ulflags: u32, hmachine: Option<isize>) -> CONFIGRET {
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Set_DevNode_Property_ExW(dndevinst : u32, propertykey : *const super::super::Foundation:: DEVPROPKEY, propertytype : super::Properties:: DEVPROPTYPE, propertybuffer : *const u8, propertybuffersize : u32, ulflags : u32, hmachine : isize) -> CONFIGRET);
    unsafe { CM_Set_DevNode_Property_ExW(dndevinst, propertykey, propertytype, core::mem::transmute(propertybuffer.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), propertybuffer.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CM_Set_DevNode_Registry_PropertyA(dndevinst: u32, ulproperty: u32, buffer: Option<*const core::ffi::c_void>, ullength: u32, ulflags: u32) -> CONFIGRET {
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Set_DevNode_Registry_PropertyA(dndevinst : u32, ulproperty : u32, buffer : *const core::ffi::c_void, ullength : u32, ulflags : u32) -> CONFIGRET);
    unsafe { CM_Set_DevNode_Registry_PropertyA(dndevinst, ulproperty, buffer.unwrap_or(core::mem::zeroed()) as _, ullength, ulflags) }
}
#[inline]
pub unsafe fn CM_Set_DevNode_Registry_PropertyW(dndevinst: u32, ulproperty: u32, buffer: Option<*const core::ffi::c_void>, ullength: u32, ulflags: u32) -> CONFIGRET {
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Set_DevNode_Registry_PropertyW(dndevinst : u32, ulproperty : u32, buffer : *const core::ffi::c_void, ullength : u32, ulflags : u32) -> CONFIGRET);
    unsafe { CM_Set_DevNode_Registry_PropertyW(dndevinst, ulproperty, buffer.unwrap_or(core::mem::zeroed()) as _, ullength, ulflags) }
}
#[inline]
pub unsafe fn CM_Set_DevNode_Registry_Property_ExA(dndevinst: u32, ulproperty: u32, buffer: Option<*const core::ffi::c_void>, ullength: u32, ulflags: u32, hmachine: Option<isize>) -> CONFIGRET {
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Set_DevNode_Registry_Property_ExA(dndevinst : u32, ulproperty : u32, buffer : *const core::ffi::c_void, ullength : u32, ulflags : u32, hmachine : isize) -> CONFIGRET);
    unsafe { CM_Set_DevNode_Registry_Property_ExA(dndevinst, ulproperty, buffer.unwrap_or(core::mem::zeroed()) as _, ullength, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CM_Set_DevNode_Registry_Property_ExW(dndevinst: u32, ulproperty: u32, buffer: Option<*const core::ffi::c_void>, ullength: u32, ulflags: u32, hmachine: Option<isize>) -> CONFIGRET {
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Set_DevNode_Registry_Property_ExW(dndevinst : u32, ulproperty : u32, buffer : *const core::ffi::c_void, ullength : u32, ulflags : u32, hmachine : isize) -> CONFIGRET);
    unsafe { CM_Set_DevNode_Registry_Property_ExW(dndevinst, ulproperty, buffer.unwrap_or(core::mem::zeroed()) as _, ullength, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "Win32_Devices_Properties")]
#[inline]
pub unsafe fn CM_Set_Device_Interface_PropertyW<P0>(pszdeviceinterface: P0, propertykey: *const super::super::Foundation::DEVPROPKEY, propertytype: super::Properties::DEVPROPTYPE, propertybuffer: Option<&[u8]>, ulflags: u32) -> CONFIGRET
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Set_Device_Interface_PropertyW(pszdeviceinterface : windows_core::PCWSTR, propertykey : *const super::super::Foundation:: DEVPROPKEY, propertytype : super::Properties:: DEVPROPTYPE, propertybuffer : *const u8, propertybuffersize : u32, ulflags : u32) -> CONFIGRET);
    unsafe { CM_Set_Device_Interface_PropertyW(pszdeviceinterface.param().abi(), propertykey, propertytype, core::mem::transmute(propertybuffer.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), propertybuffer.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), ulflags) }
}
#[cfg(feature = "Win32_Devices_Properties")]
#[inline]
pub unsafe fn CM_Set_Device_Interface_Property_ExW<P0>(pszdeviceinterface: P0, propertykey: *const super::super::Foundation::DEVPROPKEY, propertytype: super::Properties::DEVPROPTYPE, propertybuffer: Option<&[u8]>, ulflags: u32, hmachine: Option<isize>) -> CONFIGRET
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Set_Device_Interface_Property_ExW(pszdeviceinterface : windows_core::PCWSTR, propertykey : *const super::super::Foundation:: DEVPROPKEY, propertytype : super::Properties:: DEVPROPTYPE, propertybuffer : *const u8, propertybuffersize : u32, ulflags : u32, hmachine : isize) -> CONFIGRET);
    unsafe { CM_Set_Device_Interface_Property_ExW(pszdeviceinterface.param().abi(), propertykey, propertytype, core::mem::transmute(propertybuffer.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), propertybuffer.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CM_Set_HW_Prof(ulhardwareprofile: u32, ulflags: u32) -> CONFIGRET {
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Set_HW_Prof(ulhardwareprofile : u32, ulflags : u32) -> CONFIGRET);
    unsafe { CM_Set_HW_Prof(ulhardwareprofile, ulflags) }
}
#[inline]
pub unsafe fn CM_Set_HW_Prof_Ex(ulhardwareprofile: u32, ulflags: u32, hmachine: Option<isize>) -> CONFIGRET {
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Set_HW_Prof_Ex(ulhardwareprofile : u32, ulflags : u32, hmachine : isize) -> CONFIGRET);
    unsafe { CM_Set_HW_Prof_Ex(ulhardwareprofile, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CM_Set_HW_Prof_FlagsA<P0>(pdeviceid: P0, ulconfig: u32, ulvalue: u32, ulflags: u32) -> CONFIGRET
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Set_HW_Prof_FlagsA(pdeviceid : windows_core::PCSTR, ulconfig : u32, ulvalue : u32, ulflags : u32) -> CONFIGRET);
    unsafe { CM_Set_HW_Prof_FlagsA(pdeviceid.param().abi(), ulconfig, ulvalue, ulflags) }
}
#[inline]
pub unsafe fn CM_Set_HW_Prof_FlagsW<P0>(pdeviceid: P0, ulconfig: u32, ulvalue: u32, ulflags: u32) -> CONFIGRET
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Set_HW_Prof_FlagsW(pdeviceid : windows_core::PCWSTR, ulconfig : u32, ulvalue : u32, ulflags : u32) -> CONFIGRET);
    unsafe { CM_Set_HW_Prof_FlagsW(pdeviceid.param().abi(), ulconfig, ulvalue, ulflags) }
}
#[inline]
pub unsafe fn CM_Set_HW_Prof_Flags_ExA<P0>(pdeviceid: P0, ulconfig: u32, ulvalue: u32, ulflags: u32, hmachine: Option<isize>) -> CONFIGRET
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Set_HW_Prof_Flags_ExA(pdeviceid : windows_core::PCSTR, ulconfig : u32, ulvalue : u32, ulflags : u32, hmachine : isize) -> CONFIGRET);
    unsafe { CM_Set_HW_Prof_Flags_ExA(pdeviceid.param().abi(), ulconfig, ulvalue, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CM_Set_HW_Prof_Flags_ExW<P0>(pdeviceid: P0, ulconfig: u32, ulvalue: u32, ulflags: u32, hmachine: Option<isize>) -> CONFIGRET
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Set_HW_Prof_Flags_ExW(pdeviceid : windows_core::PCWSTR, ulconfig : u32, ulvalue : u32, ulflags : u32, hmachine : isize) -> CONFIGRET);
    unsafe { CM_Set_HW_Prof_Flags_ExW(pdeviceid.param().abi(), ulconfig, ulvalue, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CM_Setup_DevNode(dndevinst: u32, ulflags: u32) -> CONFIGRET {
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Setup_DevNode(dndevinst : u32, ulflags : u32) -> CONFIGRET);
    unsafe { CM_Setup_DevNode(dndevinst, ulflags) }
}
#[inline]
pub unsafe fn CM_Setup_DevNode_Ex(dndevinst: u32, ulflags: u32, hmachine: Option<isize>) -> CONFIGRET {
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Setup_DevNode_Ex(dndevinst : u32, ulflags : u32, hmachine : isize) -> CONFIGRET);
    unsafe { CM_Setup_DevNode_Ex(dndevinst, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CM_Test_Range_Available(ullstartvalue: u64, ullendvalue: u64, rlh: usize, ulflags: u32) -> CONFIGRET {
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Test_Range_Available(ullstartvalue : u64, ullendvalue : u64, rlh : usize, ulflags : u32) -> CONFIGRET);
    unsafe { CM_Test_Range_Available(ullstartvalue, ullendvalue, rlh, ulflags) }
}
#[inline]
pub unsafe fn CM_Uninstall_DevNode(dndevinst: u32, ulflags: u32) -> CONFIGRET {
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Uninstall_DevNode(dndevinst : u32, ulflags : u32) -> CONFIGRET);
    unsafe { CM_Uninstall_DevNode(dndevinst, ulflags) }
}
#[inline]
pub unsafe fn CM_Uninstall_DevNode_Ex(dndevinst: u32, ulflags: u32, hmachine: Option<isize>) -> CONFIGRET {
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Uninstall_DevNode_Ex(dndevinst : u32, ulflags : u32, hmachine : isize) -> CONFIGRET);
    unsafe { CM_Uninstall_DevNode_Ex(dndevinst, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CM_Unregister_Device_InterfaceA<P0>(pszdeviceinterface: P0, ulflags: u32) -> CONFIGRET
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Unregister_Device_InterfaceA(pszdeviceinterface : windows_core::PCSTR, ulflags : u32) -> CONFIGRET);
    unsafe { CM_Unregister_Device_InterfaceA(pszdeviceinterface.param().abi(), ulflags) }
}
#[inline]
pub unsafe fn CM_Unregister_Device_InterfaceW<P0>(pszdeviceinterface: P0, ulflags: u32) -> CONFIGRET
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Unregister_Device_InterfaceW(pszdeviceinterface : windows_core::PCWSTR, ulflags : u32) -> CONFIGRET);
    unsafe { CM_Unregister_Device_InterfaceW(pszdeviceinterface.param().abi(), ulflags) }
}
#[inline]
pub unsafe fn CM_Unregister_Device_Interface_ExA<P0>(pszdeviceinterface: P0, ulflags: u32, hmachine: Option<isize>) -> CONFIGRET
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Unregister_Device_Interface_ExA(pszdeviceinterface : windows_core::PCSTR, ulflags : u32, hmachine : isize) -> CONFIGRET);
    unsafe { CM_Unregister_Device_Interface_ExA(pszdeviceinterface.param().abi(), ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CM_Unregister_Device_Interface_ExW<P0>(pszdeviceinterface: P0, ulflags: u32, hmachine: Option<isize>) -> CONFIGRET
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Unregister_Device_Interface_ExW(pszdeviceinterface : windows_core::PCWSTR, ulflags : u32, hmachine : isize) -> CONFIGRET);
    unsafe { CM_Unregister_Device_Interface_ExW(pszdeviceinterface.param().abi(), ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CM_Unregister_Notification(notifycontext: HCMNOTIFICATION) -> CONFIGRET {
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Unregister_Notification(notifycontext : HCMNOTIFICATION) -> CONFIGRET);
    unsafe { CM_Unregister_Notification(notifycontext) }
}
#[inline]
pub unsafe fn DiInstallDevice(hwndparent: Option<super::super::Foundation::HWND>, deviceinfoset: HDEVINFO, deviceinfodata: *const SP_DEVINFO_DATA, driverinfodata: Option<*const SP_DRVINFO_DATA_V2_W>, flags: DIINSTALLDEVICE_FLAGS, needreboot: Option<*mut windows_core::BOOL>) -> windows_core::Result<()> {
    windows_core::link!("newdev.dll" "system" fn DiInstallDevice(hwndparent : super::super::Foundation:: HWND, deviceinfoset : HDEVINFO, deviceinfodata : *const SP_DEVINFO_DATA, driverinfodata : *const SP_DRVINFO_DATA_V2_W, flags : DIINSTALLDEVICE_FLAGS, needreboot : *mut windows_core::BOOL) -> windows_core::BOOL);
    unsafe { DiInstallDevice(hwndparent.unwrap_or(core::mem::zeroed()) as _, deviceinfoset, deviceinfodata, driverinfodata.unwrap_or(core::mem::zeroed()) as _, flags, needreboot.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn DiInstallDriverA<P1>(hwndparent: Option<super::super::Foundation::HWND>, infpath: P1, flags: DIINSTALLDRIVER_FLAGS, needreboot: Option<*mut windows_core::BOOL>) -> windows_core::Result<()>
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("newdev.dll" "system" fn DiInstallDriverA(hwndparent : super::super::Foundation:: HWND, infpath : windows_core::PCSTR, flags : DIINSTALLDRIVER_FLAGS, needreboot : *mut windows_core::BOOL) -> windows_core::BOOL);
    unsafe { DiInstallDriverA(hwndparent.unwrap_or(core::mem::zeroed()) as _, infpath.param().abi(), flags, needreboot.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn DiInstallDriverW<P1>(hwndparent: Option<super::super::Foundation::HWND>, infpath: P1, flags: DIINSTALLDRIVER_FLAGS, needreboot: Option<*mut windows_core::BOOL>) -> windows_core::Result<()>
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("newdev.dll" "system" fn DiInstallDriverW(hwndparent : super::super::Foundation:: HWND, infpath : windows_core::PCWSTR, flags : DIINSTALLDRIVER_FLAGS, needreboot : *mut windows_core::BOOL) -> windows_core::BOOL);
    unsafe { DiInstallDriverW(hwndparent.unwrap_or(core::mem::zeroed()) as _, infpath.param().abi(), flags, needreboot.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn DiRollbackDriver(deviceinfoset: HDEVINFO, deviceinfodata: *const SP_DEVINFO_DATA, hwndparent: Option<super::super::Foundation::HWND>, flags: DIROLLBACKDRIVER_FLAGS, needreboot: Option<*mut windows_core::BOOL>) -> windows_core::Result<()> {
    windows_core::link!("newdev.dll" "system" fn DiRollbackDriver(deviceinfoset : HDEVINFO, deviceinfodata : *const SP_DEVINFO_DATA, hwndparent : super::super::Foundation:: HWND, flags : DIROLLBACKDRIVER_FLAGS, needreboot : *mut windows_core::BOOL) -> windows_core::BOOL);
    unsafe { DiRollbackDriver(deviceinfoset, deviceinfodata, hwndparent.unwrap_or(core::mem::zeroed()) as _, flags, needreboot.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn DiShowUpdateDevice(hwndparent: Option<super::super::Foundation::HWND>, deviceinfoset: HDEVINFO, deviceinfodata: *const SP_DEVINFO_DATA, flags: u32, needreboot: Option<*mut windows_core::BOOL>) -> windows_core::Result<()> {
    windows_core::link!("newdev.dll" "system" fn DiShowUpdateDevice(hwndparent : super::super::Foundation:: HWND, deviceinfoset : HDEVINFO, deviceinfodata : *const SP_DEVINFO_DATA, flags : u32, needreboot : *mut windows_core::BOOL) -> windows_core::BOOL);
    unsafe { DiShowUpdateDevice(hwndparent.unwrap_or(core::mem::zeroed()) as _, deviceinfoset, deviceinfodata, flags, needreboot.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn DiShowUpdateDriver<P1>(hwndparent: Option<super::super::Foundation::HWND>, filepath: P1, flags: u32, needreboot: Option<*mut windows_core::BOOL>) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("newdev.dll" "system" fn DiShowUpdateDriver(hwndparent : super::super::Foundation:: HWND, filepath : windows_core::PCWSTR, flags : u32, needreboot : *mut windows_core::BOOL) -> windows_core::BOOL);
    unsafe { DiShowUpdateDriver(hwndparent.unwrap_or(core::mem::zeroed()) as _, filepath.param().abi(), flags, needreboot.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn DiUninstallDevice(hwndparent: super::super::Foundation::HWND, deviceinfoset: HDEVINFO, deviceinfodata: *const SP_DEVINFO_DATA, flags: u32, needreboot: Option<*mut windows_core::BOOL>) -> windows_core::Result<()> {
    windows_core::link!("newdev.dll" "system" fn DiUninstallDevice(hwndparent : super::super::Foundation:: HWND, deviceinfoset : HDEVINFO, deviceinfodata : *const SP_DEVINFO_DATA, flags : u32, needreboot : *mut windows_core::BOOL) -> windows_core::BOOL);
    unsafe { DiUninstallDevice(hwndparent, deviceinfoset, deviceinfodata, flags, needreboot.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn DiUninstallDriverA<P1>(hwndparent: Option<super::super::Foundation::HWND>, infpath: P1, flags: DIUNINSTALLDRIVER_FLAGS, needreboot: Option<*mut windows_core::BOOL>) -> windows_core::Result<()>
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("newdev.dll" "system" fn DiUninstallDriverA(hwndparent : super::super::Foundation:: HWND, infpath : windows_core::PCSTR, flags : DIUNINSTALLDRIVER_FLAGS, needreboot : *mut windows_core::BOOL) -> windows_core::BOOL);
    unsafe { DiUninstallDriverA(hwndparent.unwrap_or(core::mem::zeroed()) as _, infpath.param().abi(), flags, needreboot.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn DiUninstallDriverW<P1>(hwndparent: Option<super::super::Foundation::HWND>, infpath: P1, flags: DIUNINSTALLDRIVER_FLAGS, needreboot: Option<*mut windows_core::BOOL>) -> windows_core::Result<()>
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("newdev.dll" "system" fn DiUninstallDriverW(hwndparent : super::super::Foundation:: HWND, infpath : windows_core::PCWSTR, flags : DIUNINSTALLDRIVER_FLAGS, needreboot : *mut windows_core::BOOL) -> windows_core::BOOL);
    unsafe { DiUninstallDriverW(hwndparent.unwrap_or(core::mem::zeroed()) as _, infpath.param().abi(), flags, needreboot.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn InstallHinfSectionA<P2>(window: super::super::Foundation::HWND, modulehandle: super::super::Foundation::HINSTANCE, commandline: P2, showcommand: i32)
where
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn InstallHinfSectionA(window : super::super::Foundation:: HWND, modulehandle : super::super::Foundation:: HINSTANCE, commandline : windows_core::PCSTR, showcommand : i32));
    unsafe { InstallHinfSectionA(window, modulehandle, commandline.param().abi(), showcommand) }
}
#[inline]
pub unsafe fn InstallHinfSectionW<P2>(window: super::super::Foundation::HWND, modulehandle: super::super::Foundation::HINSTANCE, commandline: P2, showcommand: i32)
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn InstallHinfSectionW(window : super::super::Foundation:: HWND, modulehandle : super::super::Foundation:: HINSTANCE, commandline : windows_core::PCWSTR, showcommand : i32));
    unsafe { InstallHinfSectionW(window, modulehandle, commandline.param().abi(), showcommand) }
}
#[inline]
pub unsafe fn SetupAddInstallSectionToDiskSpaceListA<P3>(diskspace: *const core::ffi::c_void, infhandle: *const core::ffi::c_void, layoutinfhandle: Option<*const core::ffi::c_void>, sectionname: P3, reserved1: Option<*const core::ffi::c_void>, reserved2: Option<u32>) -> windows_core::Result<()>
where
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupAddInstallSectionToDiskSpaceListA(diskspace : *const core::ffi::c_void, infhandle : *const core::ffi::c_void, layoutinfhandle : *const core::ffi::c_void, sectionname : windows_core::PCSTR, reserved1 : *const core::ffi::c_void, reserved2 : u32) -> windows_core::BOOL);
    unsafe { SetupAddInstallSectionToDiskSpaceListA(diskspace, infhandle, layoutinfhandle.unwrap_or(core::mem::zeroed()) as _, sectionname.param().abi(), reserved1.unwrap_or(core::mem::zeroed()) as _, reserved2.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn SetupAddInstallSectionToDiskSpaceListW<P3>(diskspace: *const core::ffi::c_void, infhandle: *const core::ffi::c_void, layoutinfhandle: Option<*const core::ffi::c_void>, sectionname: P3, reserved1: Option<*const core::ffi::c_void>, reserved2: Option<u32>) -> windows_core::Result<()>
where
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupAddInstallSectionToDiskSpaceListW(diskspace : *const core::ffi::c_void, infhandle : *const core::ffi::c_void, layoutinfhandle : *const core::ffi::c_void, sectionname : windows_core::PCWSTR, reserved1 : *const core::ffi::c_void, reserved2 : u32) -> windows_core::BOOL);
    unsafe { SetupAddInstallSectionToDiskSpaceListW(diskspace, infhandle, layoutinfhandle.unwrap_or(core::mem::zeroed()) as _, sectionname.param().abi(), reserved1.unwrap_or(core::mem::zeroed()) as _, reserved2.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn SetupAddSectionToDiskSpaceListA<P3>(diskspace: *const core::ffi::c_void, infhandle: *const core::ffi::c_void, listinfhandle: Option<*const core::ffi::c_void>, sectionname: P3, operation: SETUP_FILE_OPERATION, reserved1: Option<*const core::ffi::c_void>, reserved2: Option<u32>) -> windows_core::Result<()>
where
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupAddSectionToDiskSpaceListA(diskspace : *const core::ffi::c_void, infhandle : *const core::ffi::c_void, listinfhandle : *const core::ffi::c_void, sectionname : windows_core::PCSTR, operation : SETUP_FILE_OPERATION, reserved1 : *const core::ffi::c_void, reserved2 : u32) -> windows_core::BOOL);
    unsafe { SetupAddSectionToDiskSpaceListA(diskspace, infhandle, listinfhandle.unwrap_or(core::mem::zeroed()) as _, sectionname.param().abi(), operation, reserved1.unwrap_or(core::mem::zeroed()) as _, reserved2.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn SetupAddSectionToDiskSpaceListW<P3>(diskspace: *const core::ffi::c_void, infhandle: *const core::ffi::c_void, listinfhandle: Option<*const core::ffi::c_void>, sectionname: P3, operation: SETUP_FILE_OPERATION, reserved1: Option<*const core::ffi::c_void>, reserved2: Option<u32>) -> windows_core::Result<()>
where
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupAddSectionToDiskSpaceListW(diskspace : *const core::ffi::c_void, infhandle : *const core::ffi::c_void, listinfhandle : *const core::ffi::c_void, sectionname : windows_core::PCWSTR, operation : SETUP_FILE_OPERATION, reserved1 : *const core::ffi::c_void, reserved2 : u32) -> windows_core::BOOL);
    unsafe { SetupAddSectionToDiskSpaceListW(diskspace, infhandle, listinfhandle.unwrap_or(core::mem::zeroed()) as _, sectionname.param().abi(), operation, reserved1.unwrap_or(core::mem::zeroed()) as _, reserved2.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn SetupAddToDiskSpaceListA<P1>(diskspace: *const core::ffi::c_void, targetfilespec: P1, filesize: i64, operation: SETUP_FILE_OPERATION, reserved1: Option<*const core::ffi::c_void>, reserved2: Option<u32>) -> windows_core::Result<()>
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupAddToDiskSpaceListA(diskspace : *const core::ffi::c_void, targetfilespec : windows_core::PCSTR, filesize : i64, operation : SETUP_FILE_OPERATION, reserved1 : *const core::ffi::c_void, reserved2 : u32) -> windows_core::BOOL);
    unsafe { SetupAddToDiskSpaceListA(diskspace, targetfilespec.param().abi(), filesize, operation, reserved1.unwrap_or(core::mem::zeroed()) as _, reserved2.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn SetupAddToDiskSpaceListW<P1>(diskspace: *const core::ffi::c_void, targetfilespec: P1, filesize: i64, operation: SETUP_FILE_OPERATION, reserved1: Option<*const core::ffi::c_void>, reserved2: Option<u32>) -> windows_core::Result<()>
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupAddToDiskSpaceListW(diskspace : *const core::ffi::c_void, targetfilespec : windows_core::PCWSTR, filesize : i64, operation : SETUP_FILE_OPERATION, reserved1 : *const core::ffi::c_void, reserved2 : u32) -> windows_core::BOOL);
    unsafe { SetupAddToDiskSpaceListW(diskspace, targetfilespec.param().abi(), filesize, operation, reserved1.unwrap_or(core::mem::zeroed()) as _, reserved2.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn SetupAddToSourceListA<P1>(flags: u32, source: P1) -> windows_core::Result<()>
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupAddToSourceListA(flags : u32, source : windows_core::PCSTR) -> windows_core::BOOL);
    unsafe { SetupAddToSourceListA(flags, source.param().abi()).ok() }
}
#[inline]
pub unsafe fn SetupAddToSourceListW<P1>(flags: u32, source: P1) -> windows_core::Result<()>
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupAddToSourceListW(flags : u32, source : windows_core::PCWSTR) -> windows_core::BOOL);
    unsafe { SetupAddToSourceListW(flags, source.param().abi()).ok() }
}
#[inline]
pub unsafe fn SetupAdjustDiskSpaceListA<P1>(diskspace: *const core::ffi::c_void, driveroot: P1, amount: i64, reserved1: Option<*const core::ffi::c_void>, reserved2: Option<u32>) -> windows_core::Result<()>
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupAdjustDiskSpaceListA(diskspace : *const core::ffi::c_void, driveroot : windows_core::PCSTR, amount : i64, reserved1 : *const core::ffi::c_void, reserved2 : u32) -> windows_core::BOOL);
    unsafe { SetupAdjustDiskSpaceListA(diskspace, driveroot.param().abi(), amount, reserved1.unwrap_or(core::mem::zeroed()) as _, reserved2.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn SetupAdjustDiskSpaceListW<P1>(diskspace: *const core::ffi::c_void, driveroot: P1, amount: i64, reserved1: Option<*const core::ffi::c_void>, reserved2: Option<u32>) -> windows_core::Result<()>
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupAdjustDiskSpaceListW(diskspace : *const core::ffi::c_void, driveroot : windows_core::PCWSTR, amount : i64, reserved1 : *const core::ffi::c_void, reserved2 : u32) -> windows_core::BOOL);
    unsafe { SetupAdjustDiskSpaceListW(diskspace, driveroot.param().abi(), amount, reserved1.unwrap_or(core::mem::zeroed()) as _, reserved2.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn SetupBackupErrorA<P1, P2, P3>(hwndparent: super::super::Foundation::HWND, dialogtitle: P1, sourcefile: P2, targetfile: P3, win32errorcode: u32, style: u32) -> u32
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupBackupErrorA(hwndparent : super::super::Foundation:: HWND, dialogtitle : windows_core::PCSTR, sourcefile : windows_core::PCSTR, targetfile : windows_core::PCSTR, win32errorcode : u32, style : u32) -> u32);
    unsafe { SetupBackupErrorA(hwndparent, dialogtitle.param().abi(), sourcefile.param().abi(), targetfile.param().abi(), win32errorcode, style) }
}
#[inline]
pub unsafe fn SetupBackupErrorW<P1, P2, P3>(hwndparent: super::super::Foundation::HWND, dialogtitle: P1, sourcefile: P2, targetfile: P3, win32errorcode: u32, style: u32) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupBackupErrorW(hwndparent : super::super::Foundation:: HWND, dialogtitle : windows_core::PCWSTR, sourcefile : windows_core::PCWSTR, targetfile : windows_core::PCWSTR, win32errorcode : u32, style : u32) -> u32);
    unsafe { SetupBackupErrorW(hwndparent, dialogtitle.param().abi(), sourcefile.param().abi(), targetfile.param().abi(), win32errorcode, style) }
}
#[inline]
pub unsafe fn SetupCancelTemporarySourceList() -> windows_core::Result<()> {
    windows_core::link!("setupapi.dll" "system" fn SetupCancelTemporarySourceList() -> windows_core::BOOL);
    unsafe { SetupCancelTemporarySourceList().ok() }
}
#[inline]
pub unsafe fn SetupCloseFileQueue(queuehandle: *const core::ffi::c_void) -> windows_core::BOOL {
    windows_core::link!("setupapi.dll" "system" fn SetupCloseFileQueue(queuehandle : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe { SetupCloseFileQueue(queuehandle) }
}
#[inline]
pub unsafe fn SetupCloseInfFile(infhandle: *const core::ffi::c_void) {
    windows_core::link!("setupapi.dll" "system" fn SetupCloseInfFile(infhandle : *const core::ffi::c_void));
    unsafe { SetupCloseInfFile(infhandle) }
}
#[inline]
pub unsafe fn SetupCloseLog() {
    windows_core::link!("setupapi.dll" "system" fn SetupCloseLog());
    unsafe { SetupCloseLog() }
}
#[inline]
pub unsafe fn SetupCommitFileQueueA(owner: Option<super::super::Foundation::HWND>, queuehandle: *const core::ffi::c_void, msghandler: PSP_FILE_CALLBACK_A, context: *const core::ffi::c_void) -> windows_core::Result<()> {
    windows_core::link!("setupapi.dll" "system" fn SetupCommitFileQueueA(owner : super::super::Foundation:: HWND, queuehandle : *const core::ffi::c_void, msghandler : PSP_FILE_CALLBACK_A, context : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe { SetupCommitFileQueueA(owner.unwrap_or(core::mem::zeroed()) as _, queuehandle, msghandler, context).ok() }
}
#[inline]
pub unsafe fn SetupCommitFileQueueW(owner: Option<super::super::Foundation::HWND>, queuehandle: *const core::ffi::c_void, msghandler: PSP_FILE_CALLBACK_W, context: *const core::ffi::c_void) -> windows_core::Result<()> {
    windows_core::link!("setupapi.dll" "system" fn SetupCommitFileQueueW(owner : super::super::Foundation:: HWND, queuehandle : *const core::ffi::c_void, msghandler : PSP_FILE_CALLBACK_W, context : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe { SetupCommitFileQueueW(owner.unwrap_or(core::mem::zeroed()) as _, queuehandle, msghandler, context).ok() }
}
#[inline]
pub unsafe fn SetupConfigureWmiFromInfSectionA<P1>(infhandle: *const core::ffi::c_void, sectionname: P1, flags: u32) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupConfigureWmiFromInfSectionA(infhandle : *const core::ffi::c_void, sectionname : windows_core::PCSTR, flags : u32) -> windows_core::BOOL);
    unsafe { SetupConfigureWmiFromInfSectionA(infhandle, sectionname.param().abi(), flags) }
}
#[inline]
pub unsafe fn SetupConfigureWmiFromInfSectionW<P1>(infhandle: *const core::ffi::c_void, sectionname: P1, flags: u32) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupConfigureWmiFromInfSectionW(infhandle : *const core::ffi::c_void, sectionname : windows_core::PCWSTR, flags : u32) -> windows_core::BOOL);
    unsafe { SetupConfigureWmiFromInfSectionW(infhandle, sectionname.param().abi(), flags) }
}
#[inline]
pub unsafe fn SetupCopyErrorA<P1, P2, P3, P4, P5>(hwndparent: super::super::Foundation::HWND, dialogtitle: P1, diskname: P2, pathtosource: P3, sourcefile: P4, targetpathfile: P5, win32errorcode: u32, style: u32, pathbuffer: Option<&mut [u8]>, pathrequiredsize: Option<*mut u32>) -> u32
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
    P3: windows_core::Param<windows_core::PCSTR>,
    P4: windows_core::Param<windows_core::PCSTR>,
    P5: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupCopyErrorA(hwndparent : super::super::Foundation:: HWND, dialogtitle : windows_core::PCSTR, diskname : windows_core::PCSTR, pathtosource : windows_core::PCSTR, sourcefile : windows_core::PCSTR, targetpathfile : windows_core::PCSTR, win32errorcode : u32, style : u32, pathbuffer : windows_core::PSTR, pathbuffersize : u32, pathrequiredsize : *mut u32) -> u32);
    unsafe { SetupCopyErrorA(hwndparent, dialogtitle.param().abi(), diskname.param().abi(), pathtosource.param().abi(), sourcefile.param().abi(), targetpathfile.param().abi(), win32errorcode, style, core::mem::transmute(pathbuffer.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pathbuffer.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), pathrequiredsize.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn SetupCopyErrorW<P1, P2, P3, P4, P5>(hwndparent: super::super::Foundation::HWND, dialogtitle: P1, diskname: P2, pathtosource: P3, sourcefile: P4, targetpathfile: P5, win32errorcode: u32, style: u32, pathbuffer: Option<&mut [u16]>, pathrequiredsize: Option<*mut u32>) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
    P4: windows_core::Param<windows_core::PCWSTR>,
    P5: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupCopyErrorW(hwndparent : super::super::Foundation:: HWND, dialogtitle : windows_core::PCWSTR, diskname : windows_core::PCWSTR, pathtosource : windows_core::PCWSTR, sourcefile : windows_core::PCWSTR, targetpathfile : windows_core::PCWSTR, win32errorcode : u32, style : u32, pathbuffer : windows_core::PWSTR, pathbuffersize : u32, pathrequiredsize : *mut u32) -> u32);
    unsafe { SetupCopyErrorW(hwndparent, dialogtitle.param().abi(), diskname.param().abi(), pathtosource.param().abi(), sourcefile.param().abi(), targetpathfile.param().abi(), win32errorcode, style, core::mem::transmute(pathbuffer.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pathbuffer.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), pathrequiredsize.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn SetupCopyOEMInfA<P0, P1>(sourceinffilename: P0, oemsourcemedialocation: P1, oemsourcemediatype: OEM_SOURCE_MEDIA_TYPE, copystyle: SP_COPY_STYLE, destinationinffilename: Option<&mut [u8]>, requiredsize: Option<*mut u32>, destinationinffilenamecomponent: Option<*mut windows_core::PSTR>) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupCopyOEMInfA(sourceinffilename : windows_core::PCSTR, oemsourcemedialocation : windows_core::PCSTR, oemsourcemediatype : OEM_SOURCE_MEDIA_TYPE, copystyle : SP_COPY_STYLE, destinationinffilename : windows_core::PSTR, destinationinffilenamesize : u32, requiredsize : *mut u32, destinationinffilenamecomponent : *mut windows_core::PSTR) -> windows_core::BOOL);
    unsafe { SetupCopyOEMInfA(sourceinffilename.param().abi(), oemsourcemedialocation.param().abi(), oemsourcemediatype, copystyle, core::mem::transmute(destinationinffilename.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), destinationinffilename.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), requiredsize.unwrap_or(core::mem::zeroed()) as _, destinationinffilenamecomponent.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn SetupCopyOEMInfW<P0, P1>(sourceinffilename: P0, oemsourcemedialocation: P1, oemsourcemediatype: OEM_SOURCE_MEDIA_TYPE, copystyle: SP_COPY_STYLE, destinationinffilename: Option<&mut [u16]>, requiredsize: Option<*mut u32>, destinationinffilenamecomponent: Option<*mut windows_core::PWSTR>) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupCopyOEMInfW(sourceinffilename : windows_core::PCWSTR, oemsourcemedialocation : windows_core::PCWSTR, oemsourcemediatype : OEM_SOURCE_MEDIA_TYPE, copystyle : SP_COPY_STYLE, destinationinffilename : windows_core::PWSTR, destinationinffilenamesize : u32, requiredsize : *mut u32, destinationinffilenamecomponent : *mut windows_core::PWSTR) -> windows_core::BOOL);
    unsafe { SetupCopyOEMInfW(sourceinffilename.param().abi(), oemsourcemedialocation.param().abi(), oemsourcemediatype, copystyle, core::mem::transmute(destinationinffilename.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), destinationinffilename.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), requiredsize.unwrap_or(core::mem::zeroed()) as _, destinationinffilenamecomponent.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn SetupCreateDiskSpaceListA(reserved1: Option<*const core::ffi::c_void>, reserved2: Option<u32>, flags: u32) -> *mut core::ffi::c_void {
    windows_core::link!("setupapi.dll" "system" fn SetupCreateDiskSpaceListA(reserved1 : *const core::ffi::c_void, reserved2 : u32, flags : u32) -> *mut core::ffi::c_void);
    unsafe { SetupCreateDiskSpaceListA(reserved1.unwrap_or(core::mem::zeroed()) as _, reserved2.unwrap_or(core::mem::zeroed()) as _, flags) }
}
#[inline]
pub unsafe fn SetupCreateDiskSpaceListW(reserved1: Option<*const core::ffi::c_void>, reserved2: Option<u32>, flags: u32) -> *mut core::ffi::c_void {
    windows_core::link!("setupapi.dll" "system" fn SetupCreateDiskSpaceListW(reserved1 : *const core::ffi::c_void, reserved2 : u32, flags : u32) -> *mut core::ffi::c_void);
    unsafe { SetupCreateDiskSpaceListW(reserved1.unwrap_or(core::mem::zeroed()) as _, reserved2.unwrap_or(core::mem::zeroed()) as _, flags) }
}
#[inline]
pub unsafe fn SetupDecompressOrCopyFileA<P0, P1>(sourcefilename: P0, targetfilename: P1, compressiontype: Option<*const FILE_COMPRESSION_TYPE>) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupDecompressOrCopyFileA(sourcefilename : windows_core::PCSTR, targetfilename : windows_core::PCSTR, compressiontype : *const FILE_COMPRESSION_TYPE) -> u32);
    unsafe { SetupDecompressOrCopyFileA(sourcefilename.param().abi(), targetfilename.param().abi(), compressiontype.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn SetupDecompressOrCopyFileW<P0, P1>(sourcefilename: P0, targetfilename: P1, compressiontype: Option<*const FILE_COMPRESSION_TYPE>) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupDecompressOrCopyFileW(sourcefilename : windows_core::PCWSTR, targetfilename : windows_core::PCWSTR, compressiontype : *const FILE_COMPRESSION_TYPE) -> u32);
    unsafe { SetupDecompressOrCopyFileW(sourcefilename.param().abi(), targetfilename.param().abi(), compressiontype.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn SetupDefaultQueueCallbackA(context: *const core::ffi::c_void, notification: u32, param1: usize, param2: usize) -> u32 {
    windows_core::link!("setupapi.dll" "system" fn SetupDefaultQueueCallbackA(context : *const core::ffi::c_void, notification : u32, param1 : usize, param2 : usize) -> u32);
    unsafe { SetupDefaultQueueCallbackA(context, notification, param1, param2) }
}
#[inline]
pub unsafe fn SetupDefaultQueueCallbackW(context: *const core::ffi::c_void, notification: u32, param1: usize, param2: usize) -> u32 {
    windows_core::link!("setupapi.dll" "system" fn SetupDefaultQueueCallbackW(context : *const core::ffi::c_void, notification : u32, param1 : usize, param2 : usize) -> u32);
    unsafe { SetupDefaultQueueCallbackW(context, notification, param1, param2) }
}
#[inline]
pub unsafe fn SetupDeleteErrorA<P1, P2>(hwndparent: super::super::Foundation::HWND, dialogtitle: P1, file: P2, win32errorcode: u32, style: u32) -> u32
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupDeleteErrorA(hwndparent : super::super::Foundation:: HWND, dialogtitle : windows_core::PCSTR, file : windows_core::PCSTR, win32errorcode : u32, style : u32) -> u32);
    unsafe { SetupDeleteErrorA(hwndparent, dialogtitle.param().abi(), file.param().abi(), win32errorcode, style) }
}
#[inline]
pub unsafe fn SetupDeleteErrorW<P1, P2>(hwndparent: super::super::Foundation::HWND, dialogtitle: P1, file: P2, win32errorcode: u32, style: u32) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupDeleteErrorW(hwndparent : super::super::Foundation:: HWND, dialogtitle : windows_core::PCWSTR, file : windows_core::PCWSTR, win32errorcode : u32, style : u32) -> u32);
    unsafe { SetupDeleteErrorW(hwndparent, dialogtitle.param().abi(), file.param().abi(), win32errorcode, style) }
}
#[inline]
pub unsafe fn SetupDestroyDiskSpaceList(diskspace: *mut core::ffi::c_void) -> windows_core::Result<()> {
    windows_core::link!("setupapi.dll" "system" fn SetupDestroyDiskSpaceList(diskspace : *mut core::ffi::c_void) -> windows_core::BOOL);
    unsafe { SetupDestroyDiskSpaceList(diskspace as _).ok() }
}
#[inline]
pub unsafe fn SetupDiAskForOEMDisk(deviceinfoset: HDEVINFO, deviceinfodata: Option<*const SP_DEVINFO_DATA>) -> windows_core::Result<()> {
    windows_core::link!("setupapi.dll" "system" fn SetupDiAskForOEMDisk(deviceinfoset : HDEVINFO, deviceinfodata : *const SP_DEVINFO_DATA) -> windows_core::BOOL);
    unsafe { SetupDiAskForOEMDisk(deviceinfoset, deviceinfodata.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn SetupDiBuildClassInfoList(flags: u32, classguidlist: Option<&mut [windows_core::GUID]>, requiredsize: *mut u32) -> windows_core::Result<()> {
    windows_core::link!("setupapi.dll" "system" fn SetupDiBuildClassInfoList(flags : u32, classguidlist : *mut windows_core::GUID, classguidlistsize : u32, requiredsize : *mut u32) -> windows_core::BOOL);
    unsafe { SetupDiBuildClassInfoList(flags, core::mem::transmute(classguidlist.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), classguidlist.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), requiredsize as _).ok() }
}
#[inline]
pub unsafe fn SetupDiBuildClassInfoListExA<P4>(flags: u32, classguidlist: Option<&mut [windows_core::GUID]>, requiredsize: *mut u32, machinename: P4, reserved: Option<*const core::ffi::c_void>) -> windows_core::Result<()>
where
    P4: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupDiBuildClassInfoListExA(flags : u32, classguidlist : *mut windows_core::GUID, classguidlistsize : u32, requiredsize : *mut u32, machinename : windows_core::PCSTR, reserved : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe { SetupDiBuildClassInfoListExA(flags, core::mem::transmute(classguidlist.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), classguidlist.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), requiredsize as _, machinename.param().abi(), reserved.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn SetupDiBuildClassInfoListExW<P4>(flags: u32, classguidlist: Option<&mut [windows_core::GUID]>, requiredsize: *mut u32, machinename: P4, reserved: Option<*const core::ffi::c_void>) -> windows_core::Result<()>
where
    P4: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupDiBuildClassInfoListExW(flags : u32, classguidlist : *mut windows_core::GUID, classguidlistsize : u32, requiredsize : *mut u32, machinename : windows_core::PCWSTR, reserved : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe { SetupDiBuildClassInfoListExW(flags, core::mem::transmute(classguidlist.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), classguidlist.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), requiredsize as _, machinename.param().abi(), reserved.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn SetupDiBuildDriverInfoList(deviceinfoset: HDEVINFO, deviceinfodata: Option<*mut SP_DEVINFO_DATA>, drivertype: SETUP_DI_DRIVER_TYPE) -> windows_core::Result<()> {
    windows_core::link!("setupapi.dll" "system" fn SetupDiBuildDriverInfoList(deviceinfoset : HDEVINFO, deviceinfodata : *mut SP_DEVINFO_DATA, drivertype : SETUP_DI_DRIVER_TYPE) -> windows_core::BOOL);
    unsafe { SetupDiBuildDriverInfoList(deviceinfoset, deviceinfodata.unwrap_or(core::mem::zeroed()) as _, drivertype).ok() }
}
#[inline]
pub unsafe fn SetupDiCallClassInstaller(installfunction: DI_FUNCTION, deviceinfoset: HDEVINFO, deviceinfodata: Option<*const SP_DEVINFO_DATA>) -> windows_core::Result<()> {
    windows_core::link!("setupapi.dll" "system" fn SetupDiCallClassInstaller(installfunction : DI_FUNCTION, deviceinfoset : HDEVINFO, deviceinfodata : *const SP_DEVINFO_DATA) -> windows_core::BOOL);
    unsafe { SetupDiCallClassInstaller(installfunction, deviceinfoset, deviceinfodata.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn SetupDiCancelDriverInfoSearch(deviceinfoset: HDEVINFO) -> windows_core::Result<()> {
    windows_core::link!("setupapi.dll" "system" fn SetupDiCancelDriverInfoSearch(deviceinfoset : HDEVINFO) -> windows_core::BOOL);
    unsafe { SetupDiCancelDriverInfoSearch(deviceinfoset).ok() }
}
#[inline]
pub unsafe fn SetupDiChangeState(deviceinfoset: HDEVINFO, deviceinfodata: *mut SP_DEVINFO_DATA) -> windows_core::Result<()> {
    windows_core::link!("setupapi.dll" "system" fn SetupDiChangeState(deviceinfoset : HDEVINFO, deviceinfodata : *mut SP_DEVINFO_DATA) -> windows_core::BOOL);
    unsafe { SetupDiChangeState(deviceinfoset, deviceinfodata as _).ok() }
}
#[inline]
pub unsafe fn SetupDiClassGuidsFromNameA<P0>(classname: P0, classguidlist: &mut [windows_core::GUID], requiredsize: *mut u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupDiClassGuidsFromNameA(classname : windows_core::PCSTR, classguidlist : *mut windows_core::GUID, classguidlistsize : u32, requiredsize : *mut u32) -> windows_core::BOOL);
    unsafe { SetupDiClassGuidsFromNameA(classname.param().abi(), core::mem::transmute(classguidlist.as_ptr()), classguidlist.len().try_into().unwrap(), requiredsize as _).ok() }
}
#[inline]
pub unsafe fn SetupDiClassGuidsFromNameExA<P0, P4>(classname: P0, classguidlist: &mut [windows_core::GUID], requiredsize: *mut u32, machinename: P4, reserved: Option<*const core::ffi::c_void>) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P4: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupDiClassGuidsFromNameExA(classname : windows_core::PCSTR, classguidlist : *mut windows_core::GUID, classguidlistsize : u32, requiredsize : *mut u32, machinename : windows_core::PCSTR, reserved : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe { SetupDiClassGuidsFromNameExA(classname.param().abi(), core::mem::transmute(classguidlist.as_ptr()), classguidlist.len().try_into().unwrap(), requiredsize as _, machinename.param().abi(), reserved.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn SetupDiClassGuidsFromNameExW<P0, P4>(classname: P0, classguidlist: &mut [windows_core::GUID], requiredsize: *mut u32, machinename: P4, reserved: Option<*const core::ffi::c_void>) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P4: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupDiClassGuidsFromNameExW(classname : windows_core::PCWSTR, classguidlist : *mut windows_core::GUID, classguidlistsize : u32, requiredsize : *mut u32, machinename : windows_core::PCWSTR, reserved : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe { SetupDiClassGuidsFromNameExW(classname.param().abi(), core::mem::transmute(classguidlist.as_ptr()), classguidlist.len().try_into().unwrap(), requiredsize as _, machinename.param().abi(), reserved.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn SetupDiClassGuidsFromNameW<P0>(classname: P0, classguidlist: &mut [windows_core::GUID], requiredsize: *mut u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupDiClassGuidsFromNameW(classname : windows_core::PCWSTR, classguidlist : *mut windows_core::GUID, classguidlistsize : u32, requiredsize : *mut u32) -> windows_core::BOOL);
    unsafe { SetupDiClassGuidsFromNameW(classname.param().abi(), core::mem::transmute(classguidlist.as_ptr()), classguidlist.len().try_into().unwrap(), requiredsize as _).ok() }
}
#[inline]
pub unsafe fn SetupDiClassNameFromGuidA(classguid: *const windows_core::GUID, classname: &mut [u8], requiredsize: Option<*mut u32>) -> windows_core::Result<()> {
    windows_core::link!("setupapi.dll" "system" fn SetupDiClassNameFromGuidA(classguid : *const windows_core::GUID, classname : windows_core::PSTR, classnamesize : u32, requiredsize : *mut u32) -> windows_core::BOOL);
    unsafe { SetupDiClassNameFromGuidA(classguid, core::mem::transmute(classname.as_ptr()), classname.len().try_into().unwrap(), requiredsize.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn SetupDiClassNameFromGuidExA<P4>(classguid: *const windows_core::GUID, classname: &mut [u8], requiredsize: Option<*mut u32>, machinename: P4, reserved: Option<*const core::ffi::c_void>) -> windows_core::Result<()>
where
    P4: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupDiClassNameFromGuidExA(classguid : *const windows_core::GUID, classname : windows_core::PSTR, classnamesize : u32, requiredsize : *mut u32, machinename : windows_core::PCSTR, reserved : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe { SetupDiClassNameFromGuidExA(classguid, core::mem::transmute(classname.as_ptr()), classname.len().try_into().unwrap(), requiredsize.unwrap_or(core::mem::zeroed()) as _, machinename.param().abi(), reserved.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn SetupDiClassNameFromGuidExW<P4>(classguid: *const windows_core::GUID, classname: &mut [u16], requiredsize: Option<*mut u32>, machinename: P4, reserved: Option<*const core::ffi::c_void>) -> windows_core::Result<()>
where
    P4: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupDiClassNameFromGuidExW(classguid : *const windows_core::GUID, classname : windows_core::PWSTR, classnamesize : u32, requiredsize : *mut u32, machinename : windows_core::PCWSTR, reserved : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe { SetupDiClassNameFromGuidExW(classguid, core::mem::transmute(classname.as_ptr()), classname.len().try_into().unwrap(), requiredsize.unwrap_or(core::mem::zeroed()) as _, machinename.param().abi(), reserved.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn SetupDiClassNameFromGuidW(classguid: *const windows_core::GUID, classname: &mut [u16], requiredsize: Option<*mut u32>) -> windows_core::Result<()> {
    windows_core::link!("setupapi.dll" "system" fn SetupDiClassNameFromGuidW(classguid : *const windows_core::GUID, classname : windows_core::PWSTR, classnamesize : u32, requiredsize : *mut u32) -> windows_core::BOOL);
    unsafe { SetupDiClassNameFromGuidW(classguid, core::mem::transmute(classname.as_ptr()), classname.len().try_into().unwrap(), requiredsize.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn SetupDiCreateDevRegKeyA<P6>(deviceinfoset: HDEVINFO, deviceinfodata: *const SP_DEVINFO_DATA, scope: u32, hwprofile: u32, keytype: u32, infhandle: Option<*const core::ffi::c_void>, infsectionname: P6) -> windows_core::Result<super::super::System::Registry::HKEY>
where
    P6: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupDiCreateDevRegKeyA(deviceinfoset : HDEVINFO, deviceinfodata : *const SP_DEVINFO_DATA, scope : u32, hwprofile : u32, keytype : u32, infhandle : *const core::ffi::c_void, infsectionname : windows_core::PCSTR) -> super::super::System::Registry:: HKEY);
    let result__ = unsafe { SetupDiCreateDevRegKeyA(deviceinfoset, deviceinfodata, scope, hwprofile, keytype, infhandle.unwrap_or(core::mem::zeroed()) as _, infsectionname.param().abi()) };
    (!result__.is_invalid()).then_some(result__).ok_or_else(windows_core::Error::from_thread)
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn SetupDiCreateDevRegKeyW<P6>(deviceinfoset: HDEVINFO, deviceinfodata: *const SP_DEVINFO_DATA, scope: u32, hwprofile: u32, keytype: u32, infhandle: Option<*const core::ffi::c_void>, infsectionname: P6) -> windows_core::Result<super::super::System::Registry::HKEY>
where
    P6: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupDiCreateDevRegKeyW(deviceinfoset : HDEVINFO, deviceinfodata : *const SP_DEVINFO_DATA, scope : u32, hwprofile : u32, keytype : u32, infhandle : *const core::ffi::c_void, infsectionname : windows_core::PCWSTR) -> super::super::System::Registry:: HKEY);
    let result__ = unsafe { SetupDiCreateDevRegKeyW(deviceinfoset, deviceinfodata, scope, hwprofile, keytype, infhandle.unwrap_or(core::mem::zeroed()) as _, infsectionname.param().abi()) };
    (!result__.is_invalid()).then_some(result__).ok_or_else(windows_core::Error::from_thread)
}
#[inline]
pub unsafe fn SetupDiCreateDeviceInfoA<P1, P3>(deviceinfoset: HDEVINFO, devicename: P1, classguid: *const windows_core::GUID, devicedescription: P3, hwndparent: Option<super::super::Foundation::HWND>, creationflags: SETUP_DI_DEVICE_CREATION_FLAGS, deviceinfodata: Option<*mut SP_DEVINFO_DATA>) -> windows_core::Result<()>
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupDiCreateDeviceInfoA(deviceinfoset : HDEVINFO, devicename : windows_core::PCSTR, classguid : *const windows_core::GUID, devicedescription : windows_core::PCSTR, hwndparent : super::super::Foundation:: HWND, creationflags : SETUP_DI_DEVICE_CREATION_FLAGS, deviceinfodata : *mut SP_DEVINFO_DATA) -> windows_core::BOOL);
    unsafe { SetupDiCreateDeviceInfoA(deviceinfoset, devicename.param().abi(), classguid, devicedescription.param().abi(), hwndparent.unwrap_or(core::mem::zeroed()) as _, creationflags, deviceinfodata.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn SetupDiCreateDeviceInfoList(classguid: Option<*const windows_core::GUID>, hwndparent: Option<super::super::Foundation::HWND>) -> windows_core::Result<HDEVINFO> {
    windows_core::link!("setupapi.dll" "system" fn SetupDiCreateDeviceInfoList(classguid : *const windows_core::GUID, hwndparent : super::super::Foundation:: HWND) -> HDEVINFO);
    let result__ = unsafe { SetupDiCreateDeviceInfoList(classguid.unwrap_or(core::mem::zeroed()) as _, hwndparent.unwrap_or(core::mem::zeroed()) as _) };
    (!result__.is_invalid()).then_some(result__).ok_or_else(windows_core::Error::from_thread)
}
#[inline]
pub unsafe fn SetupDiCreateDeviceInfoListExA<P2>(classguid: Option<*const windows_core::GUID>, hwndparent: Option<super::super::Foundation::HWND>, machinename: P2, reserved: Option<*const core::ffi::c_void>) -> windows_core::Result<HDEVINFO>
where
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupDiCreateDeviceInfoListExA(classguid : *const windows_core::GUID, hwndparent : super::super::Foundation:: HWND, machinename : windows_core::PCSTR, reserved : *const core::ffi::c_void) -> HDEVINFO);
    let result__ = unsafe { SetupDiCreateDeviceInfoListExA(classguid.unwrap_or(core::mem::zeroed()) as _, hwndparent.unwrap_or(core::mem::zeroed()) as _, machinename.param().abi(), reserved.unwrap_or(core::mem::zeroed()) as _) };
    (!result__.is_invalid()).then_some(result__).ok_or_else(windows_core::Error::from_thread)
}
#[inline]
pub unsafe fn SetupDiCreateDeviceInfoListExW<P2>(classguid: Option<*const windows_core::GUID>, hwndparent: Option<super::super::Foundation::HWND>, machinename: P2, reserved: Option<*const core::ffi::c_void>) -> windows_core::Result<HDEVINFO>
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupDiCreateDeviceInfoListExW(classguid : *const windows_core::GUID, hwndparent : super::super::Foundation:: HWND, machinename : windows_core::PCWSTR, reserved : *const core::ffi::c_void) -> HDEVINFO);
    let result__ = unsafe { SetupDiCreateDeviceInfoListExW(classguid.unwrap_or(core::mem::zeroed()) as _, hwndparent.unwrap_or(core::mem::zeroed()) as _, machinename.param().abi(), reserved.unwrap_or(core::mem::zeroed()) as _) };
    (!result__.is_invalid()).then_some(result__).ok_or_else(windows_core::Error::from_thread)
}
#[inline]
pub unsafe fn SetupDiCreateDeviceInfoW<P1, P3>(deviceinfoset: HDEVINFO, devicename: P1, classguid: *const windows_core::GUID, devicedescription: P3, hwndparent: Option<super::super::Foundation::HWND>, creationflags: SETUP_DI_DEVICE_CREATION_FLAGS, deviceinfodata: Option<*mut SP_DEVINFO_DATA>) -> windows_core::Result<()>
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupDiCreateDeviceInfoW(deviceinfoset : HDEVINFO, devicename : windows_core::PCWSTR, classguid : *const windows_core::GUID, devicedescription : windows_core::PCWSTR, hwndparent : super::super::Foundation:: HWND, creationflags : SETUP_DI_DEVICE_CREATION_FLAGS, deviceinfodata : *mut SP_DEVINFO_DATA) -> windows_core::BOOL);
    unsafe { SetupDiCreateDeviceInfoW(deviceinfoset, devicename.param().abi(), classguid, devicedescription.param().abi(), hwndparent.unwrap_or(core::mem::zeroed()) as _, creationflags, deviceinfodata.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn SetupDiCreateDeviceInterfaceA<P3>(deviceinfoset: HDEVINFO, deviceinfodata: *const SP_DEVINFO_DATA, interfaceclassguid: *const windows_core::GUID, referencestring: P3, creationflags: u32, deviceinterfacedata: Option<*mut SP_DEVICE_INTERFACE_DATA>) -> windows_core::Result<()>
where
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupDiCreateDeviceInterfaceA(deviceinfoset : HDEVINFO, deviceinfodata : *const SP_DEVINFO_DATA, interfaceclassguid : *const windows_core::GUID, referencestring : windows_core::PCSTR, creationflags : u32, deviceinterfacedata : *mut SP_DEVICE_INTERFACE_DATA) -> windows_core::BOOL);
    unsafe { SetupDiCreateDeviceInterfaceA(deviceinfoset, deviceinfodata, interfaceclassguid, referencestring.param().abi(), creationflags, deviceinterfacedata.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn SetupDiCreateDeviceInterfaceRegKeyA<P5>(deviceinfoset: HDEVINFO, deviceinterfacedata: *const SP_DEVICE_INTERFACE_DATA, reserved: Option<u32>, samdesired: u32, infhandle: Option<*const core::ffi::c_void>, infsectionname: P5) -> windows_core::Result<super::super::System::Registry::HKEY>
where
    P5: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupDiCreateDeviceInterfaceRegKeyA(deviceinfoset : HDEVINFO, deviceinterfacedata : *const SP_DEVICE_INTERFACE_DATA, reserved : u32, samdesired : u32, infhandle : *const core::ffi::c_void, infsectionname : windows_core::PCSTR) -> super::super::System::Registry:: HKEY);
    let result__ = unsafe { SetupDiCreateDeviceInterfaceRegKeyA(deviceinfoset, deviceinterfacedata, reserved.unwrap_or(core::mem::zeroed()) as _, samdesired, infhandle.unwrap_or(core::mem::zeroed()) as _, infsectionname.param().abi()) };
    (!result__.is_invalid()).then_some(result__).ok_or_else(windows_core::Error::from_thread)
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn SetupDiCreateDeviceInterfaceRegKeyW<P5>(deviceinfoset: HDEVINFO, deviceinterfacedata: *const SP_DEVICE_INTERFACE_DATA, reserved: Option<u32>, samdesired: u32, infhandle: Option<*const core::ffi::c_void>, infsectionname: P5) -> windows_core::Result<super::super::System::Registry::HKEY>
where
    P5: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupDiCreateDeviceInterfaceRegKeyW(deviceinfoset : HDEVINFO, deviceinterfacedata : *const SP_DEVICE_INTERFACE_DATA, reserved : u32, samdesired : u32, infhandle : *const core::ffi::c_void, infsectionname : windows_core::PCWSTR) -> super::super::System::Registry:: HKEY);
    let result__ = unsafe { SetupDiCreateDeviceInterfaceRegKeyW(deviceinfoset, deviceinterfacedata, reserved.unwrap_or(core::mem::zeroed()) as _, samdesired, infhandle.unwrap_or(core::mem::zeroed()) as _, infsectionname.param().abi()) };
    (!result__.is_invalid()).then_some(result__).ok_or_else(windows_core::Error::from_thread)
}
#[inline]
pub unsafe fn SetupDiCreateDeviceInterfaceW<P3>(deviceinfoset: HDEVINFO, deviceinfodata: *const SP_DEVINFO_DATA, interfaceclassguid: *const windows_core::GUID, referencestring: P3, creationflags: u32, deviceinterfacedata: Option<*mut SP_DEVICE_INTERFACE_DATA>) -> windows_core::Result<()>
where
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupDiCreateDeviceInterfaceW(deviceinfoset : HDEVINFO, deviceinfodata : *const SP_DEVINFO_DATA, interfaceclassguid : *const windows_core::GUID, referencestring : windows_core::PCWSTR, creationflags : u32, deviceinterfacedata : *mut SP_DEVICE_INTERFACE_DATA) -> windows_core::BOOL);
    unsafe { SetupDiCreateDeviceInterfaceW(deviceinfoset, deviceinfodata, interfaceclassguid, referencestring.param().abi(), creationflags, deviceinterfacedata.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn SetupDiDeleteDevRegKey(deviceinfoset: HDEVINFO, deviceinfodata: *const SP_DEVINFO_DATA, scope: u32, hwprofile: u32, keytype: u32) -> windows_core::Result<()> {
    windows_core::link!("setupapi.dll" "system" fn SetupDiDeleteDevRegKey(deviceinfoset : HDEVINFO, deviceinfodata : *const SP_DEVINFO_DATA, scope : u32, hwprofile : u32, keytype : u32) -> windows_core::BOOL);
    unsafe { SetupDiDeleteDevRegKey(deviceinfoset, deviceinfodata, scope, hwprofile, keytype).ok() }
}
#[inline]
pub unsafe fn SetupDiDeleteDeviceInfo(deviceinfoset: HDEVINFO, deviceinfodata: *const SP_DEVINFO_DATA) -> windows_core::Result<()> {
    windows_core::link!("setupapi.dll" "system" fn SetupDiDeleteDeviceInfo(deviceinfoset : HDEVINFO, deviceinfodata : *const SP_DEVINFO_DATA) -> windows_core::BOOL);
    unsafe { SetupDiDeleteDeviceInfo(deviceinfoset, deviceinfodata).ok() }
}
#[inline]
pub unsafe fn SetupDiDeleteDeviceInterfaceData(deviceinfoset: HDEVINFO, deviceinterfacedata: *const SP_DEVICE_INTERFACE_DATA) -> windows_core::Result<()> {
    windows_core::link!("setupapi.dll" "system" fn SetupDiDeleteDeviceInterfaceData(deviceinfoset : HDEVINFO, deviceinterfacedata : *const SP_DEVICE_INTERFACE_DATA) -> windows_core::BOOL);
    unsafe { SetupDiDeleteDeviceInterfaceData(deviceinfoset, deviceinterfacedata).ok() }
}
#[inline]
pub unsafe fn SetupDiDeleteDeviceInterfaceRegKey(deviceinfoset: HDEVINFO, deviceinterfacedata: *const SP_DEVICE_INTERFACE_DATA, reserved: Option<u32>) -> windows_core::Result<()> {
    windows_core::link!("setupapi.dll" "system" fn SetupDiDeleteDeviceInterfaceRegKey(deviceinfoset : HDEVINFO, deviceinterfacedata : *const SP_DEVICE_INTERFACE_DATA, reserved : u32) -> windows_core::BOOL);
    unsafe { SetupDiDeleteDeviceInterfaceRegKey(deviceinfoset, deviceinterfacedata, reserved.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[cfg(feature = "Win32_UI_Controls")]
#[inline]
pub unsafe fn SetupDiDestroyClassImageList(classimagelistdata: *const SP_CLASSIMAGELIST_DATA) -> windows_core::Result<()> {
    windows_core::link!("setupapi.dll" "system" fn SetupDiDestroyClassImageList(classimagelistdata : *const SP_CLASSIMAGELIST_DATA) -> windows_core::BOOL);
    unsafe { SetupDiDestroyClassImageList(classimagelistdata).ok() }
}
#[inline]
pub unsafe fn SetupDiDestroyDeviceInfoList(deviceinfoset: HDEVINFO) -> windows_core::Result<()> {
    windows_core::link!("setupapi.dll" "system" fn SetupDiDestroyDeviceInfoList(deviceinfoset : HDEVINFO) -> windows_core::BOOL);
    unsafe { SetupDiDestroyDeviceInfoList(deviceinfoset).ok() }
}
#[inline]
pub unsafe fn SetupDiDestroyDriverInfoList(deviceinfoset: HDEVINFO, deviceinfodata: Option<*const SP_DEVINFO_DATA>, drivertype: SETUP_DI_DRIVER_TYPE) -> windows_core::Result<()> {
    windows_core::link!("setupapi.dll" "system" fn SetupDiDestroyDriverInfoList(deviceinfoset : HDEVINFO, deviceinfodata : *const SP_DEVINFO_DATA, drivertype : SETUP_DI_DRIVER_TYPE) -> windows_core::BOOL);
    unsafe { SetupDiDestroyDriverInfoList(deviceinfoset, deviceinfodata.unwrap_or(core::mem::zeroed()) as _, drivertype).ok() }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn SetupDiDrawMiniIcon(hdc: super::super::Graphics::Gdi::HDC, rc: super::super::Foundation::RECT, miniiconindex: i32, flags: u32) -> i32 {
    windows_core::link!("setupapi.dll" "system" fn SetupDiDrawMiniIcon(hdc : super::super::Graphics::Gdi:: HDC, rc : super::super::Foundation:: RECT, miniiconindex : i32, flags : u32) -> i32);
    unsafe { SetupDiDrawMiniIcon(hdc, core::mem::transmute(rc), miniiconindex, flags) }
}
#[inline]
pub unsafe fn SetupDiEnumDeviceInfo(deviceinfoset: HDEVINFO, memberindex: u32, deviceinfodata: *mut SP_DEVINFO_DATA) -> windows_core::Result<()> {
    windows_core::link!("setupapi.dll" "system" fn SetupDiEnumDeviceInfo(deviceinfoset : HDEVINFO, memberindex : u32, deviceinfodata : *mut SP_DEVINFO_DATA) -> windows_core::BOOL);
    unsafe { SetupDiEnumDeviceInfo(deviceinfoset, memberindex, deviceinfodata as _).ok() }
}
#[inline]
pub unsafe fn SetupDiEnumDeviceInterfaces(deviceinfoset: HDEVINFO, deviceinfodata: Option<*const SP_DEVINFO_DATA>, interfaceclassguid: *const windows_core::GUID, memberindex: u32, deviceinterfacedata: *mut SP_DEVICE_INTERFACE_DATA) -> windows_core::Result<()> {
    windows_core::link!("setupapi.dll" "system" fn SetupDiEnumDeviceInterfaces(deviceinfoset : HDEVINFO, deviceinfodata : *const SP_DEVINFO_DATA, interfaceclassguid : *const windows_core::GUID, memberindex : u32, deviceinterfacedata : *mut SP_DEVICE_INTERFACE_DATA) -> windows_core::BOOL);
    unsafe { SetupDiEnumDeviceInterfaces(deviceinfoset, deviceinfodata.unwrap_or(core::mem::zeroed()) as _, interfaceclassguid, memberindex, deviceinterfacedata as _).ok() }
}
#[inline]
pub unsafe fn SetupDiEnumDriverInfoA(deviceinfoset: HDEVINFO, deviceinfodata: Option<*const SP_DEVINFO_DATA>, drivertype: SETUP_DI_DRIVER_TYPE, memberindex: u32, driverinfodata: *mut SP_DRVINFO_DATA_V2_A) -> windows_core::Result<()> {
    windows_core::link!("setupapi.dll" "system" fn SetupDiEnumDriverInfoA(deviceinfoset : HDEVINFO, deviceinfodata : *const SP_DEVINFO_DATA, drivertype : SETUP_DI_DRIVER_TYPE, memberindex : u32, driverinfodata : *mut SP_DRVINFO_DATA_V2_A) -> windows_core::BOOL);
    unsafe { SetupDiEnumDriverInfoA(deviceinfoset, deviceinfodata.unwrap_or(core::mem::zeroed()) as _, drivertype, memberindex, driverinfodata as _).ok() }
}
#[inline]
pub unsafe fn SetupDiEnumDriverInfoW(deviceinfoset: HDEVINFO, deviceinfodata: Option<*const SP_DEVINFO_DATA>, drivertype: SETUP_DI_DRIVER_TYPE, memberindex: u32, driverinfodata: *mut SP_DRVINFO_DATA_V2_W) -> windows_core::Result<()> {
    windows_core::link!("setupapi.dll" "system" fn SetupDiEnumDriverInfoW(deviceinfoset : HDEVINFO, deviceinfodata : *const SP_DEVINFO_DATA, drivertype : SETUP_DI_DRIVER_TYPE, memberindex : u32, driverinfodata : *mut SP_DRVINFO_DATA_V2_W) -> windows_core::BOOL);
    unsafe { SetupDiEnumDriverInfoW(deviceinfoset, deviceinfodata.unwrap_or(core::mem::zeroed()) as _, drivertype, memberindex, driverinfodata as _).ok() }
}
#[cfg(all(feature = "Win32_System_Diagnostics_Debug", feature = "Win32_System_SystemInformation"))]
#[inline]
pub unsafe fn SetupDiGetActualModelsSectionA(context: *const INFCONTEXT, alternateplatforminfo: Option<*const SP_ALTPLATFORM_INFO_V2>, infsectionwithext: Option<&mut [u8]>, requiredsize: Option<*mut u32>, reserved: Option<*const core::ffi::c_void>) -> windows_core::Result<()> {
    windows_core::link!("setupapi.dll" "system" fn SetupDiGetActualModelsSectionA(context : *const INFCONTEXT, alternateplatforminfo : *const SP_ALTPLATFORM_INFO_V2, infsectionwithext : windows_core::PSTR, infsectionwithextsize : u32, requiredsize : *mut u32, reserved : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe { SetupDiGetActualModelsSectionA(context, alternateplatforminfo.unwrap_or(core::mem::zeroed()) as _, core::mem::transmute(infsectionwithext.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), infsectionwithext.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), requiredsize.unwrap_or(core::mem::zeroed()) as _, reserved.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[cfg(all(feature = "Win32_System_Diagnostics_Debug", feature = "Win32_System_SystemInformation"))]
#[inline]
pub unsafe fn SetupDiGetActualModelsSectionW(context: *const INFCONTEXT, alternateplatforminfo: Option<*const SP_ALTPLATFORM_INFO_V2>, infsectionwithext: Option<&mut [u16]>, requiredsize: Option<*mut u32>, reserved: Option<*const core::ffi::c_void>) -> windows_core::Result<()> {
    windows_core::link!("setupapi.dll" "system" fn SetupDiGetActualModelsSectionW(context : *const INFCONTEXT, alternateplatforminfo : *const SP_ALTPLATFORM_INFO_V2, infsectionwithext : windows_core::PWSTR, infsectionwithextsize : u32, requiredsize : *mut u32, reserved : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe { SetupDiGetActualModelsSectionW(context, alternateplatforminfo.unwrap_or(core::mem::zeroed()) as _, core::mem::transmute(infsectionwithext.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), infsectionwithext.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), requiredsize.unwrap_or(core::mem::zeroed()) as _, reserved.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn SetupDiGetActualSectionToInstallA<P1>(infhandle: *const core::ffi::c_void, infsectionname: P1, infsectionwithext: Option<&mut [u8]>, requiredsize: Option<*mut u32>, extension: Option<*mut windows_core::PSTR>) -> windows_core::Result<()>
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupDiGetActualSectionToInstallA(infhandle : *const core::ffi::c_void, infsectionname : windows_core::PCSTR, infsectionwithext : windows_core::PSTR, infsectionwithextsize : u32, requiredsize : *mut u32, extension : *mut windows_core::PSTR) -> windows_core::BOOL);
    unsafe { SetupDiGetActualSectionToInstallA(infhandle, infsectionname.param().abi(), core::mem::transmute(infsectionwithext.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), infsectionwithext.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), requiredsize.unwrap_or(core::mem::zeroed()) as _, extension.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[cfg(all(feature = "Win32_System_Diagnostics_Debug", feature = "Win32_System_SystemInformation"))]
#[inline]
pub unsafe fn SetupDiGetActualSectionToInstallExA<P1>(infhandle: *const core::ffi::c_void, infsectionname: P1, alternateplatforminfo: Option<*const SP_ALTPLATFORM_INFO_V2>, infsectionwithext: Option<&mut [u8]>, requiredsize: Option<*mut u32>, extension: Option<*mut windows_core::PSTR>, reserved: Option<*const core::ffi::c_void>) -> windows_core::Result<()>
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupDiGetActualSectionToInstallExA(infhandle : *const core::ffi::c_void, infsectionname : windows_core::PCSTR, alternateplatforminfo : *const SP_ALTPLATFORM_INFO_V2, infsectionwithext : windows_core::PSTR, infsectionwithextsize : u32, requiredsize : *mut u32, extension : *mut windows_core::PSTR, reserved : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe { SetupDiGetActualSectionToInstallExA(infhandle, infsectionname.param().abi(), alternateplatforminfo.unwrap_or(core::mem::zeroed()) as _, core::mem::transmute(infsectionwithext.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), infsectionwithext.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), requiredsize.unwrap_or(core::mem::zeroed()) as _, extension.unwrap_or(core::mem::zeroed()) as _, reserved.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[cfg(all(feature = "Win32_System_Diagnostics_Debug", feature = "Win32_System_SystemInformation"))]
#[inline]
pub unsafe fn SetupDiGetActualSectionToInstallExW<P1>(infhandle: *const core::ffi::c_void, infsectionname: P1, alternateplatforminfo: Option<*const SP_ALTPLATFORM_INFO_V2>, infsectionwithext: Option<&mut [u16]>, requiredsize: Option<*mut u32>, extension: Option<*mut windows_core::PWSTR>, reserved: Option<*const core::ffi::c_void>) -> windows_core::Result<()>
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupDiGetActualSectionToInstallExW(infhandle : *const core::ffi::c_void, infsectionname : windows_core::PCWSTR, alternateplatforminfo : *const SP_ALTPLATFORM_INFO_V2, infsectionwithext : windows_core::PWSTR, infsectionwithextsize : u32, requiredsize : *mut u32, extension : *mut windows_core::PWSTR, reserved : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe { SetupDiGetActualSectionToInstallExW(infhandle, infsectionname.param().abi(), alternateplatforminfo.unwrap_or(core::mem::zeroed()) as _, core::mem::transmute(infsectionwithext.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), infsectionwithext.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), requiredsize.unwrap_or(core::mem::zeroed()) as _, extension.unwrap_or(core::mem::zeroed()) as _, reserved.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn SetupDiGetActualSectionToInstallW<P1>(infhandle: *const core::ffi::c_void, infsectionname: P1, infsectionwithext: Option<&mut [u16]>, requiredsize: Option<*mut u32>, extension: Option<*mut windows_core::PWSTR>) -> windows_core::Result<()>
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupDiGetActualSectionToInstallW(infhandle : *const core::ffi::c_void, infsectionname : windows_core::PCWSTR, infsectionwithext : windows_core::PWSTR, infsectionwithextsize : u32, requiredsize : *mut u32, extension : *mut windows_core::PWSTR) -> windows_core::BOOL);
    unsafe { SetupDiGetActualSectionToInstallW(infhandle, infsectionname.param().abi(), core::mem::transmute(infsectionwithext.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), infsectionwithext.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), requiredsize.unwrap_or(core::mem::zeroed()) as _, extension.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn SetupDiGetClassBitmapIndex(classguid: Option<*const windows_core::GUID>, miniiconindex: *mut i32) -> windows_core::Result<()> {
    windows_core::link!("setupapi.dll" "system" fn SetupDiGetClassBitmapIndex(classguid : *const windows_core::GUID, miniiconindex : *mut i32) -> windows_core::BOOL);
    unsafe { SetupDiGetClassBitmapIndex(classguid.unwrap_or(core::mem::zeroed()) as _, miniiconindex as _).ok() }
}
#[inline]
pub unsafe fn SetupDiGetClassDescriptionA(classguid: *const windows_core::GUID, classdescription: &mut [u8], requiredsize: Option<*mut u32>) -> windows_core::Result<()> {
    windows_core::link!("setupapi.dll" "system" fn SetupDiGetClassDescriptionA(classguid : *const windows_core::GUID, classdescription : windows_core::PSTR, classdescriptionsize : u32, requiredsize : *mut u32) -> windows_core::BOOL);
    unsafe { SetupDiGetClassDescriptionA(classguid, core::mem::transmute(classdescription.as_ptr()), classdescription.len().try_into().unwrap(), requiredsize.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn SetupDiGetClassDescriptionExA<P4>(classguid: *const windows_core::GUID, classdescription: &mut [u8], requiredsize: Option<*mut u32>, machinename: P4, reserved: Option<*const core::ffi::c_void>) -> windows_core::Result<()>
where
    P4: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupDiGetClassDescriptionExA(classguid : *const windows_core::GUID, classdescription : windows_core::PSTR, classdescriptionsize : u32, requiredsize : *mut u32, machinename : windows_core::PCSTR, reserved : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe { SetupDiGetClassDescriptionExA(classguid, core::mem::transmute(classdescription.as_ptr()), classdescription.len().try_into().unwrap(), requiredsize.unwrap_or(core::mem::zeroed()) as _, machinename.param().abi(), reserved.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn SetupDiGetClassDescriptionExW<P4>(classguid: *const windows_core::GUID, classdescription: &mut [u16], requiredsize: Option<*mut u32>, machinename: P4, reserved: Option<*const core::ffi::c_void>) -> windows_core::Result<()>
where
    P4: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupDiGetClassDescriptionExW(classguid : *const windows_core::GUID, classdescription : windows_core::PWSTR, classdescriptionsize : u32, requiredsize : *mut u32, machinename : windows_core::PCWSTR, reserved : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe { SetupDiGetClassDescriptionExW(classguid, core::mem::transmute(classdescription.as_ptr()), classdescription.len().try_into().unwrap(), requiredsize.unwrap_or(core::mem::zeroed()) as _, machinename.param().abi(), reserved.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn SetupDiGetClassDescriptionW(classguid: *const windows_core::GUID, classdescription: &mut [u16], requiredsize: Option<*mut u32>) -> windows_core::Result<()> {
    windows_core::link!("setupapi.dll" "system" fn SetupDiGetClassDescriptionW(classguid : *const windows_core::GUID, classdescription : windows_core::PWSTR, classdescriptionsize : u32, requiredsize : *mut u32) -> windows_core::BOOL);
    unsafe { SetupDiGetClassDescriptionW(classguid, core::mem::transmute(classdescription.as_ptr()), classdescription.len().try_into().unwrap(), requiredsize.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
#[inline]
pub unsafe fn SetupDiGetClassDevPropertySheetsA(deviceinfoset: HDEVINFO, deviceinfodata: Option<*const SP_DEVINFO_DATA>, propertysheetheader: *const super::super::UI::Controls::PROPSHEETHEADERA_V2, propertysheetheaderpagelistsize: u32, requiredsize: Option<*mut u32>, propertysheettype: u32) -> windows_core::Result<()> {
    windows_core::link!("setupapi.dll" "system" fn SetupDiGetClassDevPropertySheetsA(deviceinfoset : HDEVINFO, deviceinfodata : *const SP_DEVINFO_DATA, propertysheetheader : *const super::super::UI::Controls:: PROPSHEETHEADERA_V2, propertysheetheaderpagelistsize : u32, requiredsize : *mut u32, propertysheettype : u32) -> windows_core::BOOL);
    unsafe { SetupDiGetClassDevPropertySheetsA(deviceinfoset, deviceinfodata.unwrap_or(core::mem::zeroed()) as _, propertysheetheader, propertysheetheaderpagelistsize, requiredsize.unwrap_or(core::mem::zeroed()) as _, propertysheettype).ok() }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
#[inline]
pub unsafe fn SetupDiGetClassDevPropertySheetsW(deviceinfoset: HDEVINFO, deviceinfodata: Option<*const SP_DEVINFO_DATA>, propertysheetheader: *const super::super::UI::Controls::PROPSHEETHEADERW_V2, propertysheetheaderpagelistsize: u32, requiredsize: Option<*mut u32>, propertysheettype: u32) -> windows_core::Result<()> {
    windows_core::link!("setupapi.dll" "system" fn SetupDiGetClassDevPropertySheetsW(deviceinfoset : HDEVINFO, deviceinfodata : *const SP_DEVINFO_DATA, propertysheetheader : *const super::super::UI::Controls:: PROPSHEETHEADERW_V2, propertysheetheaderpagelistsize : u32, requiredsize : *mut u32, propertysheettype : u32) -> windows_core::BOOL);
    unsafe { SetupDiGetClassDevPropertySheetsW(deviceinfoset, deviceinfodata.unwrap_or(core::mem::zeroed()) as _, propertysheetheader, propertysheetheaderpagelistsize, requiredsize.unwrap_or(core::mem::zeroed()) as _, propertysheettype).ok() }
}
#[inline]
pub unsafe fn SetupDiGetClassDevsA<P1>(classguid: Option<*const windows_core::GUID>, enumerator: P1, hwndparent: Option<super::super::Foundation::HWND>, flags: SETUP_DI_GET_CLASS_DEVS_FLAGS) -> windows_core::Result<HDEVINFO>
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupDiGetClassDevsA(classguid : *const windows_core::GUID, enumerator : windows_core::PCSTR, hwndparent : super::super::Foundation:: HWND, flags : SETUP_DI_GET_CLASS_DEVS_FLAGS) -> HDEVINFO);
    let result__ = unsafe { SetupDiGetClassDevsA(classguid.unwrap_or(core::mem::zeroed()) as _, enumerator.param().abi(), hwndparent.unwrap_or(core::mem::zeroed()) as _, flags) };
    (!result__.is_invalid()).then_some(result__).ok_or_else(windows_core::Error::from_thread)
}
#[inline]
pub unsafe fn SetupDiGetClassDevsExA<P1, P5>(classguid: Option<*const windows_core::GUID>, enumerator: P1, hwndparent: Option<super::super::Foundation::HWND>, flags: SETUP_DI_GET_CLASS_DEVS_FLAGS, deviceinfoset: Option<HDEVINFO>, machinename: P5, reserved: Option<*const core::ffi::c_void>) -> windows_core::Result<HDEVINFO>
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P5: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupDiGetClassDevsExA(classguid : *const windows_core::GUID, enumerator : windows_core::PCSTR, hwndparent : super::super::Foundation:: HWND, flags : SETUP_DI_GET_CLASS_DEVS_FLAGS, deviceinfoset : HDEVINFO, machinename : windows_core::PCSTR, reserved : *const core::ffi::c_void) -> HDEVINFO);
    let result__ = unsafe { SetupDiGetClassDevsExA(classguid.unwrap_or(core::mem::zeroed()) as _, enumerator.param().abi(), hwndparent.unwrap_or(core::mem::zeroed()) as _, flags, deviceinfoset.unwrap_or(core::mem::zeroed()) as _, machinename.param().abi(), reserved.unwrap_or(core::mem::zeroed()) as _) };
    (!result__.is_invalid()).then_some(result__).ok_or_else(windows_core::Error::from_thread)
}
#[inline]
pub unsafe fn SetupDiGetClassDevsExW<P1, P5>(classguid: Option<*const windows_core::GUID>, enumerator: P1, hwndparent: Option<super::super::Foundation::HWND>, flags: SETUP_DI_GET_CLASS_DEVS_FLAGS, deviceinfoset: Option<HDEVINFO>, machinename: P5, reserved: Option<*const core::ffi::c_void>) -> windows_core::Result<HDEVINFO>
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P5: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupDiGetClassDevsExW(classguid : *const windows_core::GUID, enumerator : windows_core::PCWSTR, hwndparent : super::super::Foundation:: HWND, flags : SETUP_DI_GET_CLASS_DEVS_FLAGS, deviceinfoset : HDEVINFO, machinename : windows_core::PCWSTR, reserved : *const core::ffi::c_void) -> HDEVINFO);
    let result__ = unsafe { SetupDiGetClassDevsExW(classguid.unwrap_or(core::mem::zeroed()) as _, enumerator.param().abi(), hwndparent.unwrap_or(core::mem::zeroed()) as _, flags, deviceinfoset.unwrap_or(core::mem::zeroed()) as _, machinename.param().abi(), reserved.unwrap_or(core::mem::zeroed()) as _) };
    (!result__.is_invalid()).then_some(result__).ok_or_else(windows_core::Error::from_thread)
}
#[inline]
pub unsafe fn SetupDiGetClassDevsW<P1>(classguid: Option<*const windows_core::GUID>, enumerator: P1, hwndparent: Option<super::super::Foundation::HWND>, flags: SETUP_DI_GET_CLASS_DEVS_FLAGS) -> windows_core::Result<HDEVINFO>
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupDiGetClassDevsW(classguid : *const windows_core::GUID, enumerator : windows_core::PCWSTR, hwndparent : super::super::Foundation:: HWND, flags : SETUP_DI_GET_CLASS_DEVS_FLAGS) -> HDEVINFO);
    let result__ = unsafe { SetupDiGetClassDevsW(classguid.unwrap_or(core::mem::zeroed()) as _, enumerator.param().abi(), hwndparent.unwrap_or(core::mem::zeroed()) as _, flags) };
    (!result__.is_invalid()).then_some(result__).ok_or_else(windows_core::Error::from_thread)
}
#[cfg(feature = "Win32_UI_Controls")]
#[inline]
pub unsafe fn SetupDiGetClassImageIndex(classimagelistdata: *const SP_CLASSIMAGELIST_DATA, classguid: *const windows_core::GUID, imageindex: *mut i32) -> windows_core::Result<()> {
    windows_core::link!("setupapi.dll" "system" fn SetupDiGetClassImageIndex(classimagelistdata : *const SP_CLASSIMAGELIST_DATA, classguid : *const windows_core::GUID, imageindex : *mut i32) -> windows_core::BOOL);
    unsafe { SetupDiGetClassImageIndex(classimagelistdata, classguid, imageindex as _).ok() }
}
#[cfg(feature = "Win32_UI_Controls")]
#[inline]
pub unsafe fn SetupDiGetClassImageList(classimagelistdata: *mut SP_CLASSIMAGELIST_DATA) -> windows_core::Result<()> {
    windows_core::link!("setupapi.dll" "system" fn SetupDiGetClassImageList(classimagelistdata : *mut SP_CLASSIMAGELIST_DATA) -> windows_core::BOOL);
    unsafe { SetupDiGetClassImageList(classimagelistdata as _).ok() }
}
#[cfg(feature = "Win32_UI_Controls")]
#[inline]
pub unsafe fn SetupDiGetClassImageListExA<P1>(classimagelistdata: *mut SP_CLASSIMAGELIST_DATA, machinename: P1, reserved: Option<*const core::ffi::c_void>) -> windows_core::Result<()>
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupDiGetClassImageListExA(classimagelistdata : *mut SP_CLASSIMAGELIST_DATA, machinename : windows_core::PCSTR, reserved : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe { SetupDiGetClassImageListExA(classimagelistdata as _, machinename.param().abi(), reserved.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[cfg(feature = "Win32_UI_Controls")]
#[inline]
pub unsafe fn SetupDiGetClassImageListExW<P1>(classimagelistdata: *mut SP_CLASSIMAGELIST_DATA, machinename: P1, reserved: Option<*const core::ffi::c_void>) -> windows_core::Result<()>
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupDiGetClassImageListExW(classimagelistdata : *mut SP_CLASSIMAGELIST_DATA, machinename : windows_core::PCWSTR, reserved : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe { SetupDiGetClassImageListExW(classimagelistdata as _, machinename.param().abi(), reserved.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn SetupDiGetClassInstallParamsA(deviceinfoset: HDEVINFO, deviceinfodata: Option<*const SP_DEVINFO_DATA>, classinstallparams: Option<*mut SP_CLASSINSTALL_HEADER>, classinstallparamssize: u32, requiredsize: Option<*mut u32>) -> windows_core::Result<()> {
    windows_core::link!("setupapi.dll" "system" fn SetupDiGetClassInstallParamsA(deviceinfoset : HDEVINFO, deviceinfodata : *const SP_DEVINFO_DATA, classinstallparams : *mut SP_CLASSINSTALL_HEADER, classinstallparamssize : u32, requiredsize : *mut u32) -> windows_core::BOOL);
    unsafe { SetupDiGetClassInstallParamsA(deviceinfoset, deviceinfodata.unwrap_or(core::mem::zeroed()) as _, classinstallparams.unwrap_or(core::mem::zeroed()) as _, classinstallparamssize, requiredsize.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn SetupDiGetClassInstallParamsW(deviceinfoset: HDEVINFO, deviceinfodata: Option<*const SP_DEVINFO_DATA>, classinstallparams: Option<*mut SP_CLASSINSTALL_HEADER>, classinstallparamssize: u32, requiredsize: Option<*mut u32>) -> windows_core::Result<()> {
    windows_core::link!("setupapi.dll" "system" fn SetupDiGetClassInstallParamsW(deviceinfoset : HDEVINFO, deviceinfodata : *const SP_DEVINFO_DATA, classinstallparams : *mut SP_CLASSINSTALL_HEADER, classinstallparamssize : u32, requiredsize : *mut u32) -> windows_core::BOOL);
    unsafe { SetupDiGetClassInstallParamsW(deviceinfoset, deviceinfodata.unwrap_or(core::mem::zeroed()) as _, classinstallparams.unwrap_or(core::mem::zeroed()) as _, classinstallparamssize, requiredsize.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[cfg(feature = "Win32_Devices_Properties")]
#[inline]
pub unsafe fn SetupDiGetClassPropertyExW<P7>(classguid: *const windows_core::GUID, propertykey: *const super::super::Foundation::DEVPROPKEY, propertytype: *mut super::Properties::DEVPROPTYPE, propertybuffer: Option<&mut [u8]>, requiredsize: Option<*mut u32>, flags: u32, machinename: P7, reserved: Option<*const core::ffi::c_void>) -> windows_core::Result<()>
where
    P7: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupDiGetClassPropertyExW(classguid : *const windows_core::GUID, propertykey : *const super::super::Foundation:: DEVPROPKEY, propertytype : *mut super::Properties:: DEVPROPTYPE, propertybuffer : *mut u8, propertybuffersize : u32, requiredsize : *mut u32, flags : u32, machinename : windows_core::PCWSTR, reserved : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe { SetupDiGetClassPropertyExW(classguid, propertykey, propertytype as _, core::mem::transmute(propertybuffer.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), propertybuffer.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), requiredsize.unwrap_or(core::mem::zeroed()) as _, flags, machinename.param().abi(), reserved.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn SetupDiGetClassPropertyKeys(classguid: *const windows_core::GUID, propertykeyarray: Option<&mut [super::super::Foundation::DEVPROPKEY]>, requiredpropertykeycount: Option<*mut u32>, flags: u32) -> windows_core::Result<()> {
    windows_core::link!("setupapi.dll" "system" fn SetupDiGetClassPropertyKeys(classguid : *const windows_core::GUID, propertykeyarray : *mut super::super::Foundation:: DEVPROPKEY, propertykeycount : u32, requiredpropertykeycount : *mut u32, flags : u32) -> windows_core::BOOL);
    unsafe { SetupDiGetClassPropertyKeys(classguid, core::mem::transmute(propertykeyarray.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), propertykeyarray.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), requiredpropertykeycount.unwrap_or(core::mem::zeroed()) as _, flags).ok() }
}
#[inline]
pub unsafe fn SetupDiGetClassPropertyKeysExW<P5>(classguid: *const windows_core::GUID, propertykeyarray: Option<&mut [super::super::Foundation::DEVPROPKEY]>, requiredpropertykeycount: Option<*mut u32>, flags: u32, machinename: P5, reserved: Option<*const core::ffi::c_void>) -> windows_core::Result<()>
where
    P5: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupDiGetClassPropertyKeysExW(classguid : *const windows_core::GUID, propertykeyarray : *mut super::super::Foundation:: DEVPROPKEY, propertykeycount : u32, requiredpropertykeycount : *mut u32, flags : u32, machinename : windows_core::PCWSTR, reserved : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe { SetupDiGetClassPropertyKeysExW(classguid, core::mem::transmute(propertykeyarray.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), propertykeyarray.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), requiredpropertykeycount.unwrap_or(core::mem::zeroed()) as _, flags, machinename.param().abi(), reserved.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[cfg(feature = "Win32_Devices_Properties")]
#[inline]
pub unsafe fn SetupDiGetClassPropertyW(classguid: *const windows_core::GUID, propertykey: *const super::super::Foundation::DEVPROPKEY, propertytype: *mut super::Properties::DEVPROPTYPE, propertybuffer: Option<&mut [u8]>, requiredsize: Option<*mut u32>, flags: u32) -> windows_core::Result<()> {
    windows_core::link!("setupapi.dll" "system" fn SetupDiGetClassPropertyW(classguid : *const windows_core::GUID, propertykey : *const super::super::Foundation:: DEVPROPKEY, propertytype : *mut super::Properties:: DEVPROPTYPE, propertybuffer : *mut u8, propertybuffersize : u32, requiredsize : *mut u32, flags : u32) -> windows_core::BOOL);
    unsafe { SetupDiGetClassPropertyW(classguid, propertykey, propertytype as _, core::mem::transmute(propertybuffer.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), propertybuffer.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), requiredsize.unwrap_or(core::mem::zeroed()) as _, flags).ok() }
}
#[inline]
pub unsafe fn SetupDiGetClassRegistryPropertyA<P6>(classguid: *const windows_core::GUID, property: u32, propertyregdatatype: Option<*mut u32>, propertybuffer: &mut [u8], requiredsize: Option<*mut u32>, machinename: P6, reserved: Option<*const core::ffi::c_void>) -> windows_core::Result<()>
where
    P6: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupDiGetClassRegistryPropertyA(classguid : *const windows_core::GUID, property : u32, propertyregdatatype : *mut u32, propertybuffer : *mut u8, propertybuffersize : u32, requiredsize : *mut u32, machinename : windows_core::PCSTR, reserved : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe { SetupDiGetClassRegistryPropertyA(classguid, property, propertyregdatatype.unwrap_or(core::mem::zeroed()) as _, core::mem::transmute(propertybuffer.as_ptr()), propertybuffer.len().try_into().unwrap(), requiredsize.unwrap_or(core::mem::zeroed()) as _, machinename.param().abi(), reserved.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn SetupDiGetClassRegistryPropertyW<P6>(classguid: *const windows_core::GUID, property: u32, propertyregdatatype: Option<*mut u32>, propertybuffer: &mut [u8], requiredsize: Option<*mut u32>, machinename: P6, reserved: Option<*const core::ffi::c_void>) -> windows_core::Result<()>
where
    P6: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupDiGetClassRegistryPropertyW(classguid : *const windows_core::GUID, property : u32, propertyregdatatype : *mut u32, propertybuffer : *mut u8, propertybuffersize : u32, requiredsize : *mut u32, machinename : windows_core::PCWSTR, reserved : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe { SetupDiGetClassRegistryPropertyW(classguid, property, propertyregdatatype.unwrap_or(core::mem::zeroed()) as _, core::mem::transmute(propertybuffer.as_ptr()), propertybuffer.len().try_into().unwrap(), requiredsize.unwrap_or(core::mem::zeroed()) as _, machinename.param().abi(), reserved.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn SetupDiGetCustomDevicePropertyA<P2>(deviceinfoset: HDEVINFO, deviceinfodata: *const SP_DEVINFO_DATA, custompropertyname: P2, flags: u32, propertyregdatatype: Option<*mut u32>, propertybuffer: &mut [u8], requiredsize: Option<*mut u32>) -> windows_core::Result<()>
where
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupDiGetCustomDevicePropertyA(deviceinfoset : HDEVINFO, deviceinfodata : *const SP_DEVINFO_DATA, custompropertyname : windows_core::PCSTR, flags : u32, propertyregdatatype : *mut u32, propertybuffer : *mut u8, propertybuffersize : u32, requiredsize : *mut u32) -> windows_core::BOOL);
    unsafe { SetupDiGetCustomDevicePropertyA(deviceinfoset, deviceinfodata, custompropertyname.param().abi(), flags, propertyregdatatype.unwrap_or(core::mem::zeroed()) as _, core::mem::transmute(propertybuffer.as_ptr()), propertybuffer.len().try_into().unwrap(), requiredsize.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn SetupDiGetCustomDevicePropertyW<P2>(deviceinfoset: HDEVINFO, deviceinfodata: *const SP_DEVINFO_DATA, custompropertyname: P2, flags: u32, propertyregdatatype: Option<*mut u32>, propertybuffer: &mut [u8], requiredsize: Option<*mut u32>) -> windows_core::Result<()>
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupDiGetCustomDevicePropertyW(deviceinfoset : HDEVINFO, deviceinfodata : *const SP_DEVINFO_DATA, custompropertyname : windows_core::PCWSTR, flags : u32, propertyregdatatype : *mut u32, propertybuffer : *mut u8, propertybuffersize : u32, requiredsize : *mut u32) -> windows_core::BOOL);
    unsafe { SetupDiGetCustomDevicePropertyW(deviceinfoset, deviceinfodata, custompropertyname.param().abi(), flags, propertyregdatatype.unwrap_or(core::mem::zeroed()) as _, core::mem::transmute(propertybuffer.as_ptr()), propertybuffer.len().try_into().unwrap(), requiredsize.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn SetupDiGetDeviceInfoListClass(deviceinfoset: HDEVINFO, classguid: *mut windows_core::GUID) -> windows_core::Result<()> {
    windows_core::link!("setupapi.dll" "system" fn SetupDiGetDeviceInfoListClass(deviceinfoset : HDEVINFO, classguid : *mut windows_core::GUID) -> windows_core::BOOL);
    unsafe { SetupDiGetDeviceInfoListClass(deviceinfoset, classguid as _).ok() }
}
#[inline]
pub unsafe fn SetupDiGetDeviceInfoListDetailA(deviceinfoset: HDEVINFO, deviceinfosetdetaildata: *mut SP_DEVINFO_LIST_DETAIL_DATA_A) -> windows_core::Result<()> {
    windows_core::link!("setupapi.dll" "system" fn SetupDiGetDeviceInfoListDetailA(deviceinfoset : HDEVINFO, deviceinfosetdetaildata : *mut SP_DEVINFO_LIST_DETAIL_DATA_A) -> windows_core::BOOL);
    unsafe { SetupDiGetDeviceInfoListDetailA(deviceinfoset, deviceinfosetdetaildata as _).ok() }
}
#[inline]
pub unsafe fn SetupDiGetDeviceInfoListDetailW(deviceinfoset: HDEVINFO, deviceinfosetdetaildata: *mut SP_DEVINFO_LIST_DETAIL_DATA_W) -> windows_core::Result<()> {
    windows_core::link!("setupapi.dll" "system" fn SetupDiGetDeviceInfoListDetailW(deviceinfoset : HDEVINFO, deviceinfosetdetaildata : *mut SP_DEVINFO_LIST_DETAIL_DATA_W) -> windows_core::BOOL);
    unsafe { SetupDiGetDeviceInfoListDetailW(deviceinfoset, deviceinfosetdetaildata as _).ok() }
}
#[inline]
pub unsafe fn SetupDiGetDeviceInstallParamsA(deviceinfoset: HDEVINFO, deviceinfodata: Option<*const SP_DEVINFO_DATA>, deviceinstallparams: *mut SP_DEVINSTALL_PARAMS_A) -> windows_core::Result<()> {
    windows_core::link!("setupapi.dll" "system" fn SetupDiGetDeviceInstallParamsA(deviceinfoset : HDEVINFO, deviceinfodata : *const SP_DEVINFO_DATA, deviceinstallparams : *mut SP_DEVINSTALL_PARAMS_A) -> windows_core::BOOL);
    unsafe { SetupDiGetDeviceInstallParamsA(deviceinfoset, deviceinfodata.unwrap_or(core::mem::zeroed()) as _, deviceinstallparams as _).ok() }
}
#[inline]
pub unsafe fn SetupDiGetDeviceInstallParamsW(deviceinfoset: HDEVINFO, deviceinfodata: Option<*const SP_DEVINFO_DATA>, deviceinstallparams: *mut SP_DEVINSTALL_PARAMS_W) -> windows_core::Result<()> {
    windows_core::link!("setupapi.dll" "system" fn SetupDiGetDeviceInstallParamsW(deviceinfoset : HDEVINFO, deviceinfodata : *const SP_DEVINFO_DATA, deviceinstallparams : *mut SP_DEVINSTALL_PARAMS_W) -> windows_core::BOOL);
    unsafe { SetupDiGetDeviceInstallParamsW(deviceinfoset, deviceinfodata.unwrap_or(core::mem::zeroed()) as _, deviceinstallparams as _).ok() }
}
#[inline]
pub unsafe fn SetupDiGetDeviceInstanceIdA(deviceinfoset: HDEVINFO, deviceinfodata: *const SP_DEVINFO_DATA, deviceinstanceid: Option<&mut [u8]>, requiredsize: Option<*mut u32>) -> windows_core::Result<()> {
    windows_core::link!("setupapi.dll" "system" fn SetupDiGetDeviceInstanceIdA(deviceinfoset : HDEVINFO, deviceinfodata : *const SP_DEVINFO_DATA, deviceinstanceid : windows_core::PSTR, deviceinstanceidsize : u32, requiredsize : *mut u32) -> windows_core::BOOL);
    unsafe { SetupDiGetDeviceInstanceIdA(deviceinfoset, deviceinfodata, core::mem::transmute(deviceinstanceid.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), deviceinstanceid.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), requiredsize.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn SetupDiGetDeviceInstanceIdW(deviceinfoset: HDEVINFO, deviceinfodata: *const SP_DEVINFO_DATA, deviceinstanceid: Option<&mut [u16]>, requiredsize: Option<*mut u32>) -> windows_core::Result<()> {
    windows_core::link!("setupapi.dll" "system" fn SetupDiGetDeviceInstanceIdW(deviceinfoset : HDEVINFO, deviceinfodata : *const SP_DEVINFO_DATA, deviceinstanceid : windows_core::PWSTR, deviceinstanceidsize : u32, requiredsize : *mut u32) -> windows_core::BOOL);
    unsafe { SetupDiGetDeviceInstanceIdW(deviceinfoset, deviceinfodata, core::mem::transmute(deviceinstanceid.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), deviceinstanceid.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), requiredsize.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn SetupDiGetDeviceInterfaceAlias(deviceinfoset: HDEVINFO, deviceinterfacedata: *const SP_DEVICE_INTERFACE_DATA, aliasinterfaceclassguid: *const windows_core::GUID, aliasdeviceinterfacedata: *mut SP_DEVICE_INTERFACE_DATA) -> windows_core::Result<()> {
    windows_core::link!("setupapi.dll" "system" fn SetupDiGetDeviceInterfaceAlias(deviceinfoset : HDEVINFO, deviceinterfacedata : *const SP_DEVICE_INTERFACE_DATA, aliasinterfaceclassguid : *const windows_core::GUID, aliasdeviceinterfacedata : *mut SP_DEVICE_INTERFACE_DATA) -> windows_core::BOOL);
    unsafe { SetupDiGetDeviceInterfaceAlias(deviceinfoset, deviceinterfacedata, aliasinterfaceclassguid, aliasdeviceinterfacedata as _).ok() }
}
#[inline]
pub unsafe fn SetupDiGetDeviceInterfaceDetailA(deviceinfoset: HDEVINFO, deviceinterfacedata: *const SP_DEVICE_INTERFACE_DATA, deviceinterfacedetaildata: Option<*mut SP_DEVICE_INTERFACE_DETAIL_DATA_A>, deviceinterfacedetaildatasize: u32, requiredsize: Option<*mut u32>, deviceinfodata: Option<*mut SP_DEVINFO_DATA>) -> windows_core::Result<()> {
    windows_core::link!("setupapi.dll" "system" fn SetupDiGetDeviceInterfaceDetailA(deviceinfoset : HDEVINFO, deviceinterfacedata : *const SP_DEVICE_INTERFACE_DATA, deviceinterfacedetaildata : *mut SP_DEVICE_INTERFACE_DETAIL_DATA_A, deviceinterfacedetaildatasize : u32, requiredsize : *mut u32, deviceinfodata : *mut SP_DEVINFO_DATA) -> windows_core::BOOL);
    unsafe { SetupDiGetDeviceInterfaceDetailA(deviceinfoset, deviceinterfacedata, deviceinterfacedetaildata.unwrap_or(core::mem::zeroed()) as _, deviceinterfacedetaildatasize, requiredsize.unwrap_or(core::mem::zeroed()) as _, deviceinfodata.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn SetupDiGetDeviceInterfaceDetailW(deviceinfoset: HDEVINFO, deviceinterfacedata: *const SP_DEVICE_INTERFACE_DATA, deviceinterfacedetaildata: Option<*mut SP_DEVICE_INTERFACE_DETAIL_DATA_W>, deviceinterfacedetaildatasize: u32, requiredsize: Option<*mut u32>, deviceinfodata: Option<*mut SP_DEVINFO_DATA>) -> windows_core::Result<()> {
    windows_core::link!("setupapi.dll" "system" fn SetupDiGetDeviceInterfaceDetailW(deviceinfoset : HDEVINFO, deviceinterfacedata : *const SP_DEVICE_INTERFACE_DATA, deviceinterfacedetaildata : *mut SP_DEVICE_INTERFACE_DETAIL_DATA_W, deviceinterfacedetaildatasize : u32, requiredsize : *mut u32, deviceinfodata : *mut SP_DEVINFO_DATA) -> windows_core::BOOL);
    unsafe { SetupDiGetDeviceInterfaceDetailW(deviceinfoset, deviceinterfacedata, deviceinterfacedetaildata.unwrap_or(core::mem::zeroed()) as _, deviceinterfacedetaildatasize, requiredsize.unwrap_or(core::mem::zeroed()) as _, deviceinfodata.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn SetupDiGetDeviceInterfacePropertyKeys(deviceinfoset: HDEVINFO, deviceinterfacedata: *const SP_DEVICE_INTERFACE_DATA, propertykeyarray: Option<&mut [super::super::Foundation::DEVPROPKEY]>, requiredpropertykeycount: Option<*mut u32>, flags: u32) -> windows_core::Result<()> {
    windows_core::link!("setupapi.dll" "system" fn SetupDiGetDeviceInterfacePropertyKeys(deviceinfoset : HDEVINFO, deviceinterfacedata : *const SP_DEVICE_INTERFACE_DATA, propertykeyarray : *mut super::super::Foundation:: DEVPROPKEY, propertykeycount : u32, requiredpropertykeycount : *mut u32, flags : u32) -> windows_core::BOOL);
    unsafe { SetupDiGetDeviceInterfacePropertyKeys(deviceinfoset, deviceinterfacedata, core::mem::transmute(propertykeyarray.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), propertykeyarray.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), requiredpropertykeycount.unwrap_or(core::mem::zeroed()) as _, flags).ok() }
}
#[cfg(feature = "Win32_Devices_Properties")]
#[inline]
pub unsafe fn SetupDiGetDeviceInterfacePropertyW(deviceinfoset: HDEVINFO, deviceinterfacedata: *const SP_DEVICE_INTERFACE_DATA, propertykey: *const super::super::Foundation::DEVPROPKEY, propertytype: *mut super::Properties::DEVPROPTYPE, propertybuffer: Option<&mut [u8]>, requiredsize: Option<*mut u32>, flags: u32) -> windows_core::Result<()> {
    windows_core::link!("setupapi.dll" "system" fn SetupDiGetDeviceInterfacePropertyW(deviceinfoset : HDEVINFO, deviceinterfacedata : *const SP_DEVICE_INTERFACE_DATA, propertykey : *const super::super::Foundation:: DEVPROPKEY, propertytype : *mut super::Properties:: DEVPROPTYPE, propertybuffer : *mut u8, propertybuffersize : u32, requiredsize : *mut u32, flags : u32) -> windows_core::BOOL);
    unsafe { SetupDiGetDeviceInterfacePropertyW(deviceinfoset, deviceinterfacedata, propertykey, propertytype as _, core::mem::transmute(propertybuffer.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), propertybuffer.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), requiredsize.unwrap_or(core::mem::zeroed()) as _, flags).ok() }
}
#[inline]
pub unsafe fn SetupDiGetDevicePropertyKeys(deviceinfoset: HDEVINFO, deviceinfodata: *const SP_DEVINFO_DATA, propertykeyarray: Option<&mut [super::super::Foundation::DEVPROPKEY]>, requiredpropertykeycount: Option<*mut u32>, flags: u32) -> windows_core::Result<()> {
    windows_core::link!("setupapi.dll" "system" fn SetupDiGetDevicePropertyKeys(deviceinfoset : HDEVINFO, deviceinfodata : *const SP_DEVINFO_DATA, propertykeyarray : *mut super::super::Foundation:: DEVPROPKEY, propertykeycount : u32, requiredpropertykeycount : *mut u32, flags : u32) -> windows_core::BOOL);
    unsafe { SetupDiGetDevicePropertyKeys(deviceinfoset, deviceinfodata, core::mem::transmute(propertykeyarray.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), propertykeyarray.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), requiredpropertykeycount.unwrap_or(core::mem::zeroed()) as _, flags).ok() }
}
#[cfg(feature = "Win32_Devices_Properties")]
#[inline]
pub unsafe fn SetupDiGetDevicePropertyW(deviceinfoset: HDEVINFO, deviceinfodata: *const SP_DEVINFO_DATA, propertykey: *const super::super::Foundation::DEVPROPKEY, propertytype: *mut super::Properties::DEVPROPTYPE, propertybuffer: Option<&mut [u8]>, requiredsize: Option<*mut u32>, flags: u32) -> windows_core::Result<()> {
    windows_core::link!("setupapi.dll" "system" fn SetupDiGetDevicePropertyW(deviceinfoset : HDEVINFO, deviceinfodata : *const SP_DEVINFO_DATA, propertykey : *const super::super::Foundation:: DEVPROPKEY, propertytype : *mut super::Properties:: DEVPROPTYPE, propertybuffer : *mut u8, propertybuffersize : u32, requiredsize : *mut u32, flags : u32) -> windows_core::BOOL);
    unsafe { SetupDiGetDevicePropertyW(deviceinfoset, deviceinfodata, propertykey, propertytype as _, core::mem::transmute(propertybuffer.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), propertybuffer.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), requiredsize.unwrap_or(core::mem::zeroed()) as _, flags).ok() }
}
#[inline]
pub unsafe fn SetupDiGetDeviceRegistryPropertyA(deviceinfoset: HDEVINFO, deviceinfodata: *const SP_DEVINFO_DATA, property: SETUP_DI_REGISTRY_PROPERTY, propertyregdatatype: Option<*mut u32>, propertybuffer: Option<&mut [u8]>, requiredsize: Option<*mut u32>) -> windows_core::Result<()> {
    windows_core::link!("setupapi.dll" "system" fn SetupDiGetDeviceRegistryPropertyA(deviceinfoset : HDEVINFO, deviceinfodata : *const SP_DEVINFO_DATA, property : SETUP_DI_REGISTRY_PROPERTY, propertyregdatatype : *mut u32, propertybuffer : *mut u8, propertybuffersize : u32, requiredsize : *mut u32) -> windows_core::BOOL);
    unsafe { SetupDiGetDeviceRegistryPropertyA(deviceinfoset, deviceinfodata, property, propertyregdatatype.unwrap_or(core::mem::zeroed()) as _, core::mem::transmute(propertybuffer.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), propertybuffer.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), requiredsize.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn SetupDiGetDeviceRegistryPropertyW(deviceinfoset: HDEVINFO, deviceinfodata: *const SP_DEVINFO_DATA, property: SETUP_DI_REGISTRY_PROPERTY, propertyregdatatype: Option<*mut u32>, propertybuffer: Option<&mut [u8]>, requiredsize: Option<*mut u32>) -> windows_core::Result<()> {
    windows_core::link!("setupapi.dll" "system" fn SetupDiGetDeviceRegistryPropertyW(deviceinfoset : HDEVINFO, deviceinfodata : *const SP_DEVINFO_DATA, property : SETUP_DI_REGISTRY_PROPERTY, propertyregdatatype : *mut u32, propertybuffer : *mut u8, propertybuffersize : u32, requiredsize : *mut u32) -> windows_core::BOOL);
    unsafe { SetupDiGetDeviceRegistryPropertyW(deviceinfoset, deviceinfodata, property, propertyregdatatype.unwrap_or(core::mem::zeroed()) as _, core::mem::transmute(propertybuffer.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), propertybuffer.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), requiredsize.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn SetupDiGetDriverInfoDetailA(deviceinfoset: HDEVINFO, deviceinfodata: Option<*const SP_DEVINFO_DATA>, driverinfodata: *const SP_DRVINFO_DATA_V2_A, driverinfodetaildata: Option<*mut SP_DRVINFO_DETAIL_DATA_A>, driverinfodetaildatasize: u32, requiredsize: Option<*mut u32>) -> windows_core::Result<()> {
    windows_core::link!("setupapi.dll" "system" fn SetupDiGetDriverInfoDetailA(deviceinfoset : HDEVINFO, deviceinfodata : *const SP_DEVINFO_DATA, driverinfodata : *const SP_DRVINFO_DATA_V2_A, driverinfodetaildata : *mut SP_DRVINFO_DETAIL_DATA_A, driverinfodetaildatasize : u32, requiredsize : *mut u32) -> windows_core::BOOL);
    unsafe { SetupDiGetDriverInfoDetailA(deviceinfoset, deviceinfodata.unwrap_or(core::mem::zeroed()) as _, driverinfodata, driverinfodetaildata.unwrap_or(core::mem::zeroed()) as _, driverinfodetaildatasize, requiredsize.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn SetupDiGetDriverInfoDetailW(deviceinfoset: HDEVINFO, deviceinfodata: Option<*const SP_DEVINFO_DATA>, driverinfodata: *const SP_DRVINFO_DATA_V2_W, driverinfodetaildata: Option<*mut SP_DRVINFO_DETAIL_DATA_W>, driverinfodetaildatasize: u32, requiredsize: Option<*mut u32>) -> windows_core::Result<()> {
    windows_core::link!("setupapi.dll" "system" fn SetupDiGetDriverInfoDetailW(deviceinfoset : HDEVINFO, deviceinfodata : *const SP_DEVINFO_DATA, driverinfodata : *const SP_DRVINFO_DATA_V2_W, driverinfodetaildata : *mut SP_DRVINFO_DETAIL_DATA_W, driverinfodetaildatasize : u32, requiredsize : *mut u32) -> windows_core::BOOL);
    unsafe { SetupDiGetDriverInfoDetailW(deviceinfoset, deviceinfodata.unwrap_or(core::mem::zeroed()) as _, driverinfodata, driverinfodetaildata.unwrap_or(core::mem::zeroed()) as _, driverinfodetaildatasize, requiredsize.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn SetupDiGetDriverInstallParamsA(deviceinfoset: HDEVINFO, deviceinfodata: Option<*const SP_DEVINFO_DATA>, driverinfodata: *const SP_DRVINFO_DATA_V2_A, driverinstallparams: *mut SP_DRVINSTALL_PARAMS) -> windows_core::Result<()> {
    windows_core::link!("setupapi.dll" "system" fn SetupDiGetDriverInstallParamsA(deviceinfoset : HDEVINFO, deviceinfodata : *const SP_DEVINFO_DATA, driverinfodata : *const SP_DRVINFO_DATA_V2_A, driverinstallparams : *mut SP_DRVINSTALL_PARAMS) -> windows_core::BOOL);
    unsafe { SetupDiGetDriverInstallParamsA(deviceinfoset, deviceinfodata.unwrap_or(core::mem::zeroed()) as _, driverinfodata, driverinstallparams as _).ok() }
}
#[inline]
pub unsafe fn SetupDiGetDriverInstallParamsW(deviceinfoset: HDEVINFO, deviceinfodata: Option<*const SP_DEVINFO_DATA>, driverinfodata: *const SP_DRVINFO_DATA_V2_W, driverinstallparams: *mut SP_DRVINSTALL_PARAMS) -> windows_core::Result<()> {
    windows_core::link!("setupapi.dll" "system" fn SetupDiGetDriverInstallParamsW(deviceinfoset : HDEVINFO, deviceinfodata : *const SP_DEVINFO_DATA, driverinfodata : *const SP_DRVINFO_DATA_V2_W, driverinstallparams : *mut SP_DRVINSTALL_PARAMS) -> windows_core::BOOL);
    unsafe { SetupDiGetDriverInstallParamsW(deviceinfoset, deviceinfodata.unwrap_or(core::mem::zeroed()) as _, driverinfodata, driverinstallparams as _).ok() }
}
#[inline]
pub unsafe fn SetupDiGetHwProfileFriendlyNameA(hwprofile: u32, friendlyname: &mut [u8], requiredsize: Option<*mut u32>) -> windows_core::Result<()> {
    windows_core::link!("setupapi.dll" "system" fn SetupDiGetHwProfileFriendlyNameA(hwprofile : u32, friendlyname : windows_core::PSTR, friendlynamesize : u32, requiredsize : *mut u32) -> windows_core::BOOL);
    unsafe { SetupDiGetHwProfileFriendlyNameA(hwprofile, core::mem::transmute(friendlyname.as_ptr()), friendlyname.len().try_into().unwrap(), requiredsize.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn SetupDiGetHwProfileFriendlyNameExA<P4>(hwprofile: u32, friendlyname: &mut [u8], requiredsize: Option<*mut u32>, machinename: P4, reserved: Option<*const core::ffi::c_void>) -> windows_core::Result<()>
where
    P4: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupDiGetHwProfileFriendlyNameExA(hwprofile : u32, friendlyname : windows_core::PSTR, friendlynamesize : u32, requiredsize : *mut u32, machinename : windows_core::PCSTR, reserved : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe { SetupDiGetHwProfileFriendlyNameExA(hwprofile, core::mem::transmute(friendlyname.as_ptr()), friendlyname.len().try_into().unwrap(), requiredsize.unwrap_or(core::mem::zeroed()) as _, machinename.param().abi(), reserved.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn SetupDiGetHwProfileFriendlyNameExW<P4>(hwprofile: u32, friendlyname: &mut [u16], requiredsize: Option<*mut u32>, machinename: P4, reserved: Option<*const core::ffi::c_void>) -> windows_core::Result<()>
where
    P4: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupDiGetHwProfileFriendlyNameExW(hwprofile : u32, friendlyname : windows_core::PWSTR, friendlynamesize : u32, requiredsize : *mut u32, machinename : windows_core::PCWSTR, reserved : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe { SetupDiGetHwProfileFriendlyNameExW(hwprofile, core::mem::transmute(friendlyname.as_ptr()), friendlyname.len().try_into().unwrap(), requiredsize.unwrap_or(core::mem::zeroed()) as _, machinename.param().abi(), reserved.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn SetupDiGetHwProfileFriendlyNameW(hwprofile: u32, friendlyname: &mut [u16], requiredsize: Option<*mut u32>) -> windows_core::Result<()> {
    windows_core::link!("setupapi.dll" "system" fn SetupDiGetHwProfileFriendlyNameW(hwprofile : u32, friendlyname : windows_core::PWSTR, friendlynamesize : u32, requiredsize : *mut u32) -> windows_core::BOOL);
    unsafe { SetupDiGetHwProfileFriendlyNameW(hwprofile, core::mem::transmute(friendlyname.as_ptr()), friendlyname.len().try_into().unwrap(), requiredsize.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn SetupDiGetHwProfileList(hwprofilelist: &mut [u32], requiredsize: *mut u32, currentlyactiveindex: Option<*mut u32>) -> windows_core::Result<()> {
    windows_core::link!("setupapi.dll" "system" fn SetupDiGetHwProfileList(hwprofilelist : *mut u32, hwprofilelistsize : u32, requiredsize : *mut u32, currentlyactiveindex : *mut u32) -> windows_core::BOOL);
    unsafe { SetupDiGetHwProfileList(core::mem::transmute(hwprofilelist.as_ptr()), hwprofilelist.len().try_into().unwrap(), requiredsize as _, currentlyactiveindex.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn SetupDiGetHwProfileListExA<P4>(hwprofilelist: &mut [u32], requiredsize: *mut u32, currentlyactiveindex: Option<*mut u32>, machinename: P4, reserved: Option<*const core::ffi::c_void>) -> windows_core::Result<()>
where
    P4: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupDiGetHwProfileListExA(hwprofilelist : *mut u32, hwprofilelistsize : u32, requiredsize : *mut u32, currentlyactiveindex : *mut u32, machinename : windows_core::PCSTR, reserved : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe { SetupDiGetHwProfileListExA(core::mem::transmute(hwprofilelist.as_ptr()), hwprofilelist.len().try_into().unwrap(), requiredsize as _, currentlyactiveindex.unwrap_or(core::mem::zeroed()) as _, machinename.param().abi(), reserved.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn SetupDiGetHwProfileListExW<P4>(hwprofilelist: &mut [u32], requiredsize: *mut u32, currentlyactiveindex: Option<*mut u32>, machinename: P4, reserved: Option<*const core::ffi::c_void>) -> windows_core::Result<()>
where
    P4: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupDiGetHwProfileListExW(hwprofilelist : *mut u32, hwprofilelistsize : u32, requiredsize : *mut u32, currentlyactiveindex : *mut u32, machinename : windows_core::PCWSTR, reserved : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe { SetupDiGetHwProfileListExW(core::mem::transmute(hwprofilelist.as_ptr()), hwprofilelist.len().try_into().unwrap(), requiredsize as _, currentlyactiveindex.unwrap_or(core::mem::zeroed()) as _, machinename.param().abi(), reserved.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn SetupDiGetINFClassA<P0>(infname: P0, classguid: *mut windows_core::GUID, classname: &mut [u8], requiredsize: Option<*mut u32>) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupDiGetINFClassA(infname : windows_core::PCSTR, classguid : *mut windows_core::GUID, classname : windows_core::PSTR, classnamesize : u32, requiredsize : *mut u32) -> windows_core::BOOL);
    unsafe { SetupDiGetINFClassA(infname.param().abi(), classguid as _, core::mem::transmute(classname.as_ptr()), classname.len().try_into().unwrap(), requiredsize.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn SetupDiGetINFClassW<P0>(infname: P0, classguid: *mut windows_core::GUID, classname: &mut [u16], requiredsize: Option<*mut u32>) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupDiGetINFClassW(infname : windows_core::PCWSTR, classguid : *mut windows_core::GUID, classname : windows_core::PWSTR, classnamesize : u32, requiredsize : *mut u32) -> windows_core::BOOL);
    unsafe { SetupDiGetINFClassW(infname.param().abi(), classguid as _, core::mem::transmute(classname.as_ptr()), classname.len().try_into().unwrap(), requiredsize.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn SetupDiGetSelectedDevice(deviceinfoset: HDEVINFO, deviceinfodata: *mut SP_DEVINFO_DATA) -> windows_core::Result<()> {
    windows_core::link!("setupapi.dll" "system" fn SetupDiGetSelectedDevice(deviceinfoset : HDEVINFO, deviceinfodata : *mut SP_DEVINFO_DATA) -> windows_core::BOOL);
    unsafe { SetupDiGetSelectedDevice(deviceinfoset, deviceinfodata as _).ok() }
}
#[inline]
pub unsafe fn SetupDiGetSelectedDriverA(deviceinfoset: HDEVINFO, deviceinfodata: Option<*const SP_DEVINFO_DATA>, driverinfodata: *mut SP_DRVINFO_DATA_V2_A) -> windows_core::Result<()> {
    windows_core::link!("setupapi.dll" "system" fn SetupDiGetSelectedDriverA(deviceinfoset : HDEVINFO, deviceinfodata : *const SP_DEVINFO_DATA, driverinfodata : *mut SP_DRVINFO_DATA_V2_A) -> windows_core::BOOL);
    unsafe { SetupDiGetSelectedDriverA(deviceinfoset, deviceinfodata.unwrap_or(core::mem::zeroed()) as _, driverinfodata as _).ok() }
}
#[inline]
pub unsafe fn SetupDiGetSelectedDriverW(deviceinfoset: HDEVINFO, deviceinfodata: Option<*const SP_DEVINFO_DATA>, driverinfodata: *mut SP_DRVINFO_DATA_V2_W) -> windows_core::Result<()> {
    windows_core::link!("setupapi.dll" "system" fn SetupDiGetSelectedDriverW(deviceinfoset : HDEVINFO, deviceinfodata : *const SP_DEVINFO_DATA, driverinfodata : *mut SP_DRVINFO_DATA_V2_W) -> windows_core::BOOL);
    unsafe { SetupDiGetSelectedDriverW(deviceinfoset, deviceinfodata.unwrap_or(core::mem::zeroed()) as _, driverinfodata as _).ok() }
}
#[cfg(feature = "Win32_UI_Controls")]
#[inline]
pub unsafe fn SetupDiGetWizardPage(deviceinfoset: HDEVINFO, deviceinfodata: Option<*const SP_DEVINFO_DATA>, installwizarddata: *const SP_INSTALLWIZARD_DATA, pagetype: u32, flags: u32) -> super::super::UI::Controls::HPROPSHEETPAGE {
    windows_core::link!("setupapi.dll" "system" fn SetupDiGetWizardPage(deviceinfoset : HDEVINFO, deviceinfodata : *const SP_DEVINFO_DATA, installwizarddata : *const SP_INSTALLWIZARD_DATA, pagetype : u32, flags : u32) -> super::super::UI::Controls:: HPROPSHEETPAGE);
    unsafe { SetupDiGetWizardPage(deviceinfoset, deviceinfodata.unwrap_or(core::mem::zeroed()) as _, installwizarddata, pagetype, flags) }
}
#[inline]
pub unsafe fn SetupDiInstallClassA<P1>(hwndparent: Option<super::super::Foundation::HWND>, inffilename: P1, flags: u32, filequeue: Option<*const core::ffi::c_void>) -> windows_core::Result<()>
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupDiInstallClassA(hwndparent : super::super::Foundation:: HWND, inffilename : windows_core::PCSTR, flags : u32, filequeue : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe { SetupDiInstallClassA(hwndparent.unwrap_or(core::mem::zeroed()) as _, inffilename.param().abi(), flags, filequeue.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn SetupDiInstallClassExA<P1>(hwndparent: Option<super::super::Foundation::HWND>, inffilename: P1, flags: u32, filequeue: Option<*const core::ffi::c_void>, interfaceclassguid: Option<*const windows_core::GUID>, reserved1: Option<*const core::ffi::c_void>, reserved2: Option<*const core::ffi::c_void>) -> windows_core::Result<()>
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupDiInstallClassExA(hwndparent : super::super::Foundation:: HWND, inffilename : windows_core::PCSTR, flags : u32, filequeue : *const core::ffi::c_void, interfaceclassguid : *const windows_core::GUID, reserved1 : *const core::ffi::c_void, reserved2 : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe { SetupDiInstallClassExA(hwndparent.unwrap_or(core::mem::zeroed()) as _, inffilename.param().abi(), flags, filequeue.unwrap_or(core::mem::zeroed()) as _, interfaceclassguid.unwrap_or(core::mem::zeroed()) as _, reserved1.unwrap_or(core::mem::zeroed()) as _, reserved2.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn SetupDiInstallClassExW<P1>(hwndparent: Option<super::super::Foundation::HWND>, inffilename: P1, flags: u32, filequeue: Option<*const core::ffi::c_void>, interfaceclassguid: Option<*const windows_core::GUID>, reserved1: Option<*const core::ffi::c_void>, reserved2: Option<*const core::ffi::c_void>) -> windows_core::Result<()>
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupDiInstallClassExW(hwndparent : super::super::Foundation:: HWND, inffilename : windows_core::PCWSTR, flags : u32, filequeue : *const core::ffi::c_void, interfaceclassguid : *const windows_core::GUID, reserved1 : *const core::ffi::c_void, reserved2 : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe { SetupDiInstallClassExW(hwndparent.unwrap_or(core::mem::zeroed()) as _, inffilename.param().abi(), flags, filequeue.unwrap_or(core::mem::zeroed()) as _, interfaceclassguid.unwrap_or(core::mem::zeroed()) as _, reserved1.unwrap_or(core::mem::zeroed()) as _, reserved2.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn SetupDiInstallClassW<P1>(hwndparent: Option<super::super::Foundation::HWND>, inffilename: P1, flags: u32, filequeue: Option<*const core::ffi::c_void>) -> windows_core::Result<()>
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupDiInstallClassW(hwndparent : super::super::Foundation:: HWND, inffilename : windows_core::PCWSTR, flags : u32, filequeue : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe { SetupDiInstallClassW(hwndparent.unwrap_or(core::mem::zeroed()) as _, inffilename.param().abi(), flags, filequeue.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn SetupDiInstallDevice(deviceinfoset: HDEVINFO, deviceinfodata: *mut SP_DEVINFO_DATA) -> windows_core::Result<()> {
    windows_core::link!("setupapi.dll" "system" fn SetupDiInstallDevice(deviceinfoset : HDEVINFO, deviceinfodata : *mut SP_DEVINFO_DATA) -> windows_core::BOOL);
    unsafe { SetupDiInstallDevice(deviceinfoset, deviceinfodata as _).ok() }
}
#[inline]
pub unsafe fn SetupDiInstallDeviceInterfaces(deviceinfoset: HDEVINFO, deviceinfodata: *const SP_DEVINFO_DATA) -> windows_core::Result<()> {
    windows_core::link!("setupapi.dll" "system" fn SetupDiInstallDeviceInterfaces(deviceinfoset : HDEVINFO, deviceinfodata : *const SP_DEVINFO_DATA) -> windows_core::BOOL);
    unsafe { SetupDiInstallDeviceInterfaces(deviceinfoset, deviceinfodata).ok() }
}
#[inline]
pub unsafe fn SetupDiInstallDriverFiles(deviceinfoset: HDEVINFO, deviceinfodata: *const SP_DEVINFO_DATA) -> windows_core::Result<()> {
    windows_core::link!("setupapi.dll" "system" fn SetupDiInstallDriverFiles(deviceinfoset : HDEVINFO, deviceinfodata : *const SP_DEVINFO_DATA) -> windows_core::BOOL);
    unsafe { SetupDiInstallDriverFiles(deviceinfoset, deviceinfodata).ok() }
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn SetupDiLoadClassIcon(classguid: *const windows_core::GUID, largeicon: Option<*mut super::super::UI::WindowsAndMessaging::HICON>, miniiconindex: Option<*mut i32>) -> windows_core::Result<()> {
    windows_core::link!("setupapi.dll" "system" fn SetupDiLoadClassIcon(classguid : *const windows_core::GUID, largeicon : *mut super::super::UI::WindowsAndMessaging:: HICON, miniiconindex : *mut i32) -> windows_core::BOOL);
    unsafe { SetupDiLoadClassIcon(classguid, largeicon.unwrap_or(core::mem::zeroed()) as _, miniiconindex.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn SetupDiLoadDeviceIcon(deviceinfoset: HDEVINFO, deviceinfodata: *const SP_DEVINFO_DATA, cxicon: u32, cyicon: u32, flags: u32, hicon: *mut super::super::UI::WindowsAndMessaging::HICON) -> windows_core::Result<()> {
    windows_core::link!("setupapi.dll" "system" fn SetupDiLoadDeviceIcon(deviceinfoset : HDEVINFO, deviceinfodata : *const SP_DEVINFO_DATA, cxicon : u32, cyicon : u32, flags : u32, hicon : *mut super::super::UI::WindowsAndMessaging:: HICON) -> windows_core::BOOL);
    unsafe { SetupDiLoadDeviceIcon(deviceinfoset, deviceinfodata, cxicon, cyicon, flags, hicon as _).ok() }
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn SetupDiOpenClassRegKey(classguid: Option<*const windows_core::GUID>, samdesired: u32) -> windows_core::Result<super::super::System::Registry::HKEY> {
    windows_core::link!("setupapi.dll" "system" fn SetupDiOpenClassRegKey(classguid : *const windows_core::GUID, samdesired : u32) -> super::super::System::Registry:: HKEY);
    let result__ = unsafe { SetupDiOpenClassRegKey(classguid.unwrap_or(core::mem::zeroed()) as _, samdesired) };
    (!result__.is_invalid()).then_some(result__).ok_or_else(windows_core::Error::from_thread)
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn SetupDiOpenClassRegKeyExA<P3>(classguid: Option<*const windows_core::GUID>, samdesired: u32, flags: u32, machinename: P3, reserved: Option<*const core::ffi::c_void>) -> windows_core::Result<super::super::System::Registry::HKEY>
where
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupDiOpenClassRegKeyExA(classguid : *const windows_core::GUID, samdesired : u32, flags : u32, machinename : windows_core::PCSTR, reserved : *const core::ffi::c_void) -> super::super::System::Registry:: HKEY);
    let result__ = unsafe { SetupDiOpenClassRegKeyExA(classguid.unwrap_or(core::mem::zeroed()) as _, samdesired, flags, machinename.param().abi(), reserved.unwrap_or(core::mem::zeroed()) as _) };
    (!result__.is_invalid()).then_some(result__).ok_or_else(windows_core::Error::from_thread)
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn SetupDiOpenClassRegKeyExW<P3>(classguid: Option<*const windows_core::GUID>, samdesired: u32, flags: u32, machinename: P3, reserved: Option<*const core::ffi::c_void>) -> windows_core::Result<super::super::System::Registry::HKEY>
where
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupDiOpenClassRegKeyExW(classguid : *const windows_core::GUID, samdesired : u32, flags : u32, machinename : windows_core::PCWSTR, reserved : *const core::ffi::c_void) -> super::super::System::Registry:: HKEY);
    let result__ = unsafe { SetupDiOpenClassRegKeyExW(classguid.unwrap_or(core::mem::zeroed()) as _, samdesired, flags, machinename.param().abi(), reserved.unwrap_or(core::mem::zeroed()) as _) };
    (!result__.is_invalid()).then_some(result__).ok_or_else(windows_core::Error::from_thread)
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn SetupDiOpenDevRegKey(deviceinfoset: HDEVINFO, deviceinfodata: *const SP_DEVINFO_DATA, scope: u32, hwprofile: u32, keytype: u32, samdesired: u32) -> windows_core::Result<super::super::System::Registry::HKEY> {
    windows_core::link!("setupapi.dll" "system" fn SetupDiOpenDevRegKey(deviceinfoset : HDEVINFO, deviceinfodata : *const SP_DEVINFO_DATA, scope : u32, hwprofile : u32, keytype : u32, samdesired : u32) -> super::super::System::Registry:: HKEY);
    let result__ = unsafe { SetupDiOpenDevRegKey(deviceinfoset, deviceinfodata, scope, hwprofile, keytype, samdesired) };
    (!result__.is_invalid()).then_some(result__).ok_or_else(windows_core::Error::from_thread)
}
#[inline]
pub unsafe fn SetupDiOpenDeviceInfoA<P1>(deviceinfoset: HDEVINFO, deviceinstanceid: P1, hwndparent: Option<super::super::Foundation::HWND>, openflags: u32, deviceinfodata: Option<*mut SP_DEVINFO_DATA>) -> windows_core::Result<()>
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupDiOpenDeviceInfoA(deviceinfoset : HDEVINFO, deviceinstanceid : windows_core::PCSTR, hwndparent : super::super::Foundation:: HWND, openflags : u32, deviceinfodata : *mut SP_DEVINFO_DATA) -> windows_core::BOOL);
    unsafe { SetupDiOpenDeviceInfoA(deviceinfoset, deviceinstanceid.param().abi(), hwndparent.unwrap_or(core::mem::zeroed()) as _, openflags, deviceinfodata.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn SetupDiOpenDeviceInfoW<P1>(deviceinfoset: HDEVINFO, deviceinstanceid: P1, hwndparent: Option<super::super::Foundation::HWND>, openflags: u32, deviceinfodata: Option<*mut SP_DEVINFO_DATA>) -> windows_core::Result<()>
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupDiOpenDeviceInfoW(deviceinfoset : HDEVINFO, deviceinstanceid : windows_core::PCWSTR, hwndparent : super::super::Foundation:: HWND, openflags : u32, deviceinfodata : *mut SP_DEVINFO_DATA) -> windows_core::BOOL);
    unsafe { SetupDiOpenDeviceInfoW(deviceinfoset, deviceinstanceid.param().abi(), hwndparent.unwrap_or(core::mem::zeroed()) as _, openflags, deviceinfodata.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn SetupDiOpenDeviceInterfaceA<P1>(deviceinfoset: HDEVINFO, devicepath: P1, openflags: u32, deviceinterfacedata: Option<*mut SP_DEVICE_INTERFACE_DATA>) -> windows_core::Result<()>
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupDiOpenDeviceInterfaceA(deviceinfoset : HDEVINFO, devicepath : windows_core::PCSTR, openflags : u32, deviceinterfacedata : *mut SP_DEVICE_INTERFACE_DATA) -> windows_core::BOOL);
    unsafe { SetupDiOpenDeviceInterfaceA(deviceinfoset, devicepath.param().abi(), openflags, deviceinterfacedata.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn SetupDiOpenDeviceInterfaceRegKey(deviceinfoset: HDEVINFO, deviceinterfacedata: *const SP_DEVICE_INTERFACE_DATA, reserved: Option<u32>, samdesired: u32) -> windows_core::Result<super::super::System::Registry::HKEY> {
    windows_core::link!("setupapi.dll" "system" fn SetupDiOpenDeviceInterfaceRegKey(deviceinfoset : HDEVINFO, deviceinterfacedata : *const SP_DEVICE_INTERFACE_DATA, reserved : u32, samdesired : u32) -> super::super::System::Registry:: HKEY);
    let result__ = unsafe { SetupDiOpenDeviceInterfaceRegKey(deviceinfoset, deviceinterfacedata, reserved.unwrap_or(core::mem::zeroed()) as _, samdesired) };
    (!result__.is_invalid()).then_some(result__).ok_or_else(windows_core::Error::from_thread)
}
#[inline]
pub unsafe fn SetupDiOpenDeviceInterfaceW<P1>(deviceinfoset: HDEVINFO, devicepath: P1, openflags: u32, deviceinterfacedata: Option<*mut SP_DEVICE_INTERFACE_DATA>) -> windows_core::Result<()>
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupDiOpenDeviceInterfaceW(deviceinfoset : HDEVINFO, devicepath : windows_core::PCWSTR, openflags : u32, deviceinterfacedata : *mut SP_DEVICE_INTERFACE_DATA) -> windows_core::BOOL);
    unsafe { SetupDiOpenDeviceInterfaceW(deviceinfoset, devicepath.param().abi(), openflags, deviceinterfacedata.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn SetupDiRegisterCoDeviceInstallers(deviceinfoset: HDEVINFO, deviceinfodata: *const SP_DEVINFO_DATA) -> windows_core::Result<()> {
    windows_core::link!("setupapi.dll" "system" fn SetupDiRegisterCoDeviceInstallers(deviceinfoset : HDEVINFO, deviceinfodata : *const SP_DEVINFO_DATA) -> windows_core::BOOL);
    unsafe { SetupDiRegisterCoDeviceInstallers(deviceinfoset, deviceinfodata).ok() }
}
#[inline]
pub unsafe fn SetupDiRegisterDeviceInfo(deviceinfoset: HDEVINFO, deviceinfodata: *mut SP_DEVINFO_DATA, flags: u32, compareproc: PSP_DETSIG_CMPPROC, comparecontext: Option<*const core::ffi::c_void>, dupdeviceinfodata: Option<*mut SP_DEVINFO_DATA>) -> windows_core::Result<()> {
    windows_core::link!("setupapi.dll" "system" fn SetupDiRegisterDeviceInfo(deviceinfoset : HDEVINFO, deviceinfodata : *mut SP_DEVINFO_DATA, flags : u32, compareproc : PSP_DETSIG_CMPPROC, comparecontext : *const core::ffi::c_void, dupdeviceinfodata : *mut SP_DEVINFO_DATA) -> windows_core::BOOL);
    unsafe { SetupDiRegisterDeviceInfo(deviceinfoset, deviceinfodata as _, flags, compareproc, comparecontext.unwrap_or(core::mem::zeroed()) as _, dupdeviceinfodata.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn SetupDiRemoveDevice(deviceinfoset: HDEVINFO, deviceinfodata: *mut SP_DEVINFO_DATA) -> windows_core::BOOL {
    windows_core::link!("setupapi.dll" "system" fn SetupDiRemoveDevice(deviceinfoset : HDEVINFO, deviceinfodata : *mut SP_DEVINFO_DATA) -> windows_core::BOOL);
    unsafe { SetupDiRemoveDevice(deviceinfoset, deviceinfodata as _) }
}
#[inline]
pub unsafe fn SetupDiRemoveDeviceInterface(deviceinfoset: HDEVINFO, deviceinterfacedata: *mut SP_DEVICE_INTERFACE_DATA) -> windows_core::Result<()> {
    windows_core::link!("setupapi.dll" "system" fn SetupDiRemoveDeviceInterface(deviceinfoset : HDEVINFO, deviceinterfacedata : *mut SP_DEVICE_INTERFACE_DATA) -> windows_core::BOOL);
    unsafe { SetupDiRemoveDeviceInterface(deviceinfoset, deviceinterfacedata as _).ok() }
}
#[inline]
pub unsafe fn SetupDiRestartDevices(deviceinfoset: HDEVINFO, deviceinfodata: *mut SP_DEVINFO_DATA) -> windows_core::Result<()> {
    windows_core::link!("setupapi.dll" "system" fn SetupDiRestartDevices(deviceinfoset : HDEVINFO, deviceinfodata : *mut SP_DEVINFO_DATA) -> windows_core::BOOL);
    unsafe { SetupDiRestartDevices(deviceinfoset, deviceinfodata as _).ok() }
}
#[inline]
pub unsafe fn SetupDiSelectBestCompatDrv(deviceinfoset: HDEVINFO, deviceinfodata: Option<*mut SP_DEVINFO_DATA>) -> windows_core::Result<()> {
    windows_core::link!("setupapi.dll" "system" fn SetupDiSelectBestCompatDrv(deviceinfoset : HDEVINFO, deviceinfodata : *mut SP_DEVINFO_DATA) -> windows_core::BOOL);
    unsafe { SetupDiSelectBestCompatDrv(deviceinfoset, deviceinfodata.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn SetupDiSelectDevice(deviceinfoset: HDEVINFO, deviceinfodata: Option<*mut SP_DEVINFO_DATA>) -> windows_core::Result<()> {
    windows_core::link!("setupapi.dll" "system" fn SetupDiSelectDevice(deviceinfoset : HDEVINFO, deviceinfodata : *mut SP_DEVINFO_DATA) -> windows_core::BOOL);
    unsafe { SetupDiSelectDevice(deviceinfoset, deviceinfodata.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn SetupDiSelectOEMDrv(hwndparent: Option<super::super::Foundation::HWND>, deviceinfoset: HDEVINFO, deviceinfodata: Option<*mut SP_DEVINFO_DATA>) -> windows_core::Result<()> {
    windows_core::link!("setupapi.dll" "system" fn SetupDiSelectOEMDrv(hwndparent : super::super::Foundation:: HWND, deviceinfoset : HDEVINFO, deviceinfodata : *mut SP_DEVINFO_DATA) -> windows_core::BOOL);
    unsafe { SetupDiSelectOEMDrv(hwndparent.unwrap_or(core::mem::zeroed()) as _, deviceinfoset, deviceinfodata.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn SetupDiSetClassInstallParamsA(deviceinfoset: HDEVINFO, deviceinfodata: Option<*const SP_DEVINFO_DATA>, classinstallparams: Option<*const SP_CLASSINSTALL_HEADER>, classinstallparamssize: u32) -> windows_core::Result<()> {
    windows_core::link!("setupapi.dll" "system" fn SetupDiSetClassInstallParamsA(deviceinfoset : HDEVINFO, deviceinfodata : *const SP_DEVINFO_DATA, classinstallparams : *const SP_CLASSINSTALL_HEADER, classinstallparamssize : u32) -> windows_core::BOOL);
    unsafe { SetupDiSetClassInstallParamsA(deviceinfoset, deviceinfodata.unwrap_or(core::mem::zeroed()) as _, classinstallparams.unwrap_or(core::mem::zeroed()) as _, classinstallparamssize).ok() }
}
#[inline]
pub unsafe fn SetupDiSetClassInstallParamsW(deviceinfoset: HDEVINFO, deviceinfodata: Option<*const SP_DEVINFO_DATA>, classinstallparams: Option<*const SP_CLASSINSTALL_HEADER>, classinstallparamssize: u32) -> windows_core::Result<()> {
    windows_core::link!("setupapi.dll" "system" fn SetupDiSetClassInstallParamsW(deviceinfoset : HDEVINFO, deviceinfodata : *const SP_DEVINFO_DATA, classinstallparams : *const SP_CLASSINSTALL_HEADER, classinstallparamssize : u32) -> windows_core::BOOL);
    unsafe { SetupDiSetClassInstallParamsW(deviceinfoset, deviceinfodata.unwrap_or(core::mem::zeroed()) as _, classinstallparams.unwrap_or(core::mem::zeroed()) as _, classinstallparamssize).ok() }
}
#[cfg(feature = "Win32_Devices_Properties")]
#[inline]
pub unsafe fn SetupDiSetClassPropertyExW<P6>(classguid: *const windows_core::GUID, propertykey: *const super::super::Foundation::DEVPROPKEY, propertytype: super::Properties::DEVPROPTYPE, propertybuffer: Option<&[u8]>, flags: u32, machinename: P6, reserved: Option<*const core::ffi::c_void>) -> windows_core::Result<()>
where
    P6: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupDiSetClassPropertyExW(classguid : *const windows_core::GUID, propertykey : *const super::super::Foundation:: DEVPROPKEY, propertytype : super::Properties:: DEVPROPTYPE, propertybuffer : *const u8, propertybuffersize : u32, flags : u32, machinename : windows_core::PCWSTR, reserved : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe { SetupDiSetClassPropertyExW(classguid, propertykey, propertytype, core::mem::transmute(propertybuffer.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), propertybuffer.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), flags, machinename.param().abi(), reserved.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[cfg(feature = "Win32_Devices_Properties")]
#[inline]
pub unsafe fn SetupDiSetClassPropertyW(classguid: *const windows_core::GUID, propertykey: *const super::super::Foundation::DEVPROPKEY, propertytype: super::Properties::DEVPROPTYPE, propertybuffer: Option<&[u8]>, flags: u32) -> windows_core::Result<()> {
    windows_core::link!("setupapi.dll" "system" fn SetupDiSetClassPropertyW(classguid : *const windows_core::GUID, propertykey : *const super::super::Foundation:: DEVPROPKEY, propertytype : super::Properties:: DEVPROPTYPE, propertybuffer : *const u8, propertybuffersize : u32, flags : u32) -> windows_core::BOOL);
    unsafe { SetupDiSetClassPropertyW(classguid, propertykey, propertytype, core::mem::transmute(propertybuffer.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), propertybuffer.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), flags).ok() }
}
#[inline]
pub unsafe fn SetupDiSetClassRegistryPropertyA<P4>(classguid: *const windows_core::GUID, property: u32, propertybuffer: Option<&[u8]>, machinename: P4, reserved: Option<*const core::ffi::c_void>) -> windows_core::Result<()>
where
    P4: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupDiSetClassRegistryPropertyA(classguid : *const windows_core::GUID, property : u32, propertybuffer : *const u8, propertybuffersize : u32, machinename : windows_core::PCSTR, reserved : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe { SetupDiSetClassRegistryPropertyA(classguid, property, core::mem::transmute(propertybuffer.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), propertybuffer.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), machinename.param().abi(), reserved.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn SetupDiSetClassRegistryPropertyW<P4>(classguid: *const windows_core::GUID, property: u32, propertybuffer: Option<&[u8]>, machinename: P4, reserved: Option<*const core::ffi::c_void>) -> windows_core::Result<()>
where
    P4: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupDiSetClassRegistryPropertyW(classguid : *const windows_core::GUID, property : u32, propertybuffer : *const u8, propertybuffersize : u32, machinename : windows_core::PCWSTR, reserved : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe { SetupDiSetClassRegistryPropertyW(classguid, property, core::mem::transmute(propertybuffer.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), propertybuffer.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), machinename.param().abi(), reserved.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn SetupDiSetDeviceInstallParamsA(deviceinfoset: HDEVINFO, deviceinfodata: Option<*const SP_DEVINFO_DATA>, deviceinstallparams: *const SP_DEVINSTALL_PARAMS_A) -> windows_core::Result<()> {
    windows_core::link!("setupapi.dll" "system" fn SetupDiSetDeviceInstallParamsA(deviceinfoset : HDEVINFO, deviceinfodata : *const SP_DEVINFO_DATA, deviceinstallparams : *const SP_DEVINSTALL_PARAMS_A) -> windows_core::BOOL);
    unsafe { SetupDiSetDeviceInstallParamsA(deviceinfoset, deviceinfodata.unwrap_or(core::mem::zeroed()) as _, deviceinstallparams).ok() }
}
#[inline]
pub unsafe fn SetupDiSetDeviceInstallParamsW(deviceinfoset: HDEVINFO, deviceinfodata: Option<*const SP_DEVINFO_DATA>, deviceinstallparams: *const SP_DEVINSTALL_PARAMS_W) -> windows_core::Result<()> {
    windows_core::link!("setupapi.dll" "system" fn SetupDiSetDeviceInstallParamsW(deviceinfoset : HDEVINFO, deviceinfodata : *const SP_DEVINFO_DATA, deviceinstallparams : *const SP_DEVINSTALL_PARAMS_W) -> windows_core::BOOL);
    unsafe { SetupDiSetDeviceInstallParamsW(deviceinfoset, deviceinfodata.unwrap_or(core::mem::zeroed()) as _, deviceinstallparams).ok() }
}
#[inline]
pub unsafe fn SetupDiSetDeviceInterfaceDefault(deviceinfoset: HDEVINFO, deviceinterfacedata: *mut SP_DEVICE_INTERFACE_DATA, flags: u32, reserved: Option<*const core::ffi::c_void>) -> windows_core::Result<()> {
    windows_core::link!("setupapi.dll" "system" fn SetupDiSetDeviceInterfaceDefault(deviceinfoset : HDEVINFO, deviceinterfacedata : *mut SP_DEVICE_INTERFACE_DATA, flags : u32, reserved : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe { SetupDiSetDeviceInterfaceDefault(deviceinfoset, deviceinterfacedata as _, flags, reserved.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[cfg(feature = "Win32_Devices_Properties")]
#[inline]
pub unsafe fn SetupDiSetDeviceInterfacePropertyW(deviceinfoset: HDEVINFO, deviceinterfacedata: *const SP_DEVICE_INTERFACE_DATA, propertykey: *const super::super::Foundation::DEVPROPKEY, propertytype: super::Properties::DEVPROPTYPE, propertybuffer: Option<&[u8]>, flags: u32) -> windows_core::Result<()> {
    windows_core::link!("setupapi.dll" "system" fn SetupDiSetDeviceInterfacePropertyW(deviceinfoset : HDEVINFO, deviceinterfacedata : *const SP_DEVICE_INTERFACE_DATA, propertykey : *const super::super::Foundation:: DEVPROPKEY, propertytype : super::Properties:: DEVPROPTYPE, propertybuffer : *const u8, propertybuffersize : u32, flags : u32) -> windows_core::BOOL);
    unsafe { SetupDiSetDeviceInterfacePropertyW(deviceinfoset, deviceinterfacedata, propertykey, propertytype, core::mem::transmute(propertybuffer.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), propertybuffer.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), flags).ok() }
}
#[cfg(feature = "Win32_Devices_Properties")]
#[inline]
pub unsafe fn SetupDiSetDevicePropertyW(deviceinfoset: HDEVINFO, deviceinfodata: *const SP_DEVINFO_DATA, propertykey: *const super::super::Foundation::DEVPROPKEY, propertytype: super::Properties::DEVPROPTYPE, propertybuffer: Option<&[u8]>, flags: u32) -> windows_core::Result<()> {
    windows_core::link!("setupapi.dll" "system" fn SetupDiSetDevicePropertyW(deviceinfoset : HDEVINFO, deviceinfodata : *const SP_DEVINFO_DATA, propertykey : *const super::super::Foundation:: DEVPROPKEY, propertytype : super::Properties:: DEVPROPTYPE, propertybuffer : *const u8, propertybuffersize : u32, flags : u32) -> windows_core::BOOL);
    unsafe { SetupDiSetDevicePropertyW(deviceinfoset, deviceinfodata, propertykey, propertytype, core::mem::transmute(propertybuffer.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), propertybuffer.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), flags).ok() }
}
#[inline]
pub unsafe fn SetupDiSetDeviceRegistryPropertyA(deviceinfoset: HDEVINFO, deviceinfodata: *mut SP_DEVINFO_DATA, property: SETUP_DI_REGISTRY_PROPERTY, propertybuffer: Option<&[u8]>) -> windows_core::Result<()> {
    windows_core::link!("setupapi.dll" "system" fn SetupDiSetDeviceRegistryPropertyA(deviceinfoset : HDEVINFO, deviceinfodata : *mut SP_DEVINFO_DATA, property : SETUP_DI_REGISTRY_PROPERTY, propertybuffer : *const u8, propertybuffersize : u32) -> windows_core::BOOL);
    unsafe { SetupDiSetDeviceRegistryPropertyA(deviceinfoset, deviceinfodata as _, property, core::mem::transmute(propertybuffer.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), propertybuffer.as_deref().map_or(0, |slice| slice.len().try_into().unwrap())).ok() }
}
#[inline]
pub unsafe fn SetupDiSetDeviceRegistryPropertyW(deviceinfoset: HDEVINFO, deviceinfodata: *mut SP_DEVINFO_DATA, property: SETUP_DI_REGISTRY_PROPERTY, propertybuffer: Option<&[u8]>) -> windows_core::Result<()> {
    windows_core::link!("setupapi.dll" "system" fn SetupDiSetDeviceRegistryPropertyW(deviceinfoset : HDEVINFO, deviceinfodata : *mut SP_DEVINFO_DATA, property : SETUP_DI_REGISTRY_PROPERTY, propertybuffer : *const u8, propertybuffersize : u32) -> windows_core::BOOL);
    unsafe { SetupDiSetDeviceRegistryPropertyW(deviceinfoset, deviceinfodata as _, property, core::mem::transmute(propertybuffer.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), propertybuffer.as_deref().map_or(0, |slice| slice.len().try_into().unwrap())).ok() }
}
#[inline]
pub unsafe fn SetupDiSetDriverInstallParamsA(deviceinfoset: HDEVINFO, deviceinfodata: Option<*const SP_DEVINFO_DATA>, driverinfodata: *const SP_DRVINFO_DATA_V2_A, driverinstallparams: *const SP_DRVINSTALL_PARAMS) -> windows_core::Result<()> {
    windows_core::link!("setupapi.dll" "system" fn SetupDiSetDriverInstallParamsA(deviceinfoset : HDEVINFO, deviceinfodata : *const SP_DEVINFO_DATA, driverinfodata : *const SP_DRVINFO_DATA_V2_A, driverinstallparams : *const SP_DRVINSTALL_PARAMS) -> windows_core::BOOL);
    unsafe { SetupDiSetDriverInstallParamsA(deviceinfoset, deviceinfodata.unwrap_or(core::mem::zeroed()) as _, driverinfodata, driverinstallparams).ok() }
}
#[inline]
pub unsafe fn SetupDiSetDriverInstallParamsW(deviceinfoset: HDEVINFO, deviceinfodata: Option<*const SP_DEVINFO_DATA>, driverinfodata: *const SP_DRVINFO_DATA_V2_W, driverinstallparams: *const SP_DRVINSTALL_PARAMS) -> windows_core::Result<()> {
    windows_core::link!("setupapi.dll" "system" fn SetupDiSetDriverInstallParamsW(deviceinfoset : HDEVINFO, deviceinfodata : *const SP_DEVINFO_DATA, driverinfodata : *const SP_DRVINFO_DATA_V2_W, driverinstallparams : *const SP_DRVINSTALL_PARAMS) -> windows_core::BOOL);
    unsafe { SetupDiSetDriverInstallParamsW(deviceinfoset, deviceinfodata.unwrap_or(core::mem::zeroed()) as _, driverinfodata, driverinstallparams).ok() }
}
#[inline]
pub unsafe fn SetupDiSetSelectedDevice(deviceinfoset: HDEVINFO, deviceinfodata: *const SP_DEVINFO_DATA) -> windows_core::Result<()> {
    windows_core::link!("setupapi.dll" "system" fn SetupDiSetSelectedDevice(deviceinfoset : HDEVINFO, deviceinfodata : *const SP_DEVINFO_DATA) -> windows_core::BOOL);
    unsafe { SetupDiSetSelectedDevice(deviceinfoset, deviceinfodata).ok() }
}
#[inline]
pub unsafe fn SetupDiSetSelectedDriverA(deviceinfoset: HDEVINFO, deviceinfodata: Option<*mut SP_DEVINFO_DATA>, driverinfodata: Option<*mut SP_DRVINFO_DATA_V2_A>) -> windows_core::Result<()> {
    windows_core::link!("setupapi.dll" "system" fn SetupDiSetSelectedDriverA(deviceinfoset : HDEVINFO, deviceinfodata : *mut SP_DEVINFO_DATA, driverinfodata : *mut SP_DRVINFO_DATA_V2_A) -> windows_core::BOOL);
    unsafe { SetupDiSetSelectedDriverA(deviceinfoset, deviceinfodata.unwrap_or(core::mem::zeroed()) as _, driverinfodata.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn SetupDiSetSelectedDriverW(deviceinfoset: HDEVINFO, deviceinfodata: Option<*mut SP_DEVINFO_DATA>, driverinfodata: Option<*mut SP_DRVINFO_DATA_V2_W>) -> windows_core::Result<()> {
    windows_core::link!("setupapi.dll" "system" fn SetupDiSetSelectedDriverW(deviceinfoset : HDEVINFO, deviceinfodata : *mut SP_DEVINFO_DATA, driverinfodata : *mut SP_DRVINFO_DATA_V2_W) -> windows_core::BOOL);
    unsafe { SetupDiSetSelectedDriverW(deviceinfoset, deviceinfodata.unwrap_or(core::mem::zeroed()) as _, driverinfodata.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn SetupDiUnremoveDevice(deviceinfoset: HDEVINFO, deviceinfodata: *mut SP_DEVINFO_DATA) -> windows_core::Result<()> {
    windows_core::link!("setupapi.dll" "system" fn SetupDiUnremoveDevice(deviceinfoset : HDEVINFO, deviceinfodata : *mut SP_DEVINFO_DATA) -> windows_core::BOOL);
    unsafe { SetupDiUnremoveDevice(deviceinfoset, deviceinfodata as _).ok() }
}
#[inline]
pub unsafe fn SetupDuplicateDiskSpaceListA(diskspace: *const core::ffi::c_void, reserved1: Option<*const core::ffi::c_void>, reserved2: Option<u32>, flags: u32) -> *mut core::ffi::c_void {
    windows_core::link!("setupapi.dll" "system" fn SetupDuplicateDiskSpaceListA(diskspace : *const core::ffi::c_void, reserved1 : *const core::ffi::c_void, reserved2 : u32, flags : u32) -> *mut core::ffi::c_void);
    unsafe { SetupDuplicateDiskSpaceListA(diskspace, reserved1.unwrap_or(core::mem::zeroed()) as _, reserved2.unwrap_or(core::mem::zeroed()) as _, flags) }
}
#[inline]
pub unsafe fn SetupDuplicateDiskSpaceListW(diskspace: *const core::ffi::c_void, reserved1: Option<*const core::ffi::c_void>, reserved2: Option<u32>, flags: u32) -> *mut core::ffi::c_void {
    windows_core::link!("setupapi.dll" "system" fn SetupDuplicateDiskSpaceListW(diskspace : *const core::ffi::c_void, reserved1 : *const core::ffi::c_void, reserved2 : u32, flags : u32) -> *mut core::ffi::c_void);
    unsafe { SetupDuplicateDiskSpaceListW(diskspace, reserved1.unwrap_or(core::mem::zeroed()) as _, reserved2.unwrap_or(core::mem::zeroed()) as _, flags) }
}
#[inline]
pub unsafe fn SetupEnumInfSectionsA(infhandle: *const core::ffi::c_void, index: u32, buffer: Option<&mut [u8]>, sizeneeded: Option<*mut u32>) -> windows_core::Result<()> {
    windows_core::link!("setupapi.dll" "system" fn SetupEnumInfSectionsA(infhandle : *const core::ffi::c_void, index : u32, buffer : windows_core::PSTR, size : u32, sizeneeded : *mut u32) -> windows_core::BOOL);
    unsafe { SetupEnumInfSectionsA(infhandle, index, core::mem::transmute(buffer.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), buffer.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), sizeneeded.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn SetupEnumInfSectionsW(infhandle: *const core::ffi::c_void, index: u32, buffer: Option<&mut [u16]>, sizeneeded: Option<*mut u32>) -> windows_core::Result<()> {
    windows_core::link!("setupapi.dll" "system" fn SetupEnumInfSectionsW(infhandle : *const core::ffi::c_void, index : u32, buffer : windows_core::PWSTR, size : u32, sizeneeded : *mut u32) -> windows_core::BOOL);
    unsafe { SetupEnumInfSectionsW(infhandle, index, core::mem::transmute(buffer.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), buffer.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), sizeneeded.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn SetupFindFirstLineA<P1, P2>(infhandle: *const core::ffi::c_void, section: P1, key: P2, context: *mut INFCONTEXT) -> windows_core::Result<()>
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupFindFirstLineA(infhandle : *const core::ffi::c_void, section : windows_core::PCSTR, key : windows_core::PCSTR, context : *mut INFCONTEXT) -> windows_core::BOOL);
    unsafe { SetupFindFirstLineA(infhandle, section.param().abi(), key.param().abi(), context as _).ok() }
}
#[inline]
pub unsafe fn SetupFindFirstLineW<P1, P2>(infhandle: *const core::ffi::c_void, section: P1, key: P2, context: *mut INFCONTEXT) -> windows_core::Result<()>
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupFindFirstLineW(infhandle : *const core::ffi::c_void, section : windows_core::PCWSTR, key : windows_core::PCWSTR, context : *mut INFCONTEXT) -> windows_core::BOOL);
    unsafe { SetupFindFirstLineW(infhandle, section.param().abi(), key.param().abi(), context as _).ok() }
}
#[inline]
pub unsafe fn SetupFindNextLine(contextin: *const INFCONTEXT, contextout: *mut INFCONTEXT) -> windows_core::Result<()> {
    windows_core::link!("setupapi.dll" "system" fn SetupFindNextLine(contextin : *const INFCONTEXT, contextout : *mut INFCONTEXT) -> windows_core::BOOL);
    unsafe { SetupFindNextLine(contextin, contextout as _).ok() }
}
#[inline]
pub unsafe fn SetupFindNextMatchLineA<P1>(contextin: *const INFCONTEXT, key: P1, contextout: *mut INFCONTEXT) -> windows_core::Result<()>
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupFindNextMatchLineA(contextin : *const INFCONTEXT, key : windows_core::PCSTR, contextout : *mut INFCONTEXT) -> windows_core::BOOL);
    unsafe { SetupFindNextMatchLineA(contextin, key.param().abi(), contextout as _).ok() }
}
#[inline]
pub unsafe fn SetupFindNextMatchLineW<P1>(contextin: *const INFCONTEXT, key: P1, contextout: *mut INFCONTEXT) -> windows_core::Result<()>
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupFindNextMatchLineW(contextin : *const INFCONTEXT, key : windows_core::PCWSTR, contextout : *mut INFCONTEXT) -> windows_core::BOOL);
    unsafe { SetupFindNextMatchLineW(contextin, key.param().abi(), contextout as _).ok() }
}
#[inline]
pub unsafe fn SetupFreeSourceListA(list: &mut [*mut windows_core::PCSTR]) -> windows_core::Result<()> {
    windows_core::link!("setupapi.dll" "system" fn SetupFreeSourceListA(list : *mut *mut windows_core::PCSTR, count : u32) -> windows_core::BOOL);
    unsafe { SetupFreeSourceListA(core::mem::transmute(list.as_ptr()), list.len().try_into().unwrap()).ok() }
}
#[inline]
pub unsafe fn SetupFreeSourceListW(list: &mut [*mut windows_core::PCWSTR]) -> windows_core::Result<()> {
    windows_core::link!("setupapi.dll" "system" fn SetupFreeSourceListW(list : *mut *mut windows_core::PCWSTR, count : u32) -> windows_core::BOOL);
    unsafe { SetupFreeSourceListW(core::mem::transmute(list.as_ptr()), list.len().try_into().unwrap()).ok() }
}
#[inline]
pub unsafe fn SetupGetBackupInformationA(queuehandle: *const core::ffi::c_void, backupparams: *mut SP_BACKUP_QUEUE_PARAMS_V2_A) -> windows_core::BOOL {
    windows_core::link!("setupapi.dll" "system" fn SetupGetBackupInformationA(queuehandle : *const core::ffi::c_void, backupparams : *mut SP_BACKUP_QUEUE_PARAMS_V2_A) -> windows_core::BOOL);
    unsafe { SetupGetBackupInformationA(queuehandle, backupparams as _) }
}
#[inline]
pub unsafe fn SetupGetBackupInformationW(queuehandle: *const core::ffi::c_void, backupparams: *mut SP_BACKUP_QUEUE_PARAMS_V2_W) -> windows_core::BOOL {
    windows_core::link!("setupapi.dll" "system" fn SetupGetBackupInformationW(queuehandle : *const core::ffi::c_void, backupparams : *mut SP_BACKUP_QUEUE_PARAMS_V2_W) -> windows_core::BOOL);
    unsafe { SetupGetBackupInformationW(queuehandle, backupparams as _) }
}
#[inline]
pub unsafe fn SetupGetBinaryField(context: *const INFCONTEXT, fieldindex: u32, returnbuffer: Option<&mut [u8]>, requiredsize: Option<*mut u32>) -> windows_core::Result<()> {
    windows_core::link!("setupapi.dll" "system" fn SetupGetBinaryField(context : *const INFCONTEXT, fieldindex : u32, returnbuffer : *mut u8, returnbuffersize : u32, requiredsize : *mut u32) -> windows_core::BOOL);
    unsafe { SetupGetBinaryField(context, fieldindex, core::mem::transmute(returnbuffer.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), returnbuffer.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), requiredsize.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn SetupGetFieldCount(context: *const INFCONTEXT) -> u32 {
    windows_core::link!("setupapi.dll" "system" fn SetupGetFieldCount(context : *const INFCONTEXT) -> u32);
    unsafe { SetupGetFieldCount(context) }
}
#[inline]
pub unsafe fn SetupGetFileCompressionInfoA<P0>(sourcefilename: P0, actualsourcefilename: *mut windows_core::PSTR, sourcefilesize: *mut u32, targetfilesize: *mut u32, compressiontype: *mut FILE_COMPRESSION_TYPE) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupGetFileCompressionInfoA(sourcefilename : windows_core::PCSTR, actualsourcefilename : *mut windows_core::PSTR, sourcefilesize : *mut u32, targetfilesize : *mut u32, compressiontype : *mut FILE_COMPRESSION_TYPE) -> u32);
    unsafe { SetupGetFileCompressionInfoA(sourcefilename.param().abi(), actualsourcefilename as _, sourcefilesize as _, targetfilesize as _, compressiontype as _) }
}
#[inline]
pub unsafe fn SetupGetFileCompressionInfoExA<P0>(sourcefilename: P0, actualsourcefilenamebuffer: Option<&[u8]>, requiredbufferlen: Option<*mut u32>, sourcefilesize: *mut u32, targetfilesize: *mut u32, compressiontype: *mut FILE_COMPRESSION_TYPE) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupGetFileCompressionInfoExA(sourcefilename : windows_core::PCSTR, actualsourcefilenamebuffer : windows_core::PCSTR, actualsourcefilenamebufferlen : u32, requiredbufferlen : *mut u32, sourcefilesize : *mut u32, targetfilesize : *mut u32, compressiontype : *mut FILE_COMPRESSION_TYPE) -> windows_core::BOOL);
    unsafe { SetupGetFileCompressionInfoExA(sourcefilename.param().abi(), core::mem::transmute(actualsourcefilenamebuffer.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), actualsourcefilenamebuffer.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), requiredbufferlen.unwrap_or(core::mem::zeroed()) as _, sourcefilesize as _, targetfilesize as _, compressiontype as _).ok() }
}
#[inline]
pub unsafe fn SetupGetFileCompressionInfoExW<P0>(sourcefilename: P0, actualsourcefilenamebuffer: Option<&[u16]>, requiredbufferlen: Option<*mut u32>, sourcefilesize: *mut u32, targetfilesize: *mut u32, compressiontype: *mut FILE_COMPRESSION_TYPE) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupGetFileCompressionInfoExW(sourcefilename : windows_core::PCWSTR, actualsourcefilenamebuffer : windows_core::PCWSTR, actualsourcefilenamebufferlen : u32, requiredbufferlen : *mut u32, sourcefilesize : *mut u32, targetfilesize : *mut u32, compressiontype : *mut FILE_COMPRESSION_TYPE) -> windows_core::BOOL);
    unsafe { SetupGetFileCompressionInfoExW(sourcefilename.param().abi(), core::mem::transmute(actualsourcefilenamebuffer.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), actualsourcefilenamebuffer.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), requiredbufferlen.unwrap_or(core::mem::zeroed()) as _, sourcefilesize as _, targetfilesize as _, compressiontype as _).ok() }
}
#[inline]
pub unsafe fn SetupGetFileCompressionInfoW<P0>(sourcefilename: P0, actualsourcefilename: *mut windows_core::PWSTR, sourcefilesize: *mut u32, targetfilesize: *mut u32, compressiontype: *mut FILE_COMPRESSION_TYPE) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupGetFileCompressionInfoW(sourcefilename : windows_core::PCWSTR, actualsourcefilename : *mut windows_core::PWSTR, sourcefilesize : *mut u32, targetfilesize : *mut u32, compressiontype : *mut FILE_COMPRESSION_TYPE) -> u32);
    unsafe { SetupGetFileCompressionInfoW(sourcefilename.param().abi(), actualsourcefilename as _, sourcefilesize as _, targetfilesize as _, compressiontype as _) }
}
#[inline]
pub unsafe fn SetupGetFileQueueCount(filequeue: *const core::ffi::c_void, subqueuefileop: u32, numoperations: *mut u32) -> windows_core::Result<()> {
    windows_core::link!("setupapi.dll" "system" fn SetupGetFileQueueCount(filequeue : *const core::ffi::c_void, subqueuefileop : u32, numoperations : *mut u32) -> windows_core::BOOL);
    unsafe { SetupGetFileQueueCount(filequeue, subqueuefileop, numoperations as _).ok() }
}
#[inline]
pub unsafe fn SetupGetFileQueueFlags(filequeue: *const core::ffi::c_void, flags: *mut u32) -> windows_core::Result<()> {
    windows_core::link!("setupapi.dll" "system" fn SetupGetFileQueueFlags(filequeue : *const core::ffi::c_void, flags : *mut u32) -> windows_core::BOOL);
    unsafe { SetupGetFileQueueFlags(filequeue, flags as _).ok() }
}
#[cfg(all(feature = "Win32_System_Diagnostics_Debug", feature = "Win32_System_SystemInformation"))]
#[inline]
pub unsafe fn SetupGetInfDriverStoreLocationA<P0, P2>(filename: P0, alternateplatforminfo: Option<*const SP_ALTPLATFORM_INFO_V2>, localename: P2, returnbuffer: &mut [u8], requiredsize: Option<*mut u32>) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupGetInfDriverStoreLocationA(filename : windows_core::PCSTR, alternateplatforminfo : *const SP_ALTPLATFORM_INFO_V2, localename : windows_core::PCSTR, returnbuffer : windows_core::PSTR, returnbuffersize : u32, requiredsize : *mut u32) -> windows_core::BOOL);
    unsafe { SetupGetInfDriverStoreLocationA(filename.param().abi(), alternateplatforminfo.unwrap_or(core::mem::zeroed()) as _, localename.param().abi(), core::mem::transmute(returnbuffer.as_ptr()), returnbuffer.len().try_into().unwrap(), requiredsize.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[cfg(all(feature = "Win32_System_Diagnostics_Debug", feature = "Win32_System_SystemInformation"))]
#[inline]
pub unsafe fn SetupGetInfDriverStoreLocationW<P0, P2>(filename: P0, alternateplatforminfo: Option<*const SP_ALTPLATFORM_INFO_V2>, localename: P2, returnbuffer: &mut [u16], requiredsize: Option<*mut u32>) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupGetInfDriverStoreLocationW(filename : windows_core::PCWSTR, alternateplatforminfo : *const SP_ALTPLATFORM_INFO_V2, localename : windows_core::PCWSTR, returnbuffer : windows_core::PWSTR, returnbuffersize : u32, requiredsize : *mut u32) -> windows_core::BOOL);
    unsafe { SetupGetInfDriverStoreLocationW(filename.param().abi(), alternateplatforminfo.unwrap_or(core::mem::zeroed()) as _, localename.param().abi(), core::mem::transmute(returnbuffer.as_ptr()), returnbuffer.len().try_into().unwrap(), requiredsize.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn SetupGetInfFileListA<P0>(directorypath: P0, infstyle: INF_STYLE, returnbuffer: Option<&mut [u8]>, requiredsize: Option<*mut u32>) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupGetInfFileListA(directorypath : windows_core::PCSTR, infstyle : INF_STYLE, returnbuffer : windows_core::PSTR, returnbuffersize : u32, requiredsize : *mut u32) -> windows_core::BOOL);
    unsafe { SetupGetInfFileListA(directorypath.param().abi(), infstyle, core::mem::transmute(returnbuffer.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), returnbuffer.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), requiredsize.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn SetupGetInfFileListW<P0>(directorypath: P0, infstyle: INF_STYLE, returnbuffer: &mut [u16], requiredsize: Option<*mut u32>) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupGetInfFileListW(directorypath : windows_core::PCWSTR, infstyle : INF_STYLE, returnbuffer : windows_core::PWSTR, returnbuffersize : u32, requiredsize : *mut u32) -> windows_core::BOOL);
    unsafe { SetupGetInfFileListW(directorypath.param().abi(), infstyle, core::mem::transmute(returnbuffer.as_ptr()), returnbuffer.len().try_into().unwrap(), requiredsize.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn SetupGetInfInformationA(infspec: *const core::ffi::c_void, searchcontrol: u32, returnbuffer: Option<*mut SP_INF_INFORMATION>, returnbuffersize: u32, requiredsize: Option<*mut u32>) -> windows_core::Result<()> {
    windows_core::link!("setupapi.dll" "system" fn SetupGetInfInformationA(infspec : *const core::ffi::c_void, searchcontrol : u32, returnbuffer : *mut SP_INF_INFORMATION, returnbuffersize : u32, requiredsize : *mut u32) -> windows_core::BOOL);
    unsafe { SetupGetInfInformationA(infspec, searchcontrol, returnbuffer.unwrap_or(core::mem::zeroed()) as _, returnbuffersize, requiredsize.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn SetupGetInfInformationW(infspec: *const core::ffi::c_void, searchcontrol: u32, returnbuffer: Option<*mut SP_INF_INFORMATION>, returnbuffersize: u32, requiredsize: Option<*mut u32>) -> windows_core::Result<()> {
    windows_core::link!("setupapi.dll" "system" fn SetupGetInfInformationW(infspec : *const core::ffi::c_void, searchcontrol : u32, returnbuffer : *mut SP_INF_INFORMATION, returnbuffersize : u32, requiredsize : *mut u32) -> windows_core::BOOL);
    unsafe { SetupGetInfInformationW(infspec, searchcontrol, returnbuffer.unwrap_or(core::mem::zeroed()) as _, returnbuffersize, requiredsize.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn SetupGetInfPublishedNameA<P0>(driverstorelocation: P0, returnbuffer: &mut [u8], requiredsize: Option<*mut u32>) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupGetInfPublishedNameA(driverstorelocation : windows_core::PCSTR, returnbuffer : windows_core::PSTR, returnbuffersize : u32, requiredsize : *mut u32) -> windows_core::BOOL);
    unsafe { SetupGetInfPublishedNameA(driverstorelocation.param().abi(), core::mem::transmute(returnbuffer.as_ptr()), returnbuffer.len().try_into().unwrap(), requiredsize.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn SetupGetInfPublishedNameW<P0>(driverstorelocation: P0, returnbuffer: &mut [u16], requiredsize: Option<*mut u32>) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupGetInfPublishedNameW(driverstorelocation : windows_core::PCWSTR, returnbuffer : windows_core::PWSTR, returnbuffersize : u32, requiredsize : *mut u32) -> windows_core::BOOL);
    unsafe { SetupGetInfPublishedNameW(driverstorelocation.param().abi(), core::mem::transmute(returnbuffer.as_ptr()), returnbuffer.len().try_into().unwrap(), requiredsize.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn SetupGetIntField(context: *const INFCONTEXT, fieldindex: u32, integervalue: *mut i32) -> windows_core::Result<()> {
    windows_core::link!("setupapi.dll" "system" fn SetupGetIntField(context : *const INFCONTEXT, fieldindex : u32, integervalue : *mut i32) -> windows_core::BOOL);
    unsafe { SetupGetIntField(context, fieldindex, integervalue as _).ok() }
}
#[inline]
pub unsafe fn SetupGetLineByIndexA<P1>(infhandle: *const core::ffi::c_void, section: P1, index: u32, context: *mut INFCONTEXT) -> windows_core::Result<()>
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupGetLineByIndexA(infhandle : *const core::ffi::c_void, section : windows_core::PCSTR, index : u32, context : *mut INFCONTEXT) -> windows_core::BOOL);
    unsafe { SetupGetLineByIndexA(infhandle, section.param().abi(), index, context as _).ok() }
}
#[inline]
pub unsafe fn SetupGetLineByIndexW<P1>(infhandle: *const core::ffi::c_void, section: P1, index: u32, context: *mut INFCONTEXT) -> windows_core::Result<()>
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupGetLineByIndexW(infhandle : *const core::ffi::c_void, section : windows_core::PCWSTR, index : u32, context : *mut INFCONTEXT) -> windows_core::BOOL);
    unsafe { SetupGetLineByIndexW(infhandle, section.param().abi(), index, context as _).ok() }
}
#[inline]
pub unsafe fn SetupGetLineCountA<P1>(infhandle: *const core::ffi::c_void, section: P1) -> i32
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupGetLineCountA(infhandle : *const core::ffi::c_void, section : windows_core::PCSTR) -> i32);
    unsafe { SetupGetLineCountA(infhandle, section.param().abi()) }
}
#[inline]
pub unsafe fn SetupGetLineCountW<P1>(infhandle: *const core::ffi::c_void, section: P1) -> i32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupGetLineCountW(infhandle : *const core::ffi::c_void, section : windows_core::PCWSTR) -> i32);
    unsafe { SetupGetLineCountW(infhandle, section.param().abi()) }
}
#[inline]
pub unsafe fn SetupGetLineTextA<P2, P3>(context: Option<*const INFCONTEXT>, infhandle: Option<*const core::ffi::c_void>, section: P2, key: P3, returnbuffer: Option<&mut [u8]>, requiredsize: Option<*mut u32>) -> windows_core::Result<()>
where
    P2: windows_core::Param<windows_core::PCSTR>,
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupGetLineTextA(context : *const INFCONTEXT, infhandle : *const core::ffi::c_void, section : windows_core::PCSTR, key : windows_core::PCSTR, returnbuffer : windows_core::PSTR, returnbuffersize : u32, requiredsize : *mut u32) -> windows_core::BOOL);
    unsafe { SetupGetLineTextA(context.unwrap_or(core::mem::zeroed()) as _, infhandle.unwrap_or(core::mem::zeroed()) as _, section.param().abi(), key.param().abi(), core::mem::transmute(returnbuffer.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), returnbuffer.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), requiredsize.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn SetupGetLineTextW<P2, P3>(context: Option<*const INFCONTEXT>, infhandle: Option<*const core::ffi::c_void>, section: P2, key: P3, returnbuffer: Option<&mut [u16]>, requiredsize: Option<*mut u32>) -> windows_core::Result<()>
where
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupGetLineTextW(context : *const INFCONTEXT, infhandle : *const core::ffi::c_void, section : windows_core::PCWSTR, key : windows_core::PCWSTR, returnbuffer : windows_core::PWSTR, returnbuffersize : u32, requiredsize : *mut u32) -> windows_core::BOOL);
    unsafe { SetupGetLineTextW(context.unwrap_or(core::mem::zeroed()) as _, infhandle.unwrap_or(core::mem::zeroed()) as _, section.param().abi(), key.param().abi(), core::mem::transmute(returnbuffer.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), returnbuffer.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), requiredsize.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn SetupGetMultiSzFieldA(context: *const INFCONTEXT, fieldindex: u32, returnbuffer: Option<&mut [u8]>, requiredsize: Option<*mut u32>) -> windows_core::Result<()> {
    windows_core::link!("setupapi.dll" "system" fn SetupGetMultiSzFieldA(context : *const INFCONTEXT, fieldindex : u32, returnbuffer : windows_core::PSTR, returnbuffersize : u32, requiredsize : *mut u32) -> windows_core::BOOL);
    unsafe { SetupGetMultiSzFieldA(context, fieldindex, core::mem::transmute(returnbuffer.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), returnbuffer.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), requiredsize.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn SetupGetMultiSzFieldW(context: *const INFCONTEXT, fieldindex: u32, returnbuffer: Option<&mut [u16]>, requiredsize: Option<*mut u32>) -> windows_core::Result<()> {
    windows_core::link!("setupapi.dll" "system" fn SetupGetMultiSzFieldW(context : *const INFCONTEXT, fieldindex : u32, returnbuffer : windows_core::PWSTR, returnbuffersize : u32, requiredsize : *mut u32) -> windows_core::BOOL);
    unsafe { SetupGetMultiSzFieldW(context, fieldindex, core::mem::transmute(returnbuffer.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), returnbuffer.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), requiredsize.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn SetupGetNonInteractiveMode() -> windows_core::BOOL {
    windows_core::link!("setupapi.dll" "system" fn SetupGetNonInteractiveMode() -> windows_core::BOOL);
    unsafe { SetupGetNonInteractiveMode() }
}
#[inline]
pub unsafe fn SetupGetSourceFileLocationA<P2>(infhandle: *const core::ffi::c_void, infcontext: Option<*const INFCONTEXT>, filename: P2, sourceid: *mut u32, returnbuffer: Option<&mut [u8]>, requiredsize: Option<*mut u32>) -> windows_core::Result<()>
where
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupGetSourceFileLocationA(infhandle : *const core::ffi::c_void, infcontext : *const INFCONTEXT, filename : windows_core::PCSTR, sourceid : *mut u32, returnbuffer : windows_core::PSTR, returnbuffersize : u32, requiredsize : *mut u32) -> windows_core::BOOL);
    unsafe { SetupGetSourceFileLocationA(infhandle, infcontext.unwrap_or(core::mem::zeroed()) as _, filename.param().abi(), sourceid as _, core::mem::transmute(returnbuffer.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), returnbuffer.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), requiredsize.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn SetupGetSourceFileLocationW<P2>(infhandle: *const core::ffi::c_void, infcontext: Option<*const INFCONTEXT>, filename: P2, sourceid: *mut u32, returnbuffer: Option<&mut [u16]>, requiredsize: Option<*mut u32>) -> windows_core::Result<()>
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupGetSourceFileLocationW(infhandle : *const core::ffi::c_void, infcontext : *const INFCONTEXT, filename : windows_core::PCWSTR, sourceid : *mut u32, returnbuffer : windows_core::PWSTR, returnbuffersize : u32, requiredsize : *mut u32) -> windows_core::BOOL);
    unsafe { SetupGetSourceFileLocationW(infhandle, infcontext.unwrap_or(core::mem::zeroed()) as _, filename.param().abi(), sourceid as _, core::mem::transmute(returnbuffer.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), returnbuffer.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), requiredsize.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn SetupGetSourceFileSizeA<P2, P3>(infhandle: *const core::ffi::c_void, infcontext: Option<*const INFCONTEXT>, filename: P2, section: P3, filesize: *mut u32, roundingfactor: u32) -> windows_core::Result<()>
where
    P2: windows_core::Param<windows_core::PCSTR>,
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupGetSourceFileSizeA(infhandle : *const core::ffi::c_void, infcontext : *const INFCONTEXT, filename : windows_core::PCSTR, section : windows_core::PCSTR, filesize : *mut u32, roundingfactor : u32) -> windows_core::BOOL);
    unsafe { SetupGetSourceFileSizeA(infhandle, infcontext.unwrap_or(core::mem::zeroed()) as _, filename.param().abi(), section.param().abi(), filesize as _, roundingfactor).ok() }
}
#[inline]
pub unsafe fn SetupGetSourceFileSizeW<P2, P3>(infhandle: *const core::ffi::c_void, infcontext: Option<*const INFCONTEXT>, filename: P2, section: P3, filesize: *mut u32, roundingfactor: u32) -> windows_core::Result<()>
where
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupGetSourceFileSizeW(infhandle : *const core::ffi::c_void, infcontext : *const INFCONTEXT, filename : windows_core::PCWSTR, section : windows_core::PCWSTR, filesize : *mut u32, roundingfactor : u32) -> windows_core::BOOL);
    unsafe { SetupGetSourceFileSizeW(infhandle, infcontext.unwrap_or(core::mem::zeroed()) as _, filename.param().abi(), section.param().abi(), filesize as _, roundingfactor).ok() }
}
#[inline]
pub unsafe fn SetupGetSourceInfoA(infhandle: *const core::ffi::c_void, sourceid: u32, infodesired: u32, returnbuffer: Option<&mut [u8]>, requiredsize: Option<*mut u32>) -> windows_core::Result<()> {
    windows_core::link!("setupapi.dll" "system" fn SetupGetSourceInfoA(infhandle : *const core::ffi::c_void, sourceid : u32, infodesired : u32, returnbuffer : windows_core::PSTR, returnbuffersize : u32, requiredsize : *mut u32) -> windows_core::BOOL);
    unsafe { SetupGetSourceInfoA(infhandle, sourceid, infodesired, core::mem::transmute(returnbuffer.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), returnbuffer.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), requiredsize.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn SetupGetSourceInfoW(infhandle: *const core::ffi::c_void, sourceid: u32, infodesired: u32, returnbuffer: Option<&mut [u16]>, requiredsize: Option<*mut u32>) -> windows_core::Result<()> {
    windows_core::link!("setupapi.dll" "system" fn SetupGetSourceInfoW(infhandle : *const core::ffi::c_void, sourceid : u32, infodesired : u32, returnbuffer : windows_core::PWSTR, returnbuffersize : u32, requiredsize : *mut u32) -> windows_core::BOOL);
    unsafe { SetupGetSourceInfoW(infhandle, sourceid, infodesired, core::mem::transmute(returnbuffer.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), returnbuffer.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), requiredsize.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn SetupGetStringFieldA(context: *const INFCONTEXT, fieldindex: u32, returnbuffer: Option<&mut [u8]>, requiredsize: Option<*mut u32>) -> windows_core::Result<()> {
    windows_core::link!("setupapi.dll" "system" fn SetupGetStringFieldA(context : *const INFCONTEXT, fieldindex : u32, returnbuffer : windows_core::PSTR, returnbuffersize : u32, requiredsize : *mut u32) -> windows_core::BOOL);
    unsafe { SetupGetStringFieldA(context, fieldindex, core::mem::transmute(returnbuffer.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), returnbuffer.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), requiredsize.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn SetupGetStringFieldW(context: *const INFCONTEXT, fieldindex: u32, returnbuffer: Option<&mut [u16]>, requiredsize: Option<*mut u32>) -> windows_core::Result<()> {
    windows_core::link!("setupapi.dll" "system" fn SetupGetStringFieldW(context : *const INFCONTEXT, fieldindex : u32, returnbuffer : windows_core::PWSTR, returnbuffersize : u32, requiredsize : *mut u32) -> windows_core::BOOL);
    unsafe { SetupGetStringFieldW(context, fieldindex, core::mem::transmute(returnbuffer.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), returnbuffer.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), requiredsize.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn SetupGetTargetPathA<P2>(infhandle: *const core::ffi::c_void, infcontext: Option<*const INFCONTEXT>, section: P2, returnbuffer: Option<&mut [u8]>, requiredsize: Option<*mut u32>) -> windows_core::Result<()>
where
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupGetTargetPathA(infhandle : *const core::ffi::c_void, infcontext : *const INFCONTEXT, section : windows_core::PCSTR, returnbuffer : windows_core::PSTR, returnbuffersize : u32, requiredsize : *mut u32) -> windows_core::BOOL);
    unsafe { SetupGetTargetPathA(infhandle, infcontext.unwrap_or(core::mem::zeroed()) as _, section.param().abi(), core::mem::transmute(returnbuffer.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), returnbuffer.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), requiredsize.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn SetupGetTargetPathW<P2>(infhandle: *const core::ffi::c_void, infcontext: Option<*const INFCONTEXT>, section: P2, returnbuffer: Option<&mut [u16]>, requiredsize: Option<*mut u32>) -> windows_core::Result<()>
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupGetTargetPathW(infhandle : *const core::ffi::c_void, infcontext : *const INFCONTEXT, section : windows_core::PCWSTR, returnbuffer : windows_core::PWSTR, returnbuffersize : u32, requiredsize : *mut u32) -> windows_core::BOOL);
    unsafe { SetupGetTargetPathW(infhandle, infcontext.unwrap_or(core::mem::zeroed()) as _, section.param().abi(), core::mem::transmute(returnbuffer.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), returnbuffer.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), requiredsize.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn SetupGetThreadLogToken() -> u64 {
    windows_core::link!("setupapi.dll" "system" fn SetupGetThreadLogToken() -> u64);
    unsafe { SetupGetThreadLogToken() }
}
#[inline]
pub unsafe fn SetupInitDefaultQueueCallback(ownerwindow: Option<super::super::Foundation::HWND>) -> *mut core::ffi::c_void {
    windows_core::link!("setupapi.dll" "system" fn SetupInitDefaultQueueCallback(ownerwindow : super::super::Foundation:: HWND) -> *mut core::ffi::c_void);
    unsafe { SetupInitDefaultQueueCallback(ownerwindow.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn SetupInitDefaultQueueCallbackEx(ownerwindow: Option<super::super::Foundation::HWND>, alternateprogresswindow: Option<super::super::Foundation::HWND>, progressmessage: u32, reserved1: Option<u32>, reserved2: Option<*const core::ffi::c_void>) -> *mut core::ffi::c_void {
    windows_core::link!("setupapi.dll" "system" fn SetupInitDefaultQueueCallbackEx(ownerwindow : super::super::Foundation:: HWND, alternateprogresswindow : super::super::Foundation:: HWND, progressmessage : u32, reserved1 : u32, reserved2 : *const core::ffi::c_void) -> *mut core::ffi::c_void);
    unsafe { SetupInitDefaultQueueCallbackEx(ownerwindow.unwrap_or(core::mem::zeroed()) as _, alternateprogresswindow.unwrap_or(core::mem::zeroed()) as _, progressmessage, reserved1.unwrap_or(core::mem::zeroed()) as _, reserved2.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn SetupInitializeFileLogA<P0>(logfilename: P0, flags: u32) -> *mut core::ffi::c_void
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupInitializeFileLogA(logfilename : windows_core::PCSTR, flags : u32) -> *mut core::ffi::c_void);
    unsafe { SetupInitializeFileLogA(logfilename.param().abi(), flags) }
}
#[inline]
pub unsafe fn SetupInitializeFileLogW<P0>(logfilename: P0, flags: u32) -> *mut core::ffi::c_void
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupInitializeFileLogW(logfilename : windows_core::PCWSTR, flags : u32) -> *mut core::ffi::c_void);
    unsafe { SetupInitializeFileLogW(logfilename.param().abi(), flags) }
}
#[inline]
pub unsafe fn SetupInstallFileA<P2, P3, P4>(infhandle: Option<*const core::ffi::c_void>, infcontext: Option<*const INFCONTEXT>, sourcefile: P2, sourcepathroot: P3, destinationname: P4, copystyle: SP_COPY_STYLE, copymsghandler: PSP_FILE_CALLBACK_A, context: Option<*const core::ffi::c_void>) -> windows_core::Result<()>
where
    P2: windows_core::Param<windows_core::PCSTR>,
    P3: windows_core::Param<windows_core::PCSTR>,
    P4: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupInstallFileA(infhandle : *const core::ffi::c_void, infcontext : *const INFCONTEXT, sourcefile : windows_core::PCSTR, sourcepathroot : windows_core::PCSTR, destinationname : windows_core::PCSTR, copystyle : SP_COPY_STYLE, copymsghandler : PSP_FILE_CALLBACK_A, context : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe { SetupInstallFileA(infhandle.unwrap_or(core::mem::zeroed()) as _, infcontext.unwrap_or(core::mem::zeroed()) as _, sourcefile.param().abi(), sourcepathroot.param().abi(), destinationname.param().abi(), copystyle, copymsghandler, context.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn SetupInstallFileExA<P2, P3, P4>(infhandle: Option<*const core::ffi::c_void>, infcontext: Option<*const INFCONTEXT>, sourcefile: P2, sourcepathroot: P3, destinationname: P4, copystyle: SP_COPY_STYLE, copymsghandler: PSP_FILE_CALLBACK_A, context: Option<*const core::ffi::c_void>, filewasinuse: *mut windows_core::BOOL) -> windows_core::Result<()>
where
    P2: windows_core::Param<windows_core::PCSTR>,
    P3: windows_core::Param<windows_core::PCSTR>,
    P4: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupInstallFileExA(infhandle : *const core::ffi::c_void, infcontext : *const INFCONTEXT, sourcefile : windows_core::PCSTR, sourcepathroot : windows_core::PCSTR, destinationname : windows_core::PCSTR, copystyle : SP_COPY_STYLE, copymsghandler : PSP_FILE_CALLBACK_A, context : *const core::ffi::c_void, filewasinuse : *mut windows_core::BOOL) -> windows_core::BOOL);
    unsafe { SetupInstallFileExA(infhandle.unwrap_or(core::mem::zeroed()) as _, infcontext.unwrap_or(core::mem::zeroed()) as _, sourcefile.param().abi(), sourcepathroot.param().abi(), destinationname.param().abi(), copystyle, copymsghandler, context.unwrap_or(core::mem::zeroed()) as _, filewasinuse as _).ok() }
}
#[inline]
pub unsafe fn SetupInstallFileExW<P2, P3, P4>(infhandle: Option<*const core::ffi::c_void>, infcontext: Option<*const INFCONTEXT>, sourcefile: P2, sourcepathroot: P3, destinationname: P4, copystyle: SP_COPY_STYLE, copymsghandler: PSP_FILE_CALLBACK_W, context: Option<*const core::ffi::c_void>, filewasinuse: *mut windows_core::BOOL) -> windows_core::Result<()>
where
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
    P4: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupInstallFileExW(infhandle : *const core::ffi::c_void, infcontext : *const INFCONTEXT, sourcefile : windows_core::PCWSTR, sourcepathroot : windows_core::PCWSTR, destinationname : windows_core::PCWSTR, copystyle : SP_COPY_STYLE, copymsghandler : PSP_FILE_CALLBACK_W, context : *const core::ffi::c_void, filewasinuse : *mut windows_core::BOOL) -> windows_core::BOOL);
    unsafe { SetupInstallFileExW(infhandle.unwrap_or(core::mem::zeroed()) as _, infcontext.unwrap_or(core::mem::zeroed()) as _, sourcefile.param().abi(), sourcepathroot.param().abi(), destinationname.param().abi(), copystyle, copymsghandler, context.unwrap_or(core::mem::zeroed()) as _, filewasinuse as _).ok() }
}
#[inline]
pub unsafe fn SetupInstallFileW<P2, P3, P4>(infhandle: Option<*const core::ffi::c_void>, infcontext: Option<*const INFCONTEXT>, sourcefile: P2, sourcepathroot: P3, destinationname: P4, copystyle: SP_COPY_STYLE, copymsghandler: PSP_FILE_CALLBACK_W, context: Option<*const core::ffi::c_void>) -> windows_core::Result<()>
where
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
    P4: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupInstallFileW(infhandle : *const core::ffi::c_void, infcontext : *const INFCONTEXT, sourcefile : windows_core::PCWSTR, sourcepathroot : windows_core::PCWSTR, destinationname : windows_core::PCWSTR, copystyle : SP_COPY_STYLE, copymsghandler : PSP_FILE_CALLBACK_W, context : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe { SetupInstallFileW(infhandle.unwrap_or(core::mem::zeroed()) as _, infcontext.unwrap_or(core::mem::zeroed()) as _, sourcefile.param().abi(), sourcepathroot.param().abi(), destinationname.param().abi(), copystyle, copymsghandler, context.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn SetupInstallFilesFromInfSectionA<P3, P4>(infhandle: *const core::ffi::c_void, layoutinfhandle: Option<*const core::ffi::c_void>, filequeue: *const core::ffi::c_void, sectionname: P3, sourcerootpath: P4, copyflags: u32) -> windows_core::Result<()>
where
    P3: windows_core::Param<windows_core::PCSTR>,
    P4: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupInstallFilesFromInfSectionA(infhandle : *const core::ffi::c_void, layoutinfhandle : *const core::ffi::c_void, filequeue : *const core::ffi::c_void, sectionname : windows_core::PCSTR, sourcerootpath : windows_core::PCSTR, copyflags : u32) -> windows_core::BOOL);
    unsafe { SetupInstallFilesFromInfSectionA(infhandle, layoutinfhandle.unwrap_or(core::mem::zeroed()) as _, filequeue, sectionname.param().abi(), sourcerootpath.param().abi(), copyflags).ok() }
}
#[inline]
pub unsafe fn SetupInstallFilesFromInfSectionW<P3, P4>(infhandle: *const core::ffi::c_void, layoutinfhandle: Option<*const core::ffi::c_void>, filequeue: *const core::ffi::c_void, sectionname: P3, sourcerootpath: P4, copyflags: u32) -> windows_core::Result<()>
where
    P3: windows_core::Param<windows_core::PCWSTR>,
    P4: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupInstallFilesFromInfSectionW(infhandle : *const core::ffi::c_void, layoutinfhandle : *const core::ffi::c_void, filequeue : *const core::ffi::c_void, sectionname : windows_core::PCWSTR, sourcerootpath : windows_core::PCWSTR, copyflags : u32) -> windows_core::BOOL);
    unsafe { SetupInstallFilesFromInfSectionW(infhandle, layoutinfhandle.unwrap_or(core::mem::zeroed()) as _, filequeue, sectionname.param().abi(), sourcerootpath.param().abi(), copyflags).ok() }
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn SetupInstallFromInfSectionA<P2, P5>(owner: Option<super::super::Foundation::HWND>, infhandle: *const core::ffi::c_void, sectionname: P2, flags: u32, relativekeyroot: Option<super::super::System::Registry::HKEY>, sourcerootpath: P5, copyflags: u32, msghandler: PSP_FILE_CALLBACK_A, context: Option<*const core::ffi::c_void>, deviceinfoset: Option<HDEVINFO>, deviceinfodata: Option<*const SP_DEVINFO_DATA>) -> windows_core::Result<()>
where
    P2: windows_core::Param<windows_core::PCSTR>,
    P5: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupInstallFromInfSectionA(owner : super::super::Foundation:: HWND, infhandle : *const core::ffi::c_void, sectionname : windows_core::PCSTR, flags : u32, relativekeyroot : super::super::System::Registry:: HKEY, sourcerootpath : windows_core::PCSTR, copyflags : u32, msghandler : PSP_FILE_CALLBACK_A, context : *const core::ffi::c_void, deviceinfoset : HDEVINFO, deviceinfodata : *const SP_DEVINFO_DATA) -> windows_core::BOOL);
    unsafe { SetupInstallFromInfSectionA(owner.unwrap_or(core::mem::zeroed()) as _, infhandle, sectionname.param().abi(), flags, relativekeyroot.unwrap_or(core::mem::zeroed()) as _, sourcerootpath.param().abi(), copyflags, msghandler, context.unwrap_or(core::mem::zeroed()) as _, deviceinfoset.unwrap_or(core::mem::zeroed()) as _, deviceinfodata.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn SetupInstallFromInfSectionW<P2, P5>(owner: Option<super::super::Foundation::HWND>, infhandle: *const core::ffi::c_void, sectionname: P2, flags: u32, relativekeyroot: Option<super::super::System::Registry::HKEY>, sourcerootpath: P5, copyflags: u32, msghandler: PSP_FILE_CALLBACK_W, context: Option<*const core::ffi::c_void>, deviceinfoset: Option<HDEVINFO>, deviceinfodata: Option<*const SP_DEVINFO_DATA>) -> windows_core::Result<()>
where
    P2: windows_core::Param<windows_core::PCWSTR>,
    P5: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupInstallFromInfSectionW(owner : super::super::Foundation:: HWND, infhandle : *const core::ffi::c_void, sectionname : windows_core::PCWSTR, flags : u32, relativekeyroot : super::super::System::Registry:: HKEY, sourcerootpath : windows_core::PCWSTR, copyflags : u32, msghandler : PSP_FILE_CALLBACK_W, context : *const core::ffi::c_void, deviceinfoset : HDEVINFO, deviceinfodata : *const SP_DEVINFO_DATA) -> windows_core::BOOL);
    unsafe { SetupInstallFromInfSectionW(owner.unwrap_or(core::mem::zeroed()) as _, infhandle, sectionname.param().abi(), flags, relativekeyroot.unwrap_or(core::mem::zeroed()) as _, sourcerootpath.param().abi(), copyflags, msghandler, context.unwrap_or(core::mem::zeroed()) as _, deviceinfoset.unwrap_or(core::mem::zeroed()) as _, deviceinfodata.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn SetupInstallServicesFromInfSectionA<P1>(infhandle: *const core::ffi::c_void, sectionname: P1, flags: SPSVCINST_FLAGS) -> windows_core::Result<()>
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupInstallServicesFromInfSectionA(infhandle : *const core::ffi::c_void, sectionname : windows_core::PCSTR, flags : SPSVCINST_FLAGS) -> windows_core::BOOL);
    unsafe { SetupInstallServicesFromInfSectionA(infhandle, sectionname.param().abi(), flags).ok() }
}
#[inline]
pub unsafe fn SetupInstallServicesFromInfSectionExA<P1>(infhandle: *const core::ffi::c_void, sectionname: P1, flags: SPSVCINST_FLAGS, deviceinfoset: Option<HDEVINFO>, deviceinfodata: Option<*const SP_DEVINFO_DATA>, reserved1: Option<*const core::ffi::c_void>, reserved2: Option<*const core::ffi::c_void>) -> windows_core::Result<()>
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupInstallServicesFromInfSectionExA(infhandle : *const core::ffi::c_void, sectionname : windows_core::PCSTR, flags : SPSVCINST_FLAGS, deviceinfoset : HDEVINFO, deviceinfodata : *const SP_DEVINFO_DATA, reserved1 : *const core::ffi::c_void, reserved2 : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe { SetupInstallServicesFromInfSectionExA(infhandle, sectionname.param().abi(), flags, deviceinfoset.unwrap_or(core::mem::zeroed()) as _, deviceinfodata.unwrap_or(core::mem::zeroed()) as _, reserved1.unwrap_or(core::mem::zeroed()) as _, reserved2.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn SetupInstallServicesFromInfSectionExW<P1>(infhandle: *const core::ffi::c_void, sectionname: P1, flags: SPSVCINST_FLAGS, deviceinfoset: Option<HDEVINFO>, deviceinfodata: Option<*const SP_DEVINFO_DATA>, reserved1: Option<*const core::ffi::c_void>, reserved2: Option<*const core::ffi::c_void>) -> windows_core::Result<()>
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupInstallServicesFromInfSectionExW(infhandle : *const core::ffi::c_void, sectionname : windows_core::PCWSTR, flags : SPSVCINST_FLAGS, deviceinfoset : HDEVINFO, deviceinfodata : *const SP_DEVINFO_DATA, reserved1 : *const core::ffi::c_void, reserved2 : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe { SetupInstallServicesFromInfSectionExW(infhandle, sectionname.param().abi(), flags, deviceinfoset.unwrap_or(core::mem::zeroed()) as _, deviceinfodata.unwrap_or(core::mem::zeroed()) as _, reserved1.unwrap_or(core::mem::zeroed()) as _, reserved2.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn SetupInstallServicesFromInfSectionW<P1>(infhandle: *const core::ffi::c_void, sectionname: P1, flags: SPSVCINST_FLAGS) -> windows_core::Result<()>
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupInstallServicesFromInfSectionW(infhandle : *const core::ffi::c_void, sectionname : windows_core::PCWSTR, flags : SPSVCINST_FLAGS) -> windows_core::BOOL);
    unsafe { SetupInstallServicesFromInfSectionW(infhandle, sectionname.param().abi(), flags).ok() }
}
#[inline]
pub unsafe fn SetupIterateCabinetA<P0>(cabinetfile: P0, reserved: Option<u32>, msghandler: PSP_FILE_CALLBACK_A, context: *const core::ffi::c_void) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupIterateCabinetA(cabinetfile : windows_core::PCSTR, reserved : u32, msghandler : PSP_FILE_CALLBACK_A, context : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe { SetupIterateCabinetA(cabinetfile.param().abi(), reserved.unwrap_or(core::mem::zeroed()) as _, msghandler, context).ok() }
}
#[inline]
pub unsafe fn SetupIterateCabinetW<P0>(cabinetfile: P0, reserved: Option<u32>, msghandler: PSP_FILE_CALLBACK_W, context: *const core::ffi::c_void) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupIterateCabinetW(cabinetfile : windows_core::PCWSTR, reserved : u32, msghandler : PSP_FILE_CALLBACK_W, context : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe { SetupIterateCabinetW(cabinetfile.param().abi(), reserved.unwrap_or(core::mem::zeroed()) as _, msghandler, context).ok() }
}
#[inline]
pub unsafe fn SetupLogErrorA<P0>(messagestring: P0, severity: u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupLogErrorA(messagestring : windows_core::PCSTR, severity : u32) -> windows_core::BOOL);
    unsafe { SetupLogErrorA(messagestring.param().abi(), severity).ok() }
}
#[inline]
pub unsafe fn SetupLogErrorW<P0>(messagestring: P0, severity: u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupLogErrorW(messagestring : windows_core::PCWSTR, severity : u32) -> windows_core::BOOL);
    unsafe { SetupLogErrorW(messagestring.param().abi(), severity).ok() }
}
#[inline]
pub unsafe fn SetupLogFileA<P1, P2, P3, P5, P6, P7>(fileloghandle: *const core::ffi::c_void, logsectionname: P1, sourcefilename: P2, targetfilename: P3, checksum: u32, disktagfile: P5, diskdescription: P6, otherinfo: P7, flags: u32) -> windows_core::Result<()>
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
    P3: windows_core::Param<windows_core::PCSTR>,
    P5: windows_core::Param<windows_core::PCSTR>,
    P6: windows_core::Param<windows_core::PCSTR>,
    P7: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupLogFileA(fileloghandle : *const core::ffi::c_void, logsectionname : windows_core::PCSTR, sourcefilename : windows_core::PCSTR, targetfilename : windows_core::PCSTR, checksum : u32, disktagfile : windows_core::PCSTR, diskdescription : windows_core::PCSTR, otherinfo : windows_core::PCSTR, flags : u32) -> windows_core::BOOL);
    unsafe { SetupLogFileA(fileloghandle, logsectionname.param().abi(), sourcefilename.param().abi(), targetfilename.param().abi(), checksum, disktagfile.param().abi(), diskdescription.param().abi(), otherinfo.param().abi(), flags).ok() }
}
#[inline]
pub unsafe fn SetupLogFileW<P1, P2, P3, P5, P6, P7>(fileloghandle: *const core::ffi::c_void, logsectionname: P1, sourcefilename: P2, targetfilename: P3, checksum: u32, disktagfile: P5, diskdescription: P6, otherinfo: P7, flags: u32) -> windows_core::Result<()>
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
    P5: windows_core::Param<windows_core::PCWSTR>,
    P6: windows_core::Param<windows_core::PCWSTR>,
    P7: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupLogFileW(fileloghandle : *const core::ffi::c_void, logsectionname : windows_core::PCWSTR, sourcefilename : windows_core::PCWSTR, targetfilename : windows_core::PCWSTR, checksum : u32, disktagfile : windows_core::PCWSTR, diskdescription : windows_core::PCWSTR, otherinfo : windows_core::PCWSTR, flags : u32) -> windows_core::BOOL);
    unsafe { SetupLogFileW(fileloghandle, logsectionname.param().abi(), sourcefilename.param().abi(), targetfilename.param().abi(), checksum, disktagfile.param().abi(), diskdescription.param().abi(), otherinfo.param().abi(), flags).ok() }
}
#[inline]
pub unsafe fn SetupOpenAppendInfFileA<P0>(filename: P0, infhandle: *const core::ffi::c_void, errorline: Option<*mut u32>) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupOpenAppendInfFileA(filename : windows_core::PCSTR, infhandle : *const core::ffi::c_void, errorline : *mut u32) -> windows_core::BOOL);
    unsafe { SetupOpenAppendInfFileA(filename.param().abi(), infhandle, errorline.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn SetupOpenAppendInfFileW<P0>(filename: P0, infhandle: *const core::ffi::c_void, errorline: Option<*mut u32>) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupOpenAppendInfFileW(filename : windows_core::PCWSTR, infhandle : *const core::ffi::c_void, errorline : *mut u32) -> windows_core::BOOL);
    unsafe { SetupOpenAppendInfFileW(filename.param().abi(), infhandle, errorline.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn SetupOpenFileQueue() -> *mut core::ffi::c_void {
    windows_core::link!("setupapi.dll" "system" fn SetupOpenFileQueue() -> *mut core::ffi::c_void);
    unsafe { SetupOpenFileQueue() }
}
#[inline]
pub unsafe fn SetupOpenInfFileA<P0, P1>(filename: P0, infclass: P1, infstyle: INF_STYLE, errorline: Option<*mut u32>) -> *mut core::ffi::c_void
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupOpenInfFileA(filename : windows_core::PCSTR, infclass : windows_core::PCSTR, infstyle : INF_STYLE, errorline : *mut u32) -> *mut core::ffi::c_void);
    unsafe { SetupOpenInfFileA(filename.param().abi(), infclass.param().abi(), infstyle, errorline.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn SetupOpenInfFileW<P0, P1>(filename: P0, infclass: P1, infstyle: INF_STYLE, errorline: Option<*mut u32>) -> *mut core::ffi::c_void
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupOpenInfFileW(filename : windows_core::PCWSTR, infclass : windows_core::PCWSTR, infstyle : INF_STYLE, errorline : *mut u32) -> *mut core::ffi::c_void);
    unsafe { SetupOpenInfFileW(filename.param().abi(), infclass.param().abi(), infstyle, errorline.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn SetupOpenLog(erase: bool) -> windows_core::Result<()> {
    windows_core::link!("setupapi.dll" "system" fn SetupOpenLog(erase : windows_core::BOOL) -> windows_core::BOOL);
    unsafe { SetupOpenLog(erase.into()).ok() }
}
#[inline]
pub unsafe fn SetupOpenMasterInf() -> *mut core::ffi::c_void {
    windows_core::link!("setupapi.dll" "system" fn SetupOpenMasterInf() -> *mut core::ffi::c_void);
    unsafe { SetupOpenMasterInf() }
}
#[inline]
pub unsafe fn SetupPrepareQueueForRestoreA<P1>(queuehandle: *const core::ffi::c_void, backuppath: P1, restoreflags: u32) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupPrepareQueueForRestoreA(queuehandle : *const core::ffi::c_void, backuppath : windows_core::PCSTR, restoreflags : u32) -> windows_core::BOOL);
    unsafe { SetupPrepareQueueForRestoreA(queuehandle, backuppath.param().abi(), restoreflags) }
}
#[inline]
pub unsafe fn SetupPrepareQueueForRestoreW<P1>(queuehandle: *const core::ffi::c_void, backuppath: P1, restoreflags: u32) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupPrepareQueueForRestoreW(queuehandle : *const core::ffi::c_void, backuppath : windows_core::PCWSTR, restoreflags : u32) -> windows_core::BOOL);
    unsafe { SetupPrepareQueueForRestoreW(queuehandle, backuppath.param().abi(), restoreflags) }
}
#[inline]
pub unsafe fn SetupPromptForDiskA<P1, P2, P3, P4, P5>(hwndparent: super::super::Foundation::HWND, dialogtitle: P1, diskname: P2, pathtosource: P3, filesought: P4, tagfile: P5, diskpromptstyle: u32, pathbuffer: Option<&mut [u8]>, pathrequiredsize: Option<*mut u32>) -> u32
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
    P3: windows_core::Param<windows_core::PCSTR>,
    P4: windows_core::Param<windows_core::PCSTR>,
    P5: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupPromptForDiskA(hwndparent : super::super::Foundation:: HWND, dialogtitle : windows_core::PCSTR, diskname : windows_core::PCSTR, pathtosource : windows_core::PCSTR, filesought : windows_core::PCSTR, tagfile : windows_core::PCSTR, diskpromptstyle : u32, pathbuffer : windows_core::PSTR, pathbuffersize : u32, pathrequiredsize : *mut u32) -> u32);
    unsafe { SetupPromptForDiskA(hwndparent, dialogtitle.param().abi(), diskname.param().abi(), pathtosource.param().abi(), filesought.param().abi(), tagfile.param().abi(), diskpromptstyle, core::mem::transmute(pathbuffer.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pathbuffer.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), pathrequiredsize.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn SetupPromptForDiskW<P1, P2, P3, P4, P5>(hwndparent: super::super::Foundation::HWND, dialogtitle: P1, diskname: P2, pathtosource: P3, filesought: P4, tagfile: P5, diskpromptstyle: u32, pathbuffer: Option<&mut [u16]>, pathrequiredsize: Option<*mut u32>) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
    P4: windows_core::Param<windows_core::PCWSTR>,
    P5: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupPromptForDiskW(hwndparent : super::super::Foundation:: HWND, dialogtitle : windows_core::PCWSTR, diskname : windows_core::PCWSTR, pathtosource : windows_core::PCWSTR, filesought : windows_core::PCWSTR, tagfile : windows_core::PCWSTR, diskpromptstyle : u32, pathbuffer : windows_core::PWSTR, pathbuffersize : u32, pathrequiredsize : *mut u32) -> u32);
    unsafe { SetupPromptForDiskW(hwndparent, dialogtitle.param().abi(), diskname.param().abi(), pathtosource.param().abi(), filesought.param().abi(), tagfile.param().abi(), diskpromptstyle, core::mem::transmute(pathbuffer.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pathbuffer.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), pathrequiredsize.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn SetupPromptReboot(filequeue: Option<*const core::ffi::c_void>, owner: Option<super::super::Foundation::HWND>, scanonly: bool) -> i32 {
    windows_core::link!("setupapi.dll" "system" fn SetupPromptReboot(filequeue : *const core::ffi::c_void, owner : super::super::Foundation:: HWND, scanonly : windows_core::BOOL) -> i32);
    unsafe { SetupPromptReboot(filequeue.unwrap_or(core::mem::zeroed()) as _, owner.unwrap_or(core::mem::zeroed()) as _, scanonly.into()) }
}
#[inline]
pub unsafe fn SetupQueryDrivesInDiskSpaceListA(diskspace: *const core::ffi::c_void, returnbuffer: Option<&mut [u8]>, requiredsize: Option<*mut u32>) -> windows_core::Result<()> {
    windows_core::link!("setupapi.dll" "system" fn SetupQueryDrivesInDiskSpaceListA(diskspace : *const core::ffi::c_void, returnbuffer : windows_core::PSTR, returnbuffersize : u32, requiredsize : *mut u32) -> windows_core::BOOL);
    unsafe { SetupQueryDrivesInDiskSpaceListA(diskspace, core::mem::transmute(returnbuffer.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), returnbuffer.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), requiredsize.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn SetupQueryDrivesInDiskSpaceListW(diskspace: *const core::ffi::c_void, returnbuffer: Option<&mut [u16]>, requiredsize: Option<*mut u32>) -> windows_core::Result<()> {
    windows_core::link!("setupapi.dll" "system" fn SetupQueryDrivesInDiskSpaceListW(diskspace : *const core::ffi::c_void, returnbuffer : windows_core::PWSTR, returnbuffersize : u32, requiredsize : *mut u32) -> windows_core::BOOL);
    unsafe { SetupQueryDrivesInDiskSpaceListW(diskspace, core::mem::transmute(returnbuffer.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), returnbuffer.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), requiredsize.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn SetupQueryFileLogA<P1, P2>(fileloghandle: *const core::ffi::c_void, logsectionname: P1, targetfilename: P2, desiredinfo: SetupFileLogInfo, dataout: Option<&mut [u8]>, requiredsize: Option<*mut u32>) -> windows_core::Result<()>
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupQueryFileLogA(fileloghandle : *const core::ffi::c_void, logsectionname : windows_core::PCSTR, targetfilename : windows_core::PCSTR, desiredinfo : SetupFileLogInfo, dataout : windows_core::PSTR, returnbuffersize : u32, requiredsize : *mut u32) -> windows_core::BOOL);
    unsafe { SetupQueryFileLogA(fileloghandle, logsectionname.param().abi(), targetfilename.param().abi(), desiredinfo, core::mem::transmute(dataout.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), dataout.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), requiredsize.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn SetupQueryFileLogW<P1, P2>(fileloghandle: *const core::ffi::c_void, logsectionname: P1, targetfilename: P2, desiredinfo: SetupFileLogInfo, dataout: Option<&mut [u16]>, requiredsize: Option<*mut u32>) -> windows_core::Result<()>
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupQueryFileLogW(fileloghandle : *const core::ffi::c_void, logsectionname : windows_core::PCWSTR, targetfilename : windows_core::PCWSTR, desiredinfo : SetupFileLogInfo, dataout : windows_core::PWSTR, returnbuffersize : u32, requiredsize : *mut u32) -> windows_core::BOOL);
    unsafe { SetupQueryFileLogW(fileloghandle, logsectionname.param().abi(), targetfilename.param().abi(), desiredinfo, core::mem::transmute(dataout.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), dataout.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), requiredsize.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn SetupQueryInfFileInformationA(infinformation: *const SP_INF_INFORMATION, infindex: u32, returnbuffer: Option<&mut [u8]>, requiredsize: Option<*mut u32>) -> windows_core::Result<()> {
    windows_core::link!("setupapi.dll" "system" fn SetupQueryInfFileInformationA(infinformation : *const SP_INF_INFORMATION, infindex : u32, returnbuffer : windows_core::PSTR, returnbuffersize : u32, requiredsize : *mut u32) -> windows_core::BOOL);
    unsafe { SetupQueryInfFileInformationA(infinformation, infindex, core::mem::transmute(returnbuffer.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), returnbuffer.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), requiredsize.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn SetupQueryInfFileInformationW(infinformation: *const SP_INF_INFORMATION, infindex: u32, returnbuffer: Option<&mut [u16]>, requiredsize: Option<*mut u32>) -> windows_core::Result<()> {
    windows_core::link!("setupapi.dll" "system" fn SetupQueryInfFileInformationW(infinformation : *const SP_INF_INFORMATION, infindex : u32, returnbuffer : windows_core::PWSTR, returnbuffersize : u32, requiredsize : *mut u32) -> windows_core::BOOL);
    unsafe { SetupQueryInfFileInformationW(infinformation, infindex, core::mem::transmute(returnbuffer.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), returnbuffer.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), requiredsize.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[cfg(all(feature = "Win32_System_Diagnostics_Debug", feature = "Win32_System_SystemInformation"))]
#[inline]
pub unsafe fn SetupQueryInfOriginalFileInformationA(infinformation: *const SP_INF_INFORMATION, infindex: u32, alternateplatforminfo: Option<*const SP_ALTPLATFORM_INFO_V2>, originalfileinfo: *mut SP_ORIGINAL_FILE_INFO_A) -> windows_core::Result<()> {
    windows_core::link!("setupapi.dll" "system" fn SetupQueryInfOriginalFileInformationA(infinformation : *const SP_INF_INFORMATION, infindex : u32, alternateplatforminfo : *const SP_ALTPLATFORM_INFO_V2, originalfileinfo : *mut SP_ORIGINAL_FILE_INFO_A) -> windows_core::BOOL);
    unsafe { SetupQueryInfOriginalFileInformationA(infinformation, infindex, alternateplatforminfo.unwrap_or(core::mem::zeroed()) as _, originalfileinfo as _).ok() }
}
#[cfg(all(feature = "Win32_System_Diagnostics_Debug", feature = "Win32_System_SystemInformation"))]
#[inline]
pub unsafe fn SetupQueryInfOriginalFileInformationW(infinformation: *const SP_INF_INFORMATION, infindex: u32, alternateplatforminfo: Option<*const SP_ALTPLATFORM_INFO_V2>, originalfileinfo: *mut SP_ORIGINAL_FILE_INFO_W) -> windows_core::Result<()> {
    windows_core::link!("setupapi.dll" "system" fn SetupQueryInfOriginalFileInformationW(infinformation : *const SP_INF_INFORMATION, infindex : u32, alternateplatforminfo : *const SP_ALTPLATFORM_INFO_V2, originalfileinfo : *mut SP_ORIGINAL_FILE_INFO_W) -> windows_core::BOOL);
    unsafe { SetupQueryInfOriginalFileInformationW(infinformation, infindex, alternateplatforminfo.unwrap_or(core::mem::zeroed()) as _, originalfileinfo as _).ok() }
}
#[inline]
pub unsafe fn SetupQueryInfVersionInformationA<P2>(infinformation: *const SP_INF_INFORMATION, infindex: u32, key: P2, returnbuffer: Option<&mut [u8]>, requiredsize: Option<*mut u32>) -> windows_core::Result<()>
where
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupQueryInfVersionInformationA(infinformation : *const SP_INF_INFORMATION, infindex : u32, key : windows_core::PCSTR, returnbuffer : windows_core::PSTR, returnbuffersize : u32, requiredsize : *mut u32) -> windows_core::BOOL);
    unsafe { SetupQueryInfVersionInformationA(infinformation, infindex, key.param().abi(), core::mem::transmute(returnbuffer.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), returnbuffer.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), requiredsize.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn SetupQueryInfVersionInformationW<P2>(infinformation: *const SP_INF_INFORMATION, infindex: u32, key: P2, returnbuffer: Option<&mut [u16]>, requiredsize: Option<*mut u32>) -> windows_core::Result<()>
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupQueryInfVersionInformationW(infinformation : *const SP_INF_INFORMATION, infindex : u32, key : windows_core::PCWSTR, returnbuffer : windows_core::PWSTR, returnbuffersize : u32, requiredsize : *mut u32) -> windows_core::BOOL);
    unsafe { SetupQueryInfVersionInformationW(infinformation, infindex, key.param().abi(), core::mem::transmute(returnbuffer.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), returnbuffer.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), requiredsize.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn SetupQuerySourceListA(flags: u32, list: *mut *mut windows_core::PCSTR, count: *mut u32) -> windows_core::Result<()> {
    windows_core::link!("setupapi.dll" "system" fn SetupQuerySourceListA(flags : u32, list : *mut *mut windows_core::PCSTR, count : *mut u32) -> windows_core::BOOL);
    unsafe { SetupQuerySourceListA(flags, list as _, count as _).ok() }
}
#[inline]
pub unsafe fn SetupQuerySourceListW(flags: u32, list: *mut *mut windows_core::PCWSTR, count: *mut u32) -> windows_core::Result<()> {
    windows_core::link!("setupapi.dll" "system" fn SetupQuerySourceListW(flags : u32, list : *mut *mut windows_core::PCWSTR, count : *mut u32) -> windows_core::BOOL);
    unsafe { SetupQuerySourceListW(flags, list as _, count as _).ok() }
}
#[inline]
pub unsafe fn SetupQuerySpaceRequiredOnDriveA<P1>(diskspace: *const core::ffi::c_void, drivespec: P1, spacerequired: *mut i64, reserved1: Option<*const core::ffi::c_void>, reserved2: Option<u32>) -> windows_core::Result<()>
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupQuerySpaceRequiredOnDriveA(diskspace : *const core::ffi::c_void, drivespec : windows_core::PCSTR, spacerequired : *mut i64, reserved1 : *const core::ffi::c_void, reserved2 : u32) -> windows_core::BOOL);
    unsafe { SetupQuerySpaceRequiredOnDriveA(diskspace, drivespec.param().abi(), spacerequired as _, reserved1.unwrap_or(core::mem::zeroed()) as _, reserved2.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn SetupQuerySpaceRequiredOnDriveW<P1>(diskspace: *const core::ffi::c_void, drivespec: P1, spacerequired: *mut i64, reserved1: Option<*const core::ffi::c_void>, reserved2: Option<u32>) -> windows_core::Result<()>
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupQuerySpaceRequiredOnDriveW(diskspace : *const core::ffi::c_void, drivespec : windows_core::PCWSTR, spacerequired : *mut i64, reserved1 : *const core::ffi::c_void, reserved2 : u32) -> windows_core::BOOL);
    unsafe { SetupQuerySpaceRequiredOnDriveW(diskspace, drivespec.param().abi(), spacerequired as _, reserved1.unwrap_or(core::mem::zeroed()) as _, reserved2.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn SetupQueueCopyA<P1, P2, P3, P4, P5, P6, P7>(queuehandle: *const core::ffi::c_void, sourcerootpath: P1, sourcepath: P2, sourcefilename: P3, sourcedescription: P4, sourcetagfile: P5, targetdirectory: P6, targetfilename: P7, copystyle: u32) -> windows_core::Result<()>
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
    P3: windows_core::Param<windows_core::PCSTR>,
    P4: windows_core::Param<windows_core::PCSTR>,
    P5: windows_core::Param<windows_core::PCSTR>,
    P6: windows_core::Param<windows_core::PCSTR>,
    P7: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupQueueCopyA(queuehandle : *const core::ffi::c_void, sourcerootpath : windows_core::PCSTR, sourcepath : windows_core::PCSTR, sourcefilename : windows_core::PCSTR, sourcedescription : windows_core::PCSTR, sourcetagfile : windows_core::PCSTR, targetdirectory : windows_core::PCSTR, targetfilename : windows_core::PCSTR, copystyle : u32) -> windows_core::BOOL);
    unsafe { SetupQueueCopyA(queuehandle, sourcerootpath.param().abi(), sourcepath.param().abi(), sourcefilename.param().abi(), sourcedescription.param().abi(), sourcetagfile.param().abi(), targetdirectory.param().abi(), targetfilename.param().abi(), copystyle).ok() }
}
#[inline]
pub unsafe fn SetupQueueCopyIndirectA(copyparams: *const SP_FILE_COPY_PARAMS_A) -> windows_core::Result<()> {
    windows_core::link!("setupapi.dll" "system" fn SetupQueueCopyIndirectA(copyparams : *const SP_FILE_COPY_PARAMS_A) -> windows_core::BOOL);
    unsafe { SetupQueueCopyIndirectA(copyparams).ok() }
}
#[inline]
pub unsafe fn SetupQueueCopyIndirectW(copyparams: *const SP_FILE_COPY_PARAMS_W) -> windows_core::Result<()> {
    windows_core::link!("setupapi.dll" "system" fn SetupQueueCopyIndirectW(copyparams : *const SP_FILE_COPY_PARAMS_W) -> windows_core::BOOL);
    unsafe { SetupQueueCopyIndirectW(copyparams).ok() }
}
#[inline]
pub unsafe fn SetupQueueCopySectionA<P1, P4>(queuehandle: *const core::ffi::c_void, sourcerootpath: P1, infhandle: *const core::ffi::c_void, listinfhandle: Option<*const core::ffi::c_void>, section: P4, copystyle: u32) -> windows_core::Result<()>
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P4: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupQueueCopySectionA(queuehandle : *const core::ffi::c_void, sourcerootpath : windows_core::PCSTR, infhandle : *const core::ffi::c_void, listinfhandle : *const core::ffi::c_void, section : windows_core::PCSTR, copystyle : u32) -> windows_core::BOOL);
    unsafe { SetupQueueCopySectionA(queuehandle, sourcerootpath.param().abi(), infhandle, listinfhandle.unwrap_or(core::mem::zeroed()) as _, section.param().abi(), copystyle).ok() }
}
#[inline]
pub unsafe fn SetupQueueCopySectionW<P1, P4>(queuehandle: *const core::ffi::c_void, sourcerootpath: P1, infhandle: *const core::ffi::c_void, listinfhandle: Option<*const core::ffi::c_void>, section: P4, copystyle: u32) -> windows_core::Result<()>
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P4: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupQueueCopySectionW(queuehandle : *const core::ffi::c_void, sourcerootpath : windows_core::PCWSTR, infhandle : *const core::ffi::c_void, listinfhandle : *const core::ffi::c_void, section : windows_core::PCWSTR, copystyle : u32) -> windows_core::BOOL);
    unsafe { SetupQueueCopySectionW(queuehandle, sourcerootpath.param().abi(), infhandle, listinfhandle.unwrap_or(core::mem::zeroed()) as _, section.param().abi(), copystyle).ok() }
}
#[inline]
pub unsafe fn SetupQueueCopyW<P1, P2, P3, P4, P5, P6, P7>(queuehandle: *const core::ffi::c_void, sourcerootpath: P1, sourcepath: P2, sourcefilename: P3, sourcedescription: P4, sourcetagfile: P5, targetdirectory: P6, targetfilename: P7, copystyle: u32) -> windows_core::Result<()>
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
    P4: windows_core::Param<windows_core::PCWSTR>,
    P5: windows_core::Param<windows_core::PCWSTR>,
    P6: windows_core::Param<windows_core::PCWSTR>,
    P7: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupQueueCopyW(queuehandle : *const core::ffi::c_void, sourcerootpath : windows_core::PCWSTR, sourcepath : windows_core::PCWSTR, sourcefilename : windows_core::PCWSTR, sourcedescription : windows_core::PCWSTR, sourcetagfile : windows_core::PCWSTR, targetdirectory : windows_core::PCWSTR, targetfilename : windows_core::PCWSTR, copystyle : u32) -> windows_core::BOOL);
    unsafe { SetupQueueCopyW(queuehandle, sourcerootpath.param().abi(), sourcepath.param().abi(), sourcefilename.param().abi(), sourcedescription.param().abi(), sourcetagfile.param().abi(), targetdirectory.param().abi(), targetfilename.param().abi(), copystyle).ok() }
}
#[inline]
pub unsafe fn SetupQueueDefaultCopyA<P2, P3, P4>(queuehandle: *const core::ffi::c_void, infhandle: *const core::ffi::c_void, sourcerootpath: P2, sourcefilename: P3, targetfilename: P4, copystyle: u32) -> windows_core::Result<()>
where
    P2: windows_core::Param<windows_core::PCSTR>,
    P3: windows_core::Param<windows_core::PCSTR>,
    P4: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupQueueDefaultCopyA(queuehandle : *const core::ffi::c_void, infhandle : *const core::ffi::c_void, sourcerootpath : windows_core::PCSTR, sourcefilename : windows_core::PCSTR, targetfilename : windows_core::PCSTR, copystyle : u32) -> windows_core::BOOL);
    unsafe { SetupQueueDefaultCopyA(queuehandle, infhandle, sourcerootpath.param().abi(), sourcefilename.param().abi(), targetfilename.param().abi(), copystyle).ok() }
}
#[inline]
pub unsafe fn SetupQueueDefaultCopyW<P2, P3, P4>(queuehandle: *const core::ffi::c_void, infhandle: *const core::ffi::c_void, sourcerootpath: P2, sourcefilename: P3, targetfilename: P4, copystyle: u32) -> windows_core::Result<()>
where
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
    P4: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupQueueDefaultCopyW(queuehandle : *const core::ffi::c_void, infhandle : *const core::ffi::c_void, sourcerootpath : windows_core::PCWSTR, sourcefilename : windows_core::PCWSTR, targetfilename : windows_core::PCWSTR, copystyle : u32) -> windows_core::BOOL);
    unsafe { SetupQueueDefaultCopyW(queuehandle, infhandle, sourcerootpath.param().abi(), sourcefilename.param().abi(), targetfilename.param().abi(), copystyle).ok() }
}
#[inline]
pub unsafe fn SetupQueueDeleteA<P1, P2>(queuehandle: *const core::ffi::c_void, pathpart1: P1, pathpart2: P2) -> windows_core::Result<()>
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupQueueDeleteA(queuehandle : *const core::ffi::c_void, pathpart1 : windows_core::PCSTR, pathpart2 : windows_core::PCSTR) -> windows_core::BOOL);
    unsafe { SetupQueueDeleteA(queuehandle, pathpart1.param().abi(), pathpart2.param().abi()).ok() }
}
#[inline]
pub unsafe fn SetupQueueDeleteSectionA<P3>(queuehandle: *const core::ffi::c_void, infhandle: *const core::ffi::c_void, listinfhandle: Option<*const core::ffi::c_void>, section: P3) -> windows_core::Result<()>
where
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupQueueDeleteSectionA(queuehandle : *const core::ffi::c_void, infhandle : *const core::ffi::c_void, listinfhandle : *const core::ffi::c_void, section : windows_core::PCSTR) -> windows_core::BOOL);
    unsafe { SetupQueueDeleteSectionA(queuehandle, infhandle, listinfhandle.unwrap_or(core::mem::zeroed()) as _, section.param().abi()).ok() }
}
#[inline]
pub unsafe fn SetupQueueDeleteSectionW<P3>(queuehandle: *const core::ffi::c_void, infhandle: *const core::ffi::c_void, listinfhandle: Option<*const core::ffi::c_void>, section: P3) -> windows_core::Result<()>
where
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupQueueDeleteSectionW(queuehandle : *const core::ffi::c_void, infhandle : *const core::ffi::c_void, listinfhandle : *const core::ffi::c_void, section : windows_core::PCWSTR) -> windows_core::BOOL);
    unsafe { SetupQueueDeleteSectionW(queuehandle, infhandle, listinfhandle.unwrap_or(core::mem::zeroed()) as _, section.param().abi()).ok() }
}
#[inline]
pub unsafe fn SetupQueueDeleteW<P1, P2>(queuehandle: *const core::ffi::c_void, pathpart1: P1, pathpart2: P2) -> windows_core::Result<()>
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupQueueDeleteW(queuehandle : *const core::ffi::c_void, pathpart1 : windows_core::PCWSTR, pathpart2 : windows_core::PCWSTR) -> windows_core::BOOL);
    unsafe { SetupQueueDeleteW(queuehandle, pathpart1.param().abi(), pathpart2.param().abi()).ok() }
}
#[inline]
pub unsafe fn SetupQueueRenameA<P1, P2, P3, P4>(queuehandle: *const core::ffi::c_void, sourcepath: P1, sourcefilename: P2, targetpath: P3, targetfilename: P4) -> windows_core::Result<()>
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
    P3: windows_core::Param<windows_core::PCSTR>,
    P4: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupQueueRenameA(queuehandle : *const core::ffi::c_void, sourcepath : windows_core::PCSTR, sourcefilename : windows_core::PCSTR, targetpath : windows_core::PCSTR, targetfilename : windows_core::PCSTR) -> windows_core::BOOL);
    unsafe { SetupQueueRenameA(queuehandle, sourcepath.param().abi(), sourcefilename.param().abi(), targetpath.param().abi(), targetfilename.param().abi()).ok() }
}
#[inline]
pub unsafe fn SetupQueueRenameSectionA<P3>(queuehandle: *const core::ffi::c_void, infhandle: *const core::ffi::c_void, listinfhandle: Option<*const core::ffi::c_void>, section: P3) -> windows_core::Result<()>
where
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupQueueRenameSectionA(queuehandle : *const core::ffi::c_void, infhandle : *const core::ffi::c_void, listinfhandle : *const core::ffi::c_void, section : windows_core::PCSTR) -> windows_core::BOOL);
    unsafe { SetupQueueRenameSectionA(queuehandle, infhandle, listinfhandle.unwrap_or(core::mem::zeroed()) as _, section.param().abi()).ok() }
}
#[inline]
pub unsafe fn SetupQueueRenameSectionW<P3>(queuehandle: *const core::ffi::c_void, infhandle: *const core::ffi::c_void, listinfhandle: Option<*const core::ffi::c_void>, section: P3) -> windows_core::Result<()>
where
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupQueueRenameSectionW(queuehandle : *const core::ffi::c_void, infhandle : *const core::ffi::c_void, listinfhandle : *const core::ffi::c_void, section : windows_core::PCWSTR) -> windows_core::BOOL);
    unsafe { SetupQueueRenameSectionW(queuehandle, infhandle, listinfhandle.unwrap_or(core::mem::zeroed()) as _, section.param().abi()).ok() }
}
#[inline]
pub unsafe fn SetupQueueRenameW<P1, P2, P3, P4>(queuehandle: *const core::ffi::c_void, sourcepath: P1, sourcefilename: P2, targetpath: P3, targetfilename: P4) -> windows_core::Result<()>
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
    P4: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupQueueRenameW(queuehandle : *const core::ffi::c_void, sourcepath : windows_core::PCWSTR, sourcefilename : windows_core::PCWSTR, targetpath : windows_core::PCWSTR, targetfilename : windows_core::PCWSTR) -> windows_core::BOOL);
    unsafe { SetupQueueRenameW(queuehandle, sourcepath.param().abi(), sourcefilename.param().abi(), targetpath.param().abi(), targetfilename.param().abi()).ok() }
}
#[inline]
pub unsafe fn SetupRemoveFileLogEntryA<P1, P2>(fileloghandle: *const core::ffi::c_void, logsectionname: P1, targetfilename: P2) -> windows_core::Result<()>
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupRemoveFileLogEntryA(fileloghandle : *const core::ffi::c_void, logsectionname : windows_core::PCSTR, targetfilename : windows_core::PCSTR) -> windows_core::BOOL);
    unsafe { SetupRemoveFileLogEntryA(fileloghandle, logsectionname.param().abi(), targetfilename.param().abi()).ok() }
}
#[inline]
pub unsafe fn SetupRemoveFileLogEntryW<P1, P2>(fileloghandle: *const core::ffi::c_void, logsectionname: P1, targetfilename: P2) -> windows_core::Result<()>
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupRemoveFileLogEntryW(fileloghandle : *const core::ffi::c_void, logsectionname : windows_core::PCWSTR, targetfilename : windows_core::PCWSTR) -> windows_core::BOOL);
    unsafe { SetupRemoveFileLogEntryW(fileloghandle, logsectionname.param().abi(), targetfilename.param().abi()).ok() }
}
#[inline]
pub unsafe fn SetupRemoveFromDiskSpaceListA<P1>(diskspace: *const core::ffi::c_void, targetfilespec: P1, operation: SETUP_FILE_OPERATION, reserved1: Option<*const core::ffi::c_void>, reserved2: Option<u32>) -> windows_core::Result<()>
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupRemoveFromDiskSpaceListA(diskspace : *const core::ffi::c_void, targetfilespec : windows_core::PCSTR, operation : SETUP_FILE_OPERATION, reserved1 : *const core::ffi::c_void, reserved2 : u32) -> windows_core::BOOL);
    unsafe { SetupRemoveFromDiskSpaceListA(diskspace, targetfilespec.param().abi(), operation, reserved1.unwrap_or(core::mem::zeroed()) as _, reserved2.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn SetupRemoveFromDiskSpaceListW<P1>(diskspace: *const core::ffi::c_void, targetfilespec: P1, operation: SETUP_FILE_OPERATION, reserved1: Option<*const core::ffi::c_void>, reserved2: Option<u32>) -> windows_core::Result<()>
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupRemoveFromDiskSpaceListW(diskspace : *const core::ffi::c_void, targetfilespec : windows_core::PCWSTR, operation : SETUP_FILE_OPERATION, reserved1 : *const core::ffi::c_void, reserved2 : u32) -> windows_core::BOOL);
    unsafe { SetupRemoveFromDiskSpaceListW(diskspace, targetfilespec.param().abi(), operation, reserved1.unwrap_or(core::mem::zeroed()) as _, reserved2.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn SetupRemoveFromSourceListA<P1>(flags: u32, source: P1) -> windows_core::Result<()>
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupRemoveFromSourceListA(flags : u32, source : windows_core::PCSTR) -> windows_core::BOOL);
    unsafe { SetupRemoveFromSourceListA(flags, source.param().abi()).ok() }
}
#[inline]
pub unsafe fn SetupRemoveFromSourceListW<P1>(flags: u32, source: P1) -> windows_core::Result<()>
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupRemoveFromSourceListW(flags : u32, source : windows_core::PCWSTR) -> windows_core::BOOL);
    unsafe { SetupRemoveFromSourceListW(flags, source.param().abi()).ok() }
}
#[inline]
pub unsafe fn SetupRemoveInstallSectionFromDiskSpaceListA<P3>(diskspace: *const core::ffi::c_void, infhandle: *const core::ffi::c_void, layoutinfhandle: Option<*const core::ffi::c_void>, sectionname: P3, reserved1: Option<*const core::ffi::c_void>, reserved2: Option<u32>) -> windows_core::Result<()>
where
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupRemoveInstallSectionFromDiskSpaceListA(diskspace : *const core::ffi::c_void, infhandle : *const core::ffi::c_void, layoutinfhandle : *const core::ffi::c_void, sectionname : windows_core::PCSTR, reserved1 : *const core::ffi::c_void, reserved2 : u32) -> windows_core::BOOL);
    unsafe { SetupRemoveInstallSectionFromDiskSpaceListA(diskspace, infhandle, layoutinfhandle.unwrap_or(core::mem::zeroed()) as _, sectionname.param().abi(), reserved1.unwrap_or(core::mem::zeroed()) as _, reserved2.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn SetupRemoveInstallSectionFromDiskSpaceListW<P3>(diskspace: *const core::ffi::c_void, infhandle: *const core::ffi::c_void, layoutinfhandle: Option<*const core::ffi::c_void>, sectionname: P3, reserved1: Option<*const core::ffi::c_void>, reserved2: Option<u32>) -> windows_core::Result<()>
where
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupRemoveInstallSectionFromDiskSpaceListW(diskspace : *const core::ffi::c_void, infhandle : *const core::ffi::c_void, layoutinfhandle : *const core::ffi::c_void, sectionname : windows_core::PCWSTR, reserved1 : *const core::ffi::c_void, reserved2 : u32) -> windows_core::BOOL);
    unsafe { SetupRemoveInstallSectionFromDiskSpaceListW(diskspace, infhandle, layoutinfhandle.unwrap_or(core::mem::zeroed()) as _, sectionname.param().abi(), reserved1.unwrap_or(core::mem::zeroed()) as _, reserved2.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn SetupRemoveSectionFromDiskSpaceListA<P3>(diskspace: *const core::ffi::c_void, infhandle: *const core::ffi::c_void, listinfhandle: Option<*const core::ffi::c_void>, sectionname: P3, operation: SETUP_FILE_OPERATION, reserved1: Option<*const core::ffi::c_void>, reserved2: Option<u32>) -> windows_core::Result<()>
where
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupRemoveSectionFromDiskSpaceListA(diskspace : *const core::ffi::c_void, infhandle : *const core::ffi::c_void, listinfhandle : *const core::ffi::c_void, sectionname : windows_core::PCSTR, operation : SETUP_FILE_OPERATION, reserved1 : *const core::ffi::c_void, reserved2 : u32) -> windows_core::BOOL);
    unsafe { SetupRemoveSectionFromDiskSpaceListA(diskspace, infhandle, listinfhandle.unwrap_or(core::mem::zeroed()) as _, sectionname.param().abi(), operation, reserved1.unwrap_or(core::mem::zeroed()) as _, reserved2.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn SetupRemoveSectionFromDiskSpaceListW<P3>(diskspace: *const core::ffi::c_void, infhandle: *const core::ffi::c_void, listinfhandle: Option<*const core::ffi::c_void>, sectionname: P3, operation: SETUP_FILE_OPERATION, reserved1: Option<*const core::ffi::c_void>, reserved2: Option<u32>) -> windows_core::Result<()>
where
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupRemoveSectionFromDiskSpaceListW(diskspace : *const core::ffi::c_void, infhandle : *const core::ffi::c_void, listinfhandle : *const core::ffi::c_void, sectionname : windows_core::PCWSTR, operation : SETUP_FILE_OPERATION, reserved1 : *const core::ffi::c_void, reserved2 : u32) -> windows_core::BOOL);
    unsafe { SetupRemoveSectionFromDiskSpaceListW(diskspace, infhandle, listinfhandle.unwrap_or(core::mem::zeroed()) as _, sectionname.param().abi(), operation, reserved1.unwrap_or(core::mem::zeroed()) as _, reserved2.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn SetupRenameErrorA<P1, P2, P3>(hwndparent: super::super::Foundation::HWND, dialogtitle: P1, sourcefile: P2, targetfile: P3, win32errorcode: u32, style: u32) -> u32
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupRenameErrorA(hwndparent : super::super::Foundation:: HWND, dialogtitle : windows_core::PCSTR, sourcefile : windows_core::PCSTR, targetfile : windows_core::PCSTR, win32errorcode : u32, style : u32) -> u32);
    unsafe { SetupRenameErrorA(hwndparent, dialogtitle.param().abi(), sourcefile.param().abi(), targetfile.param().abi(), win32errorcode, style) }
}
#[inline]
pub unsafe fn SetupRenameErrorW<P1, P2, P3>(hwndparent: super::super::Foundation::HWND, dialogtitle: P1, sourcefile: P2, targetfile: P3, win32errorcode: u32, style: u32) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupRenameErrorW(hwndparent : super::super::Foundation:: HWND, dialogtitle : windows_core::PCWSTR, sourcefile : windows_core::PCWSTR, targetfile : windows_core::PCWSTR, win32errorcode : u32, style : u32) -> u32);
    unsafe { SetupRenameErrorW(hwndparent, dialogtitle.param().abi(), sourcefile.param().abi(), targetfile.param().abi(), win32errorcode, style) }
}
#[inline]
pub unsafe fn SetupScanFileQueueA(filequeue: *const core::ffi::c_void, flags: SETUPSCANFILEQUEUE_FLAGS, window: Option<super::super::Foundation::HWND>, callbackroutine: PSP_FILE_CALLBACK_A, callbackcontext: Option<*const core::ffi::c_void>, result: *mut u32) -> windows_core::Result<()> {
    windows_core::link!("setupapi.dll" "system" fn SetupScanFileQueueA(filequeue : *const core::ffi::c_void, flags : SETUPSCANFILEQUEUE_FLAGS, window : super::super::Foundation:: HWND, callbackroutine : PSP_FILE_CALLBACK_A, callbackcontext : *const core::ffi::c_void, result : *mut u32) -> windows_core::BOOL);
    unsafe { SetupScanFileQueueA(filequeue, flags, window.unwrap_or(core::mem::zeroed()) as _, callbackroutine, callbackcontext.unwrap_or(core::mem::zeroed()) as _, result as _).ok() }
}
#[inline]
pub unsafe fn SetupScanFileQueueW(filequeue: *const core::ffi::c_void, flags: SETUPSCANFILEQUEUE_FLAGS, window: Option<super::super::Foundation::HWND>, callbackroutine: PSP_FILE_CALLBACK_W, callbackcontext: Option<*const core::ffi::c_void>, result: *mut u32) -> windows_core::Result<()> {
    windows_core::link!("setupapi.dll" "system" fn SetupScanFileQueueW(filequeue : *const core::ffi::c_void, flags : SETUPSCANFILEQUEUE_FLAGS, window : super::super::Foundation:: HWND, callbackroutine : PSP_FILE_CALLBACK_W, callbackcontext : *const core::ffi::c_void, result : *mut u32) -> windows_core::BOOL);
    unsafe { SetupScanFileQueueW(filequeue, flags, window.unwrap_or(core::mem::zeroed()) as _, callbackroutine, callbackcontext.unwrap_or(core::mem::zeroed()) as _, result as _).ok() }
}
#[inline]
pub unsafe fn SetupSetDirectoryIdA<P2>(infhandle: *const core::ffi::c_void, id: u32, directory: P2) -> windows_core::Result<()>
where
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupSetDirectoryIdA(infhandle : *const core::ffi::c_void, id : u32, directory : windows_core::PCSTR) -> windows_core::BOOL);
    unsafe { SetupSetDirectoryIdA(infhandle, id, directory.param().abi()).ok() }
}
#[inline]
pub unsafe fn SetupSetDirectoryIdExA<P2>(infhandle: *const core::ffi::c_void, id: u32, directory: P2, flags: u32, reserved1: Option<u32>, reserved2: Option<*const core::ffi::c_void>) -> windows_core::Result<()>
where
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupSetDirectoryIdExA(infhandle : *const core::ffi::c_void, id : u32, directory : windows_core::PCSTR, flags : u32, reserved1 : u32, reserved2 : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe { SetupSetDirectoryIdExA(infhandle, id, directory.param().abi(), flags, reserved1.unwrap_or(core::mem::zeroed()) as _, reserved2.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn SetupSetDirectoryIdExW<P2>(infhandle: *const core::ffi::c_void, id: u32, directory: P2, flags: u32, reserved1: Option<u32>, reserved2: Option<*const core::ffi::c_void>) -> windows_core::Result<()>
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupSetDirectoryIdExW(infhandle : *const core::ffi::c_void, id : u32, directory : windows_core::PCWSTR, flags : u32, reserved1 : u32, reserved2 : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe { SetupSetDirectoryIdExW(infhandle, id, directory.param().abi(), flags, reserved1.unwrap_or(core::mem::zeroed()) as _, reserved2.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn SetupSetDirectoryIdW<P2>(infhandle: *const core::ffi::c_void, id: u32, directory: P2) -> windows_core::Result<()>
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupSetDirectoryIdW(infhandle : *const core::ffi::c_void, id : u32, directory : windows_core::PCWSTR) -> windows_core::BOOL);
    unsafe { SetupSetDirectoryIdW(infhandle, id, directory.param().abi()).ok() }
}
#[cfg(all(feature = "Win32_System_Diagnostics_Debug", feature = "Win32_System_SystemInformation"))]
#[inline]
pub unsafe fn SetupSetFileQueueAlternatePlatformA<P2>(queuehandle: *const core::ffi::c_void, alternateplatforminfo: Option<*const SP_ALTPLATFORM_INFO_V2>, alternatedefaultcatalogfile: P2) -> windows_core::Result<()>
where
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupSetFileQueueAlternatePlatformA(queuehandle : *const core::ffi::c_void, alternateplatforminfo : *const SP_ALTPLATFORM_INFO_V2, alternatedefaultcatalogfile : windows_core::PCSTR) -> windows_core::BOOL);
    unsafe { SetupSetFileQueueAlternatePlatformA(queuehandle, alternateplatforminfo.unwrap_or(core::mem::zeroed()) as _, alternatedefaultcatalogfile.param().abi()).ok() }
}
#[cfg(all(feature = "Win32_System_Diagnostics_Debug", feature = "Win32_System_SystemInformation"))]
#[inline]
pub unsafe fn SetupSetFileQueueAlternatePlatformW<P2>(queuehandle: *const core::ffi::c_void, alternateplatforminfo: Option<*const SP_ALTPLATFORM_INFO_V2>, alternatedefaultcatalogfile: P2) -> windows_core::Result<()>
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupSetFileQueueAlternatePlatformW(queuehandle : *const core::ffi::c_void, alternateplatforminfo : *const SP_ALTPLATFORM_INFO_V2, alternatedefaultcatalogfile : windows_core::PCWSTR) -> windows_core::BOOL);
    unsafe { SetupSetFileQueueAlternatePlatformW(queuehandle, alternateplatforminfo.unwrap_or(core::mem::zeroed()) as _, alternatedefaultcatalogfile.param().abi()).ok() }
}
#[inline]
pub unsafe fn SetupSetFileQueueFlags(filequeue: *const core::ffi::c_void, flagmask: u32, flags: u32) -> windows_core::Result<()> {
    windows_core::link!("setupapi.dll" "system" fn SetupSetFileQueueFlags(filequeue : *const core::ffi::c_void, flagmask : u32, flags : u32) -> windows_core::BOOL);
    unsafe { SetupSetFileQueueFlags(filequeue, flagmask, flags).ok() }
}
#[inline]
pub unsafe fn SetupSetNonInteractiveMode(noninteractiveflag: bool) -> windows_core::BOOL {
    windows_core::link!("setupapi.dll" "system" fn SetupSetNonInteractiveMode(noninteractiveflag : windows_core::BOOL) -> windows_core::BOOL);
    unsafe { SetupSetNonInteractiveMode(noninteractiveflag.into()) }
}
#[inline]
pub unsafe fn SetupSetPlatformPathOverrideA<P0>(r#override: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupSetPlatformPathOverrideA(r#override : windows_core::PCSTR) -> windows_core::BOOL);
    unsafe { SetupSetPlatformPathOverrideA(r#override.param().abi()).ok() }
}
#[inline]
pub unsafe fn SetupSetPlatformPathOverrideW<P0>(r#override: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupSetPlatformPathOverrideW(r#override : windows_core::PCWSTR) -> windows_core::BOOL);
    unsafe { SetupSetPlatformPathOverrideW(r#override.param().abi()).ok() }
}
#[inline]
pub unsafe fn SetupSetSourceListA(flags: u32, sourcelist: &[windows_core::PCSTR]) -> windows_core::Result<()> {
    windows_core::link!("setupapi.dll" "system" fn SetupSetSourceListA(flags : u32, sourcelist : *const windows_core::PCSTR, sourcecount : u32) -> windows_core::BOOL);
    unsafe { SetupSetSourceListA(flags, core::mem::transmute(sourcelist.as_ptr()), sourcelist.len().try_into().unwrap()).ok() }
}
#[inline]
pub unsafe fn SetupSetSourceListW(flags: u32, sourcelist: &[windows_core::PCWSTR]) -> windows_core::Result<()> {
    windows_core::link!("setupapi.dll" "system" fn SetupSetSourceListW(flags : u32, sourcelist : *const windows_core::PCWSTR, sourcecount : u32) -> windows_core::BOOL);
    unsafe { SetupSetSourceListW(flags, core::mem::transmute(sourcelist.as_ptr()), sourcelist.len().try_into().unwrap()).ok() }
}
#[inline]
pub unsafe fn SetupSetThreadLogToken(logtoken: u64) {
    windows_core::link!("setupapi.dll" "system" fn SetupSetThreadLogToken(logtoken : u64));
    unsafe { SetupSetThreadLogToken(logtoken) }
}
#[inline]
pub unsafe fn SetupTermDefaultQueueCallback(context: *const core::ffi::c_void) {
    windows_core::link!("setupapi.dll" "system" fn SetupTermDefaultQueueCallback(context : *const core::ffi::c_void));
    unsafe { SetupTermDefaultQueueCallback(context) }
}
#[inline]
pub unsafe fn SetupTerminateFileLog(fileloghandle: *const core::ffi::c_void) -> windows_core::Result<()> {
    windows_core::link!("setupapi.dll" "system" fn SetupTerminateFileLog(fileloghandle : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe { SetupTerminateFileLog(fileloghandle).ok() }
}
#[inline]
pub unsafe fn SetupUninstallNewlyCopiedInfs(filequeue: *const core::ffi::c_void, flags: u32, reserved: Option<*const core::ffi::c_void>) -> windows_core::Result<()> {
    windows_core::link!("setupapi.dll" "system" fn SetupUninstallNewlyCopiedInfs(filequeue : *const core::ffi::c_void, flags : u32, reserved : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe { SetupUninstallNewlyCopiedInfs(filequeue, flags, reserved.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn SetupUninstallOEMInfA<P0>(inffilename: P0, flags: u32, reserved: Option<*const core::ffi::c_void>) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupUninstallOEMInfA(inffilename : windows_core::PCSTR, flags : u32, reserved : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe { SetupUninstallOEMInfA(inffilename.param().abi(), flags, reserved.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn SetupUninstallOEMInfW<P0>(inffilename: P0, flags: u32, reserved: Option<*const core::ffi::c_void>) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupUninstallOEMInfW(inffilename : windows_core::PCWSTR, flags : u32, reserved : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe { SetupUninstallOEMInfW(inffilename.param().abi(), flags, reserved.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "Win32_System_Diagnostics_Debug", feature = "Win32_System_SystemInformation"))]
#[inline]
pub unsafe fn SetupVerifyInfFileA<P0>(infname: P0, altplatforminfo: Option<*const SP_ALTPLATFORM_INFO_V2>, infsignerinfo: *mut SP_INF_SIGNER_INFO_V2_A) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupVerifyInfFileA(infname : windows_core::PCSTR, altplatforminfo : *const SP_ALTPLATFORM_INFO_V2, infsignerinfo : *mut SP_INF_SIGNER_INFO_V2_A) -> windows_core::BOOL);
    unsafe { SetupVerifyInfFileA(infname.param().abi(), altplatforminfo.unwrap_or(core::mem::zeroed()) as _, infsignerinfo as _) }
}
#[cfg(all(feature = "Win32_System_Diagnostics_Debug", feature = "Win32_System_SystemInformation"))]
#[inline]
pub unsafe fn SetupVerifyInfFileW<P0>(infname: P0, altplatforminfo: Option<*const SP_ALTPLATFORM_INFO_V2>, infsignerinfo: *mut SP_INF_SIGNER_INFO_V2_W) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn SetupVerifyInfFileW(infname : windows_core::PCWSTR, altplatforminfo : *const SP_ALTPLATFORM_INFO_V2, infsignerinfo : *mut SP_INF_SIGNER_INFO_V2_W) -> windows_core::BOOL);
    unsafe { SetupVerifyInfFileW(infname.param().abi(), altplatforminfo.unwrap_or(core::mem::zeroed()) as _, infsignerinfo as _) }
}
#[inline]
pub unsafe fn SetupWriteTextLog<P3>(logtoken: u64, category: u32, flags: u32, messagestr: P3)
where
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("setupapi.dll" "C" fn SetupWriteTextLog(logtoken : u64, category : u32, flags : u32, messagestr : windows_core::PCSTR));
    unsafe { SetupWriteTextLog(logtoken, category, flags, messagestr.param().abi()) }
}
#[inline]
pub unsafe fn SetupWriteTextLogError<P4>(logtoken: u64, category: u32, logflags: u32, error: u32, messagestr: P4)
where
    P4: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("setupapi.dll" "C" fn SetupWriteTextLogError(logtoken : u64, category : u32, logflags : u32, error : u32, messagestr : windows_core::PCSTR));
    unsafe { SetupWriteTextLogError(logtoken, category, logflags, error, messagestr.param().abi()) }
}
#[inline]
pub unsafe fn SetupWriteTextLogInfLine(logtoken: u64, flags: u32, infhandle: *const core::ffi::c_void, context: *const INFCONTEXT) {
    windows_core::link!("setupapi.dll" "system" fn SetupWriteTextLogInfLine(logtoken : u64, flags : u32, infhandle : *const core::ffi::c_void, context : *const INFCONTEXT));
    unsafe { SetupWriteTextLogInfLine(logtoken, flags, infhandle, context) }
}
#[inline]
pub unsafe fn UpdateDriverForPlugAndPlayDevicesA<P1, P2>(hwndparent: Option<super::super::Foundation::HWND>, hardwareid: P1, fullinfpath: P2, installflags: UPDATEDRIVERFORPLUGANDPLAYDEVICES_FLAGS, brebootrequired: Option<*mut windows_core::BOOL>) -> windows_core::Result<()>
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("newdev.dll" "system" fn UpdateDriverForPlugAndPlayDevicesA(hwndparent : super::super::Foundation:: HWND, hardwareid : windows_core::PCSTR, fullinfpath : windows_core::PCSTR, installflags : UPDATEDRIVERFORPLUGANDPLAYDEVICES_FLAGS, brebootrequired : *mut windows_core::BOOL) -> windows_core::BOOL);
    unsafe { UpdateDriverForPlugAndPlayDevicesA(hwndparent.unwrap_or(core::mem::zeroed()) as _, hardwareid.param().abi(), fullinfpath.param().abi(), installflags, brebootrequired.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn UpdateDriverForPlugAndPlayDevicesW<P1, P2>(hwndparent: Option<super::super::Foundation::HWND>, hardwareid: P1, fullinfpath: P2, installflags: UPDATEDRIVERFORPLUGANDPLAYDEVICES_FLAGS, brebootrequired: Option<*mut windows_core::BOOL>) -> windows_core::Result<()>
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("newdev.dll" "system" fn UpdateDriverForPlugAndPlayDevicesW(hwndparent : super::super::Foundation:: HWND, hardwareid : windows_core::PCWSTR, fullinfpath : windows_core::PCWSTR, installflags : UPDATEDRIVERFORPLUGANDPLAYDEVICES_FLAGS, brebootrequired : *mut windows_core::BOOL) -> windows_core::BOOL);
    unsafe { UpdateDriverForPlugAndPlayDevicesW(hwndparent.unwrap_or(core::mem::zeroed()) as _, hardwareid.param().abi(), fullinfpath.param().abi(), installflags, brebootrequired.unwrap_or(core::mem::zeroed()) as _).ok() }
}
pub const ALLOC_LOG_CONF: CM_LOG_CONF = CM_LOG_CONF(2u32);
pub const BASIC_LOG_CONF: CM_LOG_CONF = CM_LOG_CONF(0u32);
pub const BOOT_LOG_CONF: CM_LOG_CONF = CM_LOG_CONF(3u32);
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct BUSNUMBER_DES {
    pub BUSD_Count: u32,
    pub BUSD_Type: u32,
    pub BUSD_Flags: u32,
    pub BUSD_Alloc_Base: u32,
    pub BUSD_Alloc_End: u32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct BUSNUMBER_RANGE {
    pub BUSR_Min: u32,
    pub BUSR_Max: u32,
    pub BUSR_nBusNumbers: u32,
    pub BUSR_Flags: u32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct BUSNUMBER_RESOURCE {
    pub BusNumber_Header: BUSNUMBER_DES,
    pub BusNumber_Data: [BUSNUMBER_RANGE; 1],
}
impl Default for BUSNUMBER_RESOURCE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy, Default)]
pub struct CABINET_INFO_A {
    pub CabinetPath: windows_core::PCSTR,
    pub CabinetFile: windows_core::PCSTR,
    pub DiskName: windows_core::PCSTR,
    pub SetId: u16,
    pub CabinetNumber: u16,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CABINET_INFO_A {
    pub CabinetPath: windows_core::PCSTR,
    pub CabinetFile: windows_core::PCSTR,
    pub DiskName: windows_core::PCSTR,
    pub SetId: u16,
    pub CabinetNumber: u16,
}
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy, Default)]
pub struct CABINET_INFO_W {
    pub CabinetPath: windows_core::PCWSTR,
    pub CabinetFile: windows_core::PCWSTR,
    pub DiskName: windows_core::PCWSTR,
    pub SetId: u16,
    pub CabinetNumber: u16,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CABINET_INFO_W {
    pub CabinetPath: windows_core::PCWSTR,
    pub CabinetFile: windows_core::PCWSTR,
    pub DiskName: windows_core::PCWSTR,
    pub SetId: u16,
    pub CabinetNumber: u16,
}
pub const CM_ADD_ID_BITS: u32 = 1u32;
pub const CM_ADD_ID_COMPATIBLE: u32 = 1u32;
pub const CM_ADD_ID_HARDWARE: u32 = 0u32;
pub const CM_ADD_RANGE_ADDIFCONFLICT: u32 = 0u32;
pub const CM_ADD_RANGE_BITS: u32 = 1u32;
pub const CM_ADD_RANGE_DONOTADDIFCONFLICT: u32 = 1u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CM_CDFLAGS(pub u32);
impl CM_CDFLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for CM_CDFLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for CM_CDFLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for CM_CDFLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for CM_CDFLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for CM_CDFLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const CM_CDFLAGS_DRIVER: CM_CDFLAGS = CM_CDFLAGS(1u32);
pub const CM_CDFLAGS_RESERVED: CM_CDFLAGS = CM_CDFLAGS(4u32);
pub const CM_CDFLAGS_ROOT_OWNED: CM_CDFLAGS = CM_CDFLAGS(2u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CM_CDMASK(pub u32);
impl CM_CDMASK {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for CM_CDMASK {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for CM_CDMASK {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for CM_CDMASK {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for CM_CDMASK {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for CM_CDMASK {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const CM_CDMASK_DESCRIPTION: CM_CDMASK = CM_CDMASK(8u32);
pub const CM_CDMASK_DEVINST: CM_CDMASK = CM_CDMASK(1u32);
pub const CM_CDMASK_FLAGS: CM_CDMASK = CM_CDMASK(4u32);
pub const CM_CDMASK_RESDES: CM_CDMASK = CM_CDMASK(2u32);
pub const CM_CDMASK_VALID: CM_CDMASK = CM_CDMASK(15u32);
pub const CM_CLASS_PROPERTY_BITS: u32 = 1u32;
pub const CM_CLASS_PROPERTY_INSTALLER: u32 = 0u32;
pub const CM_CLASS_PROPERTY_INTERFACE: u32 = 1u32;
pub const CM_CREATE_DEVINST_BITS: u32 = 15u32;
pub const CM_CREATE_DEVINST_DO_NOT_INSTALL: u32 = 8u32;
pub const CM_CREATE_DEVINST_GENERATE_ID: u32 = 4u32;
pub const CM_CREATE_DEVINST_NORMAL: u32 = 0u32;
pub const CM_CREATE_DEVINST_NO_WAIT_INSTALL: u32 = 1u32;
pub const CM_CREATE_DEVINST_PHANTOM: u32 = 2u32;
pub const CM_CREATE_DEVNODE_BITS: u32 = 15u32;
pub const CM_CREATE_DEVNODE_DO_NOT_INSTALL: u32 = 8u32;
pub const CM_CREATE_DEVNODE_GENERATE_ID: u32 = 4u32;
pub const CM_CREATE_DEVNODE_NORMAL: u32 = 0u32;
pub const CM_CREATE_DEVNODE_NO_WAIT_INSTALL: u32 = 1u32;
pub const CM_CREATE_DEVNODE_PHANTOM: u32 = 2u32;
pub const CM_CRP_CHARACTERISTICS: u32 = 28u32;
pub const CM_CRP_DEVTYPE: u32 = 26u32;
pub const CM_CRP_EXCLUSIVE: u32 = 27u32;
pub const CM_CRP_LOWERFILTERS: u32 = 19u32;
pub const CM_CRP_MAX: u32 = 37u32;
pub const CM_CRP_MIN: u32 = 1u32;
pub const CM_CRP_SECURITY: u32 = 24u32;
pub const CM_CRP_SECURITY_SDS: u32 = 25u32;
pub const CM_CRP_UPPERFILTERS: u32 = 18u32;
pub const CM_CUSTOMDEVPROP_BITS: u32 = 1u32;
pub const CM_CUSTOMDEVPROP_MERGE_MULTISZ: u32 = 1u32;
pub const CM_DELETE_CLASS_BITS: u32 = 3u32;
pub const CM_DELETE_CLASS_INTERFACE: u32 = 2u32;
pub const CM_DELETE_CLASS_ONLY: u32 = 0u32;
pub const CM_DELETE_CLASS_SUBKEYS: u32 = 1u32;
pub const CM_DETECT_BITS: u32 = 2147483655u32;
pub const CM_DETECT_CRASHED: u32 = 2u32;
pub const CM_DETECT_HWPROF_FIRST_BOOT: u32 = 4u32;
pub const CM_DETECT_NEW_PROFILE: u32 = 1u32;
pub const CM_DETECT_RUN: u32 = 2147483648u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CM_DEVCAP(pub u32);
impl CM_DEVCAP {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for CM_DEVCAP {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for CM_DEVCAP {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for CM_DEVCAP {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for CM_DEVCAP {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for CM_DEVCAP {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const CM_DEVCAP_DOCKDEVICE: CM_DEVCAP = CM_DEVCAP(8u32);
pub const CM_DEVCAP_EJECTSUPPORTED: CM_DEVCAP = CM_DEVCAP(2u32);
pub const CM_DEVCAP_HARDWAREDISABLED: CM_DEVCAP = CM_DEVCAP(256u32);
pub const CM_DEVCAP_LOCKSUPPORTED: CM_DEVCAP = CM_DEVCAP(1u32);
pub const CM_DEVCAP_NONDYNAMIC: CM_DEVCAP = CM_DEVCAP(512u32);
pub const CM_DEVCAP_RAWDEVICEOK: CM_DEVCAP = CM_DEVCAP(64u32);
pub const CM_DEVCAP_REMOVABLE: CM_DEVCAP = CM_DEVCAP(4u32);
pub const CM_DEVCAP_SECUREDEVICE: CM_DEVCAP = CM_DEVCAP(1024u32);
pub const CM_DEVCAP_SILENTINSTALL: CM_DEVCAP = CM_DEVCAP(32u32);
pub const CM_DEVCAP_SURPRISEREMOVALOK: CM_DEVCAP = CM_DEVCAP(128u32);
pub const CM_DEVCAP_UNIQUEID: CM_DEVCAP = CM_DEVCAP(16u32);
pub const CM_DEVICE_PANEL_EDGE_BOTTOM: u32 = 2u32;
pub const CM_DEVICE_PANEL_EDGE_LEFT: u32 = 3u32;
pub const CM_DEVICE_PANEL_EDGE_RIGHT: u32 = 4u32;
pub const CM_DEVICE_PANEL_EDGE_TOP: u32 = 1u32;
pub const CM_DEVICE_PANEL_EDGE_UNKNOWN: u32 = 0u32;
pub const CM_DEVICE_PANEL_JOINT_TYPE_HINGE: u32 = 2u32;
pub const CM_DEVICE_PANEL_JOINT_TYPE_PIVOT: u32 = 3u32;
pub const CM_DEVICE_PANEL_JOINT_TYPE_PLANAR: u32 = 1u32;
pub const CM_DEVICE_PANEL_JOINT_TYPE_SWIVEL: u32 = 4u32;
pub const CM_DEVICE_PANEL_JOINT_TYPE_UNKNOWN: u32 = 0u32;
pub const CM_DEVICE_PANEL_ORIENTATION_HORIZONTAL: u32 = 0u32;
pub const CM_DEVICE_PANEL_ORIENTATION_VERTICAL: u32 = 1u32;
pub const CM_DEVICE_PANEL_SHAPE_OVAL: u32 = 2u32;
pub const CM_DEVICE_PANEL_SHAPE_RECTANGLE: u32 = 1u32;
pub const CM_DEVICE_PANEL_SHAPE_UNKNOWN: u32 = 0u32;
pub const CM_DEVICE_PANEL_SIDE_BACK: u32 = 6u32;
pub const CM_DEVICE_PANEL_SIDE_BOTTOM: u32 = 2u32;
pub const CM_DEVICE_PANEL_SIDE_FRONT: u32 = 5u32;
pub const CM_DEVICE_PANEL_SIDE_LEFT: u32 = 3u32;
pub const CM_DEVICE_PANEL_SIDE_RIGHT: u32 = 4u32;
pub const CM_DEVICE_PANEL_SIDE_TOP: u32 = 1u32;
pub const CM_DEVICE_PANEL_SIDE_UNKNOWN: u32 = 0u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CM_DEVNODE_STATUS_FLAGS(pub u32);
impl CM_DEVNODE_STATUS_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for CM_DEVNODE_STATUS_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for CM_DEVNODE_STATUS_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for CM_DEVNODE_STATUS_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for CM_DEVNODE_STATUS_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for CM_DEVNODE_STATUS_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const CM_DISABLE_ABSOLUTE: u32 = 1u32;
pub const CM_DISABLE_BITS: u32 = 15u32;
pub const CM_DISABLE_HARDWARE: u32 = 2u32;
pub const CM_DISABLE_PERSIST: u32 = 8u32;
pub const CM_DISABLE_POLITE: u32 = 0u32;
pub const CM_DISABLE_UI_NOT_OK: u32 = 4u32;
pub const CM_DRP_ADDRESS: u32 = 29u32;
pub const CM_DRP_BASE_CONTAINERID: u32 = 37u32;
pub const CM_DRP_BUSNUMBER: u32 = 22u32;
pub const CM_DRP_BUSTYPEGUID: u32 = 20u32;
pub const CM_DRP_CAPABILITIES: u32 = 16u32;
pub const CM_DRP_CHARACTERISTICS: u32 = 28u32;
pub const CM_DRP_CLASS: u32 = 8u32;
pub const CM_DRP_CLASSGUID: u32 = 9u32;
pub const CM_DRP_COMPATIBLEIDS: u32 = 3u32;
pub const CM_DRP_CONFIGFLAGS: u32 = 11u32;
pub const CM_DRP_DEVICEDESC: u32 = 1u32;
pub const CM_DRP_DEVICE_POWER_DATA: u32 = 31u32;
pub const CM_DRP_DEVTYPE: u32 = 26u32;
pub const CM_DRP_DRIVER: u32 = 10u32;
pub const CM_DRP_ENUMERATOR_NAME: u32 = 23u32;
pub const CM_DRP_EXCLUSIVE: u32 = 27u32;
pub const CM_DRP_FRIENDLYNAME: u32 = 13u32;
pub const CM_DRP_HARDWAREID: u32 = 2u32;
pub const CM_DRP_INSTALL_STATE: u32 = 35u32;
pub const CM_DRP_LEGACYBUSTYPE: u32 = 21u32;
pub const CM_DRP_LOCATION_INFORMATION: u32 = 14u32;
pub const CM_DRP_LOCATION_PATHS: u32 = 36u32;
pub const CM_DRP_LOWERFILTERS: u32 = 19u32;
pub const CM_DRP_MAX: u32 = 37u32;
pub const CM_DRP_MFG: u32 = 12u32;
pub const CM_DRP_MIN: u32 = 1u32;
pub const CM_DRP_PHYSICAL_DEVICE_OBJECT_NAME: u32 = 15u32;
pub const CM_DRP_REMOVAL_POLICY: u32 = 32u32;
pub const CM_DRP_REMOVAL_POLICY_HW_DEFAULT: u32 = 33u32;
pub const CM_DRP_REMOVAL_POLICY_OVERRIDE: u32 = 34u32;
pub const CM_DRP_SECURITY: u32 = 24u32;
pub const CM_DRP_SECURITY_SDS: u32 = 25u32;
pub const CM_DRP_SERVICE: u32 = 5u32;
pub const CM_DRP_UI_NUMBER: u32 = 17u32;
pub const CM_DRP_UI_NUMBER_DESC_FORMAT: u32 = 30u32;
pub const CM_DRP_UNUSED0: u32 = 4u32;
pub const CM_DRP_UNUSED1: u32 = 6u32;
pub const CM_DRP_UNUSED2: u32 = 7u32;
pub const CM_DRP_UPPERFILTERS: u32 = 18u32;
pub const CM_ENUMERATE_CLASSES_BITS: CM_ENUMERATE_FLAGS = CM_ENUMERATE_FLAGS(1u32);
pub const CM_ENUMERATE_CLASSES_INSTALLER: CM_ENUMERATE_FLAGS = CM_ENUMERATE_FLAGS(0u32);
pub const CM_ENUMERATE_CLASSES_INTERFACE: CM_ENUMERATE_FLAGS = CM_ENUMERATE_FLAGS(1u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CM_ENUMERATE_FLAGS(pub u32);
impl CM_ENUMERATE_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for CM_ENUMERATE_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for CM_ENUMERATE_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for CM_ENUMERATE_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for CM_ENUMERATE_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for CM_ENUMERATE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const CM_GETIDLIST_DONOTGENERATE: u32 = 268435520u32;
pub const CM_GETIDLIST_FILTER_BITS: u32 = 268435583u32;
pub const CM_GETIDLIST_FILTER_BUSRELATIONS: u32 = 32u32;
pub const CM_GETIDLIST_FILTER_CLASS: u32 = 512u32;
pub const CM_GETIDLIST_FILTER_EJECTRELATIONS: u32 = 4u32;
pub const CM_GETIDLIST_FILTER_ENUMERATOR: u32 = 1u32;
pub const CM_GETIDLIST_FILTER_NONE: u32 = 0u32;
pub const CM_GETIDLIST_FILTER_POWERRELATIONS: u32 = 16u32;
pub const CM_GETIDLIST_FILTER_PRESENT: u32 = 256u32;
pub const CM_GETIDLIST_FILTER_REMOVALRELATIONS: u32 = 8u32;
pub const CM_GETIDLIST_FILTER_SERVICE: u32 = 2u32;
pub const CM_GETIDLIST_FILTER_TRANSPORTRELATIONS: u32 = 128u32;
pub const CM_GET_DEVICE_INTERFACE_LIST_ALL_DEVICES: CM_GET_DEVICE_INTERFACE_LIST_FLAGS = CM_GET_DEVICE_INTERFACE_LIST_FLAGS(1u32);
pub const CM_GET_DEVICE_INTERFACE_LIST_BITS: CM_GET_DEVICE_INTERFACE_LIST_FLAGS = CM_GET_DEVICE_INTERFACE_LIST_FLAGS(1u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CM_GET_DEVICE_INTERFACE_LIST_FLAGS(pub u32);
pub const CM_GET_DEVICE_INTERFACE_LIST_PRESENT: CM_GET_DEVICE_INTERFACE_LIST_FLAGS = CM_GET_DEVICE_INTERFACE_LIST_FLAGS(0u32);
pub const CM_GLOBAL_STATE_CAN_DO_UI: u32 = 1u32;
pub const CM_GLOBAL_STATE_DETECTION_PENDING: u32 = 16u32;
pub const CM_GLOBAL_STATE_ON_BIG_STACK: u32 = 2u32;
pub const CM_GLOBAL_STATE_REBOOT_REQUIRED: u32 = 32u32;
pub const CM_GLOBAL_STATE_SERVICES_AVAILABLE: u32 = 4u32;
pub const CM_GLOBAL_STATE_SHUTTING_DOWN: u32 = 8u32;
pub const CM_HWPI_DOCKED: u32 = 2u32;
pub const CM_HWPI_NOT_DOCKABLE: u32 = 0u32;
pub const CM_HWPI_UNDOCKED: u32 = 1u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CM_INSTALL_STATE(pub u32);
pub const CM_INSTALL_STATE_FAILED_INSTALL: CM_INSTALL_STATE = CM_INSTALL_STATE(2u32);
pub const CM_INSTALL_STATE_FINISH_INSTALL: CM_INSTALL_STATE = CM_INSTALL_STATE(3u32);
pub const CM_INSTALL_STATE_INSTALLED: CM_INSTALL_STATE = CM_INSTALL_STATE(0u32);
pub const CM_INSTALL_STATE_NEEDS_REINSTALL: CM_INSTALL_STATE = CM_INSTALL_STATE(1u32);
pub const CM_LOCATE_DEVNODE_BITS: CM_LOCATE_DEVNODE_FLAGS = CM_LOCATE_DEVNODE_FLAGS(7u32);
pub const CM_LOCATE_DEVNODE_CANCELREMOVE: CM_LOCATE_DEVNODE_FLAGS = CM_LOCATE_DEVNODE_FLAGS(2u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CM_LOCATE_DEVNODE_FLAGS(pub u32);
pub const CM_LOCATE_DEVNODE_NORMAL: CM_LOCATE_DEVNODE_FLAGS = CM_LOCATE_DEVNODE_FLAGS(0u32);
pub const CM_LOCATE_DEVNODE_NOVALIDATION: CM_LOCATE_DEVNODE_FLAGS = CM_LOCATE_DEVNODE_FLAGS(4u32);
pub const CM_LOCATE_DEVNODE_PHANTOM: CM_LOCATE_DEVNODE_FLAGS = CM_LOCATE_DEVNODE_FLAGS(1u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CM_LOG_CONF(pub u32);
pub const CM_NAME_ATTRIBUTE_NAME_RETRIEVED_FROM_DEVICE: u32 = 1u32;
pub const CM_NAME_ATTRIBUTE_USER_ASSIGNED_NAME: u32 = 2u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CM_NOTIFY_ACTION(pub i32);
pub const CM_NOTIFY_ACTION_DEVICECUSTOMEVENT: CM_NOTIFY_ACTION = CM_NOTIFY_ACTION(6i32);
pub const CM_NOTIFY_ACTION_DEVICEINSTANCEENUMERATED: CM_NOTIFY_ACTION = CM_NOTIFY_ACTION(7i32);
pub const CM_NOTIFY_ACTION_DEVICEINSTANCEREMOVED: CM_NOTIFY_ACTION = CM_NOTIFY_ACTION(9i32);
pub const CM_NOTIFY_ACTION_DEVICEINSTANCESTARTED: CM_NOTIFY_ACTION = CM_NOTIFY_ACTION(8i32);
pub const CM_NOTIFY_ACTION_DEVICEINTERFACEARRIVAL: CM_NOTIFY_ACTION = CM_NOTIFY_ACTION(0i32);
pub const CM_NOTIFY_ACTION_DEVICEINTERFACEREMOVAL: CM_NOTIFY_ACTION = CM_NOTIFY_ACTION(1i32);
pub const CM_NOTIFY_ACTION_DEVICEQUERYREMOVE: CM_NOTIFY_ACTION = CM_NOTIFY_ACTION(2i32);
pub const CM_NOTIFY_ACTION_DEVICEQUERYREMOVEFAILED: CM_NOTIFY_ACTION = CM_NOTIFY_ACTION(3i32);
pub const CM_NOTIFY_ACTION_DEVICEREMOVECOMPLETE: CM_NOTIFY_ACTION = CM_NOTIFY_ACTION(5i32);
pub const CM_NOTIFY_ACTION_DEVICEREMOVEPENDING: CM_NOTIFY_ACTION = CM_NOTIFY_ACTION(4i32);
pub const CM_NOTIFY_ACTION_MAX: CM_NOTIFY_ACTION = CM_NOTIFY_ACTION(10i32);
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CM_NOTIFY_EVENT_DATA {
    pub FilterType: CM_NOTIFY_FILTER_TYPE,
    pub Reserved: u32,
    pub u: CM_NOTIFY_EVENT_DATA_0,
}
impl Default for CM_NOTIFY_EVENT_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union CM_NOTIFY_EVENT_DATA_0 {
    pub DeviceInterface: CM_NOTIFY_EVENT_DATA_0_0,
    pub DeviceHandle: CM_NOTIFY_EVENT_DATA_0_1,
    pub DeviceInstance: CM_NOTIFY_EVENT_DATA_0_2,
}
impl Default for CM_NOTIFY_EVENT_DATA_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CM_NOTIFY_EVENT_DATA_0_1 {
    pub EventGuid: windows_core::GUID,
    pub NameOffset: i32,
    pub DataSize: u32,
    pub Data: [u8; 1],
}
impl Default for CM_NOTIFY_EVENT_DATA_0_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CM_NOTIFY_EVENT_DATA_0_2 {
    pub InstanceId: [u16; 1],
}
impl Default for CM_NOTIFY_EVENT_DATA_0_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CM_NOTIFY_EVENT_DATA_0_0 {
    pub ClassGuid: windows_core::GUID,
    pub SymbolicLink: [u16; 1],
}
impl Default for CM_NOTIFY_EVENT_DATA_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CM_NOTIFY_FILTER {
    pub cbSize: u32,
    pub Flags: u32,
    pub FilterType: CM_NOTIFY_FILTER_TYPE,
    pub Reserved: u32,
    pub u: CM_NOTIFY_FILTER_0,
}
impl Default for CM_NOTIFY_FILTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union CM_NOTIFY_FILTER_0 {
    pub DeviceInterface: CM_NOTIFY_FILTER_0_0,
    pub DeviceHandle: CM_NOTIFY_FILTER_0_1,
    pub DeviceInstance: CM_NOTIFY_FILTER_0_2,
}
impl Default for CM_NOTIFY_FILTER_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CM_NOTIFY_FILTER_0_1 {
    pub hTarget: super::super::Foundation::HANDLE,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CM_NOTIFY_FILTER_0_2 {
    pub InstanceId: [u16; 200],
}
impl Default for CM_NOTIFY_FILTER_0_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CM_NOTIFY_FILTER_0_0 {
    pub ClassGuid: windows_core::GUID,
}
pub const CM_NOTIFY_FILTER_FLAG_ALL_DEVICE_INSTANCES: u32 = 2u32;
pub const CM_NOTIFY_FILTER_FLAG_ALL_INTERFACE_CLASSES: u32 = 1u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CM_NOTIFY_FILTER_TYPE(pub i32);
pub const CM_NOTIFY_FILTER_TYPE_DEVICEHANDLE: CM_NOTIFY_FILTER_TYPE = CM_NOTIFY_FILTER_TYPE(1i32);
pub const CM_NOTIFY_FILTER_TYPE_DEVICEINSTANCE: CM_NOTIFY_FILTER_TYPE = CM_NOTIFY_FILTER_TYPE(2i32);
pub const CM_NOTIFY_FILTER_TYPE_DEVICEINTERFACE: CM_NOTIFY_FILTER_TYPE = CM_NOTIFY_FILTER_TYPE(0i32);
pub const CM_NOTIFY_FILTER_TYPE_MAX: CM_NOTIFY_FILTER_TYPE = CM_NOTIFY_FILTER_TYPE(3i32);
pub const CM_OPEN_CLASS_KEY_BITS: u32 = 1u32;
pub const CM_OPEN_CLASS_KEY_INSTALLER: u32 = 0u32;
pub const CM_OPEN_CLASS_KEY_INTERFACE: u32 = 1u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CM_PROB(pub u32);
pub const CM_PROB_BIOS_TABLE: CM_PROB = CM_PROB(35u32);
pub const CM_PROB_BOOT_CONFIG_CONFLICT: CM_PROB = CM_PROB(6u32);
pub const CM_PROB_CANT_SHARE_IRQ: CM_PROB = CM_PROB(30u32);
pub const CM_PROB_CONSOLE_LOCKED: CM_PROB = CM_PROB(55u32);
pub const CM_PROB_DEVICE_NOT_THERE: CM_PROB = CM_PROB(24u32);
pub const CM_PROB_DEVICE_RESET: CM_PROB = CM_PROB(54u32);
pub const CM_PROB_DEVLOADER_FAILED: CM_PROB = CM_PROB(2u32);
pub const CM_PROB_DEVLOADER_NOT_FOUND: CM_PROB = CM_PROB(8u32);
pub const CM_PROB_DEVLOADER_NOT_READY: CM_PROB = CM_PROB(23u32);
pub const CM_PROB_DISABLED: CM_PROB = CM_PROB(22u32);
pub const CM_PROB_DISABLED_SERVICE: CM_PROB = CM_PROB(32u32);
pub const CM_PROB_DRIVER_BLOCKED: CM_PROB = CM_PROB(48u32);
pub const CM_PROB_DRIVER_FAILED_LOAD: CM_PROB = CM_PROB(39u32);
pub const CM_PROB_DRIVER_FAILED_PRIOR_UNLOAD: CM_PROB = CM_PROB(38u32);
pub const CM_PROB_DRIVER_SERVICE_KEY_INVALID: CM_PROB = CM_PROB(40u32);
pub const CM_PROB_DUPLICATE_DEVICE: CM_PROB = CM_PROB(42u32);
pub const CM_PROB_ENTRY_IS_WRONG_TYPE: CM_PROB = CM_PROB(4u32);
pub const CM_PROB_FAILED_ADD: CM_PROB = CM_PROB(31u32);
pub const CM_PROB_FAILED_DRIVER_ENTRY: CM_PROB = CM_PROB(37u32);
pub const CM_PROB_FAILED_FILTER: CM_PROB = CM_PROB(7u32);
pub const CM_PROB_FAILED_INSTALL: CM_PROB = CM_PROB(28u32);
pub const CM_PROB_FAILED_POST_START: CM_PROB = CM_PROB(43u32);
pub const CM_PROB_FAILED_START: CM_PROB = CM_PROB(10u32);
pub const CM_PROB_GUEST_ASSIGNMENT_FAILED: CM_PROB = CM_PROB(57u32);
pub const CM_PROB_HALTED: CM_PROB = CM_PROB(44u32);
pub const CM_PROB_HARDWARE_DISABLED: CM_PROB = CM_PROB(29u32);
pub const CM_PROB_HELD_FOR_EJECT: CM_PROB = CM_PROB(47u32);
pub const CM_PROB_INVALID_DATA: CM_PROB = CM_PROB(9u32);
pub const CM_PROB_IRQ_TRANSLATION_FAILED: CM_PROB = CM_PROB(36u32);
pub const CM_PROB_LACKED_ARBITRATOR: CM_PROB = CM_PROB(5u32);
pub const CM_PROB_LEGACY_SERVICE_NO_DEVICES: CM_PROB = CM_PROB(41u32);
pub const CM_PROB_LIAR: CM_PROB = CM_PROB(11u32);
pub const CM_PROB_MOVED: CM_PROB = CM_PROB(25u32);
pub const CM_PROB_NEED_CLASS_CONFIG: CM_PROB = CM_PROB(56u32);
pub const CM_PROB_NEED_RESTART: CM_PROB = CM_PROB(14u32);
pub const CM_PROB_NORMAL_CONFLICT: CM_PROB = CM_PROB(12u32);
pub const CM_PROB_NOT_CONFIGURED: CM_PROB = CM_PROB(1u32);
pub const CM_PROB_NOT_VERIFIED: CM_PROB = CM_PROB(13u32);
pub const CM_PROB_NO_SOFTCONFIG: CM_PROB = CM_PROB(34u32);
pub const CM_PROB_NO_VALID_LOG_CONF: CM_PROB = CM_PROB(27u32);
pub const CM_PROB_OUT_OF_MEMORY: CM_PROB = CM_PROB(3u32);
pub const CM_PROB_PARTIAL_LOG_CONF: CM_PROB = CM_PROB(16u32);
pub const CM_PROB_PHANTOM: CM_PROB = CM_PROB(45u32);
pub const CM_PROB_REENUMERATION: CM_PROB = CM_PROB(15u32);
pub const CM_PROB_REGISTRY: CM_PROB = CM_PROB(19u32);
pub const CM_PROB_REGISTRY_TOO_LARGE: CM_PROB = CM_PROB(49u32);
pub const CM_PROB_REINSTALL: CM_PROB = CM_PROB(18u32);
pub const CM_PROB_SETPROPERTIES_FAILED: CM_PROB = CM_PROB(50u32);
pub const CM_PROB_SYSTEM_SHUTDOWN: CM_PROB = CM_PROB(46u32);
pub const CM_PROB_TOO_EARLY: CM_PROB = CM_PROB(26u32);
pub const CM_PROB_TRANSLATION_FAILED: CM_PROB = CM_PROB(33u32);
pub const CM_PROB_UNKNOWN_RESOURCE: CM_PROB = CM_PROB(17u32);
pub const CM_PROB_UNSIGNED_DRIVER: CM_PROB = CM_PROB(52u32);
pub const CM_PROB_USED_BY_DEBUGGER: CM_PROB = CM_PROB(53u32);
pub const CM_PROB_VXDLDR: CM_PROB = CM_PROB(20u32);
pub const CM_PROB_WAITING_ON_DEPENDENCY: CM_PROB = CM_PROB(51u32);
pub const CM_PROB_WILL_BE_REMOVED: CM_PROB = CM_PROB(21u32);
pub const CM_QUERY_ARBITRATOR_BITS: u32 = 1u32;
pub const CM_QUERY_ARBITRATOR_RAW: u32 = 0u32;
pub const CM_QUERY_ARBITRATOR_TRANSLATED: u32 = 1u32;
pub const CM_QUERY_REMOVE_UI_NOT_OK: u32 = 1u32;
pub const CM_QUERY_REMOVE_UI_OK: u32 = 0u32;
pub const CM_REENUMERATE_ASYNCHRONOUS: CM_REENUMERATE_FLAGS = CM_REENUMERATE_FLAGS(4u32);
pub const CM_REENUMERATE_BITS: CM_REENUMERATE_FLAGS = CM_REENUMERATE_FLAGS(7u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CM_REENUMERATE_FLAGS(pub u32);
impl CM_REENUMERATE_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for CM_REENUMERATE_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for CM_REENUMERATE_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for CM_REENUMERATE_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for CM_REENUMERATE_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for CM_REENUMERATE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const CM_REENUMERATE_NORMAL: CM_REENUMERATE_FLAGS = CM_REENUMERATE_FLAGS(0u32);
pub const CM_REENUMERATE_RETRY_INSTALLATION: CM_REENUMERATE_FLAGS = CM_REENUMERATE_FLAGS(2u32);
pub const CM_REENUMERATE_SYNCHRONOUS: CM_REENUMERATE_FLAGS = CM_REENUMERATE_FLAGS(1u32);
pub const CM_REGISTER_DEVICE_DRIVER_BITS: u32 = 3u32;
pub const CM_REGISTER_DEVICE_DRIVER_DISABLEABLE: u32 = 1u32;
pub const CM_REGISTER_DEVICE_DRIVER_REMOVABLE: u32 = 2u32;
pub const CM_REGISTER_DEVICE_DRIVER_STATIC: u32 = 0u32;
pub const CM_REGISTRY_BITS: u32 = 769u32;
pub const CM_REGISTRY_CONFIG: u32 = 512u32;
pub const CM_REGISTRY_HARDWARE: u32 = 0u32;
pub const CM_REGISTRY_SOFTWARE: u32 = 1u32;
pub const CM_REGISTRY_USER: u32 = 256u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CM_REMOVAL_POLICY(pub u32);
pub const CM_REMOVAL_POLICY_EXPECT_NO_REMOVAL: CM_REMOVAL_POLICY = CM_REMOVAL_POLICY(1u32);
pub const CM_REMOVAL_POLICY_EXPECT_ORDERLY_REMOVAL: CM_REMOVAL_POLICY = CM_REMOVAL_POLICY(2u32);
pub const CM_REMOVAL_POLICY_EXPECT_SURPRISE_REMOVAL: CM_REMOVAL_POLICY = CM_REMOVAL_POLICY(3u32);
pub const CM_REMOVE_BITS: u32 = 7u32;
pub const CM_REMOVE_DISABLE: u32 = 4u32;
pub const CM_REMOVE_NO_RESTART: u32 = 2u32;
pub const CM_REMOVE_UI_NOT_OK: u32 = 1u32;
pub const CM_REMOVE_UI_OK: u32 = 0u32;
pub const CM_RESDES_WIDTH_32: u32 = 1u32;
pub const CM_RESDES_WIDTH_64: u32 = 2u32;
pub const CM_RESDES_WIDTH_BITS: u32 = 3u32;
pub const CM_RESDES_WIDTH_DEFAULT: u32 = 0u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CM_RESTYPE(pub u32);
pub const CM_SETUP_BITS: u32 = 15u32;
pub const CM_SETUP_DEVINST_CONFIG: u32 = 5u32;
pub const CM_SETUP_DEVINST_CONFIG_CLASS: u32 = 6u32;
pub const CM_SETUP_DEVINST_CONFIG_EXTENSIONS: u32 = 7u32;
pub const CM_SETUP_DEVINST_CONFIG_RESET: u32 = 8u32;
pub const CM_SETUP_DEVINST_READY: u32 = 0u32;
pub const CM_SETUP_DEVINST_RESET: u32 = 4u32;
pub const CM_SETUP_DEVNODE_CONFIG: u32 = 5u32;
pub const CM_SETUP_DEVNODE_CONFIG_CLASS: u32 = 6u32;
pub const CM_SETUP_DEVNODE_CONFIG_EXTENSIONS: u32 = 7u32;
pub const CM_SETUP_DEVNODE_CONFIG_RESET: u32 = 8u32;
pub const CM_SETUP_DEVNODE_READY: u32 = 0u32;
pub const CM_SETUP_DEVNODE_RESET: u32 = 4u32;
pub const CM_SETUP_DOWNLOAD: u32 = 1u32;
pub const CM_SETUP_PROP_CHANGE: u32 = 3u32;
pub const CM_SETUP_WRITE_LOG_CONFS: u32 = 2u32;
pub const CM_SET_DEVINST_PROBLEM_BITS: u32 = 1u32;
pub const CM_SET_DEVINST_PROBLEM_NORMAL: u32 = 0u32;
pub const CM_SET_DEVINST_PROBLEM_OVERRIDE: u32 = 1u32;
pub const CM_SET_DEVNODE_PROBLEM_BITS: u32 = 1u32;
pub const CM_SET_DEVNODE_PROBLEM_NORMAL: u32 = 0u32;
pub const CM_SET_DEVNODE_PROBLEM_OVERRIDE: u32 = 1u32;
pub const CM_SET_HW_PROF_FLAGS_BITS: u32 = 1u32;
pub const CM_SET_HW_PROF_FLAGS_UI_NOT_OK: u32 = 1u32;
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy)]
pub struct COINSTALLER_CONTEXT_DATA {
    pub PostProcessing: windows_core::BOOL,
    pub InstallResult: u32,
    pub PrivateData: *mut core::ffi::c_void,
}
#[cfg(target_arch = "x86")]
impl Default for COINSTALLER_CONTEXT_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct COINSTALLER_CONTEXT_DATA {
    pub PostProcessing: windows_core::BOOL,
    pub InstallResult: u32,
    pub PrivateData: *mut core::ffi::c_void,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for COINSTALLER_CONTEXT_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CONFIGFLAG_BOOT_DEVICE: SETUP_DI_DEVICE_CONFIGURATION_FLAGS = SETUP_DI_DEVICE_CONFIGURATION_FLAGS(262144u32);
pub const CONFIGFLAG_CANTSTOPACHILD: SETUP_DI_DEVICE_CONFIGURATION_FLAGS = SETUP_DI_DEVICE_CONFIGURATION_FLAGS(128u32);
pub const CONFIGFLAG_DISABLED: SETUP_DI_DEVICE_CONFIGURATION_FLAGS = SETUP_DI_DEVICE_CONFIGURATION_FLAGS(1u32);
pub const CONFIGFLAG_FAILEDINSTALL: SETUP_DI_DEVICE_CONFIGURATION_FLAGS = SETUP_DI_DEVICE_CONFIGURATION_FLAGS(64u32);
pub const CONFIGFLAG_FINISHINSTALL_ACTION: SETUP_DI_DEVICE_CONFIGURATION_FLAGS = SETUP_DI_DEVICE_CONFIGURATION_FLAGS(131072u32);
pub const CONFIGFLAG_FINISHINSTALL_UI: SETUP_DI_DEVICE_CONFIGURATION_FLAGS = SETUP_DI_DEVICE_CONFIGURATION_FLAGS(65536u32);
pub const CONFIGFLAG_FINISH_INSTALL: SETUP_DI_DEVICE_CONFIGURATION_FLAGS = SETUP_DI_DEVICE_CONFIGURATION_FLAGS(1024u32);
pub const CONFIGFLAG_IGNORE_BOOT_LC: SETUP_DI_DEVICE_CONFIGURATION_FLAGS = SETUP_DI_DEVICE_CONFIGURATION_FLAGS(8u32);
pub const CONFIGFLAG_MANUAL_INSTALL: SETUP_DI_DEVICE_CONFIGURATION_FLAGS = SETUP_DI_DEVICE_CONFIGURATION_FLAGS(4u32);
pub const CONFIGFLAG_NEEDS_CLASS_CONFIG: SETUP_DI_DEVICE_CONFIGURATION_FLAGS = SETUP_DI_DEVICE_CONFIGURATION_FLAGS(524288u32);
pub const CONFIGFLAG_NEEDS_FORCED_CONFIG: SETUP_DI_DEVICE_CONFIGURATION_FLAGS = SETUP_DI_DEVICE_CONFIGURATION_FLAGS(2048u32);
pub const CONFIGFLAG_NETBOOT_CARD: SETUP_DI_DEVICE_CONFIGURATION_FLAGS = SETUP_DI_DEVICE_CONFIGURATION_FLAGS(4096u32);
pub const CONFIGFLAG_NET_BOOT: SETUP_DI_DEVICE_CONFIGURATION_FLAGS = SETUP_DI_DEVICE_CONFIGURATION_FLAGS(16u32);
pub const CONFIGFLAG_NOREMOVEEXIT: SETUP_DI_DEVICE_CONFIGURATION_FLAGS = SETUP_DI_DEVICE_CONFIGURATION_FLAGS(512u32);
pub const CONFIGFLAG_OKREMOVEROM: SETUP_DI_DEVICE_CONFIGURATION_FLAGS = SETUP_DI_DEVICE_CONFIGURATION_FLAGS(256u32);
pub const CONFIGFLAG_PARTIAL_LOG_CONF: SETUP_DI_DEVICE_CONFIGURATION_FLAGS = SETUP_DI_DEVICE_CONFIGURATION_FLAGS(8192u32);
pub const CONFIGFLAG_REINSTALL: SETUP_DI_DEVICE_CONFIGURATION_FLAGS = SETUP_DI_DEVICE_CONFIGURATION_FLAGS(32u32);
pub const CONFIGFLAG_REMOVED: SETUP_DI_DEVICE_CONFIGURATION_FLAGS = SETUP_DI_DEVICE_CONFIGURATION_FLAGS(2u32);
pub const CONFIGFLAG_SUPPRESS_SURPRISE: SETUP_DI_DEVICE_CONFIGURATION_FLAGS = SETUP_DI_DEVICE_CONFIGURATION_FLAGS(16384u32);
pub const CONFIGFLAG_VERIFY_HARDWARE: SETUP_DI_DEVICE_CONFIGURATION_FLAGS = SETUP_DI_DEVICE_CONFIGURATION_FLAGS(32768u32);
pub const CONFIGMG_VERSION: u32 = 1024u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CONFIGRET(pub u32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CONFLICT_DETAILS_A {
    pub CD_ulSize: u32,
    pub CD_ulMask: CM_CDMASK,
    pub CD_dnDevInst: u32,
    pub CD_rdResDes: usize,
    pub CD_ulFlags: CM_CDFLAGS,
    pub CD_szDescription: [i8; 260],
}
impl Default for CONFLICT_DETAILS_A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CONFLICT_DETAILS_W {
    pub CD_ulSize: u32,
    pub CD_ulMask: CM_CDMASK,
    pub CD_dnDevInst: u32,
    pub CD_rdResDes: usize,
    pub CD_ulFlags: CM_CDFLAGS,
    pub CD_szDescription: [u16; 260],
}
impl Default for CONFLICT_DETAILS_W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct CONNECTION_DES {
    pub COND_Type: u32,
    pub COND_Flags: u32,
    pub COND_Class: u8,
    pub COND_ClassType: u8,
    pub COND_Reserved1: u8,
    pub COND_Reserved2: u8,
    pub COND_Id: i64,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct CONNECTION_RESOURCE {
    pub Connection_Header: CONNECTION_DES,
}
pub const COPYFLG_FORCE_FILE_IN_USE: u32 = 8u32;
pub const COPYFLG_IN_USE_TRY_RENAME: u32 = 16384u32;
pub const COPYFLG_NODECOMP: u32 = 2048u32;
pub const COPYFLG_NOPRUNE: u32 = 8192u32;
pub const COPYFLG_NOSKIP: u32 = 2u32;
pub const COPYFLG_NOVERSIONCHECK: u32 = 4u32;
pub const COPYFLG_NO_OVERWRITE: u32 = 16u32;
pub const COPYFLG_NO_VERSION_DIALOG: u32 = 32u32;
pub const COPYFLG_OVERWRITE_OLDER_ONLY: u32 = 64u32;
pub const COPYFLG_PROTECTED_WINDOWS_DRIVER_FILE: u32 = 256u32;
pub const COPYFLG_REPLACEONLY: u32 = 1024u32;
pub const COPYFLG_REPLACE_BOOT_FILE: u32 = 4096u32;
pub const COPYFLG_WARN_IF_SKIP: u32 = 1u32;
pub const CR_ACCESS_DENIED: CONFIGRET = CONFIGRET(51u32);
pub const CR_ALREADY_SUCH_DEVINST: CONFIGRET = CONFIGRET(16u32);
pub const CR_ALREADY_SUCH_DEVNODE: CONFIGRET = CONFIGRET(16u32);
pub const CR_APM_VETOED: CONFIGRET = CONFIGRET(24u32);
pub const CR_BUFFER_SMALL: CONFIGRET = CONFIGRET(26u32);
pub const CR_CALL_NOT_IMPLEMENTED: CONFIGRET = CONFIGRET(52u32);
pub const CR_CANT_SHARE_IRQ: CONFIGRET = CONFIGRET(43u32);
pub const CR_CREATE_BLOCKED: CONFIGRET = CONFIGRET(21u32);
pub const CR_DEFAULT: CONFIGRET = CONFIGRET(1u32);
pub const CR_DEVICE_INTERFACE_ACTIVE: CONFIGRET = CONFIGRET(54u32);
pub const CR_DEVICE_NOT_THERE: CONFIGRET = CONFIGRET(36u32);
pub const CR_DEVINST_HAS_REQS: CONFIGRET = CONFIGRET(10u32);
pub const CR_DEVLOADER_NOT_READY: CONFIGRET = CONFIGRET(33u32);
pub const CR_DEVNODE_HAS_REQS: CONFIGRET = CONFIGRET(10u32);
pub const CR_DLVXD_NOT_FOUND: CONFIGRET = CONFIGRET(12u32);
pub const CR_FAILURE: CONFIGRET = CONFIGRET(19u32);
pub const CR_FREE_RESOURCES: CONFIGRET = CONFIGRET(41u32);
pub const CR_INVALID_API: CONFIGRET = CONFIGRET(32u32);
pub const CR_INVALID_ARBITRATOR: CONFIGRET = CONFIGRET(8u32);
pub const CR_INVALID_CONFLICT_LIST: CONFIGRET = CONFIGRET(57u32);
pub const CR_INVALID_DATA: CONFIGRET = CONFIGRET(31u32);
pub const CR_INVALID_DEVICE_ID: CONFIGRET = CONFIGRET(30u32);
pub const CR_INVALID_DEVINST: CONFIGRET = CONFIGRET(5u32);
pub const CR_INVALID_DEVNODE: CONFIGRET = CONFIGRET(5u32);
pub const CR_INVALID_FLAG: CONFIGRET = CONFIGRET(4u32);
pub const CR_INVALID_INDEX: CONFIGRET = CONFIGRET(58u32);
pub const CR_INVALID_LOAD_TYPE: CONFIGRET = CONFIGRET(25u32);
pub const CR_INVALID_LOG_CONF: CONFIGRET = CONFIGRET(7u32);
pub const CR_INVALID_MACHINENAME: CONFIGRET = CONFIGRET(47u32);
pub const CR_INVALID_NODELIST: CONFIGRET = CONFIGRET(9u32);
pub const CR_INVALID_POINTER: CONFIGRET = CONFIGRET(3u32);
pub const CR_INVALID_PRIORITY: CONFIGRET = CONFIGRET(39u32);
pub const CR_INVALID_PROPERTY: CONFIGRET = CONFIGRET(53u32);
pub const CR_INVALID_RANGE: CONFIGRET = CONFIGRET(18u32);
pub const CR_INVALID_RANGE_LIST: CONFIGRET = CONFIGRET(17u32);
pub const CR_INVALID_REFERENCE_STRING: CONFIGRET = CONFIGRET(56u32);
pub const CR_INVALID_RESOURCEID: CONFIGRET = CONFIGRET(11u32);
pub const CR_INVALID_RES_DES: CONFIGRET = CONFIGRET(6u32);
pub const CR_INVALID_STRUCTURE_SIZE: CONFIGRET = CONFIGRET(59u32);
pub const CR_MACHINE_UNAVAILABLE: CONFIGRET = CONFIGRET(49u32);
pub const CR_NEED_RESTART: CONFIGRET = CONFIGRET(34u32);
pub const CR_NOT_DISABLEABLE: CONFIGRET = CONFIGRET(40u32);
pub const CR_NOT_SYSTEM_VM: CONFIGRET = CONFIGRET(22u32);
pub const CR_NO_ARBITRATOR: CONFIGRET = CONFIGRET(27u32);
pub const CR_NO_CM_SERVICES: CONFIGRET = CONFIGRET(50u32);
pub const CR_NO_DEPENDENT: CONFIGRET = CONFIGRET(44u32);
pub const CR_NO_MORE_HW_PROFILES: CONFIGRET = CONFIGRET(35u32);
pub const CR_NO_MORE_LOG_CONF: CONFIGRET = CONFIGRET(14u32);
pub const CR_NO_MORE_RES_DES: CONFIGRET = CONFIGRET(15u32);
pub const CR_NO_REGISTRY_HANDLE: CONFIGRET = CONFIGRET(28u32);
pub const CR_NO_SUCH_DEVICE_INTERFACE: CONFIGRET = CONFIGRET(55u32);
pub const CR_NO_SUCH_DEVINST: CONFIGRET = CONFIGRET(13u32);
pub const CR_NO_SUCH_DEVNODE: CONFIGRET = CONFIGRET(13u32);
pub const CR_NO_SUCH_LOGICAL_DEV: CONFIGRET = CONFIGRET(20u32);
pub const CR_NO_SUCH_REGISTRY_KEY: CONFIGRET = CONFIGRET(46u32);
pub const CR_NO_SUCH_VALUE: CONFIGRET = CONFIGRET(37u32);
pub const CR_OUT_OF_MEMORY: CONFIGRET = CONFIGRET(2u32);
pub const CR_QUERY_VETOED: CONFIGRET = CONFIGRET(42u32);
pub const CR_REGISTRY_ERROR: CONFIGRET = CONFIGRET(29u32);
pub const CR_REMOTE_COMM_FAILURE: CONFIGRET = CONFIGRET(48u32);
pub const CR_REMOVE_VETOED: CONFIGRET = CONFIGRET(23u32);
pub const CR_SAME_RESOURCES: CONFIGRET = CONFIGRET(45u32);
pub const CR_SUCCESS: CONFIGRET = CONFIGRET(0u32);
pub const CR_WRONG_TYPE: CONFIGRET = CONFIGRET(38u32);
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct CS_DES {
    pub CSD_SignatureLength: u32,
    pub CSD_LegacyDataOffset: u32,
    pub CSD_LegacyDataSize: u32,
    pub CSD_Flags: u32,
    pub CSD_ClassGuid: windows_core::GUID,
    pub CSD_Signature: [u8; 1],
}
impl Default for CS_DES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct CS_RESOURCE {
    pub CS_Header: CS_DES,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DD_FLAGS(pub u32);
impl DD_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for DD_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for DD_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for DD_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for DD_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for DD_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const DELFLG_IN_USE: u32 = 1u32;
pub const DELFLG_IN_USE1: u32 = 65536u32;
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct DEVPRIVATE_DES {
    pub PD_Count: u32,
    pub PD_Type: u32,
    pub PD_Data1: u32,
    pub PD_Data2: u32,
    pub PD_Data3: u32,
    pub PD_Flags: u32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct DEVPRIVATE_RANGE {
    pub PR_Data1: u32,
    pub PR_Data2: u32,
    pub PR_Data3: u32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct DEVPRIVATE_RESOURCE {
    pub PRV_Header: DEVPRIVATE_DES,
    pub PRV_Data: [DEVPRIVATE_RANGE; 1],
}
impl Default for DEVPRIVATE_RESOURCE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DIBCI_NODISPLAYCLASS: u32 = 2u32;
pub const DIBCI_NOINSTALLCLASS: u32 = 1u32;
pub const DICD_GENERATE_ID: SETUP_DI_DEVICE_CREATION_FLAGS = SETUP_DI_DEVICE_CREATION_FLAGS(1u32);
pub const DICD_INHERIT_CLASSDRVS: SETUP_DI_DEVICE_CREATION_FLAGS = SETUP_DI_DEVICE_CREATION_FLAGS(2u32);
pub const DICLASSPROP_INSTALLER: u32 = 1u32;
pub const DICLASSPROP_INTERFACE: u32 = 2u32;
pub const DICS_DISABLE: SETUP_DI_STATE_CHANGE = SETUP_DI_STATE_CHANGE(2u32);
pub const DICS_ENABLE: SETUP_DI_STATE_CHANGE = SETUP_DI_STATE_CHANGE(1u32);
pub const DICS_FLAG_CONFIGGENERAL: SETUP_DI_PROPERTY_CHANGE_SCOPE = SETUP_DI_PROPERTY_CHANGE_SCOPE(4u32);
pub const DICS_FLAG_CONFIGSPECIFIC: SETUP_DI_PROPERTY_CHANGE_SCOPE = SETUP_DI_PROPERTY_CHANGE_SCOPE(2u32);
pub const DICS_FLAG_GLOBAL: SETUP_DI_PROPERTY_CHANGE_SCOPE = SETUP_DI_PROPERTY_CHANGE_SCOPE(1u32);
pub const DICS_PROPCHANGE: SETUP_DI_STATE_CHANGE = SETUP_DI_STATE_CHANGE(3u32);
pub const DICS_START: SETUP_DI_STATE_CHANGE = SETUP_DI_STATE_CHANGE(4u32);
pub const DICS_STOP: SETUP_DI_STATE_CHANGE = SETUP_DI_STATE_CHANGE(5u32);
pub const DICUSTOMDEVPROP_MERGE_MULTISZ: u32 = 1u32;
pub const DIF_ADDPROPERTYPAGE_ADVANCED: DI_FUNCTION = DI_FUNCTION(35u32);
pub const DIF_ADDPROPERTYPAGE_BASIC: DI_FUNCTION = DI_FUNCTION(36u32);
pub const DIF_ADDREMOTEPROPERTYPAGE_ADVANCED: DI_FUNCTION = DI_FUNCTION(40u32);
pub const DIF_ALLOW_INSTALL: DI_FUNCTION = DI_FUNCTION(24u32);
pub const DIF_ASSIGNRESOURCES: DI_FUNCTION = DI_FUNCTION(3u32);
pub const DIF_CALCDISKSPACE: DI_FUNCTION = DI_FUNCTION(11u32);
pub const DIF_DESTROYPRIVATEDATA: DI_FUNCTION = DI_FUNCTION(12u32);
pub const DIF_DESTROYWIZARDDATA: DI_FUNCTION = DI_FUNCTION(17u32);
pub const DIF_DETECT: DI_FUNCTION = DI_FUNCTION(15u32);
pub const DIF_DETECTCANCEL: DI_FUNCTION = DI_FUNCTION(33u32);
pub const DIF_DETECTVERIFY: DI_FUNCTION = DI_FUNCTION(20u32);
pub const DIF_ENABLECLASS: DI_FUNCTION = DI_FUNCTION(19u32);
pub const DIF_FINISHINSTALL_ACTION: DI_FUNCTION = DI_FUNCTION(42u32);
pub const DIF_FIRSTTIMESETUP: DI_FUNCTION = DI_FUNCTION(6u32);
pub const DIF_FOUNDDEVICE: DI_FUNCTION = DI_FUNCTION(7u32);
pub const DIF_INSTALLCLASSDRIVERS: DI_FUNCTION = DI_FUNCTION(10u32);
pub const DIF_INSTALLDEVICE: DI_FUNCTION = DI_FUNCTION(2u32);
pub const DIF_INSTALLDEVICEFILES: DI_FUNCTION = DI_FUNCTION(21u32);
pub const DIF_INSTALLINTERFACES: DI_FUNCTION = DI_FUNCTION(32u32);
pub const DIF_INSTALLWIZARD: DI_FUNCTION = DI_FUNCTION(16u32);
pub const DIF_MOVEDEVICE: DI_FUNCTION = DI_FUNCTION(14u32);
pub const DIF_NEWDEVICEWIZARD_FINISHINSTALL: DI_FUNCTION = DI_FUNCTION(30u32);
pub const DIF_NEWDEVICEWIZARD_POSTANALYZE: DI_FUNCTION = DI_FUNCTION(29u32);
pub const DIF_NEWDEVICEWIZARD_PREANALYZE: DI_FUNCTION = DI_FUNCTION(28u32);
pub const DIF_NEWDEVICEWIZARD_PRESELECT: DI_FUNCTION = DI_FUNCTION(26u32);
pub const DIF_NEWDEVICEWIZARD_SELECT: DI_FUNCTION = DI_FUNCTION(27u32);
pub const DIF_POWERMESSAGEWAKE: DI_FUNCTION = DI_FUNCTION(39u32);
pub const DIF_PROPERTIES: DI_FUNCTION = DI_FUNCTION(4u32);
pub const DIF_PROPERTYCHANGE: DI_FUNCTION = DI_FUNCTION(18u32);
pub const DIF_REGISTERDEVICE: DI_FUNCTION = DI_FUNCTION(25u32);
pub const DIF_REGISTER_COINSTALLERS: DI_FUNCTION = DI_FUNCTION(34u32);
pub const DIF_REMOVE: DI_FUNCTION = DI_FUNCTION(5u32);
pub const DIF_RESERVED1: DI_FUNCTION = DI_FUNCTION(37u32);
pub const DIF_RESERVED2: DI_FUNCTION = DI_FUNCTION(48u32);
pub const DIF_SELECTBESTCOMPATDRV: DI_FUNCTION = DI_FUNCTION(23u32);
pub const DIF_SELECTCLASSDRIVERS: DI_FUNCTION = DI_FUNCTION(8u32);
pub const DIF_SELECTDEVICE: DI_FUNCTION = DI_FUNCTION(1u32);
pub const DIF_TROUBLESHOOTER: DI_FUNCTION = DI_FUNCTION(38u32);
pub const DIF_UNREMOVE: DI_FUNCTION = DI_FUNCTION(22u32);
pub const DIF_UNUSED1: DI_FUNCTION = DI_FUNCTION(31u32);
pub const DIF_UPDATEDRIVER_UI: DI_FUNCTION = DI_FUNCTION(41u32);
pub const DIF_VALIDATECLASSDRIVERS: DI_FUNCTION = DI_FUNCTION(9u32);
pub const DIF_VALIDATEDRIVER: DI_FUNCTION = DI_FUNCTION(13u32);
pub const DIGCDP_FLAG_ADVANCED: u32 = 2u32;
pub const DIGCDP_FLAG_BASIC: u32 = 1u32;
pub const DIGCDP_FLAG_REMOTE_ADVANCED: u32 = 4u32;
pub const DIGCDP_FLAG_REMOTE_BASIC: u32 = 3u32;
pub const DIGCF_ALLCLASSES: SETUP_DI_GET_CLASS_DEVS_FLAGS = SETUP_DI_GET_CLASS_DEVS_FLAGS(4u32);
pub const DIGCF_DEFAULT: SETUP_DI_GET_CLASS_DEVS_FLAGS = SETUP_DI_GET_CLASS_DEVS_FLAGS(1u32);
pub const DIGCF_DEVICEINTERFACE: SETUP_DI_GET_CLASS_DEVS_FLAGS = SETUP_DI_GET_CLASS_DEVS_FLAGS(16u32);
pub const DIGCF_INTERFACEDEVICE: SETUP_DI_GET_CLASS_DEVS_FLAGS = SETUP_DI_GET_CLASS_DEVS_FLAGS(16u32);
pub const DIGCF_PRESENT: SETUP_DI_GET_CLASS_DEVS_FLAGS = SETUP_DI_GET_CLASS_DEVS_FLAGS(2u32);
pub const DIGCF_PROFILE: SETUP_DI_GET_CLASS_DEVS_FLAGS = SETUP_DI_GET_CLASS_DEVS_FLAGS(8u32);
pub const DIIDFLAG_BITS: DIINSTALLDEVICE_FLAGS = DIINSTALLDEVICE_FLAGS(15u32);
pub const DIIDFLAG_INSTALLCOPYINFDRIVERS: DIINSTALLDEVICE_FLAGS = DIINSTALLDEVICE_FLAGS(8u32);
pub const DIIDFLAG_INSTALLNULLDRIVER: DIINSTALLDEVICE_FLAGS = DIINSTALLDEVICE_FLAGS(4u32);
pub const DIIDFLAG_NOFINISHINSTALLUI: DIINSTALLDEVICE_FLAGS = DIINSTALLDEVICE_FLAGS(2u32);
pub const DIIDFLAG_SHOWSEARCHUI: DIINSTALLDEVICE_FLAGS = DIINSTALLDEVICE_FLAGS(1u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DIINSTALLDEVICE_FLAGS(pub u32);
impl DIINSTALLDEVICE_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for DIINSTALLDEVICE_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for DIINSTALLDEVICE_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for DIINSTALLDEVICE_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for DIINSTALLDEVICE_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for DIINSTALLDEVICE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DIINSTALLDRIVER_FLAGS(pub u32);
impl DIINSTALLDRIVER_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for DIINSTALLDRIVER_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for DIINSTALLDRIVER_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for DIINSTALLDRIVER_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for DIINSTALLDRIVER_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for DIINSTALLDRIVER_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const DIIRFLAG_BITS: DIINSTALLDRIVER_FLAGS = DIINSTALLDRIVER_FLAGS(106u32);
pub const DIIRFLAG_FORCE_INF: DIINSTALLDRIVER_FLAGS = DIINSTALLDRIVER_FLAGS(2u32);
pub const DIIRFLAG_HOTPATCH: DIINSTALLDRIVER_FLAGS = DIINSTALLDRIVER_FLAGS(8u32);
pub const DIIRFLAG_HW_USING_THE_INF: DIINSTALLDRIVER_FLAGS = DIINSTALLDRIVER_FLAGS(4u32);
pub const DIIRFLAG_INF_ALREADY_COPIED: DIINSTALLDRIVER_FLAGS = DIINSTALLDRIVER_FLAGS(1u32);
pub const DIIRFLAG_INSTALL_AS_SET: DIINSTALLDRIVER_FLAGS = DIINSTALLDRIVER_FLAGS(64u32);
pub const DIIRFLAG_NOBACKUP: DIINSTALLDRIVER_FLAGS = DIINSTALLDRIVER_FLAGS(16u32);
pub const DIIRFLAG_PRE_CONFIGURE_INF: DIINSTALLDRIVER_FLAGS = DIINSTALLDRIVER_FLAGS(32u32);
pub const DIIRFLAG_SYSTEM_BITS: DIINSTALLDRIVER_FLAGS = DIINSTALLDRIVER_FLAGS(127u32);
pub const DIOCR_INSTALLER: u32 = 1u32;
pub const DIOCR_INTERFACE: u32 = 2u32;
pub const DIODI_NO_ADD: u32 = 1u32;
pub const DIOD_CANCEL_REMOVE: u32 = 4u32;
pub const DIOD_INHERIT_CLASSDRVS: u32 = 2u32;
pub const DIREG_BOTH: u32 = 4u32;
pub const DIREG_DEV: u32 = 1u32;
pub const DIREG_DRV: u32 = 2u32;
pub const DIRID_ABSOLUTE: i32 = -1i32;
pub const DIRID_ABSOLUTE_16BIT: u32 = 65535u32;
pub const DIRID_APPS: u32 = 24u32;
pub const DIRID_BOOT: u32 = 30u32;
pub const DIRID_COLOR: u32 = 23u32;
pub const DIRID_COMMON_APPDATA: u32 = 16419u32;
pub const DIRID_COMMON_DESKTOPDIRECTORY: u32 = 16409u32;
pub const DIRID_COMMON_DOCUMENTS: u32 = 16430u32;
pub const DIRID_COMMON_FAVORITES: u32 = 16415u32;
pub const DIRID_COMMON_PROGRAMS: u32 = 16407u32;
pub const DIRID_COMMON_STARTMENU: u32 = 16406u32;
pub const DIRID_COMMON_STARTUP: u32 = 16408u32;
pub const DIRID_COMMON_TEMPLATES: u32 = 16429u32;
pub const DIRID_DEFAULT: u32 = 11u32;
pub const DIRID_DRIVERS: u32 = 12u32;
pub const DIRID_DRIVER_STORE: u32 = 13u32;
pub const DIRID_FONTS: u32 = 20u32;
pub const DIRID_HELP: u32 = 18u32;
pub const DIRID_INF: u32 = 17u32;
pub const DIRID_IOSUBSYS: u32 = 12u32;
pub const DIRID_LOADER: u32 = 54u32;
pub const DIRID_NULL: u32 = 0u32;
pub const DIRID_PRINTPROCESSOR: u32 = 55u32;
pub const DIRID_PROGRAM_FILES: u32 = 16422u32;
pub const DIRID_PROGRAM_FILES_COMMON: u32 = 16427u32;
pub const DIRID_PROGRAM_FILES_COMMONX86: u32 = 16428u32;
pub const DIRID_PROGRAM_FILES_X86: u32 = 16426u32;
pub const DIRID_SHARED: u32 = 25u32;
pub const DIRID_SPOOL: u32 = 51u32;
pub const DIRID_SPOOLDRIVERS: u32 = 52u32;
pub const DIRID_SRCPATH: u32 = 1u32;
pub const DIRID_SYSTEM: u32 = 11u32;
pub const DIRID_SYSTEM16: u32 = 50u32;
pub const DIRID_SYSTEM_X86: u32 = 16425u32;
pub const DIRID_USER: u32 = 32768u32;
pub const DIRID_USERPROFILE: u32 = 53u32;
pub const DIRID_VIEWERS: u32 = 21u32;
pub const DIRID_WINDOWS: u32 = 10u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DIROLLBACKDRIVER_FLAGS(pub u32);
impl DIROLLBACKDRIVER_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for DIROLLBACKDRIVER_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for DIROLLBACKDRIVER_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for DIROLLBACKDRIVER_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for DIROLLBACKDRIVER_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for DIROLLBACKDRIVER_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DIUNINSTALLDRIVER_FLAGS(pub u32);
impl DIUNINSTALLDRIVER_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for DIUNINSTALLDRIVER_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for DIUNINSTALLDRIVER_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for DIUNINSTALLDRIVER_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for DIUNINSTALLDRIVER_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for DIUNINSTALLDRIVER_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const DIURFLAG_NO_REMOVE_INF: DIUNINSTALLDRIVER_FLAGS = DIUNINSTALLDRIVER_FLAGS(1u32);
pub const DIURFLAG_RESERVED: DIUNINSTALLDRIVER_FLAGS = DIUNINSTALLDRIVER_FLAGS(2u32);
pub const DIURFLAG_VALID: DIUNINSTALLDRIVER_FLAGS = DIUNINSTALLDRIVER_FLAGS(3u32);
pub const DI_AUTOASSIGNRES: SETUP_DI_DEVICE_INSTALL_FLAGS = SETUP_DI_DEVICE_INSTALL_FLAGS(64u32);
pub const DI_CLASSINSTALLPARAMS: SETUP_DI_DEVICE_INSTALL_FLAGS = SETUP_DI_DEVICE_INSTALL_FLAGS(1048576u32);
pub const DI_COMPAT_FROM_CLASS: SETUP_DI_DEVICE_INSTALL_FLAGS = SETUP_DI_DEVICE_INSTALL_FLAGS(524288u32);
pub const DI_DIDCLASS: SETUP_DI_DEVICE_INSTALL_FLAGS = SETUP_DI_DEVICE_INSTALL_FLAGS(32u32);
pub const DI_DIDCOMPAT: SETUP_DI_DEVICE_INSTALL_FLAGS = SETUP_DI_DEVICE_INSTALL_FLAGS(16u32);
pub const DI_DISABLED: SETUP_DI_DEVICE_INSTALL_FLAGS = SETUP_DI_DEVICE_INSTALL_FLAGS(2048u32);
pub const DI_DONOTCALLCONFIGMG: SETUP_DI_DEVICE_INSTALL_FLAGS = SETUP_DI_DEVICE_INSTALL_FLAGS(131072u32);
pub const DI_DRIVERPAGE_ADDED: SETUP_DI_DEVICE_INSTALL_FLAGS = SETUP_DI_DEVICE_INSTALL_FLAGS(67108864u32);
pub const DI_ENUMSINGLEINF: SETUP_DI_DEVICE_INSTALL_FLAGS = SETUP_DI_DEVICE_INSTALL_FLAGS(65536u32);
pub const DI_FLAGSEX_ALLOWEXCLUDEDDRVS: SETUP_DI_DEVICE_INSTALL_FLAGS_EX = SETUP_DI_DEVICE_INSTALL_FLAGS_EX(2048u32);
pub const DI_FLAGSEX_ALTPLATFORM_DRVSEARCH: SETUP_DI_DEVICE_INSTALL_FLAGS_EX = SETUP_DI_DEVICE_INSTALL_FLAGS_EX(268435456u32);
pub const DI_FLAGSEX_ALWAYSWRITEIDS: SETUP_DI_DEVICE_INSTALL_FLAGS_EX = SETUP_DI_DEVICE_INSTALL_FLAGS_EX(512u32);
pub const DI_FLAGSEX_APPENDDRIVERLIST: SETUP_DI_DEVICE_INSTALL_FLAGS_EX = SETUP_DI_DEVICE_INSTALL_FLAGS_EX(262144u32);
pub const DI_FLAGSEX_BACKUPONREPLACE: SETUP_DI_DEVICE_INSTALL_FLAGS_EX = SETUP_DI_DEVICE_INSTALL_FLAGS_EX(1048576u32);
pub const DI_FLAGSEX_CI_FAILED: SETUP_DI_DEVICE_INSTALL_FLAGS_EX = SETUP_DI_DEVICE_INSTALL_FLAGS_EX(4u32);
pub const DI_FLAGSEX_DEVICECHANGE: SETUP_DI_DEVICE_INSTALL_FLAGS_EX = SETUP_DI_DEVICE_INSTALL_FLAGS_EX(256u32);
pub const DI_FLAGSEX_DIDCOMPATINFO: SETUP_DI_DEVICE_INSTALL_FLAGS_EX = SETUP_DI_DEVICE_INSTALL_FLAGS_EX(32u32);
pub const DI_FLAGSEX_DIDINFOLIST: SETUP_DI_DEVICE_INSTALL_FLAGS_EX = SETUP_DI_DEVICE_INSTALL_FLAGS_EX(16u32);
pub const DI_FLAGSEX_DRIVERLIST_FROM_URL: SETUP_DI_DEVICE_INSTALL_FLAGS_EX = SETUP_DI_DEVICE_INSTALL_FLAGS_EX(2097152u32);
pub const DI_FLAGSEX_EXCLUDE_OLD_INET_DRIVERS: SETUP_DI_DEVICE_INSTALL_FLAGS_EX = SETUP_DI_DEVICE_INSTALL_FLAGS_EX(8388608u32);
pub const DI_FLAGSEX_FILTERCLASSES: SETUP_DI_DEVICE_INSTALL_FLAGS_EX = SETUP_DI_DEVICE_INSTALL_FLAGS_EX(64u32);
pub const DI_FLAGSEX_FILTERSIMILARDRIVERS: SETUP_DI_DEVICE_INSTALL_FLAGS_EX = SETUP_DI_DEVICE_INSTALL_FLAGS_EX(33554432u32);
pub const DI_FLAGSEX_FINISHINSTALL_ACTION: SETUP_DI_DEVICE_INSTALL_FLAGS_EX = SETUP_DI_DEVICE_INSTALL_FLAGS_EX(8u32);
pub const DI_FLAGSEX_INET_DRIVER: SETUP_DI_DEVICE_INSTALL_FLAGS_EX = SETUP_DI_DEVICE_INSTALL_FLAGS_EX(131072u32);
pub const DI_FLAGSEX_INSTALLEDDRIVER: SETUP_DI_DEVICE_INSTALL_FLAGS_EX = SETUP_DI_DEVICE_INSTALL_FLAGS_EX(67108864u32);
pub const DI_FLAGSEX_IN_SYSTEM_SETUP: SETUP_DI_DEVICE_INSTALL_FLAGS_EX = SETUP_DI_DEVICE_INSTALL_FLAGS_EX(65536u32);
pub const DI_FLAGSEX_NOUIONQUERYREMOVE: SETUP_DI_DEVICE_INSTALL_FLAGS_EX = SETUP_DI_DEVICE_INSTALL_FLAGS_EX(4096u32);
pub const DI_FLAGSEX_NO_CLASSLIST_NODE_MERGE: SETUP_DI_DEVICE_INSTALL_FLAGS_EX = SETUP_DI_DEVICE_INSTALL_FLAGS_EX(134217728u32);
pub const DI_FLAGSEX_NO_DRVREG_MODIFY: SETUP_DI_DEVICE_INSTALL_FLAGS_EX = SETUP_DI_DEVICE_INSTALL_FLAGS_EX(32768u32);
pub const DI_FLAGSEX_POWERPAGE_ADDED: SETUP_DI_DEVICE_INSTALL_FLAGS_EX = SETUP_DI_DEVICE_INSTALL_FLAGS_EX(16777216u32);
pub const DI_FLAGSEX_PREINSTALLBACKUP: SETUP_DI_DEVICE_INSTALL_FLAGS_EX = SETUP_DI_DEVICE_INSTALL_FLAGS_EX(524288u32);
pub const DI_FLAGSEX_PROPCHANGE_PENDING: SETUP_DI_DEVICE_INSTALL_FLAGS_EX = SETUP_DI_DEVICE_INSTALL_FLAGS_EX(1024u32);
pub const DI_FLAGSEX_RECURSIVESEARCH: SETUP_DI_DEVICE_INSTALL_FLAGS_EX = SETUP_DI_DEVICE_INSTALL_FLAGS_EX(1073741824u32);
pub const DI_FLAGSEX_RESERVED1: SETUP_DI_DEVICE_INSTALL_FLAGS_EX = SETUP_DI_DEVICE_INSTALL_FLAGS_EX(4194304u32);
pub const DI_FLAGSEX_RESERVED2: SETUP_DI_DEVICE_INSTALL_FLAGS_EX = SETUP_DI_DEVICE_INSTALL_FLAGS_EX(1u32);
pub const DI_FLAGSEX_RESERVED3: SETUP_DI_DEVICE_INSTALL_FLAGS_EX = SETUP_DI_DEVICE_INSTALL_FLAGS_EX(2u32);
pub const DI_FLAGSEX_RESERVED4: SETUP_DI_DEVICE_INSTALL_FLAGS_EX = SETUP_DI_DEVICE_INSTALL_FLAGS_EX(16384u32);
pub const DI_FLAGSEX_RESTART_DEVICE_ONLY: SETUP_DI_DEVICE_INSTALL_FLAGS_EX = SETUP_DI_DEVICE_INSTALL_FLAGS_EX(536870912u32);
pub const DI_FLAGSEX_SEARCH_PUBLISHED_INFS: SETUP_DI_DEVICE_INSTALL_FLAGS_EX = SETUP_DI_DEVICE_INSTALL_FLAGS_EX(2147483648u32);
pub const DI_FLAGSEX_SETFAILEDINSTALL: SETUP_DI_DEVICE_INSTALL_FLAGS_EX = SETUP_DI_DEVICE_INSTALL_FLAGS_EX(128u32);
pub const DI_FLAGSEX_USECLASSFORCOMPAT: SETUP_DI_DEVICE_INSTALL_FLAGS_EX = SETUP_DI_DEVICE_INSTALL_FLAGS_EX(8192u32);
pub const DI_FORCECOPY: SETUP_DI_DEVICE_INSTALL_FLAGS = SETUP_DI_DEVICE_INSTALL_FLAGS(33554432u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DI_FUNCTION(pub u32);
pub const DI_GENERALPAGE_ADDED: SETUP_DI_DEVICE_INSTALL_FLAGS = SETUP_DI_DEVICE_INSTALL_FLAGS(4096u32);
pub const DI_INF_IS_SORTED: SETUP_DI_DEVICE_INSTALL_FLAGS = SETUP_DI_DEVICE_INSTALL_FLAGS(32768u32);
pub const DI_INSTALLDISABLED: SETUP_DI_DEVICE_INSTALL_FLAGS = SETUP_DI_DEVICE_INSTALL_FLAGS(262144u32);
pub const DI_MULTMFGS: SETUP_DI_DEVICE_INSTALL_FLAGS = SETUP_DI_DEVICE_INSTALL_FLAGS(1024u32);
pub const DI_NEEDREBOOT: SETUP_DI_DEVICE_INSTALL_FLAGS = SETUP_DI_DEVICE_INSTALL_FLAGS(256u32);
pub const DI_NEEDRESTART: SETUP_DI_DEVICE_INSTALL_FLAGS = SETUP_DI_DEVICE_INSTALL_FLAGS(128u32);
pub const DI_NOBROWSE: SETUP_DI_DEVICE_INSTALL_FLAGS = SETUP_DI_DEVICE_INSTALL_FLAGS(512u32);
pub const DI_NODI_DEFAULTACTION: SETUP_DI_DEVICE_INSTALL_FLAGS = SETUP_DI_DEVICE_INSTALL_FLAGS(2097152u32);
pub const DI_NOFILECOPY: SETUP_DI_DEVICE_INSTALL_FLAGS = SETUP_DI_DEVICE_INSTALL_FLAGS(16777216u32);
pub const DI_NOSELECTICONS: SETUP_DI_DEVICE_INSTALL_FLAGS = SETUP_DI_DEVICE_INSTALL_FLAGS(1073741824u32);
pub const DI_NOVCP: SETUP_DI_DEVICE_INSTALL_FLAGS = SETUP_DI_DEVICE_INSTALL_FLAGS(8u32);
pub const DI_NOWRITE_IDS: SETUP_DI_DEVICE_INSTALL_FLAGS = SETUP_DI_DEVICE_INSTALL_FLAGS(2147483648u32);
pub const DI_OVERRIDE_INFFLAGS: SETUP_DI_DEVICE_INSTALL_FLAGS = SETUP_DI_DEVICE_INSTALL_FLAGS(268435456u32);
pub const DI_PROPERTIES_CHANGE: SETUP_DI_DEVICE_INSTALL_FLAGS = SETUP_DI_DEVICE_INSTALL_FLAGS(16384u32);
pub const DI_PROPS_NOCHANGEUSAGE: SETUP_DI_DEVICE_INSTALL_FLAGS = SETUP_DI_DEVICE_INSTALL_FLAGS(536870912u32);
pub const DI_QUIETINSTALL: SETUP_DI_DEVICE_INSTALL_FLAGS = SETUP_DI_DEVICE_INSTALL_FLAGS(8388608u32);
pub const DI_REMOVEDEVICE_CONFIGSPECIFIC: SETUP_DI_REMOVE_DEVICE_SCOPE = SETUP_DI_REMOVE_DEVICE_SCOPE(2u32);
pub const DI_REMOVEDEVICE_GLOBAL: SETUP_DI_REMOVE_DEVICE_SCOPE = SETUP_DI_REMOVE_DEVICE_SCOPE(1u32);
pub const DI_RESOURCEPAGE_ADDED: SETUP_DI_DEVICE_INSTALL_FLAGS = SETUP_DI_DEVICE_INSTALL_FLAGS(8192u32);
pub const DI_SHOWALL: SETUP_DI_DEVICE_INSTALL_FLAGS = SETUP_DI_DEVICE_INSTALL_FLAGS(7u32);
pub const DI_SHOWCLASS: SETUP_DI_DEVICE_INSTALL_FLAGS = SETUP_DI_DEVICE_INSTALL_FLAGS(4u32);
pub const DI_SHOWCOMPAT: SETUP_DI_DEVICE_INSTALL_FLAGS = SETUP_DI_DEVICE_INSTALL_FLAGS(2u32);
pub const DI_SHOWOEM: SETUP_DI_DEVICE_INSTALL_FLAGS = SETUP_DI_DEVICE_INSTALL_FLAGS(1u32);
pub const DI_UNREMOVEDEVICE_CONFIGSPECIFIC: SETUP_DI_DEVICE_INSTALL_FLAGS = SETUP_DI_DEVICE_INSTALL_FLAGS(2u32);
pub const DI_USECI_SELECTSTRINGS: SETUP_DI_DEVICE_INSTALL_FLAGS = SETUP_DI_DEVICE_INSTALL_FLAGS(134217728u32);
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct DMA_DES {
    pub DD_Count: u32,
    pub DD_Type: u32,
    pub DD_Flags: DD_FLAGS,
    pub DD_Alloc_Chan: u32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct DMA_RANGE {
    pub DR_Min: u32,
    pub DR_Max: u32,
    pub DR_Flags: DD_FLAGS,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct DMA_RESOURCE {
    pub DMA_Header: DMA_DES,
    pub DMA_Data: [DMA_RANGE; 1],
}
impl Default for DMA_RESOURCE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DMI_BKCOLOR: u32 = 2u32;
pub const DMI_MASK: u32 = 1u32;
pub const DMI_USERECT: u32 = 4u32;
pub const DNF_ALWAYSEXCLUDEFROMLIST: SETUP_DI_DRIVER_INSTALL_FLAGS = SETUP_DI_DRIVER_INSTALL_FLAGS(524288u32);
pub const DNF_AUTHENTICODE_SIGNED: SETUP_DI_DRIVER_INSTALL_FLAGS = SETUP_DI_DRIVER_INSTALL_FLAGS(131072u32);
pub const DNF_BAD_DRIVER: SETUP_DI_DRIVER_INSTALL_FLAGS = SETUP_DI_DRIVER_INSTALL_FLAGS(2048u32);
pub const DNF_BASIC_DRIVER: SETUP_DI_DRIVER_INSTALL_FLAGS = SETUP_DI_DRIVER_INSTALL_FLAGS(65536u32);
pub const DNF_CLASS_DRIVER: SETUP_DI_DRIVER_INSTALL_FLAGS = SETUP_DI_DRIVER_INSTALL_FLAGS(32u32);
pub const DNF_COMPATIBLE_DRIVER: SETUP_DI_DRIVER_INSTALL_FLAGS = SETUP_DI_DRIVER_INSTALL_FLAGS(64u32);
pub const DNF_DUPDESC: SETUP_DI_DRIVER_INSTALL_FLAGS = SETUP_DI_DRIVER_INSTALL_FLAGS(1u32);
pub const DNF_DUPDRIVERVER: SETUP_DI_DRIVER_INSTALL_FLAGS = SETUP_DI_DRIVER_INSTALL_FLAGS(32768u32);
pub const DNF_DUPPROVIDER: SETUP_DI_DRIVER_INSTALL_FLAGS = SETUP_DI_DRIVER_INSTALL_FLAGS(4096u32);
pub const DNF_EXCLUDEFROMLIST: SETUP_DI_DRIVER_INSTALL_FLAGS = SETUP_DI_DRIVER_INSTALL_FLAGS(4u32);
pub const DNF_INBOX_DRIVER: SETUP_DI_DRIVER_INSTALL_FLAGS = SETUP_DI_DRIVER_INSTALL_FLAGS(1048576u32);
pub const DNF_INET_DRIVER: SETUP_DI_DRIVER_INSTALL_FLAGS = SETUP_DI_DRIVER_INSTALL_FLAGS(128u32);
pub const DNF_INF_IS_SIGNED: SETUP_DI_DRIVER_INSTALL_FLAGS = SETUP_DI_DRIVER_INSTALL_FLAGS(8192u32);
pub const DNF_INSTALLEDDRIVER: SETUP_DI_DRIVER_INSTALL_FLAGS = SETUP_DI_DRIVER_INSTALL_FLAGS(262144u32);
pub const DNF_LEGACYINF: SETUP_DI_DRIVER_INSTALL_FLAGS = SETUP_DI_DRIVER_INSTALL_FLAGS(16u32);
pub const DNF_NODRIVER: SETUP_DI_DRIVER_INSTALL_FLAGS = SETUP_DI_DRIVER_INSTALL_FLAGS(8u32);
pub const DNF_OEM_F6_INF: SETUP_DI_DRIVER_INSTALL_FLAGS = SETUP_DI_DRIVER_INSTALL_FLAGS(16384u32);
pub const DNF_OLDDRIVER: SETUP_DI_DRIVER_INSTALL_FLAGS = SETUP_DI_DRIVER_INSTALL_FLAGS(2u32);
pub const DNF_OLD_INET_DRIVER: SETUP_DI_DRIVER_INSTALL_FLAGS = SETUP_DI_DRIVER_INSTALL_FLAGS(1024u32);
pub const DNF_REQUESTADDITIONALSOFTWARE: SETUP_DI_DRIVER_INSTALL_FLAGS = SETUP_DI_DRIVER_INSTALL_FLAGS(2097152u32);
pub const DNF_UNUSED1: SETUP_DI_DRIVER_INSTALL_FLAGS = SETUP_DI_DRIVER_INSTALL_FLAGS(256u32);
pub const DNF_UNUSED2: SETUP_DI_DRIVER_INSTALL_FLAGS = SETUP_DI_DRIVER_INSTALL_FLAGS(512u32);
pub const DNF_UNUSED_22: SETUP_DI_DRIVER_INSTALL_FLAGS = SETUP_DI_DRIVER_INSTALL_FLAGS(4194304u32);
pub const DNF_UNUSED_23: SETUP_DI_DRIVER_INSTALL_FLAGS = SETUP_DI_DRIVER_INSTALL_FLAGS(8388608u32);
pub const DNF_UNUSED_24: SETUP_DI_DRIVER_INSTALL_FLAGS = SETUP_DI_DRIVER_INSTALL_FLAGS(16777216u32);
pub const DNF_UNUSED_25: SETUP_DI_DRIVER_INSTALL_FLAGS = SETUP_DI_DRIVER_INSTALL_FLAGS(33554432u32);
pub const DNF_UNUSED_26: SETUP_DI_DRIVER_INSTALL_FLAGS = SETUP_DI_DRIVER_INSTALL_FLAGS(67108864u32);
pub const DNF_UNUSED_27: SETUP_DI_DRIVER_INSTALL_FLAGS = SETUP_DI_DRIVER_INSTALL_FLAGS(134217728u32);
pub const DNF_UNUSED_28: SETUP_DI_DRIVER_INSTALL_FLAGS = SETUP_DI_DRIVER_INSTALL_FLAGS(268435456u32);
pub const DNF_UNUSED_29: SETUP_DI_DRIVER_INSTALL_FLAGS = SETUP_DI_DRIVER_INSTALL_FLAGS(536870912u32);
pub const DNF_UNUSED_30: SETUP_DI_DRIVER_INSTALL_FLAGS = SETUP_DI_DRIVER_INSTALL_FLAGS(1073741824u32);
pub const DNF_UNUSED_31: SETUP_DI_DRIVER_INSTALL_FLAGS = SETUP_DI_DRIVER_INSTALL_FLAGS(2147483648u32);
pub const DN_APM_DRIVER: CM_DEVNODE_STATUS_FLAGS = CM_DEVNODE_STATUS_FLAGS(268435456u32);
pub const DN_APM_ENUMERATOR: CM_DEVNODE_STATUS_FLAGS = CM_DEVNODE_STATUS_FLAGS(134217728u32);
pub const DN_ARM_WAKEUP: CM_DEVNODE_STATUS_FLAGS = CM_DEVNODE_STATUS_FLAGS(67108864u32);
pub const DN_BAD_PARTIAL: CM_DEVNODE_STATUS_FLAGS = CM_DEVNODE_STATUS_FLAGS(4194304u32);
pub const DN_BOOT_LOG_PROB: CM_DEVNODE_STATUS_FLAGS = CM_DEVNODE_STATUS_FLAGS(2147483648u32);
pub const DN_CHANGEABLE_FLAGS: CM_DEVNODE_STATUS_FLAGS = CM_DEVNODE_STATUS_FLAGS(1639670464u32);
pub const DN_CHILD_WITH_INVALID_ID: CM_DEVNODE_STATUS_FLAGS = CM_DEVNODE_STATUS_FLAGS(512u32);
pub const DN_DEVICE_DISCONNECTED: CM_DEVNODE_STATUS_FLAGS = CM_DEVNODE_STATUS_FLAGS(33554432u32);
pub const DN_DISABLEABLE: CM_DEVNODE_STATUS_FLAGS = CM_DEVNODE_STATUS_FLAGS(8192u32);
pub const DN_DRIVER_BLOCKED: CM_DEVNODE_STATUS_FLAGS = CM_DEVNODE_STATUS_FLAGS(64u32);
pub const DN_DRIVER_LOADED: CM_DEVNODE_STATUS_FLAGS = CM_DEVNODE_STATUS_FLAGS(2u32);
pub const DN_ENUM_LOADED: CM_DEVNODE_STATUS_FLAGS = CM_DEVNODE_STATUS_FLAGS(4u32);
pub const DN_FILTERED: CM_DEVNODE_STATUS_FLAGS = CM_DEVNODE_STATUS_FLAGS(2048u32);
pub const DN_HARDWARE_ENUM: CM_DEVNODE_STATUS_FLAGS = CM_DEVNODE_STATUS_FLAGS(128u32);
pub const DN_HAS_MARK: CM_DEVNODE_STATUS_FLAGS = CM_DEVNODE_STATUS_FLAGS(512u32);
pub const DN_HAS_PROBLEM: CM_DEVNODE_STATUS_FLAGS = CM_DEVNODE_STATUS_FLAGS(1024u32);
pub const DN_LEGACY_DRIVER: CM_DEVNODE_STATUS_FLAGS = CM_DEVNODE_STATUS_FLAGS(4096u32);
pub const DN_LIAR: CM_DEVNODE_STATUS_FLAGS = CM_DEVNODE_STATUS_FLAGS(256u32);
pub const DN_MANUAL: CM_DEVNODE_STATUS_FLAGS = CM_DEVNODE_STATUS_FLAGS(16u32);
pub const DN_MF_CHILD: CM_DEVNODE_STATUS_FLAGS = CM_DEVNODE_STATUS_FLAGS(131072u32);
pub const DN_MF_PARENT: CM_DEVNODE_STATUS_FLAGS = CM_DEVNODE_STATUS_FLAGS(65536u32);
pub const DN_MOVED: CM_DEVNODE_STATUS_FLAGS = CM_DEVNODE_STATUS_FLAGS(4096u32);
pub const DN_NEEDS_LOCKING: CM_DEVNODE_STATUS_FLAGS = CM_DEVNODE_STATUS_FLAGS(33554432u32);
pub const DN_NEED_RESTART: CM_DEVNODE_STATUS_FLAGS = CM_DEVNODE_STATUS_FLAGS(256u32);
pub const DN_NEED_TO_ENUM: CM_DEVNODE_STATUS_FLAGS = CM_DEVNODE_STATUS_FLAGS(32u32);
pub const DN_NOT_FIRST_TIME: CM_DEVNODE_STATUS_FLAGS = CM_DEVNODE_STATUS_FLAGS(64u32);
pub const DN_NOT_FIRST_TIMEE: CM_DEVNODE_STATUS_FLAGS = CM_DEVNODE_STATUS_FLAGS(524288u32);
pub const DN_NO_SHOW_IN_DM: CM_DEVNODE_STATUS_FLAGS = CM_DEVNODE_STATUS_FLAGS(1073741824u32);
pub const DN_NT_DRIVER: CM_DEVNODE_STATUS_FLAGS = CM_DEVNODE_STATUS_FLAGS(16777216u32);
pub const DN_NT_ENUMERATOR: CM_DEVNODE_STATUS_FLAGS = CM_DEVNODE_STATUS_FLAGS(8388608u32);
pub const DN_PRIVATE_PROBLEM: CM_DEVNODE_STATUS_FLAGS = CM_DEVNODE_STATUS_FLAGS(32768u32);
pub const DN_QUERY_REMOVE_ACTIVE: CM_DEVNODE_STATUS_FLAGS = CM_DEVNODE_STATUS_FLAGS(131072u32);
pub const DN_QUERY_REMOVE_PENDING: CM_DEVNODE_STATUS_FLAGS = CM_DEVNODE_STATUS_FLAGS(65536u32);
pub const DN_REBAL_CANDIDATE: CM_DEVNODE_STATUS_FLAGS = CM_DEVNODE_STATUS_FLAGS(2097152u32);
pub const DN_REMOVABLE: CM_DEVNODE_STATUS_FLAGS = CM_DEVNODE_STATUS_FLAGS(16384u32);
pub const DN_ROOT_ENUMERATED: CM_DEVNODE_STATUS_FLAGS = CM_DEVNODE_STATUS_FLAGS(1u32);
pub const DN_SILENT_INSTALL: CM_DEVNODE_STATUS_FLAGS = CM_DEVNODE_STATUS_FLAGS(536870912u32);
pub const DN_STARTED: CM_DEVNODE_STATUS_FLAGS = CM_DEVNODE_STATUS_FLAGS(8u32);
pub const DN_STOP_FREE_RES: CM_DEVNODE_STATUS_FLAGS = CM_DEVNODE_STATUS_FLAGS(1048576u32);
pub const DN_WILL_BE_REMOVED: CM_DEVNODE_STATUS_FLAGS = CM_DEVNODE_STATUS_FLAGS(262144u32);
pub const DPROMPT_BUFFERTOOSMALL: u32 = 3u32;
pub const DPROMPT_CANCEL: u32 = 1u32;
pub const DPROMPT_OUTOFMEMORY: u32 = 4u32;
pub const DPROMPT_SKIPFILE: u32 = 2u32;
pub const DPROMPT_SUCCESS: u32 = 0u32;
pub const DRIVER_COMPATID_RANK: u32 = 16383u32;
pub const DRIVER_HARDWAREID_MASK: u32 = 2147487743u32;
pub const DRIVER_HARDWAREID_RANK: u32 = 4095u32;
pub const DRIVER_UNTRUSTED_COMPATID_RANK: u32 = 49151u32;
pub const DRIVER_UNTRUSTED_HARDWAREID_RANK: u32 = 36863u32;
pub const DRIVER_UNTRUSTED_RANK: u32 = 2147483648u32;
pub const DRIVER_W9X_SUSPECT_COMPATID_RANK: u32 = 65535u32;
pub const DRIVER_W9X_SUSPECT_HARDWAREID_RANK: u32 = 53247u32;
pub const DRIVER_W9X_SUSPECT_RANK: u32 = 3221225472u32;
pub const DWORD_MAX: u32 = 4294967295u32;
pub const DYNAWIZ_FLAG_ANALYZE_HANDLECONFLICT: u32 = 8u32;
pub const DYNAWIZ_FLAG_INSTALLDET_NEXT: u32 = 2u32;
pub const DYNAWIZ_FLAG_INSTALLDET_PREV: u32 = 4u32;
pub const DYNAWIZ_FLAG_PAGESADDED: u32 = 1u32;
pub const ENABLECLASS_FAILURE: u32 = 2u32;
pub const ENABLECLASS_QUERY: u32 = 0u32;
pub const ENABLECLASS_SUCCESS: u32 = 1u32;
pub const FILEOP_ABORT: u32 = 0u32;
pub const FILEOP_BACKUP: u32 = 3u32;
pub const FILEOP_COPY: SETUP_FILE_OPERATION = SETUP_FILE_OPERATION(0u32);
pub const FILEOP_DELETE: SETUP_FILE_OPERATION = SETUP_FILE_OPERATION(2u32);
pub const FILEOP_DOIT: u32 = 1u32;
pub const FILEOP_NEWPATH: u32 = 4u32;
pub const FILEOP_RENAME: u32 = 1u32;
pub const FILEOP_RETRY: u32 = 1u32;
pub const FILEOP_SKIP: u32 = 2u32;
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy, Default)]
pub struct FILEPATHS_A {
    pub Target: windows_core::PCSTR,
    pub Source: windows_core::PCSTR,
    pub Win32Error: u32,
    pub Flags: u32,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct FILEPATHS_A {
    pub Target: windows_core::PCSTR,
    pub Source: windows_core::PCSTR,
    pub Win32Error: u32,
    pub Flags: u32,
}
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy, Default)]
pub struct FILEPATHS_SIGNERINFO_A {
    pub Target: windows_core::PCSTR,
    pub Source: windows_core::PCSTR,
    pub Win32Error: u32,
    pub Flags: u32,
    pub DigitalSigner: windows_core::PCSTR,
    pub Version: windows_core::PCSTR,
    pub CatalogFile: windows_core::PCSTR,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct FILEPATHS_SIGNERINFO_A {
    pub Target: windows_core::PCSTR,
    pub Source: windows_core::PCSTR,
    pub Win32Error: u32,
    pub Flags: u32,
    pub DigitalSigner: windows_core::PCSTR,
    pub Version: windows_core::PCSTR,
    pub CatalogFile: windows_core::PCSTR,
}
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy, Default)]
pub struct FILEPATHS_SIGNERINFO_W {
    pub Target: windows_core::PCWSTR,
    pub Source: windows_core::PCWSTR,
    pub Win32Error: u32,
    pub Flags: u32,
    pub DigitalSigner: windows_core::PCWSTR,
    pub Version: windows_core::PCWSTR,
    pub CatalogFile: windows_core::PCWSTR,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct FILEPATHS_SIGNERINFO_W {
    pub Target: windows_core::PCWSTR,
    pub Source: windows_core::PCWSTR,
    pub Win32Error: u32,
    pub Flags: u32,
    pub DigitalSigner: windows_core::PCWSTR,
    pub Version: windows_core::PCWSTR,
    pub CatalogFile: windows_core::PCWSTR,
}
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy, Default)]
pub struct FILEPATHS_W {
    pub Target: windows_core::PCWSTR,
    pub Source: windows_core::PCWSTR,
    pub Win32Error: u32,
    pub Flags: u32,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct FILEPATHS_W {
    pub Target: windows_core::PCWSTR,
    pub Source: windows_core::PCWSTR,
    pub Win32Error: u32,
    pub Flags: u32,
}
pub const FILE_COMPRESSION_MSZIP: FILE_COMPRESSION_TYPE = FILE_COMPRESSION_TYPE(2u32);
pub const FILE_COMPRESSION_NONE: FILE_COMPRESSION_TYPE = FILE_COMPRESSION_TYPE(0u32);
pub const FILE_COMPRESSION_NTCAB: FILE_COMPRESSION_TYPE = FILE_COMPRESSION_TYPE(3u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FILE_COMPRESSION_TYPE(pub u32);
impl FILE_COMPRESSION_TYPE {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for FILE_COMPRESSION_TYPE {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for FILE_COMPRESSION_TYPE {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for FILE_COMPRESSION_TYPE {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for FILE_COMPRESSION_TYPE {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for FILE_COMPRESSION_TYPE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const FILE_COMPRESSION_WINLZA: FILE_COMPRESSION_TYPE = FILE_COMPRESSION_TYPE(1u32);
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy)]
pub struct FILE_IN_CABINET_INFO_A {
    pub NameInCabinet: windows_core::PCSTR,
    pub FileSize: u32,
    pub Win32Error: u32,
    pub DosDate: u16,
    pub DosTime: u16,
    pub DosAttribs: u16,
    pub FullTargetName: [i8; 260],
}
#[cfg(target_arch = "x86")]
impl Default for FILE_IN_CABINET_INFO_A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FILE_IN_CABINET_INFO_A {
    pub NameInCabinet: windows_core::PCSTR,
    pub FileSize: u32,
    pub Win32Error: u32,
    pub DosDate: u16,
    pub DosTime: u16,
    pub DosAttribs: u16,
    pub FullTargetName: [i8; 260],
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for FILE_IN_CABINET_INFO_A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy)]
pub struct FILE_IN_CABINET_INFO_W {
    pub NameInCabinet: windows_core::PCWSTR,
    pub FileSize: u32,
    pub Win32Error: u32,
    pub DosDate: u16,
    pub DosTime: u16,
    pub DosAttribs: u16,
    pub FullTargetName: [u16; 260],
}
#[cfg(target_arch = "x86")]
impl Default for FILE_IN_CABINET_INFO_W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FILE_IN_CABINET_INFO_W {
    pub NameInCabinet: windows_core::PCWSTR,
    pub FileSize: u32,
    pub Win32Error: u32,
    pub DosDate: u16,
    pub DosTime: u16,
    pub DosAttribs: u16,
    pub FullTargetName: [u16; 260],
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for FILE_IN_CABINET_INFO_W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const FILTERED_LOG_CONF: CM_LOG_CONF = CM_LOG_CONF(1u32);
pub const FLG_ADDPROPERTY_AND: u32 = 16u32;
pub const FLG_ADDPROPERTY_APPEND: u32 = 4u32;
pub const FLG_ADDPROPERTY_NOCLOBBER: u32 = 1u32;
pub const FLG_ADDPROPERTY_OR: u32 = 8u32;
pub const FLG_ADDPROPERTY_OVERWRITEONLY: u32 = 2u32;
pub const FLG_ADDREG_32BITKEY: u32 = 16384u32;
pub const FLG_ADDREG_64BITKEY: u32 = 4096u32;
pub const FLG_ADDREG_APPEND: u32 = 8u32;
pub const FLG_ADDREG_BINVALUETYPE: u32 = 1u32;
pub const FLG_ADDREG_DELREG_BIT: u32 = 32768u32;
pub const FLG_ADDREG_DELVAL: u32 = 4u32;
pub const FLG_ADDREG_KEYONLY: u32 = 16u32;
pub const FLG_ADDREG_KEYONLY_COMMON: u32 = 8192u32;
pub const FLG_ADDREG_NOCLOBBER: u32 = 2u32;
pub const FLG_ADDREG_OVERWRITEONLY: u32 = 32u32;
pub const FLG_ADDREG_TYPE_EXPAND_SZ: u32 = 131072u32;
pub const FLG_ADDREG_TYPE_MULTI_SZ: u32 = 65536u32;
pub const FLG_ADDREG_TYPE_SZ: u32 = 0u32;
pub const FLG_BITREG_32BITKEY: u32 = 16384u32;
pub const FLG_BITREG_64BITKEY: u32 = 4096u32;
pub const FLG_BITREG_CLEARBITS: u32 = 0u32;
pub const FLG_BITREG_SETBITS: u32 = 1u32;
pub const FLG_DELPROPERTY_MULTI_SZ_DELSTRING: u32 = 1u32;
pub const FLG_DELREG_32BITKEY: u32 = 16384u32;
pub const FLG_DELREG_64BITKEY: u32 = 4096u32;
pub const FLG_DELREG_KEYONLY_COMMON: u32 = 8192u32;
pub const FLG_DELREG_OPERATION_MASK: u32 = 254u32;
pub const FLG_DELREG_TYPE_EXPAND_SZ: u32 = 131072u32;
pub const FLG_DELREG_TYPE_MULTI_SZ: u32 = 65536u32;
pub const FLG_DELREG_TYPE_SZ: u32 = 0u32;
pub const FLG_DELREG_VALUE: u32 = 0u32;
pub const FLG_INI2REG_32BITKEY: u32 = 16384u32;
pub const FLG_INI2REG_64BITKEY: u32 = 4096u32;
pub const FLG_PROFITEM_CSIDL: u32 = 8u32;
pub const FLG_PROFITEM_CURRENTUSER: u32 = 1u32;
pub const FLG_PROFITEM_DELETE: u32 = 2u32;
pub const FLG_PROFITEM_GROUP: u32 = 4u32;
pub const FLG_REGSVR_DLLINSTALL: u32 = 2u32;
pub const FLG_REGSVR_DLLREGISTER: u32 = 1u32;
pub const FORCED_LOG_CONF: CM_LOG_CONF = CM_LOG_CONF(4u32);
pub const GUID_ACPI_CMOS_INTERFACE_STANDARD: windows_core::GUID = windows_core::GUID::from_u128(0x3a8d0384_6505_40ca_bc39_56c15f8c5fed);
pub const GUID_ACPI_INTERFACE_STANDARD: windows_core::GUID = windows_core::GUID::from_u128(0xb091a08a_ba97_11d0_bd14_00aa00b7b32a);
pub const GUID_ACPI_INTERFACE_STANDARD2: windows_core::GUID = windows_core::GUID::from_u128(0xe8695f63_1831_4870_a8cf_9c2f03f9dcb5);
pub const GUID_ACPI_PORT_RANGES_INTERFACE_STANDARD: windows_core::GUID = windows_core::GUID::from_u128(0xf14f609b_cbbd_4957_a674_bc00213f1c97);
pub const GUID_ACPI_REGS_INTERFACE_STANDARD: windows_core::GUID = windows_core::GUID::from_u128(0x06141966_7245_6369_462e_4e656c736f6e);
pub const GUID_AGP_TARGET_BUS_INTERFACE_STANDARD: windows_core::GUID = windows_core::GUID::from_u128(0xb15cfce8_06d1_4d37_9d4c_bedde0c2a6ff);
pub const GUID_ARBITER_INTERFACE_STANDARD: windows_core::GUID = windows_core::GUID::from_u128(0xe644f185_8c0e_11d0_becf_08002be2092f);
pub const GUID_BUS_INTERFACE_STANDARD: windows_core::GUID = windows_core::GUID::from_u128(0x496b8280_6f25_11d0_beaf_08002be2092f);
pub const GUID_BUS_RESOURCE_UPDATE_INTERFACE: windows_core::GUID = windows_core::GUID::from_u128(0x27d0102d_bfb2_4164_81dd_dbb82f968b48);
pub const GUID_BUS_TYPE_1394: windows_core::GUID = windows_core::GUID::from_u128(0xf74e73eb_9ac5_45eb_be4d_772cc71ddfb3);
pub const GUID_BUS_TYPE_ACPI: windows_core::GUID = windows_core::GUID::from_u128(0xd7b46895_001a_4942_891f_a7d46610a843);
pub const GUID_BUS_TYPE_AVC: windows_core::GUID = windows_core::GUID::from_u128(0xc06ff265_ae09_48f0_812c_16753d7cba83);
pub const GUID_BUS_TYPE_DOT4PRT: windows_core::GUID = windows_core::GUID::from_u128(0x441ee001_4342_11d5_a184_00c04f60524d);
pub const GUID_BUS_TYPE_EISA: windows_core::GUID = windows_core::GUID::from_u128(0xddc35509_f3fc_11d0_a537_0000f8753ed1);
pub const GUID_BUS_TYPE_HID: windows_core::GUID = windows_core::GUID::from_u128(0xeeaf37d0_1963_47c4_aa48_72476db7cf49);
pub const GUID_BUS_TYPE_INTERNAL: windows_core::GUID = windows_core::GUID::from_u128(0x1530ea73_086b_11d1_a09f_00c04fc340b1);
pub const GUID_BUS_TYPE_IRDA: windows_core::GUID = windows_core::GUID::from_u128(0x7ae17dc1_c944_44d6_881f_4c2e61053bc1);
pub const GUID_BUS_TYPE_ISAPNP: windows_core::GUID = windows_core::GUID::from_u128(0xe676f854_d87d_11d0_92b2_00a0c9055fc5);
pub const GUID_BUS_TYPE_LPTENUM: windows_core::GUID = windows_core::GUID::from_u128(0xc4ca1000_2ddc_11d5_a17a_00c04f60524d);
pub const GUID_BUS_TYPE_MCA: windows_core::GUID = windows_core::GUID::from_u128(0x1c75997a_dc33_11d0_92b2_00a0c9055fc5);
pub const GUID_BUS_TYPE_PCI: windows_core::GUID = windows_core::GUID::from_u128(0xc8ebdfb0_b510_11d0_80e5_00a0c92542e3);
pub const GUID_BUS_TYPE_PCMCIA: windows_core::GUID = windows_core::GUID::from_u128(0x09343630_af9f_11d0_92e9_0000f81e1b30);
pub const GUID_BUS_TYPE_SCM: windows_core::GUID = windows_core::GUID::from_u128(0x375a5912_804c_45aa_bdc2_fdd25a1d9512);
pub const GUID_BUS_TYPE_SD: windows_core::GUID = windows_core::GUID::from_u128(0xe700cc04_4036_4e89_9579_89ebf45f00cd);
pub const GUID_BUS_TYPE_SERENUM: windows_core::GUID = windows_core::GUID::from_u128(0x77114a87_8944_11d1_bd90_00a0c906be2d);
pub const GUID_BUS_TYPE_SW_DEVICE: windows_core::GUID = windows_core::GUID::from_u128(0x06d10322_7de0_4cef_8e25_197d0e7442e2);
pub const GUID_BUS_TYPE_USB: windows_core::GUID = windows_core::GUID::from_u128(0x9d7debbc_c85d_11d1_9eb4_006008c3a19a);
pub const GUID_BUS_TYPE_USBPRINT: windows_core::GUID = windows_core::GUID::from_u128(0x441ee000_4342_11d5_a184_00c04f60524d);
pub const GUID_D3COLD_AUX_POWER_AND_TIMING_INTERFACE: windows_core::GUID = windows_core::GUID::from_u128(0x0044d8aa_f664_4588_9ffc_2afeaf5950b9);
pub const GUID_D3COLD_SUPPORT_INTERFACE: windows_core::GUID = windows_core::GUID::from_u128(0xb38290e5_3cd0_4f9d_9937_f5fe2b44d47a);
pub const GUID_DEVCLASS_1394: windows_core::GUID = windows_core::GUID::from_u128(0x6bdd1fc1_810f_11d0_bec7_08002be2092f);
pub const GUID_DEVCLASS_1394DEBUG: windows_core::GUID = windows_core::GUID::from_u128(0x66f250d6_7801_4a64_b139_eea80a450b24);
pub const GUID_DEVCLASS_61883: windows_core::GUID = windows_core::GUID::from_u128(0x7ebefbc0_3200_11d2_b4c2_00a0c9697d07);
pub const GUID_DEVCLASS_ADAPTER: windows_core::GUID = windows_core::GUID::from_u128(0x4d36e964_e325_11ce_bfc1_08002be10318);
pub const GUID_DEVCLASS_APMSUPPORT: windows_core::GUID = windows_core::GUID::from_u128(0xd45b1c18_c8fa_11d1_9f77_0000f805f530);
pub const GUID_DEVCLASS_AVC: windows_core::GUID = windows_core::GUID::from_u128(0xc06ff265_ae09_48f0_812c_16753d7cba83);
pub const GUID_DEVCLASS_BATTERY: windows_core::GUID = windows_core::GUID::from_u128(0x72631e54_78a4_11d0_bcf7_00aa00b7b32a);
pub const GUID_DEVCLASS_BIOMETRIC: windows_core::GUID = windows_core::GUID::from_u128(0x53d29ef7_377c_4d14_864b_eb3a85769359);
pub const GUID_DEVCLASS_BLUETOOTH: windows_core::GUID = windows_core::GUID::from_u128(0xe0cbf06c_cd8b_4647_bb8a_263b43f0f974);
pub const GUID_DEVCLASS_CAMERA: windows_core::GUID = windows_core::GUID::from_u128(0xca3e7ab9_b4c3_4ae6_8251_579ef933890f);
pub const GUID_DEVCLASS_CDROM: windows_core::GUID = windows_core::GUID::from_u128(0x4d36e965_e325_11ce_bfc1_08002be10318);
pub const GUID_DEVCLASS_COMPUTEACCELERATOR: windows_core::GUID = windows_core::GUID::from_u128(0xf01a9d53_3ff6_48d2_9f97_c8a7004be10c);
pub const GUID_DEVCLASS_COMPUTER: windows_core::GUID = windows_core::GUID::from_u128(0x4d36e966_e325_11ce_bfc1_08002be10318);
pub const GUID_DEVCLASS_DECODER: windows_core::GUID = windows_core::GUID::from_u128(0x6bdd1fc2_810f_11d0_bec7_08002be2092f);
pub const GUID_DEVCLASS_DISKDRIVE: windows_core::GUID = windows_core::GUID::from_u128(0x4d36e967_e325_11ce_bfc1_08002be10318);
pub const GUID_DEVCLASS_DISPLAY: windows_core::GUID = windows_core::GUID::from_u128(0x4d36e968_e325_11ce_bfc1_08002be10318);
pub const GUID_DEVCLASS_DOT4: windows_core::GUID = windows_core::GUID::from_u128(0x48721b56_6795_11d2_b1a8_0080c72e74a2);
pub const GUID_DEVCLASS_DOT4PRINT: windows_core::GUID = windows_core::GUID::from_u128(0x49ce6ac8_6f86_11d2_b1e5_0080c72e74a2);
pub const GUID_DEVCLASS_EHSTORAGESILO: windows_core::GUID = windows_core::GUID::from_u128(0x9da2b80f_f89f_4a49_a5c2_511b085b9e8a);
pub const GUID_DEVCLASS_ENUM1394: windows_core::GUID = windows_core::GUID::from_u128(0xc459df55_db08_11d1_b009_00a0c9081ff6);
pub const GUID_DEVCLASS_EXTENSION: windows_core::GUID = windows_core::GUID::from_u128(0xe2f84ce7_8efa_411c_aa69_97454ca4cb57);
pub const GUID_DEVCLASS_FDC: windows_core::GUID = windows_core::GUID::from_u128(0x4d36e969_e325_11ce_bfc1_08002be10318);
pub const GUID_DEVCLASS_FIRMWARE: windows_core::GUID = windows_core::GUID::from_u128(0xf2e7dd72_6468_4e36_b6f1_6488f42c1b52);
pub const GUID_DEVCLASS_FLOPPYDISK: windows_core::GUID = windows_core::GUID::from_u128(0x4d36e980_e325_11ce_bfc1_08002be10318);
pub const GUID_DEVCLASS_FSFILTER_ACTIVITYMONITOR: windows_core::GUID = windows_core::GUID::from_u128(0xb86dff51_a31e_4bac_b3cf_e8cfe75c9fc2);
pub const GUID_DEVCLASS_FSFILTER_ANTIVIRUS: windows_core::GUID = windows_core::GUID::from_u128(0xb1d1a169_c54f_4379_81db_bee7d88d7454);
pub const GUID_DEVCLASS_FSFILTER_BOTTOM: windows_core::GUID = windows_core::GUID::from_u128(0x37765ea0_5958_4fc9_b04b_2fdfef97e59e);
pub const GUID_DEVCLASS_FSFILTER_CFSMETADATASERVER: windows_core::GUID = windows_core::GUID::from_u128(0xcdcf0939_b75b_4630_bf76_80f7ba655884);
pub const GUID_DEVCLASS_FSFILTER_COMPRESSION: windows_core::GUID = windows_core::GUID::from_u128(0xf3586baf_b5aa_49b5_8d6c_0569284c639f);
pub const GUID_DEVCLASS_FSFILTER_CONTENTSCREENER: windows_core::GUID = windows_core::GUID::from_u128(0x3e3f0674_c83c_4558_bb26_9820e1eba5c5);
pub const GUID_DEVCLASS_FSFILTER_CONTINUOUSBACKUP: windows_core::GUID = windows_core::GUID::from_u128(0x71aa14f8_6fad_4622_ad77_92bb9d7e6947);
pub const GUID_DEVCLASS_FSFILTER_COPYPROTECTION: windows_core::GUID = windows_core::GUID::from_u128(0x89786ff1_9c12_402f_9c9e_17753c7f4375);
pub const GUID_DEVCLASS_FSFILTER_ENCRYPTION: windows_core::GUID = windows_core::GUID::from_u128(0xa0a701c0_a511_42ff_aa6c_06dc0395576f);
pub const GUID_DEVCLASS_FSFILTER_HSM: windows_core::GUID = windows_core::GUID::from_u128(0xd546500a_2aeb_45f6_9482_f4b1799c3177);
pub const GUID_DEVCLASS_FSFILTER_INFRASTRUCTURE: windows_core::GUID = windows_core::GUID::from_u128(0xe55fa6f9_128c_4d04_abab_630c74b1453a);
pub const GUID_DEVCLASS_FSFILTER_OPENFILEBACKUP: windows_core::GUID = windows_core::GUID::from_u128(0xf8ecafa6_66d1_41a5_899b_66585d7216b7);
pub const GUID_DEVCLASS_FSFILTER_PHYSICALQUOTAMANAGEMENT: windows_core::GUID = windows_core::GUID::from_u128(0x6a0a8e78_bba6_4fc4_a709_1e33cd09d67e);
pub const GUID_DEVCLASS_FSFILTER_QUOTAMANAGEMENT: windows_core::GUID = windows_core::GUID::from_u128(0x8503c911_a6c7_4919_8f79_5028f5866b0c);
pub const GUID_DEVCLASS_FSFILTER_REPLICATION: windows_core::GUID = windows_core::GUID::from_u128(0x48d3ebc4_4cf8_48ff_b869_9c68ad42eb9f);
pub const GUID_DEVCLASS_FSFILTER_SECURITYENHANCER: windows_core::GUID = windows_core::GUID::from_u128(0xd02bc3da_0c8e_4945_9bd5_f1883c226c8c);
pub const GUID_DEVCLASS_FSFILTER_SYSTEM: windows_core::GUID = windows_core::GUID::from_u128(0x5d1b9aaa_01e2_46af_849f_272b3f324c46);
pub const GUID_DEVCLASS_FSFILTER_SYSTEMRECOVERY: windows_core::GUID = windows_core::GUID::from_u128(0x2db15374_706e_4131_a0c7_d7c78eb0289a);
pub const GUID_DEVCLASS_FSFILTER_TOP: windows_core::GUID = windows_core::GUID::from_u128(0xb369baf4_5568_4e82_a87e_a93eb16bca87);
pub const GUID_DEVCLASS_FSFILTER_UNDELETE: windows_core::GUID = windows_core::GUID::from_u128(0xfe8f1572_c67a_48c0_bbac_0b5c6d66cafb);
pub const GUID_DEVCLASS_FSFILTER_VIRTUALIZATION: windows_core::GUID = windows_core::GUID::from_u128(0xf75a86c0_10d8_4c3a_b233_ed60e4cdfaac);
pub const GUID_DEVCLASS_GENERIC: windows_core::GUID = windows_core::GUID::from_u128(0xff494df1_c4ed_4fac_9b3f_3786f6e91e7e);
pub const GUID_DEVCLASS_GPS: windows_core::GUID = windows_core::GUID::from_u128(0x6bdd1fc3_810f_11d0_bec7_08002be2092f);
pub const GUID_DEVCLASS_HDC: windows_core::GUID = windows_core::GUID::from_u128(0x4d36e96a_e325_11ce_bfc1_08002be10318);
pub const GUID_DEVCLASS_HIDCLASS: windows_core::GUID = windows_core::GUID::from_u128(0x745a17a0_74d3_11d0_b6fe_00a0c90f57da);
pub const GUID_DEVCLASS_HOLOGRAPHIC: windows_core::GUID = windows_core::GUID::from_u128(0xd612553d_06b1_49ca_8938_e39ef80eb16f);
pub const GUID_DEVCLASS_IMAGE: windows_core::GUID = windows_core::GUID::from_u128(0x6bdd1fc6_810f_11d0_bec7_08002be2092f);
pub const GUID_DEVCLASS_INFINIBAND: windows_core::GUID = windows_core::GUID::from_u128(0x30ef7132_d858_4a0c_ac24_b9028a5cca3f);
pub const GUID_DEVCLASS_INFRARED: windows_core::GUID = windows_core::GUID::from_u128(0x6bdd1fc5_810f_11d0_bec7_08002be2092f);
pub const GUID_DEVCLASS_KEYBOARD: windows_core::GUID = windows_core::GUID::from_u128(0x4d36e96b_e325_11ce_bfc1_08002be10318);
pub const GUID_DEVCLASS_LEGACYDRIVER: windows_core::GUID = windows_core::GUID::from_u128(0x8ecc055d_047f_11d1_a537_0000f8753ed1);
pub const GUID_DEVCLASS_MEDIA: windows_core::GUID = windows_core::GUID::from_u128(0x4d36e96c_e325_11ce_bfc1_08002be10318);
pub const GUID_DEVCLASS_MEDIUM_CHANGER: windows_core::GUID = windows_core::GUID::from_u128(0xce5939ae_ebde_11d0_b181_0000f8753ec4);
pub const GUID_DEVCLASS_MEMORY: windows_core::GUID = windows_core::GUID::from_u128(0x5099944a_f6b9_4057_a056_8c550228544c);
pub const GUID_DEVCLASS_MODEM: windows_core::GUID = windows_core::GUID::from_u128(0x4d36e96d_e325_11ce_bfc1_08002be10318);
pub const GUID_DEVCLASS_MONITOR: windows_core::GUID = windows_core::GUID::from_u128(0x4d36e96e_e325_11ce_bfc1_08002be10318);
pub const GUID_DEVCLASS_MOUSE: windows_core::GUID = windows_core::GUID::from_u128(0x4d36e96f_e325_11ce_bfc1_08002be10318);
pub const GUID_DEVCLASS_MTD: windows_core::GUID = windows_core::GUID::from_u128(0x4d36e970_e325_11ce_bfc1_08002be10318);
pub const GUID_DEVCLASS_MULTIFUNCTION: windows_core::GUID = windows_core::GUID::from_u128(0x4d36e971_e325_11ce_bfc1_08002be10318);
pub const GUID_DEVCLASS_MULTIPORTSERIAL: windows_core::GUID = windows_core::GUID::from_u128(0x50906cb8_ba12_11d1_bf5d_0000f805f530);
pub const GUID_DEVCLASS_NET: windows_core::GUID = windows_core::GUID::from_u128(0x4d36e972_e325_11ce_bfc1_08002be10318);
pub const GUID_DEVCLASS_NETCLIENT: windows_core::GUID = windows_core::GUID::from_u128(0x4d36e973_e325_11ce_bfc1_08002be10318);
pub const GUID_DEVCLASS_NETDRIVER: windows_core::GUID = windows_core::GUID::from_u128(0x87ef9ad1_8f70_49ee_b215_ab1fcadcbe3c);
pub const GUID_DEVCLASS_NETSERVICE: windows_core::GUID = windows_core::GUID::from_u128(0x4d36e974_e325_11ce_bfc1_08002be10318);
pub const GUID_DEVCLASS_NETTRANS: windows_core::GUID = windows_core::GUID::from_u128(0x4d36e975_e325_11ce_bfc1_08002be10318);
pub const GUID_DEVCLASS_NETUIO: windows_core::GUID = windows_core::GUID::from_u128(0x78912bc1_cb8e_4b28_a329_f322ebadbe0f);
pub const GUID_DEVCLASS_NODRIVER: windows_core::GUID = windows_core::GUID::from_u128(0x4d36e976_e325_11ce_bfc1_08002be10318);
pub const GUID_DEVCLASS_PCMCIA: windows_core::GUID = windows_core::GUID::from_u128(0x4d36e977_e325_11ce_bfc1_08002be10318);
pub const GUID_DEVCLASS_PNPPRINTERS: windows_core::GUID = windows_core::GUID::from_u128(0x4658ee7e_f050_11d1_b6bd_00c04fa372a7);
pub const GUID_DEVCLASS_PORTS: windows_core::GUID = windows_core::GUID::from_u128(0x4d36e978_e325_11ce_bfc1_08002be10318);
pub const GUID_DEVCLASS_PRIMITIVE: windows_core::GUID = windows_core::GUID::from_u128(0x242681d1_eed3_41d2_a1ef_1468fc843106);
pub const GUID_DEVCLASS_PRINTER: windows_core::GUID = windows_core::GUID::from_u128(0x4d36e979_e325_11ce_bfc1_08002be10318);
pub const GUID_DEVCLASS_PRINTERUPGRADE: windows_core::GUID = windows_core::GUID::from_u128(0x4d36e97a_e325_11ce_bfc1_08002be10318);
pub const GUID_DEVCLASS_PRINTQUEUE: windows_core::GUID = windows_core::GUID::from_u128(0x1ed2bbf9_11f0_4084_b21f_ad83a8e6dcdc);
pub const GUID_DEVCLASS_PROCESSOR: windows_core::GUID = windows_core::GUID::from_u128(0x50127dc3_0f36_415e_a6cc_4cb3be910b65);
pub const GUID_DEVCLASS_SBP2: windows_core::GUID = windows_core::GUID::from_u128(0xd48179be_ec20_11d1_b6b8_00c04fa372a7);
pub const GUID_DEVCLASS_SCMDISK: windows_core::GUID = windows_core::GUID::from_u128(0x53966cb1_4d46_4166_bf23_c522403cd495);
pub const GUID_DEVCLASS_SCMVOLUME: windows_core::GUID = windows_core::GUID::from_u128(0x53ccb149_e543_4c84_b6e0_bce4f6b7e806);
pub const GUID_DEVCLASS_SCSIADAPTER: windows_core::GUID = windows_core::GUID::from_u128(0x4d36e97b_e325_11ce_bfc1_08002be10318);
pub const GUID_DEVCLASS_SECURITYACCELERATOR: windows_core::GUID = windows_core::GUID::from_u128(0x268c95a1_edfe_11d3_95c3_0010dc4050a5);
pub const GUID_DEVCLASS_SENSOR: windows_core::GUID = windows_core::GUID::from_u128(0x5175d334_c371_4806_b3ba_71fd53c9258d);
pub const GUID_DEVCLASS_SIDESHOW: windows_core::GUID = windows_core::GUID::from_u128(0x997b5d8d_c442_4f2e_baf3_9c8e671e9e21);
pub const GUID_DEVCLASS_SMARTCARDREADER: windows_core::GUID = windows_core::GUID::from_u128(0x50dd5230_ba8a_11d1_bf5d_0000f805f530);
pub const GUID_DEVCLASS_SMRDISK: windows_core::GUID = windows_core::GUID::from_u128(0x53487c23_680f_4585_acc3_1f10d6777e82);
pub const GUID_DEVCLASS_SMRVOLUME: windows_core::GUID = windows_core::GUID::from_u128(0x53b3cf03_8f5a_4788_91b6_d19ed9fcccbf);
pub const GUID_DEVCLASS_SOFTWARECOMPONENT: windows_core::GUID = windows_core::GUID::from_u128(0x5c4c3332_344d_483c_8739_259e934c9cc8);
pub const GUID_DEVCLASS_SOUND: windows_core::GUID = windows_core::GUID::from_u128(0x4d36e97c_e325_11ce_bfc1_08002be10318);
pub const GUID_DEVCLASS_SYSTEM: windows_core::GUID = windows_core::GUID::from_u128(0x4d36e97d_e325_11ce_bfc1_08002be10318);
pub const GUID_DEVCLASS_TAPEDRIVE: windows_core::GUID = windows_core::GUID::from_u128(0x6d807884_7d21_11cf_801c_08002be10318);
pub const GUID_DEVCLASS_UCM: windows_core::GUID = windows_core::GUID::from_u128(0xe6f1aa1c_7f3b_4473_b2e8_c97d8ac71d53);
pub const GUID_DEVCLASS_UNKNOWN: windows_core::GUID = windows_core::GUID::from_u128(0x4d36e97e_e325_11ce_bfc1_08002be10318);
pub const GUID_DEVCLASS_USB: windows_core::GUID = windows_core::GUID::from_u128(0x36fc9e60_c465_11cf_8056_444553540000);
pub const GUID_DEVCLASS_VOLUME: windows_core::GUID = windows_core::GUID::from_u128(0x71a27cdd_812a_11d0_bec7_08002be2092f);
pub const GUID_DEVCLASS_VOLUMESNAPSHOT: windows_core::GUID = windows_core::GUID::from_u128(0x533c5b84_ec70_11d2_9505_00c04f79deaf);
pub const GUID_DEVCLASS_WCEUSBS: windows_core::GUID = windows_core::GUID::from_u128(0x25dbce51_6c8f_4a72_8a6d_b54c2b4fc835);
pub const GUID_DEVCLASS_WPD: windows_core::GUID = windows_core::GUID::from_u128(0xeec5ad98_8080_425f_922a_dabf3de3f69a);
pub const GUID_DEVICE_INTERFACE_ARRIVAL: windows_core::GUID = windows_core::GUID::from_u128(0xcb3a4004_46f0_11d0_b08f_00609713053f);
pub const GUID_DEVICE_INTERFACE_REMOVAL: windows_core::GUID = windows_core::GUID::from_u128(0xcb3a4005_46f0_11d0_b08f_00609713053f);
pub const GUID_DEVICE_RESET_INTERFACE_STANDARD: windows_core::GUID = windows_core::GUID::from_u128(0x649fdf26_3bc0_4813_ad24_7e0c1eda3fa3);
pub const GUID_DMA_CACHE_COHERENCY_INTERFACE: windows_core::GUID = windows_core::GUID::from_u128(0xb520f7fa_8a5a_4e40_a3f6_6be1e162d935);
pub const GUID_HWPROFILE_CHANGE_CANCELLED: windows_core::GUID = windows_core::GUID::from_u128(0xcb3a4002_46f0_11d0_b08f_00609713053f);
pub const GUID_HWPROFILE_CHANGE_COMPLETE: windows_core::GUID = windows_core::GUID::from_u128(0xcb3a4003_46f0_11d0_b08f_00609713053f);
pub const GUID_HWPROFILE_QUERY_CHANGE: windows_core::GUID = windows_core::GUID::from_u128(0xcb3a4001_46f0_11d0_b08f_00609713053f);
pub const GUID_INT_ROUTE_INTERFACE_STANDARD: windows_core::GUID = windows_core::GUID::from_u128(0x70941bf4_0073_11d1_a09e_00c04fc340b1);
pub const GUID_IOMMU_BUS_INTERFACE: windows_core::GUID = windows_core::GUID::from_u128(0x1efee0b2_d278_4ae4_bddc_1b34dd648043);
pub const GUID_KERNEL_SOFT_RESTART_CANCEL: windows_core::GUID = windows_core::GUID::from_u128(0x31d737e7_8c0b_468a_956e_9f433ec358fb);
pub const GUID_KERNEL_SOFT_RESTART_FINALIZE: windows_core::GUID = windows_core::GUID::from_u128(0x20e91abd_350a_4d4f_8577_99c81507473a);
pub const GUID_KERNEL_SOFT_RESTART_PREPARE: windows_core::GUID = windows_core::GUID::from_u128(0xde373def_a85c_4f76_8cbf_f96bea8bd10f);
pub const GUID_LEGACY_DEVICE_DETECTION_STANDARD: windows_core::GUID = windows_core::GUID::from_u128(0x50feb0de_596a_11d2_a5b8_0000f81a4619);
pub const GUID_MF_ENUMERATION_INTERFACE: windows_core::GUID = windows_core::GUID::from_u128(0xaeb895f0_5586_11d1_8d84_00a0c906b244);
pub const GUID_MSIX_TABLE_CONFIG_INTERFACE: windows_core::GUID = windows_core::GUID::from_u128(0x1a6a460b_194f_455d_b34b_b84c5b05712b);
pub const GUID_NPEM_CONTROL_INTERFACE: windows_core::GUID = windows_core::GUID::from_u128(0x4d95573d_b774_488a_b120_4f284a9eff51);
pub const GUID_PARTITION_UNIT_INTERFACE_STANDARD: windows_core::GUID = windows_core::GUID::from_u128(0x52363f5b_d891_429b_8195_aec5fef6853c);
pub const GUID_PCC_INTERFACE_INTERNAL: windows_core::GUID = windows_core::GUID::from_u128(0x7cce62ce_c189_4814_a6a7_12112089e938);
pub const GUID_PCC_INTERFACE_STANDARD: windows_core::GUID = windows_core::GUID::from_u128(0x3ee8ba63_0f59_4a24_8a45_35808bdd1249);
pub const GUID_PCI_ATS_INTERFACE: windows_core::GUID = windows_core::GUID::from_u128(0x010a7fe8_96f5_4943_bedf_95e651b93412);
pub const GUID_PCI_BUS_INTERFACE_STANDARD: windows_core::GUID = windows_core::GUID::from_u128(0x496b8281_6f25_11d0_beaf_08002be2092f);
pub const GUID_PCI_BUS_INTERFACE_STANDARD2: windows_core::GUID = windows_core::GUID::from_u128(0xde94e966_fdff_4c9c_9998_6747b150e74c);
pub const GUID_PCI_DEVICE_PRESENT_INTERFACE: windows_core::GUID = windows_core::GUID::from_u128(0xd1b82c26_bf49_45ef_b216_71cbd7889b57);
pub const GUID_PCI_EXPRESS_LINK_QUIESCENT_INTERFACE: windows_core::GUID = windows_core::GUID::from_u128(0x146cd41c_dae3_4437_8aff_2af3f038099b);
pub const GUID_PCI_EXPRESS_ROOT_PORT_INTERFACE: windows_core::GUID = windows_core::GUID::from_u128(0x83a7734a_84c7_4161_9a98_6000ed0c4a33);
pub const GUID_PCI_FPGA_CONTROL_INTERFACE: windows_core::GUID = windows_core::GUID::from_u128(0x2df3f7a8_b9b3_4063_9215_b5d14a0b266e);
pub const GUID_PCI_PTM_CONTROL_INTERFACE: windows_core::GUID = windows_core::GUID::from_u128(0x348a5ebb_ba24_44b7_9916_285687735117);
pub const GUID_PCI_SECURITY_INTERFACE: windows_core::GUID = windows_core::GUID::from_u128(0x6e7f1451_199e_4acc_ba2d_762b4edf4674);
pub const GUID_PCI_VIRTUALIZATION_INTERFACE: windows_core::GUID = windows_core::GUID::from_u128(0x64897b47_3a4a_4d75_bc74_89dd6c078293);
pub const GUID_PCMCIA_BUS_INTERFACE_STANDARD: windows_core::GUID = windows_core::GUID::from_u128(0x76173af0_c504_11d1_947f_00c04fb960ee);
pub const GUID_PNP_CUSTOM_NOTIFICATION: windows_core::GUID = windows_core::GUID::from_u128(0xaca73f8e_8d23_11d1_ac7d_0000f87571d0);
pub const GUID_PNP_EXTENDED_ADDRESS_INTERFACE: windows_core::GUID = windows_core::GUID::from_u128(0xb8e992ec_a797_4dc4_8846_84d041707446);
pub const GUID_PNP_LOCATION_INTERFACE: windows_core::GUID = windows_core::GUID::from_u128(0x70211b0e_0afb_47db_afc1_410bf842497a);
pub const GUID_PNP_POWER_NOTIFICATION: windows_core::GUID = windows_core::GUID::from_u128(0xc2cf0660_eb7a_11d1_bd7f_0000f87571d0);
pub const GUID_PNP_POWER_SETTING_CHANGE: windows_core::GUID = windows_core::GUID::from_u128(0x29c69b3e_c79a_43bf_bbde_a932fa1bea7e);
pub const GUID_POWER_DEVICE_ENABLE: windows_core::GUID = windows_core::GUID::from_u128(0x827c0a6f_feb0_11d0_bd26_00aa00b7b32a);
pub const GUID_POWER_DEVICE_TIMEOUTS: windows_core::GUID = windows_core::GUID::from_u128(0xa45da735_feb0_11d0_bd26_00aa00b7b32a);
pub const GUID_POWER_DEVICE_WAKE_ENABLE: windows_core::GUID = windows_core::GUID::from_u128(0xa9546a82_feb0_11d0_bd26_00aa00b7b32a);
pub const GUID_PROCESSOR_PCC_INTERFACE_STANDARD: windows_core::GUID = windows_core::GUID::from_u128(0x37b17e9a_c21c_4296_972d_11c4b32b28f0);
pub const GUID_QUERY_CRASHDUMP_FUNCTIONS: windows_core::GUID = windows_core::GUID::from_u128(0x9cc6b8ff_32e2_4834_b1de_b32ef8880a4b);
pub const GUID_RECOVERY_NVMED_PREPARE_SHUTDOWN: windows_core::GUID = windows_core::GUID::from_u128(0x4b9770ea_bde7_400b_a9b9_4f684f54cc2a);
pub const GUID_RECOVERY_PCI_PREPARE_SHUTDOWN: windows_core::GUID = windows_core::GUID::from_u128(0x90d889de_8704_44cf_8115_ed8528d2b2da);
pub const GUID_REENUMERATE_SELF_INTERFACE_STANDARD: windows_core::GUID = windows_core::GUID::from_u128(0x2aeb0243_6a6e_486b_82fc_d815f6b97006);
pub const GUID_SCM_BUS_INTERFACE: windows_core::GUID = windows_core::GUID::from_u128(0x25944783_ce79_4232_815e_4a30014e8eb4);
pub const GUID_SCM_BUS_LD_INTERFACE: windows_core::GUID = windows_core::GUID::from_u128(0x9b89307d_d76b_4f48_b186_54041ae92e8d);
pub const GUID_SCM_BUS_NVD_INTERFACE: windows_core::GUID = windows_core::GUID::from_u128(0x8de064ff_b630_42e4_88ea_6f24c8641175);
pub const GUID_SCM_PHYSICAL_NVDIMM_INTERFACE: windows_core::GUID = windows_core::GUID::from_u128(0x0079c21b_917e_405e_a9ce_0732b5bbcebd);
pub const GUID_SDEV_IDENTIFIER_INTERFACE: windows_core::GUID = windows_core::GUID::from_u128(0x49d67af8_916c_4ee8_9df1_889f17d21e91);
pub const GUID_SECURE_DRIVER_INTERFACE: windows_core::GUID = windows_core::GUID::from_u128(0x370f67e1_4ff5_4a94_9a35_06c5d9cc30e2);
pub const GUID_TARGET_DEVICE_QUERY_REMOVE: windows_core::GUID = windows_core::GUID::from_u128(0xcb3a4006_46f0_11d0_b08f_00609713053f);
pub const GUID_TARGET_DEVICE_REMOVE_CANCELLED: windows_core::GUID = windows_core::GUID::from_u128(0xcb3a4007_46f0_11d0_b08f_00609713053f);
pub const GUID_TARGET_DEVICE_REMOVE_COMPLETE: windows_core::GUID = windows_core::GUID::from_u128(0xcb3a4008_46f0_11d0_b08f_00609713053f);
pub const GUID_TARGET_DEVICE_TRANSPORT_RELATIONS_CHANGED: windows_core::GUID = windows_core::GUID::from_u128(0xfcf528f6_a82f_47b1_ad3a_8050594cad28);
pub const GUID_THERMAL_COOLING_INTERFACE: windows_core::GUID = windows_core::GUID::from_u128(0xecbe47a8_c498_4bb9_bd70_e867e0940d22);
pub const GUID_TRANSLATOR_INTERFACE_STANDARD: windows_core::GUID = windows_core::GUID::from_u128(0x6c154a92_aacf_11d0_8d2a_00a0c906b244);
pub const GUID_WUDF_DEVICE_HOST_PROBLEM: windows_core::GUID = windows_core::GUID::from_u128(0xc43d25bd_9346_40ee_a2d2_d70c15f8b75b);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HCMNOTIFICATION(pub *mut core::ffi::c_void);
impl HCMNOTIFICATION {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 as _ || self.0 == 0 as _
    }
}
impl windows_core::Free for HCMNOTIFICATION {
    #[inline]
    unsafe fn free(&mut self) {
        if !self.is_invalid() {
            windows_core::link!("cfgmgr32.dll" "system" fn CM_Unregister_Notification(notifycontext : *mut core::ffi::c_void) -> u32);
            unsafe {
                CM_Unregister_Notification(self.0);
            }
        }
    }
}
impl Default for HCMNOTIFICATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct HDEVINFO(pub isize);
impl HDEVINFO {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 || self.0 == 0
    }
}
impl windows_core::Free for HDEVINFO {
    #[inline]
    unsafe fn free(&mut self) {
        if !self.is_invalid() {
            windows_core::link!("setupapi.dll" "system" fn SetupDiDestroyDeviceInfoList(deviceinfoset : isize) -> i32);
            unsafe {
                SetupDiDestroyDeviceInfoList(self.0);
            }
        }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct HWPROFILEINFO_A {
    pub HWPI_ulHWProfile: u32,
    pub HWPI_szFriendlyName: [i8; 80],
    pub HWPI_dwFlags: u32,
}
impl Default for HWPROFILEINFO_A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct HWPROFILEINFO_W {
    pub HWPI_ulHWProfile: u32,
    pub HWPI_szFriendlyName: [u16; 80],
    pub HWPI_dwFlags: u32,
}
impl Default for HWPROFILEINFO_W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const IDD_DYNAWIZ_ANALYZEDEV_PAGE: u32 = 10010u32;
pub const IDD_DYNAWIZ_ANALYZE_NEXTPAGE: u32 = 10004u32;
pub const IDD_DYNAWIZ_ANALYZE_PREVPAGE: u32 = 10003u32;
pub const IDD_DYNAWIZ_FIRSTPAGE: u32 = 10000u32;
pub const IDD_DYNAWIZ_INSTALLDETECTEDDEVS_PAGE: u32 = 10011u32;
pub const IDD_DYNAWIZ_INSTALLDETECTED_NEXTPAGE: u32 = 10007u32;
pub const IDD_DYNAWIZ_INSTALLDETECTED_NODEVS: u32 = 10008u32;
pub const IDD_DYNAWIZ_INSTALLDETECTED_PREVPAGE: u32 = 10006u32;
pub const IDD_DYNAWIZ_SELECTCLASS_PAGE: u32 = 10012u32;
pub const IDD_DYNAWIZ_SELECTDEV_PAGE: u32 = 10009u32;
pub const IDD_DYNAWIZ_SELECT_NEXTPAGE: u32 = 10002u32;
pub const IDD_DYNAWIZ_SELECT_PREVPAGE: u32 = 10001u32;
pub const IDF_CHECKFIRST: u32 = 256u32;
pub const IDF_NOBEEP: u32 = 512u32;
pub const IDF_NOBROWSE: u32 = 1u32;
pub const IDF_NOCOMPRESSED: u32 = 8u32;
pub const IDF_NODETAILS: u32 = 4u32;
pub const IDF_NOFOREGROUND: u32 = 1024u32;
pub const IDF_NOREMOVABLEMEDIAPROMPT: u32 = 4096u32;
pub const IDF_NOSKIP: u32 = 2u32;
pub const IDF_OEMDISK: u32 = 2147483648u32;
pub const IDF_USEDISKNAMEASPROMPT: u32 = 8192u32;
pub const IDF_WARNIFSKIP: u32 = 2048u32;
pub const IDI_CLASSICON_OVERLAYFIRST: u32 = 500u32;
pub const IDI_CLASSICON_OVERLAYLAST: u32 = 502u32;
pub const IDI_CONFLICT: u32 = 161u32;
pub const IDI_DISABLED_OVL: u32 = 501u32;
pub const IDI_FORCED_OVL: u32 = 502u32;
pub const IDI_PROBLEM_OVL: u32 = 500u32;
pub const IDI_RESOURCE: u32 = 159u32;
pub const IDI_RESOURCEFIRST: u32 = 159u32;
pub const IDI_RESOURCELAST: u32 = 161u32;
pub const IDI_RESOURCEOVERLAYFIRST: u32 = 161u32;
pub const IDI_RESOURCEOVERLAYLAST: u32 = 161u32;
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy)]
pub struct INFCONTEXT {
    pub Inf: *mut core::ffi::c_void,
    pub CurrentInf: *mut core::ffi::c_void,
    pub Section: u32,
    pub Line: u32,
}
#[cfg(target_arch = "x86")]
impl Default for INFCONTEXT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct INFCONTEXT {
    pub Inf: *mut core::ffi::c_void,
    pub CurrentInf: *mut core::ffi::c_void,
    pub Section: u32,
    pub Line: u32,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for INFCONTEXT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const INFINFO_DEFAULT_SEARCH: u32 = 3u32;
pub const INFINFO_INF_NAME_IS_ABSOLUTE: u32 = 2u32;
pub const INFINFO_INF_PATH_LIST_SEARCH: u32 = 5u32;
pub const INFINFO_INF_SPEC_IS_HINF: u32 = 1u32;
pub const INFINFO_REVERSE_DEFAULT_SEARCH: u32 = 4u32;
pub const INFSTR_BUS_ALL: windows_core::PCWSTR = windows_core::w!("BUS_ALL");
pub const INFSTR_BUS_EISA: windows_core::PCWSTR = windows_core::w!("BUS_EISA");
pub const INFSTR_BUS_ISA: windows_core::PCWSTR = windows_core::w!("BUS_ISA");
pub const INFSTR_BUS_MCA: windows_core::PCWSTR = windows_core::w!("BUS_MCA");
pub const INFSTR_CFGPRI_DESIRED: windows_core::PCWSTR = windows_core::w!("DESIRED");
pub const INFSTR_CFGPRI_DISABLED: windows_core::PCWSTR = windows_core::w!("DISABLED");
pub const INFSTR_CFGPRI_FORCECONFIG: windows_core::PCWSTR = windows_core::w!("FORCECONFIG");
pub const INFSTR_CFGPRI_HARDRECONFIG: windows_core::PCWSTR = windows_core::w!("HARDRECONFIG");
pub const INFSTR_CFGPRI_HARDWIRED: windows_core::PCWSTR = windows_core::w!("HARDWIRED");
pub const INFSTR_CFGPRI_NORMAL: windows_core::PCWSTR = windows_core::w!("NORMAL");
pub const INFSTR_CFGPRI_POWEROFF: windows_core::PCWSTR = windows_core::w!("POWEROFF");
pub const INFSTR_CFGPRI_REBOOT: windows_core::PCWSTR = windows_core::w!("REBOOT");
pub const INFSTR_CFGPRI_RESTART: windows_core::PCWSTR = windows_core::w!("RESTART");
pub const INFSTR_CFGPRI_SUBOPTIMAL: windows_core::PCWSTR = windows_core::w!("SUBOPTIMAL");
pub const INFSTR_CFGTYPE_BASIC: windows_core::PCWSTR = windows_core::w!("BASIC");
pub const INFSTR_CFGTYPE_FORCED: windows_core::PCWSTR = windows_core::w!("FORCED");
pub const INFSTR_CFGTYPE_OVERRIDE: windows_core::PCWSTR = windows_core::w!("OVERRIDE");
pub const INFSTR_CLASS_SAFEEXCL: windows_core::PCWSTR = windows_core::w!("SAFE_EXCL");
pub const INFSTR_CONTROLFLAGS_SECTION: windows_core::PCWSTR = windows_core::w!("ControlFlags");
pub const INFSTR_DRIVERSELECT_FUNCTIONS: windows_core::PCWSTR = windows_core::w!("DriverSelectFunctions");
pub const INFSTR_DRIVERSELECT_SECTION: windows_core::PCWSTR = windows_core::w!("DriverSelect");
pub const INFSTR_DRIVERVERSION_SECTION: windows_core::PCWSTR = windows_core::w!("DriverVer");
pub const INFSTR_KEY_ACTION: windows_core::PCWSTR = windows_core::w!("Action");
pub const INFSTR_KEY_ALWAYSEXCLUDEFROMSELECT: windows_core::PCWSTR = windows_core::w!("AlwaysExcludeFromSelect");
pub const INFSTR_KEY_BUFFER_SIZE: windows_core::PCWSTR = windows_core::w!("BufferSize");
pub const INFSTR_KEY_CATALOGFILE: windows_core::PCWSTR = windows_core::w!("CatalogFile");
pub const INFSTR_KEY_CHANNEL_ACCESS: windows_core::PCWSTR = windows_core::w!("Access");
pub const INFSTR_KEY_CHANNEL_ENABLED: windows_core::PCWSTR = windows_core::w!("Enabled");
pub const INFSTR_KEY_CHANNEL_ISOLATION: windows_core::PCWSTR = windows_core::w!("Isolation");
pub const INFSTR_KEY_CHANNEL_VALUE: windows_core::PCWSTR = windows_core::w!("Value");
pub const INFSTR_KEY_CLASS: windows_core::PCWSTR = windows_core::w!("Class");
pub const INFSTR_KEY_CLASSGUID: windows_core::PCWSTR = windows_core::w!("ClassGUID");
pub const INFSTR_KEY_CLOCK_TYPE: windows_core::PCWSTR = windows_core::w!("ClockType");
pub const INFSTR_KEY_CONFIGPRIORITY: windows_core::PCWSTR = windows_core::w!("ConfigPriority");
pub const INFSTR_KEY_COPYFILESONLY: windows_core::PCWSTR = windows_core::w!("CopyFilesOnly");
pub const INFSTR_KEY_DATA_ITEM: windows_core::PCWSTR = windows_core::w!("DataItem");
pub const INFSTR_KEY_DELAYEDAUTOSTART: windows_core::PCWSTR = windows_core::w!("DelayedAutoStart");
pub const INFSTR_KEY_DEPENDENCIES: windows_core::PCWSTR = windows_core::w!("Dependencies");
pub const INFSTR_KEY_DESCRIPTION: windows_core::PCWSTR = windows_core::w!("Description");
pub const INFSTR_KEY_DETECTLIST: windows_core::PCWSTR = windows_core::w!("DetectList");
pub const INFSTR_KEY_DETPARAMS: windows_core::PCWSTR = windows_core::w!("Params");
pub const INFSTR_KEY_DISABLE_REALTIME_PERSISTENCE: windows_core::PCWSTR = windows_core::w!("DisableRealtimePersistence");
pub const INFSTR_KEY_DISPLAYNAME: windows_core::PCWSTR = windows_core::w!("DisplayName");
pub const INFSTR_KEY_DMA: windows_core::PCWSTR = windows_core::w!("DMA");
pub const INFSTR_KEY_DMACONFIG: windows_core::PCWSTR = windows_core::w!("DMAConfig");
pub const INFSTR_KEY_DRIVERSET: windows_core::PCWSTR = windows_core::w!("DriverSet");
pub const INFSTR_KEY_ENABLED: windows_core::PCWSTR = windows_core::w!("Enabled");
pub const INFSTR_KEY_ENABLE_FLAGS: windows_core::PCWSTR = windows_core::w!("EnableFlags");
pub const INFSTR_KEY_ENABLE_LEVEL: windows_core::PCWSTR = windows_core::w!("EnableLevel");
pub const INFSTR_KEY_ENABLE_PROPERTY: windows_core::PCWSTR = windows_core::w!("EnableProperty");
pub const INFSTR_KEY_ERRORCONTROL: windows_core::PCWSTR = windows_core::w!("ErrorControl");
pub const INFSTR_KEY_EXCLUDEFROMSELECT: windows_core::PCWSTR = windows_core::w!("ExcludeFromSelect");
pub const INFSTR_KEY_EXCLUDERES: windows_core::PCWSTR = windows_core::w!("ExcludeRes");
pub const INFSTR_KEY_EXTENSIONID: windows_core::PCWSTR = windows_core::w!("ExtensionId");
pub const INFSTR_KEY_FAILURE_ACTION: windows_core::PCWSTR = windows_core::w!("Action");
pub const INFSTR_KEY_FILE_MAX: windows_core::PCWSTR = windows_core::w!("FileMax");
pub const INFSTR_KEY_FILE_NAME: windows_core::PCWSTR = windows_core::w!("FileName");
pub const INFSTR_KEY_FLUSH_TIMER: windows_core::PCWSTR = windows_core::w!("FlushTimer");
pub const INFSTR_KEY_FROMINET: windows_core::PCWSTR = windows_core::w!("FromINet");
pub const INFSTR_KEY_HARDWARE_CLASS: windows_core::PCWSTR = windows_core::w!("Class");
pub const INFSTR_KEY_HARDWARE_CLASSGUID: windows_core::PCWSTR = windows_core::w!("ClassGUID");
pub const INFSTR_KEY_INTERACTIVEINSTALL: windows_core::PCWSTR = windows_core::w!("InteractiveInstall");
pub const INFSTR_KEY_IO: windows_core::PCWSTR = windows_core::w!("IO");
pub const INFSTR_KEY_IOCONFIG: windows_core::PCWSTR = windows_core::w!("IOConfig");
pub const INFSTR_KEY_IRQ: windows_core::PCWSTR = windows_core::w!("IRQ");
pub const INFSTR_KEY_IRQCONFIG: windows_core::PCWSTR = windows_core::w!("IRQConfig");
pub const INFSTR_KEY_LOADORDERGROUP: windows_core::PCWSTR = windows_core::w!("LoadOrderGroup");
pub const INFSTR_KEY_LOGGING_AUTOBACKUP: windows_core::PCWSTR = windows_core::w!("LoggingAutoBackup");
pub const INFSTR_KEY_LOGGING_MAXSIZE: windows_core::PCWSTR = windows_core::w!("LoggingMaxSize");
pub const INFSTR_KEY_LOGGING_RETENTION: windows_core::PCWSTR = windows_core::w!("LoggingRetention");
pub const INFSTR_KEY_LOG_FILE_MODE: windows_core::PCWSTR = windows_core::w!("LogFileMode");
pub const INFSTR_KEY_MATCH_ALL_KEYWORD: windows_core::PCWSTR = windows_core::w!("MatchAllKeyword");
pub const INFSTR_KEY_MATCH_ANY_KEYWORD: windows_core::PCWSTR = windows_core::w!("MatchAnyKeyword");
pub const INFSTR_KEY_MAXIMUM_BUFFERS: windows_core::PCWSTR = windows_core::w!("MaximumBuffers");
pub const INFSTR_KEY_MAX_FILE_SIZE: windows_core::PCWSTR = windows_core::w!("MaxFileSize");
pub const INFSTR_KEY_MEM: windows_core::PCWSTR = windows_core::w!("Mem");
pub const INFSTR_KEY_MEMCONFIG: windows_core::PCWSTR = windows_core::w!("MemConfig");
pub const INFSTR_KEY_MEMLARGECONFIG: windows_core::PCWSTR = windows_core::w!("MemLargeConfig");
pub const INFSTR_KEY_MESSAGE_FILE: windows_core::PCWSTR = windows_core::w!("MessageFile");
pub const INFSTR_KEY_MFCARDCONFIG: windows_core::PCWSTR = windows_core::w!("MfCardConfig");
pub const INFSTR_KEY_MINIMUM_BUFFERS: windows_core::PCWSTR = windows_core::w!("MinimumBuffers");
pub const INFSTR_KEY_NAME: windows_core::PCWSTR = windows_core::w!("Name");
pub const INFSTR_KEY_NON_CRASH_FAILURES: windows_core::PCWSTR = windows_core::w!("NonCrashFailures");
pub const INFSTR_KEY_NOSETUPINF: windows_core::PCWSTR = windows_core::w!("NoSetupInf");
pub const INFSTR_KEY_PARAMETER_FILE: windows_core::PCWSTR = windows_core::w!("ParameterFile");
pub const INFSTR_KEY_PATH: windows_core::PCWSTR = windows_core::w!("Path");
pub const INFSTR_KEY_PCCARDCONFIG: windows_core::PCWSTR = windows_core::w!("PcCardConfig");
pub const INFSTR_KEY_PNPLOCKDOWN: windows_core::PCWSTR = windows_core::w!("PnpLockDown");
pub const INFSTR_KEY_PROVIDER: windows_core::PCWSTR = windows_core::w!("Provider");
pub const INFSTR_KEY_PROVIDER_NAME: windows_core::PCWSTR = windows_core::w!("ProviderName");
pub const INFSTR_KEY_REQUESTADDITIONALSOFTWARE: windows_core::PCWSTR = windows_core::w!("RequestAdditionalSoftware");
pub const INFSTR_KEY_REQUIREDPRIVILEGES: windows_core::PCWSTR = windows_core::w!("RequiredPrivileges");
pub const INFSTR_KEY_RESET_PERIOD: windows_core::PCWSTR = windows_core::w!("ResetPeriod");
pub const INFSTR_KEY_RESOURCE_FILE: windows_core::PCWSTR = windows_core::w!("ResourceFile");
pub const INFSTR_KEY_SECURITY: windows_core::PCWSTR = windows_core::w!("Security");
pub const INFSTR_KEY_SERVICEBINARY: windows_core::PCWSTR = windows_core::w!("ServiceBinary");
pub const INFSTR_KEY_SERVICESIDTYPE: windows_core::PCWSTR = windows_core::w!("ServiceSidType");
pub const INFSTR_KEY_SERVICETYPE: windows_core::PCWSTR = windows_core::w!("ServiceType");
pub const INFSTR_KEY_SIGNATURE: windows_core::PCWSTR = windows_core::w!("Signature");
pub const INFSTR_KEY_SKIPLIST: windows_core::PCWSTR = windows_core::w!("SkipList");
pub const INFSTR_KEY_START: windows_core::PCWSTR = windows_core::w!("Start");
pub const INFSTR_KEY_STARTNAME: windows_core::PCWSTR = windows_core::w!("StartName");
pub const INFSTR_KEY_STARTTYPE: windows_core::PCWSTR = windows_core::w!("StartType");
pub const INFSTR_KEY_SUB_TYPE: windows_core::PCWSTR = windows_core::w!("SubType");
pub const INFSTR_KEY_TRIGGER_TYPE: windows_core::PCWSTR = windows_core::w!("TriggerType");
pub const INFSTR_PLATFORM_NT: windows_core::PCWSTR = windows_core::w!("NT");
pub const INFSTR_PLATFORM_NTALPHA: windows_core::PCWSTR = windows_core::w!("NTAlpha");
pub const INFSTR_PLATFORM_NTAMD64: windows_core::PCWSTR = windows_core::w!("NTAMD64");
pub const INFSTR_PLATFORM_NTARM: windows_core::PCWSTR = windows_core::w!("NTARM");
pub const INFSTR_PLATFORM_NTARM64: windows_core::PCWSTR = windows_core::w!("NTARM64");
pub const INFSTR_PLATFORM_NTAXP64: windows_core::PCWSTR = windows_core::w!("NTAXP64");
pub const INFSTR_PLATFORM_NTIA64: windows_core::PCWSTR = windows_core::w!("NTIA64");
pub const INFSTR_PLATFORM_NTMIPS: windows_core::PCWSTR = windows_core::w!("NTMIPS");
pub const INFSTR_PLATFORM_NTPPC: windows_core::PCWSTR = windows_core::w!("NTPPC");
pub const INFSTR_PLATFORM_NTX86: windows_core::PCWSTR = windows_core::w!("NTx86");
pub const INFSTR_PLATFORM_WIN: windows_core::PCWSTR = windows_core::w!("Win");
pub const INFSTR_REBOOT: windows_core::PCWSTR = windows_core::w!("Reboot");
pub const INFSTR_RESTART: windows_core::PCWSTR = windows_core::w!("Restart");
pub const INFSTR_RISK_BIOSROMRD: windows_core::PCWSTR = windows_core::w!("RISK_BIOSROMRD");
pub const INFSTR_RISK_DELICATE: windows_core::PCWSTR = windows_core::w!("RISK_DELICATE");
pub const INFSTR_RISK_IORD: windows_core::PCWSTR = windows_core::w!("RISK_IORD");
pub const INFSTR_RISK_IOWR: windows_core::PCWSTR = windows_core::w!("RISK_IOWR");
pub const INFSTR_RISK_LOW: windows_core::PCWSTR = windows_core::w!("RISK_LOW");
pub const INFSTR_RISK_MEMRD: windows_core::PCWSTR = windows_core::w!("RISK_MEMRD");
pub const INFSTR_RISK_MEMWR: windows_core::PCWSTR = windows_core::w!("RISK_MEMWR");
pub const INFSTR_RISK_NONE: windows_core::PCWSTR = windows_core::w!("RISK_NONE");
pub const INFSTR_RISK_QUERYDRV: windows_core::PCWSTR = windows_core::w!("RISK_QUERYDRV");
pub const INFSTR_RISK_SWINT: windows_core::PCWSTR = windows_core::w!("RISK_SWINT");
pub const INFSTR_RISK_UNRELIABLE: windows_core::PCWSTR = windows_core::w!("RISK_UNRELIABLE");
pub const INFSTR_RISK_VERYHIGH: windows_core::PCWSTR = windows_core::w!("RISK_VERYHIGH");
pub const INFSTR_RISK_VERYLOW: windows_core::PCWSTR = windows_core::w!("RISK_VERYLOW");
pub const INFSTR_SECT_AUTOEXECBAT: windows_core::PCWSTR = windows_core::w!("AutoexecBatDrivers");
pub const INFSTR_SECT_AVOIDCFGSYSDEV: windows_core::PCWSTR = windows_core::w!("Det.AvoidCfgSysDev");
pub const INFSTR_SECT_AVOIDENVDEV: windows_core::PCWSTR = windows_core::w!("Det.AvoidEnvDev");
pub const INFSTR_SECT_AVOIDINIDEV: windows_core::PCWSTR = windows_core::w!("Det.AvoidIniDev");
pub const INFSTR_SECT_BADACPIBIOS: windows_core::PCWSTR = windows_core::w!("BadACPIBios");
pub const INFSTR_SECT_BADDISKBIOS: windows_core::PCWSTR = windows_core::w!("BadDiskBios");
pub const INFSTR_SECT_BADDSBIOS: windows_core::PCWSTR = windows_core::w!("BadDSBios");
pub const INFSTR_SECT_BADPMCALLBIOS: windows_core::PCWSTR = windows_core::w!("BadProtectedModeCallBios");
pub const INFSTR_SECT_BADPNPBIOS: windows_core::PCWSTR = windows_core::w!("BadPnpBios");
pub const INFSTR_SECT_BADRMCALLBIOS: windows_core::PCWSTR = windows_core::w!("BadRealModeCallBios");
pub const INFSTR_SECT_BADROUTINGTABLEBIOS: windows_core::PCWSTR = windows_core::w!("BadPCIIRQRoutingTableBios");
pub const INFSTR_SECT_CFGSYS: windows_core::PCWSTR = windows_core::w!("ConfigSysDrivers");
pub const INFSTR_SECT_CLASS_INSTALL: windows_core::PCWSTR = windows_core::w!("ClassInstall");
pub const INFSTR_SECT_CLASS_INSTALL_32: windows_core::PCWSTR = windows_core::w!("ClassInstall32");
pub const INFSTR_SECT_DEFAULT_INSTALL: windows_core::PCWSTR = windows_core::w!("DefaultInstall");
pub const INFSTR_SECT_DEFAULT_UNINSTALL: windows_core::PCWSTR = windows_core::w!("DefaultUninstall");
pub const INFSTR_SECT_DETCLASSINFO: windows_core::PCWSTR = windows_core::w!("Det.ClassInfo");
pub const INFSTR_SECT_DETMODULES: windows_core::PCWSTR = windows_core::w!("Det.Modules");
pub const INFSTR_SECT_DETOPTIONS: windows_core::PCWSTR = windows_core::w!("Det.Options");
pub const INFSTR_SECT_DEVINFS: windows_core::PCWSTR = windows_core::w!("Det.DevINFs");
pub const INFSTR_SECT_DISPLAY_CLEANUP: windows_core::PCWSTR = windows_core::w!("DisplayCleanup");
pub const INFSTR_SECT_EXTENSIONCONTRACTS: windows_core::PCWSTR = windows_core::w!("ExtensionContracts");
pub const INFSTR_SECT_FORCEHWVERIFY: windows_core::PCWSTR = windows_core::w!("Det.ForceHWVerify");
pub const INFSTR_SECT_GOODACPIBIOS: windows_core::PCWSTR = windows_core::w!("GoodACPIBios");
pub const INFSTR_SECT_HPOMNIBOOK: windows_core::PCWSTR = windows_core::w!("Det.HPOmnibook");
pub const INFSTR_SECT_INTERFACE_INSTALL_32: windows_core::PCWSTR = windows_core::w!("InterfaceInstall32");
pub const INFSTR_SECT_MACHINEIDBIOS: windows_core::PCWSTR = windows_core::w!("MachineIDBios");
pub const INFSTR_SECT_MANUALDEV: windows_core::PCWSTR = windows_core::w!("Det.ManualDev");
pub const INFSTR_SECT_MFG: windows_core::PCWSTR = windows_core::w!("Manufacturer");
pub const INFSTR_SECT_REGCFGSYSDEV: windows_core::PCWSTR = windows_core::w!("Det.RegCfgSysDev");
pub const INFSTR_SECT_REGENVDEV: windows_core::PCWSTR = windows_core::w!("Det.RegEnvDev");
pub const INFSTR_SECT_REGINIDEV: windows_core::PCWSTR = windows_core::w!("Det.RegIniDev");
pub const INFSTR_SECT_SYSINI: windows_core::PCWSTR = windows_core::w!("SystemIniDrivers");
pub const INFSTR_SECT_SYSINIDRV: windows_core::PCWSTR = windows_core::w!("SystemIniDriversLine");
pub const INFSTR_SECT_TARGETCOMPUTERS: windows_core::PCWSTR = windows_core::w!("TargetComputers");
pub const INFSTR_SECT_VERSION: windows_core::PCWSTR = windows_core::w!("Version");
pub const INFSTR_SECT_WININIRUN: windows_core::PCWSTR = windows_core::w!("WinIniRunLine");
pub const INFSTR_SOFTWAREVERSION_SECTION: windows_core::PCWSTR = windows_core::w!("SoftwareVersion");
pub const INFSTR_STRKEY_DRVDESC: windows_core::PCWSTR = windows_core::w!("DriverDesc");
pub const INFSTR_SUBKEY_COINSTALLERS: windows_core::PCWSTR = windows_core::w!("CoInstallers");
pub const INFSTR_SUBKEY_CTL: windows_core::PCWSTR = windows_core::w!("CTL");
pub const INFSTR_SUBKEY_DET: windows_core::PCWSTR = windows_core::w!("Det");
pub const INFSTR_SUBKEY_EVENTS: windows_core::PCWSTR = windows_core::w!("Events");
pub const INFSTR_SUBKEY_FACTDEF: windows_core::PCWSTR = windows_core::w!("FactDef");
pub const INFSTR_SUBKEY_FILTERS: windows_core::PCWSTR = windows_core::w!("Filters");
pub const INFSTR_SUBKEY_HW: windows_core::PCWSTR = windows_core::w!("Hw");
pub const INFSTR_SUBKEY_INTERFACES: windows_core::PCWSTR = windows_core::w!("Interfaces");
pub const INFSTR_SUBKEY_LOGCONFIG: windows_core::PCWSTR = windows_core::w!("LogConfig");
pub const INFSTR_SUBKEY_LOGCONFIGOVERRIDE: windows_core::PCWSTR = windows_core::w!("LogConfigOverride");
pub const INFSTR_SUBKEY_NORESOURCEDUPS: windows_core::PCWSTR = windows_core::w!("NoResDup");
pub const INFSTR_SUBKEY_POSSIBLEDUPS: windows_core::PCWSTR = windows_core::w!("PosDup");
pub const INFSTR_SUBKEY_SERVICES: windows_core::PCWSTR = windows_core::w!("Services");
pub const INFSTR_SUBKEY_SOFTWARE: windows_core::PCWSTR = windows_core::w!("Software");
pub const INFSTR_SUBKEY_WMI: windows_core::PCWSTR = windows_core::w!("WMI");
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct INF_STYLE(pub u32);
impl INF_STYLE {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for INF_STYLE {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for INF_STYLE {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for INF_STYLE {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for INF_STYLE {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for INF_STYLE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const INF_STYLE_CACHE_DISABLE: INF_STYLE = INF_STYLE(32u32);
pub const INF_STYLE_CACHE_ENABLE: INF_STYLE = INF_STYLE(16u32);
pub const INF_STYLE_CACHE_IGNORE: INF_STYLE = INF_STYLE(64u32);
pub const INF_STYLE_NONE: INF_STYLE = INF_STYLE(0u32);
pub const INF_STYLE_OLDNT: INF_STYLE = INF_STYLE(1u32);
pub const INF_STYLE_WIN4: INF_STYLE = INF_STYLE(2u32);
pub const INSTALLFLAG_BITS: UPDATEDRIVERFORPLUGANDPLAYDEVICES_FLAGS = UPDATEDRIVERFORPLUGANDPLAYDEVICES_FLAGS(7u32);
pub const INSTALLFLAG_FORCE: UPDATEDRIVERFORPLUGANDPLAYDEVICES_FLAGS = UPDATEDRIVERFORPLUGANDPLAYDEVICES_FLAGS(1u32);
pub const INSTALLFLAG_NONINTERACTIVE: UPDATEDRIVERFORPLUGANDPLAYDEVICES_FLAGS = UPDATEDRIVERFORPLUGANDPLAYDEVICES_FLAGS(4u32);
pub const INSTALLFLAG_READONLY: UPDATEDRIVERFORPLUGANDPLAYDEVICES_FLAGS = UPDATEDRIVERFORPLUGANDPLAYDEVICES_FLAGS(2u32);
pub const IOA_Local: u32 = 255u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct IOD_DESFLAGS(pub u32);
impl IOD_DESFLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for IOD_DESFLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for IOD_DESFLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for IOD_DESFLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for IOD_DESFLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for IOD_DESFLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const IO_ALIAS_10_BIT_DECODE: u32 = 4u32;
pub const IO_ALIAS_12_BIT_DECODE: u32 = 16u32;
pub const IO_ALIAS_16_BIT_DECODE: u32 = 0u32;
pub const IO_ALIAS_POSITIVE_DECODE: u32 = 255u32;
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct IO_DES {
    pub IOD_Count: u32,
    pub IOD_Type: u32,
    pub IOD_Alloc_Base: u64,
    pub IOD_Alloc_End: u64,
    pub IOD_DesFlags: IOD_DESFLAGS,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct IO_RANGE {
    pub IOR_Align: u64,
    pub IOR_nPorts: u32,
    pub IOR_Min: u64,
    pub IOR_Max: u64,
    pub IOR_RangeFlags: IOD_DESFLAGS,
    pub IOR_Alias: u64,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct IO_RESOURCE {
    pub IO_Header: IO_DES,
    pub IO_Data: [IO_RANGE; 1],
}
impl Default for IO_RESOURCE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct IRQD_FLAGS(pub u32);
impl IRQD_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for IRQD_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for IRQD_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for IRQD_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for IRQD_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for IRQD_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct IRQ_DES_32 {
    pub IRQD_Count: u32,
    pub IRQD_Type: u32,
    pub IRQD_Flags: IRQD_FLAGS,
    pub IRQD_Alloc_Num: u32,
    pub IRQD_Affinity: u32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct IRQ_DES_64 {
    pub IRQD_Count: u32,
    pub IRQD_Type: u32,
    pub IRQD_Flags: IRQD_FLAGS,
    pub IRQD_Alloc_Num: u32,
    pub IRQD_Affinity: u64,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct IRQ_RANGE {
    pub IRQR_Min: u32,
    pub IRQR_Max: u32,
    pub IRQR_Flags: IRQD_FLAGS,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct IRQ_RESOURCE_32 {
    pub IRQ_Header: IRQ_DES_32,
    pub IRQ_Data: [IRQ_RANGE; 1],
}
impl Default for IRQ_RESOURCE_32 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct IRQ_RESOURCE_64 {
    pub IRQ_Header: IRQ_DES_64,
    pub IRQ_Data: [IRQ_RANGE; 1],
}
impl Default for IRQ_RESOURCE_64 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const LCPRI_BOOTCONFIG: u32 = 1u32;
pub const LCPRI_DESIRED: u32 = 8192u32;
pub const LCPRI_DISABLED: u32 = 65535u32;
pub const LCPRI_FORCECONFIG: u32 = 0u32;
pub const LCPRI_HARDRECONFIG: u32 = 49152u32;
pub const LCPRI_HARDWIRED: u32 = 57344u32;
pub const LCPRI_IMPOSSIBLE: u32 = 61440u32;
pub const LCPRI_LASTBESTCONFIG: u32 = 16383u32;
pub const LCPRI_LASTSOFTCONFIG: u32 = 32767u32;
pub const LCPRI_NORMAL: u32 = 12288u32;
pub const LCPRI_POWEROFF: u32 = 40960u32;
pub const LCPRI_REBOOT: u32 = 36864u32;
pub const LCPRI_RESTART: u32 = 32768u32;
pub const LCPRI_SUBOPTIMAL: u32 = 20480u32;
pub const LINE_LEN: u32 = 256u32;
pub const LOG_CONF_BITS: u32 = 7u32;
pub const LogSevError: u32 = 2u32;
pub const LogSevFatalError: u32 = 3u32;
pub const LogSevInformation: u32 = 0u32;
pub const LogSevMaximum: u32 = 4u32;
pub const LogSevWarning: u32 = 1u32;
pub const MAX_CLASS_NAME_LEN: u32 = 32u32;
pub const MAX_CONFIG_VALUE: u32 = 9999u32;
pub const MAX_DEVICE_ID_LEN: u32 = 200u32;
pub const MAX_DEVNODE_ID_LEN: u32 = 200u32;
pub const MAX_DMA_CHANNELS: u32 = 7u32;
pub const MAX_GUID_STRING_LEN: u32 = 39u32;
pub const MAX_IDD_DYNAWIZ_RESOURCE_ID: u32 = 11000u32;
pub const MAX_INFSTR_STRKEY_LEN: u32 = 32u32;
pub const MAX_INF_FLAG: u32 = 20u32;
pub const MAX_INF_SECTION_NAME_LENGTH: u32 = 255u32;
pub const MAX_INF_STRING_LENGTH: u32 = 4096u32;
pub const MAX_INSTALLWIZARD_DYNAPAGES: u32 = 20u32;
pub const MAX_INSTANCE_VALUE: u32 = 9999u32;
pub const MAX_INSTRUCTION_LEN: u32 = 256u32;
pub const MAX_IO_PORTS: u32 = 20u32;
pub const MAX_IRQS: u32 = 7u32;
pub const MAX_KEY_LEN: u32 = 100u32;
pub const MAX_LABEL_LEN: u32 = 30u32;
pub const MAX_LCPRI: u32 = 65535u32;
pub const MAX_MEM_REGISTERS: u32 = 9u32;
pub const MAX_PRIORITYSTR_LEN: u32 = 16u32;
pub const MAX_PROFILE_LEN: u32 = 80u32;
pub const MAX_SERVICE_NAME_LEN: u32 = 256u32;
pub const MAX_SUBTITLE_LEN: u32 = 256u32;
pub const MAX_TITLE_LEN: u32 = 60u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MD_FLAGS(pub u32);
impl MD_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for MD_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for MD_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for MD_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for MD_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for MD_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct MEM_DES {
    pub MD_Count: u32,
    pub MD_Type: u32,
    pub MD_Alloc_Base: u64,
    pub MD_Alloc_End: u64,
    pub MD_Flags: MD_FLAGS,
    pub MD_Reserved: u32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct MEM_LARGE_DES {
    pub MLD_Count: u32,
    pub MLD_Type: u32,
    pub MLD_Alloc_Base: u64,
    pub MLD_Alloc_End: u64,
    pub MLD_Flags: u32,
    pub MLD_Reserved: u32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct MEM_LARGE_RANGE {
    pub MLR_Align: u64,
    pub MLR_nBytes: u64,
    pub MLR_Min: u64,
    pub MLR_Max: u64,
    pub MLR_Flags: u32,
    pub MLR_Reserved: u32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct MEM_LARGE_RESOURCE {
    pub MEM_LARGE_Header: MEM_LARGE_DES,
    pub MEM_LARGE_Data: [MEM_LARGE_RANGE; 1],
}
impl Default for MEM_LARGE_RESOURCE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct MEM_RANGE {
    pub MR_Align: u64,
    pub MR_nBytes: u32,
    pub MR_Min: u64,
    pub MR_Max: u64,
    pub MR_Flags: MD_FLAGS,
    pub MR_Reserved: u32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct MEM_RESOURCE {
    pub MEM_Header: MEM_DES,
    pub MEM_Data: [MEM_RANGE; 1],
}
impl Default for MEM_RESOURCE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct MFCARD_DES {
    pub PMF_Count: u32,
    pub PMF_Type: u32,
    pub PMF_Flags: PMF_FLAGS,
    pub PMF_ConfigOptions: u8,
    pub PMF_IoResourceIndex: u8,
    pub PMF_Reserved: [u8; 2],
    pub PMF_ConfigRegisterBase: u32,
}
impl Default for MFCARD_DES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct MFCARD_RESOURCE {
    pub MfCard_Header: MFCARD_DES,
}
pub const MIN_IDD_DYNAWIZ_RESOURCE_ID: u32 = 10000u32;
pub const NDW_INSTALLFLAG_CI_PICKED_OEM: u32 = 32768u32;
pub const NDW_INSTALLFLAG_DIDFACTDEFS: u32 = 1u32;
pub const NDW_INSTALLFLAG_EXPRESSINTRO: u32 = 1024u32;
pub const NDW_INSTALLFLAG_HARDWAREALLREADYIN: u32 = 2u32;
pub const NDW_INSTALLFLAG_INSTALLSPECIFIC: u32 = 8192u32;
pub const NDW_INSTALLFLAG_KNOWNCLASS: u32 = 524288u32;
pub const NDW_INSTALLFLAG_NEEDSHUTDOWN: u32 = 512u32;
pub const NDW_INSTALLFLAG_NODETECTEDDEVS: u32 = 4096u32;
pub const NDW_INSTALLFLAG_PCMCIADEVICE: u32 = 131072u32;
pub const NDW_INSTALLFLAG_PCMCIAMODE: u32 = 65536u32;
pub const NDW_INSTALLFLAG_SKIPCLASSLIST: u32 = 16384u32;
pub const NDW_INSTALLFLAG_SKIPISDEVINSTALLED: u32 = 2048u32;
pub const NDW_INSTALLFLAG_USERCANCEL: u32 = 262144u32;
pub const NUM_CM_PROB: u32 = 58u32;
pub const NUM_CM_PROB_V1: u32 = 37u32;
pub const NUM_CM_PROB_V2: u32 = 50u32;
pub const NUM_CM_PROB_V3: u32 = 51u32;
pub const NUM_CM_PROB_V4: u32 = 52u32;
pub const NUM_CM_PROB_V5: u32 = 53u32;
pub const NUM_CM_PROB_V6: u32 = 54u32;
pub const NUM_CM_PROB_V7: u32 = 55u32;
pub const NUM_CM_PROB_V8: u32 = 57u32;
pub const NUM_CM_PROB_V9: u32 = 58u32;
pub const NUM_CR_RESULTS: CONFIGRET = CONFIGRET(60u32);
pub const NUM_LOG_CONF: CM_LOG_CONF = CM_LOG_CONF(6u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct OEM_SOURCE_MEDIA_TYPE(pub u32);
pub const OVERRIDE_LOG_CONF: CM_LOG_CONF = CM_LOG_CONF(5u32);
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct PCCARD_DES {
    pub PCD_Count: u32,
    pub PCD_Type: u32,
    pub PCD_Flags: PCD_FLAGS,
    pub PCD_ConfigIndex: u8,
    pub PCD_Reserved: [u8; 3],
    pub PCD_MemoryCardBase1: u32,
    pub PCD_MemoryCardBase2: u32,
    pub PCD_MemoryCardBase: [u32; 2],
    pub PCD_MemoryFlags: [u16; 2],
    pub PCD_IoFlags: [u8; 2],
}
impl Default for PCCARD_DES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct PCCARD_RESOURCE {
    pub PcCard_Header: PCCARD_DES,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PCD_FLAGS(pub u32);
impl PCD_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for PCD_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for PCD_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for PCD_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for PCD_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for PCD_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const PCD_MAX_IO: u32 = 2u32;
pub const PCD_MAX_MEMORY: u32 = 2u32;
pub type PCM_NOTIFY_CALLBACK = Option<unsafe extern "system" fn(hnotify: HCMNOTIFICATION, context: *const core::ffi::c_void, action: CM_NOTIFY_ACTION, eventdata: *const CM_NOTIFY_EVENT_DATA, eventdatasize: u32) -> u32>;
pub type PDETECT_PROGRESS_NOTIFY = Option<unsafe extern "system" fn(progressnotifyparam: *const core::ffi::c_void, detectcomplete: u32) -> windows_core::BOOL>;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PMF_FLAGS(pub u32);
impl PMF_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for PMF_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for PMF_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for PMF_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for PMF_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for PMF_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PNP_VETO_TYPE(pub i32);
pub const PNP_VetoAlreadyRemoved: PNP_VETO_TYPE = PNP_VETO_TYPE(13i32);
pub const PNP_VetoDevice: PNP_VETO_TYPE = PNP_VETO_TYPE(6i32);
pub const PNP_VetoDriver: PNP_VETO_TYPE = PNP_VETO_TYPE(7i32);
pub const PNP_VetoIllegalDeviceRequest: PNP_VETO_TYPE = PNP_VETO_TYPE(8i32);
pub const PNP_VetoInsufficientPower: PNP_VETO_TYPE = PNP_VETO_TYPE(9i32);
pub const PNP_VetoInsufficientRights: PNP_VETO_TYPE = PNP_VETO_TYPE(12i32);
pub const PNP_VetoLegacyDevice: PNP_VETO_TYPE = PNP_VETO_TYPE(1i32);
pub const PNP_VetoLegacyDriver: PNP_VETO_TYPE = PNP_VETO_TYPE(11i32);
pub const PNP_VetoNonDisableable: PNP_VETO_TYPE = PNP_VETO_TYPE(10i32);
pub const PNP_VetoOutstandingOpen: PNP_VETO_TYPE = PNP_VETO_TYPE(5i32);
pub const PNP_VetoPendingClose: PNP_VETO_TYPE = PNP_VETO_TYPE(2i32);
pub const PNP_VetoTypeUnknown: PNP_VETO_TYPE = PNP_VETO_TYPE(0i32);
pub const PNP_VetoWindowsApp: PNP_VETO_TYPE = PNP_VETO_TYPE(3i32);
pub const PNP_VetoWindowsService: PNP_VETO_TYPE = PNP_VETO_TYPE(4i32);
pub const PRIORITY_BIT: u32 = 8u32;
pub const PRIORITY_EQUAL_FIRST: u32 = 8u32;
pub const PRIORITY_EQUAL_LAST: u32 = 0u32;
pub type PSP_DETSIG_CMPPROC = Option<unsafe extern "system" fn(deviceinfoset: HDEVINFO, newdevicedata: *const SP_DEVINFO_DATA, existingdevicedata: *const SP_DEVINFO_DATA, comparecontext: *const core::ffi::c_void) -> u32>;
pub type PSP_FILE_CALLBACK_A = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, notification: u32, param1: usize, param2: usize) -> u32>;
pub type PSP_FILE_CALLBACK_W = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, notification: u32, param1: usize, param2: usize) -> u32>;
pub const ROLLBACK_BITS: DIROLLBACKDRIVER_FLAGS = DIROLLBACKDRIVER_FLAGS(1u32);
pub const ROLLBACK_FLAG_NO_UI: DIROLLBACKDRIVER_FLAGS = DIROLLBACKDRIVER_FLAGS(1u32);
pub const RegDisposition_Bits: u32 = 1u32;
pub const RegDisposition_OpenAlways: u32 = 0u32;
pub const RegDisposition_OpenExisting: u32 = 1u32;
pub const ResType_All: CM_RESTYPE = CM_RESTYPE(0u32);
pub const ResType_BusNumber: CM_RESTYPE = CM_RESTYPE(6u32);
pub const ResType_ClassSpecific: CM_RESTYPE = CM_RESTYPE(65535u32);
pub const ResType_Connection: CM_RESTYPE = CM_RESTYPE(32772u32);
pub const ResType_DMA: CM_RESTYPE = CM_RESTYPE(3u32);
pub const ResType_DevicePrivate: CM_RESTYPE = CM_RESTYPE(32769u32);
pub const ResType_DoNotUse: CM_RESTYPE = CM_RESTYPE(5u32);
pub const ResType_IO: CM_RESTYPE = CM_RESTYPE(2u32);
pub const ResType_IRQ: CM_RESTYPE = CM_RESTYPE(4u32);
pub const ResType_Ignored_Bit: CM_RESTYPE = CM_RESTYPE(32768u32);
pub const ResType_MAX: CM_RESTYPE = CM_RESTYPE(7u32);
pub const ResType_Mem: CM_RESTYPE = CM_RESTYPE(1u32);
pub const ResType_MemLarge: CM_RESTYPE = CM_RESTYPE(7u32);
pub const ResType_MfCardConfig: CM_RESTYPE = CM_RESTYPE(32771u32);
pub const ResType_None: CM_RESTYPE = CM_RESTYPE(0u32);
pub const ResType_PcCardConfig: CM_RESTYPE = CM_RESTYPE(32770u32);
pub const ResType_Reserved: CM_RESTYPE = CM_RESTYPE(32768u32);
pub const SCWMI_CLOBBER_SECURITY: u32 = 1u32;
pub const SETDIRID_NOT_FULL_PATH: u32 = 1u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SETUPSCANFILEQUEUE_FLAGS(pub u32);
impl SETUPSCANFILEQUEUE_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for SETUPSCANFILEQUEUE_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for SETUPSCANFILEQUEUE_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for SETUPSCANFILEQUEUE_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for SETUPSCANFILEQUEUE_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for SETUPSCANFILEQUEUE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SETUP_DI_DEVICE_CONFIGURATION_FLAGS(pub u32);
impl SETUP_DI_DEVICE_CONFIGURATION_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for SETUP_DI_DEVICE_CONFIGURATION_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for SETUP_DI_DEVICE_CONFIGURATION_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for SETUP_DI_DEVICE_CONFIGURATION_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for SETUP_DI_DEVICE_CONFIGURATION_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for SETUP_DI_DEVICE_CONFIGURATION_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SETUP_DI_DEVICE_CREATION_FLAGS(pub u32);
impl SETUP_DI_DEVICE_CREATION_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for SETUP_DI_DEVICE_CREATION_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for SETUP_DI_DEVICE_CREATION_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for SETUP_DI_DEVICE_CREATION_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for SETUP_DI_DEVICE_CREATION_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for SETUP_DI_DEVICE_CREATION_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SETUP_DI_DEVICE_INSTALL_FLAGS(pub u32);
impl SETUP_DI_DEVICE_INSTALL_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for SETUP_DI_DEVICE_INSTALL_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for SETUP_DI_DEVICE_INSTALL_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for SETUP_DI_DEVICE_INSTALL_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for SETUP_DI_DEVICE_INSTALL_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for SETUP_DI_DEVICE_INSTALL_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SETUP_DI_DEVICE_INSTALL_FLAGS_EX(pub u32);
impl SETUP_DI_DEVICE_INSTALL_FLAGS_EX {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for SETUP_DI_DEVICE_INSTALL_FLAGS_EX {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for SETUP_DI_DEVICE_INSTALL_FLAGS_EX {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for SETUP_DI_DEVICE_INSTALL_FLAGS_EX {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for SETUP_DI_DEVICE_INSTALL_FLAGS_EX {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for SETUP_DI_DEVICE_INSTALL_FLAGS_EX {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SETUP_DI_DRIVER_INSTALL_FLAGS(pub u32);
impl SETUP_DI_DRIVER_INSTALL_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for SETUP_DI_DRIVER_INSTALL_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for SETUP_DI_DRIVER_INSTALL_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for SETUP_DI_DRIVER_INSTALL_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for SETUP_DI_DRIVER_INSTALL_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for SETUP_DI_DRIVER_INSTALL_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SETUP_DI_DRIVER_TYPE(pub u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SETUP_DI_GET_CLASS_DEVS_FLAGS(pub u32);
impl SETUP_DI_GET_CLASS_DEVS_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for SETUP_DI_GET_CLASS_DEVS_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for SETUP_DI_GET_CLASS_DEVS_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for SETUP_DI_GET_CLASS_DEVS_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for SETUP_DI_GET_CLASS_DEVS_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for SETUP_DI_GET_CLASS_DEVS_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SETUP_DI_PROPERTY_CHANGE_SCOPE(pub u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SETUP_DI_REGISTRY_PROPERTY(pub u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SETUP_DI_REMOVE_DEVICE_SCOPE(pub u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SETUP_DI_STATE_CHANGE(pub u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SETUP_FILE_OPERATION(pub u32);
pub const SIGNERSCORE_AUTHENTICODE: u32 = 251658240u32;
pub const SIGNERSCORE_INBOX: u32 = 218103811u32;
pub const SIGNERSCORE_LOGO_PREMIUM: u32 = 218103809u32;
pub const SIGNERSCORE_LOGO_STANDARD: u32 = 218103810u32;
pub const SIGNERSCORE_MASK: u32 = 4278190080u32;
pub const SIGNERSCORE_SIGNED_MASK: u32 = 4026531840u32;
pub const SIGNERSCORE_UNCLASSIFIED: u32 = 218103812u32;
pub const SIGNERSCORE_UNKNOWN: u32 = 4278190080u32;
pub const SIGNERSCORE_UNSIGNED: u32 = 2147483648u32;
pub const SIGNERSCORE_W9X_SUSPECT: u32 = 3221225472u32;
pub const SIGNERSCORE_WHQL: u32 = 218103813u32;
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy, Default)]
pub struct SOURCE_MEDIA_A {
    pub Reserved: windows_core::PCSTR,
    pub Tagfile: windows_core::PCSTR,
    pub Description: windows_core::PCSTR,
    pub SourcePath: windows_core::PCSTR,
    pub SourceFile: windows_core::PCSTR,
    pub Flags: u32,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SOURCE_MEDIA_A {
    pub Reserved: windows_core::PCSTR,
    pub Tagfile: windows_core::PCSTR,
    pub Description: windows_core::PCSTR,
    pub SourcePath: windows_core::PCSTR,
    pub SourceFile: windows_core::PCSTR,
    pub Flags: u32,
}
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy, Default)]
pub struct SOURCE_MEDIA_W {
    pub Reserved: windows_core::PCWSTR,
    pub Tagfile: windows_core::PCWSTR,
    pub Description: windows_core::PCWSTR,
    pub SourcePath: windows_core::PCWSTR,
    pub SourceFile: windows_core::PCWSTR,
    pub Flags: u32,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SOURCE_MEDIA_W {
    pub Reserved: windows_core::PCWSTR,
    pub Tagfile: windows_core::PCWSTR,
    pub Description: windows_core::PCWSTR,
    pub SourcePath: windows_core::PCWSTR,
    pub SourceFile: windows_core::PCWSTR,
    pub Flags: u32,
}
pub const SPCRP_CHARACTERISTICS: u32 = 27u32;
pub const SPCRP_DEVTYPE: u32 = 25u32;
pub const SPCRP_EXCLUSIVE: u32 = 26u32;
pub const SPCRP_LOWERFILTERS: u32 = 18u32;
pub const SPCRP_MAXIMUM_PROPERTY: u32 = 28u32;
pub const SPCRP_SECURITY: u32 = 23u32;
pub const SPCRP_SECURITY_SDS: u32 = 24u32;
pub const SPCRP_UPPERFILTERS: u32 = 17u32;
pub const SPDIT_CLASSDRIVER: SETUP_DI_DRIVER_TYPE = SETUP_DI_DRIVER_TYPE(1u32);
pub const SPDIT_COMPATDRIVER: SETUP_DI_DRIVER_TYPE = SETUP_DI_DRIVER_TYPE(2u32);
pub const SPDIT_NODRIVER: u32 = 0u32;
pub const SPDRP_ADDRESS: SETUP_DI_REGISTRY_PROPERTY = SETUP_DI_REGISTRY_PROPERTY(28u32);
pub const SPDRP_BASE_CONTAINERID: SETUP_DI_REGISTRY_PROPERTY = SETUP_DI_REGISTRY_PROPERTY(36u32);
pub const SPDRP_BUSNUMBER: SETUP_DI_REGISTRY_PROPERTY = SETUP_DI_REGISTRY_PROPERTY(21u32);
pub const SPDRP_BUSTYPEGUID: SETUP_DI_REGISTRY_PROPERTY = SETUP_DI_REGISTRY_PROPERTY(19u32);
pub const SPDRP_CAPABILITIES: SETUP_DI_REGISTRY_PROPERTY = SETUP_DI_REGISTRY_PROPERTY(15u32);
pub const SPDRP_CHARACTERISTICS: SETUP_DI_REGISTRY_PROPERTY = SETUP_DI_REGISTRY_PROPERTY(27u32);
pub const SPDRP_CLASS: SETUP_DI_REGISTRY_PROPERTY = SETUP_DI_REGISTRY_PROPERTY(7u32);
pub const SPDRP_CLASSGUID: SETUP_DI_REGISTRY_PROPERTY = SETUP_DI_REGISTRY_PROPERTY(8u32);
pub const SPDRP_COMPATIBLEIDS: SETUP_DI_REGISTRY_PROPERTY = SETUP_DI_REGISTRY_PROPERTY(2u32);
pub const SPDRP_CONFIGFLAGS: SETUP_DI_REGISTRY_PROPERTY = SETUP_DI_REGISTRY_PROPERTY(10u32);
pub const SPDRP_DEVICEDESC: SETUP_DI_REGISTRY_PROPERTY = SETUP_DI_REGISTRY_PROPERTY(0u32);
pub const SPDRP_DEVICE_POWER_DATA: SETUP_DI_REGISTRY_PROPERTY = SETUP_DI_REGISTRY_PROPERTY(30u32);
pub const SPDRP_DEVTYPE: SETUP_DI_REGISTRY_PROPERTY = SETUP_DI_REGISTRY_PROPERTY(25u32);
pub const SPDRP_DRIVER: SETUP_DI_REGISTRY_PROPERTY = SETUP_DI_REGISTRY_PROPERTY(9u32);
pub const SPDRP_ENUMERATOR_NAME: SETUP_DI_REGISTRY_PROPERTY = SETUP_DI_REGISTRY_PROPERTY(22u32);
pub const SPDRP_EXCLUSIVE: SETUP_DI_REGISTRY_PROPERTY = SETUP_DI_REGISTRY_PROPERTY(26u32);
pub const SPDRP_FRIENDLYNAME: SETUP_DI_REGISTRY_PROPERTY = SETUP_DI_REGISTRY_PROPERTY(12u32);
pub const SPDRP_HARDWAREID: SETUP_DI_REGISTRY_PROPERTY = SETUP_DI_REGISTRY_PROPERTY(1u32);
pub const SPDRP_INSTALL_STATE: SETUP_DI_REGISTRY_PROPERTY = SETUP_DI_REGISTRY_PROPERTY(34u32);
pub const SPDRP_LEGACYBUSTYPE: SETUP_DI_REGISTRY_PROPERTY = SETUP_DI_REGISTRY_PROPERTY(20u32);
pub const SPDRP_LOCATION_INFORMATION: SETUP_DI_REGISTRY_PROPERTY = SETUP_DI_REGISTRY_PROPERTY(13u32);
pub const SPDRP_LOCATION_PATHS: SETUP_DI_REGISTRY_PROPERTY = SETUP_DI_REGISTRY_PROPERTY(35u32);
pub const SPDRP_LOWERFILTERS: SETUP_DI_REGISTRY_PROPERTY = SETUP_DI_REGISTRY_PROPERTY(18u32);
pub const SPDRP_MAXIMUM_PROPERTY: SETUP_DI_REGISTRY_PROPERTY = SETUP_DI_REGISTRY_PROPERTY(37u32);
pub const SPDRP_MFG: SETUP_DI_REGISTRY_PROPERTY = SETUP_DI_REGISTRY_PROPERTY(11u32);
pub const SPDRP_PHYSICAL_DEVICE_OBJECT_NAME: SETUP_DI_REGISTRY_PROPERTY = SETUP_DI_REGISTRY_PROPERTY(14u32);
pub const SPDRP_REMOVAL_POLICY: SETUP_DI_REGISTRY_PROPERTY = SETUP_DI_REGISTRY_PROPERTY(31u32);
pub const SPDRP_REMOVAL_POLICY_HW_DEFAULT: SETUP_DI_REGISTRY_PROPERTY = SETUP_DI_REGISTRY_PROPERTY(32u32);
pub const SPDRP_REMOVAL_POLICY_OVERRIDE: SETUP_DI_REGISTRY_PROPERTY = SETUP_DI_REGISTRY_PROPERTY(33u32);
pub const SPDRP_SECURITY: SETUP_DI_REGISTRY_PROPERTY = SETUP_DI_REGISTRY_PROPERTY(23u32);
pub const SPDRP_SECURITY_SDS: SETUP_DI_REGISTRY_PROPERTY = SETUP_DI_REGISTRY_PROPERTY(24u32);
pub const SPDRP_SERVICE: SETUP_DI_REGISTRY_PROPERTY = SETUP_DI_REGISTRY_PROPERTY(4u32);
pub const SPDRP_UI_NUMBER: SETUP_DI_REGISTRY_PROPERTY = SETUP_DI_REGISTRY_PROPERTY(16u32);
pub const SPDRP_UI_NUMBER_DESC_FORMAT: SETUP_DI_REGISTRY_PROPERTY = SETUP_DI_REGISTRY_PROPERTY(29u32);
pub const SPDRP_UNUSED0: SETUP_DI_REGISTRY_PROPERTY = SETUP_DI_REGISTRY_PROPERTY(3u32);
pub const SPDRP_UNUSED1: SETUP_DI_REGISTRY_PROPERTY = SETUP_DI_REGISTRY_PROPERTY(5u32);
pub const SPDRP_UNUSED2: SETUP_DI_REGISTRY_PROPERTY = SETUP_DI_REGISTRY_PROPERTY(6u32);
pub const SPDRP_UPPERFILTERS: SETUP_DI_REGISTRY_PROPERTY = SETUP_DI_REGISTRY_PROPERTY(17u32);
pub const SPDSL_DISALLOW_NEGATIVE_ADJUST: u32 = 2u32;
pub const SPDSL_IGNORE_DISK: u32 = 1u32;
pub const SPFILELOG_FORCENEW: u32 = 2u32;
pub const SPFILELOG_OEMFILE: u32 = 1u32;
pub const SPFILELOG_QUERYONLY: u32 = 4u32;
pub const SPFILELOG_SYSTEMLOG: u32 = 1u32;
pub const SPFILENOTIFY_BACKUPERROR: u32 = 22u32;
pub const SPFILENOTIFY_CABINETINFO: u32 = 16u32;
pub const SPFILENOTIFY_COPYERROR: u32 = 13u32;
pub const SPFILENOTIFY_DELETEERROR: u32 = 7u32;
pub const SPFILENOTIFY_ENDBACKUP: u32 = 23u32;
pub const SPFILENOTIFY_ENDCOPY: u32 = 12u32;
pub const SPFILENOTIFY_ENDDELETE: u32 = 6u32;
pub const SPFILENOTIFY_ENDQUEUE: u32 = 2u32;
pub const SPFILENOTIFY_ENDREGISTRATION: u32 = 32u32;
pub const SPFILENOTIFY_ENDRENAME: u32 = 9u32;
pub const SPFILENOTIFY_ENDSUBQUEUE: u32 = 4u32;
pub const SPFILENOTIFY_FILEEXTRACTED: u32 = 19u32;
pub const SPFILENOTIFY_FILEINCABINET: u32 = 17u32;
pub const SPFILENOTIFY_FILEOPDELAYED: u32 = 20u32;
pub const SPFILENOTIFY_LANGMISMATCH: u32 = 65536u32;
pub const SPFILENOTIFY_NEEDMEDIA: u32 = 14u32;
pub const SPFILENOTIFY_NEEDNEWCABINET: u32 = 18u32;
pub const SPFILENOTIFY_QUEUESCAN: u32 = 15u32;
pub const SPFILENOTIFY_QUEUESCAN_EX: u32 = 24u32;
pub const SPFILENOTIFY_QUEUESCAN_SIGNERINFO: u32 = 64u32;
pub const SPFILENOTIFY_RENAMEERROR: u32 = 10u32;
pub const SPFILENOTIFY_STARTBACKUP: u32 = 21u32;
pub const SPFILENOTIFY_STARTCOPY: u32 = 11u32;
pub const SPFILENOTIFY_STARTDELETE: u32 = 5u32;
pub const SPFILENOTIFY_STARTQUEUE: u32 = 1u32;
pub const SPFILENOTIFY_STARTREGISTRATION: u32 = 25u32;
pub const SPFILENOTIFY_STARTRENAME: u32 = 8u32;
pub const SPFILENOTIFY_STARTSUBQUEUE: u32 = 3u32;
pub const SPFILENOTIFY_TARGETEXISTS: u32 = 131072u32;
pub const SPFILENOTIFY_TARGETNEWER: u32 = 262144u32;
pub const SPFILEQ_FILE_IN_USE: u32 = 1u32;
pub const SPFILEQ_REBOOT_IN_PROGRESS: u32 = 4u32;
pub const SPFILEQ_REBOOT_RECOMMENDED: u32 = 2u32;
pub const SPID_ACTIVE: u32 = 1u32;
pub const SPID_DEFAULT: u32 = 2u32;
pub const SPID_REMOVED: u32 = 4u32;
pub const SPINST_ALL: u32 = 2047u32;
pub const SPINST_BITREG: u32 = 32u32;
pub const SPINST_COPYINF: u32 = 512u32;
pub const SPINST_DEVICEINSTALL: u32 = 1048576u32;
pub const SPINST_FILES: u32 = 16u32;
pub const SPINST_INI2REG: u32 = 8u32;
pub const SPINST_INIFILES: u32 = 2u32;
pub const SPINST_LOGCONFIG: u32 = 1u32;
pub const SPINST_LOGCONFIGS_ARE_OVERRIDES: u32 = 262144u32;
pub const SPINST_LOGCONFIG_IS_FORCED: u32 = 131072u32;
pub const SPINST_PROFILEITEMS: u32 = 256u32;
pub const SPINST_PROPERTIES: u32 = 1024u32;
pub const SPINST_REGISTERCALLBACKAWARE: u32 = 524288u32;
pub const SPINST_REGISTRY: u32 = 4u32;
pub const SPINST_REGSVR: u32 = 64u32;
pub const SPINST_SINGLESECTION: u32 = 65536u32;
pub const SPINST_UNREGSVR: u32 = 128u32;
pub const SPINT_ACTIVE: u32 = 1u32;
pub const SPINT_DEFAULT: u32 = 2u32;
pub const SPINT_REMOVED: u32 = 4u32;
pub const SPOST_MAX: u32 = 3u32;
pub const SPOST_NONE: OEM_SOURCE_MEDIA_TYPE = OEM_SOURCE_MEDIA_TYPE(0u32);
pub const SPOST_PATH: OEM_SOURCE_MEDIA_TYPE = OEM_SOURCE_MEDIA_TYPE(1u32);
pub const SPOST_URL: OEM_SOURCE_MEDIA_TYPE = OEM_SOURCE_MEDIA_TYPE(2u32);
pub const SPPSR_ENUM_ADV_DEVICE_PROPERTIES: u32 = 3u32;
pub const SPPSR_ENUM_BASIC_DEVICE_PROPERTIES: u32 = 2u32;
pub const SPPSR_SELECT_DEVICE_RESOURCES: u32 = 1u32;
pub const SPQ_DELAYED_COPY: u32 = 1u32;
pub const SPQ_FLAG_ABORT_IF_UNSIGNED: u32 = 2u32;
pub const SPQ_FLAG_BACKUP_AWARE: u32 = 1u32;
pub const SPQ_FLAG_DO_SHUFFLEMOVE: u32 = 8u32;
pub const SPQ_FLAG_FILES_MODIFIED: u32 = 4u32;
pub const SPQ_FLAG_VALID: u32 = 15u32;
pub const SPQ_SCAN_ACTIVATE_DRP: SETUPSCANFILEQUEUE_FLAGS = SETUPSCANFILEQUEUE_FLAGS(1024u32);
pub const SPQ_SCAN_FILE_COMPARISON: SETUPSCANFILEQUEUE_FLAGS = SETUPSCANFILEQUEUE_FLAGS(512u32);
pub const SPQ_SCAN_FILE_PRESENCE: SETUPSCANFILEQUEUE_FLAGS = SETUPSCANFILEQUEUE_FLAGS(1u32);
pub const SPQ_SCAN_FILE_PRESENCE_WITHOUT_SOURCE: SETUPSCANFILEQUEUE_FLAGS = SETUPSCANFILEQUEUE_FLAGS(256u32);
pub const SPQ_SCAN_FILE_VALIDITY: SETUPSCANFILEQUEUE_FLAGS = SETUPSCANFILEQUEUE_FLAGS(2u32);
pub const SPQ_SCAN_INFORM_USER: SETUPSCANFILEQUEUE_FLAGS = SETUPSCANFILEQUEUE_FLAGS(16u32);
pub const SPQ_SCAN_PRUNE_COPY_QUEUE: SETUPSCANFILEQUEUE_FLAGS = SETUPSCANFILEQUEUE_FLAGS(32u32);
pub const SPQ_SCAN_PRUNE_DELREN: SETUPSCANFILEQUEUE_FLAGS = SETUPSCANFILEQUEUE_FLAGS(128u32);
pub const SPQ_SCAN_USE_CALLBACK: SETUPSCANFILEQUEUE_FLAGS = SETUPSCANFILEQUEUE_FLAGS(4u32);
pub const SPQ_SCAN_USE_CALLBACKEX: SETUPSCANFILEQUEUE_FLAGS = SETUPSCANFILEQUEUE_FLAGS(8u32);
pub const SPQ_SCAN_USE_CALLBACK_SIGNERINFO: SETUPSCANFILEQUEUE_FLAGS = SETUPSCANFILEQUEUE_FLAGS(64u32);
pub const SPRDI_FIND_DUPS: u32 = 1u32;
pub const SPREG_DLLINSTALL: u32 = 4u32;
pub const SPREG_GETPROCADDR: u32 = 2u32;
pub const SPREG_LOADLIBRARY: u32 = 1u32;
pub const SPREG_REGSVR: u32 = 3u32;
pub const SPREG_SUCCESS: u32 = 0u32;
pub const SPREG_TIMEOUT: u32 = 5u32;
pub const SPREG_UNKNOWN: u32 = 4294967295u32;
pub const SPSVCINST_ASSOCSERVICE: SPSVCINST_FLAGS = SPSVCINST_FLAGS(2u32);
pub const SPSVCINST_CLOBBER_SECURITY: SPSVCINST_FLAGS = SPSVCINST_FLAGS(1024u32);
pub const SPSVCINST_DELETEEVENTLOGENTRY: SPSVCINST_FLAGS = SPSVCINST_FLAGS(4u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SPSVCINST_FLAGS(pub u32);
impl SPSVCINST_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for SPSVCINST_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for SPSVCINST_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for SPSVCINST_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for SPSVCINST_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for SPSVCINST_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const SPSVCINST_NOCLOBBER_DELAYEDAUTOSTART: SPSVCINST_FLAGS = SPSVCINST_FLAGS(32768u32);
pub const SPSVCINST_NOCLOBBER_DEPENDENCIES: SPSVCINST_FLAGS = SPSVCINST_FLAGS(128u32);
pub const SPSVCINST_NOCLOBBER_DESCRIPTION: SPSVCINST_FLAGS = SPSVCINST_FLAGS(256u32);
pub const SPSVCINST_NOCLOBBER_DISPLAYNAME: SPSVCINST_FLAGS = SPSVCINST_FLAGS(8u32);
pub const SPSVCINST_NOCLOBBER_ERRORCONTROL: SPSVCINST_FLAGS = SPSVCINST_FLAGS(32u32);
pub const SPSVCINST_NOCLOBBER_FAILUREACTIONS: SPSVCINST_FLAGS = SPSVCINST_FLAGS(131072u32);
pub const SPSVCINST_NOCLOBBER_LOADORDERGROUP: SPSVCINST_FLAGS = SPSVCINST_FLAGS(64u32);
pub const SPSVCINST_NOCLOBBER_REQUIREDPRIVILEGES: SPSVCINST_FLAGS = SPSVCINST_FLAGS(4096u32);
pub const SPSVCINST_NOCLOBBER_SERVICESIDTYPE: SPSVCINST_FLAGS = SPSVCINST_FLAGS(16384u32);
pub const SPSVCINST_NOCLOBBER_STARTTYPE: SPSVCINST_FLAGS = SPSVCINST_FLAGS(16u32);
pub const SPSVCINST_NOCLOBBER_TRIGGERS: SPSVCINST_FLAGS = SPSVCINST_FLAGS(8192u32);
pub const SPSVCINST_STARTSERVICE: SPSVCINST_FLAGS = SPSVCINST_FLAGS(2048u32);
pub const SPSVCINST_STOPSERVICE: SPSVCINST_FLAGS = SPSVCINST_FLAGS(512u32);
pub const SPSVCINST_TAGTOFRONT: SPSVCINST_FLAGS = SPSVCINST_FLAGS(1u32);
pub const SPSVCINST_UNIQUE_NAME: SPSVCINST_FLAGS = SPSVCINST_FLAGS(65536u32);
pub const SPWPT_SELECTDEVICE: u32 = 1u32;
pub const SPWP_USE_DEVINFO_DATA: u32 = 1u32;
pub const SP_ALTPLATFORM_FLAGS_SUITE_MASK: u32 = 2u32;
pub const SP_ALTPLATFORM_FLAGS_VERSION_RANGE: u32 = 1u32;
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_System_Diagnostics_Debug")]
#[derive(Clone, Copy, Default)]
pub struct SP_ALTPLATFORM_INFO_V1 {
    pub cbSize: u32,
    pub Platform: super::super::System::Diagnostics::Debug::VER_PLATFORM,
    pub MajorVersion: u32,
    pub MinorVersion: u32,
    pub ProcessorArchitecture: u16,
    pub Reserved: u16,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "Win32_System_Diagnostics_Debug")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SP_ALTPLATFORM_INFO_V1 {
    pub cbSize: u32,
    pub Platform: super::super::System::Diagnostics::Debug::VER_PLATFORM,
    pub MajorVersion: u32,
    pub MinorVersion: u32,
    pub ProcessorArchitecture: u16,
    pub Reserved: u16,
}
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "Win32_System_Diagnostics_Debug", feature = "Win32_System_SystemInformation"))]
#[derive(Clone, Copy)]
pub struct SP_ALTPLATFORM_INFO_V2 {
    pub cbSize: u32,
    pub Platform: super::super::System::Diagnostics::Debug::VER_PLATFORM,
    pub MajorVersion: u32,
    pub MinorVersion: u32,
    pub ProcessorArchitecture: super::super::System::SystemInformation::PROCESSOR_ARCHITECTURE,
    pub Anonymous: SP_ALTPLATFORM_INFO_V2_0,
    pub FirstValidatedMajorVersion: u32,
    pub FirstValidatedMinorVersion: u32,
}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "Win32_System_Diagnostics_Debug", feature = "Win32_System_SystemInformation"))]
impl Default for SP_ALTPLATFORM_INFO_V2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "Win32_System_Diagnostics_Debug", feature = "Win32_System_SystemInformation"))]
#[derive(Clone, Copy)]
pub union SP_ALTPLATFORM_INFO_V2_0 {
    pub Reserved: u16,
    pub Flags: u16,
}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "Win32_System_Diagnostics_Debug", feature = "Win32_System_SystemInformation"))]
impl Default for SP_ALTPLATFORM_INFO_V2_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_System_Diagnostics_Debug", feature = "Win32_System_SystemInformation"))]
#[derive(Clone, Copy)]
pub struct SP_ALTPLATFORM_INFO_V2 {
    pub cbSize: u32,
    pub Platform: super::super::System::Diagnostics::Debug::VER_PLATFORM,
    pub MajorVersion: u32,
    pub MinorVersion: u32,
    pub ProcessorArchitecture: super::super::System::SystemInformation::PROCESSOR_ARCHITECTURE,
    pub Anonymous: SP_ALTPLATFORM_INFO_V2_0,
    pub FirstValidatedMajorVersion: u32,
    pub FirstValidatedMinorVersion: u32,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_System_Diagnostics_Debug", feature = "Win32_System_SystemInformation"))]
impl Default for SP_ALTPLATFORM_INFO_V2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_System_Diagnostics_Debug", feature = "Win32_System_SystemInformation"))]
#[derive(Clone, Copy)]
pub union SP_ALTPLATFORM_INFO_V2_0 {
    pub Reserved: u16,
    pub Flags: u16,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_System_Diagnostics_Debug", feature = "Win32_System_SystemInformation"))]
impl Default for SP_ALTPLATFORM_INFO_V2_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy)]
pub struct SP_ALTPLATFORM_INFO_V3 {
    pub cbSize: u32,
    pub Platform: u32,
    pub MajorVersion: u32,
    pub MinorVersion: u32,
    pub ProcessorArchitecture: u16,
    pub Anonymous: SP_ALTPLATFORM_INFO_V3_0,
    pub FirstValidatedMajorVersion: u32,
    pub FirstValidatedMinorVersion: u32,
    pub ProductType: u8,
    pub SuiteMask: u16,
    pub BuildNumber: u32,
}
#[cfg(target_arch = "x86")]
impl Default for SP_ALTPLATFORM_INFO_V3 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy)]
pub union SP_ALTPLATFORM_INFO_V3_0 {
    pub Reserved: u16,
    pub Flags: u16,
}
#[cfg(target_arch = "x86")]
impl Default for SP_ALTPLATFORM_INFO_V3_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub struct SP_ALTPLATFORM_INFO_V3 {
    pub cbSize: u32,
    pub Platform: u32,
    pub MajorVersion: u32,
    pub MinorVersion: u32,
    pub ProcessorArchitecture: u16,
    pub Anonymous: SP_ALTPLATFORM_INFO_V3_0,
    pub FirstValidatedMajorVersion: u32,
    pub FirstValidatedMinorVersion: u32,
    pub ProductType: u8,
    pub SuiteMask: u16,
    pub BuildNumber: u32,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for SP_ALTPLATFORM_INFO_V3 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub union SP_ALTPLATFORM_INFO_V3_0 {
    pub Reserved: u16,
    pub Flags: u16,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for SP_ALTPLATFORM_INFO_V3_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const SP_BACKUP_BACKUPPASS: u32 = 1u32;
pub const SP_BACKUP_BOOTFILE: u32 = 8u32;
pub const SP_BACKUP_DEMANDPASS: u32 = 2u32;
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy)]
pub struct SP_BACKUP_QUEUE_PARAMS_V1_A {
    pub cbSize: u32,
    pub FullInfPath: [i8; 260],
    pub FilenameOffset: i32,
}
#[cfg(target_arch = "x86")]
impl Default for SP_BACKUP_QUEUE_PARAMS_V1_A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SP_BACKUP_QUEUE_PARAMS_V1_A {
    pub cbSize: u32,
    pub FullInfPath: [i8; 260],
    pub FilenameOffset: i32,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for SP_BACKUP_QUEUE_PARAMS_V1_A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy)]
pub struct SP_BACKUP_QUEUE_PARAMS_V1_W {
    pub cbSize: u32,
    pub FullInfPath: [u16; 260],
    pub FilenameOffset: i32,
}
#[cfg(target_arch = "x86")]
impl Default for SP_BACKUP_QUEUE_PARAMS_V1_W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SP_BACKUP_QUEUE_PARAMS_V1_W {
    pub cbSize: u32,
    pub FullInfPath: [u16; 260],
    pub FilenameOffset: i32,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for SP_BACKUP_QUEUE_PARAMS_V1_W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy)]
pub struct SP_BACKUP_QUEUE_PARAMS_V2_A {
    pub cbSize: u32,
    pub FullInfPath: [i8; 260],
    pub FilenameOffset: i32,
    pub ReinstallInstance: [i8; 260],
}
#[cfg(target_arch = "x86")]
impl Default for SP_BACKUP_QUEUE_PARAMS_V2_A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SP_BACKUP_QUEUE_PARAMS_V2_A {
    pub cbSize: u32,
    pub FullInfPath: [i8; 260],
    pub FilenameOffset: i32,
    pub ReinstallInstance: [i8; 260],
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for SP_BACKUP_QUEUE_PARAMS_V2_A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy)]
pub struct SP_BACKUP_QUEUE_PARAMS_V2_W {
    pub cbSize: u32,
    pub FullInfPath: [u16; 260],
    pub FilenameOffset: i32,
    pub ReinstallInstance: [u16; 260],
}
#[cfg(target_arch = "x86")]
impl Default for SP_BACKUP_QUEUE_PARAMS_V2_W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SP_BACKUP_QUEUE_PARAMS_V2_W {
    pub cbSize: u32,
    pub FullInfPath: [u16; 260],
    pub FilenameOffset: i32,
    pub ReinstallInstance: [u16; 260],
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for SP_BACKUP_QUEUE_PARAMS_V2_W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const SP_BACKUP_SPECIAL: u32 = 4u32;
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_UI_Controls")]
#[derive(Clone, Copy, Default)]
pub struct SP_CLASSIMAGELIST_DATA {
    pub cbSize: u32,
    pub ImageList: super::super::UI::Controls::HIMAGELIST,
    pub Reserved: usize,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "Win32_UI_Controls")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SP_CLASSIMAGELIST_DATA {
    pub cbSize: u32,
    pub ImageList: super::super::UI::Controls::HIMAGELIST,
    pub Reserved: usize,
}
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy, Default)]
pub struct SP_CLASSINSTALL_HEADER {
    pub cbSize: u32,
    pub InstallFunction: DI_FUNCTION,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SP_CLASSINSTALL_HEADER {
    pub cbSize: u32,
    pub InstallFunction: DI_FUNCTION,
}
pub const SP_COPY_ALREADYDECOMP: SP_COPY_STYLE = SP_COPY_STYLE(4194304u32);
pub const SP_COPY_DELETESOURCE: SP_COPY_STYLE = SP_COPY_STYLE(1u32);
pub const SP_COPY_FORCE_IN_USE: SP_COPY_STYLE = SP_COPY_STYLE(512u32);
pub const SP_COPY_FORCE_NEWER: SP_COPY_STYLE = SP_COPY_STYLE(8192u32);
pub const SP_COPY_FORCE_NOOVERWRITE: SP_COPY_STYLE = SP_COPY_STYLE(4096u32);
pub const SP_COPY_HARDLINK: SP_COPY_STYLE = SP_COPY_STYLE(268435456u32);
pub const SP_COPY_INBOX_INF: SP_COPY_STYLE = SP_COPY_STYLE(134217728u32);
pub const SP_COPY_IN_USE_NEEDS_REBOOT: SP_COPY_STYLE = SP_COPY_STYLE(256u32);
pub const SP_COPY_IN_USE_TRY_RENAME: SP_COPY_STYLE = SP_COPY_STYLE(67108864u32);
pub const SP_COPY_LANGUAGEAWARE: SP_COPY_STYLE = SP_COPY_STYLE(32u32);
pub const SP_COPY_NEWER: SP_COPY_STYLE = SP_COPY_STYLE(4u32);
pub const SP_COPY_NEWER_ONLY: SP_COPY_STYLE = SP_COPY_STYLE(65536u32);
pub const SP_COPY_NEWER_OR_SAME: SP_COPY_STYLE = SP_COPY_STYLE(4u32);
pub const SP_COPY_NOBROWSE: SP_COPY_STYLE = SP_COPY_STYLE(32768u32);
pub const SP_COPY_NODECOMP: SP_COPY_STYLE = SP_COPY_STYLE(16u32);
pub const SP_COPY_NOOVERWRITE: SP_COPY_STYLE = SP_COPY_STYLE(8u32);
pub const SP_COPY_NOPRUNE: SP_COPY_STYLE = SP_COPY_STYLE(1048576u32);
pub const SP_COPY_NOSKIP: SP_COPY_STYLE = SP_COPY_STYLE(1024u32);
pub const SP_COPY_OEMINF_CATALOG_ONLY: SP_COPY_STYLE = SP_COPY_STYLE(262144u32);
pub const SP_COPY_OEM_F6_INF: SP_COPY_STYLE = SP_COPY_STYLE(2097152u32);
pub const SP_COPY_PNPLOCKED: SP_COPY_STYLE = SP_COPY_STYLE(33554432u32);
pub const SP_COPY_REPLACEONLY: SP_COPY_STYLE = SP_COPY_STYLE(2u32);
pub const SP_COPY_REPLACE_BOOT_FILE: SP_COPY_STYLE = SP_COPY_STYLE(524288u32);
pub const SP_COPY_RESERVED: SP_COPY_STYLE = SP_COPY_STYLE(131072u32);
pub const SP_COPY_SOURCEPATH_ABSOLUTE: SP_COPY_STYLE = SP_COPY_STYLE(128u32);
pub const SP_COPY_SOURCE_ABSOLUTE: SP_COPY_STYLE = SP_COPY_STYLE(64u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SP_COPY_STYLE(pub u32);
impl SP_COPY_STYLE {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for SP_COPY_STYLE {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for SP_COPY_STYLE {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for SP_COPY_STYLE {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for SP_COPY_STYLE {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for SP_COPY_STYLE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const SP_COPY_WARNIFSKIP: SP_COPY_STYLE = SP_COPY_STYLE(16384u32);
pub const SP_COPY_WINDOWS_SIGNED: SP_COPY_STYLE = SP_COPY_STYLE(16777216u32);
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy)]
pub struct SP_DETECTDEVICE_PARAMS {
    pub ClassInstallHeader: SP_CLASSINSTALL_HEADER,
    pub DetectProgressNotify: PDETECT_PROGRESS_NOTIFY,
    pub ProgressNotifyParam: *mut core::ffi::c_void,
}
#[cfg(target_arch = "x86")]
impl Default for SP_DETECTDEVICE_PARAMS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub struct SP_DETECTDEVICE_PARAMS {
    pub ClassInstallHeader: SP_CLASSINSTALL_HEADER,
    pub DetectProgressNotify: PDETECT_PROGRESS_NOTIFY,
    pub ProgressNotifyParam: *mut core::ffi::c_void,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for SP_DETECTDEVICE_PARAMS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy, Default)]
pub struct SP_DEVICE_INTERFACE_DATA {
    pub cbSize: u32,
    pub InterfaceClassGuid: windows_core::GUID,
    pub Flags: u32,
    pub Reserved: usize,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SP_DEVICE_INTERFACE_DATA {
    pub cbSize: u32,
    pub InterfaceClassGuid: windows_core::GUID,
    pub Flags: u32,
    pub Reserved: usize,
}
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy)]
pub struct SP_DEVICE_INTERFACE_DETAIL_DATA_A {
    pub cbSize: u32,
    pub DevicePath: [i8; 1],
}
#[cfg(target_arch = "x86")]
impl Default for SP_DEVICE_INTERFACE_DETAIL_DATA_A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SP_DEVICE_INTERFACE_DETAIL_DATA_A {
    pub cbSize: u32,
    pub DevicePath: [i8; 1],
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for SP_DEVICE_INTERFACE_DETAIL_DATA_A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy)]
pub struct SP_DEVICE_INTERFACE_DETAIL_DATA_W {
    pub cbSize: u32,
    pub DevicePath: [u16; 1],
}
#[cfg(target_arch = "x86")]
impl Default for SP_DEVICE_INTERFACE_DETAIL_DATA_W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SP_DEVICE_INTERFACE_DETAIL_DATA_W {
    pub cbSize: u32,
    pub DevicePath: [u16; 1],
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for SP_DEVICE_INTERFACE_DETAIL_DATA_W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy, Default)]
pub struct SP_DEVINFO_DATA {
    pub cbSize: u32,
    pub ClassGuid: windows_core::GUID,
    pub DevInst: u32,
    pub Reserved: usize,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SP_DEVINFO_DATA {
    pub cbSize: u32,
    pub ClassGuid: windows_core::GUID,
    pub DevInst: u32,
    pub Reserved: usize,
}
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy)]
pub struct SP_DEVINFO_LIST_DETAIL_DATA_A {
    pub cbSize: u32,
    pub ClassGuid: windows_core::GUID,
    pub RemoteMachineHandle: super::super::Foundation::HANDLE,
    pub RemoteMachineName: [i8; 263],
}
#[cfg(target_arch = "x86")]
impl Default for SP_DEVINFO_LIST_DETAIL_DATA_A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SP_DEVINFO_LIST_DETAIL_DATA_A {
    pub cbSize: u32,
    pub ClassGuid: windows_core::GUID,
    pub RemoteMachineHandle: super::super::Foundation::HANDLE,
    pub RemoteMachineName: [i8; 263],
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for SP_DEVINFO_LIST_DETAIL_DATA_A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy)]
pub struct SP_DEVINFO_LIST_DETAIL_DATA_W {
    pub cbSize: u32,
    pub ClassGuid: windows_core::GUID,
    pub RemoteMachineHandle: super::super::Foundation::HANDLE,
    pub RemoteMachineName: [u16; 263],
}
#[cfg(target_arch = "x86")]
impl Default for SP_DEVINFO_LIST_DETAIL_DATA_W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SP_DEVINFO_LIST_DETAIL_DATA_W {
    pub cbSize: u32,
    pub ClassGuid: windows_core::GUID,
    pub RemoteMachineHandle: super::super::Foundation::HANDLE,
    pub RemoteMachineName: [u16; 263],
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for SP_DEVINFO_LIST_DETAIL_DATA_W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy)]
pub struct SP_DEVINSTALL_PARAMS_A {
    pub cbSize: u32,
    pub Flags: SETUP_DI_DEVICE_INSTALL_FLAGS,
    pub FlagsEx: SETUP_DI_DEVICE_INSTALL_FLAGS_EX,
    pub hwndParent: super::super::Foundation::HWND,
    pub InstallMsgHandler: PSP_FILE_CALLBACK_A,
    pub InstallMsgHandlerContext: *mut core::ffi::c_void,
    pub FileQueue: *mut core::ffi::c_void,
    pub ClassInstallReserved: usize,
    pub Reserved: u32,
    pub DriverPath: [i8; 260],
}
#[cfg(target_arch = "x86")]
impl Default for SP_DEVINSTALL_PARAMS_A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug)]
pub struct SP_DEVINSTALL_PARAMS_A {
    pub cbSize: u32,
    pub Flags: SETUP_DI_DEVICE_INSTALL_FLAGS,
    pub FlagsEx: SETUP_DI_DEVICE_INSTALL_FLAGS_EX,
    pub hwndParent: super::super::Foundation::HWND,
    pub InstallMsgHandler: PSP_FILE_CALLBACK_A,
    pub InstallMsgHandlerContext: *mut core::ffi::c_void,
    pub FileQueue: *mut core::ffi::c_void,
    pub ClassInstallReserved: usize,
    pub Reserved: u32,
    pub DriverPath: [i8; 260],
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for SP_DEVINSTALL_PARAMS_A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy)]
pub struct SP_DEVINSTALL_PARAMS_W {
    pub cbSize: u32,
    pub Flags: SETUP_DI_DEVICE_INSTALL_FLAGS,
    pub FlagsEx: SETUP_DI_DEVICE_INSTALL_FLAGS_EX,
    pub hwndParent: super::super::Foundation::HWND,
    pub InstallMsgHandler: PSP_FILE_CALLBACK_W,
    pub InstallMsgHandlerContext: *mut core::ffi::c_void,
    pub FileQueue: *mut core::ffi::c_void,
    pub ClassInstallReserved: usize,
    pub Reserved: u32,
    pub DriverPath: [u16; 260],
}
#[cfg(target_arch = "x86")]
impl Default for SP_DEVINSTALL_PARAMS_W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug)]
pub struct SP_DEVINSTALL_PARAMS_W {
    pub cbSize: u32,
    pub Flags: SETUP_DI_DEVICE_INSTALL_FLAGS,
    pub FlagsEx: SETUP_DI_DEVICE_INSTALL_FLAGS_EX,
    pub hwndParent: super::super::Foundation::HWND,
    pub InstallMsgHandler: PSP_FILE_CALLBACK_W,
    pub InstallMsgHandlerContext: *mut core::ffi::c_void,
    pub FileQueue: *mut core::ffi::c_void,
    pub ClassInstallReserved: usize,
    pub Reserved: u32,
    pub DriverPath: [u16; 260],
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for SP_DEVINSTALL_PARAMS_W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy)]
pub struct SP_DRVINFO_DATA_V1_A {
    pub cbSize: u32,
    pub DriverType: u32,
    pub Reserved: usize,
    pub Description: [i8; 256],
    pub MfgName: [i8; 256],
    pub ProviderName: [i8; 256],
}
#[cfg(target_arch = "x86")]
impl Default for SP_DRVINFO_DATA_V1_A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SP_DRVINFO_DATA_V1_A {
    pub cbSize: u32,
    pub DriverType: u32,
    pub Reserved: usize,
    pub Description: [i8; 256],
    pub MfgName: [i8; 256],
    pub ProviderName: [i8; 256],
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for SP_DRVINFO_DATA_V1_A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy)]
pub struct SP_DRVINFO_DATA_V1_W {
    pub cbSize: u32,
    pub DriverType: u32,
    pub Reserved: usize,
    pub Description: [u16; 256],
    pub MfgName: [u16; 256],
    pub ProviderName: [u16; 256],
}
#[cfg(target_arch = "x86")]
impl Default for SP_DRVINFO_DATA_V1_W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SP_DRVINFO_DATA_V1_W {
    pub cbSize: u32,
    pub DriverType: u32,
    pub Reserved: usize,
    pub Description: [u16; 256],
    pub MfgName: [u16; 256],
    pub ProviderName: [u16; 256],
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for SP_DRVINFO_DATA_V1_W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy)]
pub struct SP_DRVINFO_DATA_V2_A {
    pub cbSize: u32,
    pub DriverType: u32,
    pub Reserved: usize,
    pub Description: [i8; 256],
    pub MfgName: [i8; 256],
    pub ProviderName: [i8; 256],
    pub DriverDate: super::super::Foundation::FILETIME,
    pub DriverVersion: u64,
}
#[cfg(target_arch = "x86")]
impl Default for SP_DRVINFO_DATA_V2_A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SP_DRVINFO_DATA_V2_A {
    pub cbSize: u32,
    pub DriverType: u32,
    pub Reserved: usize,
    pub Description: [i8; 256],
    pub MfgName: [i8; 256],
    pub ProviderName: [i8; 256],
    pub DriverDate: super::super::Foundation::FILETIME,
    pub DriverVersion: u64,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for SP_DRVINFO_DATA_V2_A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy)]
pub struct SP_DRVINFO_DATA_V2_W {
    pub cbSize: u32,
    pub DriverType: u32,
    pub Reserved: usize,
    pub Description: [u16; 256],
    pub MfgName: [u16; 256],
    pub ProviderName: [u16; 256],
    pub DriverDate: super::super::Foundation::FILETIME,
    pub DriverVersion: u64,
}
#[cfg(target_arch = "x86")]
impl Default for SP_DRVINFO_DATA_V2_W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SP_DRVINFO_DATA_V2_W {
    pub cbSize: u32,
    pub DriverType: u32,
    pub Reserved: usize,
    pub Description: [u16; 256],
    pub MfgName: [u16; 256],
    pub ProviderName: [u16; 256],
    pub DriverDate: super::super::Foundation::FILETIME,
    pub DriverVersion: u64,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for SP_DRVINFO_DATA_V2_W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy)]
pub struct SP_DRVINFO_DETAIL_DATA_A {
    pub cbSize: u32,
    pub InfDate: super::super::Foundation::FILETIME,
    pub CompatIDsOffset: u32,
    pub CompatIDsLength: u32,
    pub Reserved: usize,
    pub SectionName: [i8; 256],
    pub InfFileName: [i8; 260],
    pub DrvDescription: [i8; 256],
    pub HardwareID: [i8; 1],
}
#[cfg(target_arch = "x86")]
impl Default for SP_DRVINFO_DETAIL_DATA_A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SP_DRVINFO_DETAIL_DATA_A {
    pub cbSize: u32,
    pub InfDate: super::super::Foundation::FILETIME,
    pub CompatIDsOffset: u32,
    pub CompatIDsLength: u32,
    pub Reserved: usize,
    pub SectionName: [i8; 256],
    pub InfFileName: [i8; 260],
    pub DrvDescription: [i8; 256],
    pub HardwareID: [i8; 1],
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for SP_DRVINFO_DETAIL_DATA_A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy)]
pub struct SP_DRVINFO_DETAIL_DATA_W {
    pub cbSize: u32,
    pub InfDate: super::super::Foundation::FILETIME,
    pub CompatIDsOffset: u32,
    pub CompatIDsLength: u32,
    pub Reserved: usize,
    pub SectionName: [u16; 256],
    pub InfFileName: [u16; 260],
    pub DrvDescription: [u16; 256],
    pub HardwareID: [u16; 1],
}
#[cfg(target_arch = "x86")]
impl Default for SP_DRVINFO_DETAIL_DATA_W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SP_DRVINFO_DETAIL_DATA_W {
    pub cbSize: u32,
    pub InfDate: super::super::Foundation::FILETIME,
    pub CompatIDsOffset: u32,
    pub CompatIDsLength: u32,
    pub Reserved: usize,
    pub SectionName: [u16; 256],
    pub InfFileName: [u16; 260],
    pub DrvDescription: [u16; 256],
    pub HardwareID: [u16; 1],
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for SP_DRVINFO_DETAIL_DATA_W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy, Default)]
pub struct SP_DRVINSTALL_PARAMS {
    pub cbSize: u32,
    pub Rank: u32,
    pub Flags: SETUP_DI_DRIVER_INSTALL_FLAGS,
    pub PrivateData: usize,
    pub Reserved: u32,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SP_DRVINSTALL_PARAMS {
    pub cbSize: u32,
    pub Rank: u32,
    pub Flags: SETUP_DI_DRIVER_INSTALL_FLAGS,
    pub PrivateData: usize,
    pub Reserved: u32,
}
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy, Default)]
pub struct SP_ENABLECLASS_PARAMS {
    pub ClassInstallHeader: SP_CLASSINSTALL_HEADER,
    pub ClassGuid: windows_core::GUID,
    pub EnableMessage: u32,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Default)]
pub struct SP_ENABLECLASS_PARAMS {
    pub ClassInstallHeader: SP_CLASSINSTALL_HEADER,
    pub ClassGuid: windows_core::GUID,
    pub EnableMessage: u32,
}
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy)]
pub struct SP_FILE_COPY_PARAMS_A {
    pub cbSize: u32,
    pub QueueHandle: *mut core::ffi::c_void,
    pub SourceRootPath: windows_core::PCSTR,
    pub SourcePath: windows_core::PCSTR,
    pub SourceFilename: windows_core::PCSTR,
    pub SourceDescription: windows_core::PCSTR,
    pub SourceTagfile: windows_core::PCSTR,
    pub TargetDirectory: windows_core::PCSTR,
    pub TargetFilename: windows_core::PCSTR,
    pub CopyStyle: u32,
    pub LayoutInf: *mut core::ffi::c_void,
    pub SecurityDescriptor: windows_core::PCSTR,
}
#[cfg(target_arch = "x86")]
impl Default for SP_FILE_COPY_PARAMS_A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SP_FILE_COPY_PARAMS_A {
    pub cbSize: u32,
    pub QueueHandle: *mut core::ffi::c_void,
    pub SourceRootPath: windows_core::PCSTR,
    pub SourcePath: windows_core::PCSTR,
    pub SourceFilename: windows_core::PCSTR,
    pub SourceDescription: windows_core::PCSTR,
    pub SourceTagfile: windows_core::PCSTR,
    pub TargetDirectory: windows_core::PCSTR,
    pub TargetFilename: windows_core::PCSTR,
    pub CopyStyle: u32,
    pub LayoutInf: *mut core::ffi::c_void,
    pub SecurityDescriptor: windows_core::PCSTR,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for SP_FILE_COPY_PARAMS_A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy)]
pub struct SP_FILE_COPY_PARAMS_W {
    pub cbSize: u32,
    pub QueueHandle: *mut core::ffi::c_void,
    pub SourceRootPath: windows_core::PCWSTR,
    pub SourcePath: windows_core::PCWSTR,
    pub SourceFilename: windows_core::PCWSTR,
    pub SourceDescription: windows_core::PCWSTR,
    pub SourceTagfile: windows_core::PCWSTR,
    pub TargetDirectory: windows_core::PCWSTR,
    pub TargetFilename: windows_core::PCWSTR,
    pub CopyStyle: u32,
    pub LayoutInf: *mut core::ffi::c_void,
    pub SecurityDescriptor: windows_core::PCWSTR,
}
#[cfg(target_arch = "x86")]
impl Default for SP_FILE_COPY_PARAMS_W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SP_FILE_COPY_PARAMS_W {
    pub cbSize: u32,
    pub QueueHandle: *mut core::ffi::c_void,
    pub SourceRootPath: windows_core::PCWSTR,
    pub SourcePath: windows_core::PCWSTR,
    pub SourceFilename: windows_core::PCWSTR,
    pub SourceDescription: windows_core::PCWSTR,
    pub SourceTagfile: windows_core::PCWSTR,
    pub TargetDirectory: windows_core::PCWSTR,
    pub TargetFilename: windows_core::PCWSTR,
    pub CopyStyle: u32,
    pub LayoutInf: *mut core::ffi::c_void,
    pub SecurityDescriptor: windows_core::PCWSTR,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for SP_FILE_COPY_PARAMS_W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const SP_FLAG_CABINETCONTINUATION: u32 = 2048u32;
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy)]
pub struct SP_INF_INFORMATION {
    pub InfStyle: INF_STYLE,
    pub InfCount: u32,
    pub VersionData: [u8; 1],
}
#[cfg(target_arch = "x86")]
impl Default for SP_INF_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SP_INF_INFORMATION {
    pub InfStyle: INF_STYLE,
    pub InfCount: u32,
    pub VersionData: [u8; 1],
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for SP_INF_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy)]
pub struct SP_INF_SIGNER_INFO_V1_A {
    pub cbSize: u32,
    pub CatalogFile: [i8; 260],
    pub DigitalSigner: [i8; 260],
    pub DigitalSignerVersion: [i8; 260],
}
#[cfg(target_arch = "x86")]
impl Default for SP_INF_SIGNER_INFO_V1_A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SP_INF_SIGNER_INFO_V1_A {
    pub cbSize: u32,
    pub CatalogFile: [i8; 260],
    pub DigitalSigner: [i8; 260],
    pub DigitalSignerVersion: [i8; 260],
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for SP_INF_SIGNER_INFO_V1_A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy)]
pub struct SP_INF_SIGNER_INFO_V1_W {
    pub cbSize: u32,
    pub CatalogFile: [u16; 260],
    pub DigitalSigner: [u16; 260],
    pub DigitalSignerVersion: [u16; 260],
}
#[cfg(target_arch = "x86")]
impl Default for SP_INF_SIGNER_INFO_V1_W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SP_INF_SIGNER_INFO_V1_W {
    pub cbSize: u32,
    pub CatalogFile: [u16; 260],
    pub DigitalSigner: [u16; 260],
    pub DigitalSignerVersion: [u16; 260],
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for SP_INF_SIGNER_INFO_V1_W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy)]
pub struct SP_INF_SIGNER_INFO_V2_A {
    pub cbSize: u32,
    pub CatalogFile: [i8; 260],
    pub DigitalSigner: [i8; 260],
    pub DigitalSignerVersion: [i8; 260],
    pub SignerScore: u32,
}
#[cfg(target_arch = "x86")]
impl Default for SP_INF_SIGNER_INFO_V2_A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SP_INF_SIGNER_INFO_V2_A {
    pub cbSize: u32,
    pub CatalogFile: [i8; 260],
    pub DigitalSigner: [i8; 260],
    pub DigitalSignerVersion: [i8; 260],
    pub SignerScore: u32,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for SP_INF_SIGNER_INFO_V2_A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy)]
pub struct SP_INF_SIGNER_INFO_V2_W {
    pub cbSize: u32,
    pub CatalogFile: [u16; 260],
    pub DigitalSigner: [u16; 260],
    pub DigitalSignerVersion: [u16; 260],
    pub SignerScore: u32,
}
#[cfg(target_arch = "x86")]
impl Default for SP_INF_SIGNER_INFO_V2_W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SP_INF_SIGNER_INFO_V2_W {
    pub cbSize: u32,
    pub CatalogFile: [u16; 260],
    pub DigitalSigner: [u16; 260],
    pub DigitalSignerVersion: [u16; 260],
    pub SignerScore: u32,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for SP_INF_SIGNER_INFO_V2_W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_UI_Controls")]
#[derive(Clone, Copy)]
pub struct SP_INSTALLWIZARD_DATA {
    pub ClassInstallHeader: SP_CLASSINSTALL_HEADER,
    pub Flags: u32,
    pub DynamicPages: [super::super::UI::Controls::HPROPSHEETPAGE; 20],
    pub NumDynamicPages: u32,
    pub DynamicPageFlags: u32,
    pub PrivateFlags: u32,
    pub PrivateData: super::super::Foundation::LPARAM,
    pub hwndWizardDlg: super::super::Foundation::HWND,
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_UI_Controls")]
impl Default for SP_INSTALLWIZARD_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "Win32_UI_Controls")]
#[derive(Clone, Copy)]
pub struct SP_INSTALLWIZARD_DATA {
    pub ClassInstallHeader: SP_CLASSINSTALL_HEADER,
    pub Flags: u32,
    pub DynamicPages: [super::super::UI::Controls::HPROPSHEETPAGE; 20],
    pub NumDynamicPages: u32,
    pub DynamicPageFlags: u32,
    pub PrivateFlags: u32,
    pub PrivateData: super::super::Foundation::LPARAM,
    pub hwndWizardDlg: super::super::Foundation::HWND,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "Win32_UI_Controls")]
impl Default for SP_INSTALLWIZARD_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const SP_MAX_MACHINENAME_LENGTH: u32 = 263u32;
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_UI_Controls")]
#[derive(Clone, Copy)]
pub struct SP_NEWDEVICEWIZARD_DATA {
    pub ClassInstallHeader: SP_CLASSINSTALL_HEADER,
    pub Flags: u32,
    pub DynamicPages: [super::super::UI::Controls::HPROPSHEETPAGE; 20],
    pub NumDynamicPages: u32,
    pub hwndWizardDlg: super::super::Foundation::HWND,
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_UI_Controls")]
impl Default for SP_NEWDEVICEWIZARD_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "Win32_UI_Controls")]
#[derive(Clone, Copy)]
pub struct SP_NEWDEVICEWIZARD_DATA {
    pub ClassInstallHeader: SP_CLASSINSTALL_HEADER,
    pub Flags: u32,
    pub DynamicPages: [super::super::UI::Controls::HPROPSHEETPAGE; 20],
    pub NumDynamicPages: u32,
    pub hwndWizardDlg: super::super::Foundation::HWND,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "Win32_UI_Controls")]
impl Default for SP_NEWDEVICEWIZARD_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy)]
pub struct SP_ORIGINAL_FILE_INFO_A {
    pub cbSize: u32,
    pub OriginalInfName: [i8; 260],
    pub OriginalCatalogName: [i8; 260],
}
#[cfg(target_arch = "x86")]
impl Default for SP_ORIGINAL_FILE_INFO_A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SP_ORIGINAL_FILE_INFO_A {
    pub cbSize: u32,
    pub OriginalInfName: [i8; 260],
    pub OriginalCatalogName: [i8; 260],
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for SP_ORIGINAL_FILE_INFO_A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy)]
pub struct SP_ORIGINAL_FILE_INFO_W {
    pub cbSize: u32,
    pub OriginalInfName: [u16; 260],
    pub OriginalCatalogName: [u16; 260],
}
#[cfg(target_arch = "x86")]
impl Default for SP_ORIGINAL_FILE_INFO_W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SP_ORIGINAL_FILE_INFO_W {
    pub cbSize: u32,
    pub OriginalInfName: [u16; 260],
    pub OriginalCatalogName: [u16; 260],
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for SP_ORIGINAL_FILE_INFO_W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SP_POWERMESSAGEWAKE_PARAMS_A {
    pub ClassInstallHeader: SP_CLASSINSTALL_HEADER,
    pub PowerMessageWake: [i8; 512],
}
impl Default for SP_POWERMESSAGEWAKE_PARAMS_A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy)]
pub struct SP_POWERMESSAGEWAKE_PARAMS_W {
    pub ClassInstallHeader: SP_CLASSINSTALL_HEADER,
    pub PowerMessageWake: [u16; 512],
}
#[cfg(target_arch = "x86")]
impl Default for SP_POWERMESSAGEWAKE_PARAMS_W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub struct SP_POWERMESSAGEWAKE_PARAMS_W {
    pub ClassInstallHeader: SP_CLASSINSTALL_HEADER,
    pub PowerMessageWake: [u16; 512],
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for SP_POWERMESSAGEWAKE_PARAMS_W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy, Default)]
pub struct SP_PROPCHANGE_PARAMS {
    pub ClassInstallHeader: SP_CLASSINSTALL_HEADER,
    pub StateChange: SETUP_DI_STATE_CHANGE,
    pub Scope: SETUP_DI_PROPERTY_CHANGE_SCOPE,
    pub HwProfile: u32,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Default)]
pub struct SP_PROPCHANGE_PARAMS {
    pub ClassInstallHeader: SP_CLASSINSTALL_HEADER,
    pub StateChange: SETUP_DI_STATE_CHANGE,
    pub Scope: SETUP_DI_PROPERTY_CHANGE_SCOPE,
    pub HwProfile: u32,
}
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy)]
pub struct SP_PROPSHEETPAGE_REQUEST {
    pub cbSize: u32,
    pub PageRequested: u32,
    pub DeviceInfoSet: HDEVINFO,
    pub DeviceInfoData: *mut SP_DEVINFO_DATA,
}
#[cfg(target_arch = "x86")]
impl Default for SP_PROPSHEETPAGE_REQUEST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SP_PROPSHEETPAGE_REQUEST {
    pub cbSize: u32,
    pub PageRequested: u32,
    pub DeviceInfoSet: HDEVINFO,
    pub DeviceInfoData: *mut SP_DEVINFO_DATA,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for SP_PROPSHEETPAGE_REQUEST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy, Default)]
pub struct SP_REGISTER_CONTROL_STATUSA {
    pub cbSize: u32,
    pub FileName: windows_core::PCSTR,
    pub Win32Error: u32,
    pub FailureCode: u32,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SP_REGISTER_CONTROL_STATUSA {
    pub cbSize: u32,
    pub FileName: windows_core::PCSTR,
    pub Win32Error: u32,
    pub FailureCode: u32,
}
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy, Default)]
pub struct SP_REGISTER_CONTROL_STATUSW {
    pub cbSize: u32,
    pub FileName: windows_core::PCWSTR,
    pub Win32Error: u32,
    pub FailureCode: u32,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SP_REGISTER_CONTROL_STATUSW {
    pub cbSize: u32,
    pub FileName: windows_core::PCWSTR,
    pub Win32Error: u32,
    pub FailureCode: u32,
}
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy, Default)]
pub struct SP_REMOVEDEVICE_PARAMS {
    pub ClassInstallHeader: SP_CLASSINSTALL_HEADER,
    pub Scope: SETUP_DI_REMOVE_DEVICE_SCOPE,
    pub HwProfile: u32,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Default)]
pub struct SP_REMOVEDEVICE_PARAMS {
    pub ClassInstallHeader: SP_CLASSINSTALL_HEADER,
    pub Scope: SETUP_DI_REMOVE_DEVICE_SCOPE,
    pub HwProfile: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SP_SELECTDEVICE_PARAMS_A {
    pub ClassInstallHeader: SP_CLASSINSTALL_HEADER,
    pub Title: [i8; 60],
    pub Instructions: [i8; 256],
    pub ListLabel: [i8; 30],
    pub SubTitle: [i8; 256],
    pub Reserved: [u8; 2],
}
impl Default for SP_SELECTDEVICE_PARAMS_A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy)]
pub struct SP_SELECTDEVICE_PARAMS_W {
    pub ClassInstallHeader: SP_CLASSINSTALL_HEADER,
    pub Title: [u16; 60],
    pub Instructions: [u16; 256],
    pub ListLabel: [u16; 30],
    pub SubTitle: [u16; 256],
}
#[cfg(target_arch = "x86")]
impl Default for SP_SELECTDEVICE_PARAMS_W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub struct SP_SELECTDEVICE_PARAMS_W {
    pub ClassInstallHeader: SP_CLASSINSTALL_HEADER,
    pub Title: [u16; 60],
    pub Instructions: [u16; 256],
    pub ListLabel: [u16; 30],
    pub SubTitle: [u16; 256],
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for SP_SELECTDEVICE_PARAMS_W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SP_TROUBLESHOOTER_PARAMS_A {
    pub ClassInstallHeader: SP_CLASSINSTALL_HEADER,
    pub ChmFile: [i8; 260],
    pub HtmlTroubleShooter: [i8; 260],
}
impl Default for SP_TROUBLESHOOTER_PARAMS_A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy)]
pub struct SP_TROUBLESHOOTER_PARAMS_W {
    pub ClassInstallHeader: SP_CLASSINSTALL_HEADER,
    pub ChmFile: [u16; 260],
    pub HtmlTroubleShooter: [u16; 260],
}
#[cfg(target_arch = "x86")]
impl Default for SP_TROUBLESHOOTER_PARAMS_W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub struct SP_TROUBLESHOOTER_PARAMS_W {
    pub ClassInstallHeader: SP_CLASSINSTALL_HEADER,
    pub ChmFile: [u16; 260],
    pub HtmlTroubleShooter: [u16; 260],
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for SP_TROUBLESHOOTER_PARAMS_W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy, Default)]
pub struct SP_UNREMOVEDEVICE_PARAMS {
    pub ClassInstallHeader: SP_CLASSINSTALL_HEADER,
    pub Scope: u32,
    pub HwProfile: u32,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Default)]
pub struct SP_UNREMOVEDEVICE_PARAMS {
    pub ClassInstallHeader: SP_CLASSINSTALL_HEADER,
    pub Scope: u32,
    pub HwProfile: u32,
}
pub const SRCINFO_DESCRIPTION: u32 = 3u32;
pub const SRCINFO_FLAGS: u32 = 4u32;
pub const SRCINFO_PATH: u32 = 1u32;
pub const SRCINFO_TAGFILE: u32 = 2u32;
pub const SRCINFO_TAGFILE2: u32 = 5u32;
pub const SRCLIST_APPEND: u32 = 512u32;
pub const SRCLIST_NOBROWSE: u32 = 2u32;
pub const SRCLIST_NOSTRIPPLATFORM: u32 = 1024u32;
pub const SRCLIST_SUBDIRS: u32 = 256u32;
pub const SRCLIST_SYSIFADMIN: u32 = 64u32;
pub const SRCLIST_SYSTEM: u32 = 16u32;
pub const SRCLIST_TEMPORARY: u32 = 1u32;
pub const SRCLIST_USER: u32 = 32u32;
pub const SRC_FLAGS_CABFILE: u32 = 16u32;
pub const SUOI_FORCEDELETE: u32 = 1u32;
pub const SUOI_INTERNAL1: u32 = 2u32;
pub const SZ_KEY_ADDAUTOLOGGER: windows_core::PCWSTR = windows_core::w!("AddAutoLogger");
pub const SZ_KEY_ADDAUTOLOGGERPROVIDER: windows_core::PCWSTR = windows_core::w!("AddAutoLoggerProvider");
pub const SZ_KEY_ADDCHANNEL: windows_core::PCWSTR = windows_core::w!("AddChannel");
pub const SZ_KEY_ADDEVENTPROVIDER: windows_core::PCWSTR = windows_core::w!("AddEventProvider");
pub const SZ_KEY_ADDFILTER: windows_core::PCWSTR = windows_core::w!("AddFilter");
pub const SZ_KEY_ADDIME: windows_core::PCWSTR = windows_core::w!("AddIme");
pub const SZ_KEY_ADDINTERFACE: windows_core::PCWSTR = windows_core::w!("AddInterface");
pub const SZ_KEY_ADDPOWERSETTING: windows_core::PCWSTR = windows_core::w!("AddPowerSetting");
pub const SZ_KEY_ADDPROP: windows_core::PCWSTR = windows_core::w!("AddProperty");
pub const SZ_KEY_ADDREG: windows_core::PCWSTR = windows_core::w!("AddReg");
pub const SZ_KEY_ADDREGNOCLOBBER: windows_core::PCWSTR = windows_core::w!("AddRegNoClobber");
pub const SZ_KEY_ADDSERVICE: windows_core::PCWSTR = windows_core::w!("AddService");
pub const SZ_KEY_ADDTRIGGER: windows_core::PCWSTR = windows_core::w!("AddTrigger");
pub const SZ_KEY_BITREG: windows_core::PCWSTR = windows_core::w!("BitReg");
pub const SZ_KEY_CLEANONLY: windows_core::PCWSTR = windows_core::w!("CleanOnly");
pub const SZ_KEY_COPYFILES: windows_core::PCWSTR = windows_core::w!("CopyFiles");
pub const SZ_KEY_COPYINF: windows_core::PCWSTR = windows_core::w!("CopyINF");
pub const SZ_KEY_DEFAULTOPTION: windows_core::PCWSTR = windows_core::w!("DefaultOption");
pub const SZ_KEY_DEFDESTDIR: windows_core::PCWSTR = windows_core::w!("DefaultDestDir");
pub const SZ_KEY_DELFILES: windows_core::PCWSTR = windows_core::w!("DelFiles");
pub const SZ_KEY_DELIME: windows_core::PCWSTR = windows_core::w!("DelIme");
pub const SZ_KEY_DELPROP: windows_core::PCWSTR = windows_core::w!("DelProperty");
pub const SZ_KEY_DELREG: windows_core::PCWSTR = windows_core::w!("DelReg");
pub const SZ_KEY_DELSERVICE: windows_core::PCWSTR = windows_core::w!("DelService");
pub const SZ_KEY_DESTDIRS: windows_core::PCWSTR = windows_core::w!("DestinationDirs");
pub const SZ_KEY_EXCLUDEID: windows_core::PCWSTR = windows_core::w!("ExcludeId");
pub const SZ_KEY_FAILUREACTIONS: windows_core::PCWSTR = windows_core::w!("FailureActions");
pub const SZ_KEY_FEATURESCORE: windows_core::PCWSTR = windows_core::w!("FeatureScore");
pub const SZ_KEY_FILTERLEVEL: windows_core::PCWSTR = windows_core::w!("FilterLevel");
pub const SZ_KEY_FILTERPOSITION: windows_core::PCWSTR = windows_core::w!("FilterPosition");
pub const SZ_KEY_HARDWARE: windows_core::PCWSTR = windows_core::w!("Hardware");
pub const SZ_KEY_IMPORTCHANNEL: windows_core::PCWSTR = windows_core::w!("ImportChannel");
pub const SZ_KEY_INI2REG: windows_core::PCWSTR = windows_core::w!("Ini2Reg");
pub const SZ_KEY_LAYOUT_FILE: windows_core::PCWSTR = windows_core::w!("LayoutFile");
pub const SZ_KEY_LDIDOEM: windows_core::PCWSTR = windows_core::w!("LdidOEM");
pub const SZ_KEY_LFN_SECTION: windows_core::PCWSTR = windows_core::w!("VarLDID.LFN");
pub const SZ_KEY_LISTOPTIONS: windows_core::PCWSTR = windows_core::w!("ListOptions");
pub const SZ_KEY_LOGCONFIG: windows_core::PCWSTR = windows_core::w!("LogConfig");
pub const SZ_KEY_MODULES: windows_core::PCWSTR = windows_core::w!("Modules");
pub const SZ_KEY_OPTIONDESC: windows_core::PCWSTR = windows_core::w!("OptionDesc");
pub const SZ_KEY_PHASE1: windows_core::PCWSTR = windows_core::w!("Phase1");
pub const SZ_KEY_PROFILEITEMS: windows_core::PCWSTR = windows_core::w!("ProfileItems");
pub const SZ_KEY_REGSVR: windows_core::PCWSTR = windows_core::w!("RegisterDlls");
pub const SZ_KEY_RENFILES: windows_core::PCWSTR = windows_core::w!("RenFiles");
pub const SZ_KEY_SFN_SECTION: windows_core::PCWSTR = windows_core::w!("VarLDID.SFN");
pub const SZ_KEY_SRCDISKFILES: windows_core::PCWSTR = windows_core::w!("SourceDisksFiles");
pub const SZ_KEY_SRCDISKNAMES: windows_core::PCWSTR = windows_core::w!("SourceDisksNames");
pub const SZ_KEY_STRINGS: windows_core::PCWSTR = windows_core::w!("Strings");
pub const SZ_KEY_UNREGSVR: windows_core::PCWSTR = windows_core::w!("UnregisterDlls");
pub const SZ_KEY_UPDATEAUTOLOGGER: windows_core::PCWSTR = windows_core::w!("UpdateAutoLogger");
pub const SZ_KEY_UPDATEINIFIELDS: windows_core::PCWSTR = windows_core::w!("UpdateIniFields");
pub const SZ_KEY_UPDATEINIS: windows_core::PCWSTR = windows_core::w!("UpdateInis");
pub const SZ_KEY_UPGRADEONLY: windows_core::PCWSTR = windows_core::w!("UpgradeOnly");
pub const SetupFileLogChecksum: SetupFileLogInfo = SetupFileLogInfo(1i32);
pub const SetupFileLogDiskDescription: SetupFileLogInfo = SetupFileLogInfo(3i32);
pub const SetupFileLogDiskTagfile: SetupFileLogInfo = SetupFileLogInfo(2i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SetupFileLogInfo(pub i32);
pub const SetupFileLogMax: SetupFileLogInfo = SetupFileLogInfo(5i32);
pub const SetupFileLogOtherInfo: SetupFileLogInfo = SetupFileLogInfo(4i32);
pub const SetupFileLogSourceFilename: SetupFileLogInfo = SetupFileLogInfo(0i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct UPDATEDRIVERFORPLUGANDPLAYDEVICES_FLAGS(pub u32);
impl UPDATEDRIVERFORPLUGANDPLAYDEVICES_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for UPDATEDRIVERFORPLUGANDPLAYDEVICES_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for UPDATEDRIVERFORPLUGANDPLAYDEVICES_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for UPDATEDRIVERFORPLUGANDPLAYDEVICES_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for UPDATEDRIVERFORPLUGANDPLAYDEVICES_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for UPDATEDRIVERFORPLUGANDPLAYDEVICES_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const fDD_BYTE: DD_FLAGS = DD_FLAGS(0u32);
pub const fDD_BYTE_AND_WORD: DD_FLAGS = DD_FLAGS(3u32);
pub const fDD_BusMaster: DD_FLAGS = DD_FLAGS(4u32);
pub const fDD_DWORD: DD_FLAGS = DD_FLAGS(2u32);
pub const fDD_NoBusMaster: DD_FLAGS = DD_FLAGS(0u32);
pub const fDD_TypeA: DD_FLAGS = DD_FLAGS(8u32);
pub const fDD_TypeB: DD_FLAGS = DD_FLAGS(16u32);
pub const fDD_TypeF: DD_FLAGS = DD_FLAGS(24u32);
pub const fDD_TypeStandard: DD_FLAGS = DD_FLAGS(0u32);
pub const fDD_WORD: DD_FLAGS = DD_FLAGS(1u32);
pub const fIOD_10_BIT_DECODE: IOD_DESFLAGS = IOD_DESFLAGS(4u32);
pub const fIOD_12_BIT_DECODE: IOD_DESFLAGS = IOD_DESFLAGS(8u32);
pub const fIOD_16_BIT_DECODE: IOD_DESFLAGS = IOD_DESFLAGS(16u32);
pub const fIOD_DECODE: IOD_DESFLAGS = IOD_DESFLAGS(252u32);
pub const fIOD_IO: IOD_DESFLAGS = IOD_DESFLAGS(1u32);
pub const fIOD_Memory: IOD_DESFLAGS = IOD_DESFLAGS(0u32);
pub const fIOD_PASSIVE_DECODE: IOD_DESFLAGS = IOD_DESFLAGS(64u32);
pub const fIOD_PORT_BAR: IOD_DESFLAGS = IOD_DESFLAGS(256u32);
pub const fIOD_POSITIVE_DECODE: IOD_DESFLAGS = IOD_DESFLAGS(32u32);
pub const fIOD_PortType: IOD_DESFLAGS = IOD_DESFLAGS(1u32);
pub const fIOD_WINDOW_DECODE: IOD_DESFLAGS = IOD_DESFLAGS(128u32);
pub const fIRQD_Edge: IRQD_FLAGS = IRQD_FLAGS(2u32);
pub const fIRQD_Exclusive: IRQD_FLAGS = IRQD_FLAGS(0u32);
pub const fIRQD_Level: IRQD_FLAGS = IRQD_FLAGS(0u32);
pub const fIRQD_Level_Bit: IRQD_FLAGS = IRQD_FLAGS(1u32);
pub const fIRQD_Share: IRQD_FLAGS = IRQD_FLAGS(1u32);
pub const fIRQD_Share_Bit: IRQD_FLAGS = IRQD_FLAGS(0u32);
pub const fMD_24: MD_FLAGS = MD_FLAGS(0u32);
pub const fMD_32: MD_FLAGS = MD_FLAGS(2u32);
pub const fMD_32_24: MD_FLAGS = MD_FLAGS(2u32);
pub const fMD_Cacheable: MD_FLAGS = MD_FLAGS(32u32);
pub const fMD_CombinedWrite: MD_FLAGS = MD_FLAGS(16u32);
pub const fMD_CombinedWriteAllowed: MD_FLAGS = MD_FLAGS(16u32);
pub const fMD_CombinedWriteDisallowed: MD_FLAGS = MD_FLAGS(0u32);
pub const fMD_MEMORY_BAR: MD_FLAGS = MD_FLAGS(128u32);
pub const fMD_MemoryType: MD_FLAGS = MD_FLAGS(1u32);
pub const fMD_NonCacheable: MD_FLAGS = MD_FLAGS(0u32);
pub const fMD_Pref: MD_FLAGS = MD_FLAGS(4u32);
pub const fMD_PrefetchAllowed: MD_FLAGS = MD_FLAGS(4u32);
pub const fMD_PrefetchDisallowed: MD_FLAGS = MD_FLAGS(0u32);
pub const fMD_Prefetchable: MD_FLAGS = MD_FLAGS(4u32);
pub const fMD_RAM: MD_FLAGS = MD_FLAGS(1u32);
pub const fMD_ROM: MD_FLAGS = MD_FLAGS(0u32);
pub const fMD_ReadAllowed: MD_FLAGS = MD_FLAGS(0u32);
pub const fMD_ReadDisallowed: MD_FLAGS = MD_FLAGS(8u32);
pub const fMD_Readable: MD_FLAGS = MD_FLAGS(8u32);
pub const fMD_WINDOW_DECODE: MD_FLAGS = MD_FLAGS(64u32);
pub const fPCD_ATTRIBUTES_PER_WINDOW: PCD_FLAGS = PCD_FLAGS(32768u32);
pub const fPCD_IO1_16: PCD_FLAGS = PCD_FLAGS(65536u32);
pub const fPCD_IO1_SRC_16: PCD_FLAGS = PCD_FLAGS(262144u32);
pub const fPCD_IO1_WS_16: PCD_FLAGS = PCD_FLAGS(524288u32);
pub const fPCD_IO1_ZW_8: PCD_FLAGS = PCD_FLAGS(131072u32);
pub const fPCD_IO2_16: PCD_FLAGS = PCD_FLAGS(1048576u32);
pub const fPCD_IO2_SRC_16: PCD_FLAGS = PCD_FLAGS(4194304u32);
pub const fPCD_IO2_WS_16: PCD_FLAGS = PCD_FLAGS(8388608u32);
pub const fPCD_IO2_ZW_8: PCD_FLAGS = PCD_FLAGS(2097152u32);
pub const fPCD_IO_16: PCD_FLAGS = PCD_FLAGS(1u32);
pub const fPCD_IO_8: PCD_FLAGS = PCD_FLAGS(0u32);
pub const fPCD_IO_SRC_16: PCD_FLAGS = PCD_FLAGS(32u32);
pub const fPCD_IO_WS_16: PCD_FLAGS = PCD_FLAGS(64u32);
pub const fPCD_IO_ZW_8: PCD_FLAGS = PCD_FLAGS(16u32);
pub const fPCD_MEM1_16: PCD_FLAGS = PCD_FLAGS(67108864u32);
pub const fPCD_MEM1_A: PCD_FLAGS = PCD_FLAGS(4u32);
pub const fPCD_MEM1_WS_ONE: PCD_FLAGS = PCD_FLAGS(16777216u32);
pub const fPCD_MEM1_WS_THREE: PCD_FLAGS = PCD_FLAGS(50331648u32);
pub const fPCD_MEM1_WS_TWO: PCD_FLAGS = PCD_FLAGS(33554432u32);
pub const fPCD_MEM2_16: PCD_FLAGS = PCD_FLAGS(1073741824u32);
pub const fPCD_MEM2_A: PCD_FLAGS = PCD_FLAGS(8u32);
pub const fPCD_MEM2_WS_ONE: PCD_FLAGS = PCD_FLAGS(268435456u32);
pub const fPCD_MEM2_WS_THREE: PCD_FLAGS = PCD_FLAGS(805306368u32);
pub const fPCD_MEM2_WS_TWO: PCD_FLAGS = PCD_FLAGS(536870912u32);
pub const fPCD_MEM_16: PCD_FLAGS = PCD_FLAGS(2u32);
pub const fPCD_MEM_8: PCD_FLAGS = PCD_FLAGS(0u32);
pub const fPCD_MEM_A: PCD_FLAGS = PCD_FLAGS(4u32);
pub const fPCD_MEM_WS_ONE: PCD_FLAGS = PCD_FLAGS(256u32);
pub const fPCD_MEM_WS_THREE: PCD_FLAGS = PCD_FLAGS(768u32);
pub const fPCD_MEM_WS_TWO: PCD_FLAGS = PCD_FLAGS(512u32);
pub const fPMF_AUDIO_ENABLE: PMF_FLAGS = PMF_FLAGS(8u32);
pub const mDD_BusMaster: DD_FLAGS = DD_FLAGS(4u32);
pub const mDD_Type: DD_FLAGS = DD_FLAGS(24u32);
pub const mDD_Width: DD_FLAGS = DD_FLAGS(3u32);
pub const mIRQD_Edge_Level: IRQD_FLAGS = IRQD_FLAGS(2u32);
pub const mIRQD_Share: IRQD_FLAGS = IRQD_FLAGS(1u32);
pub const mMD_32_24: MD_FLAGS = MD_FLAGS(2u32);
pub const mMD_Cacheable: MD_FLAGS = MD_FLAGS(32u32);
pub const mMD_CombinedWrite: MD_FLAGS = MD_FLAGS(16u32);
pub const mMD_MemoryType: MD_FLAGS = MD_FLAGS(1u32);
pub const mMD_Prefetchable: MD_FLAGS = MD_FLAGS(4u32);
pub const mMD_Readable: MD_FLAGS = MD_FLAGS(8u32);
pub const mPCD_IO_8_16: PCD_FLAGS = PCD_FLAGS(1u32);
pub const mPCD_MEM1_WS: PCD_FLAGS = PCD_FLAGS(50331648u32);
pub const mPCD_MEM2_WS: PCD_FLAGS = PCD_FLAGS(805306368u32);
pub const mPCD_MEM_8_16: PCD_FLAGS = PCD_FLAGS(2u32);
pub const mPCD_MEM_A_C: PCD_FLAGS = PCD_FLAGS(12u32);
pub const mPCD_MEM_WS: PCD_FLAGS = PCD_FLAGS(768u32);
pub const mPMF_AUDIO_ENABLE: u32 = 8u32;
