#[cfg(feature = "Win32_System_Performance_HardwareCounterProfiling")]
pub mod HardwareCounterProfiling;
#[inline]
pub unsafe fn BackupPerfRegistryToFileW<P0, P1>(szfilename: P0, szcommentstring: P1) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("loadperf.dll" "system" fn BackupPerfRegistryToFileW(szfilename : windows_core::PCWSTR, szcommentstring : windows_core::PCWSTR) -> u32);
    unsafe { BackupPerfRegistryToFileW(szfilename.param().abi(), szcommentstring.param().abi()) }
}
#[inline]
pub unsafe fn InstallPerfDllA<P0, P1>(szcomputername: P0, lpinifile: P1, dwflags: usize) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("loadperf.dll" "system" fn InstallPerfDllA(szcomputername : windows_core::PCSTR, lpinifile : windows_core::PCSTR, dwflags : usize) -> u32);
    unsafe { InstallPerfDllA(szcomputername.param().abi(), lpinifile.param().abi(), dwflags) }
}
#[inline]
pub unsafe fn InstallPerfDllW<P0, P1>(szcomputername: P0, lpinifile: P1, dwflags: usize) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("loadperf.dll" "system" fn InstallPerfDllW(szcomputername : windows_core::PCWSTR, lpinifile : windows_core::PCWSTR, dwflags : usize) -> u32);
    unsafe { InstallPerfDllW(szcomputername.param().abi(), lpinifile.param().abi(), dwflags) }
}
#[inline]
pub unsafe fn LoadPerfCounterTextStringsA<P0>(lpcommandline: P0, bquietmodearg: bool) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("loadperf.dll" "system" fn LoadPerfCounterTextStringsA(lpcommandline : windows_core::PCSTR, bquietmodearg : windows_core::BOOL) -> u32);
    unsafe { LoadPerfCounterTextStringsA(lpcommandline.param().abi(), bquietmodearg.into()) }
}
#[inline]
pub unsafe fn LoadPerfCounterTextStringsW<P0>(lpcommandline: P0, bquietmodearg: bool) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("loadperf.dll" "system" fn LoadPerfCounterTextStringsW(lpcommandline : windows_core::PCWSTR, bquietmodearg : windows_core::BOOL) -> u32);
    unsafe { LoadPerfCounterTextStringsW(lpcommandline.param().abi(), bquietmodearg.into()) }
}
#[inline]
pub unsafe fn PdhAddCounterA<P1>(hquery: PDH_HQUERY, szfullcounterpath: P1, dwuserdata: usize, phcounter: *mut PDH_HCOUNTER) -> u32
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("pdh.dll" "system" fn PdhAddCounterA(hquery : PDH_HQUERY, szfullcounterpath : windows_core::PCSTR, dwuserdata : usize, phcounter : *mut PDH_HCOUNTER) -> u32);
    unsafe { PdhAddCounterA(hquery, szfullcounterpath.param().abi(), dwuserdata, phcounter as _) }
}
#[inline]
pub unsafe fn PdhAddCounterW<P1>(hquery: PDH_HQUERY, szfullcounterpath: P1, dwuserdata: usize, phcounter: *mut PDH_HCOUNTER) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("pdh.dll" "system" fn PdhAddCounterW(hquery : PDH_HQUERY, szfullcounterpath : windows_core::PCWSTR, dwuserdata : usize, phcounter : *mut PDH_HCOUNTER) -> u32);
    unsafe { PdhAddCounterW(hquery, szfullcounterpath.param().abi(), dwuserdata, phcounter as _) }
}
#[inline]
pub unsafe fn PdhAddEnglishCounterA<P1>(hquery: PDH_HQUERY, szfullcounterpath: P1, dwuserdata: usize, phcounter: *mut PDH_HCOUNTER) -> u32
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("pdh.dll" "system" fn PdhAddEnglishCounterA(hquery : PDH_HQUERY, szfullcounterpath : windows_core::PCSTR, dwuserdata : usize, phcounter : *mut PDH_HCOUNTER) -> u32);
    unsafe { PdhAddEnglishCounterA(hquery, szfullcounterpath.param().abi(), dwuserdata, phcounter as _) }
}
#[inline]
pub unsafe fn PdhAddEnglishCounterW<P1>(hquery: PDH_HQUERY, szfullcounterpath: P1, dwuserdata: usize, phcounter: *mut PDH_HCOUNTER) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("pdh.dll" "system" fn PdhAddEnglishCounterW(hquery : PDH_HQUERY, szfullcounterpath : windows_core::PCWSTR, dwuserdata : usize, phcounter : *mut PDH_HCOUNTER) -> u32);
    unsafe { PdhAddEnglishCounterW(hquery, szfullcounterpath.param().abi(), dwuserdata, phcounter as _) }
}
#[inline]
pub unsafe fn PdhBindInputDataSourceA<P1>(phdatasource: *mut PDH_HLOG, logfilenamelist: P1) -> u32
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("pdh.dll" "system" fn PdhBindInputDataSourceA(phdatasource : *mut PDH_HLOG, logfilenamelist : windows_core::PCSTR) -> u32);
    unsafe { PdhBindInputDataSourceA(phdatasource as _, logfilenamelist.param().abi()) }
}
#[inline]
pub unsafe fn PdhBindInputDataSourceW<P1>(phdatasource: *mut PDH_HLOG, logfilenamelist: P1) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("pdh.dll" "system" fn PdhBindInputDataSourceW(phdatasource : *mut PDH_HLOG, logfilenamelist : windows_core::PCWSTR) -> u32);
    unsafe { PdhBindInputDataSourceW(phdatasource as _, logfilenamelist.param().abi()) }
}
#[inline]
pub unsafe fn PdhBrowseCountersA(pbrowsedlgdata: *const PDH_BROWSE_DLG_CONFIG_A) -> u32 {
    windows_core::link!("pdh.dll" "system" fn PdhBrowseCountersA(pbrowsedlgdata : *const PDH_BROWSE_DLG_CONFIG_A) -> u32);
    unsafe { PdhBrowseCountersA(pbrowsedlgdata) }
}
#[inline]
pub unsafe fn PdhBrowseCountersHA(pbrowsedlgdata: *const PDH_BROWSE_DLG_CONFIG_HA) -> u32 {
    windows_core::link!("pdh.dll" "system" fn PdhBrowseCountersHA(pbrowsedlgdata : *const PDH_BROWSE_DLG_CONFIG_HA) -> u32);
    unsafe { PdhBrowseCountersHA(pbrowsedlgdata) }
}
#[inline]
pub unsafe fn PdhBrowseCountersHW(pbrowsedlgdata: *const PDH_BROWSE_DLG_CONFIG_HW) -> u32 {
    windows_core::link!("pdh.dll" "system" fn PdhBrowseCountersHW(pbrowsedlgdata : *const PDH_BROWSE_DLG_CONFIG_HW) -> u32);
    unsafe { PdhBrowseCountersHW(pbrowsedlgdata) }
}
#[inline]
pub unsafe fn PdhBrowseCountersW(pbrowsedlgdata: *const PDH_BROWSE_DLG_CONFIG_W) -> u32 {
    windows_core::link!("pdh.dll" "system" fn PdhBrowseCountersW(pbrowsedlgdata : *const PDH_BROWSE_DLG_CONFIG_W) -> u32);
    unsafe { PdhBrowseCountersW(pbrowsedlgdata) }
}
#[inline]
pub unsafe fn PdhCalculateCounterFromRawValue(hcounter: PDH_HCOUNTER, dwformat: PDH_FMT, rawvalue1: *const PDH_RAW_COUNTER, rawvalue2: *const PDH_RAW_COUNTER, fmtvalue: *mut PDH_FMT_COUNTERVALUE) -> u32 {
    windows_core::link!("pdh.dll" "system" fn PdhCalculateCounterFromRawValue(hcounter : PDH_HCOUNTER, dwformat : PDH_FMT, rawvalue1 : *const PDH_RAW_COUNTER, rawvalue2 : *const PDH_RAW_COUNTER, fmtvalue : *mut PDH_FMT_COUNTERVALUE) -> u32);
    unsafe { PdhCalculateCounterFromRawValue(hcounter, dwformat, rawvalue1, rawvalue2, fmtvalue as _) }
}
#[inline]
pub unsafe fn PdhCloseLog(hlog: PDH_HLOG, dwflags: u32) -> u32 {
    windows_core::link!("pdh.dll" "system" fn PdhCloseLog(hlog : PDH_HLOG, dwflags : u32) -> u32);
    unsafe { PdhCloseLog(hlog, dwflags) }
}
#[inline]
pub unsafe fn PdhCloseQuery(hquery: PDH_HQUERY) -> u32 {
    windows_core::link!("pdh.dll" "system" fn PdhCloseQuery(hquery : PDH_HQUERY) -> u32);
    unsafe { PdhCloseQuery(hquery as _) }
}
#[inline]
pub unsafe fn PdhCollectQueryData(hquery: PDH_HQUERY) -> u32 {
    windows_core::link!("pdh.dll" "system" fn PdhCollectQueryData(hquery : PDH_HQUERY) -> u32);
    unsafe { PdhCollectQueryData(hquery as _) }
}
#[inline]
pub unsafe fn PdhCollectQueryDataEx(hquery: PDH_HQUERY, dwintervaltime: u32, hnewdataevent: super::super::Foundation::HANDLE) -> u32 {
    windows_core::link!("pdh.dll" "system" fn PdhCollectQueryDataEx(hquery : PDH_HQUERY, dwintervaltime : u32, hnewdataevent : super::super::Foundation:: HANDLE) -> u32);
    unsafe { PdhCollectQueryDataEx(hquery, dwintervaltime, hnewdataevent) }
}
#[inline]
pub unsafe fn PdhCollectQueryDataWithTime(hquery: PDH_HQUERY, plltimestamp: *mut i64) -> u32 {
    windows_core::link!("pdh.dll" "system" fn PdhCollectQueryDataWithTime(hquery : PDH_HQUERY, plltimestamp : *mut i64) -> u32);
    unsafe { PdhCollectQueryDataWithTime(hquery as _, plltimestamp as _) }
}
#[inline]
pub unsafe fn PdhComputeCounterStatistics(hcounter: PDH_HCOUNTER, dwformat: PDH_FMT, dwfirstentry: u32, dwnumentries: u32, lprawvaluearray: *const PDH_RAW_COUNTER, data: *mut PDH_STATISTICS) -> u32 {
    windows_core::link!("pdh.dll" "system" fn PdhComputeCounterStatistics(hcounter : PDH_HCOUNTER, dwformat : PDH_FMT, dwfirstentry : u32, dwnumentries : u32, lprawvaluearray : *const PDH_RAW_COUNTER, data : *mut PDH_STATISTICS) -> u32);
    unsafe { PdhComputeCounterStatistics(hcounter, dwformat, dwfirstentry, dwnumentries, lprawvaluearray, data as _) }
}
#[inline]
pub unsafe fn PdhConnectMachineA<P0>(szmachinename: P0) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("pdh.dll" "system" fn PdhConnectMachineA(szmachinename : windows_core::PCSTR) -> u32);
    unsafe { PdhConnectMachineA(szmachinename.param().abi()) }
}
#[inline]
pub unsafe fn PdhConnectMachineW<P0>(szmachinename: P0) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("pdh.dll" "system" fn PdhConnectMachineW(szmachinename : windows_core::PCWSTR) -> u32);
    unsafe { PdhConnectMachineW(szmachinename.param().abi()) }
}
#[inline]
pub unsafe fn PdhCreateSQLTablesA<P0>(szdatasource: P0) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("pdh.dll" "system" fn PdhCreateSQLTablesA(szdatasource : windows_core::PCSTR) -> u32);
    unsafe { PdhCreateSQLTablesA(szdatasource.param().abi()) }
}
#[inline]
pub unsafe fn PdhCreateSQLTablesW<P0>(szdatasource: P0) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("pdh.dll" "system" fn PdhCreateSQLTablesW(szdatasource : windows_core::PCWSTR) -> u32);
    unsafe { PdhCreateSQLTablesW(szdatasource.param().abi()) }
}
#[inline]
pub unsafe fn PdhEnumLogSetNamesA<P0>(szdatasource: P0, mszdatasetnamelist: Option<windows_core::PSTR>, pcchbufferlength: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("pdh.dll" "system" fn PdhEnumLogSetNamesA(szdatasource : windows_core::PCSTR, mszdatasetnamelist : windows_core::PSTR, pcchbufferlength : *mut u32) -> u32);
    unsafe { PdhEnumLogSetNamesA(szdatasource.param().abi(), mszdatasetnamelist.unwrap_or(core::mem::zeroed()) as _, pcchbufferlength as _) }
}
#[inline]
pub unsafe fn PdhEnumLogSetNamesW<P0>(szdatasource: P0, mszdatasetnamelist: Option<windows_core::PWSTR>, pcchbufferlength: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("pdh.dll" "system" fn PdhEnumLogSetNamesW(szdatasource : windows_core::PCWSTR, mszdatasetnamelist : windows_core::PWSTR, pcchbufferlength : *mut u32) -> u32);
    unsafe { PdhEnumLogSetNamesW(szdatasource.param().abi(), mszdatasetnamelist.unwrap_or(core::mem::zeroed()) as _, pcchbufferlength as _) }
}
#[inline]
pub unsafe fn PdhEnumMachinesA<P0>(szdatasource: P0, mszmachinelist: Option<windows_core::PSTR>, pcchbuffersize: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("pdh.dll" "system" fn PdhEnumMachinesA(szdatasource : windows_core::PCSTR, mszmachinelist : windows_core::PSTR, pcchbuffersize : *mut u32) -> u32);
    unsafe { PdhEnumMachinesA(szdatasource.param().abi(), mszmachinelist.unwrap_or(core::mem::zeroed()) as _, pcchbuffersize as _) }
}
#[inline]
pub unsafe fn PdhEnumMachinesHA(hdatasource: Option<PDH_HLOG>, mszmachinelist: Option<windows_core::PSTR>, pcchbuffersize: *mut u32) -> u32 {
    windows_core::link!("pdh.dll" "system" fn PdhEnumMachinesHA(hdatasource : PDH_HLOG, mszmachinelist : windows_core::PSTR, pcchbuffersize : *mut u32) -> u32);
    unsafe { PdhEnumMachinesHA(hdatasource.unwrap_or(core::mem::zeroed()) as _, mszmachinelist.unwrap_or(core::mem::zeroed()) as _, pcchbuffersize as _) }
}
#[inline]
pub unsafe fn PdhEnumMachinesHW(hdatasource: Option<PDH_HLOG>, mszmachinelist: Option<windows_core::PWSTR>, pcchbuffersize: *mut u32) -> u32 {
    windows_core::link!("pdh.dll" "system" fn PdhEnumMachinesHW(hdatasource : PDH_HLOG, mszmachinelist : windows_core::PWSTR, pcchbuffersize : *mut u32) -> u32);
    unsafe { PdhEnumMachinesHW(hdatasource.unwrap_or(core::mem::zeroed()) as _, mszmachinelist.unwrap_or(core::mem::zeroed()) as _, pcchbuffersize as _) }
}
#[inline]
pub unsafe fn PdhEnumMachinesW<P0>(szdatasource: P0, mszmachinelist: Option<windows_core::PWSTR>, pcchbuffersize: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("pdh.dll" "system" fn PdhEnumMachinesW(szdatasource : windows_core::PCWSTR, mszmachinelist : windows_core::PWSTR, pcchbuffersize : *mut u32) -> u32);
    unsafe { PdhEnumMachinesW(szdatasource.param().abi(), mszmachinelist.unwrap_or(core::mem::zeroed()) as _, pcchbuffersize as _) }
}
#[inline]
pub unsafe fn PdhEnumObjectItemsA<P0, P1, P2>(szdatasource: P0, szmachinename: P1, szobjectname: P2, mszcounterlist: Option<windows_core::PSTR>, pcchcounterlistlength: *mut u32, mszinstancelist: Option<windows_core::PSTR>, pcchinstancelistlength: *mut u32, dwdetaillevel: PERF_DETAIL, dwflags: u32) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("pdh.dll" "system" fn PdhEnumObjectItemsA(szdatasource : windows_core::PCSTR, szmachinename : windows_core::PCSTR, szobjectname : windows_core::PCSTR, mszcounterlist : windows_core::PSTR, pcchcounterlistlength : *mut u32, mszinstancelist : windows_core::PSTR, pcchinstancelistlength : *mut u32, dwdetaillevel : PERF_DETAIL, dwflags : u32) -> u32);
    unsafe { PdhEnumObjectItemsA(szdatasource.param().abi(), szmachinename.param().abi(), szobjectname.param().abi(), mszcounterlist.unwrap_or(core::mem::zeroed()) as _, pcchcounterlistlength as _, mszinstancelist.unwrap_or(core::mem::zeroed()) as _, pcchinstancelistlength as _, dwdetaillevel, dwflags) }
}
#[inline]
pub unsafe fn PdhEnumObjectItemsHA<P1, P2>(hdatasource: Option<PDH_HLOG>, szmachinename: P1, szobjectname: P2, mszcounterlist: Option<windows_core::PSTR>, pcchcounterlistlength: *mut u32, mszinstancelist: Option<windows_core::PSTR>, pcchinstancelistlength: *mut u32, dwdetaillevel: PERF_DETAIL, dwflags: u32) -> u32
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("pdh.dll" "system" fn PdhEnumObjectItemsHA(hdatasource : PDH_HLOG, szmachinename : windows_core::PCSTR, szobjectname : windows_core::PCSTR, mszcounterlist : windows_core::PSTR, pcchcounterlistlength : *mut u32, mszinstancelist : windows_core::PSTR, pcchinstancelistlength : *mut u32, dwdetaillevel : PERF_DETAIL, dwflags : u32) -> u32);
    unsafe { PdhEnumObjectItemsHA(hdatasource.unwrap_or(core::mem::zeroed()) as _, szmachinename.param().abi(), szobjectname.param().abi(), mszcounterlist.unwrap_or(core::mem::zeroed()) as _, pcchcounterlistlength as _, mszinstancelist.unwrap_or(core::mem::zeroed()) as _, pcchinstancelistlength as _, dwdetaillevel, dwflags) }
}
#[inline]
pub unsafe fn PdhEnumObjectItemsHW<P1, P2>(hdatasource: Option<PDH_HLOG>, szmachinename: P1, szobjectname: P2, mszcounterlist: Option<windows_core::PWSTR>, pcchcounterlistlength: *mut u32, mszinstancelist: Option<windows_core::PWSTR>, pcchinstancelistlength: *mut u32, dwdetaillevel: PERF_DETAIL, dwflags: u32) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("pdh.dll" "system" fn PdhEnumObjectItemsHW(hdatasource : PDH_HLOG, szmachinename : windows_core::PCWSTR, szobjectname : windows_core::PCWSTR, mszcounterlist : windows_core::PWSTR, pcchcounterlistlength : *mut u32, mszinstancelist : windows_core::PWSTR, pcchinstancelistlength : *mut u32, dwdetaillevel : PERF_DETAIL, dwflags : u32) -> u32);
    unsafe { PdhEnumObjectItemsHW(hdatasource.unwrap_or(core::mem::zeroed()) as _, szmachinename.param().abi(), szobjectname.param().abi(), mszcounterlist.unwrap_or(core::mem::zeroed()) as _, pcchcounterlistlength as _, mszinstancelist.unwrap_or(core::mem::zeroed()) as _, pcchinstancelistlength as _, dwdetaillevel, dwflags) }
}
#[inline]
pub unsafe fn PdhEnumObjectItemsW<P0, P1, P2>(szdatasource: P0, szmachinename: P1, szobjectname: P2, mszcounterlist: Option<windows_core::PWSTR>, pcchcounterlistlength: *mut u32, mszinstancelist: Option<windows_core::PWSTR>, pcchinstancelistlength: *mut u32, dwdetaillevel: PERF_DETAIL, dwflags: u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("pdh.dll" "system" fn PdhEnumObjectItemsW(szdatasource : windows_core::PCWSTR, szmachinename : windows_core::PCWSTR, szobjectname : windows_core::PCWSTR, mszcounterlist : windows_core::PWSTR, pcchcounterlistlength : *mut u32, mszinstancelist : windows_core::PWSTR, pcchinstancelistlength : *mut u32, dwdetaillevel : PERF_DETAIL, dwflags : u32) -> u32);
    unsafe { PdhEnumObjectItemsW(szdatasource.param().abi(), szmachinename.param().abi(), szobjectname.param().abi(), mszcounterlist.unwrap_or(core::mem::zeroed()) as _, pcchcounterlistlength as _, mszinstancelist.unwrap_or(core::mem::zeroed()) as _, pcchinstancelistlength as _, dwdetaillevel, dwflags) }
}
#[inline]
pub unsafe fn PdhEnumObjectsA<P0, P1>(szdatasource: P0, szmachinename: P1, mszobjectlist: Option<windows_core::PSTR>, pcchbuffersize: *mut u32, dwdetaillevel: PERF_DETAIL, brefresh: bool) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("pdh.dll" "system" fn PdhEnumObjectsA(szdatasource : windows_core::PCSTR, szmachinename : windows_core::PCSTR, mszobjectlist : windows_core::PSTR, pcchbuffersize : *mut u32, dwdetaillevel : PERF_DETAIL, brefresh : windows_core::BOOL) -> u32);
    unsafe { PdhEnumObjectsA(szdatasource.param().abi(), szmachinename.param().abi(), mszobjectlist.unwrap_or(core::mem::zeroed()) as _, pcchbuffersize as _, dwdetaillevel, brefresh.into()) }
}
#[inline]
pub unsafe fn PdhEnumObjectsHA<P1>(hdatasource: Option<PDH_HLOG>, szmachinename: P1, mszobjectlist: Option<windows_core::PSTR>, pcchbuffersize: *mut u32, dwdetaillevel: PERF_DETAIL, brefresh: bool) -> u32
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("pdh.dll" "system" fn PdhEnumObjectsHA(hdatasource : PDH_HLOG, szmachinename : windows_core::PCSTR, mszobjectlist : windows_core::PSTR, pcchbuffersize : *mut u32, dwdetaillevel : PERF_DETAIL, brefresh : windows_core::BOOL) -> u32);
    unsafe { PdhEnumObjectsHA(hdatasource.unwrap_or(core::mem::zeroed()) as _, szmachinename.param().abi(), mszobjectlist.unwrap_or(core::mem::zeroed()) as _, pcchbuffersize as _, dwdetaillevel, brefresh.into()) }
}
#[inline]
pub unsafe fn PdhEnumObjectsHW<P1>(hdatasource: Option<PDH_HLOG>, szmachinename: P1, mszobjectlist: Option<windows_core::PWSTR>, pcchbuffersize: *mut u32, dwdetaillevel: PERF_DETAIL, brefresh: bool) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("pdh.dll" "system" fn PdhEnumObjectsHW(hdatasource : PDH_HLOG, szmachinename : windows_core::PCWSTR, mszobjectlist : windows_core::PWSTR, pcchbuffersize : *mut u32, dwdetaillevel : PERF_DETAIL, brefresh : windows_core::BOOL) -> u32);
    unsafe { PdhEnumObjectsHW(hdatasource.unwrap_or(core::mem::zeroed()) as _, szmachinename.param().abi(), mszobjectlist.unwrap_or(core::mem::zeroed()) as _, pcchbuffersize as _, dwdetaillevel, brefresh.into()) }
}
#[inline]
pub unsafe fn PdhEnumObjectsW<P0, P1>(szdatasource: P0, szmachinename: P1, mszobjectlist: Option<windows_core::PWSTR>, pcchbuffersize: *mut u32, dwdetaillevel: PERF_DETAIL, brefresh: bool) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("pdh.dll" "system" fn PdhEnumObjectsW(szdatasource : windows_core::PCWSTR, szmachinename : windows_core::PCWSTR, mszobjectlist : windows_core::PWSTR, pcchbuffersize : *mut u32, dwdetaillevel : PERF_DETAIL, brefresh : windows_core::BOOL) -> u32);
    unsafe { PdhEnumObjectsW(szdatasource.param().abi(), szmachinename.param().abi(), mszobjectlist.unwrap_or(core::mem::zeroed()) as _, pcchbuffersize as _, dwdetaillevel, brefresh.into()) }
}
#[inline]
pub unsafe fn PdhExpandCounterPathA<P0>(szwildcardpath: P0, mszexpandedpathlist: Option<windows_core::PSTR>, pcchpathlistlength: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("pdh.dll" "system" fn PdhExpandCounterPathA(szwildcardpath : windows_core::PCSTR, mszexpandedpathlist : windows_core::PSTR, pcchpathlistlength : *mut u32) -> u32);
    unsafe { PdhExpandCounterPathA(szwildcardpath.param().abi(), mszexpandedpathlist.unwrap_or(core::mem::zeroed()) as _, pcchpathlistlength as _) }
}
#[inline]
pub unsafe fn PdhExpandCounterPathW<P0>(szwildcardpath: P0, mszexpandedpathlist: Option<windows_core::PWSTR>, pcchpathlistlength: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("pdh.dll" "system" fn PdhExpandCounterPathW(szwildcardpath : windows_core::PCWSTR, mszexpandedpathlist : windows_core::PWSTR, pcchpathlistlength : *mut u32) -> u32);
    unsafe { PdhExpandCounterPathW(szwildcardpath.param().abi(), mszexpandedpathlist.unwrap_or(core::mem::zeroed()) as _, pcchpathlistlength as _) }
}
#[inline]
pub unsafe fn PdhExpandWildCardPathA<P0, P1>(szdatasource: P0, szwildcardpath: P1, mszexpandedpathlist: Option<windows_core::PSTR>, pcchpathlistlength: *mut u32, dwflags: u32) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("pdh.dll" "system" fn PdhExpandWildCardPathA(szdatasource : windows_core::PCSTR, szwildcardpath : windows_core::PCSTR, mszexpandedpathlist : windows_core::PSTR, pcchpathlistlength : *mut u32, dwflags : u32) -> u32);
    unsafe { PdhExpandWildCardPathA(szdatasource.param().abi(), szwildcardpath.param().abi(), mszexpandedpathlist.unwrap_or(core::mem::zeroed()) as _, pcchpathlistlength as _, dwflags) }
}
#[inline]
pub unsafe fn PdhExpandWildCardPathHA<P1>(hdatasource: Option<PDH_HLOG>, szwildcardpath: P1, mszexpandedpathlist: Option<windows_core::PSTR>, pcchpathlistlength: *mut u32, dwflags: u32) -> u32
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("pdh.dll" "system" fn PdhExpandWildCardPathHA(hdatasource : PDH_HLOG, szwildcardpath : windows_core::PCSTR, mszexpandedpathlist : windows_core::PSTR, pcchpathlistlength : *mut u32, dwflags : u32) -> u32);
    unsafe { PdhExpandWildCardPathHA(hdatasource.unwrap_or(core::mem::zeroed()) as _, szwildcardpath.param().abi(), mszexpandedpathlist.unwrap_or(core::mem::zeroed()) as _, pcchpathlistlength as _, dwflags) }
}
#[inline]
pub unsafe fn PdhExpandWildCardPathHW<P1>(hdatasource: Option<PDH_HLOG>, szwildcardpath: P1, mszexpandedpathlist: Option<windows_core::PWSTR>, pcchpathlistlength: *mut u32, dwflags: u32) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("pdh.dll" "system" fn PdhExpandWildCardPathHW(hdatasource : PDH_HLOG, szwildcardpath : windows_core::PCWSTR, mszexpandedpathlist : windows_core::PWSTR, pcchpathlistlength : *mut u32, dwflags : u32) -> u32);
    unsafe { PdhExpandWildCardPathHW(hdatasource.unwrap_or(core::mem::zeroed()) as _, szwildcardpath.param().abi(), mszexpandedpathlist.unwrap_or(core::mem::zeroed()) as _, pcchpathlistlength as _, dwflags) }
}
#[inline]
pub unsafe fn PdhExpandWildCardPathW<P0, P1>(szdatasource: P0, szwildcardpath: P1, mszexpandedpathlist: Option<windows_core::PWSTR>, pcchpathlistlength: *mut u32, dwflags: u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("pdh.dll" "system" fn PdhExpandWildCardPathW(szdatasource : windows_core::PCWSTR, szwildcardpath : windows_core::PCWSTR, mszexpandedpathlist : windows_core::PWSTR, pcchpathlistlength : *mut u32, dwflags : u32) -> u32);
    unsafe { PdhExpandWildCardPathW(szdatasource.param().abi(), szwildcardpath.param().abi(), mszexpandedpathlist.unwrap_or(core::mem::zeroed()) as _, pcchpathlistlength as _, dwflags) }
}
#[inline]
pub unsafe fn PdhFormatFromRawValue(dwcountertype: u32, dwformat: PDH_FMT, ptimebase: Option<*const i64>, prawvalue1: *const PDH_RAW_COUNTER, prawvalue2: *const PDH_RAW_COUNTER, pfmtvalue: *mut PDH_FMT_COUNTERVALUE) -> u32 {
    windows_core::link!("pdh.dll" "system" fn PdhFormatFromRawValue(dwcountertype : u32, dwformat : PDH_FMT, ptimebase : *const i64, prawvalue1 : *const PDH_RAW_COUNTER, prawvalue2 : *const PDH_RAW_COUNTER, pfmtvalue : *mut PDH_FMT_COUNTERVALUE) -> u32);
    unsafe { PdhFormatFromRawValue(dwcountertype, dwformat, ptimebase.unwrap_or(core::mem::zeroed()) as _, prawvalue1, prawvalue2, pfmtvalue as _) }
}
#[inline]
pub unsafe fn PdhGetCounterInfoA(hcounter: PDH_HCOUNTER, bretrieveexplaintext: bool, pdwbuffersize: *mut u32, lpbuffer: Option<*mut PDH_COUNTER_INFO_A>) -> u32 {
    windows_core::link!("pdh.dll" "system" fn PdhGetCounterInfoA(hcounter : PDH_HCOUNTER, bretrieveexplaintext : bool, pdwbuffersize : *mut u32, lpbuffer : *mut PDH_COUNTER_INFO_A) -> u32);
    unsafe { PdhGetCounterInfoA(hcounter, bretrieveexplaintext, pdwbuffersize as _, lpbuffer.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn PdhGetCounterInfoW(hcounter: PDH_HCOUNTER, bretrieveexplaintext: bool, pdwbuffersize: *mut u32, lpbuffer: Option<*mut PDH_COUNTER_INFO_W>) -> u32 {
    windows_core::link!("pdh.dll" "system" fn PdhGetCounterInfoW(hcounter : PDH_HCOUNTER, bretrieveexplaintext : bool, pdwbuffersize : *mut u32, lpbuffer : *mut PDH_COUNTER_INFO_W) -> u32);
    unsafe { PdhGetCounterInfoW(hcounter, bretrieveexplaintext, pdwbuffersize as _, lpbuffer.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn PdhGetCounterTimeBase(hcounter: PDH_HCOUNTER, ptimebase: *mut i64) -> u32 {
    windows_core::link!("pdh.dll" "system" fn PdhGetCounterTimeBase(hcounter : PDH_HCOUNTER, ptimebase : *mut i64) -> u32);
    unsafe { PdhGetCounterTimeBase(hcounter, ptimebase as _) }
}
#[inline]
pub unsafe fn PdhGetDataSourceTimeRangeA<P0>(szdatasource: P0, pdwnumentries: *mut u32, pinfo: *mut PDH_TIME_INFO, pdwbuffersize: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("pdh.dll" "system" fn PdhGetDataSourceTimeRangeA(szdatasource : windows_core::PCSTR, pdwnumentries : *mut u32, pinfo : *mut PDH_TIME_INFO, pdwbuffersize : *mut u32) -> u32);
    unsafe { PdhGetDataSourceTimeRangeA(szdatasource.param().abi(), pdwnumentries as _, pinfo as _, pdwbuffersize as _) }
}
#[inline]
pub unsafe fn PdhGetDataSourceTimeRangeH(hdatasource: Option<PDH_HLOG>, pdwnumentries: *mut u32, pinfo: *mut PDH_TIME_INFO, pdwbuffersize: *mut u32) -> u32 {
    windows_core::link!("pdh.dll" "system" fn PdhGetDataSourceTimeRangeH(hdatasource : PDH_HLOG, pdwnumentries : *mut u32, pinfo : *mut PDH_TIME_INFO, pdwbuffersize : *mut u32) -> u32);
    unsafe { PdhGetDataSourceTimeRangeH(hdatasource.unwrap_or(core::mem::zeroed()) as _, pdwnumentries as _, pinfo as _, pdwbuffersize as _) }
}
#[inline]
pub unsafe fn PdhGetDataSourceTimeRangeW<P0>(szdatasource: P0, pdwnumentries: *mut u32, pinfo: *mut PDH_TIME_INFO, pdwbuffersize: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("pdh.dll" "system" fn PdhGetDataSourceTimeRangeW(szdatasource : windows_core::PCWSTR, pdwnumentries : *mut u32, pinfo : *mut PDH_TIME_INFO, pdwbuffersize : *mut u32) -> u32);
    unsafe { PdhGetDataSourceTimeRangeW(szdatasource.param().abi(), pdwnumentries as _, pinfo as _, pdwbuffersize as _) }
}
#[inline]
pub unsafe fn PdhGetDefaultPerfCounterA<P0, P1, P2>(szdatasource: P0, szmachinename: P1, szobjectname: P2, szdefaultcountername: Option<windows_core::PSTR>, pcchbuffersize: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("pdh.dll" "system" fn PdhGetDefaultPerfCounterA(szdatasource : windows_core::PCSTR, szmachinename : windows_core::PCSTR, szobjectname : windows_core::PCSTR, szdefaultcountername : windows_core::PSTR, pcchbuffersize : *mut u32) -> u32);
    unsafe { PdhGetDefaultPerfCounterA(szdatasource.param().abi(), szmachinename.param().abi(), szobjectname.param().abi(), szdefaultcountername.unwrap_or(core::mem::zeroed()) as _, pcchbuffersize as _) }
}
#[inline]
pub unsafe fn PdhGetDefaultPerfCounterHA<P1, P2>(hdatasource: Option<PDH_HLOG>, szmachinename: P1, szobjectname: P2, szdefaultcountername: Option<windows_core::PSTR>, pcchbuffersize: *mut u32) -> u32
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("pdh.dll" "system" fn PdhGetDefaultPerfCounterHA(hdatasource : PDH_HLOG, szmachinename : windows_core::PCSTR, szobjectname : windows_core::PCSTR, szdefaultcountername : windows_core::PSTR, pcchbuffersize : *mut u32) -> u32);
    unsafe { PdhGetDefaultPerfCounterHA(hdatasource.unwrap_or(core::mem::zeroed()) as _, szmachinename.param().abi(), szobjectname.param().abi(), szdefaultcountername.unwrap_or(core::mem::zeroed()) as _, pcchbuffersize as _) }
}
#[inline]
pub unsafe fn PdhGetDefaultPerfCounterHW<P1, P2>(hdatasource: Option<PDH_HLOG>, szmachinename: P1, szobjectname: P2, szdefaultcountername: Option<windows_core::PWSTR>, pcchbuffersize: *mut u32) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("pdh.dll" "system" fn PdhGetDefaultPerfCounterHW(hdatasource : PDH_HLOG, szmachinename : windows_core::PCWSTR, szobjectname : windows_core::PCWSTR, szdefaultcountername : windows_core::PWSTR, pcchbuffersize : *mut u32) -> u32);
    unsafe { PdhGetDefaultPerfCounterHW(hdatasource.unwrap_or(core::mem::zeroed()) as _, szmachinename.param().abi(), szobjectname.param().abi(), szdefaultcountername.unwrap_or(core::mem::zeroed()) as _, pcchbuffersize as _) }
}
#[inline]
pub unsafe fn PdhGetDefaultPerfCounterW<P0, P1, P2>(szdatasource: P0, szmachinename: P1, szobjectname: P2, szdefaultcountername: Option<windows_core::PWSTR>, pcchbuffersize: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("pdh.dll" "system" fn PdhGetDefaultPerfCounterW(szdatasource : windows_core::PCWSTR, szmachinename : windows_core::PCWSTR, szobjectname : windows_core::PCWSTR, szdefaultcountername : windows_core::PWSTR, pcchbuffersize : *mut u32) -> u32);
    unsafe { PdhGetDefaultPerfCounterW(szdatasource.param().abi(), szmachinename.param().abi(), szobjectname.param().abi(), szdefaultcountername.unwrap_or(core::mem::zeroed()) as _, pcchbuffersize as _) }
}
#[inline]
pub unsafe fn PdhGetDefaultPerfObjectA<P0, P1>(szdatasource: P0, szmachinename: P1, szdefaultobjectname: Option<windows_core::PSTR>, pcchbuffersize: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("pdh.dll" "system" fn PdhGetDefaultPerfObjectA(szdatasource : windows_core::PCSTR, szmachinename : windows_core::PCSTR, szdefaultobjectname : windows_core::PSTR, pcchbuffersize : *mut u32) -> u32);
    unsafe { PdhGetDefaultPerfObjectA(szdatasource.param().abi(), szmachinename.param().abi(), szdefaultobjectname.unwrap_or(core::mem::zeroed()) as _, pcchbuffersize as _) }
}
#[inline]
pub unsafe fn PdhGetDefaultPerfObjectHA<P1>(hdatasource: Option<PDH_HLOG>, szmachinename: P1, szdefaultobjectname: Option<windows_core::PSTR>, pcchbuffersize: *mut u32) -> u32
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("pdh.dll" "system" fn PdhGetDefaultPerfObjectHA(hdatasource : PDH_HLOG, szmachinename : windows_core::PCSTR, szdefaultobjectname : windows_core::PSTR, pcchbuffersize : *mut u32) -> u32);
    unsafe { PdhGetDefaultPerfObjectHA(hdatasource.unwrap_or(core::mem::zeroed()) as _, szmachinename.param().abi(), szdefaultobjectname.unwrap_or(core::mem::zeroed()) as _, pcchbuffersize as _) }
}
#[inline]
pub unsafe fn PdhGetDefaultPerfObjectHW<P1>(hdatasource: Option<PDH_HLOG>, szmachinename: P1, szdefaultobjectname: Option<windows_core::PWSTR>, pcchbuffersize: *mut u32) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("pdh.dll" "system" fn PdhGetDefaultPerfObjectHW(hdatasource : PDH_HLOG, szmachinename : windows_core::PCWSTR, szdefaultobjectname : windows_core::PWSTR, pcchbuffersize : *mut u32) -> u32);
    unsafe { PdhGetDefaultPerfObjectHW(hdatasource.unwrap_or(core::mem::zeroed()) as _, szmachinename.param().abi(), szdefaultobjectname.unwrap_or(core::mem::zeroed()) as _, pcchbuffersize as _) }
}
#[inline]
pub unsafe fn PdhGetDefaultPerfObjectW<P0, P1>(szdatasource: P0, szmachinename: P1, szdefaultobjectname: Option<windows_core::PWSTR>, pcchbuffersize: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("pdh.dll" "system" fn PdhGetDefaultPerfObjectW(szdatasource : windows_core::PCWSTR, szmachinename : windows_core::PCWSTR, szdefaultobjectname : windows_core::PWSTR, pcchbuffersize : *mut u32) -> u32);
    unsafe { PdhGetDefaultPerfObjectW(szdatasource.param().abi(), szmachinename.param().abi(), szdefaultobjectname.unwrap_or(core::mem::zeroed()) as _, pcchbuffersize as _) }
}
#[inline]
pub unsafe fn PdhGetDllVersion(lpdwversion: Option<*mut PDH_DLL_VERSION>) -> u32 {
    windows_core::link!("pdh.dll" "system" fn PdhGetDllVersion(lpdwversion : *mut PDH_DLL_VERSION) -> u32);
    unsafe { PdhGetDllVersion(lpdwversion.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn PdhGetFormattedCounterArrayA(hcounter: PDH_HCOUNTER, dwformat: PDH_FMT, lpdwbuffersize: *mut u32, lpdwitemcount: *mut u32, itembuffer: Option<*mut PDH_FMT_COUNTERVALUE_ITEM_A>) -> u32 {
    windows_core::link!("pdh.dll" "system" fn PdhGetFormattedCounterArrayA(hcounter : PDH_HCOUNTER, dwformat : PDH_FMT, lpdwbuffersize : *mut u32, lpdwitemcount : *mut u32, itembuffer : *mut PDH_FMT_COUNTERVALUE_ITEM_A) -> u32);
    unsafe { PdhGetFormattedCounterArrayA(hcounter, dwformat, lpdwbuffersize as _, lpdwitemcount as _, itembuffer.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn PdhGetFormattedCounterArrayW(hcounter: PDH_HCOUNTER, dwformat: PDH_FMT, lpdwbuffersize: *mut u32, lpdwitemcount: *mut u32, itembuffer: Option<*mut PDH_FMT_COUNTERVALUE_ITEM_W>) -> u32 {
    windows_core::link!("pdh.dll" "system" fn PdhGetFormattedCounterArrayW(hcounter : PDH_HCOUNTER, dwformat : PDH_FMT, lpdwbuffersize : *mut u32, lpdwitemcount : *mut u32, itembuffer : *mut PDH_FMT_COUNTERVALUE_ITEM_W) -> u32);
    unsafe { PdhGetFormattedCounterArrayW(hcounter, dwformat, lpdwbuffersize as _, lpdwitemcount as _, itembuffer.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn PdhGetFormattedCounterValue(hcounter: PDH_HCOUNTER, dwformat: PDH_FMT, lpdwtype: Option<*mut u32>, pvalue: *mut PDH_FMT_COUNTERVALUE) -> u32 {
    windows_core::link!("pdh.dll" "system" fn PdhGetFormattedCounterValue(hcounter : PDH_HCOUNTER, dwformat : PDH_FMT, lpdwtype : *mut u32, pvalue : *mut PDH_FMT_COUNTERVALUE) -> u32);
    unsafe { PdhGetFormattedCounterValue(hcounter, dwformat, lpdwtype.unwrap_or(core::mem::zeroed()) as _, pvalue as _) }
}
#[inline]
pub unsafe fn PdhGetLogFileSize(hlog: PDH_HLOG, llsize: *mut i64) -> u32 {
    windows_core::link!("pdh.dll" "system" fn PdhGetLogFileSize(hlog : PDH_HLOG, llsize : *mut i64) -> u32);
    unsafe { PdhGetLogFileSize(hlog, llsize as _) }
}
#[inline]
pub unsafe fn PdhGetLogSetGUID(hlog: PDH_HLOG, pguid: Option<*mut windows_core::GUID>, prunid: Option<*mut i32>) -> u32 {
    windows_core::link!("pdh.dll" "system" fn PdhGetLogSetGUID(hlog : PDH_HLOG, pguid : *mut windows_core::GUID, prunid : *mut i32) -> u32);
    unsafe { PdhGetLogSetGUID(hlog, pguid.unwrap_or(core::mem::zeroed()) as _, prunid.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn PdhGetRawCounterArrayA(hcounter: PDH_HCOUNTER, lpdwbuffersize: *mut u32, lpdwitemcount: *mut u32, itembuffer: Option<*mut PDH_RAW_COUNTER_ITEM_A>) -> u32 {
    windows_core::link!("pdh.dll" "system" fn PdhGetRawCounterArrayA(hcounter : PDH_HCOUNTER, lpdwbuffersize : *mut u32, lpdwitemcount : *mut u32, itembuffer : *mut PDH_RAW_COUNTER_ITEM_A) -> u32);
    unsafe { PdhGetRawCounterArrayA(hcounter, lpdwbuffersize as _, lpdwitemcount as _, itembuffer.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn PdhGetRawCounterArrayW(hcounter: PDH_HCOUNTER, lpdwbuffersize: *mut u32, lpdwitemcount: *mut u32, itembuffer: Option<*mut PDH_RAW_COUNTER_ITEM_W>) -> u32 {
    windows_core::link!("pdh.dll" "system" fn PdhGetRawCounterArrayW(hcounter : PDH_HCOUNTER, lpdwbuffersize : *mut u32, lpdwitemcount : *mut u32, itembuffer : *mut PDH_RAW_COUNTER_ITEM_W) -> u32);
    unsafe { PdhGetRawCounterArrayW(hcounter, lpdwbuffersize as _, lpdwitemcount as _, itembuffer.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn PdhGetRawCounterValue(hcounter: PDH_HCOUNTER, lpdwtype: Option<*mut u32>, pvalue: *mut PDH_RAW_COUNTER) -> u32 {
    windows_core::link!("pdh.dll" "system" fn PdhGetRawCounterValue(hcounter : PDH_HCOUNTER, lpdwtype : *mut u32, pvalue : *mut PDH_RAW_COUNTER) -> u32);
    unsafe { PdhGetRawCounterValue(hcounter, lpdwtype.unwrap_or(core::mem::zeroed()) as _, pvalue as _) }
}
#[inline]
pub unsafe fn PdhIsRealTimeQuery(hquery: PDH_HQUERY) -> windows_core::BOOL {
    windows_core::link!("pdh.dll" "system" fn PdhIsRealTimeQuery(hquery : PDH_HQUERY) -> windows_core::BOOL);
    unsafe { PdhIsRealTimeQuery(hquery) }
}
#[inline]
pub unsafe fn PdhLookupPerfIndexByNameA<P0, P1>(szmachinename: P0, sznamebuffer: P1, pdwindex: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("pdh.dll" "system" fn PdhLookupPerfIndexByNameA(szmachinename : windows_core::PCSTR, sznamebuffer : windows_core::PCSTR, pdwindex : *mut u32) -> u32);
    unsafe { PdhLookupPerfIndexByNameA(szmachinename.param().abi(), sznamebuffer.param().abi(), pdwindex as _) }
}
#[inline]
pub unsafe fn PdhLookupPerfIndexByNameW<P0, P1>(szmachinename: P0, sznamebuffer: P1, pdwindex: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("pdh.dll" "system" fn PdhLookupPerfIndexByNameW(szmachinename : windows_core::PCWSTR, sznamebuffer : windows_core::PCWSTR, pdwindex : *mut u32) -> u32);
    unsafe { PdhLookupPerfIndexByNameW(szmachinename.param().abi(), sznamebuffer.param().abi(), pdwindex as _) }
}
#[inline]
pub unsafe fn PdhLookupPerfNameByIndexA<P0>(szmachinename: P0, dwnameindex: u32, sznamebuffer: Option<windows_core::PSTR>, pcchnamebuffersize: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("pdh.dll" "system" fn PdhLookupPerfNameByIndexA(szmachinename : windows_core::PCSTR, dwnameindex : u32, sznamebuffer : windows_core::PSTR, pcchnamebuffersize : *mut u32) -> u32);
    unsafe { PdhLookupPerfNameByIndexA(szmachinename.param().abi(), dwnameindex, sznamebuffer.unwrap_or(core::mem::zeroed()) as _, pcchnamebuffersize as _) }
}
#[inline]
pub unsafe fn PdhLookupPerfNameByIndexW<P0>(szmachinename: P0, dwnameindex: u32, sznamebuffer: Option<windows_core::PWSTR>, pcchnamebuffersize: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("pdh.dll" "system" fn PdhLookupPerfNameByIndexW(szmachinename : windows_core::PCWSTR, dwnameindex : u32, sznamebuffer : windows_core::PWSTR, pcchnamebuffersize : *mut u32) -> u32);
    unsafe { PdhLookupPerfNameByIndexW(szmachinename.param().abi(), dwnameindex, sznamebuffer.unwrap_or(core::mem::zeroed()) as _, pcchnamebuffersize as _) }
}
#[inline]
pub unsafe fn PdhMakeCounterPathA(pcounterpathelements: *const PDH_COUNTER_PATH_ELEMENTS_A, szfullpathbuffer: Option<windows_core::PSTR>, pcchbuffersize: *mut u32, dwflags: PDH_PATH_FLAGS) -> u32 {
    windows_core::link!("pdh.dll" "system" fn PdhMakeCounterPathA(pcounterpathelements : *const PDH_COUNTER_PATH_ELEMENTS_A, szfullpathbuffer : windows_core::PSTR, pcchbuffersize : *mut u32, dwflags : PDH_PATH_FLAGS) -> u32);
    unsafe { PdhMakeCounterPathA(pcounterpathelements, szfullpathbuffer.unwrap_or(core::mem::zeroed()) as _, pcchbuffersize as _, dwflags) }
}
#[inline]
pub unsafe fn PdhMakeCounterPathW(pcounterpathelements: *const PDH_COUNTER_PATH_ELEMENTS_W, szfullpathbuffer: Option<windows_core::PWSTR>, pcchbuffersize: *mut u32, dwflags: PDH_PATH_FLAGS) -> u32 {
    windows_core::link!("pdh.dll" "system" fn PdhMakeCounterPathW(pcounterpathelements : *const PDH_COUNTER_PATH_ELEMENTS_W, szfullpathbuffer : windows_core::PWSTR, pcchbuffersize : *mut u32, dwflags : PDH_PATH_FLAGS) -> u32);
    unsafe { PdhMakeCounterPathW(pcounterpathelements, szfullpathbuffer.unwrap_or(core::mem::zeroed()) as _, pcchbuffersize as _, dwflags) }
}
#[inline]
pub unsafe fn PdhOpenLogA<P0, P5>(szlogfilename: P0, dwaccessflags: PDH_LOG, lpdwlogtype: *mut PDH_LOG_TYPE, hquery: Option<PDH_HQUERY>, dwmaxsize: u32, szusercaption: P5, phlog: *mut PDH_HLOG) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P5: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("pdh.dll" "system" fn PdhOpenLogA(szlogfilename : windows_core::PCSTR, dwaccessflags : PDH_LOG, lpdwlogtype : *mut PDH_LOG_TYPE, hquery : PDH_HQUERY, dwmaxsize : u32, szusercaption : windows_core::PCSTR, phlog : *mut PDH_HLOG) -> u32);
    unsafe { PdhOpenLogA(szlogfilename.param().abi(), dwaccessflags, lpdwlogtype as _, hquery.unwrap_or(core::mem::zeroed()) as _, dwmaxsize, szusercaption.param().abi(), phlog as _) }
}
#[inline]
pub unsafe fn PdhOpenLogW<P0, P5>(szlogfilename: P0, dwaccessflags: PDH_LOG, lpdwlogtype: *mut PDH_LOG_TYPE, hquery: Option<PDH_HQUERY>, dwmaxsize: u32, szusercaption: P5, phlog: *mut PDH_HLOG) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P5: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("pdh.dll" "system" fn PdhOpenLogW(szlogfilename : windows_core::PCWSTR, dwaccessflags : PDH_LOG, lpdwlogtype : *mut PDH_LOG_TYPE, hquery : PDH_HQUERY, dwmaxsize : u32, szusercaption : windows_core::PCWSTR, phlog : *mut PDH_HLOG) -> u32);
    unsafe { PdhOpenLogW(szlogfilename.param().abi(), dwaccessflags, lpdwlogtype as _, hquery.unwrap_or(core::mem::zeroed()) as _, dwmaxsize, szusercaption.param().abi(), phlog as _) }
}
#[inline]
pub unsafe fn PdhOpenQueryA<P0>(szdatasource: P0, dwuserdata: usize, phquery: *mut PDH_HQUERY) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("pdh.dll" "system" fn PdhOpenQueryA(szdatasource : windows_core::PCSTR, dwuserdata : usize, phquery : *mut PDH_HQUERY) -> u32);
    unsafe { PdhOpenQueryA(szdatasource.param().abi(), dwuserdata, phquery as _) }
}
#[inline]
pub unsafe fn PdhOpenQueryH(hdatasource: Option<PDH_HLOG>, dwuserdata: usize, phquery: *mut PDH_HQUERY) -> u32 {
    windows_core::link!("pdh.dll" "system" fn PdhOpenQueryH(hdatasource : PDH_HLOG, dwuserdata : usize, phquery : *mut PDH_HQUERY) -> u32);
    unsafe { PdhOpenQueryH(hdatasource.unwrap_or(core::mem::zeroed()) as _, dwuserdata, phquery as _) }
}
#[inline]
pub unsafe fn PdhOpenQueryW<P0>(szdatasource: P0, dwuserdata: usize, phquery: *mut PDH_HQUERY) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("pdh.dll" "system" fn PdhOpenQueryW(szdatasource : windows_core::PCWSTR, dwuserdata : usize, phquery : *mut PDH_HQUERY) -> u32);
    unsafe { PdhOpenQueryW(szdatasource.param().abi(), dwuserdata, phquery as _) }
}
#[inline]
pub unsafe fn PdhParseCounterPathA<P0>(szfullpathbuffer: P0, pcounterpathelements: Option<*mut PDH_COUNTER_PATH_ELEMENTS_A>, pdwbuffersize: *mut u32, dwflags: u32) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("pdh.dll" "system" fn PdhParseCounterPathA(szfullpathbuffer : windows_core::PCSTR, pcounterpathelements : *mut PDH_COUNTER_PATH_ELEMENTS_A, pdwbuffersize : *mut u32, dwflags : u32) -> u32);
    unsafe { PdhParseCounterPathA(szfullpathbuffer.param().abi(), pcounterpathelements.unwrap_or(core::mem::zeroed()) as _, pdwbuffersize as _, dwflags) }
}
#[inline]
pub unsafe fn PdhParseCounterPathW<P0>(szfullpathbuffer: P0, pcounterpathelements: Option<*mut PDH_COUNTER_PATH_ELEMENTS_W>, pdwbuffersize: *mut u32, dwflags: u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("pdh.dll" "system" fn PdhParseCounterPathW(szfullpathbuffer : windows_core::PCWSTR, pcounterpathelements : *mut PDH_COUNTER_PATH_ELEMENTS_W, pdwbuffersize : *mut u32, dwflags : u32) -> u32);
    unsafe { PdhParseCounterPathW(szfullpathbuffer.param().abi(), pcounterpathelements.unwrap_or(core::mem::zeroed()) as _, pdwbuffersize as _, dwflags) }
}
#[inline]
pub unsafe fn PdhParseInstanceNameA<P0>(szinstancestring: P0, szinstancename: Option<windows_core::PSTR>, pcchinstancenamelength: *mut u32, szparentname: Option<windows_core::PSTR>, pcchparentnamelength: *mut u32, lpindex: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("pdh.dll" "system" fn PdhParseInstanceNameA(szinstancestring : windows_core::PCSTR, szinstancename : windows_core::PSTR, pcchinstancenamelength : *mut u32, szparentname : windows_core::PSTR, pcchparentnamelength : *mut u32, lpindex : *mut u32) -> u32);
    unsafe { PdhParseInstanceNameA(szinstancestring.param().abi(), szinstancename.unwrap_or(core::mem::zeroed()) as _, pcchinstancenamelength as _, szparentname.unwrap_or(core::mem::zeroed()) as _, pcchparentnamelength as _, lpindex as _) }
}
#[inline]
pub unsafe fn PdhParseInstanceNameW<P0>(szinstancestring: P0, szinstancename: Option<windows_core::PWSTR>, pcchinstancenamelength: *mut u32, szparentname: Option<windows_core::PWSTR>, pcchparentnamelength: *mut u32, lpindex: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("pdh.dll" "system" fn PdhParseInstanceNameW(szinstancestring : windows_core::PCWSTR, szinstancename : windows_core::PWSTR, pcchinstancenamelength : *mut u32, szparentname : windows_core::PWSTR, pcchparentnamelength : *mut u32, lpindex : *mut u32) -> u32);
    unsafe { PdhParseInstanceNameW(szinstancestring.param().abi(), szinstancename.unwrap_or(core::mem::zeroed()) as _, pcchinstancenamelength as _, szparentname.unwrap_or(core::mem::zeroed()) as _, pcchparentnamelength as _, lpindex as _) }
}
#[inline]
pub unsafe fn PdhReadRawLogRecord(hlog: PDH_HLOG, ftrecord: super::super::Foundation::FILETIME, prawlogrecord: Option<*mut PDH_RAW_LOG_RECORD>, pdwbufferlength: *mut u32) -> u32 {
    windows_core::link!("pdh.dll" "system" fn PdhReadRawLogRecord(hlog : PDH_HLOG, ftrecord : super::super::Foundation:: FILETIME, prawlogrecord : *mut PDH_RAW_LOG_RECORD, pdwbufferlength : *mut u32) -> u32);
    unsafe { PdhReadRawLogRecord(hlog, core::mem::transmute(ftrecord), prawlogrecord.unwrap_or(core::mem::zeroed()) as _, pdwbufferlength as _) }
}
#[inline]
pub unsafe fn PdhRemoveCounter(hcounter: PDH_HCOUNTER) -> u32 {
    windows_core::link!("pdh.dll" "system" fn PdhRemoveCounter(hcounter : PDH_HCOUNTER) -> u32);
    unsafe { PdhRemoveCounter(hcounter) }
}
#[inline]
pub unsafe fn PdhSelectDataSourceA(hwndowner: super::super::Foundation::HWND, dwflags: PDH_SELECT_DATA_SOURCE_FLAGS, szdatasource: windows_core::PSTR, pcchbufferlength: *mut u32) -> u32 {
    windows_core::link!("pdh.dll" "system" fn PdhSelectDataSourceA(hwndowner : super::super::Foundation:: HWND, dwflags : PDH_SELECT_DATA_SOURCE_FLAGS, szdatasource : windows_core::PSTR, pcchbufferlength : *mut u32) -> u32);
    unsafe { PdhSelectDataSourceA(hwndowner, dwflags, core::mem::transmute(szdatasource), pcchbufferlength as _) }
}
#[inline]
pub unsafe fn PdhSelectDataSourceW(hwndowner: super::super::Foundation::HWND, dwflags: PDH_SELECT_DATA_SOURCE_FLAGS, szdatasource: windows_core::PWSTR, pcchbufferlength: *mut u32) -> u32 {
    windows_core::link!("pdh.dll" "system" fn PdhSelectDataSourceW(hwndowner : super::super::Foundation:: HWND, dwflags : PDH_SELECT_DATA_SOURCE_FLAGS, szdatasource : windows_core::PWSTR, pcchbufferlength : *mut u32) -> u32);
    unsafe { PdhSelectDataSourceW(hwndowner, dwflags, core::mem::transmute(szdatasource), pcchbufferlength as _) }
}
#[inline]
pub unsafe fn PdhSetCounterScaleFactor(hcounter: PDH_HCOUNTER, lfactor: i32) -> u32 {
    windows_core::link!("pdh.dll" "system" fn PdhSetCounterScaleFactor(hcounter : PDH_HCOUNTER, lfactor : i32) -> u32);
    unsafe { PdhSetCounterScaleFactor(hcounter as _, lfactor) }
}
#[inline]
pub unsafe fn PdhSetDefaultRealTimeDataSource(dwdatasourceid: REAL_TIME_DATA_SOURCE_ID_FLAGS) -> u32 {
    windows_core::link!("pdh.dll" "system" fn PdhSetDefaultRealTimeDataSource(dwdatasourceid : REAL_TIME_DATA_SOURCE_ID_FLAGS) -> u32);
    unsafe { PdhSetDefaultRealTimeDataSource(dwdatasourceid) }
}
#[inline]
pub unsafe fn PdhSetLogSetRunID(hlog: PDH_HLOG, runid: i32) -> u32 {
    windows_core::link!("pdh.dll" "system" fn PdhSetLogSetRunID(hlog : PDH_HLOG, runid : i32) -> u32);
    unsafe { PdhSetLogSetRunID(hlog as _, runid) }
}
#[inline]
pub unsafe fn PdhSetQueryTimeRange(hquery: PDH_HQUERY, pinfo: *const PDH_TIME_INFO) -> u32 {
    windows_core::link!("pdh.dll" "system" fn PdhSetQueryTimeRange(hquery : PDH_HQUERY, pinfo : *const PDH_TIME_INFO) -> u32);
    unsafe { PdhSetQueryTimeRange(hquery, pinfo) }
}
#[inline]
pub unsafe fn PdhUpdateLogA<P1>(hlog: PDH_HLOG, szuserstring: P1) -> u32
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("pdh.dll" "system" fn PdhUpdateLogA(hlog : PDH_HLOG, szuserstring : windows_core::PCSTR) -> u32);
    unsafe { PdhUpdateLogA(hlog, szuserstring.param().abi()) }
}
#[inline]
pub unsafe fn PdhUpdateLogFileCatalog(hlog: PDH_HLOG) -> u32 {
    windows_core::link!("pdh.dll" "system" fn PdhUpdateLogFileCatalog(hlog : PDH_HLOG) -> u32);
    unsafe { PdhUpdateLogFileCatalog(hlog) }
}
#[inline]
pub unsafe fn PdhUpdateLogW<P1>(hlog: PDH_HLOG, szuserstring: P1) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("pdh.dll" "system" fn PdhUpdateLogW(hlog : PDH_HLOG, szuserstring : windows_core::PCWSTR) -> u32);
    unsafe { PdhUpdateLogW(hlog, szuserstring.param().abi()) }
}
#[inline]
pub unsafe fn PdhValidatePathA<P0>(szfullpathbuffer: P0) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("pdh.dll" "system" fn PdhValidatePathA(szfullpathbuffer : windows_core::PCSTR) -> u32);
    unsafe { PdhValidatePathA(szfullpathbuffer.param().abi()) }
}
#[inline]
pub unsafe fn PdhValidatePathExA<P1>(hdatasource: Option<PDH_HLOG>, szfullpathbuffer: P1) -> u32
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("pdh.dll" "system" fn PdhValidatePathExA(hdatasource : PDH_HLOG, szfullpathbuffer : windows_core::PCSTR) -> u32);
    unsafe { PdhValidatePathExA(hdatasource.unwrap_or(core::mem::zeroed()) as _, szfullpathbuffer.param().abi()) }
}
#[inline]
pub unsafe fn PdhValidatePathExW<P1>(hdatasource: Option<PDH_HLOG>, szfullpathbuffer: P1) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("pdh.dll" "system" fn PdhValidatePathExW(hdatasource : PDH_HLOG, szfullpathbuffer : windows_core::PCWSTR) -> u32);
    unsafe { PdhValidatePathExW(hdatasource.unwrap_or(core::mem::zeroed()) as _, szfullpathbuffer.param().abi()) }
}
#[inline]
pub unsafe fn PdhValidatePathW<P0>(szfullpathbuffer: P0) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("pdh.dll" "system" fn PdhValidatePathW(szfullpathbuffer : windows_core::PCWSTR) -> u32);
    unsafe { PdhValidatePathW(szfullpathbuffer.param().abi()) }
}
#[inline]
pub unsafe fn PdhVerifySQLDBA<P0>(szdatasource: P0) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("pdh.dll" "system" fn PdhVerifySQLDBA(szdatasource : windows_core::PCSTR) -> u32);
    unsafe { PdhVerifySQLDBA(szdatasource.param().abi()) }
}
#[inline]
pub unsafe fn PdhVerifySQLDBW<P0>(szdatasource: P0) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("pdh.dll" "system" fn PdhVerifySQLDBW(szdatasource : windows_core::PCWSTR) -> u32);
    unsafe { PdhVerifySQLDBW(szdatasource.param().abi()) }
}
#[inline]
pub unsafe fn PerfAddCounters(hquery: super::super::Foundation::HANDLE, pcounters: *mut PERF_COUNTER_IDENTIFIER, cbcounters: u32) -> u32 {
    windows_core::link!("advapi32.dll" "system" fn PerfAddCounters(hquery : super::super::Foundation:: HANDLE, pcounters : *mut PERF_COUNTER_IDENTIFIER, cbcounters : u32) -> u32);
    unsafe { PerfAddCounters(hquery, pcounters as _, cbcounters) }
}
#[inline]
pub unsafe fn PerfCloseQueryHandle(hquery: super::super::Foundation::HANDLE) -> u32 {
    windows_core::link!("advapi32.dll" "system" fn PerfCloseQueryHandle(hquery : super::super::Foundation:: HANDLE) -> u32);
    unsafe { PerfCloseQueryHandle(hquery) }
}
#[inline]
pub unsafe fn PerfCreateInstance<P2>(providerhandle: super::super::Foundation::HANDLE, countersetguid: *const windows_core::GUID, name: P2, id: u32) -> *mut PERF_COUNTERSET_INSTANCE
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn PerfCreateInstance(providerhandle : super::super::Foundation:: HANDLE, countersetguid : *const windows_core::GUID, name : windows_core::PCWSTR, id : u32) -> *mut PERF_COUNTERSET_INSTANCE);
    unsafe { PerfCreateInstance(providerhandle, countersetguid, name.param().abi(), id) }
}
#[inline]
pub unsafe fn PerfDecrementULongCounterValue(provider: super::super::Foundation::HANDLE, instance: *mut PERF_COUNTERSET_INSTANCE, counterid: u32, value: u32) -> u32 {
    windows_core::link!("advapi32.dll" "system" fn PerfDecrementULongCounterValue(provider : super::super::Foundation:: HANDLE, instance : *mut PERF_COUNTERSET_INSTANCE, counterid : u32, value : u32) -> u32);
    unsafe { PerfDecrementULongCounterValue(provider, instance as _, counterid, value) }
}
#[inline]
pub unsafe fn PerfDecrementULongLongCounterValue(provider: super::super::Foundation::HANDLE, instance: *mut PERF_COUNTERSET_INSTANCE, counterid: u32, value: u64) -> u32 {
    windows_core::link!("advapi32.dll" "system" fn PerfDecrementULongLongCounterValue(provider : super::super::Foundation:: HANDLE, instance : *mut PERF_COUNTERSET_INSTANCE, counterid : u32, value : u64) -> u32);
    unsafe { PerfDecrementULongLongCounterValue(provider, instance as _, counterid, value) }
}
#[inline]
pub unsafe fn PerfDeleteCounters(hquery: super::super::Foundation::HANDLE, pcounters: *mut PERF_COUNTER_IDENTIFIER, cbcounters: u32) -> u32 {
    windows_core::link!("advapi32.dll" "system" fn PerfDeleteCounters(hquery : super::super::Foundation:: HANDLE, pcounters : *mut PERF_COUNTER_IDENTIFIER, cbcounters : u32) -> u32);
    unsafe { PerfDeleteCounters(hquery, pcounters as _, cbcounters) }
}
#[inline]
pub unsafe fn PerfDeleteInstance(provider: super::super::Foundation::HANDLE, instanceblock: *const PERF_COUNTERSET_INSTANCE) -> u32 {
    windows_core::link!("advapi32.dll" "system" fn PerfDeleteInstance(provider : super::super::Foundation:: HANDLE, instanceblock : *const PERF_COUNTERSET_INSTANCE) -> u32);
    unsafe { PerfDeleteInstance(provider, instanceblock) }
}
#[inline]
pub unsafe fn PerfEnumerateCounterSet<P0>(szmachine: P0, pcountersetids: Option<&mut [windows_core::GUID]>, pccountersetidsactual: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn PerfEnumerateCounterSet(szmachine : windows_core::PCWSTR, pcountersetids : *mut windows_core::GUID, ccountersetids : u32, pccountersetidsactual : *mut u32) -> u32);
    unsafe { PerfEnumerateCounterSet(szmachine.param().abi(), core::mem::transmute(pcountersetids.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pcountersetids.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), pccountersetidsactual as _) }
}
#[inline]
pub unsafe fn PerfEnumerateCounterSetInstances<P0>(szmachine: P0, pcountersetid: *const windows_core::GUID, pinstances: Option<*mut PERF_INSTANCE_HEADER>, cbinstances: u32, pcbinstancesactual: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn PerfEnumerateCounterSetInstances(szmachine : windows_core::PCWSTR, pcountersetid : *const windows_core::GUID, pinstances : *mut PERF_INSTANCE_HEADER, cbinstances : u32, pcbinstancesactual : *mut u32) -> u32);
    unsafe { PerfEnumerateCounterSetInstances(szmachine.param().abi(), pcountersetid, pinstances.unwrap_or(core::mem::zeroed()) as _, cbinstances, pcbinstancesactual as _) }
}
#[inline]
pub unsafe fn PerfIncrementULongCounterValue(provider: super::super::Foundation::HANDLE, instance: *mut PERF_COUNTERSET_INSTANCE, counterid: u32, value: u32) -> u32 {
    windows_core::link!("advapi32.dll" "system" fn PerfIncrementULongCounterValue(provider : super::super::Foundation:: HANDLE, instance : *mut PERF_COUNTERSET_INSTANCE, counterid : u32, value : u32) -> u32);
    unsafe { PerfIncrementULongCounterValue(provider, instance as _, counterid, value) }
}
#[inline]
pub unsafe fn PerfIncrementULongLongCounterValue(provider: super::super::Foundation::HANDLE, instance: *mut PERF_COUNTERSET_INSTANCE, counterid: u32, value: u64) -> u32 {
    windows_core::link!("advapi32.dll" "system" fn PerfIncrementULongLongCounterValue(provider : super::super::Foundation:: HANDLE, instance : *mut PERF_COUNTERSET_INSTANCE, counterid : u32, value : u64) -> u32);
    unsafe { PerfIncrementULongLongCounterValue(provider, instance as _, counterid, value) }
}
#[inline]
pub unsafe fn PerfOpenQueryHandle<P0>(szmachine: P0, phquery: *mut super::super::Foundation::HANDLE) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn PerfOpenQueryHandle(szmachine : windows_core::PCWSTR, phquery : *mut super::super::Foundation:: HANDLE) -> u32);
    unsafe { PerfOpenQueryHandle(szmachine.param().abi(), phquery as _) }
}
#[inline]
pub unsafe fn PerfQueryCounterData(hquery: super::super::Foundation::HANDLE, pcounterblock: Option<*mut PERF_DATA_HEADER>, cbcounterblock: u32, pcbcounterblockactual: *mut u32) -> u32 {
    windows_core::link!("advapi32.dll" "system" fn PerfQueryCounterData(hquery : super::super::Foundation:: HANDLE, pcounterblock : *mut PERF_DATA_HEADER, cbcounterblock : u32, pcbcounterblockactual : *mut u32) -> u32);
    unsafe { PerfQueryCounterData(hquery, pcounterblock.unwrap_or(core::mem::zeroed()) as _, cbcounterblock, pcbcounterblockactual as _) }
}
#[inline]
pub unsafe fn PerfQueryCounterInfo(hquery: super::super::Foundation::HANDLE, pcounters: Option<*mut PERF_COUNTER_IDENTIFIER>, cbcounters: u32, pcbcountersactual: *mut u32) -> u32 {
    windows_core::link!("advapi32.dll" "system" fn PerfQueryCounterInfo(hquery : super::super::Foundation:: HANDLE, pcounters : *mut PERF_COUNTER_IDENTIFIER, cbcounters : u32, pcbcountersactual : *mut u32) -> u32);
    unsafe { PerfQueryCounterInfo(hquery, pcounters.unwrap_or(core::mem::zeroed()) as _, cbcounters, pcbcountersactual as _) }
}
#[inline]
pub unsafe fn PerfQueryCounterSetRegistrationInfo<P0>(szmachine: P0, pcountersetid: *const windows_core::GUID, requestcode: PerfRegInfoType, requestlangid: u32, pbreginfo: Option<&mut [u8]>, pcbreginfoactual: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn PerfQueryCounterSetRegistrationInfo(szmachine : windows_core::PCWSTR, pcountersetid : *const windows_core::GUID, requestcode : PerfRegInfoType, requestlangid : u32, pbreginfo : *mut u8, cbreginfo : u32, pcbreginfoactual : *mut u32) -> u32);
    unsafe { PerfQueryCounterSetRegistrationInfo(szmachine.param().abi(), pcountersetid, requestcode, requestlangid, core::mem::transmute(pbreginfo.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pbreginfo.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), pcbreginfoactual as _) }
}
#[inline]
pub unsafe fn PerfQueryInstance<P2>(providerhandle: super::super::Foundation::HANDLE, countersetguid: *const windows_core::GUID, name: P2, id: u32) -> *mut PERF_COUNTERSET_INSTANCE
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn PerfQueryInstance(providerhandle : super::super::Foundation:: HANDLE, countersetguid : *const windows_core::GUID, name : windows_core::PCWSTR, id : u32) -> *mut PERF_COUNTERSET_INSTANCE);
    unsafe { PerfQueryInstance(providerhandle, countersetguid, name.param().abi(), id) }
}
#[inline]
pub unsafe fn PerfSetCounterRefValue(provider: super::super::Foundation::HANDLE, instance: *mut PERF_COUNTERSET_INSTANCE, counterid: u32, address: *const core::ffi::c_void) -> u32 {
    windows_core::link!("advapi32.dll" "system" fn PerfSetCounterRefValue(provider : super::super::Foundation:: HANDLE, instance : *mut PERF_COUNTERSET_INSTANCE, counterid : u32, address : *const core::ffi::c_void) -> u32);
    unsafe { PerfSetCounterRefValue(provider, instance as _, counterid, address) }
}
#[inline]
pub unsafe fn PerfSetCounterSetInfo(providerhandle: super::super::Foundation::HANDLE, template: *mut PERF_COUNTERSET_INFO, templatesize: u32) -> u32 {
    windows_core::link!("advapi32.dll" "system" fn PerfSetCounterSetInfo(providerhandle : super::super::Foundation:: HANDLE, template : *mut PERF_COUNTERSET_INFO, templatesize : u32) -> u32);
    unsafe { PerfSetCounterSetInfo(providerhandle, template as _, templatesize) }
}
#[inline]
pub unsafe fn PerfSetULongCounterValue(provider: super::super::Foundation::HANDLE, instance: *mut PERF_COUNTERSET_INSTANCE, counterid: u32, value: u32) -> u32 {
    windows_core::link!("advapi32.dll" "system" fn PerfSetULongCounterValue(provider : super::super::Foundation:: HANDLE, instance : *mut PERF_COUNTERSET_INSTANCE, counterid : u32, value : u32) -> u32);
    unsafe { PerfSetULongCounterValue(provider, instance as _, counterid, value) }
}
#[inline]
pub unsafe fn PerfSetULongLongCounterValue(provider: super::super::Foundation::HANDLE, instance: *mut PERF_COUNTERSET_INSTANCE, counterid: u32, value: u64) -> u32 {
    windows_core::link!("advapi32.dll" "system" fn PerfSetULongLongCounterValue(provider : super::super::Foundation:: HANDLE, instance : *mut PERF_COUNTERSET_INSTANCE, counterid : u32, value : u64) -> u32);
    unsafe { PerfSetULongLongCounterValue(provider, instance as _, counterid, value) }
}
#[inline]
pub unsafe fn PerfStartProvider(providerguid: *const windows_core::GUID, controlcallback: PERFLIBREQUEST, phprovider: *mut super::super::Foundation::HANDLE) -> u32 {
    windows_core::link!("advapi32.dll" "system" fn PerfStartProvider(providerguid : *const windows_core::GUID, controlcallback : PERFLIBREQUEST, phprovider : *mut super::super::Foundation:: HANDLE) -> u32);
    unsafe { PerfStartProvider(providerguid, controlcallback, phprovider as _) }
}
#[inline]
pub unsafe fn PerfStartProviderEx(providerguid: *const windows_core::GUID, providercontext: Option<*const PERF_PROVIDER_CONTEXT>, provider: *mut super::super::Foundation::HANDLE) -> u32 {
    windows_core::link!("advapi32.dll" "system" fn PerfStartProviderEx(providerguid : *const windows_core::GUID, providercontext : *const PERF_PROVIDER_CONTEXT, provider : *mut super::super::Foundation:: HANDLE) -> u32);
    unsafe { PerfStartProviderEx(providerguid, providercontext.unwrap_or(core::mem::zeroed()) as _, provider as _) }
}
#[inline]
pub unsafe fn PerfStopProvider(providerhandle: super::super::Foundation::HANDLE) -> u32 {
    windows_core::link!("advapi32.dll" "system" fn PerfStopProvider(providerhandle : super::super::Foundation:: HANDLE) -> u32);
    unsafe { PerfStopProvider(providerhandle) }
}
#[inline]
pub unsafe fn QueryPerformanceCounter(lpperformancecount: *mut i64) -> windows_core::Result<()> {
    windows_core::link!("kernel32.dll" "system" fn QueryPerformanceCounter(lpperformancecount : *mut i64) -> windows_core::BOOL);
    unsafe { QueryPerformanceCounter(lpperformancecount as _).ok() }
}
#[inline]
pub unsafe fn QueryPerformanceFrequency(lpfrequency: *mut i64) -> windows_core::Result<()> {
    windows_core::link!("kernel32.dll" "system" fn QueryPerformanceFrequency(lpfrequency : *mut i64) -> windows_core::BOOL);
    unsafe { QueryPerformanceFrequency(lpfrequency as _).ok() }
}
#[inline]
pub unsafe fn RestorePerfRegistryFromFileW<P0, P1>(szfilename: P0, szlangid: P1) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("loadperf.dll" "system" fn RestorePerfRegistryFromFileW(szfilename : windows_core::PCWSTR, szlangid : windows_core::PCWSTR) -> u32);
    unsafe { RestorePerfRegistryFromFileW(szfilename.param().abi(), szlangid.param().abi()) }
}
#[inline]
pub unsafe fn SetServiceAsTrustedA<P0, P1>(szreserved: P0, szservicename: P1) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("loadperf.dll" "system" fn SetServiceAsTrustedA(szreserved : windows_core::PCSTR, szservicename : windows_core::PCSTR) -> u32);
    unsafe { SetServiceAsTrustedA(szreserved.param().abi(), szservicename.param().abi()) }
}
#[inline]
pub unsafe fn SetServiceAsTrustedW<P0, P1>(szreserved: P0, szservicename: P1) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("loadperf.dll" "system" fn SetServiceAsTrustedW(szreserved : windows_core::PCWSTR, szservicename : windows_core::PCWSTR) -> u32);
    unsafe { SetServiceAsTrustedW(szreserved.param().abi(), szservicename.param().abi()) }
}
#[inline]
pub unsafe fn UnloadPerfCounterTextStringsA<P0>(lpcommandline: P0, bquietmodearg: bool) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("loadperf.dll" "system" fn UnloadPerfCounterTextStringsA(lpcommandline : windows_core::PCSTR, bquietmodearg : windows_core::BOOL) -> u32);
    unsafe { UnloadPerfCounterTextStringsA(lpcommandline.param().abi(), bquietmodearg.into()) }
}
#[inline]
pub unsafe fn UnloadPerfCounterTextStringsW<P0>(lpcommandline: P0, bquietmodearg: bool) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("loadperf.dll" "system" fn UnloadPerfCounterTextStringsW(lpcommandline : windows_core::PCWSTR, bquietmodearg : windows_core::BOOL) -> u32);
    unsafe { UnloadPerfCounterTextStringsW(lpcommandline.param().abi(), bquietmodearg.into()) }
}
#[inline]
pub unsafe fn UpdatePerfNameFilesA<P0, P1, P2>(sznewctrfilepath: P0, sznewhlpfilepath: P1, szlanguageid: P2, dwflags: usize) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("loadperf.dll" "system" fn UpdatePerfNameFilesA(sznewctrfilepath : windows_core::PCSTR, sznewhlpfilepath : windows_core::PCSTR, szlanguageid : windows_core::PCSTR, dwflags : usize) -> u32);
    unsafe { UpdatePerfNameFilesA(sznewctrfilepath.param().abi(), sznewhlpfilepath.param().abi(), szlanguageid.param().abi(), dwflags) }
}
#[inline]
pub unsafe fn UpdatePerfNameFilesW<P0, P1, P2>(sznewctrfilepath: P0, sznewhlpfilepath: P1, szlanguageid: P2, dwflags: usize) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("loadperf.dll" "system" fn UpdatePerfNameFilesW(sznewctrfilepath : windows_core::PCWSTR, sznewhlpfilepath : windows_core::PCWSTR, szlanguageid : windows_core::PCWSTR, dwflags : usize) -> u32);
    unsafe { UpdatePerfNameFilesW(sznewctrfilepath.param().abi(), sznewhlpfilepath.param().abi(), szlanguageid.param().abi(), dwflags) }
}
pub const AppearPropPage: windows_core::GUID = windows_core::GUID::from_u128(0xe49741e9_93a8_4ab1_8e96_bf4482282e9c);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct AutoPathFormat(pub i32);
pub const BootTraceSession: windows_core::GUID = windows_core::GUID::from_u128(0x03837538_098b_11d8_9414_505054503030);
pub const BootTraceSessionCollection: windows_core::GUID = windows_core::GUID::from_u128(0x03837539_098b_11d8_9414_505054503030);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ClockType(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CommitMode(pub i32);
pub const CounterItem: windows_core::GUID = windows_core::GUID::from_u128(0xc4d2d8e0_d1dd_11ce_940f_008029004348);
pub const CounterItem2: windows_core::GUID = windows_core::GUID::from_u128(0x43196c62_c31f_4ce3_a02e_79efe0f6a525);
pub type CounterPathCallBack = Option<unsafe extern "system" fn(param0: usize) -> i32>;
pub const CounterPropPage: windows_core::GUID = windows_core::GUID::from_u128(0xcf948561_ede8_11ce_941e_008029004347);
pub const Counters: windows_core::GUID = windows_core::GUID::from_u128(0xb2b066d2_2aac_11cf_942f_008029004347);
pub const DATA_SOURCE_REGISTRY: REAL_TIME_DATA_SOURCE_ID_FLAGS = REAL_TIME_DATA_SOURCE_ID_FLAGS(1u32);
pub const DATA_SOURCE_WBEM: REAL_TIME_DATA_SOURCE_ID_FLAGS = REAL_TIME_DATA_SOURCE_ID_FLAGS(4u32);
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(DICounterItem, DICounterItem_Vtbl, 0xc08c4ff2_0e2e_11cf_942c_008029004347);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for DICounterItem {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(DICounterItem, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct DICounterItem_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait DICounterItem_Impl: super::Com::IDispatch_Impl {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl DICounterItem_Vtbl {
    pub const fn new<Identity: DICounterItem_Impl, const OFFSET: isize>() -> Self {
        Self { base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<DICounterItem as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for DICounterItem {}
pub const DIID_DICounterItem: windows_core::GUID = windows_core::GUID::from_u128(0xc08c4ff2_0e2e_11cf_942c_008029004347);
pub const DIID_DILogFileItem: windows_core::GUID = windows_core::GUID::from_u128(0x8d093ffc_f777_4917_82d1_833fbc54c58f);
pub const DIID_DISystemMonitor: windows_core::GUID = windows_core::GUID::from_u128(0x13d73d81_c32e_11cf_9398_00aa00a3ddea);
pub const DIID_DISystemMonitorEvents: windows_core::GUID = windows_core::GUID::from_u128(0x84979930_4ab3_11cf_943a_008029004347);
pub const DIID_DISystemMonitorInternal: windows_core::GUID = windows_core::GUID::from_u128(0x194eb242_c32c_11cf_9398_00aa00a3ddea);
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(DILogFileItem, DILogFileItem_Vtbl, 0x8d093ffc_f777_4917_82d1_833fbc54c58f);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for DILogFileItem {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(DILogFileItem, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct DILogFileItem_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait DILogFileItem_Impl: super::Com::IDispatch_Impl {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl DILogFileItem_Vtbl {
    pub const fn new<Identity: DILogFileItem_Impl, const OFFSET: isize>() -> Self {
        Self { base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<DILogFileItem as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for DILogFileItem {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(DISystemMonitor, DISystemMonitor_Vtbl, 0x13d73d81_c32e_11cf_9398_00aa00a3ddea);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for DISystemMonitor {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(DISystemMonitor, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct DISystemMonitor_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait DISystemMonitor_Impl: super::Com::IDispatch_Impl {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl DISystemMonitor_Vtbl {
    pub const fn new<Identity: DISystemMonitor_Impl, const OFFSET: isize>() -> Self {
        Self { base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<DISystemMonitor as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for DISystemMonitor {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(DISystemMonitorEvents, DISystemMonitorEvents_Vtbl, 0x84979930_4ab3_11cf_943a_008029004347);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for DISystemMonitorEvents {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(DISystemMonitorEvents, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct DISystemMonitorEvents_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait DISystemMonitorEvents_Impl: super::Com::IDispatch_Impl {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl DISystemMonitorEvents_Vtbl {
    pub const fn new<Identity: DISystemMonitorEvents_Impl, const OFFSET: isize>() -> Self {
        Self { base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<DISystemMonitorEvents as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for DISystemMonitorEvents {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(DISystemMonitorInternal, DISystemMonitorInternal_Vtbl, 0x194eb242_c32c_11cf_9398_00aa00a3ddea);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for DISystemMonitorInternal {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(DISystemMonitorInternal, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct DISystemMonitorInternal_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait DISystemMonitorInternal_Impl: super::Com::IDispatch_Impl {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl DISystemMonitorInternal_Vtbl {
    pub const fn new<Identity: DISystemMonitorInternal_Impl, const OFFSET: isize>() -> Self {
        Self { base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<DISystemMonitorInternal as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for DISystemMonitorInternal {}
pub const DataCollectorSet: windows_core::GUID = windows_core::GUID::from_u128(0x03837521_098b_11d8_9414_505054503030);
pub const DataCollectorSetCollection: windows_core::GUID = windows_core::GUID::from_u128(0x03837525_098b_11d8_9414_505054503030);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DataCollectorSetStatus(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DataCollectorType(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DataManagerSteps(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DataSourceTypeConstants(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DisplayTypeConstants(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FileFormat(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FolderActionSteps(pub i32);
pub const GeneralPropPage: windows_core::GUID = windows_core::GUID::from_u128(0xc3e5d3d2_1a03_11cf_942d_008029004347);
pub const GraphPropPage: windows_core::GUID = windows_core::GUID::from_u128(0xc3e5d3d3_1a03_11cf_942d_008029004347);
pub const H_WBEM_DATASOURCE: i32 = -1i32;
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IAlertDataCollector, IAlertDataCollector_Vtbl, 0x03837516_098b_11d8_9414_505054503030);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IAlertDataCollector {
    type Target = IDataCollector;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IAlertDataCollector, windows_core::IUnknown, super::Com::IDispatch, IDataCollector);
#[cfg(feature = "Win32_System_Com")]
impl IAlertDataCollector {
    pub unsafe fn AlertThresholds(&self) -> windows_core::Result<*mut super::Com::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AlertThresholds)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetAlertThresholds(&self, alerts: *const super::Com::SAFEARRAY) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetAlertThresholds)(windows_core::Interface::as_raw(self), alerts).ok() }
    }
    pub unsafe fn EventLog(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EventLog)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetEventLog(&self, log: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetEventLog)(windows_core::Interface::as_raw(self), log).ok() }
    }
    pub unsafe fn SampleInterval(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SampleInterval)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetSampleInterval(&self, interval: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetSampleInterval)(windows_core::Interface::as_raw(self), interval).ok() }
    }
    pub unsafe fn Task(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Task)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetTask(&self, task: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetTask)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(task)).ok() }
    }
    pub unsafe fn TaskRunAsSelf(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).TaskRunAsSelf)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetTaskRunAsSelf(&self, runasself: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetTaskRunAsSelf)(windows_core::Interface::as_raw(self), runasself).ok() }
    }
    pub unsafe fn TaskArguments(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).TaskArguments)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetTaskArguments(&self, task: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetTaskArguments)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(task)).ok() }
    }
    pub unsafe fn TaskUserTextArguments(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).TaskUserTextArguments)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetTaskUserTextArguments(&self, task: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetTaskUserTextArguments)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(task)).ok() }
    }
    pub unsafe fn TriggerDataCollectorSet(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).TriggerDataCollectorSet)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetTriggerDataCollectorSet(&self, name: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetTriggerDataCollectorSet)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(name)).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IAlertDataCollector_Vtbl {
    pub base__: IDataCollector_Vtbl,
    pub AlertThresholds: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::Com::SAFEARRAY) -> windows_core::HRESULT,
    pub SetAlertThresholds: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::Com::SAFEARRAY) -> windows_core::HRESULT,
    pub EventLog: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetEventLog: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SampleInterval: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetSampleInterval: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Task: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetTask: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TaskRunAsSelf: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetTaskRunAsSelf: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub TaskArguments: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetTaskArguments: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TaskUserTextArguments: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetTaskUserTextArguments: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TriggerDataCollectorSet: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetTriggerDataCollectorSet: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IAlertDataCollector_Impl: IDataCollector_Impl {
    fn AlertThresholds(&self) -> windows_core::Result<*mut super::Com::SAFEARRAY>;
    fn SetAlertThresholds(&self, alerts: *const super::Com::SAFEARRAY) -> windows_core::Result<()>;
    fn EventLog(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetEventLog(&self, log: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn SampleInterval(&self) -> windows_core::Result<u32>;
    fn SetSampleInterval(&self, interval: u32) -> windows_core::Result<()>;
    fn Task(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetTask(&self, task: &windows_core::BSTR) -> windows_core::Result<()>;
    fn TaskRunAsSelf(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetTaskRunAsSelf(&self, runasself: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn TaskArguments(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetTaskArguments(&self, task: &windows_core::BSTR) -> windows_core::Result<()>;
    fn TaskUserTextArguments(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetTaskUserTextArguments(&self, task: &windows_core::BSTR) -> windows_core::Result<()>;
    fn TriggerDataCollectorSet(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetTriggerDataCollectorSet(&self, name: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IAlertDataCollector_Vtbl {
    pub const fn new<Identity: IAlertDataCollector_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AlertThresholds<Identity: IAlertDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, alerts: *mut *mut super::Com::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAlertDataCollector_Impl::AlertThresholds(this) {
                    Ok(ok__) => {
                        alerts.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetAlertThresholds<Identity: IAlertDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, alerts: *const super::Com::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAlertDataCollector_Impl::SetAlertThresholds(this, core::mem::transmute_copy(&alerts)).into()
            }
        }
        unsafe extern "system" fn EventLog<Identity: IAlertDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, log: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAlertDataCollector_Impl::EventLog(this) {
                    Ok(ok__) => {
                        log.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetEventLog<Identity: IAlertDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, log: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAlertDataCollector_Impl::SetEventLog(this, core::mem::transmute_copy(&log)).into()
            }
        }
        unsafe extern "system" fn SampleInterval<Identity: IAlertDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, interval: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAlertDataCollector_Impl::SampleInterval(this) {
                    Ok(ok__) => {
                        interval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetSampleInterval<Identity: IAlertDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, interval: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAlertDataCollector_Impl::SetSampleInterval(this, core::mem::transmute_copy(&interval)).into()
            }
        }
        unsafe extern "system" fn Task<Identity: IAlertDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, task: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAlertDataCollector_Impl::Task(this) {
                    Ok(ok__) => {
                        task.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetTask<Identity: IAlertDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, task: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAlertDataCollector_Impl::SetTask(this, core::mem::transmute(&task)).into()
            }
        }
        unsafe extern "system" fn TaskRunAsSelf<Identity: IAlertDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, runasself: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAlertDataCollector_Impl::TaskRunAsSelf(this) {
                    Ok(ok__) => {
                        runasself.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetTaskRunAsSelf<Identity: IAlertDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, runasself: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAlertDataCollector_Impl::SetTaskRunAsSelf(this, core::mem::transmute_copy(&runasself)).into()
            }
        }
        unsafe extern "system" fn TaskArguments<Identity: IAlertDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, task: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAlertDataCollector_Impl::TaskArguments(this) {
                    Ok(ok__) => {
                        task.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetTaskArguments<Identity: IAlertDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, task: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAlertDataCollector_Impl::SetTaskArguments(this, core::mem::transmute(&task)).into()
            }
        }
        unsafe extern "system" fn TaskUserTextArguments<Identity: IAlertDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, task: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAlertDataCollector_Impl::TaskUserTextArguments(this) {
                    Ok(ok__) => {
                        task.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetTaskUserTextArguments<Identity: IAlertDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, task: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAlertDataCollector_Impl::SetTaskUserTextArguments(this, core::mem::transmute(&task)).into()
            }
        }
        unsafe extern "system" fn TriggerDataCollectorSet<Identity: IAlertDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAlertDataCollector_Impl::TriggerDataCollectorSet(this) {
                    Ok(ok__) => {
                        name.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetTriggerDataCollectorSet<Identity: IAlertDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAlertDataCollector_Impl::SetTriggerDataCollectorSet(this, core::mem::transmute(&name)).into()
            }
        }
        Self {
            base__: IDataCollector_Vtbl::new::<Identity, OFFSET>(),
            AlertThresholds: AlertThresholds::<Identity, OFFSET>,
            SetAlertThresholds: SetAlertThresholds::<Identity, OFFSET>,
            EventLog: EventLog::<Identity, OFFSET>,
            SetEventLog: SetEventLog::<Identity, OFFSET>,
            SampleInterval: SampleInterval::<Identity, OFFSET>,
            SetSampleInterval: SetSampleInterval::<Identity, OFFSET>,
            Task: Task::<Identity, OFFSET>,
            SetTask: SetTask::<Identity, OFFSET>,
            TaskRunAsSelf: TaskRunAsSelf::<Identity, OFFSET>,
            SetTaskRunAsSelf: SetTaskRunAsSelf::<Identity, OFFSET>,
            TaskArguments: TaskArguments::<Identity, OFFSET>,
            SetTaskArguments: SetTaskArguments::<Identity, OFFSET>,
            TaskUserTextArguments: TaskUserTextArguments::<Identity, OFFSET>,
            SetTaskUserTextArguments: SetTaskUserTextArguments::<Identity, OFFSET>,
            TriggerDataCollectorSet: TriggerDataCollectorSet::<Identity, OFFSET>,
            SetTriggerDataCollectorSet: SetTriggerDataCollectorSet::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAlertDataCollector as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID || iid == &<IDataCollector as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IAlertDataCollector {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IApiTracingDataCollector, IApiTracingDataCollector_Vtbl, 0x0383751a_098b_11d8_9414_505054503030);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IApiTracingDataCollector {
    type Target = IDataCollector;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IApiTracingDataCollector, windows_core::IUnknown, super::Com::IDispatch, IDataCollector);
#[cfg(feature = "Win32_System_Com")]
impl IApiTracingDataCollector {
    pub unsafe fn LogApiNamesOnly(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).LogApiNamesOnly)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetLogApiNamesOnly(&self, logapinames: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetLogApiNamesOnly)(windows_core::Interface::as_raw(self), logapinames).ok() }
    }
    pub unsafe fn LogApisRecursively(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).LogApisRecursively)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetLogApisRecursively(&self, logrecursively: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetLogApisRecursively)(windows_core::Interface::as_raw(self), logrecursively).ok() }
    }
    pub unsafe fn ExePath(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ExePath)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetExePath(&self, exepath: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetExePath)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(exepath)).ok() }
    }
    pub unsafe fn LogFilePath(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).LogFilePath)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetLogFilePath(&self, logfilepath: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetLogFilePath)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(logfilepath)).ok() }
    }
    pub unsafe fn IncludeModules(&self) -> windows_core::Result<*mut super::Com::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IncludeModules)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetIncludeModules(&self, includemodules: *const super::Com::SAFEARRAY) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetIncludeModules)(windows_core::Interface::as_raw(self), includemodules).ok() }
    }
    pub unsafe fn IncludeApis(&self) -> windows_core::Result<*mut super::Com::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IncludeApis)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetIncludeApis(&self, includeapis: *const super::Com::SAFEARRAY) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetIncludeApis)(windows_core::Interface::as_raw(self), includeapis).ok() }
    }
    pub unsafe fn ExcludeApis(&self) -> windows_core::Result<*mut super::Com::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ExcludeApis)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetExcludeApis(&self, excludeapis: *const super::Com::SAFEARRAY) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetExcludeApis)(windows_core::Interface::as_raw(self), excludeapis).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IApiTracingDataCollector_Vtbl {
    pub base__: IDataCollector_Vtbl,
    pub LogApiNamesOnly: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetLogApiNamesOnly: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub LogApisRecursively: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetLogApisRecursively: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub ExePath: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetExePath: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub LogFilePath: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetLogFilePath: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IncludeModules: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::Com::SAFEARRAY) -> windows_core::HRESULT,
    pub SetIncludeModules: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::Com::SAFEARRAY) -> windows_core::HRESULT,
    pub IncludeApis: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::Com::SAFEARRAY) -> windows_core::HRESULT,
    pub SetIncludeApis: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::Com::SAFEARRAY) -> windows_core::HRESULT,
    pub ExcludeApis: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::Com::SAFEARRAY) -> windows_core::HRESULT,
    pub SetExcludeApis: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::Com::SAFEARRAY) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IApiTracingDataCollector_Impl: IDataCollector_Impl {
    fn LogApiNamesOnly(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetLogApiNamesOnly(&self, logapinames: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn LogApisRecursively(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetLogApisRecursively(&self, logrecursively: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn ExePath(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetExePath(&self, exepath: &windows_core::BSTR) -> windows_core::Result<()>;
    fn LogFilePath(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetLogFilePath(&self, logfilepath: &windows_core::BSTR) -> windows_core::Result<()>;
    fn IncludeModules(&self) -> windows_core::Result<*mut super::Com::SAFEARRAY>;
    fn SetIncludeModules(&self, includemodules: *const super::Com::SAFEARRAY) -> windows_core::Result<()>;
    fn IncludeApis(&self) -> windows_core::Result<*mut super::Com::SAFEARRAY>;
    fn SetIncludeApis(&self, includeapis: *const super::Com::SAFEARRAY) -> windows_core::Result<()>;
    fn ExcludeApis(&self) -> windows_core::Result<*mut super::Com::SAFEARRAY>;
    fn SetExcludeApis(&self, excludeapis: *const super::Com::SAFEARRAY) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IApiTracingDataCollector_Vtbl {
    pub const fn new<Identity: IApiTracingDataCollector_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn LogApiNamesOnly<Identity: IApiTracingDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, logapinames: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IApiTracingDataCollector_Impl::LogApiNamesOnly(this) {
                    Ok(ok__) => {
                        logapinames.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetLogApiNamesOnly<Identity: IApiTracingDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, logapinames: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IApiTracingDataCollector_Impl::SetLogApiNamesOnly(this, core::mem::transmute_copy(&logapinames)).into()
            }
        }
        unsafe extern "system" fn LogApisRecursively<Identity: IApiTracingDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, logrecursively: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IApiTracingDataCollector_Impl::LogApisRecursively(this) {
                    Ok(ok__) => {
                        logrecursively.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetLogApisRecursively<Identity: IApiTracingDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, logrecursively: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IApiTracingDataCollector_Impl::SetLogApisRecursively(this, core::mem::transmute_copy(&logrecursively)).into()
            }
        }
        unsafe extern "system" fn ExePath<Identity: IApiTracingDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, exepath: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IApiTracingDataCollector_Impl::ExePath(this) {
                    Ok(ok__) => {
                        exepath.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetExePath<Identity: IApiTracingDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, exepath: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IApiTracingDataCollector_Impl::SetExePath(this, core::mem::transmute(&exepath)).into()
            }
        }
        unsafe extern "system" fn LogFilePath<Identity: IApiTracingDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, logfilepath: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IApiTracingDataCollector_Impl::LogFilePath(this) {
                    Ok(ok__) => {
                        logfilepath.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetLogFilePath<Identity: IApiTracingDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, logfilepath: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IApiTracingDataCollector_Impl::SetLogFilePath(this, core::mem::transmute(&logfilepath)).into()
            }
        }
        unsafe extern "system" fn IncludeModules<Identity: IApiTracingDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, includemodules: *mut *mut super::Com::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IApiTracingDataCollector_Impl::IncludeModules(this) {
                    Ok(ok__) => {
                        includemodules.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetIncludeModules<Identity: IApiTracingDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, includemodules: *const super::Com::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IApiTracingDataCollector_Impl::SetIncludeModules(this, core::mem::transmute_copy(&includemodules)).into()
            }
        }
        unsafe extern "system" fn IncludeApis<Identity: IApiTracingDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, includeapis: *mut *mut super::Com::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IApiTracingDataCollector_Impl::IncludeApis(this) {
                    Ok(ok__) => {
                        includeapis.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetIncludeApis<Identity: IApiTracingDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, includeapis: *const super::Com::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IApiTracingDataCollector_Impl::SetIncludeApis(this, core::mem::transmute_copy(&includeapis)).into()
            }
        }
        unsafe extern "system" fn ExcludeApis<Identity: IApiTracingDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, excludeapis: *mut *mut super::Com::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IApiTracingDataCollector_Impl::ExcludeApis(this) {
                    Ok(ok__) => {
                        excludeapis.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetExcludeApis<Identity: IApiTracingDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, excludeapis: *const super::Com::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IApiTracingDataCollector_Impl::SetExcludeApis(this, core::mem::transmute_copy(&excludeapis)).into()
            }
        }
        Self {
            base__: IDataCollector_Vtbl::new::<Identity, OFFSET>(),
            LogApiNamesOnly: LogApiNamesOnly::<Identity, OFFSET>,
            SetLogApiNamesOnly: SetLogApiNamesOnly::<Identity, OFFSET>,
            LogApisRecursively: LogApisRecursively::<Identity, OFFSET>,
            SetLogApisRecursively: SetLogApisRecursively::<Identity, OFFSET>,
            ExePath: ExePath::<Identity, OFFSET>,
            SetExePath: SetExePath::<Identity, OFFSET>,
            LogFilePath: LogFilePath::<Identity, OFFSET>,
            SetLogFilePath: SetLogFilePath::<Identity, OFFSET>,
            IncludeModules: IncludeModules::<Identity, OFFSET>,
            SetIncludeModules: SetIncludeModules::<Identity, OFFSET>,
            IncludeApis: IncludeApis::<Identity, OFFSET>,
            SetIncludeApis: SetIncludeApis::<Identity, OFFSET>,
            ExcludeApis: ExcludeApis::<Identity, OFFSET>,
            SetExcludeApis: SetExcludeApis::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IApiTracingDataCollector as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID || iid == &<IDataCollector as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IApiTracingDataCollector {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IConfigurationDataCollector, IConfigurationDataCollector_Vtbl, 0x03837514_098b_11d8_9414_505054503030);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IConfigurationDataCollector {
    type Target = IDataCollector;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IConfigurationDataCollector, windows_core::IUnknown, super::Com::IDispatch, IDataCollector);
#[cfg(feature = "Win32_System_Com")]
impl IConfigurationDataCollector {
    pub unsafe fn FileMaxCount(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FileMaxCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetFileMaxCount(&self, count: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetFileMaxCount)(windows_core::Interface::as_raw(self), count).ok() }
    }
    pub unsafe fn FileMaxRecursiveDepth(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FileMaxRecursiveDepth)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetFileMaxRecursiveDepth(&self, depth: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetFileMaxRecursiveDepth)(windows_core::Interface::as_raw(self), depth).ok() }
    }
    pub unsafe fn FileMaxTotalSize(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FileMaxTotalSize)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetFileMaxTotalSize(&self, size: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetFileMaxTotalSize)(windows_core::Interface::as_raw(self), size).ok() }
    }
    pub unsafe fn Files(&self) -> windows_core::Result<*mut super::Com::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Files)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetFiles(&self, files: *const super::Com::SAFEARRAY) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetFiles)(windows_core::Interface::as_raw(self), files).ok() }
    }
    pub unsafe fn ManagementQueries(&self) -> windows_core::Result<*mut super::Com::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ManagementQueries)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetManagementQueries(&self, queries: *const super::Com::SAFEARRAY) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetManagementQueries)(windows_core::Interface::as_raw(self), queries).ok() }
    }
    pub unsafe fn QueryNetworkAdapters(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).QueryNetworkAdapters)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetQueryNetworkAdapters(&self, network: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetQueryNetworkAdapters)(windows_core::Interface::as_raw(self), network).ok() }
    }
    pub unsafe fn RegistryKeys(&self) -> windows_core::Result<*mut super::Com::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RegistryKeys)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetRegistryKeys(&self, query: *const super::Com::SAFEARRAY) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetRegistryKeys)(windows_core::Interface::as_raw(self), query).ok() }
    }
    pub unsafe fn RegistryMaxRecursiveDepth(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RegistryMaxRecursiveDepth)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetRegistryMaxRecursiveDepth(&self, depth: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetRegistryMaxRecursiveDepth)(windows_core::Interface::as_raw(self), depth).ok() }
    }
    pub unsafe fn SystemStateFile(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SystemStateFile)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetSystemStateFile(&self, filename: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetSystemStateFile)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(filename)).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IConfigurationDataCollector_Vtbl {
    pub base__: IDataCollector_Vtbl,
    pub FileMaxCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetFileMaxCount: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub FileMaxRecursiveDepth: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetFileMaxRecursiveDepth: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub FileMaxTotalSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetFileMaxTotalSize: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Files: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::Com::SAFEARRAY) -> windows_core::HRESULT,
    pub SetFiles: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::Com::SAFEARRAY) -> windows_core::HRESULT,
    pub ManagementQueries: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::Com::SAFEARRAY) -> windows_core::HRESULT,
    pub SetManagementQueries: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::Com::SAFEARRAY) -> windows_core::HRESULT,
    pub QueryNetworkAdapters: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetQueryNetworkAdapters: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub RegistryKeys: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::Com::SAFEARRAY) -> windows_core::HRESULT,
    pub SetRegistryKeys: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::Com::SAFEARRAY) -> windows_core::HRESULT,
    pub RegistryMaxRecursiveDepth: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetRegistryMaxRecursiveDepth: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub SystemStateFile: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetSystemStateFile: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IConfigurationDataCollector_Impl: IDataCollector_Impl {
    fn FileMaxCount(&self) -> windows_core::Result<u32>;
    fn SetFileMaxCount(&self, count: u32) -> windows_core::Result<()>;
    fn FileMaxRecursiveDepth(&self) -> windows_core::Result<u32>;
    fn SetFileMaxRecursiveDepth(&self, depth: u32) -> windows_core::Result<()>;
    fn FileMaxTotalSize(&self) -> windows_core::Result<u32>;
    fn SetFileMaxTotalSize(&self, size: u32) -> windows_core::Result<()>;
    fn Files(&self) -> windows_core::Result<*mut super::Com::SAFEARRAY>;
    fn SetFiles(&self, files: *const super::Com::SAFEARRAY) -> windows_core::Result<()>;
    fn ManagementQueries(&self) -> windows_core::Result<*mut super::Com::SAFEARRAY>;
    fn SetManagementQueries(&self, queries: *const super::Com::SAFEARRAY) -> windows_core::Result<()>;
    fn QueryNetworkAdapters(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetQueryNetworkAdapters(&self, network: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn RegistryKeys(&self) -> windows_core::Result<*mut super::Com::SAFEARRAY>;
    fn SetRegistryKeys(&self, query: *const super::Com::SAFEARRAY) -> windows_core::Result<()>;
    fn RegistryMaxRecursiveDepth(&self) -> windows_core::Result<u32>;
    fn SetRegistryMaxRecursiveDepth(&self, depth: u32) -> windows_core::Result<()>;
    fn SystemStateFile(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetSystemStateFile(&self, filename: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IConfigurationDataCollector_Vtbl {
    pub const fn new<Identity: IConfigurationDataCollector_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn FileMaxCount<Identity: IConfigurationDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, count: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IConfigurationDataCollector_Impl::FileMaxCount(this) {
                    Ok(ok__) => {
                        count.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetFileMaxCount<Identity: IConfigurationDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, count: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IConfigurationDataCollector_Impl::SetFileMaxCount(this, core::mem::transmute_copy(&count)).into()
            }
        }
        unsafe extern "system" fn FileMaxRecursiveDepth<Identity: IConfigurationDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, depth: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IConfigurationDataCollector_Impl::FileMaxRecursiveDepth(this) {
                    Ok(ok__) => {
                        depth.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetFileMaxRecursiveDepth<Identity: IConfigurationDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, depth: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IConfigurationDataCollector_Impl::SetFileMaxRecursiveDepth(this, core::mem::transmute_copy(&depth)).into()
            }
        }
        unsafe extern "system" fn FileMaxTotalSize<Identity: IConfigurationDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, size: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IConfigurationDataCollector_Impl::FileMaxTotalSize(this) {
                    Ok(ok__) => {
                        size.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetFileMaxTotalSize<Identity: IConfigurationDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, size: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IConfigurationDataCollector_Impl::SetFileMaxTotalSize(this, core::mem::transmute_copy(&size)).into()
            }
        }
        unsafe extern "system" fn Files<Identity: IConfigurationDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, files: *mut *mut super::Com::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IConfigurationDataCollector_Impl::Files(this) {
                    Ok(ok__) => {
                        files.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetFiles<Identity: IConfigurationDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, files: *const super::Com::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IConfigurationDataCollector_Impl::SetFiles(this, core::mem::transmute_copy(&files)).into()
            }
        }
        unsafe extern "system" fn ManagementQueries<Identity: IConfigurationDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, queries: *mut *mut super::Com::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IConfigurationDataCollector_Impl::ManagementQueries(this) {
                    Ok(ok__) => {
                        queries.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetManagementQueries<Identity: IConfigurationDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, queries: *const super::Com::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IConfigurationDataCollector_Impl::SetManagementQueries(this, core::mem::transmute_copy(&queries)).into()
            }
        }
        unsafe extern "system" fn QueryNetworkAdapters<Identity: IConfigurationDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, network: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IConfigurationDataCollector_Impl::QueryNetworkAdapters(this) {
                    Ok(ok__) => {
                        network.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetQueryNetworkAdapters<Identity: IConfigurationDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, network: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IConfigurationDataCollector_Impl::SetQueryNetworkAdapters(this, core::mem::transmute_copy(&network)).into()
            }
        }
        unsafe extern "system" fn RegistryKeys<Identity: IConfigurationDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, query: *mut *mut super::Com::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IConfigurationDataCollector_Impl::RegistryKeys(this) {
                    Ok(ok__) => {
                        query.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetRegistryKeys<Identity: IConfigurationDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, query: *const super::Com::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IConfigurationDataCollector_Impl::SetRegistryKeys(this, core::mem::transmute_copy(&query)).into()
            }
        }
        unsafe extern "system" fn RegistryMaxRecursiveDepth<Identity: IConfigurationDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, depth: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IConfigurationDataCollector_Impl::RegistryMaxRecursiveDepth(this) {
                    Ok(ok__) => {
                        depth.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetRegistryMaxRecursiveDepth<Identity: IConfigurationDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, depth: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IConfigurationDataCollector_Impl::SetRegistryMaxRecursiveDepth(this, core::mem::transmute_copy(&depth)).into()
            }
        }
        unsafe extern "system" fn SystemStateFile<Identity: IConfigurationDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, filename: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IConfigurationDataCollector_Impl::SystemStateFile(this) {
                    Ok(ok__) => {
                        filename.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetSystemStateFile<Identity: IConfigurationDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, filename: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IConfigurationDataCollector_Impl::SetSystemStateFile(this, core::mem::transmute(&filename)).into()
            }
        }
        Self {
            base__: IDataCollector_Vtbl::new::<Identity, OFFSET>(),
            FileMaxCount: FileMaxCount::<Identity, OFFSET>,
            SetFileMaxCount: SetFileMaxCount::<Identity, OFFSET>,
            FileMaxRecursiveDepth: FileMaxRecursiveDepth::<Identity, OFFSET>,
            SetFileMaxRecursiveDepth: SetFileMaxRecursiveDepth::<Identity, OFFSET>,
            FileMaxTotalSize: FileMaxTotalSize::<Identity, OFFSET>,
            SetFileMaxTotalSize: SetFileMaxTotalSize::<Identity, OFFSET>,
            Files: Files::<Identity, OFFSET>,
            SetFiles: SetFiles::<Identity, OFFSET>,
            ManagementQueries: ManagementQueries::<Identity, OFFSET>,
            SetManagementQueries: SetManagementQueries::<Identity, OFFSET>,
            QueryNetworkAdapters: QueryNetworkAdapters::<Identity, OFFSET>,
            SetQueryNetworkAdapters: SetQueryNetworkAdapters::<Identity, OFFSET>,
            RegistryKeys: RegistryKeys::<Identity, OFFSET>,
            SetRegistryKeys: SetRegistryKeys::<Identity, OFFSET>,
            RegistryMaxRecursiveDepth: RegistryMaxRecursiveDepth::<Identity, OFFSET>,
            SetRegistryMaxRecursiveDepth: SetRegistryMaxRecursiveDepth::<Identity, OFFSET>,
            SystemStateFile: SystemStateFile::<Identity, OFFSET>,
            SetSystemStateFile: SetSystemStateFile::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IConfigurationDataCollector as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID || iid == &<IDataCollector as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IConfigurationDataCollector {}
windows_core::imp::define_interface!(ICounterItem, ICounterItem_Vtbl, 0x771a9520_ee28_11ce_941e_008029004347);
windows_core::imp::interface_hierarchy!(ICounterItem, windows_core::IUnknown);
impl ICounterItem {
    pub unsafe fn Value(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Value)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetColor(&self, color: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetColor)(windows_core::Interface::as_raw(self), color).ok() }
    }
    pub unsafe fn Color(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Color)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetWidth(&self, iwidth: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetWidth)(windows_core::Interface::as_raw(self), iwidth).ok() }
    }
    pub unsafe fn Width(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Width)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetLineStyle(&self, ilinestyle: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetLineStyle)(windows_core::Interface::as_raw(self), ilinestyle).ok() }
    }
    pub unsafe fn LineStyle(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).LineStyle)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetScaleFactor(&self, iscale: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetScaleFactor)(windows_core::Interface::as_raw(self), iscale).ok() }
    }
    pub unsafe fn ScaleFactor(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ScaleFactor)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Path(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Path)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn GetValue(&self, value: *mut f64, status: *mut i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetValue)(windows_core::Interface::as_raw(self), value as _, status as _).ok() }
    }
    pub unsafe fn GetStatistics(&self, max: *mut f64, min: *mut f64, avg: *mut f64, status: *mut i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetStatistics)(windows_core::Interface::as_raw(self), max as _, min as _, avg as _, status as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICounterItem_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Value: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub SetColor: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Color: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetWidth: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub Width: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetLineStyle: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub LineStyle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetScaleFactor: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub ScaleFactor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub Path: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetValue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64, *mut i32) -> windows_core::HRESULT,
    pub GetStatistics: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64, *mut f64, *mut f64, *mut i32) -> windows_core::HRESULT,
}
pub trait ICounterItem_Impl: windows_core::IUnknownImpl {
    fn Value(&self) -> windows_core::Result<f64>;
    fn SetColor(&self, color: u32) -> windows_core::Result<()>;
    fn Color(&self) -> windows_core::Result<u32>;
    fn SetWidth(&self, iwidth: i32) -> windows_core::Result<()>;
    fn Width(&self) -> windows_core::Result<i32>;
    fn SetLineStyle(&self, ilinestyle: i32) -> windows_core::Result<()>;
    fn LineStyle(&self) -> windows_core::Result<i32>;
    fn SetScaleFactor(&self, iscale: i32) -> windows_core::Result<()>;
    fn ScaleFactor(&self) -> windows_core::Result<i32>;
    fn Path(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetValue(&self, value: *mut f64, status: *mut i32) -> windows_core::Result<()>;
    fn GetStatistics(&self, max: *mut f64, min: *mut f64, avg: *mut f64, status: *mut i32) -> windows_core::Result<()>;
}
impl ICounterItem_Vtbl {
    pub const fn new<Identity: ICounterItem_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Value<Identity: ICounterItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdblvalue: *mut f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICounterItem_Impl::Value(this) {
                    Ok(ok__) => {
                        pdblvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetColor<Identity: ICounterItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, color: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICounterItem_Impl::SetColor(this, core::mem::transmute_copy(&color)).into()
            }
        }
        unsafe extern "system" fn Color<Identity: ICounterItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcolor: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICounterItem_Impl::Color(this) {
                    Ok(ok__) => {
                        pcolor.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetWidth<Identity: ICounterItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iwidth: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICounterItem_Impl::SetWidth(this, core::mem::transmute_copy(&iwidth)).into()
            }
        }
        unsafe extern "system" fn Width<Identity: ICounterItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pivalue: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICounterItem_Impl::Width(this) {
                    Ok(ok__) => {
                        pivalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetLineStyle<Identity: ICounterItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ilinestyle: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICounterItem_Impl::SetLineStyle(this, core::mem::transmute_copy(&ilinestyle)).into()
            }
        }
        unsafe extern "system" fn LineStyle<Identity: ICounterItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pivalue: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICounterItem_Impl::LineStyle(this) {
                    Ok(ok__) => {
                        pivalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetScaleFactor<Identity: ICounterItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iscale: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICounterItem_Impl::SetScaleFactor(this, core::mem::transmute_copy(&iscale)).into()
            }
        }
        unsafe extern "system" fn ScaleFactor<Identity: ICounterItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pivalue: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICounterItem_Impl::ScaleFactor(this) {
                    Ok(ok__) => {
                        pivalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Path<Identity: ICounterItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstrvalue: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICounterItem_Impl::Path(this) {
                    Ok(ok__) => {
                        pstrvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetValue<Identity: ICounterItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut f64, status: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICounterItem_Impl::GetValue(this, core::mem::transmute_copy(&value), core::mem::transmute_copy(&status)).into()
            }
        }
        unsafe extern "system" fn GetStatistics<Identity: ICounterItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, max: *mut f64, min: *mut f64, avg: *mut f64, status: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICounterItem_Impl::GetStatistics(this, core::mem::transmute_copy(&max), core::mem::transmute_copy(&min), core::mem::transmute_copy(&avg), core::mem::transmute_copy(&status)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Value: Value::<Identity, OFFSET>,
            SetColor: SetColor::<Identity, OFFSET>,
            Color: Color::<Identity, OFFSET>,
            SetWidth: SetWidth::<Identity, OFFSET>,
            Width: Width::<Identity, OFFSET>,
            SetLineStyle: SetLineStyle::<Identity, OFFSET>,
            LineStyle: LineStyle::<Identity, OFFSET>,
            SetScaleFactor: SetScaleFactor::<Identity, OFFSET>,
            ScaleFactor: ScaleFactor::<Identity, OFFSET>,
            Path: Path::<Identity, OFFSET>,
            GetValue: GetValue::<Identity, OFFSET>,
            GetStatistics: GetStatistics::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICounterItem as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ICounterItem {}
windows_core::imp::define_interface!(ICounterItem2, ICounterItem2_Vtbl, 0xeefcd4e1_ea1c_4435_b7f4_e341ba03b4f9);
impl core::ops::Deref for ICounterItem2 {
    type Target = ICounterItem;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ICounterItem2, windows_core::IUnknown, ICounterItem);
impl ICounterItem2 {
    pub unsafe fn SetSelected(&self, bstate: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetSelected)(windows_core::Interface::as_raw(self), bstate).ok() }
    }
    pub unsafe fn Selected(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Selected)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetVisible(&self, bstate: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetVisible)(windows_core::Interface::as_raw(self), bstate).ok() }
    }
    pub unsafe fn Visible(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Visible)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn GetDataAt(&self, iindex: i32, iwhich: SysmonDataType) -> windows_core::Result<super::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDataAt)(windows_core::Interface::as_raw(self), iindex, iwhich, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICounterItem2_Vtbl {
    pub base__: ICounterItem_Vtbl,
    pub SetSelected: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub Selected: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetVisible: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub Visible: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub GetDataAt: unsafe extern "system" fn(*mut core::ffi::c_void, i32, SysmonDataType, *mut super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    GetDataAt: usize,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ICounterItem2_Impl: ICounterItem_Impl {
    fn SetSelected(&self, bstate: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn Selected(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetVisible(&self, bstate: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn Visible(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn GetDataAt(&self, iindex: i32, iwhich: SysmonDataType) -> windows_core::Result<super::Variant::VARIANT>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ICounterItem2_Vtbl {
    pub const fn new<Identity: ICounterItem2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetSelected<Identity: ICounterItem2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstate: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICounterItem2_Impl::SetSelected(this, core::mem::transmute_copy(&bstate)).into()
            }
        }
        unsafe extern "system" fn Selected<Identity: ICounterItem2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstate: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICounterItem2_Impl::Selected(this) {
                    Ok(ok__) => {
                        pbstate.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetVisible<Identity: ICounterItem2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstate: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICounterItem2_Impl::SetVisible(this, core::mem::transmute_copy(&bstate)).into()
            }
        }
        unsafe extern "system" fn Visible<Identity: ICounterItem2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstate: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICounterItem2_Impl::Visible(this) {
                    Ok(ok__) => {
                        pbstate.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetDataAt<Identity: ICounterItem2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iindex: i32, iwhich: SysmonDataType, pvariant: *mut super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICounterItem2_Impl::GetDataAt(this, core::mem::transmute_copy(&iindex), core::mem::transmute_copy(&iwhich)) {
                    Ok(ok__) => {
                        pvariant.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: ICounterItem_Vtbl::new::<Identity, OFFSET>(),
            SetSelected: SetSelected::<Identity, OFFSET>,
            Selected: Selected::<Identity, OFFSET>,
            SetVisible: SetVisible::<Identity, OFFSET>,
            Visible: Visible::<Identity, OFFSET>,
            GetDataAt: GetDataAt::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICounterItem2 as windows_core::Interface>::IID || iid == &<ICounterItem as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ICounterItem2 {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ICounters, ICounters_Vtbl, 0x79167962_28fc_11cf_942f_008029004347);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ICounters {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ICounters, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ICounters {
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn get_Item(&self, index: &super::Variant::VARIANT) -> windows_core::Result<DICounterItem> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Item)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(index), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Add(&self, pathname: &windows_core::BSTR) -> windows_core::Result<DICounterItem> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Add)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(pathname), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Remove(&self, index: &super::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Remove)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(index)).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ICounters_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub get_Item: unsafe extern "system" fn(*mut core::ffi::c_void, super::Variant::VARIANT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    get_Item: usize,
    pub Add: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub Remove: unsafe extern "system" fn(*mut core::ffi::c_void, super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    Remove: usize,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ICounters_Impl: super::Com::IDispatch_Impl {
    fn Count(&self) -> windows_core::Result<i32>;
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn get_Item(&self, index: &super::Variant::VARIANT) -> windows_core::Result<DICounterItem>;
    fn Add(&self, pathname: &windows_core::BSTR) -> windows_core::Result<DICounterItem>;
    fn Remove(&self, index: &super::Variant::VARIANT) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ICounters_Vtbl {
    pub const fn new<Identity: ICounters_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Count<Identity: ICounters_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plong: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICounters_Impl::Count(this) {
                    Ok(ok__) => {
                        plong.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: ICounters_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppiunk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICounters_Impl::_NewEnum(this) {
                    Ok(ok__) => {
                        ppiunk.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Item<Identity: ICounters_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: super::Variant::VARIANT, ppi: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICounters_Impl::get_Item(this, core::mem::transmute(&index)) {
                    Ok(ok__) => {
                        ppi.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Add<Identity: ICounters_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pathname: *mut core::ffi::c_void, ppi: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICounters_Impl::Add(this, core::mem::transmute(&pathname)) {
                    Ok(ok__) => {
                        ppi.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Remove<Identity: ICounters_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICounters_Impl::Remove(this, core::mem::transmute(&index)).into()
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Count: Count::<Identity, OFFSET>,
            _NewEnum: _NewEnum::<Identity, OFFSET>,
            get_Item: get_Item::<Identity, OFFSET>,
            Add: Add::<Identity, OFFSET>,
            Remove: Remove::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICounters as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ICounters {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IDataCollector, IDataCollector_Vtbl, 0x038374ff_098b_11d8_9414_505054503030);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IDataCollector {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IDataCollector, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IDataCollector {
    pub unsafe fn DataCollectorSet(&self) -> windows_core::Result<IDataCollectorSet> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DataCollectorSet)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetDataCollectorSet<P0>(&self, group: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDataCollectorSet>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetDataCollectorSet)(windows_core::Interface::as_raw(self), group.param().abi()).ok() }
    }
    pub unsafe fn DataCollectorType(&self) -> windows_core::Result<DataCollectorType> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DataCollectorType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn FileName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FileName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetFileName(&self, name: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetFileName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(name)).ok() }
    }
    pub unsafe fn FileNameFormat(&self) -> windows_core::Result<AutoPathFormat> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FileNameFormat)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetFileNameFormat(&self, format: AutoPathFormat) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetFileNameFormat)(windows_core::Interface::as_raw(self), format).ok() }
    }
    pub unsafe fn FileNameFormatPattern(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FileNameFormatPattern)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetFileNameFormatPattern(&self, pattern: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetFileNameFormatPattern)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(pattern)).ok() }
    }
    pub unsafe fn LatestOutputLocation(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).LatestOutputLocation)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetLatestOutputLocation(&self, path: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetLatestOutputLocation)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(path)).ok() }
    }
    pub unsafe fn LogAppend(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).LogAppend)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetLogAppend(&self, append: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetLogAppend)(windows_core::Interface::as_raw(self), append).ok() }
    }
    pub unsafe fn LogCircular(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).LogCircular)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetLogCircular(&self, circular: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetLogCircular)(windows_core::Interface::as_raw(self), circular).ok() }
    }
    pub unsafe fn LogOverwrite(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).LogOverwrite)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetLogOverwrite(&self, overwrite: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetLogOverwrite)(windows_core::Interface::as_raw(self), overwrite).ok() }
    }
    pub unsafe fn Name(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Name)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetName(&self, name: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(name)).ok() }
    }
    pub unsafe fn OutputLocation(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).OutputLocation)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn Index(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Index)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetIndex(&self, index: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetIndex)(windows_core::Interface::as_raw(self), index).ok() }
    }
    pub unsafe fn Xml(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Xml)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetXml(&self, xml: &windows_core::BSTR) -> windows_core::Result<IValueMap> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SetXml)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(xml), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateOutputLocation(&self, latest: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateOutputLocation)(windows_core::Interface::as_raw(self), latest, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IDataCollector_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub DataCollectorSet: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetDataCollectorSet: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DataCollectorType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DataCollectorType) -> windows_core::HRESULT,
    pub FileName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetFileName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FileNameFormat: unsafe extern "system" fn(*mut core::ffi::c_void, *mut AutoPathFormat) -> windows_core::HRESULT,
    pub SetFileNameFormat: unsafe extern "system" fn(*mut core::ffi::c_void, AutoPathFormat) -> windows_core::HRESULT,
    pub FileNameFormatPattern: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetFileNameFormatPattern: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub LatestOutputLocation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetLatestOutputLocation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub LogAppend: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetLogAppend: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub LogCircular: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetLogCircular: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub LogOverwrite: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetLogOverwrite: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OutputLocation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Index: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetIndex: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub Xml: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetXml: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateOutputLocation: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IDataCollector_Impl: super::Com::IDispatch_Impl {
    fn DataCollectorSet(&self) -> windows_core::Result<IDataCollectorSet>;
    fn SetDataCollectorSet(&self, group: windows_core::Ref<IDataCollectorSet>) -> windows_core::Result<()>;
    fn DataCollectorType(&self) -> windows_core::Result<DataCollectorType>;
    fn FileName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetFileName(&self, name: &windows_core::BSTR) -> windows_core::Result<()>;
    fn FileNameFormat(&self) -> windows_core::Result<AutoPathFormat>;
    fn SetFileNameFormat(&self, format: AutoPathFormat) -> windows_core::Result<()>;
    fn FileNameFormatPattern(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetFileNameFormatPattern(&self, pattern: &windows_core::BSTR) -> windows_core::Result<()>;
    fn LatestOutputLocation(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetLatestOutputLocation(&self, path: &windows_core::BSTR) -> windows_core::Result<()>;
    fn LogAppend(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetLogAppend(&self, append: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn LogCircular(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetLogCircular(&self, circular: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn LogOverwrite(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetLogOverwrite(&self, overwrite: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn Name(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetName(&self, name: &windows_core::BSTR) -> windows_core::Result<()>;
    fn OutputLocation(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Index(&self) -> windows_core::Result<i32>;
    fn SetIndex(&self, index: i32) -> windows_core::Result<()>;
    fn Xml(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetXml(&self, xml: &windows_core::BSTR) -> windows_core::Result<IValueMap>;
    fn CreateOutputLocation(&self, latest: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IDataCollector_Vtbl {
    pub const fn new<Identity: IDataCollector_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn DataCollectorSet<Identity: IDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, group: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataCollector_Impl::DataCollectorSet(this) {
                    Ok(ok__) => {
                        group.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetDataCollectorSet<Identity: IDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, group: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDataCollector_Impl::SetDataCollectorSet(this, core::mem::transmute_copy(&group)).into()
            }
        }
        unsafe extern "system" fn DataCollectorType<Identity: IDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, r#type: *mut DataCollectorType) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataCollector_Impl::DataCollectorType(this) {
                    Ok(ok__) => {
                        r#type.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn FileName<Identity: IDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataCollector_Impl::FileName(this) {
                    Ok(ok__) => {
                        name.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetFileName<Identity: IDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDataCollector_Impl::SetFileName(this, core::mem::transmute(&name)).into()
            }
        }
        unsafe extern "system" fn FileNameFormat<Identity: IDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, format: *mut AutoPathFormat) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataCollector_Impl::FileNameFormat(this) {
                    Ok(ok__) => {
                        format.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetFileNameFormat<Identity: IDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, format: AutoPathFormat) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDataCollector_Impl::SetFileNameFormat(this, core::mem::transmute_copy(&format)).into()
            }
        }
        unsafe extern "system" fn FileNameFormatPattern<Identity: IDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pattern: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataCollector_Impl::FileNameFormatPattern(this) {
                    Ok(ok__) => {
                        pattern.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetFileNameFormatPattern<Identity: IDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pattern: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDataCollector_Impl::SetFileNameFormatPattern(this, core::mem::transmute(&pattern)).into()
            }
        }
        unsafe extern "system" fn LatestOutputLocation<Identity: IDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, path: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataCollector_Impl::LatestOutputLocation(this) {
                    Ok(ok__) => {
                        path.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetLatestOutputLocation<Identity: IDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, path: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDataCollector_Impl::SetLatestOutputLocation(this, core::mem::transmute(&path)).into()
            }
        }
        unsafe extern "system" fn LogAppend<Identity: IDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, append: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataCollector_Impl::LogAppend(this) {
                    Ok(ok__) => {
                        append.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetLogAppend<Identity: IDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, append: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDataCollector_Impl::SetLogAppend(this, core::mem::transmute_copy(&append)).into()
            }
        }
        unsafe extern "system" fn LogCircular<Identity: IDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, circular: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataCollector_Impl::LogCircular(this) {
                    Ok(ok__) => {
                        circular.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetLogCircular<Identity: IDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, circular: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDataCollector_Impl::SetLogCircular(this, core::mem::transmute_copy(&circular)).into()
            }
        }
        unsafe extern "system" fn LogOverwrite<Identity: IDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, overwrite: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataCollector_Impl::LogOverwrite(this) {
                    Ok(ok__) => {
                        overwrite.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetLogOverwrite<Identity: IDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, overwrite: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDataCollector_Impl::SetLogOverwrite(this, core::mem::transmute_copy(&overwrite)).into()
            }
        }
        unsafe extern "system" fn Name<Identity: IDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataCollector_Impl::Name(this) {
                    Ok(ok__) => {
                        name.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetName<Identity: IDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDataCollector_Impl::SetName(this, core::mem::transmute(&name)).into()
            }
        }
        unsafe extern "system" fn OutputLocation<Identity: IDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, path: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataCollector_Impl::OutputLocation(this) {
                    Ok(ok__) => {
                        path.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Index<Identity: IDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataCollector_Impl::Index(this) {
                    Ok(ok__) => {
                        index.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetIndex<Identity: IDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDataCollector_Impl::SetIndex(this, core::mem::transmute_copy(&index)).into()
            }
        }
        unsafe extern "system" fn Xml<Identity: IDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, xml: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataCollector_Impl::Xml(this) {
                    Ok(ok__) => {
                        xml.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetXml<Identity: IDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, xml: *mut core::ffi::c_void, validation: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataCollector_Impl::SetXml(this, core::mem::transmute(&xml)) {
                    Ok(ok__) => {
                        validation.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateOutputLocation<Identity: IDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, latest: super::super::Foundation::VARIANT_BOOL, location: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataCollector_Impl::CreateOutputLocation(this, core::mem::transmute_copy(&latest)) {
                    Ok(ok__) => {
                        location.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            DataCollectorSet: DataCollectorSet::<Identity, OFFSET>,
            SetDataCollectorSet: SetDataCollectorSet::<Identity, OFFSET>,
            DataCollectorType: DataCollectorType::<Identity, OFFSET>,
            FileName: FileName::<Identity, OFFSET>,
            SetFileName: SetFileName::<Identity, OFFSET>,
            FileNameFormat: FileNameFormat::<Identity, OFFSET>,
            SetFileNameFormat: SetFileNameFormat::<Identity, OFFSET>,
            FileNameFormatPattern: FileNameFormatPattern::<Identity, OFFSET>,
            SetFileNameFormatPattern: SetFileNameFormatPattern::<Identity, OFFSET>,
            LatestOutputLocation: LatestOutputLocation::<Identity, OFFSET>,
            SetLatestOutputLocation: SetLatestOutputLocation::<Identity, OFFSET>,
            LogAppend: LogAppend::<Identity, OFFSET>,
            SetLogAppend: SetLogAppend::<Identity, OFFSET>,
            LogCircular: LogCircular::<Identity, OFFSET>,
            SetLogCircular: SetLogCircular::<Identity, OFFSET>,
            LogOverwrite: LogOverwrite::<Identity, OFFSET>,
            SetLogOverwrite: SetLogOverwrite::<Identity, OFFSET>,
            Name: Name::<Identity, OFFSET>,
            SetName: SetName::<Identity, OFFSET>,
            OutputLocation: OutputLocation::<Identity, OFFSET>,
            Index: Index::<Identity, OFFSET>,
            SetIndex: SetIndex::<Identity, OFFSET>,
            Xml: Xml::<Identity, OFFSET>,
            SetXml: SetXml::<Identity, OFFSET>,
            CreateOutputLocation: CreateOutputLocation::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDataCollector as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IDataCollector {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IDataCollectorCollection, IDataCollectorCollection_Vtbl, 0x03837502_098b_11d8_9414_505054503030);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IDataCollectorCollection {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IDataCollectorCollection, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IDataCollectorCollection {
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn get_Item(&self, index: &super::Variant::VARIANT) -> windows_core::Result<IDataCollector> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Item)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(index), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Add<P0>(&self, collector: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDataCollector>,
    {
        unsafe { (windows_core::Interface::vtable(self).Add)(windows_core::Interface::as_raw(self), collector.param().abi()).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Remove(&self, collector: &super::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Remove)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(collector)).ok() }
    }
    pub unsafe fn Clear(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Clear)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn AddRange<P0>(&self, collectors: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDataCollectorCollection>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddRange)(windows_core::Interface::as_raw(self), collectors.param().abi()).ok() }
    }
    pub unsafe fn CreateDataCollectorFromXml(&self, bstrxml: &windows_core::BSTR, pvalidation: *mut Option<IValueMap>, pcollector: *mut Option<IDataCollector>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).CreateDataCollectorFromXml)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrxml), core::mem::transmute(pvalidation), core::mem::transmute(pcollector)).ok() }
    }
    pub unsafe fn CreateDataCollector(&self, r#type: DataCollectorType) -> windows_core::Result<IDataCollector> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateDataCollector)(windows_core::Interface::as_raw(self), r#type, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IDataCollectorCollection_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub get_Item: unsafe extern "system" fn(*mut core::ffi::c_void, super::Variant::VARIANT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    get_Item: usize,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Add: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub Remove: unsafe extern "system" fn(*mut core::ffi::c_void, super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    Remove: usize,
    pub Clear: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddRange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateDataCollectorFromXml: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateDataCollector: unsafe extern "system" fn(*mut core::ffi::c_void, DataCollectorType, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IDataCollectorCollection_Impl: super::Com::IDispatch_Impl {
    fn Count(&self) -> windows_core::Result<i32>;
    fn get_Item(&self, index: &super::Variant::VARIANT) -> windows_core::Result<IDataCollector>;
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn Add(&self, collector: windows_core::Ref<IDataCollector>) -> windows_core::Result<()>;
    fn Remove(&self, collector: &super::Variant::VARIANT) -> windows_core::Result<()>;
    fn Clear(&self) -> windows_core::Result<()>;
    fn AddRange(&self, collectors: windows_core::Ref<IDataCollectorCollection>) -> windows_core::Result<()>;
    fn CreateDataCollectorFromXml(&self, bstrxml: &windows_core::BSTR, pvalidation: windows_core::OutRef<IValueMap>, pcollector: windows_core::OutRef<IDataCollector>) -> windows_core::Result<()>;
    fn CreateDataCollector(&self, r#type: DataCollectorType) -> windows_core::Result<IDataCollector>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IDataCollectorCollection_Vtbl {
    pub const fn new<Identity: IDataCollectorCollection_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Count<Identity: IDataCollectorCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataCollectorCollection_Impl::Count(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Item<Identity: IDataCollectorCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: super::Variant::VARIANT, collector: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataCollectorCollection_Impl::get_Item(this, core::mem::transmute(&index)) {
                    Ok(ok__) => {
                        collector.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: IDataCollectorCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataCollectorCollection_Impl::_NewEnum(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Add<Identity: IDataCollectorCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, collector: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDataCollectorCollection_Impl::Add(this, core::mem::transmute_copy(&collector)).into()
            }
        }
        unsafe extern "system" fn Remove<Identity: IDataCollectorCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, collector: super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDataCollectorCollection_Impl::Remove(this, core::mem::transmute(&collector)).into()
            }
        }
        unsafe extern "system" fn Clear<Identity: IDataCollectorCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDataCollectorCollection_Impl::Clear(this).into()
            }
        }
        unsafe extern "system" fn AddRange<Identity: IDataCollectorCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, collectors: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDataCollectorCollection_Impl::AddRange(this, core::mem::transmute_copy(&collectors)).into()
            }
        }
        unsafe extern "system" fn CreateDataCollectorFromXml<Identity: IDataCollectorCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrxml: *mut core::ffi::c_void, pvalidation: *mut *mut core::ffi::c_void, pcollector: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDataCollectorCollection_Impl::CreateDataCollectorFromXml(this, core::mem::transmute(&bstrxml), core::mem::transmute_copy(&pvalidation), core::mem::transmute_copy(&pcollector)).into()
            }
        }
        unsafe extern "system" fn CreateDataCollector<Identity: IDataCollectorCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, r#type: DataCollectorType, collector: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataCollectorCollection_Impl::CreateDataCollector(this, core::mem::transmute_copy(&r#type)) {
                    Ok(ok__) => {
                        collector.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Count: Count::<Identity, OFFSET>,
            get_Item: get_Item::<Identity, OFFSET>,
            _NewEnum: _NewEnum::<Identity, OFFSET>,
            Add: Add::<Identity, OFFSET>,
            Remove: Remove::<Identity, OFFSET>,
            Clear: Clear::<Identity, OFFSET>,
            AddRange: AddRange::<Identity, OFFSET>,
            CreateDataCollectorFromXml: CreateDataCollectorFromXml::<Identity, OFFSET>,
            CreateDataCollector: CreateDataCollector::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDataCollectorCollection as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IDataCollectorCollection {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IDataCollectorSet, IDataCollectorSet_Vtbl, 0x03837520_098b_11d8_9414_505054503030);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IDataCollectorSet {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IDataCollectorSet, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IDataCollectorSet {
    pub unsafe fn DataCollectors(&self) -> windows_core::Result<IDataCollectorCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DataCollectors)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Duration(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Duration)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetDuration(&self, seconds: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetDuration)(windows_core::Interface::as_raw(self), seconds).ok() }
    }
    pub unsafe fn Description(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Description)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetDescription(&self, description: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetDescription)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(description)).ok() }
    }
    pub unsafe fn DescriptionUnresolved(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DescriptionUnresolved)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn DisplayName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DisplayName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetDisplayName(&self, displayname: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetDisplayName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(displayname)).ok() }
    }
    pub unsafe fn DisplayNameUnresolved(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DisplayNameUnresolved)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn Keywords(&self) -> windows_core::Result<*mut super::Com::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Keywords)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetKeywords(&self, keywords: *const super::Com::SAFEARRAY) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetKeywords)(windows_core::Interface::as_raw(self), keywords).ok() }
    }
    pub unsafe fn LatestOutputLocation(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).LatestOutputLocation)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetLatestOutputLocation(&self, path: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetLatestOutputLocation)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(path)).ok() }
    }
    pub unsafe fn Name(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Name)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn OutputLocation(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).OutputLocation)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn RootPath(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RootPath)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetRootPath(&self, folder: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetRootPath)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(folder)).ok() }
    }
    pub unsafe fn Segment(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Segment)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetSegment(&self, segment: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetSegment)(windows_core::Interface::as_raw(self), segment).ok() }
    }
    pub unsafe fn SegmentMaxDuration(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SegmentMaxDuration)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetSegmentMaxDuration(&self, seconds: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetSegmentMaxDuration)(windows_core::Interface::as_raw(self), seconds).ok() }
    }
    pub unsafe fn SegmentMaxSize(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SegmentMaxSize)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetSegmentMaxSize(&self, size: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetSegmentMaxSize)(windows_core::Interface::as_raw(self), size).ok() }
    }
    pub unsafe fn SerialNumber(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SerialNumber)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetSerialNumber(&self, index: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetSerialNumber)(windows_core::Interface::as_raw(self), index).ok() }
    }
    pub unsafe fn Server(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Server)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn Status(&self) -> windows_core::Result<DataCollectorSetStatus> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Status)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Subdirectory(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Subdirectory)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetSubdirectory(&self, folder: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetSubdirectory)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(folder)).ok() }
    }
    pub unsafe fn SubdirectoryFormat(&self) -> windows_core::Result<AutoPathFormat> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SubdirectoryFormat)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetSubdirectoryFormat(&self, format: AutoPathFormat) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetSubdirectoryFormat)(windows_core::Interface::as_raw(self), format).ok() }
    }
    pub unsafe fn SubdirectoryFormatPattern(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SubdirectoryFormatPattern)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetSubdirectoryFormatPattern(&self, pattern: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetSubdirectoryFormatPattern)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(pattern)).ok() }
    }
    pub unsafe fn Task(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Task)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetTask(&self, task: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetTask)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(task)).ok() }
    }
    pub unsafe fn TaskRunAsSelf(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).TaskRunAsSelf)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetTaskRunAsSelf(&self, runasself: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetTaskRunAsSelf)(windows_core::Interface::as_raw(self), runasself).ok() }
    }
    pub unsafe fn TaskArguments(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).TaskArguments)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetTaskArguments(&self, task: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetTaskArguments)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(task)).ok() }
    }
    pub unsafe fn TaskUserTextArguments(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).TaskUserTextArguments)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetTaskUserTextArguments(&self, usertext: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetTaskUserTextArguments)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(usertext)).ok() }
    }
    pub unsafe fn Schedules(&self) -> windows_core::Result<IScheduleCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Schedules)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SchedulesEnabled(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SchedulesEnabled)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetSchedulesEnabled(&self, enabled: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetSchedulesEnabled)(windows_core::Interface::as_raw(self), enabled).ok() }
    }
    pub unsafe fn UserAccount(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).UserAccount)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn Xml(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Xml)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn Security(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Security)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetSecurity(&self, bstrsecurity: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetSecurity)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrsecurity)).ok() }
    }
    pub unsafe fn StopOnCompletion(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).StopOnCompletion)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetStopOnCompletion(&self, stop: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetStopOnCompletion)(windows_core::Interface::as_raw(self), stop).ok() }
    }
    pub unsafe fn DataManager(&self) -> windows_core::Result<IDataManager> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DataManager)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetCredentials(&self, user: &windows_core::BSTR, password: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetCredentials)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(user), core::mem::transmute_copy(password)).ok() }
    }
    pub unsafe fn Query(&self, name: &windows_core::BSTR, server: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Query)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(name), core::mem::transmute_copy(server)).ok() }
    }
    pub unsafe fn Commit(&self, name: &windows_core::BSTR, server: &windows_core::BSTR, mode: CommitMode) -> windows_core::Result<IValueMap> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Commit)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(name), core::mem::transmute_copy(server), mode, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Delete(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Delete)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Start(&self, synchronous: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Start)(windows_core::Interface::as_raw(self), synchronous).ok() }
    }
    pub unsafe fn Stop(&self, synchronous: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Stop)(windows_core::Interface::as_raw(self), synchronous).ok() }
    }
    pub unsafe fn SetXml(&self, xml: &windows_core::BSTR) -> windows_core::Result<IValueMap> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SetXml)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(xml), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetValue(&self, key: &windows_core::BSTR, value: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetValue)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(key), core::mem::transmute_copy(value)).ok() }
    }
    pub unsafe fn GetValue(&self, key: &windows_core::BSTR) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetValue)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(key), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IDataCollectorSet_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub DataCollectors: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Duration: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetDuration: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Description: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DescriptionUnresolved: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DisplayName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetDisplayName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DisplayNameUnresolved: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Keywords: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::Com::SAFEARRAY) -> windows_core::HRESULT,
    pub SetKeywords: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::Com::SAFEARRAY) -> windows_core::HRESULT,
    pub LatestOutputLocation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetLatestOutputLocation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OutputLocation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RootPath: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetRootPath: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Segment: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetSegment: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SegmentMaxDuration: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetSegmentMaxDuration: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub SegmentMaxSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetSegmentMaxSize: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub SerialNumber: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetSerialNumber: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Server: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Status: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DataCollectorSetStatus) -> windows_core::HRESULT,
    pub Subdirectory: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetSubdirectory: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SubdirectoryFormat: unsafe extern "system" fn(*mut core::ffi::c_void, *mut AutoPathFormat) -> windows_core::HRESULT,
    pub SetSubdirectoryFormat: unsafe extern "system" fn(*mut core::ffi::c_void, AutoPathFormat) -> windows_core::HRESULT,
    pub SubdirectoryFormatPattern: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetSubdirectoryFormatPattern: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Task: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetTask: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TaskRunAsSelf: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetTaskRunAsSelf: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub TaskArguments: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetTaskArguments: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TaskUserTextArguments: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetTaskUserTextArguments: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Schedules: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SchedulesEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetSchedulesEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub UserAccount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Xml: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Security: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetSecurity: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub StopOnCompletion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetStopOnCompletion: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub DataManager: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetCredentials: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Query: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Commit: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, CommitMode, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Delete: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Start: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub Stop: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetXml: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetValue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetValue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IDataCollectorSet_Impl: super::Com::IDispatch_Impl {
    fn DataCollectors(&self) -> windows_core::Result<IDataCollectorCollection>;
    fn Duration(&self) -> windows_core::Result<u32>;
    fn SetDuration(&self, seconds: u32) -> windows_core::Result<()>;
    fn Description(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetDescription(&self, description: &windows_core::BSTR) -> windows_core::Result<()>;
    fn DescriptionUnresolved(&self) -> windows_core::Result<windows_core::BSTR>;
    fn DisplayName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetDisplayName(&self, displayname: &windows_core::BSTR) -> windows_core::Result<()>;
    fn DisplayNameUnresolved(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Keywords(&self) -> windows_core::Result<*mut super::Com::SAFEARRAY>;
    fn SetKeywords(&self, keywords: *const super::Com::SAFEARRAY) -> windows_core::Result<()>;
    fn LatestOutputLocation(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetLatestOutputLocation(&self, path: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Name(&self) -> windows_core::Result<windows_core::BSTR>;
    fn OutputLocation(&self) -> windows_core::Result<windows_core::BSTR>;
    fn RootPath(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetRootPath(&self, folder: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Segment(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetSegment(&self, segment: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn SegmentMaxDuration(&self) -> windows_core::Result<u32>;
    fn SetSegmentMaxDuration(&self, seconds: u32) -> windows_core::Result<()>;
    fn SegmentMaxSize(&self) -> windows_core::Result<u32>;
    fn SetSegmentMaxSize(&self, size: u32) -> windows_core::Result<()>;
    fn SerialNumber(&self) -> windows_core::Result<u32>;
    fn SetSerialNumber(&self, index: u32) -> windows_core::Result<()>;
    fn Server(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Status(&self) -> windows_core::Result<DataCollectorSetStatus>;
    fn Subdirectory(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetSubdirectory(&self, folder: &windows_core::BSTR) -> windows_core::Result<()>;
    fn SubdirectoryFormat(&self) -> windows_core::Result<AutoPathFormat>;
    fn SetSubdirectoryFormat(&self, format: AutoPathFormat) -> windows_core::Result<()>;
    fn SubdirectoryFormatPattern(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetSubdirectoryFormatPattern(&self, pattern: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Task(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetTask(&self, task: &windows_core::BSTR) -> windows_core::Result<()>;
    fn TaskRunAsSelf(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetTaskRunAsSelf(&self, runasself: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn TaskArguments(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetTaskArguments(&self, task: &windows_core::BSTR) -> windows_core::Result<()>;
    fn TaskUserTextArguments(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetTaskUserTextArguments(&self, usertext: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Schedules(&self) -> windows_core::Result<IScheduleCollection>;
    fn SchedulesEnabled(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetSchedulesEnabled(&self, enabled: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn UserAccount(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Xml(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Security(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetSecurity(&self, bstrsecurity: &windows_core::BSTR) -> windows_core::Result<()>;
    fn StopOnCompletion(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetStopOnCompletion(&self, stop: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn DataManager(&self) -> windows_core::Result<IDataManager>;
    fn SetCredentials(&self, user: &windows_core::BSTR, password: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Query(&self, name: &windows_core::BSTR, server: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Commit(&self, name: &windows_core::BSTR, server: &windows_core::BSTR, mode: CommitMode) -> windows_core::Result<IValueMap>;
    fn Delete(&self) -> windows_core::Result<()>;
    fn Start(&self, synchronous: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn Stop(&self, synchronous: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn SetXml(&self, xml: &windows_core::BSTR) -> windows_core::Result<IValueMap>;
    fn SetValue(&self, key: &windows_core::BSTR, value: &windows_core::BSTR) -> windows_core::Result<()>;
    fn GetValue(&self, key: &windows_core::BSTR) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IDataCollectorSet_Vtbl {
    pub const fn new<Identity: IDataCollectorSet_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn DataCollectors<Identity: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, collectors: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataCollectorSet_Impl::DataCollectors(this) {
                    Ok(ok__) => {
                        collectors.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Duration<Identity: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, seconds: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataCollectorSet_Impl::Duration(this) {
                    Ok(ok__) => {
                        seconds.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetDuration<Identity: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, seconds: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDataCollectorSet_Impl::SetDuration(this, core::mem::transmute_copy(&seconds)).into()
            }
        }
        unsafe extern "system" fn Description<Identity: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, description: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataCollectorSet_Impl::Description(this) {
                    Ok(ok__) => {
                        description.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetDescription<Identity: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, description: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDataCollectorSet_Impl::SetDescription(this, core::mem::transmute(&description)).into()
            }
        }
        unsafe extern "system" fn DescriptionUnresolved<Identity: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, descr: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataCollectorSet_Impl::DescriptionUnresolved(this) {
                    Ok(ok__) => {
                        descr.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DisplayName<Identity: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, displayname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataCollectorSet_Impl::DisplayName(this) {
                    Ok(ok__) => {
                        displayname.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetDisplayName<Identity: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, displayname: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDataCollectorSet_Impl::SetDisplayName(this, core::mem::transmute(&displayname)).into()
            }
        }
        unsafe extern "system" fn DisplayNameUnresolved<Identity: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataCollectorSet_Impl::DisplayNameUnresolved(this) {
                    Ok(ok__) => {
                        name.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Keywords<Identity: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, keywords: *mut *mut super::Com::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataCollectorSet_Impl::Keywords(this) {
                    Ok(ok__) => {
                        keywords.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetKeywords<Identity: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, keywords: *const super::Com::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDataCollectorSet_Impl::SetKeywords(this, core::mem::transmute_copy(&keywords)).into()
            }
        }
        unsafe extern "system" fn LatestOutputLocation<Identity: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, path: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataCollectorSet_Impl::LatestOutputLocation(this) {
                    Ok(ok__) => {
                        path.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetLatestOutputLocation<Identity: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, path: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDataCollectorSet_Impl::SetLatestOutputLocation(this, core::mem::transmute(&path)).into()
            }
        }
        unsafe extern "system" fn Name<Identity: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataCollectorSet_Impl::Name(this) {
                    Ok(ok__) => {
                        name.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn OutputLocation<Identity: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, path: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataCollectorSet_Impl::OutputLocation(this) {
                    Ok(ok__) => {
                        path.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RootPath<Identity: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, folder: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataCollectorSet_Impl::RootPath(this) {
                    Ok(ok__) => {
                        folder.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetRootPath<Identity: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, folder: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDataCollectorSet_Impl::SetRootPath(this, core::mem::transmute(&folder)).into()
            }
        }
        unsafe extern "system" fn Segment<Identity: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, segment: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataCollectorSet_Impl::Segment(this) {
                    Ok(ok__) => {
                        segment.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetSegment<Identity: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, segment: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDataCollectorSet_Impl::SetSegment(this, core::mem::transmute_copy(&segment)).into()
            }
        }
        unsafe extern "system" fn SegmentMaxDuration<Identity: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, seconds: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataCollectorSet_Impl::SegmentMaxDuration(this) {
                    Ok(ok__) => {
                        seconds.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetSegmentMaxDuration<Identity: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, seconds: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDataCollectorSet_Impl::SetSegmentMaxDuration(this, core::mem::transmute_copy(&seconds)).into()
            }
        }
        unsafe extern "system" fn SegmentMaxSize<Identity: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, size: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataCollectorSet_Impl::SegmentMaxSize(this) {
                    Ok(ok__) => {
                        size.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetSegmentMaxSize<Identity: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, size: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDataCollectorSet_Impl::SetSegmentMaxSize(this, core::mem::transmute_copy(&size)).into()
            }
        }
        unsafe extern "system" fn SerialNumber<Identity: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataCollectorSet_Impl::SerialNumber(this) {
                    Ok(ok__) => {
                        index.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetSerialNumber<Identity: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDataCollectorSet_Impl::SetSerialNumber(this, core::mem::transmute_copy(&index)).into()
            }
        }
        unsafe extern "system" fn Server<Identity: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, server: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataCollectorSet_Impl::Server(this) {
                    Ok(ok__) => {
                        server.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Status<Identity: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, status: *mut DataCollectorSetStatus) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataCollectorSet_Impl::Status(this) {
                    Ok(ok__) => {
                        status.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Subdirectory<Identity: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, folder: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataCollectorSet_Impl::Subdirectory(this) {
                    Ok(ok__) => {
                        folder.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetSubdirectory<Identity: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, folder: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDataCollectorSet_Impl::SetSubdirectory(this, core::mem::transmute(&folder)).into()
            }
        }
        unsafe extern "system" fn SubdirectoryFormat<Identity: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, format: *mut AutoPathFormat) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataCollectorSet_Impl::SubdirectoryFormat(this) {
                    Ok(ok__) => {
                        format.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetSubdirectoryFormat<Identity: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, format: AutoPathFormat) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDataCollectorSet_Impl::SetSubdirectoryFormat(this, core::mem::transmute_copy(&format)).into()
            }
        }
        unsafe extern "system" fn SubdirectoryFormatPattern<Identity: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pattern: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataCollectorSet_Impl::SubdirectoryFormatPattern(this) {
                    Ok(ok__) => {
                        pattern.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetSubdirectoryFormatPattern<Identity: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pattern: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDataCollectorSet_Impl::SetSubdirectoryFormatPattern(this, core::mem::transmute(&pattern)).into()
            }
        }
        unsafe extern "system" fn Task<Identity: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, task: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataCollectorSet_Impl::Task(this) {
                    Ok(ok__) => {
                        task.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetTask<Identity: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, task: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDataCollectorSet_Impl::SetTask(this, core::mem::transmute(&task)).into()
            }
        }
        unsafe extern "system" fn TaskRunAsSelf<Identity: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, runasself: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataCollectorSet_Impl::TaskRunAsSelf(this) {
                    Ok(ok__) => {
                        runasself.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetTaskRunAsSelf<Identity: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, runasself: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDataCollectorSet_Impl::SetTaskRunAsSelf(this, core::mem::transmute_copy(&runasself)).into()
            }
        }
        unsafe extern "system" fn TaskArguments<Identity: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, task: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataCollectorSet_Impl::TaskArguments(this) {
                    Ok(ok__) => {
                        task.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetTaskArguments<Identity: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, task: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDataCollectorSet_Impl::SetTaskArguments(this, core::mem::transmute(&task)).into()
            }
        }
        unsafe extern "system" fn TaskUserTextArguments<Identity: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, usertext: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataCollectorSet_Impl::TaskUserTextArguments(this) {
                    Ok(ok__) => {
                        usertext.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetTaskUserTextArguments<Identity: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, usertext: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDataCollectorSet_Impl::SetTaskUserTextArguments(this, core::mem::transmute(&usertext)).into()
            }
        }
        unsafe extern "system" fn Schedules<Identity: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppschedules: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataCollectorSet_Impl::Schedules(this) {
                    Ok(ok__) => {
                        ppschedules.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SchedulesEnabled<Identity: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, enabled: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataCollectorSet_Impl::SchedulesEnabled(this) {
                    Ok(ok__) => {
                        enabled.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetSchedulesEnabled<Identity: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, enabled: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDataCollectorSet_Impl::SetSchedulesEnabled(this, core::mem::transmute_copy(&enabled)).into()
            }
        }
        unsafe extern "system" fn UserAccount<Identity: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, user: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataCollectorSet_Impl::UserAccount(this) {
                    Ok(ok__) => {
                        user.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Xml<Identity: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, xml: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataCollectorSet_Impl::Xml(this) {
                    Ok(ok__) => {
                        xml.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Security<Identity: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrsecurity: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataCollectorSet_Impl::Security(this) {
                    Ok(ok__) => {
                        pbstrsecurity.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetSecurity<Identity: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrsecurity: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDataCollectorSet_Impl::SetSecurity(this, core::mem::transmute(&bstrsecurity)).into()
            }
        }
        unsafe extern "system" fn StopOnCompletion<Identity: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, stop: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataCollectorSet_Impl::StopOnCompletion(this) {
                    Ok(ok__) => {
                        stop.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetStopOnCompletion<Identity: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, stop: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDataCollectorSet_Impl::SetStopOnCompletion(this, core::mem::transmute_copy(&stop)).into()
            }
        }
        unsafe extern "system" fn DataManager<Identity: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, datamanager: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataCollectorSet_Impl::DataManager(this) {
                    Ok(ok__) => {
                        datamanager.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetCredentials<Identity: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, user: *mut core::ffi::c_void, password: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDataCollectorSet_Impl::SetCredentials(this, core::mem::transmute(&user), core::mem::transmute(&password)).into()
            }
        }
        unsafe extern "system" fn Query<Identity: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut core::ffi::c_void, server: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDataCollectorSet_Impl::Query(this, core::mem::transmute(&name), core::mem::transmute(&server)).into()
            }
        }
        unsafe extern "system" fn Commit<Identity: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut core::ffi::c_void, server: *mut core::ffi::c_void, mode: CommitMode, validation: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataCollectorSet_Impl::Commit(this, core::mem::transmute(&name), core::mem::transmute(&server), core::mem::transmute_copy(&mode)) {
                    Ok(ok__) => {
                        validation.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Delete<Identity: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDataCollectorSet_Impl::Delete(this).into()
            }
        }
        unsafe extern "system" fn Start<Identity: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, synchronous: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDataCollectorSet_Impl::Start(this, core::mem::transmute_copy(&synchronous)).into()
            }
        }
        unsafe extern "system" fn Stop<Identity: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, synchronous: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDataCollectorSet_Impl::Stop(this, core::mem::transmute_copy(&synchronous)).into()
            }
        }
        unsafe extern "system" fn SetXml<Identity: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, xml: *mut core::ffi::c_void, validation: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataCollectorSet_Impl::SetXml(this, core::mem::transmute(&xml)) {
                    Ok(ok__) => {
                        validation.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetValue<Identity: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, key: *mut core::ffi::c_void, value: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDataCollectorSet_Impl::SetValue(this, core::mem::transmute(&key), core::mem::transmute(&value)).into()
            }
        }
        unsafe extern "system" fn GetValue<Identity: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, key: *mut core::ffi::c_void, value: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataCollectorSet_Impl::GetValue(this, core::mem::transmute(&key)) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            DataCollectors: DataCollectors::<Identity, OFFSET>,
            Duration: Duration::<Identity, OFFSET>,
            SetDuration: SetDuration::<Identity, OFFSET>,
            Description: Description::<Identity, OFFSET>,
            SetDescription: SetDescription::<Identity, OFFSET>,
            DescriptionUnresolved: DescriptionUnresolved::<Identity, OFFSET>,
            DisplayName: DisplayName::<Identity, OFFSET>,
            SetDisplayName: SetDisplayName::<Identity, OFFSET>,
            DisplayNameUnresolved: DisplayNameUnresolved::<Identity, OFFSET>,
            Keywords: Keywords::<Identity, OFFSET>,
            SetKeywords: SetKeywords::<Identity, OFFSET>,
            LatestOutputLocation: LatestOutputLocation::<Identity, OFFSET>,
            SetLatestOutputLocation: SetLatestOutputLocation::<Identity, OFFSET>,
            Name: Name::<Identity, OFFSET>,
            OutputLocation: OutputLocation::<Identity, OFFSET>,
            RootPath: RootPath::<Identity, OFFSET>,
            SetRootPath: SetRootPath::<Identity, OFFSET>,
            Segment: Segment::<Identity, OFFSET>,
            SetSegment: SetSegment::<Identity, OFFSET>,
            SegmentMaxDuration: SegmentMaxDuration::<Identity, OFFSET>,
            SetSegmentMaxDuration: SetSegmentMaxDuration::<Identity, OFFSET>,
            SegmentMaxSize: SegmentMaxSize::<Identity, OFFSET>,
            SetSegmentMaxSize: SetSegmentMaxSize::<Identity, OFFSET>,
            SerialNumber: SerialNumber::<Identity, OFFSET>,
            SetSerialNumber: SetSerialNumber::<Identity, OFFSET>,
            Server: Server::<Identity, OFFSET>,
            Status: Status::<Identity, OFFSET>,
            Subdirectory: Subdirectory::<Identity, OFFSET>,
            SetSubdirectory: SetSubdirectory::<Identity, OFFSET>,
            SubdirectoryFormat: SubdirectoryFormat::<Identity, OFFSET>,
            SetSubdirectoryFormat: SetSubdirectoryFormat::<Identity, OFFSET>,
            SubdirectoryFormatPattern: SubdirectoryFormatPattern::<Identity, OFFSET>,
            SetSubdirectoryFormatPattern: SetSubdirectoryFormatPattern::<Identity, OFFSET>,
            Task: Task::<Identity, OFFSET>,
            SetTask: SetTask::<Identity, OFFSET>,
            TaskRunAsSelf: TaskRunAsSelf::<Identity, OFFSET>,
            SetTaskRunAsSelf: SetTaskRunAsSelf::<Identity, OFFSET>,
            TaskArguments: TaskArguments::<Identity, OFFSET>,
            SetTaskArguments: SetTaskArguments::<Identity, OFFSET>,
            TaskUserTextArguments: TaskUserTextArguments::<Identity, OFFSET>,
            SetTaskUserTextArguments: SetTaskUserTextArguments::<Identity, OFFSET>,
            Schedules: Schedules::<Identity, OFFSET>,
            SchedulesEnabled: SchedulesEnabled::<Identity, OFFSET>,
            SetSchedulesEnabled: SetSchedulesEnabled::<Identity, OFFSET>,
            UserAccount: UserAccount::<Identity, OFFSET>,
            Xml: Xml::<Identity, OFFSET>,
            Security: Security::<Identity, OFFSET>,
            SetSecurity: SetSecurity::<Identity, OFFSET>,
            StopOnCompletion: StopOnCompletion::<Identity, OFFSET>,
            SetStopOnCompletion: SetStopOnCompletion::<Identity, OFFSET>,
            DataManager: DataManager::<Identity, OFFSET>,
            SetCredentials: SetCredentials::<Identity, OFFSET>,
            Query: Query::<Identity, OFFSET>,
            Commit: Commit::<Identity, OFFSET>,
            Delete: Delete::<Identity, OFFSET>,
            Start: Start::<Identity, OFFSET>,
            Stop: Stop::<Identity, OFFSET>,
            SetXml: SetXml::<Identity, OFFSET>,
            SetValue: SetValue::<Identity, OFFSET>,
            GetValue: GetValue::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDataCollectorSet as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IDataCollectorSet {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IDataCollectorSetCollection, IDataCollectorSetCollection_Vtbl, 0x03837524_098b_11d8_9414_505054503030);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IDataCollectorSetCollection {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IDataCollectorSetCollection, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IDataCollectorSetCollection {
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn get_Item(&self, index: &super::Variant::VARIANT) -> windows_core::Result<IDataCollectorSet> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Item)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(index), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Add<P0>(&self, set: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDataCollectorSet>,
    {
        unsafe { (windows_core::Interface::vtable(self).Add)(windows_core::Interface::as_raw(self), set.param().abi()).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Remove(&self, set: &super::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Remove)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(set)).ok() }
    }
    pub unsafe fn Clear(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Clear)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn AddRange<P0>(&self, sets: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDataCollectorSetCollection>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddRange)(windows_core::Interface::as_raw(self), sets.param().abi()).ok() }
    }
    pub unsafe fn GetDataCollectorSets(&self, server: &windows_core::BSTR, filter: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetDataCollectorSets)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(server), core::mem::transmute_copy(filter)).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IDataCollectorSetCollection_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub get_Item: unsafe extern "system" fn(*mut core::ffi::c_void, super::Variant::VARIANT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    get_Item: usize,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Add: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub Remove: unsafe extern "system" fn(*mut core::ffi::c_void, super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    Remove: usize,
    pub Clear: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddRange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDataCollectorSets: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IDataCollectorSetCollection_Impl: super::Com::IDispatch_Impl {
    fn Count(&self) -> windows_core::Result<i32>;
    fn get_Item(&self, index: &super::Variant::VARIANT) -> windows_core::Result<IDataCollectorSet>;
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn Add(&self, set: windows_core::Ref<IDataCollectorSet>) -> windows_core::Result<()>;
    fn Remove(&self, set: &super::Variant::VARIANT) -> windows_core::Result<()>;
    fn Clear(&self) -> windows_core::Result<()>;
    fn AddRange(&self, sets: windows_core::Ref<IDataCollectorSetCollection>) -> windows_core::Result<()>;
    fn GetDataCollectorSets(&self, server: &windows_core::BSTR, filter: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IDataCollectorSetCollection_Vtbl {
    pub const fn new<Identity: IDataCollectorSetCollection_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Count<Identity: IDataCollectorSetCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataCollectorSetCollection_Impl::Count(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Item<Identity: IDataCollectorSetCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: super::Variant::VARIANT, set: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataCollectorSetCollection_Impl::get_Item(this, core::mem::transmute(&index)) {
                    Ok(ok__) => {
                        set.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: IDataCollectorSetCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataCollectorSetCollection_Impl::_NewEnum(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Add<Identity: IDataCollectorSetCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, set: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDataCollectorSetCollection_Impl::Add(this, core::mem::transmute_copy(&set)).into()
            }
        }
        unsafe extern "system" fn Remove<Identity: IDataCollectorSetCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, set: super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDataCollectorSetCollection_Impl::Remove(this, core::mem::transmute(&set)).into()
            }
        }
        unsafe extern "system" fn Clear<Identity: IDataCollectorSetCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDataCollectorSetCollection_Impl::Clear(this).into()
            }
        }
        unsafe extern "system" fn AddRange<Identity: IDataCollectorSetCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sets: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDataCollectorSetCollection_Impl::AddRange(this, core::mem::transmute_copy(&sets)).into()
            }
        }
        unsafe extern "system" fn GetDataCollectorSets<Identity: IDataCollectorSetCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, server: *mut core::ffi::c_void, filter: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDataCollectorSetCollection_Impl::GetDataCollectorSets(this, core::mem::transmute(&server), core::mem::transmute(&filter)).into()
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Count: Count::<Identity, OFFSET>,
            get_Item: get_Item::<Identity, OFFSET>,
            _NewEnum: _NewEnum::<Identity, OFFSET>,
            Add: Add::<Identity, OFFSET>,
            Remove: Remove::<Identity, OFFSET>,
            Clear: Clear::<Identity, OFFSET>,
            AddRange: AddRange::<Identity, OFFSET>,
            GetDataCollectorSets: GetDataCollectorSets::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDataCollectorSetCollection as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IDataCollectorSetCollection {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IDataManager, IDataManager_Vtbl, 0x03837541_098b_11d8_9414_505054503030);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IDataManager {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IDataManager, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IDataManager {
    pub unsafe fn Enabled(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Enabled)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetEnabled(&self, fenabled: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetEnabled)(windows_core::Interface::as_raw(self), fenabled).ok() }
    }
    pub unsafe fn CheckBeforeRunning(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CheckBeforeRunning)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetCheckBeforeRunning(&self, fcheck: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetCheckBeforeRunning)(windows_core::Interface::as_raw(self), fcheck).ok() }
    }
    pub unsafe fn MinFreeDisk(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MinFreeDisk)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetMinFreeDisk(&self, minfreedisk: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetMinFreeDisk)(windows_core::Interface::as_raw(self), minfreedisk).ok() }
    }
    pub unsafe fn MaxSize(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MaxSize)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetMaxSize(&self, ulmaxsize: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetMaxSize)(windows_core::Interface::as_raw(self), ulmaxsize).ok() }
    }
    pub unsafe fn MaxFolderCount(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MaxFolderCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetMaxFolderCount(&self, ulmaxfoldercount: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetMaxFolderCount)(windows_core::Interface::as_raw(self), ulmaxfoldercount).ok() }
    }
    pub unsafe fn ResourcePolicy(&self) -> windows_core::Result<ResourcePolicy> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ResourcePolicy)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetResourcePolicy(&self, policy: ResourcePolicy) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetResourcePolicy)(windows_core::Interface::as_raw(self), policy).ok() }
    }
    pub unsafe fn FolderActions(&self) -> windows_core::Result<IFolderActionCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FolderActions)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn ReportSchema(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ReportSchema)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetReportSchema(&self, reportschema: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetReportSchema)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(reportschema)).ok() }
    }
    pub unsafe fn ReportFileName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ReportFileName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetReportFileName(&self, pbstrfilename: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetReportFileName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(pbstrfilename)).ok() }
    }
    pub unsafe fn RuleTargetFileName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RuleTargetFileName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetRuleTargetFileName(&self, filename: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetRuleTargetFileName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(filename)).ok() }
    }
    pub unsafe fn EventsFileName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EventsFileName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetEventsFileName(&self, pbstrfilename: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetEventsFileName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(pbstrfilename)).ok() }
    }
    pub unsafe fn Rules(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Rules)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetRules(&self, bstrxml: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetRules)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrxml)).ok() }
    }
    pub unsafe fn Run(&self, steps: DataManagerSteps, bstrfolder: &windows_core::BSTR) -> windows_core::Result<IValueMap> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Run)(windows_core::Interface::as_raw(self), steps, core::mem::transmute_copy(bstrfolder), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Extract(&self, cabfilename: &windows_core::BSTR, destinationpath: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Extract)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(cabfilename), core::mem::transmute_copy(destinationpath)).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IDataManager_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Enabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub CheckBeforeRunning: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetCheckBeforeRunning: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub MinFreeDisk: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetMinFreeDisk: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub MaxSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetMaxSize: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub MaxFolderCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetMaxFolderCount: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub ResourcePolicy: unsafe extern "system" fn(*mut core::ffi::c_void, *mut ResourcePolicy) -> windows_core::HRESULT,
    pub SetResourcePolicy: unsafe extern "system" fn(*mut core::ffi::c_void, ResourcePolicy) -> windows_core::HRESULT,
    pub FolderActions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ReportSchema: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetReportSchema: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ReportFileName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetReportFileName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RuleTargetFileName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetRuleTargetFileName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EventsFileName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetEventsFileName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Rules: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetRules: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Run: unsafe extern "system" fn(*mut core::ffi::c_void, DataManagerSteps, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Extract: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IDataManager_Impl: super::Com::IDispatch_Impl {
    fn Enabled(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetEnabled(&self, fenabled: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn CheckBeforeRunning(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetCheckBeforeRunning(&self, fcheck: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn MinFreeDisk(&self) -> windows_core::Result<u32>;
    fn SetMinFreeDisk(&self, minfreedisk: u32) -> windows_core::Result<()>;
    fn MaxSize(&self) -> windows_core::Result<u32>;
    fn SetMaxSize(&self, ulmaxsize: u32) -> windows_core::Result<()>;
    fn MaxFolderCount(&self) -> windows_core::Result<u32>;
    fn SetMaxFolderCount(&self, ulmaxfoldercount: u32) -> windows_core::Result<()>;
    fn ResourcePolicy(&self) -> windows_core::Result<ResourcePolicy>;
    fn SetResourcePolicy(&self, policy: ResourcePolicy) -> windows_core::Result<()>;
    fn FolderActions(&self) -> windows_core::Result<IFolderActionCollection>;
    fn ReportSchema(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetReportSchema(&self, reportschema: &windows_core::BSTR) -> windows_core::Result<()>;
    fn ReportFileName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetReportFileName(&self, pbstrfilename: &windows_core::BSTR) -> windows_core::Result<()>;
    fn RuleTargetFileName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetRuleTargetFileName(&self, filename: &windows_core::BSTR) -> windows_core::Result<()>;
    fn EventsFileName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetEventsFileName(&self, pbstrfilename: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Rules(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetRules(&self, bstrxml: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Run(&self, steps: DataManagerSteps, bstrfolder: &windows_core::BSTR) -> windows_core::Result<IValueMap>;
    fn Extract(&self, cabfilename: &windows_core::BSTR, destinationpath: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IDataManager_Vtbl {
    pub const fn new<Identity: IDataManager_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Enabled<Identity: IDataManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfenabled: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataManager_Impl::Enabled(this) {
                    Ok(ok__) => {
                        pfenabled.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetEnabled<Identity: IDataManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fenabled: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDataManager_Impl::SetEnabled(this, core::mem::transmute_copy(&fenabled)).into()
            }
        }
        unsafe extern "system" fn CheckBeforeRunning<Identity: IDataManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfcheck: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataManager_Impl::CheckBeforeRunning(this) {
                    Ok(ok__) => {
                        pfcheck.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetCheckBeforeRunning<Identity: IDataManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fcheck: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDataManager_Impl::SetCheckBeforeRunning(this, core::mem::transmute_copy(&fcheck)).into()
            }
        }
        unsafe extern "system" fn MinFreeDisk<Identity: IDataManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, minfreedisk: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataManager_Impl::MinFreeDisk(this) {
                    Ok(ok__) => {
                        minfreedisk.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetMinFreeDisk<Identity: IDataManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, minfreedisk: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDataManager_Impl::SetMinFreeDisk(this, core::mem::transmute_copy(&minfreedisk)).into()
            }
        }
        unsafe extern "system" fn MaxSize<Identity: IDataManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pulmaxsize: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataManager_Impl::MaxSize(this) {
                    Ok(ok__) => {
                        pulmaxsize.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetMaxSize<Identity: IDataManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulmaxsize: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDataManager_Impl::SetMaxSize(this, core::mem::transmute_copy(&ulmaxsize)).into()
            }
        }
        unsafe extern "system" fn MaxFolderCount<Identity: IDataManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pulmaxfoldercount: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataManager_Impl::MaxFolderCount(this) {
                    Ok(ok__) => {
                        pulmaxfoldercount.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetMaxFolderCount<Identity: IDataManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulmaxfoldercount: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDataManager_Impl::SetMaxFolderCount(this, core::mem::transmute_copy(&ulmaxfoldercount)).into()
            }
        }
        unsafe extern "system" fn ResourcePolicy<Identity: IDataManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppolicy: *mut ResourcePolicy) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataManager_Impl::ResourcePolicy(this) {
                    Ok(ok__) => {
                        ppolicy.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetResourcePolicy<Identity: IDataManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, policy: ResourcePolicy) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDataManager_Impl::SetResourcePolicy(this, core::mem::transmute_copy(&policy)).into()
            }
        }
        unsafe extern "system" fn FolderActions<Identity: IDataManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, actions: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataManager_Impl::FolderActions(this) {
                    Ok(ok__) => {
                        actions.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ReportSchema<Identity: IDataManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, reportschema: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataManager_Impl::ReportSchema(this) {
                    Ok(ok__) => {
                        reportschema.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetReportSchema<Identity: IDataManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, reportschema: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDataManager_Impl::SetReportSchema(this, core::mem::transmute(&reportschema)).into()
            }
        }
        unsafe extern "system" fn ReportFileName<Identity: IDataManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrfilename: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataManager_Impl::ReportFileName(this) {
                    Ok(ok__) => {
                        pbstrfilename.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetReportFileName<Identity: IDataManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrfilename: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDataManager_Impl::SetReportFileName(this, core::mem::transmute(&pbstrfilename)).into()
            }
        }
        unsafe extern "system" fn RuleTargetFileName<Identity: IDataManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, filename: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataManager_Impl::RuleTargetFileName(this) {
                    Ok(ok__) => {
                        filename.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetRuleTargetFileName<Identity: IDataManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, filename: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDataManager_Impl::SetRuleTargetFileName(this, core::mem::transmute(&filename)).into()
            }
        }
        unsafe extern "system" fn EventsFileName<Identity: IDataManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrfilename: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataManager_Impl::EventsFileName(this) {
                    Ok(ok__) => {
                        pbstrfilename.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetEventsFileName<Identity: IDataManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrfilename: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDataManager_Impl::SetEventsFileName(this, core::mem::transmute(&pbstrfilename)).into()
            }
        }
        unsafe extern "system" fn Rules<Identity: IDataManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrxml: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataManager_Impl::Rules(this) {
                    Ok(ok__) => {
                        pbstrxml.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetRules<Identity: IDataManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrxml: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDataManager_Impl::SetRules(this, core::mem::transmute(&bstrxml)).into()
            }
        }
        unsafe extern "system" fn Run<Identity: IDataManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, steps: DataManagerSteps, bstrfolder: *mut core::ffi::c_void, errors: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataManager_Impl::Run(this, core::mem::transmute_copy(&steps), core::mem::transmute(&bstrfolder)) {
                    Ok(ok__) => {
                        errors.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Extract<Identity: IDataManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cabfilename: *mut core::ffi::c_void, destinationpath: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDataManager_Impl::Extract(this, core::mem::transmute(&cabfilename), core::mem::transmute(&destinationpath)).into()
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Enabled: Enabled::<Identity, OFFSET>,
            SetEnabled: SetEnabled::<Identity, OFFSET>,
            CheckBeforeRunning: CheckBeforeRunning::<Identity, OFFSET>,
            SetCheckBeforeRunning: SetCheckBeforeRunning::<Identity, OFFSET>,
            MinFreeDisk: MinFreeDisk::<Identity, OFFSET>,
            SetMinFreeDisk: SetMinFreeDisk::<Identity, OFFSET>,
            MaxSize: MaxSize::<Identity, OFFSET>,
            SetMaxSize: SetMaxSize::<Identity, OFFSET>,
            MaxFolderCount: MaxFolderCount::<Identity, OFFSET>,
            SetMaxFolderCount: SetMaxFolderCount::<Identity, OFFSET>,
            ResourcePolicy: ResourcePolicy::<Identity, OFFSET>,
            SetResourcePolicy: SetResourcePolicy::<Identity, OFFSET>,
            FolderActions: FolderActions::<Identity, OFFSET>,
            ReportSchema: ReportSchema::<Identity, OFFSET>,
            SetReportSchema: SetReportSchema::<Identity, OFFSET>,
            ReportFileName: ReportFileName::<Identity, OFFSET>,
            SetReportFileName: SetReportFileName::<Identity, OFFSET>,
            RuleTargetFileName: RuleTargetFileName::<Identity, OFFSET>,
            SetRuleTargetFileName: SetRuleTargetFileName::<Identity, OFFSET>,
            EventsFileName: EventsFileName::<Identity, OFFSET>,
            SetEventsFileName: SetEventsFileName::<Identity, OFFSET>,
            Rules: Rules::<Identity, OFFSET>,
            SetRules: SetRules::<Identity, OFFSET>,
            Run: Run::<Identity, OFFSET>,
            Extract: Extract::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDataManager as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IDataManager {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IFolderAction, IFolderAction_Vtbl, 0x03837543_098b_11d8_9414_505054503030);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IFolderAction {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IFolderAction, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IFolderAction {
    pub unsafe fn Age(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Age)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetAge(&self, ulage: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetAge)(windows_core::Interface::as_raw(self), ulage).ok() }
    }
    pub unsafe fn Size(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Size)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetSize(&self, ulage: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetSize)(windows_core::Interface::as_raw(self), ulage).ok() }
    }
    pub unsafe fn Actions(&self) -> windows_core::Result<FolderActionSteps> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Actions)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetActions(&self, steps: FolderActionSteps) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetActions)(windows_core::Interface::as_raw(self), steps).ok() }
    }
    pub unsafe fn SendCabTo(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SendCabTo)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetSendCabTo(&self, bstrdestination: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetSendCabTo)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrdestination)).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFolderAction_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Age: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetAge: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Size: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetSize: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Actions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut FolderActionSteps) -> windows_core::HRESULT,
    pub SetActions: unsafe extern "system" fn(*mut core::ffi::c_void, FolderActionSteps) -> windows_core::HRESULT,
    pub SendCabTo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetSendCabTo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFolderAction_Impl: super::Com::IDispatch_Impl {
    fn Age(&self) -> windows_core::Result<u32>;
    fn SetAge(&self, ulage: u32) -> windows_core::Result<()>;
    fn Size(&self) -> windows_core::Result<u32>;
    fn SetSize(&self, ulage: u32) -> windows_core::Result<()>;
    fn Actions(&self) -> windows_core::Result<FolderActionSteps>;
    fn SetActions(&self, steps: FolderActionSteps) -> windows_core::Result<()>;
    fn SendCabTo(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetSendCabTo(&self, bstrdestination: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IFolderAction_Vtbl {
    pub const fn new<Identity: IFolderAction_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Age<Identity: IFolderAction_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pulage: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFolderAction_Impl::Age(this) {
                    Ok(ok__) => {
                        pulage.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetAge<Identity: IFolderAction_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulage: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFolderAction_Impl::SetAge(this, core::mem::transmute_copy(&ulage)).into()
            }
        }
        unsafe extern "system" fn Size<Identity: IFolderAction_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pulage: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFolderAction_Impl::Size(this) {
                    Ok(ok__) => {
                        pulage.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetSize<Identity: IFolderAction_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulage: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFolderAction_Impl::SetSize(this, core::mem::transmute_copy(&ulage)).into()
            }
        }
        unsafe extern "system" fn Actions<Identity: IFolderAction_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, steps: *mut FolderActionSteps) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFolderAction_Impl::Actions(this) {
                    Ok(ok__) => {
                        steps.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetActions<Identity: IFolderAction_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, steps: FolderActionSteps) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFolderAction_Impl::SetActions(this, core::mem::transmute_copy(&steps)).into()
            }
        }
        unsafe extern "system" fn SendCabTo<Identity: IFolderAction_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrdestination: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFolderAction_Impl::SendCabTo(this) {
                    Ok(ok__) => {
                        pbstrdestination.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetSendCabTo<Identity: IFolderAction_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrdestination: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFolderAction_Impl::SetSendCabTo(this, core::mem::transmute(&bstrdestination)).into()
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Age: Age::<Identity, OFFSET>,
            SetAge: SetAge::<Identity, OFFSET>,
            Size: Size::<Identity, OFFSET>,
            SetSize: SetSize::<Identity, OFFSET>,
            Actions: Actions::<Identity, OFFSET>,
            SetActions: SetActions::<Identity, OFFSET>,
            SendCabTo: SendCabTo::<Identity, OFFSET>,
            SetSendCabTo: SetSendCabTo::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFolderAction as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IFolderAction {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IFolderActionCollection, IFolderActionCollection_Vtbl, 0x03837544_098b_11d8_9414_505054503030);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IFolderActionCollection {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IFolderActionCollection, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IFolderActionCollection {
    pub unsafe fn Count(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn get_Item(&self, index: &super::Variant::VARIANT) -> windows_core::Result<IFolderAction> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Item)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(index), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Add<P0>(&self, action: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IFolderAction>,
    {
        unsafe { (windows_core::Interface::vtable(self).Add)(windows_core::Interface::as_raw(self), action.param().abi()).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Remove(&self, index: &super::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Remove)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(index)).ok() }
    }
    pub unsafe fn Clear(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Clear)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn AddRange<P0>(&self, actions: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IFolderActionCollection>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddRange)(windows_core::Interface::as_raw(self), actions.param().abi()).ok() }
    }
    pub unsafe fn CreateFolderAction(&self) -> windows_core::Result<IFolderAction> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateFolderAction)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFolderActionCollection_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub get_Item: unsafe extern "system" fn(*mut core::ffi::c_void, super::Variant::VARIANT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    get_Item: usize,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Add: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub Remove: unsafe extern "system" fn(*mut core::ffi::c_void, super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    Remove: usize,
    pub Clear: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddRange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateFolderAction: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFolderActionCollection_Impl: super::Com::IDispatch_Impl {
    fn Count(&self) -> windows_core::Result<u32>;
    fn get_Item(&self, index: &super::Variant::VARIANT) -> windows_core::Result<IFolderAction>;
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn Add(&self, action: windows_core::Ref<IFolderAction>) -> windows_core::Result<()>;
    fn Remove(&self, index: &super::Variant::VARIANT) -> windows_core::Result<()>;
    fn Clear(&self) -> windows_core::Result<()>;
    fn AddRange(&self, actions: windows_core::Ref<IFolderActionCollection>) -> windows_core::Result<()>;
    fn CreateFolderAction(&self) -> windows_core::Result<IFolderAction>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IFolderActionCollection_Vtbl {
    pub const fn new<Identity: IFolderActionCollection_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Count<Identity: IFolderActionCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, count: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFolderActionCollection_Impl::Count(this) {
                    Ok(ok__) => {
                        count.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Item<Identity: IFolderActionCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: super::Variant::VARIANT, action: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFolderActionCollection_Impl::get_Item(this, core::mem::transmute(&index)) {
                    Ok(ok__) => {
                        action.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: IFolderActionCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, r#enum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFolderActionCollection_Impl::_NewEnum(this) {
                    Ok(ok__) => {
                        r#enum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Add<Identity: IFolderActionCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, action: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFolderActionCollection_Impl::Add(this, core::mem::transmute_copy(&action)).into()
            }
        }
        unsafe extern "system" fn Remove<Identity: IFolderActionCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFolderActionCollection_Impl::Remove(this, core::mem::transmute(&index)).into()
            }
        }
        unsafe extern "system" fn Clear<Identity: IFolderActionCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFolderActionCollection_Impl::Clear(this).into()
            }
        }
        unsafe extern "system" fn AddRange<Identity: IFolderActionCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, actions: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFolderActionCollection_Impl::AddRange(this, core::mem::transmute_copy(&actions)).into()
            }
        }
        unsafe extern "system" fn CreateFolderAction<Identity: IFolderActionCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, folderaction: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFolderActionCollection_Impl::CreateFolderAction(this) {
                    Ok(ok__) => {
                        folderaction.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Count: Count::<Identity, OFFSET>,
            get_Item: get_Item::<Identity, OFFSET>,
            _NewEnum: _NewEnum::<Identity, OFFSET>,
            Add: Add::<Identity, OFFSET>,
            Remove: Remove::<Identity, OFFSET>,
            Clear: Clear::<Identity, OFFSET>,
            AddRange: AddRange::<Identity, OFFSET>,
            CreateFolderAction: CreateFolderAction::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFolderActionCollection as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IFolderActionCollection {}
windows_core::imp::define_interface!(ILogFileItem, ILogFileItem_Vtbl, 0xd6b518dd_05c7_418a_89e6_4f9ce8c6841e);
windows_core::imp::interface_hierarchy!(ILogFileItem, windows_core::IUnknown);
impl ILogFileItem {
    pub unsafe fn Path(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Path)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ILogFileItem_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Path: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ILogFileItem_Impl: windows_core::IUnknownImpl {
    fn Path(&self) -> windows_core::Result<windows_core::BSTR>;
}
impl ILogFileItem_Vtbl {
    pub const fn new<Identity: ILogFileItem_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Path<Identity: ILogFileItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstrvalue: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ILogFileItem_Impl::Path(this) {
                    Ok(ok__) => {
                        pstrvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Path: Path::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ILogFileItem as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ILogFileItem {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ILogFiles, ILogFiles_Vtbl, 0x6a2a97e6_6851_41ea_87ad_2a8225335865);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ILogFiles {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ILogFiles, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ILogFiles {
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn get_Item(&self, index: &super::Variant::VARIANT) -> windows_core::Result<DILogFileItem> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Item)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(index), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Add(&self, pathname: &windows_core::BSTR) -> windows_core::Result<DILogFileItem> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Add)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(pathname), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Remove(&self, index: &super::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Remove)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(index)).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ILogFiles_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub get_Item: unsafe extern "system" fn(*mut core::ffi::c_void, super::Variant::VARIANT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    get_Item: usize,
    pub Add: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub Remove: unsafe extern "system" fn(*mut core::ffi::c_void, super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    Remove: usize,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ILogFiles_Impl: super::Com::IDispatch_Impl {
    fn Count(&self) -> windows_core::Result<i32>;
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn get_Item(&self, index: &super::Variant::VARIANT) -> windows_core::Result<DILogFileItem>;
    fn Add(&self, pathname: &windows_core::BSTR) -> windows_core::Result<DILogFileItem>;
    fn Remove(&self, index: &super::Variant::VARIANT) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ILogFiles_Vtbl {
    pub const fn new<Identity: ILogFiles_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Count<Identity: ILogFiles_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plong: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ILogFiles_Impl::Count(this) {
                    Ok(ok__) => {
                        plong.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: ILogFiles_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppiunk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ILogFiles_Impl::_NewEnum(this) {
                    Ok(ok__) => {
                        ppiunk.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Item<Identity: ILogFiles_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: super::Variant::VARIANT, ppi: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ILogFiles_Impl::get_Item(this, core::mem::transmute(&index)) {
                    Ok(ok__) => {
                        ppi.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Add<Identity: ILogFiles_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pathname: *mut core::ffi::c_void, ppi: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ILogFiles_Impl::Add(this, core::mem::transmute(&pathname)) {
                    Ok(ok__) => {
                        ppi.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Remove<Identity: ILogFiles_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ILogFiles_Impl::Remove(this, core::mem::transmute(&index)).into()
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Count: Count::<Identity, OFFSET>,
            _NewEnum: _NewEnum::<Identity, OFFSET>,
            get_Item: get_Item::<Identity, OFFSET>,
            Add: Add::<Identity, OFFSET>,
            Remove: Remove::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ILogFiles as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ILogFiles {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IPerformanceCounterDataCollector, IPerformanceCounterDataCollector_Vtbl, 0x03837506_098b_11d8_9414_505054503030);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IPerformanceCounterDataCollector {
    type Target = IDataCollector;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IPerformanceCounterDataCollector, windows_core::IUnknown, super::Com::IDispatch, IDataCollector);
#[cfg(feature = "Win32_System_Com")]
impl IPerformanceCounterDataCollector {
    pub unsafe fn DataSourceName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DataSourceName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetDataSourceName(&self, dsn: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetDataSourceName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(dsn)).ok() }
    }
    pub unsafe fn PerformanceCounters(&self) -> windows_core::Result<*mut super::Com::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PerformanceCounters)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetPerformanceCounters(&self, counters: *const super::Com::SAFEARRAY) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetPerformanceCounters)(windows_core::Interface::as_raw(self), counters).ok() }
    }
    pub unsafe fn LogFileFormat(&self) -> windows_core::Result<FileFormat> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).LogFileFormat)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetLogFileFormat(&self, format: FileFormat) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetLogFileFormat)(windows_core::Interface::as_raw(self), format).ok() }
    }
    pub unsafe fn SampleInterval(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SampleInterval)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetSampleInterval(&self, interval: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetSampleInterval)(windows_core::Interface::as_raw(self), interval).ok() }
    }
    pub unsafe fn SegmentMaxRecords(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SegmentMaxRecords)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetSegmentMaxRecords(&self, records: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetSegmentMaxRecords)(windows_core::Interface::as_raw(self), records).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IPerformanceCounterDataCollector_Vtbl {
    pub base__: IDataCollector_Vtbl,
    pub DataSourceName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetDataSourceName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PerformanceCounters: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::Com::SAFEARRAY) -> windows_core::HRESULT,
    pub SetPerformanceCounters: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::Com::SAFEARRAY) -> windows_core::HRESULT,
    pub LogFileFormat: unsafe extern "system" fn(*mut core::ffi::c_void, *mut FileFormat) -> windows_core::HRESULT,
    pub SetLogFileFormat: unsafe extern "system" fn(*mut core::ffi::c_void, FileFormat) -> windows_core::HRESULT,
    pub SampleInterval: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetSampleInterval: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub SegmentMaxRecords: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetSegmentMaxRecords: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IPerformanceCounterDataCollector_Impl: IDataCollector_Impl {
    fn DataSourceName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetDataSourceName(&self, dsn: &windows_core::BSTR) -> windows_core::Result<()>;
    fn PerformanceCounters(&self) -> windows_core::Result<*mut super::Com::SAFEARRAY>;
    fn SetPerformanceCounters(&self, counters: *const super::Com::SAFEARRAY) -> windows_core::Result<()>;
    fn LogFileFormat(&self) -> windows_core::Result<FileFormat>;
    fn SetLogFileFormat(&self, format: FileFormat) -> windows_core::Result<()>;
    fn SampleInterval(&self) -> windows_core::Result<u32>;
    fn SetSampleInterval(&self, interval: u32) -> windows_core::Result<()>;
    fn SegmentMaxRecords(&self) -> windows_core::Result<u32>;
    fn SetSegmentMaxRecords(&self, records: u32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IPerformanceCounterDataCollector_Vtbl {
    pub const fn new<Identity: IPerformanceCounterDataCollector_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn DataSourceName<Identity: IPerformanceCounterDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dsn: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPerformanceCounterDataCollector_Impl::DataSourceName(this) {
                    Ok(ok__) => {
                        dsn.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetDataSourceName<Identity: IPerformanceCounterDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dsn: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPerformanceCounterDataCollector_Impl::SetDataSourceName(this, core::mem::transmute(&dsn)).into()
            }
        }
        unsafe extern "system" fn PerformanceCounters<Identity: IPerformanceCounterDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, counters: *mut *mut super::Com::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPerformanceCounterDataCollector_Impl::PerformanceCounters(this) {
                    Ok(ok__) => {
                        counters.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetPerformanceCounters<Identity: IPerformanceCounterDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, counters: *const super::Com::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPerformanceCounterDataCollector_Impl::SetPerformanceCounters(this, core::mem::transmute_copy(&counters)).into()
            }
        }
        unsafe extern "system" fn LogFileFormat<Identity: IPerformanceCounterDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, format: *mut FileFormat) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPerformanceCounterDataCollector_Impl::LogFileFormat(this) {
                    Ok(ok__) => {
                        format.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetLogFileFormat<Identity: IPerformanceCounterDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, format: FileFormat) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPerformanceCounterDataCollector_Impl::SetLogFileFormat(this, core::mem::transmute_copy(&format)).into()
            }
        }
        unsafe extern "system" fn SampleInterval<Identity: IPerformanceCounterDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, interval: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPerformanceCounterDataCollector_Impl::SampleInterval(this) {
                    Ok(ok__) => {
                        interval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetSampleInterval<Identity: IPerformanceCounterDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, interval: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPerformanceCounterDataCollector_Impl::SetSampleInterval(this, core::mem::transmute_copy(&interval)).into()
            }
        }
        unsafe extern "system" fn SegmentMaxRecords<Identity: IPerformanceCounterDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, records: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPerformanceCounterDataCollector_Impl::SegmentMaxRecords(this) {
                    Ok(ok__) => {
                        records.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetSegmentMaxRecords<Identity: IPerformanceCounterDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, records: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPerformanceCounterDataCollector_Impl::SetSegmentMaxRecords(this, core::mem::transmute_copy(&records)).into()
            }
        }
        Self {
            base__: IDataCollector_Vtbl::new::<Identity, OFFSET>(),
            DataSourceName: DataSourceName::<Identity, OFFSET>,
            SetDataSourceName: SetDataSourceName::<Identity, OFFSET>,
            PerformanceCounters: PerformanceCounters::<Identity, OFFSET>,
            SetPerformanceCounters: SetPerformanceCounters::<Identity, OFFSET>,
            LogFileFormat: LogFileFormat::<Identity, OFFSET>,
            SetLogFileFormat: SetLogFileFormat::<Identity, OFFSET>,
            SampleInterval: SampleInterval::<Identity, OFFSET>,
            SetSampleInterval: SetSampleInterval::<Identity, OFFSET>,
            SegmentMaxRecords: SegmentMaxRecords::<Identity, OFFSET>,
            SetSegmentMaxRecords: SetSegmentMaxRecords::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPerformanceCounterDataCollector as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID || iid == &<IDataCollector as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IPerformanceCounterDataCollector {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ISchedule, ISchedule_Vtbl, 0x0383753a_098b_11d8_9414_505054503030);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ISchedule {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ISchedule, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ISchedule {
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn StartDate(&self) -> windows_core::Result<super::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).StartDate)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn SetStartDate(&self, start: &super::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetStartDate)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(start)).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn EndDate(&self) -> windows_core::Result<super::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EndDate)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn SetEndDate(&self, end: &super::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetEndDate)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(end)).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn StartTime(&self) -> windows_core::Result<super::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).StartTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn SetStartTime(&self, start: &super::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetStartTime)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(start)).ok() }
    }
    pub unsafe fn Days(&self) -> windows_core::Result<WeekDays> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Days)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetDays(&self, days: WeekDays) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetDays)(windows_core::Interface::as_raw(self), days).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISchedule_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub StartDate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    StartDate: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub SetStartDate: unsafe extern "system" fn(*mut core::ffi::c_void, super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    SetStartDate: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub EndDate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    EndDate: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub SetEndDate: unsafe extern "system" fn(*mut core::ffi::c_void, super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    SetEndDate: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub StartTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    StartTime: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub SetStartTime: unsafe extern "system" fn(*mut core::ffi::c_void, super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    SetStartTime: usize,
    pub Days: unsafe extern "system" fn(*mut core::ffi::c_void, *mut WeekDays) -> windows_core::HRESULT,
    pub SetDays: unsafe extern "system" fn(*mut core::ffi::c_void, WeekDays) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISchedule_Impl: super::Com::IDispatch_Impl {
    fn StartDate(&self) -> windows_core::Result<super::Variant::VARIANT>;
    fn SetStartDate(&self, start: &super::Variant::VARIANT) -> windows_core::Result<()>;
    fn EndDate(&self) -> windows_core::Result<super::Variant::VARIANT>;
    fn SetEndDate(&self, end: &super::Variant::VARIANT) -> windows_core::Result<()>;
    fn StartTime(&self) -> windows_core::Result<super::Variant::VARIANT>;
    fn SetStartTime(&self, start: &super::Variant::VARIANT) -> windows_core::Result<()>;
    fn Days(&self) -> windows_core::Result<WeekDays>;
    fn SetDays(&self, days: WeekDays) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISchedule_Vtbl {
    pub const fn new<Identity: ISchedule_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn StartDate<Identity: ISchedule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, start: *mut super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISchedule_Impl::StartDate(this) {
                    Ok(ok__) => {
                        start.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetStartDate<Identity: ISchedule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, start: super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISchedule_Impl::SetStartDate(this, core::mem::transmute(&start)).into()
            }
        }
        unsafe extern "system" fn EndDate<Identity: ISchedule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, end: *mut super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISchedule_Impl::EndDate(this) {
                    Ok(ok__) => {
                        end.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetEndDate<Identity: ISchedule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, end: super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISchedule_Impl::SetEndDate(this, core::mem::transmute(&end)).into()
            }
        }
        unsafe extern "system" fn StartTime<Identity: ISchedule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, start: *mut super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISchedule_Impl::StartTime(this) {
                    Ok(ok__) => {
                        start.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetStartTime<Identity: ISchedule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, start: super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISchedule_Impl::SetStartTime(this, core::mem::transmute(&start)).into()
            }
        }
        unsafe extern "system" fn Days<Identity: ISchedule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, days: *mut WeekDays) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISchedule_Impl::Days(this) {
                    Ok(ok__) => {
                        days.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetDays<Identity: ISchedule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, days: WeekDays) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISchedule_Impl::SetDays(this, core::mem::transmute_copy(&days)).into()
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            StartDate: StartDate::<Identity, OFFSET>,
            SetStartDate: SetStartDate::<Identity, OFFSET>,
            EndDate: EndDate::<Identity, OFFSET>,
            SetEndDate: SetEndDate::<Identity, OFFSET>,
            StartTime: StartTime::<Identity, OFFSET>,
            SetStartTime: SetStartTime::<Identity, OFFSET>,
            Days: Days::<Identity, OFFSET>,
            SetDays: SetDays::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISchedule as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ISchedule {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IScheduleCollection, IScheduleCollection_Vtbl, 0x0383753d_098b_11d8_9414_505054503030);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IScheduleCollection {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IScheduleCollection, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IScheduleCollection {
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn get_Item(&self, index: &super::Variant::VARIANT) -> windows_core::Result<ISchedule> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Item)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(index), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Add<P0>(&self, pschedule: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ISchedule>,
    {
        unsafe { (windows_core::Interface::vtable(self).Add)(windows_core::Interface::as_raw(self), pschedule.param().abi()).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Remove(&self, vschedule: &super::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Remove)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(vschedule)).ok() }
    }
    pub unsafe fn Clear(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Clear)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn AddRange<P0>(&self, pschedules: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IScheduleCollection>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddRange)(windows_core::Interface::as_raw(self), pschedules.param().abi()).ok() }
    }
    pub unsafe fn CreateSchedule(&self) -> windows_core::Result<ISchedule> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateSchedule)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IScheduleCollection_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub get_Item: unsafe extern "system" fn(*mut core::ffi::c_void, super::Variant::VARIANT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    get_Item: usize,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Add: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub Remove: unsafe extern "system" fn(*mut core::ffi::c_void, super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    Remove: usize,
    pub Clear: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddRange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateSchedule: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IScheduleCollection_Impl: super::Com::IDispatch_Impl {
    fn Count(&self) -> windows_core::Result<i32>;
    fn get_Item(&self, index: &super::Variant::VARIANT) -> windows_core::Result<ISchedule>;
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn Add(&self, pschedule: windows_core::Ref<ISchedule>) -> windows_core::Result<()>;
    fn Remove(&self, vschedule: &super::Variant::VARIANT) -> windows_core::Result<()>;
    fn Clear(&self) -> windows_core::Result<()>;
    fn AddRange(&self, pschedules: windows_core::Ref<IScheduleCollection>) -> windows_core::Result<()>;
    fn CreateSchedule(&self) -> windows_core::Result<ISchedule>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IScheduleCollection_Vtbl {
    pub const fn new<Identity: IScheduleCollection_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Count<Identity: IScheduleCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IScheduleCollection_Impl::Count(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Item<Identity: IScheduleCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: super::Variant::VARIANT, ppschedule: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IScheduleCollection_Impl::get_Item(this, core::mem::transmute(&index)) {
                    Ok(ok__) => {
                        ppschedule.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: IScheduleCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ienum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IScheduleCollection_Impl::_NewEnum(this) {
                    Ok(ok__) => {
                        ienum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Add<Identity: IScheduleCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pschedule: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IScheduleCollection_Impl::Add(this, core::mem::transmute_copy(&pschedule)).into()
            }
        }
        unsafe extern "system" fn Remove<Identity: IScheduleCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vschedule: super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IScheduleCollection_Impl::Remove(this, core::mem::transmute(&vschedule)).into()
            }
        }
        unsafe extern "system" fn Clear<Identity: IScheduleCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IScheduleCollection_Impl::Clear(this).into()
            }
        }
        unsafe extern "system" fn AddRange<Identity: IScheduleCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pschedules: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IScheduleCollection_Impl::AddRange(this, core::mem::transmute_copy(&pschedules)).into()
            }
        }
        unsafe extern "system" fn CreateSchedule<Identity: IScheduleCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, schedule: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IScheduleCollection_Impl::CreateSchedule(this) {
                    Ok(ok__) => {
                        schedule.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Count: Count::<Identity, OFFSET>,
            get_Item: get_Item::<Identity, OFFSET>,
            _NewEnum: _NewEnum::<Identity, OFFSET>,
            Add: Add::<Identity, OFFSET>,
            Remove: Remove::<Identity, OFFSET>,
            Clear: Clear::<Identity, OFFSET>,
            AddRange: AddRange::<Identity, OFFSET>,
            CreateSchedule: CreateSchedule::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IScheduleCollection as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IScheduleCollection {}
windows_core::imp::define_interface!(ISystemMonitor, ISystemMonitor_Vtbl, 0x194eb241_c32c_11cf_9398_00aa00a3ddea);
windows_core::imp::interface_hierarchy!(ISystemMonitor, windows_core::IUnknown);
impl ISystemMonitor {
    pub unsafe fn Appearance(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Appearance)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetAppearance(&self, iappearance: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetAppearance)(windows_core::Interface::as_raw(self), iappearance).ok() }
    }
    pub unsafe fn BackColor(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).BackColor)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetBackColor(&self, color: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetBackColor)(windows_core::Interface::as_raw(self), color).ok() }
    }
    pub unsafe fn BorderStyle(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).BorderStyle)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetBorderStyle(&self, iborderstyle: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetBorderStyle)(windows_core::Interface::as_raw(self), iborderstyle).ok() }
    }
    pub unsafe fn ForeColor(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ForeColor)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetForeColor(&self, color: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetForeColor)(windows_core::Interface::as_raw(self), color).ok() }
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Font(&self) -> windows_core::Result<super::Ole::IFontDisp> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Font)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn putref_Font<P0>(&self, pfont: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Ole::IFontDisp>,
    {
        unsafe { (windows_core::Interface::vtable(self).putref_Font)(windows_core::Interface::as_raw(self), pfont.param().abi()).ok() }
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Counters(&self) -> windows_core::Result<ICounters> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Counters)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetShowVerticalGrid(&self, bstate: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetShowVerticalGrid)(windows_core::Interface::as_raw(self), bstate).ok() }
    }
    pub unsafe fn ShowVerticalGrid(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ShowVerticalGrid)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetShowHorizontalGrid(&self, bstate: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetShowHorizontalGrid)(windows_core::Interface::as_raw(self), bstate).ok() }
    }
    pub unsafe fn ShowHorizontalGrid(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ShowHorizontalGrid)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetShowLegend(&self, bstate: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetShowLegend)(windows_core::Interface::as_raw(self), bstate).ok() }
    }
    pub unsafe fn ShowLegend(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ShowLegend)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetShowScaleLabels(&self, bstate: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetShowScaleLabels)(windows_core::Interface::as_raw(self), bstate).ok() }
    }
    pub unsafe fn ShowScaleLabels(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ShowScaleLabels)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetShowValueBar(&self, bstate: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetShowValueBar)(windows_core::Interface::as_raw(self), bstate).ok() }
    }
    pub unsafe fn ShowValueBar(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ShowValueBar)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetMaximumScale(&self, ivalue: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetMaximumScale)(windows_core::Interface::as_raw(self), ivalue).ok() }
    }
    pub unsafe fn MaximumScale(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MaximumScale)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetMinimumScale(&self, ivalue: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetMinimumScale)(windows_core::Interface::as_raw(self), ivalue).ok() }
    }
    pub unsafe fn MinimumScale(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MinimumScale)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetUpdateInterval(&self, fvalue: f32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetUpdateInterval)(windows_core::Interface::as_raw(self), fvalue).ok() }
    }
    pub unsafe fn UpdateInterval(&self) -> windows_core::Result<f32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).UpdateInterval)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetDisplayType(&self, edisplaytype: DisplayTypeConstants) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetDisplayType)(windows_core::Interface::as_raw(self), edisplaytype).ok() }
    }
    pub unsafe fn DisplayType(&self) -> windows_core::Result<DisplayTypeConstants> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DisplayType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetManualUpdate(&self, bstate: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetManualUpdate)(windows_core::Interface::as_raw(self), bstate).ok() }
    }
    pub unsafe fn ManualUpdate(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ManualUpdate)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetGraphTitle(&self, bstitle: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetGraphTitle)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstitle)).ok() }
    }
    pub unsafe fn GraphTitle(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GraphTitle)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetYAxisLabel(&self, bstitle: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetYAxisLabel)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstitle)).ok() }
    }
    pub unsafe fn YAxisLabel(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).YAxisLabel)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn CollectSample(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).CollectSample)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn UpdateGraph(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).UpdateGraph)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn BrowseCounters(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).BrowseCounters)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn DisplayProperties(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).DisplayProperties)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Counter(&self, iindex: i32) -> windows_core::Result<ICounterItem> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Counter)(windows_core::Interface::as_raw(self), iindex, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn AddCounter(&self, bspath: &windows_core::BSTR) -> windows_core::Result<ICounterItem> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AddCounter)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bspath), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn DeleteCounter<P0>(&self, pctr: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ICounterItem>,
    {
        unsafe { (windows_core::Interface::vtable(self).DeleteCounter)(windows_core::Interface::as_raw(self), pctr.param().abi()).ok() }
    }
    pub unsafe fn BackColorCtl(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).BackColorCtl)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetBackColorCtl(&self, color: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetBackColorCtl)(windows_core::Interface::as_raw(self), color).ok() }
    }
    pub unsafe fn SetLogFileName(&self, bsfilename: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetLogFileName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bsfilename)).ok() }
    }
    pub unsafe fn LogFileName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).LogFileName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetLogViewStart(&self, starttime: f64) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetLogViewStart)(windows_core::Interface::as_raw(self), starttime).ok() }
    }
    pub unsafe fn LogViewStart(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).LogViewStart)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetLogViewStop(&self, stoptime: f64) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetLogViewStop)(windows_core::Interface::as_raw(self), stoptime).ok() }
    }
    pub unsafe fn LogViewStop(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).LogViewStop)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GridColor(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GridColor)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetGridColor(&self, color: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetGridColor)(windows_core::Interface::as_raw(self), color).ok() }
    }
    pub unsafe fn TimeBarColor(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).TimeBarColor)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetTimeBarColor(&self, color: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetTimeBarColor)(windows_core::Interface::as_raw(self), color).ok() }
    }
    pub unsafe fn Highlight(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Highlight)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetHighlight(&self, bstate: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetHighlight)(windows_core::Interface::as_raw(self), bstate).ok() }
    }
    pub unsafe fn ShowToolbar(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ShowToolbar)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetShowToolbar(&self, bstate: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetShowToolbar)(windows_core::Interface::as_raw(self), bstate).ok() }
    }
    pub unsafe fn Paste(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Paste)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Copy(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Copy)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn SetReadOnly(&self, bstate: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetReadOnly)(windows_core::Interface::as_raw(self), bstate).ok() }
    }
    pub unsafe fn ReadOnly(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ReadOnly)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetReportValueType(&self, ereportvaluetype: ReportValueTypeConstants) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetReportValueType)(windows_core::Interface::as_raw(self), ereportvaluetype).ok() }
    }
    pub unsafe fn ReportValueType(&self) -> windows_core::Result<ReportValueTypeConstants> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ReportValueType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetMonitorDuplicateInstances(&self, bstate: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetMonitorDuplicateInstances)(windows_core::Interface::as_raw(self), bstate).ok() }
    }
    pub unsafe fn MonitorDuplicateInstances(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MonitorDuplicateInstances)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetDisplayFilter(&self, ivalue: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetDisplayFilter)(windows_core::Interface::as_raw(self), ivalue).ok() }
    }
    pub unsafe fn DisplayFilter(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DisplayFilter)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn LogFiles(&self) -> windows_core::Result<ILogFiles> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).LogFiles)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetDataSourceType(&self, edatasourcetype: DataSourceTypeConstants) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetDataSourceType)(windows_core::Interface::as_raw(self), edatasourcetype).ok() }
    }
    pub unsafe fn DataSourceType(&self) -> windows_core::Result<DataSourceTypeConstants> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DataSourceType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetSqlDsnName(&self, bssqldsnname: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetSqlDsnName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bssqldsnname)).ok() }
    }
    pub unsafe fn SqlDsnName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SqlDsnName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetSqlLogSetName(&self, bssqllogsetname: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetSqlLogSetName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bssqllogsetname)).ok() }
    }
    pub unsafe fn SqlLogSetName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SqlLogSetName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISystemMonitor_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Appearance: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetAppearance: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub BackColor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetBackColor: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub BorderStyle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetBorderStyle: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub ForeColor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetForeColor: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Font: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Font: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub putref_Font: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    putref_Font: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Counters: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Counters: usize,
    pub SetShowVerticalGrid: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub ShowVerticalGrid: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetShowHorizontalGrid: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub ShowHorizontalGrid: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetShowLegend: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub ShowLegend: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetShowScaleLabels: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub ShowScaleLabels: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetShowValueBar: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub ShowValueBar: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetMaximumScale: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub MaximumScale: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetMinimumScale: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub MinimumScale: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetUpdateInterval: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub UpdateInterval: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub SetDisplayType: unsafe extern "system" fn(*mut core::ffi::c_void, DisplayTypeConstants) -> windows_core::HRESULT,
    pub DisplayType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DisplayTypeConstants) -> windows_core::HRESULT,
    pub SetManualUpdate: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub ManualUpdate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetGraphTitle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GraphTitle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetYAxisLabel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub YAxisLabel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CollectSample: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub UpdateGraph: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub BrowseCounters: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DisplayProperties: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Counter: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddCounter: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DeleteCounter: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub BackColorCtl: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetBackColorCtl: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub SetLogFileName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub LogFileName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetLogViewStart: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub LogViewStart: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub SetLogViewStop: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub LogViewStop: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub GridColor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetGridColor: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub TimeBarColor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetTimeBarColor: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Highlight: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetHighlight: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub ShowToolbar: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetShowToolbar: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub Paste: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Copy: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetReadOnly: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub ReadOnly: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetReportValueType: unsafe extern "system" fn(*mut core::ffi::c_void, ReportValueTypeConstants) -> windows_core::HRESULT,
    pub ReportValueType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut ReportValueTypeConstants) -> windows_core::HRESULT,
    pub SetMonitorDuplicateInstances: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub MonitorDuplicateInstances: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetDisplayFilter: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub DisplayFilter: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub LogFiles: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    LogFiles: usize,
    pub SetDataSourceType: unsafe extern "system" fn(*mut core::ffi::c_void, DataSourceTypeConstants) -> windows_core::HRESULT,
    pub DataSourceType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DataSourceTypeConstants) -> windows_core::HRESULT,
    pub SetSqlDsnName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SqlDsnName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetSqlLogSetName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SqlLogSetName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISystemMonitor_Impl: windows_core::IUnknownImpl {
    fn Appearance(&self) -> windows_core::Result<i32>;
    fn SetAppearance(&self, iappearance: i32) -> windows_core::Result<()>;
    fn BackColor(&self) -> windows_core::Result<u32>;
    fn SetBackColor(&self, color: u32) -> windows_core::Result<()>;
    fn BorderStyle(&self) -> windows_core::Result<i32>;
    fn SetBorderStyle(&self, iborderstyle: i32) -> windows_core::Result<()>;
    fn ForeColor(&self) -> windows_core::Result<u32>;
    fn SetForeColor(&self, color: u32) -> windows_core::Result<()>;
    fn Font(&self) -> windows_core::Result<super::Ole::IFontDisp>;
    fn putref_Font(&self, pfont: windows_core::Ref<super::Ole::IFontDisp>) -> windows_core::Result<()>;
    fn Counters(&self) -> windows_core::Result<ICounters>;
    fn SetShowVerticalGrid(&self, bstate: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn ShowVerticalGrid(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetShowHorizontalGrid(&self, bstate: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn ShowHorizontalGrid(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetShowLegend(&self, bstate: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn ShowLegend(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetShowScaleLabels(&self, bstate: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn ShowScaleLabels(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetShowValueBar(&self, bstate: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn ShowValueBar(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetMaximumScale(&self, ivalue: i32) -> windows_core::Result<()>;
    fn MaximumScale(&self) -> windows_core::Result<i32>;
    fn SetMinimumScale(&self, ivalue: i32) -> windows_core::Result<()>;
    fn MinimumScale(&self) -> windows_core::Result<i32>;
    fn SetUpdateInterval(&self, fvalue: f32) -> windows_core::Result<()>;
    fn UpdateInterval(&self) -> windows_core::Result<f32>;
    fn SetDisplayType(&self, edisplaytype: DisplayTypeConstants) -> windows_core::Result<()>;
    fn DisplayType(&self) -> windows_core::Result<DisplayTypeConstants>;
    fn SetManualUpdate(&self, bstate: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn ManualUpdate(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetGraphTitle(&self, bstitle: &windows_core::BSTR) -> windows_core::Result<()>;
    fn GraphTitle(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetYAxisLabel(&self, bstitle: &windows_core::BSTR) -> windows_core::Result<()>;
    fn YAxisLabel(&self) -> windows_core::Result<windows_core::BSTR>;
    fn CollectSample(&self) -> windows_core::Result<()>;
    fn UpdateGraph(&self) -> windows_core::Result<()>;
    fn BrowseCounters(&self) -> windows_core::Result<()>;
    fn DisplayProperties(&self) -> windows_core::Result<()>;
    fn Counter(&self, iindex: i32) -> windows_core::Result<ICounterItem>;
    fn AddCounter(&self, bspath: &windows_core::BSTR) -> windows_core::Result<ICounterItem>;
    fn DeleteCounter(&self, pctr: windows_core::Ref<ICounterItem>) -> windows_core::Result<()>;
    fn BackColorCtl(&self) -> windows_core::Result<u32>;
    fn SetBackColorCtl(&self, color: u32) -> windows_core::Result<()>;
    fn SetLogFileName(&self, bsfilename: &windows_core::BSTR) -> windows_core::Result<()>;
    fn LogFileName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetLogViewStart(&self, starttime: f64) -> windows_core::Result<()>;
    fn LogViewStart(&self) -> windows_core::Result<f64>;
    fn SetLogViewStop(&self, stoptime: f64) -> windows_core::Result<()>;
    fn LogViewStop(&self) -> windows_core::Result<f64>;
    fn GridColor(&self) -> windows_core::Result<u32>;
    fn SetGridColor(&self, color: u32) -> windows_core::Result<()>;
    fn TimeBarColor(&self) -> windows_core::Result<u32>;
    fn SetTimeBarColor(&self, color: u32) -> windows_core::Result<()>;
    fn Highlight(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetHighlight(&self, bstate: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn ShowToolbar(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetShowToolbar(&self, bstate: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn Paste(&self) -> windows_core::Result<()>;
    fn Copy(&self) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn SetReadOnly(&self, bstate: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn ReadOnly(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetReportValueType(&self, ereportvaluetype: ReportValueTypeConstants) -> windows_core::Result<()>;
    fn ReportValueType(&self) -> windows_core::Result<ReportValueTypeConstants>;
    fn SetMonitorDuplicateInstances(&self, bstate: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn MonitorDuplicateInstances(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetDisplayFilter(&self, ivalue: i32) -> windows_core::Result<()>;
    fn DisplayFilter(&self) -> windows_core::Result<i32>;
    fn LogFiles(&self) -> windows_core::Result<ILogFiles>;
    fn SetDataSourceType(&self, edatasourcetype: DataSourceTypeConstants) -> windows_core::Result<()>;
    fn DataSourceType(&self) -> windows_core::Result<DataSourceTypeConstants>;
    fn SetSqlDsnName(&self, bssqldsnname: &windows_core::BSTR) -> windows_core::Result<()>;
    fn SqlDsnName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetSqlLogSetName(&self, bssqllogsetname: &windows_core::BSTR) -> windows_core::Result<()>;
    fn SqlLogSetName(&self) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISystemMonitor_Vtbl {
    pub const fn new<Identity: ISystemMonitor_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Appearance<Identity: ISystemMonitor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iappearance: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISystemMonitor_Impl::Appearance(this) {
                    Ok(ok__) => {
                        iappearance.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetAppearance<Identity: ISystemMonitor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iappearance: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISystemMonitor_Impl::SetAppearance(this, core::mem::transmute_copy(&iappearance)).into()
            }
        }
        unsafe extern "system" fn BackColor<Identity: ISystemMonitor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcolor: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISystemMonitor_Impl::BackColor(this) {
                    Ok(ok__) => {
                        pcolor.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetBackColor<Identity: ISystemMonitor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, color: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISystemMonitor_Impl::SetBackColor(this, core::mem::transmute_copy(&color)).into()
            }
        }
        unsafe extern "system" fn BorderStyle<Identity: ISystemMonitor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iborderstyle: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISystemMonitor_Impl::BorderStyle(this) {
                    Ok(ok__) => {
                        iborderstyle.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetBorderStyle<Identity: ISystemMonitor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iborderstyle: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISystemMonitor_Impl::SetBorderStyle(this, core::mem::transmute_copy(&iborderstyle)).into()
            }
        }
        unsafe extern "system" fn ForeColor<Identity: ISystemMonitor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcolor: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISystemMonitor_Impl::ForeColor(this) {
                    Ok(ok__) => {
                        pcolor.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetForeColor<Identity: ISystemMonitor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, color: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISystemMonitor_Impl::SetForeColor(this, core::mem::transmute_copy(&color)).into()
            }
        }
        unsafe extern "system" fn Font<Identity: ISystemMonitor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppfont: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISystemMonitor_Impl::Font(this) {
                    Ok(ok__) => {
                        ppfont.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn putref_Font<Identity: ISystemMonitor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfont: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISystemMonitor_Impl::putref_Font(this, core::mem::transmute_copy(&pfont)).into()
            }
        }
        unsafe extern "system" fn Counters<Identity: ISystemMonitor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppicounters: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISystemMonitor_Impl::Counters(this) {
                    Ok(ok__) => {
                        ppicounters.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetShowVerticalGrid<Identity: ISystemMonitor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstate: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISystemMonitor_Impl::SetShowVerticalGrid(this, core::mem::transmute_copy(&bstate)).into()
            }
        }
        unsafe extern "system" fn ShowVerticalGrid<Identity: ISystemMonitor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstate: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISystemMonitor_Impl::ShowVerticalGrid(this) {
                    Ok(ok__) => {
                        pbstate.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetShowHorizontalGrid<Identity: ISystemMonitor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstate: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISystemMonitor_Impl::SetShowHorizontalGrid(this, core::mem::transmute_copy(&bstate)).into()
            }
        }
        unsafe extern "system" fn ShowHorizontalGrid<Identity: ISystemMonitor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstate: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISystemMonitor_Impl::ShowHorizontalGrid(this) {
                    Ok(ok__) => {
                        pbstate.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetShowLegend<Identity: ISystemMonitor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstate: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISystemMonitor_Impl::SetShowLegend(this, core::mem::transmute_copy(&bstate)).into()
            }
        }
        unsafe extern "system" fn ShowLegend<Identity: ISystemMonitor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstate: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISystemMonitor_Impl::ShowLegend(this) {
                    Ok(ok__) => {
                        pbstate.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetShowScaleLabels<Identity: ISystemMonitor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstate: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISystemMonitor_Impl::SetShowScaleLabels(this, core::mem::transmute_copy(&bstate)).into()
            }
        }
        unsafe extern "system" fn ShowScaleLabels<Identity: ISystemMonitor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstate: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISystemMonitor_Impl::ShowScaleLabels(this) {
                    Ok(ok__) => {
                        pbstate.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetShowValueBar<Identity: ISystemMonitor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstate: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISystemMonitor_Impl::SetShowValueBar(this, core::mem::transmute_copy(&bstate)).into()
            }
        }
        unsafe extern "system" fn ShowValueBar<Identity: ISystemMonitor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstate: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISystemMonitor_Impl::ShowValueBar(this) {
                    Ok(ok__) => {
                        pbstate.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetMaximumScale<Identity: ISystemMonitor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ivalue: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISystemMonitor_Impl::SetMaximumScale(this, core::mem::transmute_copy(&ivalue)).into()
            }
        }
        unsafe extern "system" fn MaximumScale<Identity: ISystemMonitor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pivalue: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISystemMonitor_Impl::MaximumScale(this) {
                    Ok(ok__) => {
                        pivalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetMinimumScale<Identity: ISystemMonitor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ivalue: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISystemMonitor_Impl::SetMinimumScale(this, core::mem::transmute_copy(&ivalue)).into()
            }
        }
        unsafe extern "system" fn MinimumScale<Identity: ISystemMonitor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pivalue: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISystemMonitor_Impl::MinimumScale(this) {
                    Ok(ok__) => {
                        pivalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetUpdateInterval<Identity: ISystemMonitor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fvalue: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISystemMonitor_Impl::SetUpdateInterval(this, core::mem::transmute_copy(&fvalue)).into()
            }
        }
        unsafe extern "system" fn UpdateInterval<Identity: ISystemMonitor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfvalue: *mut f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISystemMonitor_Impl::UpdateInterval(this) {
                    Ok(ok__) => {
                        pfvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetDisplayType<Identity: ISystemMonitor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, edisplaytype: DisplayTypeConstants) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISystemMonitor_Impl::SetDisplayType(this, core::mem::transmute_copy(&edisplaytype)).into()
            }
        }
        unsafe extern "system" fn DisplayType<Identity: ISystemMonitor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pedisplaytype: *mut DisplayTypeConstants) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISystemMonitor_Impl::DisplayType(this) {
                    Ok(ok__) => {
                        pedisplaytype.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetManualUpdate<Identity: ISystemMonitor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstate: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISystemMonitor_Impl::SetManualUpdate(this, core::mem::transmute_copy(&bstate)).into()
            }
        }
        unsafe extern "system" fn ManualUpdate<Identity: ISystemMonitor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstate: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISystemMonitor_Impl::ManualUpdate(this) {
                    Ok(ok__) => {
                        pbstate.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetGraphTitle<Identity: ISystemMonitor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstitle: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISystemMonitor_Impl::SetGraphTitle(this, core::mem::transmute(&bstitle)).into()
            }
        }
        unsafe extern "system" fn GraphTitle<Identity: ISystemMonitor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstitle: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISystemMonitor_Impl::GraphTitle(this) {
                    Ok(ok__) => {
                        pbstitle.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetYAxisLabel<Identity: ISystemMonitor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstitle: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISystemMonitor_Impl::SetYAxisLabel(this, core::mem::transmute(&bstitle)).into()
            }
        }
        unsafe extern "system" fn YAxisLabel<Identity: ISystemMonitor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstitle: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISystemMonitor_Impl::YAxisLabel(this) {
                    Ok(ok__) => {
                        pbstitle.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CollectSample<Identity: ISystemMonitor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISystemMonitor_Impl::CollectSample(this).into()
            }
        }
        unsafe extern "system" fn UpdateGraph<Identity: ISystemMonitor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISystemMonitor_Impl::UpdateGraph(this).into()
            }
        }
        unsafe extern "system" fn BrowseCounters<Identity: ISystemMonitor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISystemMonitor_Impl::BrowseCounters(this).into()
            }
        }
        unsafe extern "system" fn DisplayProperties<Identity: ISystemMonitor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISystemMonitor_Impl::DisplayProperties(this).into()
            }
        }
        unsafe extern "system" fn Counter<Identity: ISystemMonitor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iindex: i32, ppicounter: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISystemMonitor_Impl::Counter(this, core::mem::transmute_copy(&iindex)) {
                    Ok(ok__) => {
                        ppicounter.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn AddCounter<Identity: ISystemMonitor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bspath: *mut core::ffi::c_void, ppicounter: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISystemMonitor_Impl::AddCounter(this, core::mem::transmute(&bspath)) {
                    Ok(ok__) => {
                        ppicounter.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DeleteCounter<Identity: ISystemMonitor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pctr: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISystemMonitor_Impl::DeleteCounter(this, core::mem::transmute_copy(&pctr)).into()
            }
        }
        unsafe extern "system" fn BackColorCtl<Identity: ISystemMonitor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcolor: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISystemMonitor_Impl::BackColorCtl(this) {
                    Ok(ok__) => {
                        pcolor.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetBackColorCtl<Identity: ISystemMonitor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, color: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISystemMonitor_Impl::SetBackColorCtl(this, core::mem::transmute_copy(&color)).into()
            }
        }
        unsafe extern "system" fn SetLogFileName<Identity: ISystemMonitor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bsfilename: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISystemMonitor_Impl::SetLogFileName(this, core::mem::transmute(&bsfilename)).into()
            }
        }
        unsafe extern "system" fn LogFileName<Identity: ISystemMonitor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bsfilename: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISystemMonitor_Impl::LogFileName(this) {
                    Ok(ok__) => {
                        bsfilename.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetLogViewStart<Identity: ISystemMonitor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, starttime: f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISystemMonitor_Impl::SetLogViewStart(this, core::mem::transmute_copy(&starttime)).into()
            }
        }
        unsafe extern "system" fn LogViewStart<Identity: ISystemMonitor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, starttime: *mut f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISystemMonitor_Impl::LogViewStart(this) {
                    Ok(ok__) => {
                        starttime.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetLogViewStop<Identity: ISystemMonitor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, stoptime: f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISystemMonitor_Impl::SetLogViewStop(this, core::mem::transmute_copy(&stoptime)).into()
            }
        }
        unsafe extern "system" fn LogViewStop<Identity: ISystemMonitor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, stoptime: *mut f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISystemMonitor_Impl::LogViewStop(this) {
                    Ok(ok__) => {
                        stoptime.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GridColor<Identity: ISystemMonitor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcolor: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISystemMonitor_Impl::GridColor(this) {
                    Ok(ok__) => {
                        pcolor.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetGridColor<Identity: ISystemMonitor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, color: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISystemMonitor_Impl::SetGridColor(this, core::mem::transmute_copy(&color)).into()
            }
        }
        unsafe extern "system" fn TimeBarColor<Identity: ISystemMonitor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcolor: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISystemMonitor_Impl::TimeBarColor(this) {
                    Ok(ok__) => {
                        pcolor.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetTimeBarColor<Identity: ISystemMonitor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, color: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISystemMonitor_Impl::SetTimeBarColor(this, core::mem::transmute_copy(&color)).into()
            }
        }
        unsafe extern "system" fn Highlight<Identity: ISystemMonitor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstate: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISystemMonitor_Impl::Highlight(this) {
                    Ok(ok__) => {
                        pbstate.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetHighlight<Identity: ISystemMonitor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstate: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISystemMonitor_Impl::SetHighlight(this, core::mem::transmute_copy(&bstate)).into()
            }
        }
        unsafe extern "system" fn ShowToolbar<Identity: ISystemMonitor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstate: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISystemMonitor_Impl::ShowToolbar(this) {
                    Ok(ok__) => {
                        pbstate.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetShowToolbar<Identity: ISystemMonitor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstate: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISystemMonitor_Impl::SetShowToolbar(this, core::mem::transmute_copy(&bstate)).into()
            }
        }
        unsafe extern "system" fn Paste<Identity: ISystemMonitor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISystemMonitor_Impl::Paste(this).into()
            }
        }
        unsafe extern "system" fn Copy<Identity: ISystemMonitor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISystemMonitor_Impl::Copy(this).into()
            }
        }
        unsafe extern "system" fn Reset<Identity: ISystemMonitor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISystemMonitor_Impl::Reset(this).into()
            }
        }
        unsafe extern "system" fn SetReadOnly<Identity: ISystemMonitor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstate: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISystemMonitor_Impl::SetReadOnly(this, core::mem::transmute_copy(&bstate)).into()
            }
        }
        unsafe extern "system" fn ReadOnly<Identity: ISystemMonitor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstate: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISystemMonitor_Impl::ReadOnly(this) {
                    Ok(ok__) => {
                        pbstate.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetReportValueType<Identity: ISystemMonitor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ereportvaluetype: ReportValueTypeConstants) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISystemMonitor_Impl::SetReportValueType(this, core::mem::transmute_copy(&ereportvaluetype)).into()
            }
        }
        unsafe extern "system" fn ReportValueType<Identity: ISystemMonitor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pereportvaluetype: *mut ReportValueTypeConstants) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISystemMonitor_Impl::ReportValueType(this) {
                    Ok(ok__) => {
                        pereportvaluetype.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetMonitorDuplicateInstances<Identity: ISystemMonitor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstate: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISystemMonitor_Impl::SetMonitorDuplicateInstances(this, core::mem::transmute_copy(&bstate)).into()
            }
        }
        unsafe extern "system" fn MonitorDuplicateInstances<Identity: ISystemMonitor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstate: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISystemMonitor_Impl::MonitorDuplicateInstances(this) {
                    Ok(ok__) => {
                        pbstate.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetDisplayFilter<Identity: ISystemMonitor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ivalue: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISystemMonitor_Impl::SetDisplayFilter(this, core::mem::transmute_copy(&ivalue)).into()
            }
        }
        unsafe extern "system" fn DisplayFilter<Identity: ISystemMonitor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pivalue: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISystemMonitor_Impl::DisplayFilter(this) {
                    Ok(ok__) => {
                        pivalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn LogFiles<Identity: ISystemMonitor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppilogfiles: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISystemMonitor_Impl::LogFiles(this) {
                    Ok(ok__) => {
                        ppilogfiles.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetDataSourceType<Identity: ISystemMonitor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, edatasourcetype: DataSourceTypeConstants) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISystemMonitor_Impl::SetDataSourceType(this, core::mem::transmute_copy(&edatasourcetype)).into()
            }
        }
        unsafe extern "system" fn DataSourceType<Identity: ISystemMonitor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pedatasourcetype: *mut DataSourceTypeConstants) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISystemMonitor_Impl::DataSourceType(this) {
                    Ok(ok__) => {
                        pedatasourcetype.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetSqlDsnName<Identity: ISystemMonitor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bssqldsnname: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISystemMonitor_Impl::SetSqlDsnName(this, core::mem::transmute(&bssqldsnname)).into()
            }
        }
        unsafe extern "system" fn SqlDsnName<Identity: ISystemMonitor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bssqldsnname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISystemMonitor_Impl::SqlDsnName(this) {
                    Ok(ok__) => {
                        bssqldsnname.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetSqlLogSetName<Identity: ISystemMonitor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bssqllogsetname: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISystemMonitor_Impl::SetSqlLogSetName(this, core::mem::transmute(&bssqllogsetname)).into()
            }
        }
        unsafe extern "system" fn SqlLogSetName<Identity: ISystemMonitor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bssqllogsetname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISystemMonitor_Impl::SqlLogSetName(this) {
                    Ok(ok__) => {
                        bssqllogsetname.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Appearance: Appearance::<Identity, OFFSET>,
            SetAppearance: SetAppearance::<Identity, OFFSET>,
            BackColor: BackColor::<Identity, OFFSET>,
            SetBackColor: SetBackColor::<Identity, OFFSET>,
            BorderStyle: BorderStyle::<Identity, OFFSET>,
            SetBorderStyle: SetBorderStyle::<Identity, OFFSET>,
            ForeColor: ForeColor::<Identity, OFFSET>,
            SetForeColor: SetForeColor::<Identity, OFFSET>,
            Font: Font::<Identity, OFFSET>,
            putref_Font: putref_Font::<Identity, OFFSET>,
            Counters: Counters::<Identity, OFFSET>,
            SetShowVerticalGrid: SetShowVerticalGrid::<Identity, OFFSET>,
            ShowVerticalGrid: ShowVerticalGrid::<Identity, OFFSET>,
            SetShowHorizontalGrid: SetShowHorizontalGrid::<Identity, OFFSET>,
            ShowHorizontalGrid: ShowHorizontalGrid::<Identity, OFFSET>,
            SetShowLegend: SetShowLegend::<Identity, OFFSET>,
            ShowLegend: ShowLegend::<Identity, OFFSET>,
            SetShowScaleLabels: SetShowScaleLabels::<Identity, OFFSET>,
            ShowScaleLabels: ShowScaleLabels::<Identity, OFFSET>,
            SetShowValueBar: SetShowValueBar::<Identity, OFFSET>,
            ShowValueBar: ShowValueBar::<Identity, OFFSET>,
            SetMaximumScale: SetMaximumScale::<Identity, OFFSET>,
            MaximumScale: MaximumScale::<Identity, OFFSET>,
            SetMinimumScale: SetMinimumScale::<Identity, OFFSET>,
            MinimumScale: MinimumScale::<Identity, OFFSET>,
            SetUpdateInterval: SetUpdateInterval::<Identity, OFFSET>,
            UpdateInterval: UpdateInterval::<Identity, OFFSET>,
            SetDisplayType: SetDisplayType::<Identity, OFFSET>,
            DisplayType: DisplayType::<Identity, OFFSET>,
            SetManualUpdate: SetManualUpdate::<Identity, OFFSET>,
            ManualUpdate: ManualUpdate::<Identity, OFFSET>,
            SetGraphTitle: SetGraphTitle::<Identity, OFFSET>,
            GraphTitle: GraphTitle::<Identity, OFFSET>,
            SetYAxisLabel: SetYAxisLabel::<Identity, OFFSET>,
            YAxisLabel: YAxisLabel::<Identity, OFFSET>,
            CollectSample: CollectSample::<Identity, OFFSET>,
            UpdateGraph: UpdateGraph::<Identity, OFFSET>,
            BrowseCounters: BrowseCounters::<Identity, OFFSET>,
            DisplayProperties: DisplayProperties::<Identity, OFFSET>,
            Counter: Counter::<Identity, OFFSET>,
            AddCounter: AddCounter::<Identity, OFFSET>,
            DeleteCounter: DeleteCounter::<Identity, OFFSET>,
            BackColorCtl: BackColorCtl::<Identity, OFFSET>,
            SetBackColorCtl: SetBackColorCtl::<Identity, OFFSET>,
            SetLogFileName: SetLogFileName::<Identity, OFFSET>,
            LogFileName: LogFileName::<Identity, OFFSET>,
            SetLogViewStart: SetLogViewStart::<Identity, OFFSET>,
            LogViewStart: LogViewStart::<Identity, OFFSET>,
            SetLogViewStop: SetLogViewStop::<Identity, OFFSET>,
            LogViewStop: LogViewStop::<Identity, OFFSET>,
            GridColor: GridColor::<Identity, OFFSET>,
            SetGridColor: SetGridColor::<Identity, OFFSET>,
            TimeBarColor: TimeBarColor::<Identity, OFFSET>,
            SetTimeBarColor: SetTimeBarColor::<Identity, OFFSET>,
            Highlight: Highlight::<Identity, OFFSET>,
            SetHighlight: SetHighlight::<Identity, OFFSET>,
            ShowToolbar: ShowToolbar::<Identity, OFFSET>,
            SetShowToolbar: SetShowToolbar::<Identity, OFFSET>,
            Paste: Paste::<Identity, OFFSET>,
            Copy: Copy::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            SetReadOnly: SetReadOnly::<Identity, OFFSET>,
            ReadOnly: ReadOnly::<Identity, OFFSET>,
            SetReportValueType: SetReportValueType::<Identity, OFFSET>,
            ReportValueType: ReportValueType::<Identity, OFFSET>,
            SetMonitorDuplicateInstances: SetMonitorDuplicateInstances::<Identity, OFFSET>,
            MonitorDuplicateInstances: MonitorDuplicateInstances::<Identity, OFFSET>,
            SetDisplayFilter: SetDisplayFilter::<Identity, OFFSET>,
            DisplayFilter: DisplayFilter::<Identity, OFFSET>,
            LogFiles: LogFiles::<Identity, OFFSET>,
            SetDataSourceType: SetDataSourceType::<Identity, OFFSET>,
            DataSourceType: DataSourceType::<Identity, OFFSET>,
            SetSqlDsnName: SetSqlDsnName::<Identity, OFFSET>,
            SqlDsnName: SqlDsnName::<Identity, OFFSET>,
            SetSqlLogSetName: SetSqlLogSetName::<Identity, OFFSET>,
            SqlLogSetName: SqlLogSetName::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISystemMonitor as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl windows_core::RuntimeName for ISystemMonitor {}
windows_core::imp::define_interface!(ISystemMonitor2, ISystemMonitor2_Vtbl, 0x08e3206a_5fd2_4fde_a8a5_8cb3b63d2677);
impl core::ops::Deref for ISystemMonitor2 {
    type Target = ISystemMonitor;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISystemMonitor2, windows_core::IUnknown, ISystemMonitor);
impl ISystemMonitor2 {
    pub unsafe fn SetEnableDigitGrouping(&self, bstate: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetEnableDigitGrouping)(windows_core::Interface::as_raw(self), bstate).ok() }
    }
    pub unsafe fn EnableDigitGrouping(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnableDigitGrouping)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetEnableToolTips(&self, bstate: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetEnableToolTips)(windows_core::Interface::as_raw(self), bstate).ok() }
    }
    pub unsafe fn EnableToolTips(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnableToolTips)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetShowTimeAxisLabels(&self, bstate: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetShowTimeAxisLabels)(windows_core::Interface::as_raw(self), bstate).ok() }
    }
    pub unsafe fn ShowTimeAxisLabels(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ShowTimeAxisLabels)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetChartScroll(&self, bscroll: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetChartScroll)(windows_core::Interface::as_raw(self), bscroll).ok() }
    }
    pub unsafe fn ChartScroll(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ChartScroll)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetDataPointCount(&self, inewcount: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetDataPointCount)(windows_core::Interface::as_raw(self), inewcount).ok() }
    }
    pub unsafe fn DataPointCount(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DataPointCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn ScaleToFit(&self, bselectedcountersonly: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ScaleToFit)(windows_core::Interface::as_raw(self), bselectedcountersonly).ok() }
    }
    pub unsafe fn SaveAs(&self, bstrfilename: &windows_core::BSTR, esysmonfiletype: SysmonFileType) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SaveAs)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrfilename), esysmonfiletype).ok() }
    }
    pub unsafe fn Relog(&self, bstrfilename: &windows_core::BSTR, esysmonfiletype: SysmonFileType, ifilter: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Relog)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrfilename), esysmonfiletype, ifilter).ok() }
    }
    pub unsafe fn ClearData(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ClearData)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn LogSourceStartTime(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).LogSourceStartTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn LogSourceStopTime(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).LogSourceStopTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetLogViewRange(&self, starttime: f64, stoptime: f64) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetLogViewRange)(windows_core::Interface::as_raw(self), starttime, stoptime).ok() }
    }
    pub unsafe fn GetLogViewRange(&self, starttime: *mut f64, stoptime: *mut f64) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetLogViewRange)(windows_core::Interface::as_raw(self), starttime as _, stoptime as _).ok() }
    }
    pub unsafe fn BatchingLock(&self, flock: super::super::Foundation::VARIANT_BOOL, ebatchreason: SysmonBatchReason) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).BatchingLock)(windows_core::Interface::as_raw(self), flock, ebatchreason).ok() }
    }
    pub unsafe fn LoadSettings(&self, bstrsettingfilename: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).LoadSettings)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrsettingfilename)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISystemMonitor2_Vtbl {
    pub base__: ISystemMonitor_Vtbl,
    pub SetEnableDigitGrouping: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub EnableDigitGrouping: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetEnableToolTips: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub EnableToolTips: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetShowTimeAxisLabels: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub ShowTimeAxisLabels: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetChartScroll: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub ChartScroll: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetDataPointCount: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub DataPointCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub ScaleToFit: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SaveAs: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, SysmonFileType) -> windows_core::HRESULT,
    pub Relog: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, SysmonFileType, i32) -> windows_core::HRESULT,
    pub ClearData: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub LogSourceStartTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub LogSourceStopTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub SetLogViewRange: unsafe extern "system" fn(*mut core::ffi::c_void, f64, f64) -> windows_core::HRESULT,
    pub GetLogViewRange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64, *mut f64) -> windows_core::HRESULT,
    pub BatchingLock: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL, SysmonBatchReason) -> windows_core::HRESULT,
    pub LoadSettings: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISystemMonitor2_Impl: ISystemMonitor_Impl {
    fn SetEnableDigitGrouping(&self, bstate: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn EnableDigitGrouping(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetEnableToolTips(&self, bstate: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn EnableToolTips(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetShowTimeAxisLabels(&self, bstate: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn ShowTimeAxisLabels(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetChartScroll(&self, bscroll: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn ChartScroll(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetDataPointCount(&self, inewcount: i32) -> windows_core::Result<()>;
    fn DataPointCount(&self) -> windows_core::Result<i32>;
    fn ScaleToFit(&self, bselectedcountersonly: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn SaveAs(&self, bstrfilename: &windows_core::BSTR, esysmonfiletype: SysmonFileType) -> windows_core::Result<()>;
    fn Relog(&self, bstrfilename: &windows_core::BSTR, esysmonfiletype: SysmonFileType, ifilter: i32) -> windows_core::Result<()>;
    fn ClearData(&self) -> windows_core::Result<()>;
    fn LogSourceStartTime(&self) -> windows_core::Result<f64>;
    fn LogSourceStopTime(&self) -> windows_core::Result<f64>;
    fn SetLogViewRange(&self, starttime: f64, stoptime: f64) -> windows_core::Result<()>;
    fn GetLogViewRange(&self, starttime: *mut f64, stoptime: *mut f64) -> windows_core::Result<()>;
    fn BatchingLock(&self, flock: super::super::Foundation::VARIANT_BOOL, ebatchreason: SysmonBatchReason) -> windows_core::Result<()>;
    fn LoadSettings(&self, bstrsettingfilename: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISystemMonitor2_Vtbl {
    pub const fn new<Identity: ISystemMonitor2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetEnableDigitGrouping<Identity: ISystemMonitor2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstate: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISystemMonitor2_Impl::SetEnableDigitGrouping(this, core::mem::transmute_copy(&bstate)).into()
            }
        }
        unsafe extern "system" fn EnableDigitGrouping<Identity: ISystemMonitor2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstate: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISystemMonitor2_Impl::EnableDigitGrouping(this) {
                    Ok(ok__) => {
                        pbstate.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetEnableToolTips<Identity: ISystemMonitor2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstate: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISystemMonitor2_Impl::SetEnableToolTips(this, core::mem::transmute_copy(&bstate)).into()
            }
        }
        unsafe extern "system" fn EnableToolTips<Identity: ISystemMonitor2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstate: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISystemMonitor2_Impl::EnableToolTips(this) {
                    Ok(ok__) => {
                        pbstate.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetShowTimeAxisLabels<Identity: ISystemMonitor2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstate: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISystemMonitor2_Impl::SetShowTimeAxisLabels(this, core::mem::transmute_copy(&bstate)).into()
            }
        }
        unsafe extern "system" fn ShowTimeAxisLabels<Identity: ISystemMonitor2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstate: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISystemMonitor2_Impl::ShowTimeAxisLabels(this) {
                    Ok(ok__) => {
                        pbstate.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetChartScroll<Identity: ISystemMonitor2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bscroll: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISystemMonitor2_Impl::SetChartScroll(this, core::mem::transmute_copy(&bscroll)).into()
            }
        }
        unsafe extern "system" fn ChartScroll<Identity: ISystemMonitor2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbscroll: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISystemMonitor2_Impl::ChartScroll(this) {
                    Ok(ok__) => {
                        pbscroll.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetDataPointCount<Identity: ISystemMonitor2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, inewcount: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISystemMonitor2_Impl::SetDataPointCount(this, core::mem::transmute_copy(&inewcount)).into()
            }
        }
        unsafe extern "system" fn DataPointCount<Identity: ISystemMonitor2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pidatapointcount: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISystemMonitor2_Impl::DataPointCount(this) {
                    Ok(ok__) => {
                        pidatapointcount.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ScaleToFit<Identity: ISystemMonitor2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bselectedcountersonly: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISystemMonitor2_Impl::ScaleToFit(this, core::mem::transmute_copy(&bselectedcountersonly)).into()
            }
        }
        unsafe extern "system" fn SaveAs<Identity: ISystemMonitor2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrfilename: *mut core::ffi::c_void, esysmonfiletype: SysmonFileType) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISystemMonitor2_Impl::SaveAs(this, core::mem::transmute(&bstrfilename), core::mem::transmute_copy(&esysmonfiletype)).into()
            }
        }
        unsafe extern "system" fn Relog<Identity: ISystemMonitor2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrfilename: *mut core::ffi::c_void, esysmonfiletype: SysmonFileType, ifilter: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISystemMonitor2_Impl::Relog(this, core::mem::transmute(&bstrfilename), core::mem::transmute_copy(&esysmonfiletype), core::mem::transmute_copy(&ifilter)).into()
            }
        }
        unsafe extern "system" fn ClearData<Identity: ISystemMonitor2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISystemMonitor2_Impl::ClearData(this).into()
            }
        }
        unsafe extern "system" fn LogSourceStartTime<Identity: ISystemMonitor2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdate: *mut f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISystemMonitor2_Impl::LogSourceStartTime(this) {
                    Ok(ok__) => {
                        pdate.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn LogSourceStopTime<Identity: ISystemMonitor2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdate: *mut f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISystemMonitor2_Impl::LogSourceStopTime(this) {
                    Ok(ok__) => {
                        pdate.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetLogViewRange<Identity: ISystemMonitor2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, starttime: f64, stoptime: f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISystemMonitor2_Impl::SetLogViewRange(this, core::mem::transmute_copy(&starttime), core::mem::transmute_copy(&stoptime)).into()
            }
        }
        unsafe extern "system" fn GetLogViewRange<Identity: ISystemMonitor2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, starttime: *mut f64, stoptime: *mut f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISystemMonitor2_Impl::GetLogViewRange(this, core::mem::transmute_copy(&starttime), core::mem::transmute_copy(&stoptime)).into()
            }
        }
        unsafe extern "system" fn BatchingLock<Identity: ISystemMonitor2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, flock: super::super::Foundation::VARIANT_BOOL, ebatchreason: SysmonBatchReason) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISystemMonitor2_Impl::BatchingLock(this, core::mem::transmute_copy(&flock), core::mem::transmute_copy(&ebatchreason)).into()
            }
        }
        unsafe extern "system" fn LoadSettings<Identity: ISystemMonitor2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrsettingfilename: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISystemMonitor2_Impl::LoadSettings(this, core::mem::transmute(&bstrsettingfilename)).into()
            }
        }
        Self {
            base__: ISystemMonitor_Vtbl::new::<Identity, OFFSET>(),
            SetEnableDigitGrouping: SetEnableDigitGrouping::<Identity, OFFSET>,
            EnableDigitGrouping: EnableDigitGrouping::<Identity, OFFSET>,
            SetEnableToolTips: SetEnableToolTips::<Identity, OFFSET>,
            EnableToolTips: EnableToolTips::<Identity, OFFSET>,
            SetShowTimeAxisLabels: SetShowTimeAxisLabels::<Identity, OFFSET>,
            ShowTimeAxisLabels: ShowTimeAxisLabels::<Identity, OFFSET>,
            SetChartScroll: SetChartScroll::<Identity, OFFSET>,
            ChartScroll: ChartScroll::<Identity, OFFSET>,
            SetDataPointCount: SetDataPointCount::<Identity, OFFSET>,
            DataPointCount: DataPointCount::<Identity, OFFSET>,
            ScaleToFit: ScaleToFit::<Identity, OFFSET>,
            SaveAs: SaveAs::<Identity, OFFSET>,
            Relog: Relog::<Identity, OFFSET>,
            ClearData: ClearData::<Identity, OFFSET>,
            LogSourceStartTime: LogSourceStartTime::<Identity, OFFSET>,
            LogSourceStopTime: LogSourceStopTime::<Identity, OFFSET>,
            SetLogViewRange: SetLogViewRange::<Identity, OFFSET>,
            GetLogViewRange: GetLogViewRange::<Identity, OFFSET>,
            BatchingLock: BatchingLock::<Identity, OFFSET>,
            LoadSettings: LoadSettings::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISystemMonitor2 as windows_core::Interface>::IID || iid == &<ISystemMonitor as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl windows_core::RuntimeName for ISystemMonitor2 {}
windows_core::imp::define_interface!(ISystemMonitorEvents, ISystemMonitorEvents_Vtbl, 0xee660ea0_4abd_11cf_943a_008029004347);
windows_core::imp::interface_hierarchy!(ISystemMonitorEvents, windows_core::IUnknown);
impl ISystemMonitorEvents {
    pub unsafe fn OnCounterSelected(&self, index: i32) {
        unsafe { (windows_core::Interface::vtable(self).OnCounterSelected)(windows_core::Interface::as_raw(self), index) }
    }
    pub unsafe fn OnCounterAdded(&self, index: i32) {
        unsafe { (windows_core::Interface::vtable(self).OnCounterAdded)(windows_core::Interface::as_raw(self), index) }
    }
    pub unsafe fn OnCounterDeleted(&self, index: i32) {
        unsafe { (windows_core::Interface::vtable(self).OnCounterDeleted)(windows_core::Interface::as_raw(self), index) }
    }
    pub unsafe fn OnSampleCollected(&self) {
        unsafe { (windows_core::Interface::vtable(self).OnSampleCollected)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn OnDblClick(&self, index: i32) {
        unsafe { (windows_core::Interface::vtable(self).OnDblClick)(windows_core::Interface::as_raw(self), index) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISystemMonitorEvents_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnCounterSelected: unsafe extern "system" fn(*mut core::ffi::c_void, i32),
    pub OnCounterAdded: unsafe extern "system" fn(*mut core::ffi::c_void, i32),
    pub OnCounterDeleted: unsafe extern "system" fn(*mut core::ffi::c_void, i32),
    pub OnSampleCollected: unsafe extern "system" fn(*mut core::ffi::c_void),
    pub OnDblClick: unsafe extern "system" fn(*mut core::ffi::c_void, i32),
}
pub trait ISystemMonitorEvents_Impl: windows_core::IUnknownImpl {
    fn OnCounterSelected(&self, index: i32);
    fn OnCounterAdded(&self, index: i32);
    fn OnCounterDeleted(&self, index: i32);
    fn OnSampleCollected(&self);
    fn OnDblClick(&self, index: i32);
}
impl ISystemMonitorEvents_Vtbl {
    pub const fn new<Identity: ISystemMonitorEvents_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnCounterSelected<Identity: ISystemMonitorEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: i32) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISystemMonitorEvents_Impl::OnCounterSelected(this, core::mem::transmute_copy(&index))
            }
        }
        unsafe extern "system" fn OnCounterAdded<Identity: ISystemMonitorEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: i32) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISystemMonitorEvents_Impl::OnCounterAdded(this, core::mem::transmute_copy(&index))
            }
        }
        unsafe extern "system" fn OnCounterDeleted<Identity: ISystemMonitorEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: i32) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISystemMonitorEvents_Impl::OnCounterDeleted(this, core::mem::transmute_copy(&index))
            }
        }
        unsafe extern "system" fn OnSampleCollected<Identity: ISystemMonitorEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISystemMonitorEvents_Impl::OnSampleCollected(this)
            }
        }
        unsafe extern "system" fn OnDblClick<Identity: ISystemMonitorEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: i32) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISystemMonitorEvents_Impl::OnDblClick(this, core::mem::transmute_copy(&index))
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnCounterSelected: OnCounterSelected::<Identity, OFFSET>,
            OnCounterAdded: OnCounterAdded::<Identity, OFFSET>,
            OnCounterDeleted: OnCounterDeleted::<Identity, OFFSET>,
            OnSampleCollected: OnSampleCollected::<Identity, OFFSET>,
            OnDblClick: OnDblClick::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISystemMonitorEvents as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ISystemMonitorEvents {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ITraceDataCollector, ITraceDataCollector_Vtbl, 0x0383750b_098b_11d8_9414_505054503030);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ITraceDataCollector {
    type Target = IDataCollector;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ITraceDataCollector, windows_core::IUnknown, super::Com::IDispatch, IDataCollector);
#[cfg(feature = "Win32_System_Com")]
impl ITraceDataCollector {
    pub unsafe fn BufferSize(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).BufferSize)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetBufferSize(&self, size: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetBufferSize)(windows_core::Interface::as_raw(self), size).ok() }
    }
    pub unsafe fn BuffersLost(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).BuffersLost)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetBuffersLost(&self, buffers: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetBuffersLost)(windows_core::Interface::as_raw(self), buffers).ok() }
    }
    pub unsafe fn BuffersWritten(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).BuffersWritten)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetBuffersWritten(&self, buffers: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetBuffersWritten)(windows_core::Interface::as_raw(self), buffers).ok() }
    }
    pub unsafe fn ClockType(&self) -> windows_core::Result<ClockType> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ClockType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetClockType(&self, clock: ClockType) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetClockType)(windows_core::Interface::as_raw(self), clock).ok() }
    }
    pub unsafe fn EventsLost(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EventsLost)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetEventsLost(&self, events: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetEventsLost)(windows_core::Interface::as_raw(self), events).ok() }
    }
    pub unsafe fn ExtendedModes(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ExtendedModes)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetExtendedModes(&self, mode: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetExtendedModes)(windows_core::Interface::as_raw(self), mode).ok() }
    }
    pub unsafe fn FlushTimer(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FlushTimer)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetFlushTimer(&self, seconds: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetFlushTimer)(windows_core::Interface::as_raw(self), seconds).ok() }
    }
    pub unsafe fn FreeBuffers(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FreeBuffers)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetFreeBuffers(&self, buffers: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetFreeBuffers)(windows_core::Interface::as_raw(self), buffers).ok() }
    }
    pub unsafe fn Guid(&self) -> windows_core::Result<windows_core::GUID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Guid)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetGuid(&self, guid: windows_core::GUID) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetGuid)(windows_core::Interface::as_raw(self), core::mem::transmute(guid)).ok() }
    }
    pub unsafe fn IsKernelTrace(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsKernelTrace)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn MaximumBuffers(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MaximumBuffers)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetMaximumBuffers(&self, buffers: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetMaximumBuffers)(windows_core::Interface::as_raw(self), buffers).ok() }
    }
    pub unsafe fn MinimumBuffers(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MinimumBuffers)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetMinimumBuffers(&self, buffers: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetMinimumBuffers)(windows_core::Interface::as_raw(self), buffers).ok() }
    }
    pub unsafe fn NumberOfBuffers(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).NumberOfBuffers)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetNumberOfBuffers(&self, buffers: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetNumberOfBuffers)(windows_core::Interface::as_raw(self), buffers).ok() }
    }
    pub unsafe fn PreallocateFile(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PreallocateFile)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetPreallocateFile(&self, allocate: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetPreallocateFile)(windows_core::Interface::as_raw(self), allocate).ok() }
    }
    pub unsafe fn ProcessMode(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ProcessMode)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetProcessMode(&self, process: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetProcessMode)(windows_core::Interface::as_raw(self), process).ok() }
    }
    pub unsafe fn RealTimeBuffersLost(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RealTimeBuffersLost)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetRealTimeBuffersLost(&self, buffers: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetRealTimeBuffersLost)(windows_core::Interface::as_raw(self), buffers).ok() }
    }
    pub unsafe fn SessionId(&self) -> windows_core::Result<u64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SessionId)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetSessionId(&self, id: u64) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetSessionId)(windows_core::Interface::as_raw(self), id).ok() }
    }
    pub unsafe fn SessionName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SessionName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetSessionName(&self, name: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetSessionName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(name)).ok() }
    }
    pub unsafe fn SessionThreadId(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SessionThreadId)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetSessionThreadId(&self, tid: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetSessionThreadId)(windows_core::Interface::as_raw(self), tid).ok() }
    }
    pub unsafe fn StreamMode(&self) -> windows_core::Result<StreamMode> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).StreamMode)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetStreamMode(&self, mode: StreamMode) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetStreamMode)(windows_core::Interface::as_raw(self), mode).ok() }
    }
    pub unsafe fn TraceDataProviders(&self) -> windows_core::Result<ITraceDataProviderCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).TraceDataProviders)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ITraceDataCollector_Vtbl {
    pub base__: IDataCollector_Vtbl,
    pub BufferSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetBufferSize: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub BuffersLost: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetBuffersLost: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub BuffersWritten: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetBuffersWritten: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub ClockType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut ClockType) -> windows_core::HRESULT,
    pub SetClockType: unsafe extern "system" fn(*mut core::ffi::c_void, ClockType) -> windows_core::HRESULT,
    pub EventsLost: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetEventsLost: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub ExtendedModes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetExtendedModes: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub FlushTimer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetFlushTimer: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub FreeBuffers: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetFreeBuffers: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Guid: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub SetGuid: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID) -> windows_core::HRESULT,
    pub IsKernelTrace: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub MaximumBuffers: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetMaximumBuffers: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub MinimumBuffers: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetMinimumBuffers: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub NumberOfBuffers: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetNumberOfBuffers: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub PreallocateFile: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetPreallocateFile: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub ProcessMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetProcessMode: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub RealTimeBuffersLost: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetRealTimeBuffersLost: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub SessionId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u64) -> windows_core::HRESULT,
    pub SetSessionId: unsafe extern "system" fn(*mut core::ffi::c_void, u64) -> windows_core::HRESULT,
    pub SessionName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetSessionName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SessionThreadId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetSessionThreadId: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub StreamMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut StreamMode) -> windows_core::HRESULT,
    pub SetStreamMode: unsafe extern "system" fn(*mut core::ffi::c_void, StreamMode) -> windows_core::HRESULT,
    pub TraceDataProviders: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ITraceDataCollector_Impl: IDataCollector_Impl {
    fn BufferSize(&self) -> windows_core::Result<u32>;
    fn SetBufferSize(&self, size: u32) -> windows_core::Result<()>;
    fn BuffersLost(&self) -> windows_core::Result<u32>;
    fn SetBuffersLost(&self, buffers: u32) -> windows_core::Result<()>;
    fn BuffersWritten(&self) -> windows_core::Result<u32>;
    fn SetBuffersWritten(&self, buffers: u32) -> windows_core::Result<()>;
    fn ClockType(&self) -> windows_core::Result<ClockType>;
    fn SetClockType(&self, clock: ClockType) -> windows_core::Result<()>;
    fn EventsLost(&self) -> windows_core::Result<u32>;
    fn SetEventsLost(&self, events: u32) -> windows_core::Result<()>;
    fn ExtendedModes(&self) -> windows_core::Result<u32>;
    fn SetExtendedModes(&self, mode: u32) -> windows_core::Result<()>;
    fn FlushTimer(&self) -> windows_core::Result<u32>;
    fn SetFlushTimer(&self, seconds: u32) -> windows_core::Result<()>;
    fn FreeBuffers(&self) -> windows_core::Result<u32>;
    fn SetFreeBuffers(&self, buffers: u32) -> windows_core::Result<()>;
    fn Guid(&self) -> windows_core::Result<windows_core::GUID>;
    fn SetGuid(&self, guid: &windows_core::GUID) -> windows_core::Result<()>;
    fn IsKernelTrace(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn MaximumBuffers(&self) -> windows_core::Result<u32>;
    fn SetMaximumBuffers(&self, buffers: u32) -> windows_core::Result<()>;
    fn MinimumBuffers(&self) -> windows_core::Result<u32>;
    fn SetMinimumBuffers(&self, buffers: u32) -> windows_core::Result<()>;
    fn NumberOfBuffers(&self) -> windows_core::Result<u32>;
    fn SetNumberOfBuffers(&self, buffers: u32) -> windows_core::Result<()>;
    fn PreallocateFile(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetPreallocateFile(&self, allocate: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn ProcessMode(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetProcessMode(&self, process: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn RealTimeBuffersLost(&self) -> windows_core::Result<u32>;
    fn SetRealTimeBuffersLost(&self, buffers: u32) -> windows_core::Result<()>;
    fn SessionId(&self) -> windows_core::Result<u64>;
    fn SetSessionId(&self, id: u64) -> windows_core::Result<()>;
    fn SessionName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetSessionName(&self, name: &windows_core::BSTR) -> windows_core::Result<()>;
    fn SessionThreadId(&self) -> windows_core::Result<u32>;
    fn SetSessionThreadId(&self, tid: u32) -> windows_core::Result<()>;
    fn StreamMode(&self) -> windows_core::Result<StreamMode>;
    fn SetStreamMode(&self, mode: StreamMode) -> windows_core::Result<()>;
    fn TraceDataProviders(&self) -> windows_core::Result<ITraceDataProviderCollection>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ITraceDataCollector_Vtbl {
    pub const fn new<Identity: ITraceDataCollector_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn BufferSize<Identity: ITraceDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, size: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITraceDataCollector_Impl::BufferSize(this) {
                    Ok(ok__) => {
                        size.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetBufferSize<Identity: ITraceDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, size: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITraceDataCollector_Impl::SetBufferSize(this, core::mem::transmute_copy(&size)).into()
            }
        }
        unsafe extern "system" fn BuffersLost<Identity: ITraceDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, buffers: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITraceDataCollector_Impl::BuffersLost(this) {
                    Ok(ok__) => {
                        buffers.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetBuffersLost<Identity: ITraceDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, buffers: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITraceDataCollector_Impl::SetBuffersLost(this, core::mem::transmute_copy(&buffers)).into()
            }
        }
        unsafe extern "system" fn BuffersWritten<Identity: ITraceDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, buffers: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITraceDataCollector_Impl::BuffersWritten(this) {
                    Ok(ok__) => {
                        buffers.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetBuffersWritten<Identity: ITraceDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, buffers: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITraceDataCollector_Impl::SetBuffersWritten(this, core::mem::transmute_copy(&buffers)).into()
            }
        }
        unsafe extern "system" fn ClockType<Identity: ITraceDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, clock: *mut ClockType) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITraceDataCollector_Impl::ClockType(this) {
                    Ok(ok__) => {
                        clock.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetClockType<Identity: ITraceDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, clock: ClockType) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITraceDataCollector_Impl::SetClockType(this, core::mem::transmute_copy(&clock)).into()
            }
        }
        unsafe extern "system" fn EventsLost<Identity: ITraceDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, events: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITraceDataCollector_Impl::EventsLost(this) {
                    Ok(ok__) => {
                        events.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetEventsLost<Identity: ITraceDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, events: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITraceDataCollector_Impl::SetEventsLost(this, core::mem::transmute_copy(&events)).into()
            }
        }
        unsafe extern "system" fn ExtendedModes<Identity: ITraceDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, mode: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITraceDataCollector_Impl::ExtendedModes(this) {
                    Ok(ok__) => {
                        mode.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetExtendedModes<Identity: ITraceDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, mode: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITraceDataCollector_Impl::SetExtendedModes(this, core::mem::transmute_copy(&mode)).into()
            }
        }
        unsafe extern "system" fn FlushTimer<Identity: ITraceDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, seconds: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITraceDataCollector_Impl::FlushTimer(this) {
                    Ok(ok__) => {
                        seconds.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetFlushTimer<Identity: ITraceDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, seconds: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITraceDataCollector_Impl::SetFlushTimer(this, core::mem::transmute_copy(&seconds)).into()
            }
        }
        unsafe extern "system" fn FreeBuffers<Identity: ITraceDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, buffers: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITraceDataCollector_Impl::FreeBuffers(this) {
                    Ok(ok__) => {
                        buffers.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetFreeBuffers<Identity: ITraceDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, buffers: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITraceDataCollector_Impl::SetFreeBuffers(this, core::mem::transmute_copy(&buffers)).into()
            }
        }
        unsafe extern "system" fn Guid<Identity: ITraceDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, guid: *mut windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITraceDataCollector_Impl::Guid(this) {
                    Ok(ok__) => {
                        guid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetGuid<Identity: ITraceDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, guid: windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITraceDataCollector_Impl::SetGuid(this, core::mem::transmute(&guid)).into()
            }
        }
        unsafe extern "system" fn IsKernelTrace<Identity: ITraceDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, kernel: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITraceDataCollector_Impl::IsKernelTrace(this) {
                    Ok(ok__) => {
                        kernel.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn MaximumBuffers<Identity: ITraceDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, buffers: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITraceDataCollector_Impl::MaximumBuffers(this) {
                    Ok(ok__) => {
                        buffers.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetMaximumBuffers<Identity: ITraceDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, buffers: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITraceDataCollector_Impl::SetMaximumBuffers(this, core::mem::transmute_copy(&buffers)).into()
            }
        }
        unsafe extern "system" fn MinimumBuffers<Identity: ITraceDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, buffers: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITraceDataCollector_Impl::MinimumBuffers(this) {
                    Ok(ok__) => {
                        buffers.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetMinimumBuffers<Identity: ITraceDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, buffers: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITraceDataCollector_Impl::SetMinimumBuffers(this, core::mem::transmute_copy(&buffers)).into()
            }
        }
        unsafe extern "system" fn NumberOfBuffers<Identity: ITraceDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, buffers: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITraceDataCollector_Impl::NumberOfBuffers(this) {
                    Ok(ok__) => {
                        buffers.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetNumberOfBuffers<Identity: ITraceDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, buffers: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITraceDataCollector_Impl::SetNumberOfBuffers(this, core::mem::transmute_copy(&buffers)).into()
            }
        }
        unsafe extern "system" fn PreallocateFile<Identity: ITraceDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, allocate: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITraceDataCollector_Impl::PreallocateFile(this) {
                    Ok(ok__) => {
                        allocate.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetPreallocateFile<Identity: ITraceDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, allocate: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITraceDataCollector_Impl::SetPreallocateFile(this, core::mem::transmute_copy(&allocate)).into()
            }
        }
        unsafe extern "system" fn ProcessMode<Identity: ITraceDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, process: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITraceDataCollector_Impl::ProcessMode(this) {
                    Ok(ok__) => {
                        process.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetProcessMode<Identity: ITraceDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, process: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITraceDataCollector_Impl::SetProcessMode(this, core::mem::transmute_copy(&process)).into()
            }
        }
        unsafe extern "system" fn RealTimeBuffersLost<Identity: ITraceDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, buffers: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITraceDataCollector_Impl::RealTimeBuffersLost(this) {
                    Ok(ok__) => {
                        buffers.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetRealTimeBuffersLost<Identity: ITraceDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, buffers: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITraceDataCollector_Impl::SetRealTimeBuffersLost(this, core::mem::transmute_copy(&buffers)).into()
            }
        }
        unsafe extern "system" fn SessionId<Identity: ITraceDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, id: *mut u64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITraceDataCollector_Impl::SessionId(this) {
                    Ok(ok__) => {
                        id.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetSessionId<Identity: ITraceDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, id: u64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITraceDataCollector_Impl::SetSessionId(this, core::mem::transmute_copy(&id)).into()
            }
        }
        unsafe extern "system" fn SessionName<Identity: ITraceDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITraceDataCollector_Impl::SessionName(this) {
                    Ok(ok__) => {
                        name.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetSessionName<Identity: ITraceDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITraceDataCollector_Impl::SetSessionName(this, core::mem::transmute(&name)).into()
            }
        }
        unsafe extern "system" fn SessionThreadId<Identity: ITraceDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, tid: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITraceDataCollector_Impl::SessionThreadId(this) {
                    Ok(ok__) => {
                        tid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetSessionThreadId<Identity: ITraceDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, tid: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITraceDataCollector_Impl::SetSessionThreadId(this, core::mem::transmute_copy(&tid)).into()
            }
        }
        unsafe extern "system" fn StreamMode<Identity: ITraceDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, mode: *mut StreamMode) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITraceDataCollector_Impl::StreamMode(this) {
                    Ok(ok__) => {
                        mode.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetStreamMode<Identity: ITraceDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, mode: StreamMode) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITraceDataCollector_Impl::SetStreamMode(this, core::mem::transmute_copy(&mode)).into()
            }
        }
        unsafe extern "system" fn TraceDataProviders<Identity: ITraceDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, providers: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITraceDataCollector_Impl::TraceDataProviders(this) {
                    Ok(ok__) => {
                        providers.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IDataCollector_Vtbl::new::<Identity, OFFSET>(),
            BufferSize: BufferSize::<Identity, OFFSET>,
            SetBufferSize: SetBufferSize::<Identity, OFFSET>,
            BuffersLost: BuffersLost::<Identity, OFFSET>,
            SetBuffersLost: SetBuffersLost::<Identity, OFFSET>,
            BuffersWritten: BuffersWritten::<Identity, OFFSET>,
            SetBuffersWritten: SetBuffersWritten::<Identity, OFFSET>,
            ClockType: ClockType::<Identity, OFFSET>,
            SetClockType: SetClockType::<Identity, OFFSET>,
            EventsLost: EventsLost::<Identity, OFFSET>,
            SetEventsLost: SetEventsLost::<Identity, OFFSET>,
            ExtendedModes: ExtendedModes::<Identity, OFFSET>,
            SetExtendedModes: SetExtendedModes::<Identity, OFFSET>,
            FlushTimer: FlushTimer::<Identity, OFFSET>,
            SetFlushTimer: SetFlushTimer::<Identity, OFFSET>,
            FreeBuffers: FreeBuffers::<Identity, OFFSET>,
            SetFreeBuffers: SetFreeBuffers::<Identity, OFFSET>,
            Guid: Guid::<Identity, OFFSET>,
            SetGuid: SetGuid::<Identity, OFFSET>,
            IsKernelTrace: IsKernelTrace::<Identity, OFFSET>,
            MaximumBuffers: MaximumBuffers::<Identity, OFFSET>,
            SetMaximumBuffers: SetMaximumBuffers::<Identity, OFFSET>,
            MinimumBuffers: MinimumBuffers::<Identity, OFFSET>,
            SetMinimumBuffers: SetMinimumBuffers::<Identity, OFFSET>,
            NumberOfBuffers: NumberOfBuffers::<Identity, OFFSET>,
            SetNumberOfBuffers: SetNumberOfBuffers::<Identity, OFFSET>,
            PreallocateFile: PreallocateFile::<Identity, OFFSET>,
            SetPreallocateFile: SetPreallocateFile::<Identity, OFFSET>,
            ProcessMode: ProcessMode::<Identity, OFFSET>,
            SetProcessMode: SetProcessMode::<Identity, OFFSET>,
            RealTimeBuffersLost: RealTimeBuffersLost::<Identity, OFFSET>,
            SetRealTimeBuffersLost: SetRealTimeBuffersLost::<Identity, OFFSET>,
            SessionId: SessionId::<Identity, OFFSET>,
            SetSessionId: SetSessionId::<Identity, OFFSET>,
            SessionName: SessionName::<Identity, OFFSET>,
            SetSessionName: SetSessionName::<Identity, OFFSET>,
            SessionThreadId: SessionThreadId::<Identity, OFFSET>,
            SetSessionThreadId: SetSessionThreadId::<Identity, OFFSET>,
            StreamMode: StreamMode::<Identity, OFFSET>,
            SetStreamMode: SetStreamMode::<Identity, OFFSET>,
            TraceDataProviders: TraceDataProviders::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITraceDataCollector as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID || iid == &<IDataCollector as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ITraceDataCollector {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ITraceDataProvider, ITraceDataProvider_Vtbl, 0x03837512_098b_11d8_9414_505054503030);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ITraceDataProvider {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ITraceDataProvider, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ITraceDataProvider {
    pub unsafe fn DisplayName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DisplayName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetDisplayName(&self, name: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetDisplayName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(name)).ok() }
    }
    pub unsafe fn Guid(&self) -> windows_core::Result<windows_core::GUID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Guid)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetGuid(&self, guid: windows_core::GUID) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetGuid)(windows_core::Interface::as_raw(self), core::mem::transmute(guid)).ok() }
    }
    pub unsafe fn Level(&self) -> windows_core::Result<IValueMap> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Level)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn KeywordsAny(&self) -> windows_core::Result<IValueMap> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).KeywordsAny)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn KeywordsAll(&self) -> windows_core::Result<IValueMap> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).KeywordsAll)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Properties(&self) -> windows_core::Result<IValueMap> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Properties)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn FilterEnabled(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FilterEnabled)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetFilterEnabled(&self, filterenabled: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetFilterEnabled)(windows_core::Interface::as_raw(self), filterenabled).ok() }
    }
    pub unsafe fn FilterType(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FilterType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetFilterType(&self, ultype: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetFilterType)(windows_core::Interface::as_raw(self), ultype).ok() }
    }
    pub unsafe fn FilterData(&self) -> windows_core::Result<*mut super::Com::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FilterData)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetFilterData(&self, pdata: *const super::Com::SAFEARRAY) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetFilterData)(windows_core::Interface::as_raw(self), pdata).ok() }
    }
    pub unsafe fn Query(&self, bstrname: &windows_core::BSTR, bstrserver: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Query)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrname), core::mem::transmute_copy(bstrserver)).ok() }
    }
    pub unsafe fn Resolve<P0>(&self, pfrom: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Com::IDispatch>,
    {
        unsafe { (windows_core::Interface::vtable(self).Resolve)(windows_core::Interface::as_raw(self), pfrom.param().abi()).ok() }
    }
    pub unsafe fn SetSecurity(&self, sddl: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetSecurity)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(sddl)).ok() }
    }
    pub unsafe fn GetSecurity(&self, securityinfo: u32) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSecurity)(windows_core::Interface::as_raw(self), securityinfo, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn GetRegisteredProcesses(&self) -> windows_core::Result<IValueMap> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetRegisteredProcesses)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ITraceDataProvider_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub DisplayName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetDisplayName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Guid: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub SetGuid: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID) -> windows_core::HRESULT,
    pub Level: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub KeywordsAny: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub KeywordsAll: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Properties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FilterEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetFilterEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub FilterType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetFilterType: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub FilterData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::Com::SAFEARRAY) -> windows_core::HRESULT,
    pub SetFilterData: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::Com::SAFEARRAY) -> windows_core::HRESULT,
    pub Query: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Resolve: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetSecurity: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetSecurity: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetRegisteredProcesses: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ITraceDataProvider_Impl: super::Com::IDispatch_Impl {
    fn DisplayName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetDisplayName(&self, name: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Guid(&self) -> windows_core::Result<windows_core::GUID>;
    fn SetGuid(&self, guid: &windows_core::GUID) -> windows_core::Result<()>;
    fn Level(&self) -> windows_core::Result<IValueMap>;
    fn KeywordsAny(&self) -> windows_core::Result<IValueMap>;
    fn KeywordsAll(&self) -> windows_core::Result<IValueMap>;
    fn Properties(&self) -> windows_core::Result<IValueMap>;
    fn FilterEnabled(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetFilterEnabled(&self, filterenabled: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn FilterType(&self) -> windows_core::Result<u32>;
    fn SetFilterType(&self, ultype: u32) -> windows_core::Result<()>;
    fn FilterData(&self) -> windows_core::Result<*mut super::Com::SAFEARRAY>;
    fn SetFilterData(&self, pdata: *const super::Com::SAFEARRAY) -> windows_core::Result<()>;
    fn Query(&self, bstrname: &windows_core::BSTR, bstrserver: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Resolve(&self, pfrom: windows_core::Ref<super::Com::IDispatch>) -> windows_core::Result<()>;
    fn SetSecurity(&self, sddl: &windows_core::BSTR) -> windows_core::Result<()>;
    fn GetSecurity(&self, securityinfo: u32) -> windows_core::Result<windows_core::BSTR>;
    fn GetRegisteredProcesses(&self) -> windows_core::Result<IValueMap>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ITraceDataProvider_Vtbl {
    pub const fn new<Identity: ITraceDataProvider_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn DisplayName<Identity: ITraceDataProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITraceDataProvider_Impl::DisplayName(this) {
                    Ok(ok__) => {
                        name.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetDisplayName<Identity: ITraceDataProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITraceDataProvider_Impl::SetDisplayName(this, core::mem::transmute(&name)).into()
            }
        }
        unsafe extern "system" fn Guid<Identity: ITraceDataProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, guid: *mut windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITraceDataProvider_Impl::Guid(this) {
                    Ok(ok__) => {
                        guid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetGuid<Identity: ITraceDataProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, guid: windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITraceDataProvider_Impl::SetGuid(this, core::mem::transmute(&guid)).into()
            }
        }
        unsafe extern "system" fn Level<Identity: ITraceDataProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pplevel: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITraceDataProvider_Impl::Level(this) {
                    Ok(ok__) => {
                        pplevel.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn KeywordsAny<Identity: ITraceDataProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppkeywords: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITraceDataProvider_Impl::KeywordsAny(this) {
                    Ok(ok__) => {
                        ppkeywords.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn KeywordsAll<Identity: ITraceDataProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppkeywords: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITraceDataProvider_Impl::KeywordsAll(this) {
                    Ok(ok__) => {
                        ppkeywords.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Properties<Identity: ITraceDataProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppproperties: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITraceDataProvider_Impl::Properties(this) {
                    Ok(ok__) => {
                        ppproperties.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn FilterEnabled<Identity: ITraceDataProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, filterenabled: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITraceDataProvider_Impl::FilterEnabled(this) {
                    Ok(ok__) => {
                        filterenabled.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetFilterEnabled<Identity: ITraceDataProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, filterenabled: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITraceDataProvider_Impl::SetFilterEnabled(this, core::mem::transmute_copy(&filterenabled)).into()
            }
        }
        unsafe extern "system" fn FilterType<Identity: ITraceDataProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pultype: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITraceDataProvider_Impl::FilterType(this) {
                    Ok(ok__) => {
                        pultype.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetFilterType<Identity: ITraceDataProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ultype: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITraceDataProvider_Impl::SetFilterType(this, core::mem::transmute_copy(&ultype)).into()
            }
        }
        unsafe extern "system" fn FilterData<Identity: ITraceDataProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppdata: *mut *mut super::Com::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITraceDataProvider_Impl::FilterData(this) {
                    Ok(ok__) => {
                        ppdata.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetFilterData<Identity: ITraceDataProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdata: *const super::Com::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITraceDataProvider_Impl::SetFilterData(this, core::mem::transmute_copy(&pdata)).into()
            }
        }
        unsafe extern "system" fn Query<Identity: ITraceDataProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrname: *mut core::ffi::c_void, bstrserver: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITraceDataProvider_Impl::Query(this, core::mem::transmute(&bstrname), core::mem::transmute(&bstrserver)).into()
            }
        }
        unsafe extern "system" fn Resolve<Identity: ITraceDataProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfrom: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITraceDataProvider_Impl::Resolve(this, core::mem::transmute_copy(&pfrom)).into()
            }
        }
        unsafe extern "system" fn SetSecurity<Identity: ITraceDataProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sddl: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITraceDataProvider_Impl::SetSecurity(this, core::mem::transmute(&sddl)).into()
            }
        }
        unsafe extern "system" fn GetSecurity<Identity: ITraceDataProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, securityinfo: u32, sddl: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITraceDataProvider_Impl::GetSecurity(this, core::mem::transmute_copy(&securityinfo)) {
                    Ok(ok__) => {
                        sddl.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetRegisteredProcesses<Identity: ITraceDataProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, processes: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITraceDataProvider_Impl::GetRegisteredProcesses(this) {
                    Ok(ok__) => {
                        processes.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            DisplayName: DisplayName::<Identity, OFFSET>,
            SetDisplayName: SetDisplayName::<Identity, OFFSET>,
            Guid: Guid::<Identity, OFFSET>,
            SetGuid: SetGuid::<Identity, OFFSET>,
            Level: Level::<Identity, OFFSET>,
            KeywordsAny: KeywordsAny::<Identity, OFFSET>,
            KeywordsAll: KeywordsAll::<Identity, OFFSET>,
            Properties: Properties::<Identity, OFFSET>,
            FilterEnabled: FilterEnabled::<Identity, OFFSET>,
            SetFilterEnabled: SetFilterEnabled::<Identity, OFFSET>,
            FilterType: FilterType::<Identity, OFFSET>,
            SetFilterType: SetFilterType::<Identity, OFFSET>,
            FilterData: FilterData::<Identity, OFFSET>,
            SetFilterData: SetFilterData::<Identity, OFFSET>,
            Query: Query::<Identity, OFFSET>,
            Resolve: Resolve::<Identity, OFFSET>,
            SetSecurity: SetSecurity::<Identity, OFFSET>,
            GetSecurity: GetSecurity::<Identity, OFFSET>,
            GetRegisteredProcesses: GetRegisteredProcesses::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITraceDataProvider as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ITraceDataProvider {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ITraceDataProviderCollection, ITraceDataProviderCollection_Vtbl, 0x03837510_098b_11d8_9414_505054503030);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ITraceDataProviderCollection {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ITraceDataProviderCollection, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ITraceDataProviderCollection {
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn get_Item(&self, index: &super::Variant::VARIANT) -> windows_core::Result<ITraceDataProvider> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Item)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(index), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Add<P0>(&self, pprovider: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ITraceDataProvider>,
    {
        unsafe { (windows_core::Interface::vtable(self).Add)(windows_core::Interface::as_raw(self), pprovider.param().abi()).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Remove(&self, vprovider: &super::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Remove)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(vprovider)).ok() }
    }
    pub unsafe fn Clear(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Clear)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn AddRange<P0>(&self, providers: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ITraceDataProviderCollection>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddRange)(windows_core::Interface::as_raw(self), providers.param().abi()).ok() }
    }
    pub unsafe fn CreateTraceDataProvider(&self) -> windows_core::Result<ITraceDataProvider> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateTraceDataProvider)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetTraceDataProviders(&self, server: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetTraceDataProviders)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(server)).ok() }
    }
    pub unsafe fn GetTraceDataProvidersByProcess(&self, server: &windows_core::BSTR, pid: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetTraceDataProvidersByProcess)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(server), pid).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ITraceDataProviderCollection_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub get_Item: unsafe extern "system" fn(*mut core::ffi::c_void, super::Variant::VARIANT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    get_Item: usize,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Add: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub Remove: unsafe extern "system" fn(*mut core::ffi::c_void, super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    Remove: usize,
    pub Clear: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddRange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateTraceDataProvider: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetTraceDataProviders: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetTraceDataProvidersByProcess: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ITraceDataProviderCollection_Impl: super::Com::IDispatch_Impl {
    fn Count(&self) -> windows_core::Result<i32>;
    fn get_Item(&self, index: &super::Variant::VARIANT) -> windows_core::Result<ITraceDataProvider>;
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn Add(&self, pprovider: windows_core::Ref<ITraceDataProvider>) -> windows_core::Result<()>;
    fn Remove(&self, vprovider: &super::Variant::VARIANT) -> windows_core::Result<()>;
    fn Clear(&self) -> windows_core::Result<()>;
    fn AddRange(&self, providers: windows_core::Ref<ITraceDataProviderCollection>) -> windows_core::Result<()>;
    fn CreateTraceDataProvider(&self) -> windows_core::Result<ITraceDataProvider>;
    fn GetTraceDataProviders(&self, server: &windows_core::BSTR) -> windows_core::Result<()>;
    fn GetTraceDataProvidersByProcess(&self, server: &windows_core::BSTR, pid: u32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ITraceDataProviderCollection_Vtbl {
    pub const fn new<Identity: ITraceDataProviderCollection_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Count<Identity: ITraceDataProviderCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITraceDataProviderCollection_Impl::Count(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Item<Identity: ITraceDataProviderCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: super::Variant::VARIANT, ppprovider: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITraceDataProviderCollection_Impl::get_Item(this, core::mem::transmute(&index)) {
                    Ok(ok__) => {
                        ppprovider.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: ITraceDataProviderCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITraceDataProviderCollection_Impl::_NewEnum(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Add<Identity: ITraceDataProviderCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pprovider: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITraceDataProviderCollection_Impl::Add(this, core::mem::transmute_copy(&pprovider)).into()
            }
        }
        unsafe extern "system" fn Remove<Identity: ITraceDataProviderCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vprovider: super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITraceDataProviderCollection_Impl::Remove(this, core::mem::transmute(&vprovider)).into()
            }
        }
        unsafe extern "system" fn Clear<Identity: ITraceDataProviderCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITraceDataProviderCollection_Impl::Clear(this).into()
            }
        }
        unsafe extern "system" fn AddRange<Identity: ITraceDataProviderCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, providers: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITraceDataProviderCollection_Impl::AddRange(this, core::mem::transmute_copy(&providers)).into()
            }
        }
        unsafe extern "system" fn CreateTraceDataProvider<Identity: ITraceDataProviderCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, provider: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITraceDataProviderCollection_Impl::CreateTraceDataProvider(this) {
                    Ok(ok__) => {
                        provider.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetTraceDataProviders<Identity: ITraceDataProviderCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, server: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITraceDataProviderCollection_Impl::GetTraceDataProviders(this, core::mem::transmute(&server)).into()
            }
        }
        unsafe extern "system" fn GetTraceDataProvidersByProcess<Identity: ITraceDataProviderCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, server: *mut core::ffi::c_void, pid: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITraceDataProviderCollection_Impl::GetTraceDataProvidersByProcess(this, core::mem::transmute(&server), core::mem::transmute_copy(&pid)).into()
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Count: Count::<Identity, OFFSET>,
            get_Item: get_Item::<Identity, OFFSET>,
            _NewEnum: _NewEnum::<Identity, OFFSET>,
            Add: Add::<Identity, OFFSET>,
            Remove: Remove::<Identity, OFFSET>,
            Clear: Clear::<Identity, OFFSET>,
            AddRange: AddRange::<Identity, OFFSET>,
            CreateTraceDataProvider: CreateTraceDataProvider::<Identity, OFFSET>,
            GetTraceDataProviders: GetTraceDataProviders::<Identity, OFFSET>,
            GetTraceDataProvidersByProcess: GetTraceDataProvidersByProcess::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITraceDataProviderCollection as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ITraceDataProviderCollection {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IValueMap, IValueMap_Vtbl, 0x03837534_098b_11d8_9414_505054503030);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IValueMap {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IValueMap, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IValueMap {
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn get_Item(&self, index: &super::Variant::VARIANT) -> windows_core::Result<IValueMapItem> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Item)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(index), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Description(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Description)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetDescription(&self, description: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetDescription)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(description)).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Value(&self) -> windows_core::Result<super::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Value)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn SetValue(&self, value: &super::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetValue)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(value)).ok() }
    }
    pub unsafe fn ValueMapType(&self) -> windows_core::Result<ValueMapType> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ValueMapType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetValueMapType(&self, r#type: ValueMapType) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetValueMapType)(windows_core::Interface::as_raw(self), r#type).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Add(&self, value: &super::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Add)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(value)).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Remove(&self, value: &super::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Remove)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(value)).ok() }
    }
    pub unsafe fn Clear(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Clear)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn AddRange<P0>(&self, map: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IValueMap>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddRange)(windows_core::Interface::as_raw(self), map.param().abi()).ok() }
    }
    pub unsafe fn CreateValueMapItem(&self) -> windows_core::Result<IValueMapItem> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateValueMapItem)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IValueMap_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub get_Item: unsafe extern "system" fn(*mut core::ffi::c_void, super::Variant::VARIANT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    get_Item: usize,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Description: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub Value: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    Value: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub SetValue: unsafe extern "system" fn(*mut core::ffi::c_void, super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    SetValue: usize,
    pub ValueMapType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut ValueMapType) -> windows_core::HRESULT,
    pub SetValueMapType: unsafe extern "system" fn(*mut core::ffi::c_void, ValueMapType) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub Add: unsafe extern "system" fn(*mut core::ffi::c_void, super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    Add: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub Remove: unsafe extern "system" fn(*mut core::ffi::c_void, super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    Remove: usize,
    pub Clear: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddRange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateValueMapItem: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IValueMap_Impl: super::Com::IDispatch_Impl {
    fn Count(&self) -> windows_core::Result<i32>;
    fn get_Item(&self, index: &super::Variant::VARIANT) -> windows_core::Result<IValueMapItem>;
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn Description(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetDescription(&self, description: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Value(&self) -> windows_core::Result<super::Variant::VARIANT>;
    fn SetValue(&self, value: &super::Variant::VARIANT) -> windows_core::Result<()>;
    fn ValueMapType(&self) -> windows_core::Result<ValueMapType>;
    fn SetValueMapType(&self, r#type: ValueMapType) -> windows_core::Result<()>;
    fn Add(&self, value: &super::Variant::VARIANT) -> windows_core::Result<()>;
    fn Remove(&self, value: &super::Variant::VARIANT) -> windows_core::Result<()>;
    fn Clear(&self) -> windows_core::Result<()>;
    fn AddRange(&self, map: windows_core::Ref<IValueMap>) -> windows_core::Result<()>;
    fn CreateValueMapItem(&self) -> windows_core::Result<IValueMapItem>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IValueMap_Vtbl {
    pub const fn new<Identity: IValueMap_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Count<Identity: IValueMap_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IValueMap_Impl::Count(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Item<Identity: IValueMap_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: super::Variant::VARIANT, value: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IValueMap_Impl::get_Item(this, core::mem::transmute(&index)) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: IValueMap_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IValueMap_Impl::_NewEnum(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Description<Identity: IValueMap_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, description: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IValueMap_Impl::Description(this) {
                    Ok(ok__) => {
                        description.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetDescription<Identity: IValueMap_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, description: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IValueMap_Impl::SetDescription(this, core::mem::transmute(&description)).into()
            }
        }
        unsafe extern "system" fn Value<Identity: IValueMap_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IValueMap_Impl::Value(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetValue<Identity: IValueMap_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IValueMap_Impl::SetValue(this, core::mem::transmute(&value)).into()
            }
        }
        unsafe extern "system" fn ValueMapType<Identity: IValueMap_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, r#type: *mut ValueMapType) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IValueMap_Impl::ValueMapType(this) {
                    Ok(ok__) => {
                        r#type.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetValueMapType<Identity: IValueMap_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, r#type: ValueMapType) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IValueMap_Impl::SetValueMapType(this, core::mem::transmute_copy(&r#type)).into()
            }
        }
        unsafe extern "system" fn Add<Identity: IValueMap_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IValueMap_Impl::Add(this, core::mem::transmute(&value)).into()
            }
        }
        unsafe extern "system" fn Remove<Identity: IValueMap_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IValueMap_Impl::Remove(this, core::mem::transmute(&value)).into()
            }
        }
        unsafe extern "system" fn Clear<Identity: IValueMap_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IValueMap_Impl::Clear(this).into()
            }
        }
        unsafe extern "system" fn AddRange<Identity: IValueMap_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, map: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IValueMap_Impl::AddRange(this, core::mem::transmute_copy(&map)).into()
            }
        }
        unsafe extern "system" fn CreateValueMapItem<Identity: IValueMap_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, item: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IValueMap_Impl::CreateValueMapItem(this) {
                    Ok(ok__) => {
                        item.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Count: Count::<Identity, OFFSET>,
            get_Item: get_Item::<Identity, OFFSET>,
            _NewEnum: _NewEnum::<Identity, OFFSET>,
            Description: Description::<Identity, OFFSET>,
            SetDescription: SetDescription::<Identity, OFFSET>,
            Value: Value::<Identity, OFFSET>,
            SetValue: SetValue::<Identity, OFFSET>,
            ValueMapType: ValueMapType::<Identity, OFFSET>,
            SetValueMapType: SetValueMapType::<Identity, OFFSET>,
            Add: Add::<Identity, OFFSET>,
            Remove: Remove::<Identity, OFFSET>,
            Clear: Clear::<Identity, OFFSET>,
            AddRange: AddRange::<Identity, OFFSET>,
            CreateValueMapItem: CreateValueMapItem::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IValueMap as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IValueMap {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IValueMapItem, IValueMapItem_Vtbl, 0x03837533_098b_11d8_9414_505054503030);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IValueMapItem {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IValueMapItem, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IValueMapItem {
    pub unsafe fn Description(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Description)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetDescription(&self, description: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetDescription)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(description)).ok() }
    }
    pub unsafe fn Enabled(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Enabled)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetEnabled(&self, enabled: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetEnabled)(windows_core::Interface::as_raw(self), enabled).ok() }
    }
    pub unsafe fn Key(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Key)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetKey(&self, key: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetKey)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(key)).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Value(&self) -> windows_core::Result<super::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Value)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn SetValue(&self, value: &super::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetValue)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(value)).ok() }
    }
    pub unsafe fn ValueMapType(&self) -> windows_core::Result<ValueMapType> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ValueMapType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetValueMapType(&self, r#type: ValueMapType) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetValueMapType)(windows_core::Interface::as_raw(self), r#type).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IValueMapItem_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Description: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Enabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub Key: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetKey: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub Value: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    Value: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub SetValue: unsafe extern "system" fn(*mut core::ffi::c_void, super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    SetValue: usize,
    pub ValueMapType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut ValueMapType) -> windows_core::HRESULT,
    pub SetValueMapType: unsafe extern "system" fn(*mut core::ffi::c_void, ValueMapType) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IValueMapItem_Impl: super::Com::IDispatch_Impl {
    fn Description(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetDescription(&self, description: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Enabled(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetEnabled(&self, enabled: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn Key(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetKey(&self, key: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Value(&self) -> windows_core::Result<super::Variant::VARIANT>;
    fn SetValue(&self, value: &super::Variant::VARIANT) -> windows_core::Result<()>;
    fn ValueMapType(&self) -> windows_core::Result<ValueMapType>;
    fn SetValueMapType(&self, r#type: ValueMapType) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IValueMapItem_Vtbl {
    pub const fn new<Identity: IValueMapItem_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Description<Identity: IValueMapItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, description: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IValueMapItem_Impl::Description(this) {
                    Ok(ok__) => {
                        description.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetDescription<Identity: IValueMapItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, description: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IValueMapItem_Impl::SetDescription(this, core::mem::transmute(&description)).into()
            }
        }
        unsafe extern "system" fn Enabled<Identity: IValueMapItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, enabled: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IValueMapItem_Impl::Enabled(this) {
                    Ok(ok__) => {
                        enabled.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetEnabled<Identity: IValueMapItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, enabled: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IValueMapItem_Impl::SetEnabled(this, core::mem::transmute_copy(&enabled)).into()
            }
        }
        unsafe extern "system" fn Key<Identity: IValueMapItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, key: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IValueMapItem_Impl::Key(this) {
                    Ok(ok__) => {
                        key.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetKey<Identity: IValueMapItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, key: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IValueMapItem_Impl::SetKey(this, core::mem::transmute(&key)).into()
            }
        }
        unsafe extern "system" fn Value<Identity: IValueMapItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IValueMapItem_Impl::Value(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetValue<Identity: IValueMapItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IValueMapItem_Impl::SetValue(this, core::mem::transmute(&value)).into()
            }
        }
        unsafe extern "system" fn ValueMapType<Identity: IValueMapItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, r#type: *mut ValueMapType) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IValueMapItem_Impl::ValueMapType(this) {
                    Ok(ok__) => {
                        r#type.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetValueMapType<Identity: IValueMapItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, r#type: ValueMapType) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IValueMapItem_Impl::SetValueMapType(this, core::mem::transmute_copy(&r#type)).into()
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Description: Description::<Identity, OFFSET>,
            SetDescription: SetDescription::<Identity, OFFSET>,
            Enabled: Enabled::<Identity, OFFSET>,
            SetEnabled: SetEnabled::<Identity, OFFSET>,
            Key: Key::<Identity, OFFSET>,
            SetKey: SetKey::<Identity, OFFSET>,
            Value: Value::<Identity, OFFSET>,
            SetValue: SetValue::<Identity, OFFSET>,
            ValueMapType: ValueMapType::<Identity, OFFSET>,
            SetValueMapType: SetValueMapType::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IValueMapItem as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IValueMapItem {}
pub const LIBID_SystemMonitor: windows_core::GUID = windows_core::GUID::from_u128(0x1b773e42_2509_11cf_942f_008029004347);
pub const LegacyDataCollectorSet: windows_core::GUID = windows_core::GUID::from_u128(0x03837526_098b_11d8_9414_505054503030);
pub const LegacyDataCollectorSetCollection: windows_core::GUID = windows_core::GUID::from_u128(0x03837527_098b_11d8_9414_505054503030);
pub const LegacyTraceSession: windows_core::GUID = windows_core::GUID::from_u128(0x03837528_098b_11d8_9414_505054503030);
pub const LegacyTraceSessionCollection: windows_core::GUID = windows_core::GUID::from_u128(0x03837529_098b_11d8_9414_505054503030);
pub const LogFileItem: windows_core::GUID = windows_core::GUID::from_u128(0x16ec5be8_df93_4237_94e4_9ee918111d71);
pub const LogFiles: windows_core::GUID = windows_core::GUID::from_u128(0x2735d9fd_f6b9_4f19_a5d9_e2d068584bc5);
pub const MAX_COUNTER_PATH: u32 = 256u32;
pub const MAX_PERF_OBJECTS_IN_QUERY_FUNCTION: i32 = 64i32;
pub const PDH_ACCESS_DENIED: u32 = 3221228507u32;
pub const PDH_ASYNC_QUERY_TIMEOUT: u32 = 2147485659u32;
pub const PDH_BINARY_LOG_CORRUPT: u32 = 3221228535u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct PDH_BROWSE_DLG_CONFIG_A {
    pub _bitfield: u32,
    pub hWndOwner: super::super::Foundation::HWND,
    pub szDataSource: windows_core::PSTR,
    pub szReturnPathBuffer: windows_core::PSTR,
    pub cchReturnPathLength: u32,
    pub pCallBack: CounterPathCallBack,
    pub dwCallBackArg: usize,
    pub CallBackStatus: i32,
    pub dwDefaultDetailLevel: PERF_DETAIL,
    pub szDialogBoxCaption: windows_core::PSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct PDH_BROWSE_DLG_CONFIG_HA {
    pub _bitfield: u32,
    pub hWndOwner: super::super::Foundation::HWND,
    pub hDataSource: PDH_HLOG,
    pub szReturnPathBuffer: windows_core::PSTR,
    pub cchReturnPathLength: u32,
    pub pCallBack: CounterPathCallBack,
    pub dwCallBackArg: usize,
    pub CallBackStatus: i32,
    pub dwDefaultDetailLevel: PERF_DETAIL,
    pub szDialogBoxCaption: windows_core::PSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct PDH_BROWSE_DLG_CONFIG_HW {
    pub _bitfield: u32,
    pub hWndOwner: super::super::Foundation::HWND,
    pub hDataSource: PDH_HLOG,
    pub szReturnPathBuffer: windows_core::PWSTR,
    pub cchReturnPathLength: u32,
    pub pCallBack: CounterPathCallBack,
    pub dwCallBackArg: usize,
    pub CallBackStatus: i32,
    pub dwDefaultDetailLevel: PERF_DETAIL,
    pub szDialogBoxCaption: windows_core::PWSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct PDH_BROWSE_DLG_CONFIG_W {
    pub _bitfield: u32,
    pub hWndOwner: super::super::Foundation::HWND,
    pub szDataSource: windows_core::PWSTR,
    pub szReturnPathBuffer: windows_core::PWSTR,
    pub cchReturnPathLength: u32,
    pub pCallBack: CounterPathCallBack,
    pub dwCallBackArg: usize,
    pub CallBackStatus: i32,
    pub dwDefaultDetailLevel: PERF_DETAIL,
    pub szDialogBoxCaption: windows_core::PWSTR,
}
pub const PDH_CALC_NEGATIVE_DENOMINATOR: u32 = 2147485654u32;
pub const PDH_CALC_NEGATIVE_TIMEBASE: u32 = 2147485655u32;
pub const PDH_CALC_NEGATIVE_VALUE: u32 = 2147485656u32;
pub const PDH_CANNOT_CONNECT_MACHINE: u32 = 3221228483u32;
pub const PDH_CANNOT_CONNECT_WMI_SERVER: u32 = 3221228520u32;
pub const PDH_CANNOT_READ_NAME_STRINGS: u32 = 3221228488u32;
pub const PDH_CANNOT_SET_DEFAULT_REALTIME_DATASOURCE: u32 = 2147485660u32;
pub const PDH_COUNTER_ALREADY_IN_QUERY: u32 = 3221228534u32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PDH_COUNTER_INFO_A {
    pub dwLength: u32,
    pub dwType: u32,
    pub CVersion: u32,
    pub CStatus: u32,
    pub lScale: i32,
    pub lDefaultScale: i32,
    pub dwUserData: usize,
    pub dwQueryUserData: usize,
    pub szFullPath: windows_core::PSTR,
    pub Anonymous: PDH_COUNTER_INFO_A_0,
    pub szExplainText: windows_core::PSTR,
    pub DataBuffer: [u32; 1],
}
impl Default for PDH_COUNTER_INFO_A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PDH_COUNTER_INFO_A_0 {
    pub DataItemPath: PDH_DATA_ITEM_PATH_ELEMENTS_A,
    pub CounterPath: PDH_COUNTER_PATH_ELEMENTS_A,
    pub Anonymous: PDH_COUNTER_INFO_A_0_0,
}
impl Default for PDH_COUNTER_INFO_A_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PDH_COUNTER_INFO_A_0_0 {
    pub szMachineName: windows_core::PSTR,
    pub szObjectName: windows_core::PSTR,
    pub szInstanceName: windows_core::PSTR,
    pub szParentInstance: windows_core::PSTR,
    pub dwInstanceIndex: u32,
    pub szCounterName: windows_core::PSTR,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PDH_COUNTER_INFO_W {
    pub dwLength: u32,
    pub dwType: u32,
    pub CVersion: u32,
    pub CStatus: u32,
    pub lScale: i32,
    pub lDefaultScale: i32,
    pub dwUserData: usize,
    pub dwQueryUserData: usize,
    pub szFullPath: windows_core::PWSTR,
    pub Anonymous: PDH_COUNTER_INFO_W_0,
    pub szExplainText: windows_core::PWSTR,
    pub DataBuffer: [u32; 1],
}
impl Default for PDH_COUNTER_INFO_W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PDH_COUNTER_INFO_W_0 {
    pub DataItemPath: PDH_DATA_ITEM_PATH_ELEMENTS_W,
    pub CounterPath: PDH_COUNTER_PATH_ELEMENTS_W,
    pub Anonymous: PDH_COUNTER_INFO_W_0_0,
}
impl Default for PDH_COUNTER_INFO_W_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PDH_COUNTER_INFO_W_0_0 {
    pub szMachineName: windows_core::PWSTR,
    pub szObjectName: windows_core::PWSTR,
    pub szInstanceName: windows_core::PWSTR,
    pub szParentInstance: windows_core::PWSTR,
    pub dwInstanceIndex: u32,
    pub szCounterName: windows_core::PWSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PDH_COUNTER_PATH_ELEMENTS_A {
    pub szMachineName: windows_core::PSTR,
    pub szObjectName: windows_core::PSTR,
    pub szInstanceName: windows_core::PSTR,
    pub szParentInstance: windows_core::PSTR,
    pub dwInstanceIndex: u32,
    pub szCounterName: windows_core::PSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PDH_COUNTER_PATH_ELEMENTS_W {
    pub szMachineName: windows_core::PWSTR,
    pub szObjectName: windows_core::PWSTR,
    pub szInstanceName: windows_core::PWSTR,
    pub szParentInstance: windows_core::PWSTR,
    pub dwInstanceIndex: u32,
    pub szCounterName: windows_core::PWSTR,
}
pub const PDH_CSTATUS_BAD_COUNTERNAME: u32 = 3221228480u32;
pub const PDH_CSTATUS_INVALID_DATA: u32 = 3221228474u32;
pub const PDH_CSTATUS_ITEM_NOT_VALIDATED: u32 = 2147485651u32;
pub const PDH_CSTATUS_NEW_DATA: u32 = 1u32;
pub const PDH_CSTATUS_NO_COUNTER: u32 = 3221228473u32;
pub const PDH_CSTATUS_NO_COUNTERNAME: u32 = 3221228479u32;
pub const PDH_CSTATUS_NO_INSTANCE: u32 = 2147485649u32;
pub const PDH_CSTATUS_NO_MACHINE: u32 = 2147485648u32;
pub const PDH_CSTATUS_NO_OBJECT: u32 = 3221228472u32;
pub const PDH_CSTATUS_VALID_DATA: u32 = 0u32;
pub const PDH_CVERSION_WIN50: PDH_DLL_VERSION = PDH_DLL_VERSION(1280u32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PDH_DATA_ITEM_PATH_ELEMENTS_A {
    pub szMachineName: windows_core::PSTR,
    pub ObjectGUID: windows_core::GUID,
    pub dwItemId: u32,
    pub szInstanceName: windows_core::PSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PDH_DATA_ITEM_PATH_ELEMENTS_W {
    pub szMachineName: windows_core::PWSTR,
    pub ObjectGUID: windows_core::GUID,
    pub dwItemId: u32,
    pub szInstanceName: windows_core::PWSTR,
}
pub const PDH_DATA_SOURCE_IS_LOG_FILE: u32 = 3221228494u32;
pub const PDH_DATA_SOURCE_IS_REAL_TIME: u32 = 3221228495u32;
pub const PDH_DIALOG_CANCELLED: u32 = 2147485657u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PDH_DLL_VERSION(pub u32);
pub const PDH_END_OF_LOG_FILE: u32 = 2147485658u32;
pub const PDH_ENTRY_NOT_IN_LOG_FILE: u32 = 3221228493u32;
pub const PDH_FILE_ALREADY_EXISTS: u32 = 3221228498u32;
pub const PDH_FILE_NOT_FOUND: u32 = 3221228497u32;
pub const PDH_FLAGS_FILE_BROWSER_ONLY: PDH_SELECT_DATA_SOURCE_FLAGS = PDH_SELECT_DATA_SOURCE_FLAGS(1u32);
pub const PDH_FLAGS_NONE: PDH_SELECT_DATA_SOURCE_FLAGS = PDH_SELECT_DATA_SOURCE_FLAGS(0u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PDH_FMT(pub u32);
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PDH_FMT_COUNTERVALUE {
    pub CStatus: u32,
    pub Anonymous: PDH_FMT_COUNTERVALUE_0,
}
impl Default for PDH_FMT_COUNTERVALUE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PDH_FMT_COUNTERVALUE_0 {
    pub longValue: i32,
    pub doubleValue: f64,
    pub largeValue: i64,
    pub AnsiStringValue: windows_core::PCSTR,
    pub WideStringValue: windows_core::PCWSTR,
}
impl Default for PDH_FMT_COUNTERVALUE_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PDH_FMT_COUNTERVALUE_ITEM_A {
    pub szName: windows_core::PSTR,
    pub FmtValue: PDH_FMT_COUNTERVALUE,
}
impl Default for PDH_FMT_COUNTERVALUE_ITEM_A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PDH_FMT_COUNTERVALUE_ITEM_W {
    pub szName: windows_core::PWSTR,
    pub FmtValue: PDH_FMT_COUNTERVALUE,
}
impl Default for PDH_FMT_COUNTERVALUE_ITEM_W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const PDH_FMT_DOUBLE: PDH_FMT = PDH_FMT(512u32);
pub const PDH_FMT_LARGE: PDH_FMT = PDH_FMT(1024u32);
pub const PDH_FMT_LONG: PDH_FMT = PDH_FMT(256u32);
pub const PDH_FUNCTION_NOT_FOUND: u32 = 3221228478u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDH_HCOUNTER(pub *mut core::ffi::c_void);
impl PDH_HCOUNTER {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PDH_HCOUNTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDH_HLOG(pub *mut core::ffi::c_void);
impl PDH_HLOG {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl windows_core::Free for PDH_HLOG {
    #[inline]
    unsafe fn free(&mut self) {
        if !self.is_invalid() {
            windows_core::link!("pdh.dll" "system" fn PdhCloseLog(hlog : *mut core::ffi::c_void, dwflags : u32) -> u32);
            unsafe {
                PdhCloseLog(self.0, 0);
            }
        }
    }
}
impl Default for PDH_HLOG {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDH_HQUERY(pub *mut core::ffi::c_void);
impl PDH_HQUERY {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl windows_core::Free for PDH_HQUERY {
    #[inline]
    unsafe fn free(&mut self) {
        if !self.is_invalid() {
            windows_core::link!("pdh.dll" "system" fn PdhCloseQuery(hquery : *mut core::ffi::c_void) -> u32);
            unsafe {
                PdhCloseQuery(self.0);
            }
        }
    }
}
impl Default for PDH_HQUERY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const PDH_INCORRECT_APPEND_TIME: u32 = 3221228539u32;
pub const PDH_INSUFFICIENT_BUFFER: u32 = 3221228482u32;
pub const PDH_INVALID_ARGUMENT: u32 = 3221228477u32;
pub const PDH_INVALID_BUFFER: u32 = 3221228481u32;
pub const PDH_INVALID_DATA: u32 = 3221228486u32;
pub const PDH_INVALID_DATASOURCE: u32 = 3221228509u32;
pub const PDH_INVALID_HANDLE: u32 = 3221228476u32;
pub const PDH_INVALID_INSTANCE: u32 = 3221228485u32;
pub const PDH_INVALID_PATH: u32 = 3221228484u32;
pub const PDH_INVALID_SQLDB: u32 = 3221228510u32;
pub const PDH_INVALID_SQL_LOG_FORMAT: u32 = 3221228533u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PDH_LOG(pub u32);
pub const PDH_LOGSVC_NOT_OPENED: u32 = 3221228505u32;
pub const PDH_LOGSVC_QUERY_NOT_FOUND: u32 = 3221228504u32;
pub const PDH_LOG_FILE_CREATE_ERROR: u32 = 3221228489u32;
pub const PDH_LOG_FILE_OPEN_ERROR: u32 = 3221228490u32;
pub const PDH_LOG_FILE_TOO_SMALL: u32 = 3221228508u32;
pub const PDH_LOG_READ_ACCESS: PDH_LOG = PDH_LOG(65536u32);
pub const PDH_LOG_SAMPLE_TOO_SMALL: u32 = 3221228536u32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PDH_LOG_SERVICE_QUERY_INFO_A {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub dwLogQuota: u32,
    pub szLogFileCaption: windows_core::PSTR,
    pub szDefaultDir: windows_core::PSTR,
    pub szBaseFileName: windows_core::PSTR,
    pub dwFileType: u32,
    pub dwReserved: u32,
    pub Anonymous: PDH_LOG_SERVICE_QUERY_INFO_A_0,
}
impl Default for PDH_LOG_SERVICE_QUERY_INFO_A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PDH_LOG_SERVICE_QUERY_INFO_A_0 {
    pub Anonymous1: PDH_LOG_SERVICE_QUERY_INFO_A_0_0,
    pub Anonymous2: PDH_LOG_SERVICE_QUERY_INFO_A_0_1,
}
impl Default for PDH_LOG_SERVICE_QUERY_INFO_A_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PDH_LOG_SERVICE_QUERY_INFO_A_0_0 {
    pub PdlAutoNameInterval: u32,
    pub PdlAutoNameUnits: u32,
    pub PdlCommandFilename: windows_core::PSTR,
    pub PdlCounterList: windows_core::PSTR,
    pub PdlAutoNameFormat: u32,
    pub PdlSampleInterval: u32,
    pub PdlLogStartTime: super::super::Foundation::FILETIME,
    pub PdlLogEndTime: super::super::Foundation::FILETIME,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PDH_LOG_SERVICE_QUERY_INFO_A_0_1 {
    pub TlNumberOfBuffers: u32,
    pub TlMinimumBuffers: u32,
    pub TlMaximumBuffers: u32,
    pub TlFreeBuffers: u32,
    pub TlBufferSize: u32,
    pub TlEventsLost: u32,
    pub TlLoggerThreadId: u32,
    pub TlBuffersWritten: u32,
    pub TlLogHandle: u32,
    pub TlLogFileName: windows_core::PSTR,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PDH_LOG_SERVICE_QUERY_INFO_W {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub dwLogQuota: u32,
    pub szLogFileCaption: windows_core::PWSTR,
    pub szDefaultDir: windows_core::PWSTR,
    pub szBaseFileName: windows_core::PWSTR,
    pub dwFileType: u32,
    pub dwReserved: u32,
    pub Anonymous: PDH_LOG_SERVICE_QUERY_INFO_W_0,
}
impl Default for PDH_LOG_SERVICE_QUERY_INFO_W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PDH_LOG_SERVICE_QUERY_INFO_W_0 {
    pub Anonymous1: PDH_LOG_SERVICE_QUERY_INFO_W_0_0,
    pub Anonymous2: PDH_LOG_SERVICE_QUERY_INFO_W_0_1,
}
impl Default for PDH_LOG_SERVICE_QUERY_INFO_W_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PDH_LOG_SERVICE_QUERY_INFO_W_0_0 {
    pub PdlAutoNameInterval: u32,
    pub PdlAutoNameUnits: u32,
    pub PdlCommandFilename: windows_core::PWSTR,
    pub PdlCounterList: windows_core::PWSTR,
    pub PdlAutoNameFormat: u32,
    pub PdlSampleInterval: u32,
    pub PdlLogStartTime: super::super::Foundation::FILETIME,
    pub PdlLogEndTime: super::super::Foundation::FILETIME,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PDH_LOG_SERVICE_QUERY_INFO_W_0_1 {
    pub TlNumberOfBuffers: u32,
    pub TlMinimumBuffers: u32,
    pub TlMaximumBuffers: u32,
    pub TlFreeBuffers: u32,
    pub TlBufferSize: u32,
    pub TlEventsLost: u32,
    pub TlLoggerThreadId: u32,
    pub TlBuffersWritten: u32,
    pub TlLogHandle: u32,
    pub TlLogFileName: windows_core::PWSTR,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PDH_LOG_TYPE(pub u32);
pub const PDH_LOG_TYPE_BINARY: PDH_LOG_TYPE = PDH_LOG_TYPE(8u32);
pub const PDH_LOG_TYPE_CSV: PDH_LOG_TYPE = PDH_LOG_TYPE(1u32);
pub const PDH_LOG_TYPE_NOT_FOUND: u32 = 3221228491u32;
pub const PDH_LOG_TYPE_PERFMON: PDH_LOG_TYPE = PDH_LOG_TYPE(6u32);
pub const PDH_LOG_TYPE_RETIRED_BIN: u32 = 3u32;
pub const PDH_LOG_TYPE_SQL: PDH_LOG_TYPE = PDH_LOG_TYPE(7u32);
pub const PDH_LOG_TYPE_TRACE_GENERIC: u32 = 5u32;
pub const PDH_LOG_TYPE_TRACE_KERNEL: u32 = 4u32;
pub const PDH_LOG_TYPE_TSV: PDH_LOG_TYPE = PDH_LOG_TYPE(2u32);
pub const PDH_LOG_TYPE_UNDEFINED: PDH_LOG_TYPE = PDH_LOG_TYPE(0u32);
pub const PDH_LOG_UPDATE_ACCESS: PDH_LOG = PDH_LOG(262144u32);
pub const PDH_LOG_WRITE_ACCESS: PDH_LOG = PDH_LOG(131072u32);
pub const PDH_MAX_COUNTER_NAME: u32 = 1024u32;
pub const PDH_MAX_COUNTER_PATH: u32 = 2048u32;
pub const PDH_MAX_DATASOURCE_PATH: u32 = 1024u32;
pub const PDH_MAX_INSTANCE_NAME: u32 = 1024u32;
pub const PDH_MAX_SCALE: i32 = 7i32;
pub const PDH_MEMORY_ALLOCATION_FAILURE: u32 = 3221228475u32;
pub const PDH_MIN_SCALE: i32 = -7i32;
pub const PDH_MORE_DATA: u32 = 2147485650u32;
pub const PDH_NOEXPANDCOUNTERS: u32 = 1u32;
pub const PDH_NOEXPANDINSTANCES: u32 = 2u32;
pub const PDH_NOT_IMPLEMENTED: u32 = 3221228499u32;
pub const PDH_NO_COUNTERS: u32 = 3221228511u32;
pub const PDH_NO_DATA: u32 = 2147485653u32;
pub const PDH_NO_DIALOG_DATA: u32 = 3221228487u32;
pub const PDH_NO_MORE_DATA: u32 = 3221228492u32;
pub const PDH_OS_EARLIER_VERSION: u32 = 3221228538u32;
pub const PDH_OS_LATER_VERSION: u32 = 3221228537u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PDH_PATH_FLAGS(pub u32);
pub const PDH_PATH_WBEM_INPUT: PDH_PATH_FLAGS = PDH_PATH_FLAGS(2u32);
pub const PDH_PATH_WBEM_NONE: PDH_PATH_FLAGS = PDH_PATH_FLAGS(0u32);
pub const PDH_PATH_WBEM_RESULT: PDH_PATH_FLAGS = PDH_PATH_FLAGS(1u32);
pub const PDH_PLA_COLLECTION_ALREADY_RUNNING: u32 = 3221228521u32;
pub const PDH_PLA_COLLECTION_NOT_FOUND: u32 = 3221228523u32;
pub const PDH_PLA_ERROR_ALREADY_EXISTS: u32 = 3221228526u32;
pub const PDH_PLA_ERROR_FILEPATH: u32 = 3221228528u32;
pub const PDH_PLA_ERROR_NAME_TOO_LONG: u32 = 3221228532u32;
pub const PDH_PLA_ERROR_NOSTART: u32 = 3221228525u32;
pub const PDH_PLA_ERROR_SCHEDULE_ELAPSED: u32 = 3221228524u32;
pub const PDH_PLA_ERROR_SCHEDULE_OVERLAP: u32 = 3221228522u32;
pub const PDH_PLA_ERROR_TYPE_MISMATCH: u32 = 3221228527u32;
pub const PDH_PLA_SERVICE_ERROR: u32 = 3221228529u32;
pub const PDH_PLA_VALIDATION_ERROR: u32 = 3221228530u32;
pub const PDH_PLA_VALIDATION_WARNING: u32 = 2147486707u32;
pub const PDH_QUERY_PERF_DATA_TIMEOUT: u32 = 3221228542u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PDH_RAW_COUNTER {
    pub CStatus: u32,
    pub TimeStamp: super::super::Foundation::FILETIME,
    pub FirstValue: i64,
    pub SecondValue: i64,
    pub MultiCount: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PDH_RAW_COUNTER_ITEM_A {
    pub szName: windows_core::PSTR,
    pub RawValue: PDH_RAW_COUNTER,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PDH_RAW_COUNTER_ITEM_W {
    pub szName: windows_core::PWSTR,
    pub RawValue: PDH_RAW_COUNTER,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PDH_RAW_LOG_RECORD {
    pub dwStructureSize: u32,
    pub dwRecordType: PDH_LOG_TYPE,
    pub dwItems: u32,
    pub RawBytes: [u8; 1],
}
impl Default for PDH_RAW_LOG_RECORD {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const PDH_REFRESHCOUNTERS: u32 = 4u32;
pub const PDH_RETRY: u32 = 2147485652u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PDH_SELECT_DATA_SOURCE_FLAGS(pub u32);
pub const PDH_SQL_ALLOCCON_FAILED: u32 = 3221228513u32;
pub const PDH_SQL_ALLOC_FAILED: u32 = 3221228512u32;
pub const PDH_SQL_ALTER_DETAIL_FAILED: u32 = 3221228541u32;
pub const PDH_SQL_BIND_FAILED: u32 = 3221228519u32;
pub const PDH_SQL_CONNECT_FAILED: u32 = 3221228518u32;
pub const PDH_SQL_EXEC_DIRECT_FAILED: u32 = 3221228514u32;
pub const PDH_SQL_FETCH_FAILED: u32 = 3221228515u32;
pub const PDH_SQL_MORE_RESULTS_FAILED: u32 = 3221228517u32;
pub const PDH_SQL_ROWCOUNT_FAILED: u32 = 3221228516u32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PDH_STATISTICS {
    pub dwFormat: u32,
    pub count: u32,
    pub min: PDH_FMT_COUNTERVALUE,
    pub max: PDH_FMT_COUNTERVALUE,
    pub mean: PDH_FMT_COUNTERVALUE,
}
impl Default for PDH_STATISTICS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const PDH_STRING_NOT_FOUND: u32 = 3221228500u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PDH_TIME_INFO {
    pub StartTime: i64,
    pub EndTime: i64,
    pub SampleCount: u32,
}
pub const PDH_UNABLE_MAP_NAME_FILES: u32 = 2147486677u32;
pub const PDH_UNABLE_READ_LOG_HEADER: u32 = 3221228496u32;
pub const PDH_UNKNOWN_LOGSVC_COMMAND: u32 = 3221228503u32;
pub const PDH_UNKNOWN_LOG_FORMAT: u32 = 3221228502u32;
pub const PDH_UNMATCHED_APPEND_COUNTER: u32 = 3221228540u32;
pub const PDH_VERSION: PDH_DLL_VERSION = PDH_DLL_VERSION(1283u32);
pub const PDH_WBEM_ERROR: u32 = 3221228506u32;
pub type PERFLIBREQUEST = Option<unsafe extern "system" fn(requestcode: u32, buffer: *mut core::ffi::c_void, buffersize: u32) -> u32>;
pub const PERF_ADD_COUNTER: u32 = 1u32;
pub const PERF_AGGREGATE_AVG: PERF_COUNTER_AGGREGATE_FUNC = PERF_COUNTER_AGGREGATE_FUNC(2u32);
pub const PERF_AGGREGATE_INSTANCE: windows_core::PCWSTR = windows_core::w!("_Total");
pub const PERF_AGGREGATE_MAX: PERF_COUNTER_AGGREGATE_FUNC = PERF_COUNTER_AGGREGATE_FUNC(4u32);
pub const PERF_AGGREGATE_MIN: PERF_COUNTER_AGGREGATE_FUNC = PERF_COUNTER_AGGREGATE_FUNC(3u32);
pub const PERF_AGGREGATE_TOTAL: PERF_COUNTER_AGGREGATE_FUNC = PERF_COUNTER_AGGREGATE_FUNC(1u32);
pub const PERF_AGGREGATE_UNDEFINED: PERF_COUNTER_AGGREGATE_FUNC = PERF_COUNTER_AGGREGATE_FUNC(0u32);
pub const PERF_ATTRIB_BY_REFERENCE: u64 = 1u64;
pub const PERF_ATTRIB_DISPLAY_AS_HEX: u64 = 16u64;
pub const PERF_ATTRIB_DISPLAY_AS_REAL: u64 = 8u64;
pub const PERF_ATTRIB_NO_DISPLAYABLE: u64 = 2u64;
pub const PERF_ATTRIB_NO_GROUP_SEPARATOR: u64 = 4u64;
pub const PERF_COLLECT_END: u32 = 6u32;
pub const PERF_COLLECT_START: u32 = 5u32;
pub const PERF_COUNTERSET: PerfCounterDataType = PerfCounterDataType(6i32);
pub const PERF_COUNTERSET_FLAG_AGGREGATE: u32 = 4u32;
pub const PERF_COUNTERSET_FLAG_HISTORY: u32 = 8u32;
pub const PERF_COUNTERSET_FLAG_INSTANCE: u32 = 16u32;
pub const PERF_COUNTERSET_FLAG_MULTIPLE: u32 = 2u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PERF_COUNTERSET_INFO {
    pub CounterSetGuid: windows_core::GUID,
    pub ProviderGuid: windows_core::GUID,
    pub NumCounters: u32,
    pub InstanceType: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PERF_COUNTERSET_INSTANCE {
    pub CounterSetGuid: windows_core::GUID,
    pub dwSize: u32,
    pub InstanceId: u32,
    pub InstanceNameOffset: u32,
    pub InstanceNameSize: u32,
}
pub const PERF_COUNTERSET_MULTI_INSTANCES: u32 = 2u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PERF_COUNTERSET_REG_INFO {
    pub CounterSetGuid: windows_core::GUID,
    pub CounterSetType: u32,
    pub DetailLevel: u32,
    pub NumCounters: u32,
    pub InstanceType: u32,
}
pub const PERF_COUNTERSET_SINGLE_AGGREGATE: u32 = 4u32;
pub const PERF_COUNTERSET_SINGLE_INSTANCE: u32 = 0u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PERF_COUNTER_AGGREGATE_FUNC(pub u32);
pub const PERF_COUNTER_BASE: u32 = 196608u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PERF_COUNTER_BLOCK {
    pub ByteLength: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PERF_COUNTER_DATA {
    pub dwDataSize: u32,
    pub dwSize: u32,
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PERF_COUNTER_DEFINITION {
    pub ByteLength: u32,
    pub CounterNameTitleIndex: u32,
    pub CounterNameTitle: windows_core::PWSTR,
    pub CounterHelpTitleIndex: u32,
    pub CounterHelpTitle: windows_core::PWSTR,
    pub DefaultScale: i32,
    pub DetailLevel: u32,
    pub CounterType: u32,
    pub CounterSize: u32,
    pub CounterOffset: u32,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PERF_COUNTER_DEFINITION {
    pub ByteLength: u32,
    pub CounterNameTitleIndex: u32,
    pub CounterNameTitle: u32,
    pub CounterHelpTitleIndex: u32,
    pub CounterHelpTitle: u32,
    pub DefaultScale: i32,
    pub DetailLevel: u32,
    pub CounterType: u32,
    pub CounterSize: u32,
    pub CounterOffset: u32,
}
pub const PERF_COUNTER_ELAPSED: u32 = 262144u32;
pub const PERF_COUNTER_FRACTION: u32 = 131072u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PERF_COUNTER_HEADER {
    pub dwStatus: u32,
    pub dwType: PerfCounterDataType,
    pub dwSize: u32,
    pub Reserved: u32,
}
pub const PERF_COUNTER_HISTOGRAM: u32 = 393216u32;
pub const PERF_COUNTER_HISTOGRAM_TYPE: u32 = 2147483648u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PERF_COUNTER_IDENTIFIER {
    pub CounterSetGuid: windows_core::GUID,
    pub Status: u32,
    pub Size: u32,
    pub CounterId: u32,
    pub InstanceId: u32,
    pub Index: u32,
    pub Reserved: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PERF_COUNTER_IDENTITY {
    pub CounterSetGuid: windows_core::GUID,
    pub BufferSize: u32,
    pub CounterId: u32,
    pub InstanceId: u32,
    pub MachineOffset: u32,
    pub NameOffset: u32,
    pub Reserved: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PERF_COUNTER_INFO {
    pub CounterId: u32,
    pub Type: u32,
    pub Attrib: u64,
    pub Size: u32,
    pub DetailLevel: u32,
    pub Scale: i32,
    pub Offset: u32,
}
pub const PERF_COUNTER_PRECISION: u32 = 458752u32;
pub const PERF_COUNTER_QUEUELEN: u32 = 327680u32;
pub const PERF_COUNTER_RATE: u32 = 65536u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PERF_COUNTER_REG_INFO {
    pub CounterId: u32,
    pub Type: u32,
    pub Attrib: u64,
    pub DetailLevel: u32,
    pub DefaultScale: i32,
    pub BaseCounterId: u32,
    pub PerfTimeId: u32,
    pub PerfFreqId: u32,
    pub MultiId: u32,
    pub AggregateFunc: PERF_COUNTER_AGGREGATE_FUNC,
    pub Reserved: u32,
}
pub const PERF_COUNTER_VALUE: u32 = 0u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PERF_DATA_BLOCK {
    pub Signature: [u16; 4],
    pub LittleEndian: u32,
    pub Version: u32,
    pub Revision: u32,
    pub TotalByteLength: u32,
    pub HeaderLength: u32,
    pub NumObjectTypes: u32,
    pub DefaultObject: i32,
    pub SystemTime: super::super::Foundation::SYSTEMTIME,
    pub PerfTime: i64,
    pub PerfFreq: i64,
    pub PerfTime100nSec: i64,
    pub SystemNameLength: u32,
    pub SystemNameOffset: u32,
}
impl Default for PERF_DATA_BLOCK {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PERF_DATA_HEADER {
    pub dwTotalSize: u32,
    pub dwNumCounters: u32,
    pub PerfTimeStamp: i64,
    pub PerfTime100NSec: i64,
    pub PerfFreq: i64,
    pub SystemTime: super::super::Foundation::SYSTEMTIME,
}
pub const PERF_DATA_REVISION: u32 = 1u32;
pub const PERF_DATA_VERSION: u32 = 1u32;
pub const PERF_DELTA_BASE: u32 = 8388608u32;
pub const PERF_DELTA_COUNTER: u32 = 4194304u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PERF_DETAIL(pub u32);
pub const PERF_DETAIL_ADVANCED: PERF_DETAIL = PERF_DETAIL(200u32);
pub const PERF_DETAIL_EXPERT: PERF_DETAIL = PERF_DETAIL(300u32);
pub const PERF_DETAIL_NOVICE: PERF_DETAIL = PERF_DETAIL(100u32);
pub const PERF_DETAIL_WIZARD: PERF_DETAIL = PERF_DETAIL(400u32);
pub const PERF_DISPLAY_NOSHOW: u32 = 1073741824u32;
pub const PERF_DISPLAY_NO_SUFFIX: u32 = 0u32;
pub const PERF_DISPLAY_PERCENT: u32 = 536870912u32;
pub const PERF_DISPLAY_PER_SEC: u32 = 268435456u32;
pub const PERF_DISPLAY_SECONDS: u32 = 805306368u32;
pub const PERF_ENUM_INSTANCES: u32 = 3u32;
pub const PERF_ERROR_RETURN: PerfCounterDataType = PerfCounterDataType(0i32);
pub const PERF_FILTER: u32 = 9u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PERF_INSTANCE_DEFINITION {
    pub ByteLength: u32,
    pub ParentObjectTitleIndex: u32,
    pub ParentObjectInstance: u32,
    pub UniqueID: i32,
    pub NameOffset: u32,
    pub NameLength: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PERF_INSTANCE_HEADER {
    pub Size: u32,
    pub InstanceId: u32,
}
pub const PERF_INVERSE_COUNTER: u32 = 16777216u32;
pub const PERF_MAX_INSTANCE_NAME: u32 = 1024u32;
pub type PERF_MEM_ALLOC = Option<unsafe extern "system" fn(allocsize: usize, pcontext: *mut core::ffi::c_void) -> *mut core::ffi::c_void>;
pub type PERF_MEM_FREE = Option<unsafe extern "system" fn(pbuffer: *mut core::ffi::c_void, pcontext: *mut core::ffi::c_void)>;
pub const PERF_METADATA_MULTIPLE_INSTANCES: i32 = -2i32;
pub const PERF_METADATA_NO_INSTANCES: i32 = -3i32;
pub const PERF_MULTIPLE_COUNTERS: PerfCounterDataType = PerfCounterDataType(2i32);
pub const PERF_MULTIPLE_INSTANCES: PerfCounterDataType = PerfCounterDataType(4i32);
pub const PERF_MULTI_COUNTER: u32 = 33554432u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PERF_MULTI_COUNTERS {
    pub dwSize: u32,
    pub dwCounters: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PERF_MULTI_INSTANCES {
    pub dwTotalSize: u32,
    pub dwInstances: u32,
}
pub const PERF_NO_INSTANCES: i32 = -1i32;
pub const PERF_NO_UNIQUE_ID: i32 = -1i32;
pub const PERF_NUMBER_DECIMAL: u32 = 65536u32;
pub const PERF_NUMBER_DEC_1000: u32 = 131072u32;
pub const PERF_NUMBER_HEX: u32 = 0u32;
pub const PERF_OBJECT_TIMER: u32 = 2097152u32;
#[repr(C)]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PERF_OBJECT_TYPE {
    pub TotalByteLength: u32,
    pub DefinitionLength: u32,
    pub HeaderLength: u32,
    pub ObjectNameTitleIndex: u32,
    pub ObjectNameTitle: windows_core::PWSTR,
    pub ObjectHelpTitleIndex: u32,
    pub ObjectHelpTitle: windows_core::PWSTR,
    pub DetailLevel: u32,
    pub NumCounters: u32,
    pub DefaultCounter: i32,
    pub NumInstances: i32,
    pub CodePage: u32,
    pub PerfTime: i64,
    pub PerfFreq: i64,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PERF_OBJECT_TYPE {
    pub TotalByteLength: u32,
    pub DefinitionLength: u32,
    pub HeaderLength: u32,
    pub ObjectNameTitleIndex: u32,
    pub ObjectNameTitle: u32,
    pub ObjectHelpTitleIndex: u32,
    pub ObjectHelpTitle: u32,
    pub DetailLevel: u32,
    pub NumCounters: u32,
    pub DefaultCounter: i32,
    pub NumInstances: i32,
    pub CodePage: u32,
    pub PerfTime: i64,
    pub PerfFreq: i64,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct PERF_PROVIDER_CONTEXT {
    pub ContextSize: u32,
    pub Reserved: u32,
    pub ControlCallback: PERFLIBREQUEST,
    pub MemAllocRoutine: PERF_MEM_ALLOC,
    pub MemFreeRoutine: PERF_MEM_FREE,
    pub pMemContext: *mut core::ffi::c_void,
}
impl Default for PERF_PROVIDER_CONTEXT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const PERF_PROVIDER_DRIVER: u32 = 2u32;
pub const PERF_PROVIDER_KERNEL_MODE: u32 = 1u32;
pub const PERF_PROVIDER_USER_MODE: u32 = 0u32;
pub const PERF_REG_COUNTERSET_ENGLISH_NAME: PerfRegInfoType = PerfRegInfoType(9i32);
pub const PERF_REG_COUNTERSET_HELP_STRING: PerfRegInfoType = PerfRegInfoType(4i32);
pub const PERF_REG_COUNTERSET_NAME_STRING: PerfRegInfoType = PerfRegInfoType(3i32);
pub const PERF_REG_COUNTERSET_STRUCT: PerfRegInfoType = PerfRegInfoType(1i32);
pub const PERF_REG_COUNTER_ENGLISH_NAMES: PerfRegInfoType = PerfRegInfoType(10i32);
pub const PERF_REG_COUNTER_HELP_STRINGS: PerfRegInfoType = PerfRegInfoType(6i32);
pub const PERF_REG_COUNTER_NAME_STRINGS: PerfRegInfoType = PerfRegInfoType(5i32);
pub const PERF_REG_COUNTER_STRUCT: PerfRegInfoType = PerfRegInfoType(2i32);
pub const PERF_REG_PROVIDER_GUID: PerfRegInfoType = PerfRegInfoType(8i32);
pub const PERF_REG_PROVIDER_NAME: PerfRegInfoType = PerfRegInfoType(7i32);
pub const PERF_REMOVE_COUNTER: u32 = 2u32;
pub const PERF_SINGLE_COUNTER: PerfCounterDataType = PerfCounterDataType(1i32);
pub const PERF_SIZE_DWORD: u32 = 0u32;
pub const PERF_SIZE_LARGE: u32 = 256u32;
pub const PERF_SIZE_VARIABLE_LEN: u32 = 768u32;
pub const PERF_SIZE_ZERO: u32 = 512u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PERF_STRING_BUFFER_HEADER {
    pub dwSize: u32,
    pub dwCounters: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PERF_STRING_COUNTER_HEADER {
    pub dwCounterId: u32,
    pub dwOffset: u32,
}
pub const PERF_TEXT_ASCII: u32 = 65536u32;
pub const PERF_TEXT_UNICODE: u32 = 0u32;
pub const PERF_TIMER_100NS: u32 = 1048576u32;
pub const PERF_TIMER_TICK: u32 = 0u32;
pub const PERF_TYPE_COUNTER: u32 = 1024u32;
pub const PERF_TYPE_NUMBER: u32 = 0u32;
pub const PERF_TYPE_TEXT: u32 = 2048u32;
pub const PERF_TYPE_ZERO: u32 = 3072u32;
pub const PERF_WILDCARD_COUNTER: u32 = 4294967295u32;
pub const PERF_WILDCARD_INSTANCE: windows_core::PCWSTR = windows_core::w!("*");
pub const PLAL_ALERT_CMD_LINE_A_NAME: u32 = 512u32;
pub const PLAL_ALERT_CMD_LINE_C_NAME: u32 = 1024u32;
pub const PLAL_ALERT_CMD_LINE_D_TIME: u32 = 2048u32;
pub const PLAL_ALERT_CMD_LINE_L_VAL: u32 = 4096u32;
pub const PLAL_ALERT_CMD_LINE_MASK: u32 = 32512u32;
pub const PLAL_ALERT_CMD_LINE_M_VAL: u32 = 8192u32;
pub const PLAL_ALERT_CMD_LINE_SINGLE: u32 = 256u32;
pub const PLAL_ALERT_CMD_LINE_U_TEXT: u32 = 16384u32;
pub type PLA_CABEXTRACT_CALLBACK = Option<unsafe extern "system" fn(filename: windows_core::PCWSTR, context: *mut core::ffi::c_void)>;
pub const PLA_CAPABILITY_AUTOLOGGER: u32 = 32u32;
pub const PLA_CAPABILITY_LEGACY_SESSION: u32 = 8u32;
pub const PLA_CAPABILITY_LEGACY_SVC: u32 = 16u32;
pub const PLA_CAPABILITY_LOCAL: u32 = 268435456u32;
pub const PLA_CAPABILITY_V1_SESSION: u32 = 2u32;
pub const PLA_CAPABILITY_V1_SVC: u32 = 1u32;
pub const PLA_CAPABILITY_V1_SYSTEM: u32 = 4u32;
pub type PM_CLOSE_PROC = Option<unsafe extern "system" fn() -> u32>;
pub type PM_COLLECT_PROC = Option<unsafe extern "system" fn(pvaluename: windows_core::PCWSTR, ppdata: *mut *mut core::ffi::c_void, pcbtotalbytes: *mut u32, pnumobjecttypes: *mut u32) -> u32>;
pub type PM_OPEN_PROC = Option<unsafe extern "system" fn(pcontext: windows_core::PCWSTR) -> u32>;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PerfCounterDataType(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PerfRegInfoType(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct REAL_TIME_DATA_SOURCE_ID_FLAGS(pub u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ReportValueTypeConstants(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ResourcePolicy(pub i32);
pub const S_PDH: windows_core::GUID = windows_core::GUID::from_u128(0x04d66358_c4a1_419b_8023_23b73902de2c);
pub const ServerDataCollectorSet: windows_core::GUID = windows_core::GUID::from_u128(0x03837531_098b_11d8_9414_505054503030);
pub const ServerDataCollectorSetCollection: windows_core::GUID = windows_core::GUID::from_u128(0x03837532_098b_11d8_9414_505054503030);
pub const SourcePropPage: windows_core::GUID = windows_core::GUID::from_u128(0x0cf32aa1_7571_11d0_93c4_00aa00a3ddea);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct StreamMode(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SysmonBatchReason(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SysmonDataType(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SysmonFileType(pub i32);
pub const SystemDataCollectorSet: windows_core::GUID = windows_core::GUID::from_u128(0x03837546_098b_11d8_9414_505054503030);
pub const SystemDataCollectorSetCollection: windows_core::GUID = windows_core::GUID::from_u128(0x03837547_098b_11d8_9414_505054503030);
pub const SystemMonitor: windows_core::GUID = windows_core::GUID::from_u128(0xc4d2d8e0_d1dd_11ce_940f_008029004347);
pub const SystemMonitor2: windows_core::GUID = windows_core::GUID::from_u128(0x7f30578c_5f38_4612_acfe_6ed04c7b7af8);
pub const TraceDataProvider: windows_core::GUID = windows_core::GUID::from_u128(0x03837513_098b_11d8_9414_505054503030);
pub const TraceDataProviderCollection: windows_core::GUID = windows_core::GUID::from_u128(0x03837511_098b_11d8_9414_505054503030);
pub const TraceSession: windows_core::GUID = windows_core::GUID::from_u128(0x0383751c_098b_11d8_9414_505054503030);
pub const TraceSessionCollection: windows_core::GUID = windows_core::GUID::from_u128(0x03837530_098b_11d8_9414_505054503030);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ValueMapType(pub i32);
pub const WINPERF_LOG_DEBUG: u32 = 2u32;
pub const WINPERF_LOG_NONE: u32 = 0u32;
pub const WINPERF_LOG_USER: u32 = 1u32;
pub const WINPERF_LOG_VERBOSE: u32 = 3u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WeekDays(pub i32);
windows_core::imp::define_interface!(_ICounterItemUnion, _ICounterItemUnion_Vtbl, 0xde1a6b74_9182_4c41_8e2c_24c2cd30ee83);
windows_core::imp::interface_hierarchy!(_ICounterItemUnion, windows_core::IUnknown);
impl _ICounterItemUnion {
    pub unsafe fn Value(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Value)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetColor(&self, color: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetColor)(windows_core::Interface::as_raw(self), color).ok() }
    }
    pub unsafe fn Color(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Color)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetWidth(&self, iwidth: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetWidth)(windows_core::Interface::as_raw(self), iwidth).ok() }
    }
    pub unsafe fn Width(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Width)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetLineStyle(&self, ilinestyle: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetLineStyle)(windows_core::Interface::as_raw(self), ilinestyle).ok() }
    }
    pub unsafe fn LineStyle(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).LineStyle)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetScaleFactor(&self, iscale: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetScaleFactor)(windows_core::Interface::as_raw(self), iscale).ok() }
    }
    pub unsafe fn ScaleFactor(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ScaleFactor)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Path(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Path)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn GetValue(&self, value: *mut f64, status: *mut i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetValue)(windows_core::Interface::as_raw(self), value as _, status as _).ok() }
    }
    pub unsafe fn GetStatistics(&self, max: *mut f64, min: *mut f64, avg: *mut f64, status: *mut i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetStatistics)(windows_core::Interface::as_raw(self), max as _, min as _, avg as _, status as _).ok() }
    }
    pub unsafe fn SetSelected(&self, bstate: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetSelected)(windows_core::Interface::as_raw(self), bstate).ok() }
    }
    pub unsafe fn Selected(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Selected)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetVisible(&self, bstate: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetVisible)(windows_core::Interface::as_raw(self), bstate).ok() }
    }
    pub unsafe fn Visible(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Visible)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn GetDataAt(&self, iindex: i32, iwhich: SysmonDataType) -> windows_core::Result<super::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDataAt)(windows_core::Interface::as_raw(self), iindex, iwhich, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct _ICounterItemUnion_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Value: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub SetColor: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Color: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetWidth: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub Width: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetLineStyle: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub LineStyle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetScaleFactor: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub ScaleFactor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub Path: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetValue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64, *mut i32) -> windows_core::HRESULT,
    pub GetStatistics: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64, *mut f64, *mut f64, *mut i32) -> windows_core::HRESULT,
    pub SetSelected: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub Selected: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetVisible: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub Visible: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub GetDataAt: unsafe extern "system" fn(*mut core::ffi::c_void, i32, SysmonDataType, *mut super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    GetDataAt: usize,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait _ICounterItemUnion_Impl: windows_core::IUnknownImpl {
    fn Value(&self) -> windows_core::Result<f64>;
    fn SetColor(&self, color: u32) -> windows_core::Result<()>;
    fn Color(&self) -> windows_core::Result<u32>;
    fn SetWidth(&self, iwidth: i32) -> windows_core::Result<()>;
    fn Width(&self) -> windows_core::Result<i32>;
    fn SetLineStyle(&self, ilinestyle: i32) -> windows_core::Result<()>;
    fn LineStyle(&self) -> windows_core::Result<i32>;
    fn SetScaleFactor(&self, iscale: i32) -> windows_core::Result<()>;
    fn ScaleFactor(&self) -> windows_core::Result<i32>;
    fn Path(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetValue(&self, value: *mut f64, status: *mut i32) -> windows_core::Result<()>;
    fn GetStatistics(&self, max: *mut f64, min: *mut f64, avg: *mut f64, status: *mut i32) -> windows_core::Result<()>;
    fn SetSelected(&self, bstate: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn Selected(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetVisible(&self, bstate: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn Visible(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn GetDataAt(&self, iindex: i32, iwhich: SysmonDataType) -> windows_core::Result<super::Variant::VARIANT>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl _ICounterItemUnion_Vtbl {
    pub const fn new<Identity: _ICounterItemUnion_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Value<Identity: _ICounterItemUnion_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdblvalue: *mut f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match _ICounterItemUnion_Impl::Value(this) {
                    Ok(ok__) => {
                        pdblvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetColor<Identity: _ICounterItemUnion_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, color: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                _ICounterItemUnion_Impl::SetColor(this, core::mem::transmute_copy(&color)).into()
            }
        }
        unsafe extern "system" fn Color<Identity: _ICounterItemUnion_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcolor: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match _ICounterItemUnion_Impl::Color(this) {
                    Ok(ok__) => {
                        pcolor.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetWidth<Identity: _ICounterItemUnion_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iwidth: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                _ICounterItemUnion_Impl::SetWidth(this, core::mem::transmute_copy(&iwidth)).into()
            }
        }
        unsafe extern "system" fn Width<Identity: _ICounterItemUnion_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pivalue: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match _ICounterItemUnion_Impl::Width(this) {
                    Ok(ok__) => {
                        pivalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetLineStyle<Identity: _ICounterItemUnion_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ilinestyle: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                _ICounterItemUnion_Impl::SetLineStyle(this, core::mem::transmute_copy(&ilinestyle)).into()
            }
        }
        unsafe extern "system" fn LineStyle<Identity: _ICounterItemUnion_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pivalue: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match _ICounterItemUnion_Impl::LineStyle(this) {
                    Ok(ok__) => {
                        pivalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetScaleFactor<Identity: _ICounterItemUnion_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iscale: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                _ICounterItemUnion_Impl::SetScaleFactor(this, core::mem::transmute_copy(&iscale)).into()
            }
        }
        unsafe extern "system" fn ScaleFactor<Identity: _ICounterItemUnion_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pivalue: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match _ICounterItemUnion_Impl::ScaleFactor(this) {
                    Ok(ok__) => {
                        pivalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Path<Identity: _ICounterItemUnion_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstrvalue: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match _ICounterItemUnion_Impl::Path(this) {
                    Ok(ok__) => {
                        pstrvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetValue<Identity: _ICounterItemUnion_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut f64, status: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                _ICounterItemUnion_Impl::GetValue(this, core::mem::transmute_copy(&value), core::mem::transmute_copy(&status)).into()
            }
        }
        unsafe extern "system" fn GetStatistics<Identity: _ICounterItemUnion_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, max: *mut f64, min: *mut f64, avg: *mut f64, status: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                _ICounterItemUnion_Impl::GetStatistics(this, core::mem::transmute_copy(&max), core::mem::transmute_copy(&min), core::mem::transmute_copy(&avg), core::mem::transmute_copy(&status)).into()
            }
        }
        unsafe extern "system" fn SetSelected<Identity: _ICounterItemUnion_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstate: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                _ICounterItemUnion_Impl::SetSelected(this, core::mem::transmute_copy(&bstate)).into()
            }
        }
        unsafe extern "system" fn Selected<Identity: _ICounterItemUnion_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstate: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match _ICounterItemUnion_Impl::Selected(this) {
                    Ok(ok__) => {
                        pbstate.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetVisible<Identity: _ICounterItemUnion_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstate: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                _ICounterItemUnion_Impl::SetVisible(this, core::mem::transmute_copy(&bstate)).into()
            }
        }
        unsafe extern "system" fn Visible<Identity: _ICounterItemUnion_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstate: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match _ICounterItemUnion_Impl::Visible(this) {
                    Ok(ok__) => {
                        pbstate.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetDataAt<Identity: _ICounterItemUnion_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iindex: i32, iwhich: SysmonDataType, pvariant: *mut super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match _ICounterItemUnion_Impl::GetDataAt(this, core::mem::transmute_copy(&iindex), core::mem::transmute_copy(&iwhich)) {
                    Ok(ok__) => {
                        pvariant.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Value: Value::<Identity, OFFSET>,
            SetColor: SetColor::<Identity, OFFSET>,
            Color: Color::<Identity, OFFSET>,
            SetWidth: SetWidth::<Identity, OFFSET>,
            Width: Width::<Identity, OFFSET>,
            SetLineStyle: SetLineStyle::<Identity, OFFSET>,
            LineStyle: LineStyle::<Identity, OFFSET>,
            SetScaleFactor: SetScaleFactor::<Identity, OFFSET>,
            ScaleFactor: ScaleFactor::<Identity, OFFSET>,
            Path: Path::<Identity, OFFSET>,
            GetValue: GetValue::<Identity, OFFSET>,
            GetStatistics: GetStatistics::<Identity, OFFSET>,
            SetSelected: SetSelected::<Identity, OFFSET>,
            Selected: Selected::<Identity, OFFSET>,
            SetVisible: SetVisible::<Identity, OFFSET>,
            Visible: Visible::<Identity, OFFSET>,
            GetDataAt: GetDataAt::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<_ICounterItemUnion as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for _ICounterItemUnion {}
windows_core::imp::define_interface!(_ISystemMonitorUnion, _ISystemMonitorUnion_Vtbl, 0xc8a77338_265f_4de5_aa25_c7da1ce5a8f4);
windows_core::imp::interface_hierarchy!(_ISystemMonitorUnion, windows_core::IUnknown);
impl _ISystemMonitorUnion {
    pub unsafe fn Appearance(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Appearance)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetAppearance(&self, iappearance: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetAppearance)(windows_core::Interface::as_raw(self), iappearance).ok() }
    }
    pub unsafe fn BackColor(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).BackColor)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetBackColor(&self, color: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetBackColor)(windows_core::Interface::as_raw(self), color).ok() }
    }
    pub unsafe fn BorderStyle(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).BorderStyle)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetBorderStyle(&self, iborderstyle: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetBorderStyle)(windows_core::Interface::as_raw(self), iborderstyle).ok() }
    }
    pub unsafe fn ForeColor(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ForeColor)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetForeColor(&self, color: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetForeColor)(windows_core::Interface::as_raw(self), color).ok() }
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Font(&self) -> windows_core::Result<super::Ole::IFontDisp> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Font)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn putref_Font<P0>(&self, pfont: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Ole::IFontDisp>,
    {
        unsafe { (windows_core::Interface::vtable(self).putref_Font)(windows_core::Interface::as_raw(self), pfont.param().abi()).ok() }
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Counters(&self) -> windows_core::Result<ICounters> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Counters)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetShowVerticalGrid(&self, bstate: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetShowVerticalGrid)(windows_core::Interface::as_raw(self), bstate).ok() }
    }
    pub unsafe fn ShowVerticalGrid(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ShowVerticalGrid)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetShowHorizontalGrid(&self, bstate: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetShowHorizontalGrid)(windows_core::Interface::as_raw(self), bstate).ok() }
    }
    pub unsafe fn ShowHorizontalGrid(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ShowHorizontalGrid)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetShowLegend(&self, bstate: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetShowLegend)(windows_core::Interface::as_raw(self), bstate).ok() }
    }
    pub unsafe fn ShowLegend(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ShowLegend)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetShowScaleLabels(&self, bstate: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetShowScaleLabels)(windows_core::Interface::as_raw(self), bstate).ok() }
    }
    pub unsafe fn ShowScaleLabels(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ShowScaleLabels)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetShowValueBar(&self, bstate: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetShowValueBar)(windows_core::Interface::as_raw(self), bstate).ok() }
    }
    pub unsafe fn ShowValueBar(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ShowValueBar)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetMaximumScale(&self, ivalue: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetMaximumScale)(windows_core::Interface::as_raw(self), ivalue).ok() }
    }
    pub unsafe fn MaximumScale(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MaximumScale)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetMinimumScale(&self, ivalue: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetMinimumScale)(windows_core::Interface::as_raw(self), ivalue).ok() }
    }
    pub unsafe fn MinimumScale(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MinimumScale)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetUpdateInterval(&self, fvalue: f32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetUpdateInterval)(windows_core::Interface::as_raw(self), fvalue).ok() }
    }
    pub unsafe fn UpdateInterval(&self) -> windows_core::Result<f32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).UpdateInterval)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetDisplayType(&self, edisplaytype: DisplayTypeConstants) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetDisplayType)(windows_core::Interface::as_raw(self), edisplaytype).ok() }
    }
    pub unsafe fn DisplayType(&self) -> windows_core::Result<DisplayTypeConstants> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DisplayType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetManualUpdate(&self, bstate: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetManualUpdate)(windows_core::Interface::as_raw(self), bstate).ok() }
    }
    pub unsafe fn ManualUpdate(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ManualUpdate)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetGraphTitle(&self, bstitle: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetGraphTitle)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstitle)).ok() }
    }
    pub unsafe fn GraphTitle(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GraphTitle)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetYAxisLabel(&self, bstitle: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetYAxisLabel)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstitle)).ok() }
    }
    pub unsafe fn YAxisLabel(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).YAxisLabel)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn CollectSample(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).CollectSample)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn UpdateGraph(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).UpdateGraph)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn BrowseCounters(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).BrowseCounters)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn DisplayProperties(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).DisplayProperties)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Counter(&self, iindex: i32) -> windows_core::Result<ICounterItem> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Counter)(windows_core::Interface::as_raw(self), iindex, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn AddCounter(&self, bspath: &windows_core::BSTR) -> windows_core::Result<ICounterItem> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AddCounter)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bspath), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn DeleteCounter<P0>(&self, pctr: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ICounterItem>,
    {
        unsafe { (windows_core::Interface::vtable(self).DeleteCounter)(windows_core::Interface::as_raw(self), pctr.param().abi()).ok() }
    }
    pub unsafe fn BackColorCtl(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).BackColorCtl)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetBackColorCtl(&self, color: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetBackColorCtl)(windows_core::Interface::as_raw(self), color).ok() }
    }
    pub unsafe fn SetLogFileName(&self, bsfilename: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetLogFileName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bsfilename)).ok() }
    }
    pub unsafe fn LogFileName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).LogFileName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetLogViewStart(&self, starttime: f64) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetLogViewStart)(windows_core::Interface::as_raw(self), starttime).ok() }
    }
    pub unsafe fn LogViewStart(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).LogViewStart)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetLogViewStop(&self, stoptime: f64) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetLogViewStop)(windows_core::Interface::as_raw(self), stoptime).ok() }
    }
    pub unsafe fn LogViewStop(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).LogViewStop)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GridColor(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GridColor)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetGridColor(&self, color: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetGridColor)(windows_core::Interface::as_raw(self), color).ok() }
    }
    pub unsafe fn TimeBarColor(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).TimeBarColor)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetTimeBarColor(&self, color: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetTimeBarColor)(windows_core::Interface::as_raw(self), color).ok() }
    }
    pub unsafe fn Highlight(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Highlight)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetHighlight(&self, bstate: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetHighlight)(windows_core::Interface::as_raw(self), bstate).ok() }
    }
    pub unsafe fn ShowToolbar(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ShowToolbar)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetShowToolbar(&self, bstate: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetShowToolbar)(windows_core::Interface::as_raw(self), bstate).ok() }
    }
    pub unsafe fn Paste(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Paste)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Copy(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Copy)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn SetReadOnly(&self, bstate: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetReadOnly)(windows_core::Interface::as_raw(self), bstate).ok() }
    }
    pub unsafe fn ReadOnly(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ReadOnly)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetReportValueType(&self, ereportvaluetype: ReportValueTypeConstants) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetReportValueType)(windows_core::Interface::as_raw(self), ereportvaluetype).ok() }
    }
    pub unsafe fn ReportValueType(&self) -> windows_core::Result<ReportValueTypeConstants> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ReportValueType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetMonitorDuplicateInstances(&self, bstate: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetMonitorDuplicateInstances)(windows_core::Interface::as_raw(self), bstate).ok() }
    }
    pub unsafe fn MonitorDuplicateInstances(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MonitorDuplicateInstances)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetDisplayFilter(&self, ivalue: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetDisplayFilter)(windows_core::Interface::as_raw(self), ivalue).ok() }
    }
    pub unsafe fn DisplayFilter(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DisplayFilter)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn LogFiles(&self) -> windows_core::Result<ILogFiles> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).LogFiles)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetDataSourceType(&self, edatasourcetype: DataSourceTypeConstants) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetDataSourceType)(windows_core::Interface::as_raw(self), edatasourcetype).ok() }
    }
    pub unsafe fn DataSourceType(&self) -> windows_core::Result<DataSourceTypeConstants> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DataSourceType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetSqlDsnName(&self, bssqldsnname: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetSqlDsnName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bssqldsnname)).ok() }
    }
    pub unsafe fn SqlDsnName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SqlDsnName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetSqlLogSetName(&self, bssqllogsetname: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetSqlLogSetName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bssqllogsetname)).ok() }
    }
    pub unsafe fn SqlLogSetName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SqlLogSetName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetEnableDigitGrouping(&self, bstate: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetEnableDigitGrouping)(windows_core::Interface::as_raw(self), bstate).ok() }
    }
    pub unsafe fn EnableDigitGrouping(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnableDigitGrouping)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetEnableToolTips(&self, bstate: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetEnableToolTips)(windows_core::Interface::as_raw(self), bstate).ok() }
    }
    pub unsafe fn EnableToolTips(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnableToolTips)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetShowTimeAxisLabels(&self, bstate: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetShowTimeAxisLabels)(windows_core::Interface::as_raw(self), bstate).ok() }
    }
    pub unsafe fn ShowTimeAxisLabels(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ShowTimeAxisLabels)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetChartScroll(&self, bscroll: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetChartScroll)(windows_core::Interface::as_raw(self), bscroll).ok() }
    }
    pub unsafe fn ChartScroll(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ChartScroll)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetDataPointCount(&self, inewcount: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetDataPointCount)(windows_core::Interface::as_raw(self), inewcount).ok() }
    }
    pub unsafe fn DataPointCount(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DataPointCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn ScaleToFit(&self, bselectedcountersonly: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ScaleToFit)(windows_core::Interface::as_raw(self), bselectedcountersonly).ok() }
    }
    pub unsafe fn SaveAs(&self, bstrfilename: &windows_core::BSTR, esysmonfiletype: SysmonFileType) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SaveAs)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrfilename), esysmonfiletype).ok() }
    }
    pub unsafe fn Relog(&self, bstrfilename: &windows_core::BSTR, esysmonfiletype: SysmonFileType, ifilter: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Relog)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrfilename), esysmonfiletype, ifilter).ok() }
    }
    pub unsafe fn ClearData(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ClearData)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn LogSourceStartTime(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).LogSourceStartTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn LogSourceStopTime(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).LogSourceStopTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetLogViewRange(&self, starttime: f64, stoptime: f64) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetLogViewRange)(windows_core::Interface::as_raw(self), starttime, stoptime).ok() }
    }
    pub unsafe fn GetLogViewRange(&self, starttime: *mut f64, stoptime: *mut f64) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetLogViewRange)(windows_core::Interface::as_raw(self), starttime as _, stoptime as _).ok() }
    }
    pub unsafe fn BatchingLock(&self, flock: super::super::Foundation::VARIANT_BOOL, ebatchreason: SysmonBatchReason) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).BatchingLock)(windows_core::Interface::as_raw(self), flock, ebatchreason).ok() }
    }
    pub unsafe fn LoadSettings(&self, bstrsettingfilename: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).LoadSettings)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrsettingfilename)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct _ISystemMonitorUnion_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Appearance: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetAppearance: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub BackColor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetBackColor: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub BorderStyle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetBorderStyle: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub ForeColor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetForeColor: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Font: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Font: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub putref_Font: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    putref_Font: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Counters: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Counters: usize,
    pub SetShowVerticalGrid: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub ShowVerticalGrid: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetShowHorizontalGrid: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub ShowHorizontalGrid: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetShowLegend: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub ShowLegend: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetShowScaleLabels: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub ShowScaleLabels: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetShowValueBar: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub ShowValueBar: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetMaximumScale: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub MaximumScale: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetMinimumScale: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub MinimumScale: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetUpdateInterval: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub UpdateInterval: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub SetDisplayType: unsafe extern "system" fn(*mut core::ffi::c_void, DisplayTypeConstants) -> windows_core::HRESULT,
    pub DisplayType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DisplayTypeConstants) -> windows_core::HRESULT,
    pub SetManualUpdate: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub ManualUpdate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetGraphTitle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GraphTitle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetYAxisLabel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub YAxisLabel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CollectSample: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub UpdateGraph: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub BrowseCounters: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DisplayProperties: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Counter: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddCounter: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DeleteCounter: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub BackColorCtl: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetBackColorCtl: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub SetLogFileName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub LogFileName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetLogViewStart: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub LogViewStart: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub SetLogViewStop: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub LogViewStop: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub GridColor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetGridColor: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub TimeBarColor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetTimeBarColor: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Highlight: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetHighlight: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub ShowToolbar: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetShowToolbar: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub Paste: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Copy: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetReadOnly: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub ReadOnly: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetReportValueType: unsafe extern "system" fn(*mut core::ffi::c_void, ReportValueTypeConstants) -> windows_core::HRESULT,
    pub ReportValueType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut ReportValueTypeConstants) -> windows_core::HRESULT,
    pub SetMonitorDuplicateInstances: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub MonitorDuplicateInstances: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetDisplayFilter: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub DisplayFilter: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub LogFiles: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    LogFiles: usize,
    pub SetDataSourceType: unsafe extern "system" fn(*mut core::ffi::c_void, DataSourceTypeConstants) -> windows_core::HRESULT,
    pub DataSourceType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DataSourceTypeConstants) -> windows_core::HRESULT,
    pub SetSqlDsnName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SqlDsnName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetSqlLogSetName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SqlLogSetName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetEnableDigitGrouping: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub EnableDigitGrouping: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetEnableToolTips: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub EnableToolTips: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetShowTimeAxisLabels: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub ShowTimeAxisLabels: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetChartScroll: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub ChartScroll: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetDataPointCount: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub DataPointCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub ScaleToFit: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SaveAs: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, SysmonFileType) -> windows_core::HRESULT,
    pub Relog: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, SysmonFileType, i32) -> windows_core::HRESULT,
    pub ClearData: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub LogSourceStartTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub LogSourceStopTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub SetLogViewRange: unsafe extern "system" fn(*mut core::ffi::c_void, f64, f64) -> windows_core::HRESULT,
    pub GetLogViewRange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64, *mut f64) -> windows_core::HRESULT,
    pub BatchingLock: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL, SysmonBatchReason) -> windows_core::HRESULT,
    pub LoadSettings: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait _ISystemMonitorUnion_Impl: windows_core::IUnknownImpl {
    fn Appearance(&self) -> windows_core::Result<i32>;
    fn SetAppearance(&self, iappearance: i32) -> windows_core::Result<()>;
    fn BackColor(&self) -> windows_core::Result<u32>;
    fn SetBackColor(&self, color: u32) -> windows_core::Result<()>;
    fn BorderStyle(&self) -> windows_core::Result<i32>;
    fn SetBorderStyle(&self, iborderstyle: i32) -> windows_core::Result<()>;
    fn ForeColor(&self) -> windows_core::Result<u32>;
    fn SetForeColor(&self, color: u32) -> windows_core::Result<()>;
    fn Font(&self) -> windows_core::Result<super::Ole::IFontDisp>;
    fn putref_Font(&self, pfont: windows_core::Ref<super::Ole::IFontDisp>) -> windows_core::Result<()>;
    fn Counters(&self) -> windows_core::Result<ICounters>;
    fn SetShowVerticalGrid(&self, bstate: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn ShowVerticalGrid(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetShowHorizontalGrid(&self, bstate: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn ShowHorizontalGrid(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetShowLegend(&self, bstate: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn ShowLegend(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetShowScaleLabels(&self, bstate: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn ShowScaleLabels(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetShowValueBar(&self, bstate: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn ShowValueBar(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetMaximumScale(&self, ivalue: i32) -> windows_core::Result<()>;
    fn MaximumScale(&self) -> windows_core::Result<i32>;
    fn SetMinimumScale(&self, ivalue: i32) -> windows_core::Result<()>;
    fn MinimumScale(&self) -> windows_core::Result<i32>;
    fn SetUpdateInterval(&self, fvalue: f32) -> windows_core::Result<()>;
    fn UpdateInterval(&self) -> windows_core::Result<f32>;
    fn SetDisplayType(&self, edisplaytype: DisplayTypeConstants) -> windows_core::Result<()>;
    fn DisplayType(&self) -> windows_core::Result<DisplayTypeConstants>;
    fn SetManualUpdate(&self, bstate: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn ManualUpdate(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetGraphTitle(&self, bstitle: &windows_core::BSTR) -> windows_core::Result<()>;
    fn GraphTitle(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetYAxisLabel(&self, bstitle: &windows_core::BSTR) -> windows_core::Result<()>;
    fn YAxisLabel(&self) -> windows_core::Result<windows_core::BSTR>;
    fn CollectSample(&self) -> windows_core::Result<()>;
    fn UpdateGraph(&self) -> windows_core::Result<()>;
    fn BrowseCounters(&self) -> windows_core::Result<()>;
    fn DisplayProperties(&self) -> windows_core::Result<()>;
    fn Counter(&self, iindex: i32) -> windows_core::Result<ICounterItem>;
    fn AddCounter(&self, bspath: &windows_core::BSTR) -> windows_core::Result<ICounterItem>;
    fn DeleteCounter(&self, pctr: windows_core::Ref<ICounterItem>) -> windows_core::Result<()>;
    fn BackColorCtl(&self) -> windows_core::Result<u32>;
    fn SetBackColorCtl(&self, color: u32) -> windows_core::Result<()>;
    fn SetLogFileName(&self, bsfilename: &windows_core::BSTR) -> windows_core::Result<()>;
    fn LogFileName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetLogViewStart(&self, starttime: f64) -> windows_core::Result<()>;
    fn LogViewStart(&self) -> windows_core::Result<f64>;
    fn SetLogViewStop(&self, stoptime: f64) -> windows_core::Result<()>;
    fn LogViewStop(&self) -> windows_core::Result<f64>;
    fn GridColor(&self) -> windows_core::Result<u32>;
    fn SetGridColor(&self, color: u32) -> windows_core::Result<()>;
    fn TimeBarColor(&self) -> windows_core::Result<u32>;
    fn SetTimeBarColor(&self, color: u32) -> windows_core::Result<()>;
    fn Highlight(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetHighlight(&self, bstate: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn ShowToolbar(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetShowToolbar(&self, bstate: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn Paste(&self) -> windows_core::Result<()>;
    fn Copy(&self) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn SetReadOnly(&self, bstate: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn ReadOnly(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetReportValueType(&self, ereportvaluetype: ReportValueTypeConstants) -> windows_core::Result<()>;
    fn ReportValueType(&self) -> windows_core::Result<ReportValueTypeConstants>;
    fn SetMonitorDuplicateInstances(&self, bstate: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn MonitorDuplicateInstances(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetDisplayFilter(&self, ivalue: i32) -> windows_core::Result<()>;
    fn DisplayFilter(&self) -> windows_core::Result<i32>;
    fn LogFiles(&self) -> windows_core::Result<ILogFiles>;
    fn SetDataSourceType(&self, edatasourcetype: DataSourceTypeConstants) -> windows_core::Result<()>;
    fn DataSourceType(&self) -> windows_core::Result<DataSourceTypeConstants>;
    fn SetSqlDsnName(&self, bssqldsnname: &windows_core::BSTR) -> windows_core::Result<()>;
    fn SqlDsnName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetSqlLogSetName(&self, bssqllogsetname: &windows_core::BSTR) -> windows_core::Result<()>;
    fn SqlLogSetName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetEnableDigitGrouping(&self, bstate: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn EnableDigitGrouping(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetEnableToolTips(&self, bstate: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn EnableToolTips(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetShowTimeAxisLabels(&self, bstate: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn ShowTimeAxisLabels(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetChartScroll(&self, bscroll: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn ChartScroll(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetDataPointCount(&self, inewcount: i32) -> windows_core::Result<()>;
    fn DataPointCount(&self) -> windows_core::Result<i32>;
    fn ScaleToFit(&self, bselectedcountersonly: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn SaveAs(&self, bstrfilename: &windows_core::BSTR, esysmonfiletype: SysmonFileType) -> windows_core::Result<()>;
    fn Relog(&self, bstrfilename: &windows_core::BSTR, esysmonfiletype: SysmonFileType, ifilter: i32) -> windows_core::Result<()>;
    fn ClearData(&self) -> windows_core::Result<()>;
    fn LogSourceStartTime(&self) -> windows_core::Result<f64>;
    fn LogSourceStopTime(&self) -> windows_core::Result<f64>;
    fn SetLogViewRange(&self, starttime: f64, stoptime: f64) -> windows_core::Result<()>;
    fn GetLogViewRange(&self, starttime: *mut f64, stoptime: *mut f64) -> windows_core::Result<()>;
    fn BatchingLock(&self, flock: super::super::Foundation::VARIANT_BOOL, ebatchreason: SysmonBatchReason) -> windows_core::Result<()>;
    fn LoadSettings(&self, bstrsettingfilename: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl _ISystemMonitorUnion_Vtbl {
    pub const fn new<Identity: _ISystemMonitorUnion_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Appearance<Identity: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iappearance: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match _ISystemMonitorUnion_Impl::Appearance(this) {
                    Ok(ok__) => {
                        iappearance.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetAppearance<Identity: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iappearance: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                _ISystemMonitorUnion_Impl::SetAppearance(this, core::mem::transmute_copy(&iappearance)).into()
            }
        }
        unsafe extern "system" fn BackColor<Identity: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcolor: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match _ISystemMonitorUnion_Impl::BackColor(this) {
                    Ok(ok__) => {
                        pcolor.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetBackColor<Identity: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, color: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                _ISystemMonitorUnion_Impl::SetBackColor(this, core::mem::transmute_copy(&color)).into()
            }
        }
        unsafe extern "system" fn BorderStyle<Identity: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iborderstyle: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match _ISystemMonitorUnion_Impl::BorderStyle(this) {
                    Ok(ok__) => {
                        iborderstyle.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetBorderStyle<Identity: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iborderstyle: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                _ISystemMonitorUnion_Impl::SetBorderStyle(this, core::mem::transmute_copy(&iborderstyle)).into()
            }
        }
        unsafe extern "system" fn ForeColor<Identity: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcolor: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match _ISystemMonitorUnion_Impl::ForeColor(this) {
                    Ok(ok__) => {
                        pcolor.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetForeColor<Identity: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, color: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                _ISystemMonitorUnion_Impl::SetForeColor(this, core::mem::transmute_copy(&color)).into()
            }
        }
        unsafe extern "system" fn Font<Identity: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppfont: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match _ISystemMonitorUnion_Impl::Font(this) {
                    Ok(ok__) => {
                        ppfont.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn putref_Font<Identity: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfont: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                _ISystemMonitorUnion_Impl::putref_Font(this, core::mem::transmute_copy(&pfont)).into()
            }
        }
        unsafe extern "system" fn Counters<Identity: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppicounters: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match _ISystemMonitorUnion_Impl::Counters(this) {
                    Ok(ok__) => {
                        ppicounters.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetShowVerticalGrid<Identity: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstate: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                _ISystemMonitorUnion_Impl::SetShowVerticalGrid(this, core::mem::transmute_copy(&bstate)).into()
            }
        }
        unsafe extern "system" fn ShowVerticalGrid<Identity: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstate: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match _ISystemMonitorUnion_Impl::ShowVerticalGrid(this) {
                    Ok(ok__) => {
                        pbstate.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetShowHorizontalGrid<Identity: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstate: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                _ISystemMonitorUnion_Impl::SetShowHorizontalGrid(this, core::mem::transmute_copy(&bstate)).into()
            }
        }
        unsafe extern "system" fn ShowHorizontalGrid<Identity: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstate: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match _ISystemMonitorUnion_Impl::ShowHorizontalGrid(this) {
                    Ok(ok__) => {
                        pbstate.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetShowLegend<Identity: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstate: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                _ISystemMonitorUnion_Impl::SetShowLegend(this, core::mem::transmute_copy(&bstate)).into()
            }
        }
        unsafe extern "system" fn ShowLegend<Identity: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstate: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match _ISystemMonitorUnion_Impl::ShowLegend(this) {
                    Ok(ok__) => {
                        pbstate.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetShowScaleLabels<Identity: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstate: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                _ISystemMonitorUnion_Impl::SetShowScaleLabels(this, core::mem::transmute_copy(&bstate)).into()
            }
        }
        unsafe extern "system" fn ShowScaleLabels<Identity: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstate: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match _ISystemMonitorUnion_Impl::ShowScaleLabels(this) {
                    Ok(ok__) => {
                        pbstate.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetShowValueBar<Identity: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstate: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                _ISystemMonitorUnion_Impl::SetShowValueBar(this, core::mem::transmute_copy(&bstate)).into()
            }
        }
        unsafe extern "system" fn ShowValueBar<Identity: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstate: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match _ISystemMonitorUnion_Impl::ShowValueBar(this) {
                    Ok(ok__) => {
                        pbstate.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetMaximumScale<Identity: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ivalue: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                _ISystemMonitorUnion_Impl::SetMaximumScale(this, core::mem::transmute_copy(&ivalue)).into()
            }
        }
        unsafe extern "system" fn MaximumScale<Identity: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pivalue: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match _ISystemMonitorUnion_Impl::MaximumScale(this) {
                    Ok(ok__) => {
                        pivalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetMinimumScale<Identity: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ivalue: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                _ISystemMonitorUnion_Impl::SetMinimumScale(this, core::mem::transmute_copy(&ivalue)).into()
            }
        }
        unsafe extern "system" fn MinimumScale<Identity: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pivalue: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match _ISystemMonitorUnion_Impl::MinimumScale(this) {
                    Ok(ok__) => {
                        pivalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetUpdateInterval<Identity: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fvalue: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                _ISystemMonitorUnion_Impl::SetUpdateInterval(this, core::mem::transmute_copy(&fvalue)).into()
            }
        }
        unsafe extern "system" fn UpdateInterval<Identity: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfvalue: *mut f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match _ISystemMonitorUnion_Impl::UpdateInterval(this) {
                    Ok(ok__) => {
                        pfvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetDisplayType<Identity: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, edisplaytype: DisplayTypeConstants) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                _ISystemMonitorUnion_Impl::SetDisplayType(this, core::mem::transmute_copy(&edisplaytype)).into()
            }
        }
        unsafe extern "system" fn DisplayType<Identity: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pedisplaytype: *mut DisplayTypeConstants) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match _ISystemMonitorUnion_Impl::DisplayType(this) {
                    Ok(ok__) => {
                        pedisplaytype.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetManualUpdate<Identity: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstate: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                _ISystemMonitorUnion_Impl::SetManualUpdate(this, core::mem::transmute_copy(&bstate)).into()
            }
        }
        unsafe extern "system" fn ManualUpdate<Identity: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstate: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match _ISystemMonitorUnion_Impl::ManualUpdate(this) {
                    Ok(ok__) => {
                        pbstate.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetGraphTitle<Identity: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstitle: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                _ISystemMonitorUnion_Impl::SetGraphTitle(this, core::mem::transmute(&bstitle)).into()
            }
        }
        unsafe extern "system" fn GraphTitle<Identity: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstitle: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match _ISystemMonitorUnion_Impl::GraphTitle(this) {
                    Ok(ok__) => {
                        pbstitle.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetYAxisLabel<Identity: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstitle: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                _ISystemMonitorUnion_Impl::SetYAxisLabel(this, core::mem::transmute(&bstitle)).into()
            }
        }
        unsafe extern "system" fn YAxisLabel<Identity: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstitle: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match _ISystemMonitorUnion_Impl::YAxisLabel(this) {
                    Ok(ok__) => {
                        pbstitle.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CollectSample<Identity: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                _ISystemMonitorUnion_Impl::CollectSample(this).into()
            }
        }
        unsafe extern "system" fn UpdateGraph<Identity: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                _ISystemMonitorUnion_Impl::UpdateGraph(this).into()
            }
        }
        unsafe extern "system" fn BrowseCounters<Identity: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                _ISystemMonitorUnion_Impl::BrowseCounters(this).into()
            }
        }
        unsafe extern "system" fn DisplayProperties<Identity: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                _ISystemMonitorUnion_Impl::DisplayProperties(this).into()
            }
        }
        unsafe extern "system" fn Counter<Identity: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iindex: i32, ppicounter: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match _ISystemMonitorUnion_Impl::Counter(this, core::mem::transmute_copy(&iindex)) {
                    Ok(ok__) => {
                        ppicounter.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn AddCounter<Identity: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bspath: *mut core::ffi::c_void, ppicounter: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match _ISystemMonitorUnion_Impl::AddCounter(this, core::mem::transmute(&bspath)) {
                    Ok(ok__) => {
                        ppicounter.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DeleteCounter<Identity: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pctr: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                _ISystemMonitorUnion_Impl::DeleteCounter(this, core::mem::transmute_copy(&pctr)).into()
            }
        }
        unsafe extern "system" fn BackColorCtl<Identity: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcolor: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match _ISystemMonitorUnion_Impl::BackColorCtl(this) {
                    Ok(ok__) => {
                        pcolor.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetBackColorCtl<Identity: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, color: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                _ISystemMonitorUnion_Impl::SetBackColorCtl(this, core::mem::transmute_copy(&color)).into()
            }
        }
        unsafe extern "system" fn SetLogFileName<Identity: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bsfilename: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                _ISystemMonitorUnion_Impl::SetLogFileName(this, core::mem::transmute(&bsfilename)).into()
            }
        }
        unsafe extern "system" fn LogFileName<Identity: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bsfilename: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match _ISystemMonitorUnion_Impl::LogFileName(this) {
                    Ok(ok__) => {
                        bsfilename.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetLogViewStart<Identity: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, starttime: f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                _ISystemMonitorUnion_Impl::SetLogViewStart(this, core::mem::transmute_copy(&starttime)).into()
            }
        }
        unsafe extern "system" fn LogViewStart<Identity: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, starttime: *mut f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match _ISystemMonitorUnion_Impl::LogViewStart(this) {
                    Ok(ok__) => {
                        starttime.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetLogViewStop<Identity: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, stoptime: f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                _ISystemMonitorUnion_Impl::SetLogViewStop(this, core::mem::transmute_copy(&stoptime)).into()
            }
        }
        unsafe extern "system" fn LogViewStop<Identity: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, stoptime: *mut f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match _ISystemMonitorUnion_Impl::LogViewStop(this) {
                    Ok(ok__) => {
                        stoptime.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GridColor<Identity: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcolor: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match _ISystemMonitorUnion_Impl::GridColor(this) {
                    Ok(ok__) => {
                        pcolor.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetGridColor<Identity: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, color: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                _ISystemMonitorUnion_Impl::SetGridColor(this, core::mem::transmute_copy(&color)).into()
            }
        }
        unsafe extern "system" fn TimeBarColor<Identity: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcolor: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match _ISystemMonitorUnion_Impl::TimeBarColor(this) {
                    Ok(ok__) => {
                        pcolor.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetTimeBarColor<Identity: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, color: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                _ISystemMonitorUnion_Impl::SetTimeBarColor(this, core::mem::transmute_copy(&color)).into()
            }
        }
        unsafe extern "system" fn Highlight<Identity: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstate: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match _ISystemMonitorUnion_Impl::Highlight(this) {
                    Ok(ok__) => {
                        pbstate.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetHighlight<Identity: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstate: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                _ISystemMonitorUnion_Impl::SetHighlight(this, core::mem::transmute_copy(&bstate)).into()
            }
        }
        unsafe extern "system" fn ShowToolbar<Identity: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstate: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match _ISystemMonitorUnion_Impl::ShowToolbar(this) {
                    Ok(ok__) => {
                        pbstate.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetShowToolbar<Identity: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstate: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                _ISystemMonitorUnion_Impl::SetShowToolbar(this, core::mem::transmute_copy(&bstate)).into()
            }
        }
        unsafe extern "system" fn Paste<Identity: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                _ISystemMonitorUnion_Impl::Paste(this).into()
            }
        }
        unsafe extern "system" fn Copy<Identity: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                _ISystemMonitorUnion_Impl::Copy(this).into()
            }
        }
        unsafe extern "system" fn Reset<Identity: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                _ISystemMonitorUnion_Impl::Reset(this).into()
            }
        }
        unsafe extern "system" fn SetReadOnly<Identity: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstate: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                _ISystemMonitorUnion_Impl::SetReadOnly(this, core::mem::transmute_copy(&bstate)).into()
            }
        }
        unsafe extern "system" fn ReadOnly<Identity: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstate: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match _ISystemMonitorUnion_Impl::ReadOnly(this) {
                    Ok(ok__) => {
                        pbstate.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetReportValueType<Identity: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ereportvaluetype: ReportValueTypeConstants) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                _ISystemMonitorUnion_Impl::SetReportValueType(this, core::mem::transmute_copy(&ereportvaluetype)).into()
            }
        }
        unsafe extern "system" fn ReportValueType<Identity: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pereportvaluetype: *mut ReportValueTypeConstants) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match _ISystemMonitorUnion_Impl::ReportValueType(this) {
                    Ok(ok__) => {
                        pereportvaluetype.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetMonitorDuplicateInstances<Identity: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstate: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                _ISystemMonitorUnion_Impl::SetMonitorDuplicateInstances(this, core::mem::transmute_copy(&bstate)).into()
            }
        }
        unsafe extern "system" fn MonitorDuplicateInstances<Identity: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstate: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match _ISystemMonitorUnion_Impl::MonitorDuplicateInstances(this) {
                    Ok(ok__) => {
                        pbstate.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetDisplayFilter<Identity: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ivalue: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                _ISystemMonitorUnion_Impl::SetDisplayFilter(this, core::mem::transmute_copy(&ivalue)).into()
            }
        }
        unsafe extern "system" fn DisplayFilter<Identity: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pivalue: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match _ISystemMonitorUnion_Impl::DisplayFilter(this) {
                    Ok(ok__) => {
                        pivalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn LogFiles<Identity: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppilogfiles: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match _ISystemMonitorUnion_Impl::LogFiles(this) {
                    Ok(ok__) => {
                        ppilogfiles.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetDataSourceType<Identity: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, edatasourcetype: DataSourceTypeConstants) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                _ISystemMonitorUnion_Impl::SetDataSourceType(this, core::mem::transmute_copy(&edatasourcetype)).into()
            }
        }
        unsafe extern "system" fn DataSourceType<Identity: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pedatasourcetype: *mut DataSourceTypeConstants) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match _ISystemMonitorUnion_Impl::DataSourceType(this) {
                    Ok(ok__) => {
                        pedatasourcetype.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetSqlDsnName<Identity: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bssqldsnname: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                _ISystemMonitorUnion_Impl::SetSqlDsnName(this, core::mem::transmute(&bssqldsnname)).into()
            }
        }
        unsafe extern "system" fn SqlDsnName<Identity: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bssqldsnname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match _ISystemMonitorUnion_Impl::SqlDsnName(this) {
                    Ok(ok__) => {
                        bssqldsnname.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetSqlLogSetName<Identity: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bssqllogsetname: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                _ISystemMonitorUnion_Impl::SetSqlLogSetName(this, core::mem::transmute(&bssqllogsetname)).into()
            }
        }
        unsafe extern "system" fn SqlLogSetName<Identity: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bssqllogsetname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match _ISystemMonitorUnion_Impl::SqlLogSetName(this) {
                    Ok(ok__) => {
                        bssqllogsetname.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetEnableDigitGrouping<Identity: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstate: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                _ISystemMonitorUnion_Impl::SetEnableDigitGrouping(this, core::mem::transmute_copy(&bstate)).into()
            }
        }
        unsafe extern "system" fn EnableDigitGrouping<Identity: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstate: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match _ISystemMonitorUnion_Impl::EnableDigitGrouping(this) {
                    Ok(ok__) => {
                        pbstate.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetEnableToolTips<Identity: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstate: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                _ISystemMonitorUnion_Impl::SetEnableToolTips(this, core::mem::transmute_copy(&bstate)).into()
            }
        }
        unsafe extern "system" fn EnableToolTips<Identity: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstate: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match _ISystemMonitorUnion_Impl::EnableToolTips(this) {
                    Ok(ok__) => {
                        pbstate.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetShowTimeAxisLabels<Identity: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstate: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                _ISystemMonitorUnion_Impl::SetShowTimeAxisLabels(this, core::mem::transmute_copy(&bstate)).into()
            }
        }
        unsafe extern "system" fn ShowTimeAxisLabels<Identity: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstate: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match _ISystemMonitorUnion_Impl::ShowTimeAxisLabels(this) {
                    Ok(ok__) => {
                        pbstate.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetChartScroll<Identity: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bscroll: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                _ISystemMonitorUnion_Impl::SetChartScroll(this, core::mem::transmute_copy(&bscroll)).into()
            }
        }
        unsafe extern "system" fn ChartScroll<Identity: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbscroll: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match _ISystemMonitorUnion_Impl::ChartScroll(this) {
                    Ok(ok__) => {
                        pbscroll.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetDataPointCount<Identity: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, inewcount: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                _ISystemMonitorUnion_Impl::SetDataPointCount(this, core::mem::transmute_copy(&inewcount)).into()
            }
        }
        unsafe extern "system" fn DataPointCount<Identity: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pidatapointcount: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match _ISystemMonitorUnion_Impl::DataPointCount(this) {
                    Ok(ok__) => {
                        pidatapointcount.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ScaleToFit<Identity: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bselectedcountersonly: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                _ISystemMonitorUnion_Impl::ScaleToFit(this, core::mem::transmute_copy(&bselectedcountersonly)).into()
            }
        }
        unsafe extern "system" fn SaveAs<Identity: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrfilename: *mut core::ffi::c_void, esysmonfiletype: SysmonFileType) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                _ISystemMonitorUnion_Impl::SaveAs(this, core::mem::transmute(&bstrfilename), core::mem::transmute_copy(&esysmonfiletype)).into()
            }
        }
        unsafe extern "system" fn Relog<Identity: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrfilename: *mut core::ffi::c_void, esysmonfiletype: SysmonFileType, ifilter: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                _ISystemMonitorUnion_Impl::Relog(this, core::mem::transmute(&bstrfilename), core::mem::transmute_copy(&esysmonfiletype), core::mem::transmute_copy(&ifilter)).into()
            }
        }
        unsafe extern "system" fn ClearData<Identity: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                _ISystemMonitorUnion_Impl::ClearData(this).into()
            }
        }
        unsafe extern "system" fn LogSourceStartTime<Identity: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdate: *mut f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match _ISystemMonitorUnion_Impl::LogSourceStartTime(this) {
                    Ok(ok__) => {
                        pdate.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn LogSourceStopTime<Identity: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdate: *mut f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match _ISystemMonitorUnion_Impl::LogSourceStopTime(this) {
                    Ok(ok__) => {
                        pdate.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetLogViewRange<Identity: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, starttime: f64, stoptime: f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                _ISystemMonitorUnion_Impl::SetLogViewRange(this, core::mem::transmute_copy(&starttime), core::mem::transmute_copy(&stoptime)).into()
            }
        }
        unsafe extern "system" fn GetLogViewRange<Identity: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, starttime: *mut f64, stoptime: *mut f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                _ISystemMonitorUnion_Impl::GetLogViewRange(this, core::mem::transmute_copy(&starttime), core::mem::transmute_copy(&stoptime)).into()
            }
        }
        unsafe extern "system" fn BatchingLock<Identity: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, flock: super::super::Foundation::VARIANT_BOOL, ebatchreason: SysmonBatchReason) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                _ISystemMonitorUnion_Impl::BatchingLock(this, core::mem::transmute_copy(&flock), core::mem::transmute_copy(&ebatchreason)).into()
            }
        }
        unsafe extern "system" fn LoadSettings<Identity: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrsettingfilename: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                _ISystemMonitorUnion_Impl::LoadSettings(this, core::mem::transmute(&bstrsettingfilename)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Appearance: Appearance::<Identity, OFFSET>,
            SetAppearance: SetAppearance::<Identity, OFFSET>,
            BackColor: BackColor::<Identity, OFFSET>,
            SetBackColor: SetBackColor::<Identity, OFFSET>,
            BorderStyle: BorderStyle::<Identity, OFFSET>,
            SetBorderStyle: SetBorderStyle::<Identity, OFFSET>,
            ForeColor: ForeColor::<Identity, OFFSET>,
            SetForeColor: SetForeColor::<Identity, OFFSET>,
            Font: Font::<Identity, OFFSET>,
            putref_Font: putref_Font::<Identity, OFFSET>,
            Counters: Counters::<Identity, OFFSET>,
            SetShowVerticalGrid: SetShowVerticalGrid::<Identity, OFFSET>,
            ShowVerticalGrid: ShowVerticalGrid::<Identity, OFFSET>,
            SetShowHorizontalGrid: SetShowHorizontalGrid::<Identity, OFFSET>,
            ShowHorizontalGrid: ShowHorizontalGrid::<Identity, OFFSET>,
            SetShowLegend: SetShowLegend::<Identity, OFFSET>,
            ShowLegend: ShowLegend::<Identity, OFFSET>,
            SetShowScaleLabels: SetShowScaleLabels::<Identity, OFFSET>,
            ShowScaleLabels: ShowScaleLabels::<Identity, OFFSET>,
            SetShowValueBar: SetShowValueBar::<Identity, OFFSET>,
            ShowValueBar: ShowValueBar::<Identity, OFFSET>,
            SetMaximumScale: SetMaximumScale::<Identity, OFFSET>,
            MaximumScale: MaximumScale::<Identity, OFFSET>,
            SetMinimumScale: SetMinimumScale::<Identity, OFFSET>,
            MinimumScale: MinimumScale::<Identity, OFFSET>,
            SetUpdateInterval: SetUpdateInterval::<Identity, OFFSET>,
            UpdateInterval: UpdateInterval::<Identity, OFFSET>,
            SetDisplayType: SetDisplayType::<Identity, OFFSET>,
            DisplayType: DisplayType::<Identity, OFFSET>,
            SetManualUpdate: SetManualUpdate::<Identity, OFFSET>,
            ManualUpdate: ManualUpdate::<Identity, OFFSET>,
            SetGraphTitle: SetGraphTitle::<Identity, OFFSET>,
            GraphTitle: GraphTitle::<Identity, OFFSET>,
            SetYAxisLabel: SetYAxisLabel::<Identity, OFFSET>,
            YAxisLabel: YAxisLabel::<Identity, OFFSET>,
            CollectSample: CollectSample::<Identity, OFFSET>,
            UpdateGraph: UpdateGraph::<Identity, OFFSET>,
            BrowseCounters: BrowseCounters::<Identity, OFFSET>,
            DisplayProperties: DisplayProperties::<Identity, OFFSET>,
            Counter: Counter::<Identity, OFFSET>,
            AddCounter: AddCounter::<Identity, OFFSET>,
            DeleteCounter: DeleteCounter::<Identity, OFFSET>,
            BackColorCtl: BackColorCtl::<Identity, OFFSET>,
            SetBackColorCtl: SetBackColorCtl::<Identity, OFFSET>,
            SetLogFileName: SetLogFileName::<Identity, OFFSET>,
            LogFileName: LogFileName::<Identity, OFFSET>,
            SetLogViewStart: SetLogViewStart::<Identity, OFFSET>,
            LogViewStart: LogViewStart::<Identity, OFFSET>,
            SetLogViewStop: SetLogViewStop::<Identity, OFFSET>,
            LogViewStop: LogViewStop::<Identity, OFFSET>,
            GridColor: GridColor::<Identity, OFFSET>,
            SetGridColor: SetGridColor::<Identity, OFFSET>,
            TimeBarColor: TimeBarColor::<Identity, OFFSET>,
            SetTimeBarColor: SetTimeBarColor::<Identity, OFFSET>,
            Highlight: Highlight::<Identity, OFFSET>,
            SetHighlight: SetHighlight::<Identity, OFFSET>,
            ShowToolbar: ShowToolbar::<Identity, OFFSET>,
            SetShowToolbar: SetShowToolbar::<Identity, OFFSET>,
            Paste: Paste::<Identity, OFFSET>,
            Copy: Copy::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            SetReadOnly: SetReadOnly::<Identity, OFFSET>,
            ReadOnly: ReadOnly::<Identity, OFFSET>,
            SetReportValueType: SetReportValueType::<Identity, OFFSET>,
            ReportValueType: ReportValueType::<Identity, OFFSET>,
            SetMonitorDuplicateInstances: SetMonitorDuplicateInstances::<Identity, OFFSET>,
            MonitorDuplicateInstances: MonitorDuplicateInstances::<Identity, OFFSET>,
            SetDisplayFilter: SetDisplayFilter::<Identity, OFFSET>,
            DisplayFilter: DisplayFilter::<Identity, OFFSET>,
            LogFiles: LogFiles::<Identity, OFFSET>,
            SetDataSourceType: SetDataSourceType::<Identity, OFFSET>,
            DataSourceType: DataSourceType::<Identity, OFFSET>,
            SetSqlDsnName: SetSqlDsnName::<Identity, OFFSET>,
            SqlDsnName: SqlDsnName::<Identity, OFFSET>,
            SetSqlLogSetName: SetSqlLogSetName::<Identity, OFFSET>,
            SqlLogSetName: SqlLogSetName::<Identity, OFFSET>,
            SetEnableDigitGrouping: SetEnableDigitGrouping::<Identity, OFFSET>,
            EnableDigitGrouping: EnableDigitGrouping::<Identity, OFFSET>,
            SetEnableToolTips: SetEnableToolTips::<Identity, OFFSET>,
            EnableToolTips: EnableToolTips::<Identity, OFFSET>,
            SetShowTimeAxisLabels: SetShowTimeAxisLabels::<Identity, OFFSET>,
            ShowTimeAxisLabels: ShowTimeAxisLabels::<Identity, OFFSET>,
            SetChartScroll: SetChartScroll::<Identity, OFFSET>,
            ChartScroll: ChartScroll::<Identity, OFFSET>,
            SetDataPointCount: SetDataPointCount::<Identity, OFFSET>,
            DataPointCount: DataPointCount::<Identity, OFFSET>,
            ScaleToFit: ScaleToFit::<Identity, OFFSET>,
            SaveAs: SaveAs::<Identity, OFFSET>,
            Relog: Relog::<Identity, OFFSET>,
            ClearData: ClearData::<Identity, OFFSET>,
            LogSourceStartTime: LogSourceStartTime::<Identity, OFFSET>,
            LogSourceStopTime: LogSourceStopTime::<Identity, OFFSET>,
            SetLogViewRange: SetLogViewRange::<Identity, OFFSET>,
            GetLogViewRange: GetLogViewRange::<Identity, OFFSET>,
            BatchingLock: BatchingLock::<Identity, OFFSET>,
            LoadSettings: LoadSettings::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<_ISystemMonitorUnion as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl windows_core::RuntimeName for _ISystemMonitorUnion {}
pub const plaAlert: DataCollectorType = DataCollectorType(3i32);
pub const plaApiTrace: DataCollectorType = DataCollectorType(4i32);
pub const plaBinary: FileFormat = FileFormat(3i32);
pub const plaBoth: StreamMode = StreamMode(3i32);
pub const plaBuffering: StreamMode = StreamMode(4i32);
pub const plaCommaSeparated: FileFormat = FileFormat(0i32);
pub const plaCompiling: DataCollectorSetStatus = DataCollectorSetStatus(2i32);
pub const plaComputer: AutoPathFormat = AutoPathFormat(2i32);
pub const plaConfiguration: DataCollectorType = DataCollectorType(2i32);
pub const plaCreateCab: FolderActionSteps = FolderActionSteps(1i32);
pub const plaCreateHtml: DataManagerSteps = DataManagerSteps(4i32);
pub const plaCreateNew: CommitMode = CommitMode(1i32);
pub const plaCreateOrModify: CommitMode = CommitMode(3i32);
pub const plaCreateReport: DataManagerSteps = DataManagerSteps(1i32);
pub const plaCycle: ClockType = ClockType(3i32);
pub const plaDeleteCab: FolderActionSteps = FolderActionSteps(8i32);
pub const plaDeleteData: FolderActionSteps = FolderActionSteps(2i32);
pub const plaDeleteLargest: ResourcePolicy = ResourcePolicy(0i32);
pub const plaDeleteOldest: ResourcePolicy = ResourcePolicy(1i32);
pub const plaDeleteReport: FolderActionSteps = FolderActionSteps(16i32);
pub const plaEveryday: WeekDays = WeekDays(127i32);
pub const plaFile: StreamMode = StreamMode(1i32);
pub const plaFlag: ValueMapType = ValueMapType(2i32);
pub const plaFlagArray: ValueMapType = ValueMapType(3i32);
pub const plaFlushTrace: CommitMode = CommitMode(32i32);
pub const plaFolderActions: DataManagerSteps = DataManagerSteps(8i32);
pub const plaFriday: WeekDays = WeekDays(32i32);
pub const plaIndex: ValueMapType = ValueMapType(1i32);
pub const plaModify: CommitMode = CommitMode(2i32);
pub const plaMonday: WeekDays = WeekDays(2i32);
pub const plaMonthDayHour: AutoPathFormat = AutoPathFormat(256i32);
pub const plaMonthDayHourMinute: AutoPathFormat = AutoPathFormat(16384i32);
pub const plaNone: AutoPathFormat = AutoPathFormat(0i32);
pub const plaPattern: AutoPathFormat = AutoPathFormat(1i32);
pub const plaPending: DataCollectorSetStatus = DataCollectorSetStatus(3i32);
pub const plaPerformance: ClockType = ClockType(1i32);
pub const plaPerformanceCounter: DataCollectorType = DataCollectorType(0i32);
pub const plaRealTime: StreamMode = StreamMode(2i32);
pub const plaResourceFreeing: DataManagerSteps = DataManagerSteps(16i32);
pub const plaRunOnce: WeekDays = WeekDays(0i32);
pub const plaRunRules: DataManagerSteps = DataManagerSteps(2i32);
pub const plaRunning: DataCollectorSetStatus = DataCollectorSetStatus(1i32);
pub const plaSaturday: WeekDays = WeekDays(64i32);
pub const plaSendCab: FolderActionSteps = FolderActionSteps(4i32);
pub const plaSerialNumber: AutoPathFormat = AutoPathFormat(512i32);
pub const plaSql: FileFormat = FileFormat(2i32);
pub const plaStopped: DataCollectorSetStatus = DataCollectorSetStatus(0i32);
pub const plaSunday: WeekDays = WeekDays(1i32);
pub const plaSystem: ClockType = ClockType(2i32);
pub const plaTabSeparated: FileFormat = FileFormat(1i32);
pub const plaThursday: WeekDays = WeekDays(16i32);
pub const plaTimeStamp: ClockType = ClockType(0i32);
pub const plaTrace: DataCollectorType = DataCollectorType(1i32);
pub const plaTuesday: WeekDays = WeekDays(4i32);
pub const plaUndefined: DataCollectorSetStatus = DataCollectorSetStatus(4i32);
pub const plaUpdateRunningInstance: CommitMode = CommitMode(16i32);
pub const plaValidateOnly: CommitMode = CommitMode(4096i32);
pub const plaValidation: ValueMapType = ValueMapType(4i32);
pub const plaWednesday: WeekDays = WeekDays(8i32);
pub const plaYearDayOfYear: AutoPathFormat = AutoPathFormat(1024i32);
pub const plaYearMonth: AutoPathFormat = AutoPathFormat(2048i32);
pub const plaYearMonthDay: AutoPathFormat = AutoPathFormat(4096i32);
pub const plaYearMonthDayHour: AutoPathFormat = AutoPathFormat(8192i32);
pub const sysmonAverage: ReportValueTypeConstants = ReportValueTypeConstants(2i32);
pub const sysmonBatchAddCounters: SysmonBatchReason = SysmonBatchReason(2i32);
pub const sysmonBatchAddFiles: SysmonBatchReason = SysmonBatchReason(1i32);
pub const sysmonBatchAddFilesAutoCounters: SysmonBatchReason = SysmonBatchReason(3i32);
pub const sysmonBatchNone: SysmonBatchReason = SysmonBatchReason(0i32);
pub const sysmonChartArea: DisplayTypeConstants = DisplayTypeConstants(4i32);
pub const sysmonChartStackedArea: DisplayTypeConstants = DisplayTypeConstants(5i32);
pub const sysmonCurrentActivity: DataSourceTypeConstants = DataSourceTypeConstants(1i32);
pub const sysmonCurrentValue: ReportValueTypeConstants = ReportValueTypeConstants(1i32);
pub const sysmonDataAvg: SysmonDataType = SysmonDataType(1i32);
pub const sysmonDataCount: SysmonDataType = SysmonDataType(5i32);
pub const sysmonDataMax: SysmonDataType = SysmonDataType(3i32);
pub const sysmonDataMin: SysmonDataType = SysmonDataType(2i32);
pub const sysmonDataTime: SysmonDataType = SysmonDataType(4i32);
pub const sysmonDefaultValue: ReportValueTypeConstants = ReportValueTypeConstants(0i32);
pub const sysmonFileBlg: SysmonFileType = SysmonFileType(5i32);
pub const sysmonFileCsv: SysmonFileType = SysmonFileType(3i32);
pub const sysmonFileGif: SysmonFileType = SysmonFileType(7i32);
pub const sysmonFileHtml: SysmonFileType = SysmonFileType(1i32);
pub const sysmonFileReport: SysmonFileType = SysmonFileType(2i32);
pub const sysmonFileRetiredBlg: SysmonFileType = SysmonFileType(6i32);
pub const sysmonFileTsv: SysmonFileType = SysmonFileType(4i32);
pub const sysmonHistogram: DisplayTypeConstants = DisplayTypeConstants(2i32);
pub const sysmonLineGraph: DisplayTypeConstants = DisplayTypeConstants(1i32);
pub const sysmonLogFiles: DataSourceTypeConstants = DataSourceTypeConstants(2i32);
pub const sysmonMaximum: ReportValueTypeConstants = ReportValueTypeConstants(4i32);
pub const sysmonMinimum: ReportValueTypeConstants = ReportValueTypeConstants(3i32);
pub const sysmonNullDataSource: DataSourceTypeConstants = DataSourceTypeConstants(-1i32);
pub const sysmonReport: DisplayTypeConstants = DisplayTypeConstants(3i32);
pub const sysmonSqlLog: DataSourceTypeConstants = DataSourceTypeConstants(3i32);
